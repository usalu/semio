"""Tower crane 1:200, 1:100, 1:50: the front skip trapezoid at the rest height of those drawings (the pose lowers the skip by 20 m)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/tower-crane")
for sc in (200, 100, 50):
    d.setv(sp.child(d.shape(sc, "front", "skip-body"), "points"), [[-0.7, 35.8], [0.7, 35.8], [0.45, 34.8], [-0.45, 34.8]])
d.commit()
