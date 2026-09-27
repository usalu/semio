#!/usr/bin/env python3
"""↩️ F9 un-landing: reverse exactly the hunks `content-id.py --write` made (a hunk qualifies when it touches the id
minting: `content_id`, `contentIds`, `DefaultHasher`, `Hasher`), leaving every peer hunk in the same files untouched, so
`content-id.py` stays re-runnable at the next window (carrier regeneration is its precondition). Dry run by default."""
import re, subprocess, sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
MINTING = re.compile(r"content_id|contentIds|DefaultHasher|Hasher|hasher|sha256_prefix|hashlib")
files = [line for line in Path(sys.argv[1]).read_text().splitlines() if line]
header = re.compile(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@")
for rel in files:
    diff = subprocess.run(["git", "-c", "core.quotepath=off", "diff", "HEAD", "-U0", "--", rel], cwd=ROOT, capture_output=True, text=True).stdout.splitlines()
    hunks, current = [], None
    for line in diff:
        m = header.match(line)
        if m:
            current = {"new_start": int(m[3]), "new_len": int(m[4]) if m[4] is not None else 1, "old": [], "new": []}
            hunks.append(current)
        elif current is not None and line.startswith("-") and not line.startswith("---"):
            current["old"].append(line[1:])
        elif current is not None and line.startswith("+") and not line.startswith("+++"):
            current["new"].append(line[1:])
    mine = [h for h in hunks if any(MINTING.search(text) for text in h["old"] + h["new"])]
    path = ROOT / rel
    lines = path.read_text().split("\n")
    for h in sorted(mine, key=lambda h: h["new_start"], reverse=True):
        start = h["new_start"] - 1 if h["new_len"] else h["new_start"]
        assert lines[start:start + h["new_len"]] == h["new"], f"{rel}: hunk at +{h['new_start']} drifted"
        lines[start:start + h["new_len"]] = h["old"]
    print(f"{'REVERT' if WRITE else 'DRY'} {len(mine)}/{len(hunks)} hunks {rel}")
    if WRITE: path.write_text("\n".join(lines))
