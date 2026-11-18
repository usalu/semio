"""Concrete mixer truck 1:50: the chute side walls end on the flat floor (underside -0.14 m at both ends), in the volume and the three drawings."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/concrete-mixer-truck")
PTS = [[0, -0.14], [2.4, -0.14], [2.4, 0.14], [0, 0.14]]
d.setv(sp.child(d.solid(50, "chute-wall"), "profile"), [[0.0, -0.14], [2.4, -0.14], [2.4, 0.14], [0.0, 0.14]])
for v in ("elevation", "front", "plan"): d.setv(sp.child(d.shape(50, v, "chute-main"), "points"), PTS)
d.commit()
