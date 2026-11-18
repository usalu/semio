"""Tower crane 1:100 and 1:50: the front cab is the traced silhouette of its rounded volume box (the swept rectangle rounds only the elevation corners; machinery-contour.ts)."""
import sys, json
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
for sc in (100, 50):
    pts = json.load(open(f"{lib.T}/🗑️generated/tc-cab-front-{sc}.json"))
    n = d.shape(sc, "front", "cab"); o = d.obj(n)
    out = {"kind": "poly", "points": pts, "closed": True}
    for k in ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material"):
        if k in o: out[k] = o[k]
    d.replace_obj(n, out)
d.commit()
