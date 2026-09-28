#!/usr/bin/env python3
"""📦️ L1: union of the Cargo / TS packages owning the recorded files of the given sets (from `w3-backup/<set>/manifest.json`).
usage: l1-union.py <set…>  → `rust\t<pkg>\t<files>` / `ts\t<pkg>\t<files>` / `?\t<top>\t<files>` lines, sorted."""
import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
STATE = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-l1-backup")
cache = {}


def owner(rel):
    directory = (ROOT / rel).parent
    rust_first = rel.endswith((".rs", ".toml", ".wit"))
    while directory != ROOT and directory != directory.parent:
        if directory in cache:
            found = cache[directory]
        else:
            found = []
            for kind, sub, pattern in (("rust", "📦️packages/🦀️rust/Cargo.toml", r'^name = "([^"]+)"'), ("ts", "📦️packages/🟦️typescript/package.json", r'"name":\s*"([^"]+)"')):
                path = directory / sub
                if path.exists():
                    match = re.search(pattern, path.read_text(encoding="utf-8"), re.M)
                    found.append((kind, match.group(1) if match else "?"))
            cache[directory] = found
        if found:
            preferred = "rust" if rust_first else "ts"
            return next((row for row in found if row[0] == preferred), found[0])
        directory = directory.parent
    return ("?", rel.split("/", 1)[0])


counts = {}
for name in sys.argv[1:]:
    for row in json.loads((STATE / name / "manifest.json").read_text(encoding="utf-8"))["files"]:
        key = owner(row["rel"])
        counts[key] = counts.get(key, 0) + 1
for (kind, name), total in sorted(counts.items()):
    print(f"{kind}\t{name}\t{total}")
