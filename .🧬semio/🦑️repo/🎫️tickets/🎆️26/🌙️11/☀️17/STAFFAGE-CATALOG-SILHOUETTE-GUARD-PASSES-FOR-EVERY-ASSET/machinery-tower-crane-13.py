"""Tower crane 1:200: front and plan shapes without a solid are placed at rest and carried by the pose (the hoist lowers the load 20 m, the trolley travels 22 m): the sling strokes of the front get +20 m and those of the plan -22 m."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for view, dx, dy in (("front", 0, 20), ("plan", -22, 0)):
    for r in lib.strokes(200, view, "^sling-", "sl"):
        d.setv(sp.child(d.shape(200, view, r["id"] + "-stroke"), "d"), lib.shift(r["d"], dx, dy))
d.commit()
