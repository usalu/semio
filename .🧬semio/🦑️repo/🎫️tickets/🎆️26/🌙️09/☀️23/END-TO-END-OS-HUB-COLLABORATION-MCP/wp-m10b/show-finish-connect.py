#!/usr/bin/env python3
from pathlib import Path

main = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/links/os-store")
sync = next(p for p in main.iterdir() if p.is_dir() and "sync" in p.name)
rs = next(p for p in sync.glob("*.rs"))
lines = rs.read_text().splitlines()
for i, line in enumerate(lines):
    if "async fn finish_connect_hub" in line:
        for j in range(i, min(i + 90, len(lines))):
            print(f"{j+1}:{lines[j]}")
        break
