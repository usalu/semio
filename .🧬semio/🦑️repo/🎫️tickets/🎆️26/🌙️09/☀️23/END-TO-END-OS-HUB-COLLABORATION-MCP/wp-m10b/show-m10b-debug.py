#!/usr/bin/env python3
from pathlib import Path

main = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/links/os-store")
sync = next(p for p in main.iterdir() if p.is_dir() and "sync" in p.name)
rs = next(p for p in sync.glob("*.rs"))
lines = rs.read_text().splitlines()
for i, line in enumerate(lines):
    if "m10b" in line and ("catchup wait: actual" in line or "Session confirmed" in line or "relay queue: confirmed" in line or "m10b_debug" in line and "format!" in line):
        start = max(0, i - 2)
        end = min(len(lines), i + 12)
        print(f"==== around {i+1} ====")
        for j in range(start, end):
            print(f"{j+1}:{lines[j]}")
