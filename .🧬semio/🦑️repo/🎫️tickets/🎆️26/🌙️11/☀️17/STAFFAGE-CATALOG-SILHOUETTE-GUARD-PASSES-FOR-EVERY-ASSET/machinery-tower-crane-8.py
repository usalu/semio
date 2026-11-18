"""Tower crane: a front shape without a solid is placed in the rest pose and carried by its bone: the skip, lowered 21 m by the pose, gets its trapezoid at the rest height (36.8 m to 35.8 m)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/tower-crane")
for sc in (500, 200, 100, 50):
    d.setv(sp.child(d.shape(sc, "front", "skip-body"), "points"), [[-0.7, 36.8], [0.7, 36.8], [0.45, 35.8], [-0.45, 35.8]])
d.commit()
