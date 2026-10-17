"""📑 C4P28 item ranges: top-level items of a Rust file with their 1-based inclusive line range including attached docs, attributes and region comments. usage: items.py <file> [min-lines]"""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rf8-tools")
from rslex import outline
t = open(sys.argv[1], encoding="utf-8").read()
mn = int(sys.argv[2]) if len(sys.argv) > 2 else 0
for it in outline(t):
    n = it["end"] - it["start"] + 1
    if n >= mn or it["kind"] in ("mod",):
        print(f"{it['start']+1:5}-{it['end']+1:5} {n:5} {it['kind']:6} {it['vis']:10} {it['name'] or ''} | {it['head'][:60]}")
