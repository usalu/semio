"""Forklift 1:50: counterweight vent on the straight part of the rear face (local y 0.1 to 0.3 m), steering wheel drawn as the edge-on ring it is in the elevation (capsule of 12 mm radius along the ring diameter), pallet goods corners rounded 10 mm in the elevation."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/forklift")
n = d.solid(50, "counterweight-vent")
d.setv(sp.child(sp.child(n, "transform"), "translation"), [-0.495, 0.2, 0.0])
d.setv(sp.child(d.shape(50, "elevation", "counterweight-vent"), "d"), "M -0.45,0.1 L -0.45,0.3 M -0.38,0.1 L -0.38,0.3 M -0.31,0.1 L -0.31,0.3")
n = d.shape(50, "elevation", "steering-wheel"); o = d.obj(n)
out = {}
for k, v in o.items():
    if k == "kind": out.update({"kind": "capsule", "x0": -0.2778, "y0": 1.1902, "x1": -0.1622, "y1": 1.5098, "r0": 0.012, "r1": 0.012})
    elif k == "filled": out[k] = True
    elif k not in ("cx", "cy", "rx", "ry", "rotationDeg"): out[k] = v
d.replace_obj(n, out)
d.setv(sp.child(d.shape(50, "elevation", "pallet-goods"), "r"), 0.01)
d.commit()
