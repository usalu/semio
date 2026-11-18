"""Concrete mixer truck 1:500: the elevation truck body hangs its cab down to 0.42 m from x 2.4 m like the cab extrusion of the volume."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/concrete-mixer-truck")
n = d.shape(500, "elevation", "truck-body"); p = d.obj(n)["points"]
p = [[2.4, y] if x == 4.25 else [x, y] for x, y in p]
d.setv(sp.child(n, "points"), p)
d.commit()
