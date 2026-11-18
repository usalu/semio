"""Concrete mixer truck 1:50: the windscreen slab sits inside the slanted cab front (its front face on the cab outline) instead of standing 10 mm out of it."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/concrete-mixer-truck")
n = d.solid(50, "cab-windscreen")
d.setv(sp.child(sp.child(n, "transform"), "translation"), [4.2997, 2.85, 0.0])
d.commit()
