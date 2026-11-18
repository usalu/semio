import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🚜️vehicles/city-car")
s = d.shape(50, "front", "body-shell")
half = [[0.72, 0.2], [0.76, 0.35], [0.765, 0.9], [0.752, 0.98], [0.739, 1.12], [0.718, 1.2], [0.69, 1.3], [0.665, 1.4], [0.65, 1.44], [0.6, 1.495]]
pts = [[0, 0.2]] + half + [[0, 1.495]] + [[-x, y] for x, y in reversed(half)] + [[0, 0.2]]
d.set(s, "points", pts)
d.commit()
