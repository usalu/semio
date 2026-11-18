"""Tower crane: the front skip body is the 1.4 m to 0.9 m trapezoid of the tapered volume extrusion (a swept path draws the full 1.4 m rectangle in the front), the mast foot top is 2.0 m wide in the front and 3.0 m in the elevation (taper only across the lateral axis)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/tower-crane")
for sc in (500, 200, 100, 50):
    n = d.shape(sc, "front", "skip-body"); o = d.obj(n)
    out = {}
    for k, v in o.items():
        if k == "d": out["kind"] = "poly"; out["points"] = [[-0.7, 15.8], [0.7, 15.8], [0.45, 14.8], [-0.45, 14.8]]; out["closed"] = True
        elif k in ("kind", "solid"): continue
        else: out[k] = v
    d.replace_obj(n, out)
    d.setv(sp.child(d.shape(sc, "front", "mast-foot"), "points"), [[-1.5, 1], [1.5, 1], [1, 1.5], [-1, 1.5]])
    d.setv(sp.child(d.shape(sc, "elevation", "mast-foot"), "points"), [[-1.5, 1], [1.5, 1], [1.5, 1.5], [-1.5, 1.5]])
d.commit()
