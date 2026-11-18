"""Diesel generator 1:50: the elevation silencer stadium is drawn with 48 arc segments so its ends follow the capsule where it meets the roof edge."""
import sys, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/diesel-generator")
R, CY, X0, X1, N = 0.285, 2.865, -1.725, -0.075, 6
pts = [[round(X1 + R * math.cos(-math.pi / 2 + math.pi * k / N), 4), round(CY + R * math.sin(-math.pi / 2 + math.pi * k / N), 4)] for k in range(N + 1)]
pts += [[round(X0 + R * math.cos(math.pi / 2 + math.pi * k / N), 4), round(CY + R * math.sin(math.pi / 2 + math.pi * k / N), 4)] for k in range(N + 1)]
d.setv(sp.child(d.shape(50, "elevation", "silencer"), "points"), pts)
d.commit()
