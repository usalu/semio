"""Tower crane 1:200 and 1:100: the three pendants are drawn in the front and plan as the projected centre lines of the volume pendants (the swept paths stood 0.02 m beside them)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
for scale in (200, 100):
    for view in ("front", "plan"):
        res = lib.strokes(scale, view, "^pendant-", "pd")
        order = {"pendant-fore-l": "pendant-fore-1", "pendant-fore-r": "pendant-fore-2", "pendant-back": "pendant-back"}
        res.sort(key=lambda r: list(order).index(r["id"]))
        lib.apply(d, scale, view, res, [order[r["id"]] for r in res], "pendant-back")
d.commit()
