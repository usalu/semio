"""Tower crane 1:100 and 1:50: the cab box has sharp edges (volume cornerRadius removed, drawn corner radius 0) and its front is the extruded rectangle again (a traced polygon would be a flat shape on the rotating slewing bone)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
src = lib.D("🏗️machinery/tower-crane")
for sc in (100, 50):
    n = d.solid(sc, "cab"); o = d.obj(n)
    d.setv(sp.child(n, "cornerRadius"), 0)
    for v in ("elevation", "plan"): d.setv(sp.child(d.shape(sc, v, "cab"), "r"), 0)
    c = src.obj(src.shape(200, "front", "cab"))
    c = {k: v for k, v in c.items() if k != "r"}
    d.insert_after(d.shape(sc, "front", "slew-platform"), c)
d.commit()
