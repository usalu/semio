"""Diesel generator 1:100 and 1:50: roof edge slab (2.24 m wide, 15 or 30 mm) drawn in the elevation and front, lifting eye in the front at 1:50, stack drawn from its base at 3.1 m in the front, hood slope slab inside the hood outline at 1:50."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/diesel-generator")
for sc, t, yc in ((100, 0.03, 2.6), (50, 0.015, 2.5975)):
    for v, x, w in (("elevation", -2.93, 5.86), ("front", -1.12, 2.24)):
        base = d.obj(d.shape(sc, v, "roof-band"))
        o = {"id": "roof-edge", "bone": "container", "role": "detail", "z": 2.2, "filled": True, "tone": 0.4, "kind": "rect", "x": x, "y": round(yc - t / 2, 4), "w": w, "h": t, "region": "roof", "material": "container-steel"}
        d.insert_after(d.shape(sc, v, "roof-band"), o)
n = d.shape(50, "front", "roof-band")
d.insert_after(n, {"id": "lifting-eye", "bone": "container", "role": "detail", "z": 2.3, "filled": True, "tone": 0.2, "kind": "rect", "x": -1.015, "y": 2.59, "w": 2.03, "h": 0.07, "region": "roof", "material": "container-steel"})
for sc in (200, 100, 50):
    n = d.shape(sc, "front", "exhaust-stack"); d.setv(sp.child(n, "y"), 3.1); d.setv(sp.child(n, "h"), 0.37)
n = d.solid(50, "hood-slope")
d.setv(sp.child(sp.child(n, "transform"), "translation"), [2.0641, 3.0859, 0.0])
d.commit()
