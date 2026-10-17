"""🔭 Shows diagnostics of a tsc output for files whose path contains a substring, with an excerpt of the source around the column. usage: tt5_show.py <tsc-output> <substring> [--code TS2769] [--w 160]"""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from tt5_blocks import blocks
out, needle = sys.argv[1], sys.argv[2]
code = sys.argv[sys.argv.index("--code") + 1] if "--code" in sys.argv else None
w = int(sys.argv[sys.argv.index("--w") + 1]) if "--w" in sys.argv else 160
for b in blocks(out):
    if needle not in b["file"] or (code and b["code"] != code): continue
    lines = open(b["file"], encoding="utf8").read().split("\n")
    s = lines[b["line"] - 1]
    c = b["col"] - 1
    print(f'--- {b["file"][-70:]}:{b["line"]}:{b["col"]} {b["code"]}')
    print("  SRC: " + s[max(0, c - w):c + w])
    for l in b["text"].split("\n")[:7]: print("  " + l.strip()[:260])
