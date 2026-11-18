"""Oil pumpjack 1:50: the elevation polished rod is drawn 25 mm longer at the top, as long as the slender volume stroke (square cap of half the stroke width) is."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/oil-pumpjack")
d.setv(sp.child(d.shape(50, "elevation", "polished-rod"), "h"), 3.145)
d.commit()
