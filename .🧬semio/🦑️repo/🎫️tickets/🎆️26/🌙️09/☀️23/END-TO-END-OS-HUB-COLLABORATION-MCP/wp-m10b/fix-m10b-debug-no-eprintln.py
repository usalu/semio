#!/usr/bin/env python3
from pathlib import Path

main = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/links/os-store")
sync = next(p for p in main.iterdir() if p.is_dir() and "sync" in p.name)
rs = next(p for p in sync.glob("*.rs"))
text = rs.read_text()
old = '''fn m10b_debug(msg: &str) {
    use std::io::Write;
    let path = "/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/generated/actor-debug.log";
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{}", msg);
        let _ = f.flush();
    }
    eprintln!("{}", msg);
}
'''
new = '''fn m10b_debug(msg: &str) {
    use std::io::Write;
    let path = "/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/generated/actor-debug.log";
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{}", msg);
        let _ = f.flush();
    }
}
'''
if old not in text:
    raise SystemExit("m10b_debug fn not found exact")
rs.write_text(text.replace(old, new, 1))
print("eprintln removed from m10b_debug")
