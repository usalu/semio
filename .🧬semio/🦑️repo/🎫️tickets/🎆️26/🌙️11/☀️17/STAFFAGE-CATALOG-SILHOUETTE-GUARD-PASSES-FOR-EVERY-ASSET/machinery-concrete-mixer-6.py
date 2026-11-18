"""Concrete mixer truck 1:50: mirror arm starts 40 mm further forward (x 4.34 m) in the volume and the three drawings, so its slanted edge leaves the cab front lower."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/concrete-mixer-truck")
n = d.solid(50, "mirror-arm"); p = d.obj(n)["profile"]; p[0] = [4.34, p[0][1]]
d.setv(sp.child(n, "profile"), p)
for v, ids in (("elevation", ("mirror-arm",)), ("plan", ("mirror-arm-left", "mirror-arm-right"))):
    for sid in ids:
        n = d.shape(50, v, sid); pts = d.obj(n)["points"]
        if v == "elevation": pts[0] = [4.34, pts[0][1]]
        else: pts = [[4.34 if x == 4.3 else x, y] for x, y in pts]
        d.setv(sp.child(n, "points"), pts)
d.commit()
