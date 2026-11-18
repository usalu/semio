"""Oil pumpjack 1:100 and 1:50: the horsehead drawn with the notch between the face corner and the slanted cheek edge (face plateau stands above the cheek line), from the per-solid hulls of machinery-hull.ts."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/oil-pumpjack")
def horse(x, ycheek):
    top = [[-2.9886, -0.2615], [-2.9544, -0.5209], [-2.8978, -0.7765], [-2.8191, -1.0261], [x, -1.0261], [x, -ycheek], [-2.1, -0.45]]
    bot = [[-2.1, 0.45], [x, ycheek], [x, 1.0261], [-2.8191, 1.0261], [-2.8978, 0.7765], [-2.9544, 0.5209], [-2.9886, 0.2615], [-3, 0]]
    return top + bot
for sc, x, y in ((100, -2.7691, 0.9861), (50, -2.7791, 0.9941)):
    d.setv(sp.child(d.shape(sc, "elevation", "horsehead"), "points"), horse(x, y))
d.commit()
