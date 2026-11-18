import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🔌️appliances/shower-head")
pts = []
for s in d.val("asset", "volumes", "200", "body"):
    if s["id"].startswith("hose-"):
        if not pts: pts.append(s["p0"])
        pts.append(s["p1"])
d.set(d.shape(200, "elevation", "hose"), "d", "M " + " L ".join(f"{p[0]} {p[1]}" for p in pts))
d.commit()
