"""Oil pumpjack: gearbox corner rounding of the three drawings equals the cornerRadius of the volume box at each scale, front skid drawn as the two skid beams (z 0.95 m) under the deck (z 1.0 m)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/oil-pumpjack")
JOIN = {500: 0.52, 200: 0.52, 100: 0.535, 50: 0.54}
for sc in (500, 200, 100, 50):
    cr = d.obj(d.solid(sc, "gearbox")).get("cornerRadius", 0)
    for v in ("elevation", "front", "plan"):
        n = d.shape(sc, v, "gearbox"); o = d.obj(n)
        if o.get("r", 0) == cr: continue
        if "r" in o: d.setv(sp.child(n, "r"), cr)
        else:
            out = {}
            for k, x in o.items():
                out[k] = x
                if k == "h": out["r"] = cr
            d.replace_obj(n, out)
    n = d.shape(sc, "front", "skid"); o = d.obj(n); j = JOIN[sc]
    pts = [[-0.95, 0.3], [0.95, 0.3], [0.95, j], [1.0, j], [1.0, 0.55], [-1.0, 0.55], [-1.0, j], [-0.95, j]]
    out = {}
    for k, x in o.items():
        if k == "kind": out["kind"] = "poly"; out["points"] = pts; out["closed"] = True
        elif k not in ("x", "y", "w", "h", "r"): out[k] = x
    d.replace_obj(n, out)
d.commit()
