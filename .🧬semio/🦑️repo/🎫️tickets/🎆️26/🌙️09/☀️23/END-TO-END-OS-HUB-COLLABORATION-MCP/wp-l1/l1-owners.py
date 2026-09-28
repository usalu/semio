#!/usr/bin/env python3
"""📦️ L1: repo-relative paths found in text files (dry-run captures, set scripts) → owning Cargo packages / TS packages.
A path token starts at a top-level repo directory name; the owner is the nearest ancestor module with
`📦️packages/🦀️rust/Cargo.toml` (Rust) or `📦️packages/🟦️typescript/package.json` (TS). A `.rs` file maps to Rust, every
other file to the nearest owner of either kind.
usage: l1-owners.py <file…>  → lines `rust|ts<TAB>package<TAB>files`"""
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TOPS = sorted((entry.name for entry in ROOT.iterdir() if not entry.name.startswith(".")), key=len, reverse=True)
TOKEN = re.compile("(?:" + "|".join(re.escape(top) for top in TOPS) + r")/[^\s'\"`,;:()\[\]{}|<>]+")


def owner(rel, rust_first):
    directory = (ROOT / rel).parent
    while directory != ROOT and directory != directory.parent:
        cargo, package = directory / "📦️packages/🦀️rust/Cargo.toml", directory / "📦️packages/🟦️typescript/package.json"
        found = []
        if cargo.exists():
            match = re.search(r'^name = "([^"]+)"', cargo.read_text(encoding="utf-8"), re.M)
            found.append(("rust", match.group(1) if match else "?"))
        if package.exists():
            match = re.search(r'"name":\s*"([^"]+)"', package.read_text(encoding="utf-8"))
            found.append(("ts", match.group(1) if match else "?"))
        if found:
            if rust_first:
                return next((row for row in found if row[0] == "rust"), found[0])
            return next((row for row in found if row[0] == "ts"), found[0])
        directory = directory.parent
    return ("?", rel.split("/", 1)[0])


paths = set()
for name in sys.argv[1:]:
    for token in TOKEN.findall(Path(name).read_text(encoding="utf-8", errors="replace")):
        token = token.rstrip(".")
        if (ROOT / token).exists() or (ROOT / token).parent.exists() or "/" in token:
            paths.add(token)
owners = {}
for rel in sorted(paths):
    if rel.endswith("/"):
        continue
    kind = owner(rel, rel.endswith(".rs") or rel.endswith("Cargo.toml") or rel.endswith(".wit"))
    owners.setdefault(kind, []).append(rel)
for (kind, name), files in sorted(owners.items()):
    print(f"{kind}\t{name}\t{len(files)}")
