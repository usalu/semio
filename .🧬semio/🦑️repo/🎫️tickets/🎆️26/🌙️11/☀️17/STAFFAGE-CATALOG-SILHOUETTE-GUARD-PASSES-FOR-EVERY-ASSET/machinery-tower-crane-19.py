"""Tower crane: the cab is a sharp box in the volume at 1:500 (1:200 done in script 12), so its drawn corners are square there; at 1:100 and 1:50 the volume rounds 0.1 m and the drawing keeps 0.1."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for sc in (500,):
    for view in ("elevation", "front", "plan"):
        try: n = d.shape(sc, view, "cab")
        except KeyError: continue
        if "r" in d.obj(n): d.setv(sp.child(n, "r"), 0)
d.commit()
