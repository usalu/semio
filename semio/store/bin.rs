//! `semio-store`: NDJSON JSON-RPC 2.0 on stdio (stdout = frames, stderr = logs).

mod jsonrpc;

use std::io::{self, BufRead, Write as _};
use std::sync::mpsc;
use std::sync::OnceLock;
use std::thread;

use semio::kit::KitStoreRef;

fn main() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .or_else(|_| std::env::var("RUST_TRACING")) // be lenient
                .unwrap_or_else(|_| "error".to_string()),
        )
        .with_target(false)
        .with_writer(io::stderr)
        .try_init();

    let (tx, rx) = mpsc::channel::<String>();
    let writer = thread::spawn(move || {
        let stdout = io::stdout();
        let mut lock = stdout.lock();
        for line in rx {
            if writeln!(&mut lock, "{line}").is_err() {
                break;
            }
            if lock.flush().is_err() {
                break;
            }
        }
    });

    let store: OnceLock<KitStoreRef> = OnceLock::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.is_empty() {
            continue;
        }
        jsonrpc::handle_line(&line, &store, &tx);
    }
    drop(tx);
    let _ = writer.join();
}
