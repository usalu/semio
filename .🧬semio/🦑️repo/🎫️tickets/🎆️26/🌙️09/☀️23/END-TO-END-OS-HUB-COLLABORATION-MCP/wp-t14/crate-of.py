#!/usr/bin/env python3
"""📦️ Maps repo-relative files (stdin, one per line) to the Cargo package that owns them: the nearest ancestor module whose
`📦️packages/🦀️rust/Cargo.toml` exists (the crate root mounts deeper files with `#[path]`)."""
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
seen = {}
for line in sys.stdin:
    rel = line.strip()
    if not rel.endswith(".rs"):
        continue
    directory = (ROOT / rel).parent
    name = None
    while directory != ROOT and directory != directory.parent:
        manifest = directory / "📦️packages/🦀️rust/Cargo.toml"
        if manifest.exists():
            found = re.search(r'^name = "([^"]+)"', manifest.read_text(encoding="utf-8"), re.M)
            name = found.group(1) if found else None
            break
        directory = directory.parent
    seen.setdefault(name or "?", []).append(rel)
for name, files in sorted(seen.items()):
    print(f"{name}\t{len(files)}")
