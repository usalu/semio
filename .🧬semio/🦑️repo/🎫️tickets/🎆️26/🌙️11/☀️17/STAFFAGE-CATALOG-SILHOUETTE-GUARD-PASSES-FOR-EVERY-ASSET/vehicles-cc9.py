import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🚜️vehicles/city-car")
for sid in ("quarter-l", "quarter-r"):
    s = d.solid(50, sid)
    p = d.field(d.field(s, "profile"), "1")
    d.setv(p, [-1.44, 1.43])
d.commit()
