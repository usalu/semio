"""📋 c4p38 outline: top-level items of a snapshot with 1-based line ranges. usage: outline38.py <n>"""
import sys, os
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/frs-tools")
from rslex import outline
n = sys.argv[1]
t = open(os.path.dirname(os.path.abspath(__file__)) + f"/orig/{n}.rs", encoding="utf-8").read()
for it in outline(t):
    print(f"{it['start']+1:5}-{it['end']+1:5} {it['end']-it['start']+1:4} {it['vis']:10} {it['kind']:7} {it['name']}")
