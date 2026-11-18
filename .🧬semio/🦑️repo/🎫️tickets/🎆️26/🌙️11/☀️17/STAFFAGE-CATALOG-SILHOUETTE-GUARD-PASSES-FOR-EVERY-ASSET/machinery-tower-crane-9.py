"""Tower crane: the mast foot tapers to 2.0 m on both lateral axes, so the elevation top edge is 2.0 m wide too."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/tower-crane")
for sc in (500, 200, 100, 50):
    d.setv(sp.child(d.shape(sc, "elevation", "mast-foot"), "points"), [[-1.5, 1], [1.5, 1], [1, 1.5], [-1, 1.5]])
d.commit()
