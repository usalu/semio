"""Excavator 1:100 and 1:50: the stack tip (volume and drawings) ends under the stack cap (capsule end y 1.76 m), so the round volume end does not stand out of the drawn rectangle in the front; the 1:50 front stack is the extruded capsule again."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/excavator")
for sc in (100, 50):
    n = d.solid(sc, "stack"); p1 = d.obj(n)["p1"]
    d.setv(sp.child(n, "p1"), [p1[0], 1.76, p1[2]])
    for v in ("elevation", "front", "plan"):
        s = d.shape(sc, v, "stack")
        if v == "front" and sc == 50:
            d.replace_obj(s, {"kind": "capsule", "x0": -1.2, "y0": 1.25, "x1": -1.2, "y1": 1.76, "r0": 0.055, "r1": 0.055, "id": "stack", "bone": "upper", "role": "detail", "z": d.obj(s)["z"], "filled": True, "tone": d.obj(s)["tone"], "region": d.obj(s).get("region"), "material": d.obj(s).get("material"), "solid": {"kind": "extruded", "sweep": "w", "z0": -0.955, "z1": -0.845}})
        else: d.setv(sp.child(s, "y1"), 1.76)
d.commit()
