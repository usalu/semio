"""Forklift 1:50: pallet course lines start 10 mm inside the goods so their pen does not close the crack beside the chassis chamfer."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/forklift")
d.setv(sp.child(d.shape(50, "elevation", "goods-courses"), "d"), "M 0.03,0.3 L 1.17,0.3 M 0.03,0.45 L 1.17,0.45 M 0.03,0.6 L 1.17,0.6 M 0.03,0.75 L 1.17,0.75 M 0.03,0.9 L 1.17,0.9")
d.commit()
