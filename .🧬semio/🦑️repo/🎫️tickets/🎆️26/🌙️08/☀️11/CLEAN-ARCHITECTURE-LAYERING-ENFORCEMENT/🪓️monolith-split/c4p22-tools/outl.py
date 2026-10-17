"""📐 c4p1 outline printer: compact top-level item outline with region markers (read only)."""
import sys, re
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/frs-tools")
from rslex import outline
t = open(sys.argv[1], encoding="utf-8").read()
mn = int(sys.argv[2]) if len(sys.argv) > 2 else 0
L = t.split("\n")
for it in outline(t):
    n = it["end"] - it["start"] + 1
    reg = [L[i].strip() for i in range(it["start"], it["end"] + 1) if re.match(r"\s*//\s*#(end)?region", L[i])]
    if n >= mn or it["kind"] in ("mod",) or reg:
        print(f"{it['start']+1:5}-{it['end']+1:5} {n:5} {it['kind']:6} {it['vis'][:10]:10} {it['name'] or ''} {' '.join(reg[:3])}")
