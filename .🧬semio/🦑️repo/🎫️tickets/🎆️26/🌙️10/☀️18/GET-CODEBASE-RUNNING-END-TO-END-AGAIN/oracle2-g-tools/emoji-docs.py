#!/usr/bin/env python3
"""usage: emoji-docs.py <file> <START>=<EMOJI> ... : replaces the leading emoji of each docstring that starts with the given words so that every docstring of a file has its own emoji. Anchored: every START must occur exactly once."""
import re, sys
path = sys.argv[1]
s = open(path, encoding="utf-8").read()
for spec in sys.argv[2:]:
    start, emoji = spec.rsplit("=", 1)
    pattern = re.compile(r"/\*\* \S+ " + re.escape(start))
    found = pattern.findall(s)
    if len(found) != 1:
        sys.exit(f"anchor {start!r} occurs {len(found)} times")
    s = pattern.sub(lambda m: f"/** {emoji}️ {start}", s)
open(path, "w", encoding="utf-8").write(s)
