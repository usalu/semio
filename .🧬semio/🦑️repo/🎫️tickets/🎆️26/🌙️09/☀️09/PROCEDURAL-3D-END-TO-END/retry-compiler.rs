use std::env;
use std::ffi::OsString;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, ExitCode, ExitStatus, Stdio};

#[cfg(unix)]
fn parent() -> i32 {
    unsafe extern "C" { fn getppid() -> i32; }
    unsafe { getppid() }
}

#[cfg(not(unix))]
fn parent() -> i32 { 0 }

#[cfg(unix)]
fn terminated(status: ExitStatus) -> bool {
    use std::os::unix::process::ExitStatusExt;
    status.signal() == Some(15)
}

#[cfg(not(unix))]
fn terminated(_: ExitStatus) -> bool { false }

fn compiler_attempt(compiler: &OsString, args: &[OsString]) -> io::Result<ExitStatus> {
    let mut child = Command::new(compiler).args(args).stderr(Stdio::piped()).spawn()?;
    let mut reader = BufReader::new(child.stderr.take().expect("compiler stderr pipe"));
    let mut line = Vec::new();
    let mut artifacts = Vec::new();
    let mut stderr = io::stderr().lock();
    loop {
        line.clear();
        if reader.read_until(b'\n', &mut line)? == 0 { break; }
        if line.starts_with(br#"{"$message_type":"artifact","#) || line.starts_with(br#"{"artifact":"#) {
            artifacts.extend_from_slice(&line);
        } else {
            stderr.write_all(&line)?;
        }
    }
    let status = child.wait()?;
    if status.success() { stderr.write_all(&artifacts)?; }
    Ok(status)
}

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let Some(compiler) = args.next() else { return ExitCode::FAILURE };
    let args: Vec<_> = args.collect();
    let owner = parent();
    let mut retries = 0;
    loop {
        let status = match compiler_attempt(&compiler, &args) {
            Ok(status) => status,
            Err(error) => { eprintln!("{error}"); return ExitCode::FAILURE }
        };
        if retries < 2 && owner > 1 && parent() == owner && terminated(status) {
            retries += 1;
            eprintln!("[DEBUG] Compiler received SIGTERM while its Cargo owner was live. Retrying unit ({retries}/2).");
            continue;
        }
        return ExitCode::from(status.code().unwrap_or(1) as u8);
    }
}
