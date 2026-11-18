"""Tower crane: end-on members keep a section diameter equal to their pen (a first try with 1.45 times the pen to enclose the square volume dot made the 1:200 and 1:100 fronts worse; this script restores the pen as section)."""
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
                d.setv(sp.child(sp.child(d.shape(sc, view, x["id"]), "section"), "d"), round(1.0 * pen, 4)); n_ += 1
print(n_)
d.commit()
