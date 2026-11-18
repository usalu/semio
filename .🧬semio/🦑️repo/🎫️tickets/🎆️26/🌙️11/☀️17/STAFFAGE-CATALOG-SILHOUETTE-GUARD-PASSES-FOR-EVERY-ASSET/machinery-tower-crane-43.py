"""Tower crane: an end-on member (projected shorter than three pen widths) gets a tiny section (4 mm): its hull ring is then a point at the bar, so the slender volume bar finds this member as its nearest drawn centre line and follows its pen, and the guard strokes the ring with that pen into the dot of the bar."""
import sys, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
n_ = 0
for sc in (200, 100, 50):
    for view in ("elevation", "front"):
        for x in d.val("asset", "representations", str(sc), "geometry", view):
            if x["kind"] != "member" or x["bone"] not in ("jib", "counter-jib", "slewing") or x["id"].startswith("sling") or "penMm" not in x: continue
            pen = x["penMm"] * sc / 1000
            a, b = x["from"], x["to"]
            L = math.hypot(a["x"] - b["x"], a["y"] - b["y"]) if view == "elevation" else math.hypot(a["z"] - b["z"], a["y"] - b["y"])
            if L < 3 * pen:
                d.setv(sp.child(sp.child(d.shape(sc, view, x["id"]), "section"), "d"), 0.004); n_ += 1
print(n_)
d.commit()
