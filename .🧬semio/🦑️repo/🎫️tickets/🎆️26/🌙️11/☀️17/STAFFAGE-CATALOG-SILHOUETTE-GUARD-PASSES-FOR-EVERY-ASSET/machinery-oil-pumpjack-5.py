"""Oil pumpjack 1:50: the two skid beams are solid 0.24 m boxes like at 1:200 and 1:100 (the H pockets were closed by the legs and cross members and left hairline holes), and the front skid is the block of the two beams under the deck again."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/oil-pumpjack")
for sid, z in (("skid-beam-l", 0.85), ("skid-beam-r", -0.85)):
    n = d.solid(50, sid); o = d.obj(n); out = {}
    for k, x in o.items():
        if k == "kind": out.update({"kind": "box"})
        elif k == "profile": out.update({"size": [7.2, 0.24, 0.2], "transform": {"translation": [3.6, 0.12, z]}})
        elif k not in ("plane", "from", "to", "holes"): out[k] = x
    d.replace_obj(n, out)
n = d.shape(50, "front", "skid"); o = d.obj(n)
pts = [[-0.95, 0.3], [0.95, 0.3], [0.95, 0.54], [1.0, 0.54], [1.0, 0.55], [-1.0, 0.55], [-1.0, 0.54], [-0.95, 0.54]]
out = {}
for k, x in o.items():
    if k == "kind": out["kind"] = "poly"; out["points"] = pts; out["closed"] = True
    elif k != "d": out[k] = x
d.replace_obj(n, out)
d.commit()
