"""Tower crane 1:50: the tower head legs, diagonals, base frame bars and cross bars (member shapes) are drawn as the projected quadrilaterals of the matching volume bars (long ones filled, short ones as pen strokes), one shape per bar copy."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
for view in ("elevation", "front", "plan"):
    old = [x["id"] for x in d.val("asset", "representations", "50", "geometry", view) if x["id"].startswith(("tower-head-leg", "tower-head-diagonal", "tower-head-frame", "tower-head-cross")) and x["kind"] == "member"]
    res = lib.strokes(50, view, "^tower-head-(leg|diagonal|frame|cross)", "thq", "quadcopies")
    print(view, len(old), len(res))
    lib.apply_copies(d, 50, view, res, old)
d.commit()
