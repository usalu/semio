use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
fn now_us() -> u128 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() }
fn main() {
    let start = now_us();
    let a: Vec<String> = std::env::args().collect();
    let task = a.get(1).cloned().unwrap_or_default();
    let out = a.get(2).cloned().unwrap_or_default();
    if let Ok(dir) = std::env::var("FAKE_STAMP_DIR") {
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(format!("{}/{}.start", dir, task.replace(':', "_")), start.to_string());
    }
    let work: u64 = std::env::var("FAKE_WORK_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(30);
    std::thread::sleep(std::time::Duration::from_millis(work));
    if !out.is_empty() {
        let _ = std::fs::create_dir_all(&out);
        let _ = std::fs::write(format!("{}/out.txt", out), format!("built {} at {}\n", task, start));
    }
    println!("built {}", task);
    let _ = std::io::stdout().flush();
}
