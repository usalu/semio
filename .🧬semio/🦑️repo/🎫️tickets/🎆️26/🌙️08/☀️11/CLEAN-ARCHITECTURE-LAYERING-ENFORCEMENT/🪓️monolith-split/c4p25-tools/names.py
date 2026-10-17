"""📇 c4p25: compact top-level item list `start:kind:name` (read only). usage: names.py <file>"""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/frs-tools")
from rslex import outline
t = open(sys.argv[1], encoding="utf-8").read()
out = []
for it in outline(t):
    v = {"": "", "pub": "+", "pub(crate)": "~"}.get(it["vis"], it["vis"][:5])
    out.append(f"{it['start']+1}:{it['kind'][:2]}{v}:{it['name'] or ''}")
line = ""
for o in out:
    if len(line) + len(o) > 190:
        print(line); line = ""
    line += o + "  "
print(line)
