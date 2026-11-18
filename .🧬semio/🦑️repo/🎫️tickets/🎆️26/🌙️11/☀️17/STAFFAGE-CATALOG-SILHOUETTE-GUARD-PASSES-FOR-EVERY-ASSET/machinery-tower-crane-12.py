"""Tower crane 1:200: the four slings are drawn as the projected centre lines of the volume slings in the three views (replacing the member shapes), the cab has square corners (the volume cab is a box)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for view in ("elevation", "front", "plan"):
    res = lib.strokes(200, view, "^sling-", "sl")
    lib.apply(d, 200, view, res, ["sling-1", "sling-2", "sling-3", "sling-4"], "sling-1")
    n = d.shape(200, view, "cab")
    if "r" in d.obj(n): d.setv(sp.child(n, "r"), 0)
d.commit()
