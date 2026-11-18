"""Wheel loader 1:50: hub plates touch the tyre sides (13 mm thick from 1.4 m, mirrored on the right side)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/wheel-loader")
for sid in ("hub-r", "hub-f"):
    n = d.solid(50, sid, "body")
    d.setv(sp.child(n, "profile"), [[0.0, 0.0], [0.0, 0.14], [0.013, 0.14], [0.013, 0.0]])
    d.setv(sp.child(n, "origin"), [0, 0, 1.4])
    d.setv(sp.child(sp.child(n, "repeat"), "translation"), [0, 0, -2.813])
d.commit()
