#!/usr/bin/env python3
"""🔧 Literal control-sequence rename across LaTeX packages.

Usage: python 🔧️rename-cs.py <dir> <old>=<new> [<old>=<new> ...] [--files a.sty,b.sty]
Every replacement is a plain literal string substitution; nothing is interpreted
as a regular expression, so backslashes survive untouched.
"""
import sys
from pathlib import Path

args = sys.argv[1:]
root = Path(args[0])
only = None
pairs = []
i = 1
while i < len(args):
    if args[i] == "--files":
        only = set(args[i + 1].split(","))
        i += 2
        continue
    old, new = args[i].split("=", 1)
    pairs.append((old, new))
    i += 1

total = 0
for path in sorted(root.glob("*.sty")):
    if only is not None and path.name not in only:
        continue
    text = path.read_text(encoding="utf-8")
    original = text
    hits = 0
    for old, new in pairs:
        hits += text.count(old)
        text = text.replace(old, new)
    if text != original:
        path.write_text(text, encoding="utf-8")
        total += hits
        print(f"{path.name}: {hits}")
print(f"total {total}")
