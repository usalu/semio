use std::env;
use std::process::{Command, ExitCode, ExitStatus};

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

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let Some(compiler) = args.next() else { return ExitCode::FAILURE };
    let args: Vec<_> = args.collect();
    let owner = parent();
    let mut retries = 0;
    loop {
        let status = match Command::new(&compiler).args(&args).status() {
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
