"""Scissor lift: the lift cylinder stands on the chassis deck (0.45 m to 0.55 m) in the volume and the front drawing."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/scissor-lift")
for sc in (200, 100, 50):
    n = d.solid(sc, "lift-cylinder")
    d.setv(sp.child(sp.child(n, "transform"), "translation"), [0.0, 0.5, 0.0])
    s = d.shape(sc, "front", "lift-cylinder")
    d.setv(sp.child(s, "y"), 0.45)
d.commit()
