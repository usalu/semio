"""Forklift 1:50: the chassis front corner (0.45, 0.78) to (0.43, 0.86) m of the elevation and the volume ends 5 mm further forward, so no 2 mm wedge remains beside the pallet goods."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/forklift")
d.setv(sp.child(d.shape(50, "elevation", "chassis-body"), "d"), "M -1.7,0.18 L 0.27,0.18 L 0.4,0.19 L 0.45,0.28 L 0.45,0.78 L 0.44,0.86 L 0.35,0.92 L 0.05,0.96 L -0.02,0.58 L -1.7,0.58 Z")
n = d.solid(50, "chassis-body"); p = d.obj(n)["profile"]
p[5] = [0.44, 0.86]
d.setv(sp.child(n, "profile"), p)
d.commit()
