"""Oil pumpjack 1:50: terminal box sunk 10 mm into the motor (no hairline crack between motor top and box), elevation motor with the square corners of its loft."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/oil-pumpjack")
n = d.solid(50, "terminal-box")
d.setv(sp.child(n, "size"), [0.3, 0.21, 0.3]); d.setv(sp.child(sp.child(n, "transform"), "translation"), [0.0, 0.545, 0.0])
for v, y in (("elevation", 0.44), ("front", 1.34)):
    n = d.shape(50, v, "terminal-box"); d.setv(sp.child(n, "y"), y); d.setv(sp.child(n, "h"), 0.21)
n = d.shape(50, "elevation", "motor"); o = d.obj(n)
if "r" in o: d.setv(sp.child(n, "r"), 0)
d.commit()
