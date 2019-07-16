use std::io;
use std::io::{BufWriter, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap();
    let remainder = args.collect::<Vec<_>>();

    let mut buf = String::new();
    while stdin.read_line(&mut buf)? > 0 {
        let mut child = Command::new(&command)
            .args(remainder.clone())
            .stdin(Stdio::piped())
            .spawn()?;

        if let Some(child_stdin) = child.stdin.as_mut() {
            let mut writer = BufWriter::new(child_stdin);
            writer.write_all(buf.as_bytes())?;
            writer.write(&[0])?; // TODO do I need this?
            writer.flush()?;
        } else {
            eprintln!("Failed to retrieve child stdin");
            std::process::exit(-1);
        }

        let status = child.wait()?;
        if !status.success() {
            std::process::exit(status.code().unwrap_or(-1))
        }

        buf.clear();
    }

    Ok(())
}
