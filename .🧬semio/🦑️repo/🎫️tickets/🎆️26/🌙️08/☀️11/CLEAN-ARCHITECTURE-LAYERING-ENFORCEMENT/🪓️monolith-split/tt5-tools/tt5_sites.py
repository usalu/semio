"""📋 Lists every TS2769 matcher diagnostic with its source line and the first message lines. usage: tt5_sites.py <tsc-output> [--code TS2769]"""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from tt5_blocks import blocks
code = sys.argv[sys.argv.index("--code") + 1] if "--code" in sys.argv else "TS2769"
cache = {}
for i, b in enumerate(blocks(sys.argv[1])):
    if b["code"] != code: continue
    f = b["file"]
    if f not in cache: cache[f] = open(f, encoding="utf8").read().split("\n")
    src = cache[f][b["line"] - 1].strip()[:220]
    msg = [l.strip() for l in b["text"].split("\n")[1:4]]
    print(f'#{i} {f.split("/")[-3] if False else f[-60:]}:{b["line"]}\n    {src}\n    {b["msg"][:160]}\n    ' + " | ".join(m[:140] for m in msg))
