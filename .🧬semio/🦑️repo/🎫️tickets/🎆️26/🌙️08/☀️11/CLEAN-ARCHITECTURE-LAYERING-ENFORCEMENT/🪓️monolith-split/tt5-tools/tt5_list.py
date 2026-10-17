"""📋 Compact list of real diagnostics (skips TS2307/TS7016 and mit-bestand): file tail, line:col, code, first message, source excerpt. usage: tt5_list.py <tsc-output> [--code TS2769] [--skip-file substr] [--only substr] [--w 90]"""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from tt5_blocks import blocks
a = sys.argv
code = a[a.index("--code") + 1] if "--code" in a else None
only = a[a.index("--only") + 1] if "--only" in a else None
w = int(a[a.index("--w") + 1]) if "--w" in a else 90
cache = {}
for b in blocks(a[1]):
    if b["code"] in ("TS2307", "TS7016") or "mit-bestand" in b["file"]: continue
    if code and b["code"] != code: continue
    if only and only not in b["file"]: continue
    f = b["file"]
    if f not in cache: cache[f] = open(f, encoding="utf8").read().split("\n") if os.path.exists(f) else []
    s = cache[f][b["line"] - 1] if b["line"] - 1 < len(cache[f]) else ""
    c = b["col"] - 1
    msg = b["msg"][:150]
    extra = [l.strip()[:150] for l in b["text"].split("\n")[1:3]]
    print(f'{f[-75:]}:{b["line"]}:{b["col"]} {b["code"]} {msg}\n    > {s[max(0, c - w):c + w]}')
    for e in extra: print("      " + e)
