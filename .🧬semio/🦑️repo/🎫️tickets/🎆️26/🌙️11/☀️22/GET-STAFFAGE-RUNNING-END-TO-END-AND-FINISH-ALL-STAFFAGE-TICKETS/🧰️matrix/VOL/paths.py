#!/usr/bin/env python3
"""🔎️ The JSON paths below `asset` that differ between two documents, at most four levels deep: paths.py <before> <after>"""
import json, sys
a, b = (json.load(open(p, encoding="utf-8")) for p in sys.argv[1:3])
out = []
def walk(x, y, path):
    if x == y: return
    if isinstance(x, dict) and isinstance(y, dict) and len(path) < 4:
        for k in sorted(set(x) | set(y)): walk(x.get(k), y.get(k), path + [k])
    else: out.append(".".join(path))
walk(a, b, [])
print(b["asset"]["id"], "differs in:", ", ".join(out) or "nothing")
