"""Tower crane 1:200, 1:100, 1:50: the hook is drawn as the traced silhouette of its volume loft in the elevation (machinery-contour.ts, filled, bone frame) and as the 0.08 m wide rectangle it is in the front."""
import sys, json
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for sc in (200, 100, 50):
    pts = json.load(open(f"{lib.T}/🗑️generated/tc-hook-{sc}.json"))
    n = d.shape(sc, "elevation", "hook"); o = d.obj(n)
    out = {k: v for k, v in o.items() if k not in ("penMm", "solid")}
    out["points"] = pts; out["filled"] = True; out["closed"] = True
    out = {k: out[k] for k in ("kind", "points", "closed", "id", "bone", "role", "z", "filled", "tone", "region", "regions", "material") if k in out}
    d.replace_obj(n, out)
    n = d.shape(sc, "front", "hook"); o = d.obj(n)
    out = {k: v for k, v in o.items() if k not in ("penMm", "d", "kind", "solid")}
    out.update({"kind": "poly", "points": [[-0.0, -0.4867], [0.01, -0.4867], [0.01, -1.239], [-0.0, -1.239]], "closed": True, "filled": True, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.04, "z1": 0.04}})
    out = {k: out[k] for k in ("kind", "points", "closed", "id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "solid") if k in out}
    d.replace_obj(n, out)
d.commit()
