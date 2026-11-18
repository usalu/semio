import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🚜️vehicles/city-car")
s = d.solid(50, "bonnet-slab")
sec = d.field(d.field(s, "sections"), "0")
d.set(sec, "at", [-0.03, -0.00983, 0.0])
s = d.solid(50, "engine-block")
sec = d.field(d.field(s, "sections"), "0")
d.set(sec, "at", [0.9, 0.89182, 0.0])
d.commit()
