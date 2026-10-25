use std::{env, fs, io::Write, thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}};

fn stamp() -> u128 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() }

fn note(log: &str, what: &str, variant: &str) {
    let mut file = fs::OpenOptions::new().create(true).append(true).open(log).unwrap();
    writeln!(file, "{} {} {} {}", what, std::process::id(), variant, stamp()).unwrap();
}

fn main() {
    println!("cargo:rerun-if-env-changed=CHURN_VARIANT");
    println!("cargo:rerun-if-env-changed=CHURN_LOG");
    let log = env::var("CHURN_LOG").unwrap();
    let variant = env::var("CHURN_VARIANT").unwrap_or_default();
    let out = env::var("OUT_DIR").unwrap();
    note(&log, "start", &variant);
    let target = format!("{}/probe.txt", out);
    fs::write(&target, "").unwrap();
    for part in 0..10 {
        let mut file = fs::OpenOptions::new().append(true).open(&target).unwrap();
        write!(file, "{}:{} ", variant, part).unwrap();
        sleep(Duration::from_millis(500));
    }
    note(&log, "end", &variant);
}
