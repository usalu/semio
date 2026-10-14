#!/usr/bin/env python3
"""🧺️ Prints `-p <package>` for every member package of a Cargo workspace whose name equals or extends (`<name>-…`)
one of the given family names.

usage: 🧺️packages.py <workspace Cargo.toml> <family name>...
"""
import os, re, sys
ws = sys.argv[1]
base = os.path.dirname(ws)
text = open(ws, encoding="utf-8").read()
members = re.findall(r'"([^"]+)"', re.search(r"(?ms)^members\s*=\s*\[(.*?)\]", text).group(1))
out = []
for m in members:
    manifest = os.path.join(base, m, "Cargo.toml")
    if not os.path.exists(manifest):
        continue
    name = re.search(r'(?m)^name\s*=\s*"([^"]+)"', open(manifest, encoding="utf-8").read()).group(1)
    if any(name == f or name.startswith(f + "-") for f in sys.argv[2:]):
        out += ["-p", name]
print(" ".join(out))
