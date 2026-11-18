"""Excavator 1:50: hood top slab sunk into the rounded house top (volume and drawings), hood top slab and grille side also in the front, hood rail posts at their true height and width in the front, stack as the stadium of its capsule in the front."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/excavator")
n = d.solid(50, "hood-grille-top")
d.setv(sp.child(n, "size"), [1.0, 0.05, 1.8]); d.setv(sp.child(sp.child(n, "transform"), "translation"), [-1.05, 1.245, 0.0])
g = d.shape(50, "elevation", "hood-grille-top")
d.setv(sp.child(g, "d"), "M -1.55,1.22 L -0.55,1.22 L -0.55,1.27 L -1.55,1.27 Z")
for v in ("front", "plan"):
    anchor = d.shape(50, v, "boom-body")
    d.insert_after(anchor, d.obj(d.shape(50, "elevation", "hood-grille-top")))
    if v == "front": d.insert_after(anchor, d.obj(d.shape(50, "elevation", "hood-grille")))
n = d.shape(50, "front", "hood-rail")
d.setv(sp.child(n, "points"), [[-1.65, 1.12], [-1.65, 1.5], [-0.3, 1.5], [-0.3, 1.12]])
d.setv(sp.child(sp.child(n, "solid"), "z0"), -1.151); d.setv(sp.child(sp.child(n, "solid"), "z1"), -1.149)
n = d.shape(50, "front", "stack"); o = d.obj(n)
out = {}
for k, v in o.items():
    if k == "x0": out.update({"x0": -0.9, "y0": 2.25, "x1": -0.9, "y1": 2.8})
    elif k not in ("y0", "x1", "y1", "solid"): out[k] = v
d.replace_obj(n, out)
d.commit()
