import sys, os, importlib, json
sys.path.insert(0, os.path.dirname(__file__))
from common import *
from probe import gaps, svg
GEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-V1"
name = sys.argv[1]
mode = sys.argv[2] if len(sys.argv) > 2 else "all"
mod = importlib.import_module("sp_" + name)
S = mod.build()
doc, areas = assemble(S)
a = doc["asset"]
print("dims", a["dimensions"], "areas", {k: round(v, 1) for k, v in areas.items()})
for sc in ["500", "200", "100", "50"]:
    sh = S.shapes[sc]
    xs = [p for s in sh for r in flat(s.cmds, 4) for p in r]
    print(sc, "bbox", [round(v, 2) for v in (min(p[0] for p in xs), min(p[1] for p in xs), max(p[0] for p in xs), max(p[1] for p in xs))], "shapes", len(sh))
    if mode != "nogap":
        g = gaps(sh, int(sc), PENS[sc])
        print("  gaps<0.15:", len(g), g[:12])
    svg(sh, int(sc), PENS[sc], "%s/%s-%s" % (GEN, S.id, sc), ppm=8.0 if sc != "500" else 8.0)
if mode != "dry":
    d = S_ROOT + "/" + S.folder
    os.makedirs(d, exist_ok=True)
    dump(d + "/🔣️.json", doc)
    print("written", d)
