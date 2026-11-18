"""Oil pumpjack: no crank shaft drawing at 1:500 (the volume has none), 1:50 shaft radius 0.2 m, wellhead flange drawn in the three views at 1:50."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/oil-pumpjack")
for v in ("front", "plan"): d.delete_item(d.shape(500, v, "crank-hub"))
for v, key in (("front", ("y", 1.55, "h", 0.4)), ("plan", ("x", 2.5, "w", 0.4))):
    n = d.shape(50, v, "crank-hub")
    d.setv(sp.child(n, key[0]), key[1]); d.setv(sp.child(n, key[2]), key[3])
for v, kw in (("elevation", {"x": -3.2, "y": 0.5, "w": 0.4, "h": 0.03}), ("front", {"x": -0.2, "y": 0.5, "w": 0.4, "h": 0.03}), ("plan", {"cx": -3, "cy": 0, "rx": 0.2, "ry": 0.2})):
    n = d.shape(50, v, "wellhead-valve"); o = d.obj(n)
    o2 = {k: x for k, x in o.items() if k not in ("x", "y", "w", "h", "cx", "cy", "rx", "ry")}
    o2["id"] = "wellhead-flange"; o2.update(kw)
    d.insert_after(n, o2)
d.commit()
