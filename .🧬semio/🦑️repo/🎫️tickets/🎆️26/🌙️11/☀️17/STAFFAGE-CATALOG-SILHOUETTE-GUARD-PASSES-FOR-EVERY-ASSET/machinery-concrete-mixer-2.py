"""Concrete mixer truck 1:200, 1:100, 1:50: hopper drawn as the convex hull of the funnel loft (drum frame), from machinery-hull.ts."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/concrete-mixer-truck")
HULL = {
    200: [[6.09, -0.24], [5.914, 0.1793], [5.562, 0.5095], [5.386, 0.5561], [5.21, 0.4659], [5.21, 0.4061], [5.386, -0.1547], [5.562, -0.3283], [6.09, -0.3]],
    100: [[6.09, -0.24], [5.9433, 0.1518], [5.5033, 0.5646], [5.3567, 0.5411], [5.21, 0.4659], [5.21, 0.4061], [5.3567, -0.0613], [5.5033, -0.332], [6.09, -0.3]],
    50: [[6.09, -0.24], [5.9922, 0.1059], [5.5033, 0.5646], [5.4056, 0.5661], [5.21, 0.4659], [5.21, 0.4061], [5.4056, -0.2169], [5.5033, -0.332], [6.09, -0.3]],
}
for sc, pts in HULL.items():
    d.setv(sp.child(d.shape(sc, "elevation", "hopper"), "points"), pts)
d.commit()
