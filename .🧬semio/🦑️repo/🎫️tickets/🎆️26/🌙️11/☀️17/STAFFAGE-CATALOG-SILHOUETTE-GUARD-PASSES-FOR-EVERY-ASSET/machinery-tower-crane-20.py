"""Tower crane 1:50: every drawn member (3D bar shape) gets the pen that equals its section diameter (d / 0.05 paper millimetres): an unfilled member whose hull encloses the volume is drawn as its hull, while the slender volume bar follows the pen of the shape (12.5 mm role pen), so the volume bars were drawn 6 to 8 times thinner than the drawing."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
count = 0
for view in ("elevation", "front", "plan"):
    for x in d.val("asset", "representations", "50", "geometry", view):
        if x["kind"] != "member" or "penMm" in x: continue
        n = d.shape(50, view, x["id"]); o = dict(x)
        o["penMm"] = round(x["section"].get("d", x["section"].get("w")) * 1000 / 50, 4)
        d.replace_obj(n, o); count += 1
print(count)
d.commit()
