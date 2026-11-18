"""Excavator 1:100: boom tip boss and stick tip boss also in the front and the plan, bucket lip drawn as the volume wedge (full width, no teeth) in the three views, hood rail in the front like at 1:50, hood top slab sunk into the house top."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/excavator")
LIP = [[1.28, 0.07], [1.35, 0.0], [1.5, -0.03]]
for v in ("front", "plan"):
    anchor = d.shape(100, v, "boom-body")
    for sid in ("stick-tip-boss", "boom-tip-boss"):
        d.insert_after(anchor, d.obj(d.shape(100, "elevation", sid)))
    for i in range(5):
        n = d.shape(100, v, f"bucket-tooth-{i}")
        d.setv(sp.child(n, "points"), LIP)
    n = d.shape(100, v, "bucket-tooth-0")
    d.setv(sp.child(sp.child(n, "solid"), "z0"), -0.75); d.setv(sp.child(sp.child(n, "solid"), "z1"), 0.25)
d.setv(sp.child(d.shape(100, "elevation", "bucket-teeth"), "d"), "M 1.28,0.07 L 1.35,0 L 1.5,-0.03 Z")
h = d.obj(d.shape(50, "front", "hood-rail"))
d.insert_after(d.shape(100, "front", "stack"), h)
n = d.solid(100, "hood-grille-top")
d.setv(sp.child(n, "size"), [1.0, 0.05, 1.8]); d.setv(sp.child(sp.child(n, "transform"), "translation"), [-1.05, 1.245, 0.0])
g = d.shape(100, "elevation", "hood-grille")
o = dict(d.obj(g)); o["id"] = "hood-grille-top"; o["d"] = "M -1.55,1.22 L -0.55,1.22 L -0.55,1.27 L -1.55,1.27 Z"; o["solid"] = {"kind": "extruded", "sweep": "w", "z0": -0.9, "z1": 0.9}
d.insert_after(g, o)
d.commit()
