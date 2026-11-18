"""Concrete mixer truck: wheel hub caps end flush with the tyre outside at 1:100 (extent 2.55 m), the cab drawn like its extrusion: elevation outline equal to the volume profile, square plan and front corners."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/concrete-mixer-truck")
for i in (1, 2, 3, 4):
    for s, sign in (("l", 1), ("r", -1)):
        n = d.solid(100, f"brake-drum-{i}-{s}"); d.setv(sp.child(n, "profile"), [[0.0, 0.3], [0.03, 0.3]])
        n = d.solid(100, f"hub-{i}-{s}"); d.setv(sp.child(n, "profile"), [[0.0, 0.1], [0.015, 0.1]]); d.setv(sp.child(n, "origin"), [0.0, 0.0, sign * 1.26])
for sc in (100, 50):
    prof = d.obj(d.solid(sc, "cab-core"))["profile"]
    d.setv(sp.child(d.shape(sc, "elevation", "cab"), "points"), prof)
    for v in ("plan", "front"):
        d.setv(sp.child(d.shape(sc, v, "cab"), "r"), 0)
d.commit()
