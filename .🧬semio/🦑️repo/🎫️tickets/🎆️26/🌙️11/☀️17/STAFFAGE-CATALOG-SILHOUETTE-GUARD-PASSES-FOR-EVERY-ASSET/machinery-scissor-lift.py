"""Scissor lift: vent outlet protrudes 5 mm like at every scale (extent 1.855 m of 1.85 m), chassis bays end at the top of their boxes (0.40 m) so no see-through slit remains above them."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/scissor-lift")
for sc in (500, 200, 100, 50):
    try: n = d.solid(sc, "motor-vent-outlet")
    except KeyError: print("no vent", sc); continue
    p = d.obj(n)["profile"]
    d.setv(sp.child(n, "profile"), [p[0], [p[1][0] if False else 0.005, p[1][1]]] if False else [p[0], [0.005, p[1][1]]])
    try: c = d.solid(sc, "chassis-core")
    except KeyError: continue
    o = d.obj(c)
    if "holes" in o:
        d.setv(sp.child(c, "holes"), [[[x, 0.4 if y == 0.43 else y] for x, y in h] for h in o["holes"]])
d.commit()
