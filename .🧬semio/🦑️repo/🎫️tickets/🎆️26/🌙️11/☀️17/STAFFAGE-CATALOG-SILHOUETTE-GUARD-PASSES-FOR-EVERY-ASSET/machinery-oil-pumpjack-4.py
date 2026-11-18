"""Oil pumpjack 1:50: the front skid is drawn as the H-section beams they are (flanges 15 mm, web 12 mm, the inner pocket a hole closed by the cross members), the cross members and the deck, instead of one solid block."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/oil-pumpjack")
def rect(x0, x1, y0, y1): return f"M {x0} {y0} L {x1} {y0} L {x1} {y1} L {x0} {y1} Z"
parts = [rect(-0.75, 0.75, 0.3, 0.54), rect(-1.0, 1.0, 0.54, 0.55)]
for s in (1, -1):
    for z0, z1, y0, y1 in ((0.75, 0.95, 0.3, 0.315), (0.75, 0.95, 0.525, 0.54), (0.844, 0.856, 0.315, 0.525)):
        a, b = sorted((s * z0, s * z1)); parts.append(rect(a, b, y0, y1))
n = d.shape(50, "front", "skid"); o = d.obj(n)
out = {}
for k, x in o.items():
    if k == "kind": out["kind"] = "path"; out["d"] = " ".join(parts)
    elif k not in ("points", "closed"): out[k] = x
d.replace_obj(n, out)
d.commit()
