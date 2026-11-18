"""Wheel loader 1:100 and 1:50: tyres of the front and plan drawn as the polygon the volume projects (lug blocks to radius 0.875 m over the axial 0.25 m, tread chamfer to radius 0.74 m at 0.3 m) instead of a 0.2 m rounded rectangle."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
AR = [(-.25, .875), (.25, .875), (.26, .845), (.3, .74), (.3, -.74), (.26, -.845), (.25, -.875), (-.25, -.875), (-.26, -.845), (-.3, -.74), (-.3, .74), (-.26, .845)]
def retype(o, pts):
    out = {}
    for k, v in o.items():
        if k == "kind": out[k] = "poly"; out["points"] = pts; out["closed"] = True
        elif k not in ("x", "y", "w", "h", "r"): out[k] = v
    return out
d = D("🏗️machinery/wheel-loader")
for sc in (100, 50):
    for sid, lat, ctr in (("tyre-f-l", 1.1, 1.7), ("tyre-f-r", -1.1, 1.7)):
        n = d.shape(sc, "front", sid)
        d.replace_obj(n, retype(d.obj(n), [[round(lat + a, 4), round(0.875 + r, 4)] for a, r in AR]))
    for sid, lat, ctr in (("tyre-f-l", 1.1, 1.7), ("tyre-f-r", -1.1, 1.7), ("tyre-r-l", 1.1, -1.7), ("tyre-r-r", -1.1, -1.7)):
        n = d.shape(sc, "plan", sid)
        d.replace_obj(n, retype(d.obj(n), [[round(ctr + r, 4), round(lat + a, 4)] for a, r in AR]))
d.commit()
