import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🚜️vehicles/city-car")
s = d.solid(50, "boot-floor")
d.set(s, "size", [0.45, 0.08, 1.24])
d.set(d.field(s, "transform"), "translation", [-1.72, 0.8, 0])
d.commit()
