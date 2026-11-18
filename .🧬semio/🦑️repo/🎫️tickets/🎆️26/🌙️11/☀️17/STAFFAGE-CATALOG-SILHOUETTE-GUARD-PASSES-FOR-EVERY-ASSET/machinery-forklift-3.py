"""Forklift 1:100 and 1:50: the wheel arches of the chassis are 10 mm inside the tyre radius, so no hairline crack runs between the arch polygon and the tyre."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/forklift")
for sc in (100, 50):
    n = d.solid(sc, "chassis-body"); p = d.obj(n)["profile"]
    out = []
    for i, (x, y) in enumerate(p):
        if 9 <= i <= 42: cx, cy, f = 0.0, 0.29, 0.28 / 0.291
        elif 43 <= i <= 58: cx, cy, f = -1.7, 0.23, 0.22 / 0.231
        else: out.append([x, y]); continue
        out.append([round(cx + (x - cx) * f, 5), round(cy + (y - cy) * f, 5)])
    d.setv(sp.child(n, "profile"), out)
d.commit()
