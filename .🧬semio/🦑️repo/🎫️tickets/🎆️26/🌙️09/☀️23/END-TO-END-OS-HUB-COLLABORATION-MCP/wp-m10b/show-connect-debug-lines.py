#!/usr/bin/env python3
from pathlib import Path
main = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/links/os-store")
sync = next(p for p in main.iterdir() if p.is_dir() and "sync" in p.name)
rs = next(p for p in sync.glob("*.rs"))
lines = rs.read_text().splitlines()
for i, line in enumerate(lines):
    if "start_connect BEGIN" in line or "before set_remote_state" in line or "after set_remote_state" in line or "connect_future entered" in line or "waiting admission" in line:
        print(f"{i+1}:{line}")
# also show 15 lines after BEGIN
for i, line in enumerate(lines):
    if "start_connect BEGIN" in line:
        for j in range(i, i+25):
            print(f"{j+1}:{lines[j]}")
        break
