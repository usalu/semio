import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🚜️vehicles/city-car")
for sid, z in (("tail-lamp-l", 0.61), ("tail-lamp-r", -0.61)):
    s = d.solid(50, sid)
    d.set(d.field(s, "transform"), "translation", [0.64, -0.028, z])
d.commit()
