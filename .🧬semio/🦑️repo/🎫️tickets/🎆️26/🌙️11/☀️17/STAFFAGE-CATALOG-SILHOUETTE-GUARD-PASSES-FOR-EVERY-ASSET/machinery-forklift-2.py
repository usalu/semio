"""Forklift: wheel hubs and rim bolts flush with the tyre sides at 1:50 (extent 1.2 m), chassis and counterweight elevation outlines as the polygons of the volume extrusions at 1:100 and 1:50, front counterweight with square corners, pallet runner in the 1:200 front, tilt cylinder in the 1:500 elevation."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/forklift")
for sid in ("hub-f-left", "hub-f-right", "hub-r-left", "hub-r-right"):
    n = d.solid(50, sid); p = d.obj(n)["profile"]
    d.setv(sp.child(n, "profile"), [p[0], [0.01, p[1][1]]])
for sid, z in (("rim-bolt-f-left", 0.592), ("rim-bolt-f-right", -0.592), ("rim-bolt-r-left", 0.312), ("rim-bolt-r-right", -0.312)):
    n = d.solid(50, sid); o = d.obj(n)["origin"]
    d.setv(sp.child(n, "origin"), [o[0], o[1], z])
CHASSIS = "M -1.7,0.18 L 0.27,0.18 L 0.4,0.19 L 0.45,0.28 L 0.45,0.78 L 0.43,0.86 L 0.35,0.92 L 0.05,0.96 L -0.02,0.58 L -1.7,0.58 Z"
COUNTER = "M 0.25,-0.52 L -0.35,-0.52 L -0.45,-0.48 L -0.5,-0.35 L -0.5,0.3 L -0.45,0.45 L -0.35,0.5 L 0.25,0.5 Z"
for sc in (100, 50):
    d.setv(sp.child(d.shape(sc, "elevation", "chassis-body"), "d"), CHASSIS)
    d.setv(sp.child(d.shape(sc, "elevation", "counterweight-body"), "d"), COUNTER)
for sc in (100, 50):
    d.setv(sp.child(d.shape(sc, "front", "counterweight-body"), "r"), 0)
n = d.shape(200, "front", "pallet-deck"); o = d.obj(n)
o["id"] = "pallet-runner"; o["y"] = 0; o["h"] = 0.1
d.insert_after(n, o)
o = d.obj(d.shape(200, "elevation", "tilt-barrel"))
o["role"] = "body"; o["r0"] = 0.0378; o["r1"] = 0.0378
d.insert_after(d.shape(500, "elevation", "chassis-body"), o)
d.commit()
