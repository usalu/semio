"""Excavator 1:100 and 1:50: track belts drawn with the square plan and front corners of their extrusion, front track frame from the car body underside (0.3 m), hood rail posts sunk 8 cm into the rounded house top in the volume and the elevation drawing."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/excavator")
for sc in (100, 50):
    for sid in ("track-belt-left", "track-belt-right"):
        d.setv(sp.child(d.shape(sc, "front", sid), "r"), 0)
        d.setv(sp.child(d.shape(sc, "plan", sid), "r"), 0)
    n = d.shape(sc, "front", "track-frame")
    d.setv(sp.child(n, "y"), 0.3); d.setv(sp.child(n, "h"), 0.56)
    n = d.solid(sc, "hood-rail-post"); p0 = d.obj(n)["p0"]
    d.setv(sp.child(n, "p0"), [p0[0], 1.12, p0[2]])
    n = d.shape(sc, "elevation", "hood-rail")
    d.setv(sp.child(n, "points"), [[-1.65, 1.12], [-1.65, 1.5], [-0.3, 1.5], [-0.3, 1.12]])
d.commit()
