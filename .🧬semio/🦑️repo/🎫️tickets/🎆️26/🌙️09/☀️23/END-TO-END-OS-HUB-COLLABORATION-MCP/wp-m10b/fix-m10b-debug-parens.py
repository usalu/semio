#!/usr/bin/env python3
from pathlib import Path

link = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b/links/os-store")
sync_dirs = [p for p in link.iterdir() if p.is_dir() and "sync" in p.name]
assert len(sync_dirs) == 1, sync_dirs
rs_files = list(sync_dirs[0].glob("*.rs"))
# prefer the main rs at sync root
candidates = [p for p in sync_dirs[0].rglob("*.rs") if p.name.endswith(".rs")]
main = None
for p in candidates:
    text = p.read_text()
    if "fn m10b_debug" in text and "relay_operations_to_hub" in text:
        main = p
        break
assert main is not None, "sync rs not found"
print("file", main)

lines = main.read_text().splitlines()
out = []
in_m10b = False
fixed = 0
for line in lines:
    if "m10b_debug(&format!" in line:
        in_m10b = True
    if in_m10b and line.strip() == ");":
        out.append(line.replace(");", "));"))
        in_m10b = False
        fixed += 1
        continue
    if in_m10b and line.strip().endswith("));"):
        in_m10b = False
    out.append(line)

main.write_text("\n".join(out) + "\n")
print("fixed", fixed)

# verify multiline closings
text = main.read_text()
for needle in ["m10b catchup wait: actual", "m10b Session confirmed", "m10b relay queue: confirmed"]:
    idx = text.find(needle)
    print(needle, "ok" if idx >= 0 and "));" in text[idx : idx + 400] else "BAD")
