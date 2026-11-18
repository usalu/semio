"""Excavator 1:100: hood rail posts drawn as two pen strokes 20 mm apart in the front (a single 2 mm wide stroke is dropped by the raster), old capsule rods of the boom and the bucket cylinder dropped from the front and the plan (the circular member shapes replace them)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/excavator")
n = d.shape(100, "front", "hood-rail")
d.setv(sp.child(sp.child(n, "solid"), "z0"), -1.16); d.setv(sp.child(sp.child(n, "solid"), "z1"), -1.14)
for v in ("front", "plan"):
    for sid in ("boom-rod-left", "boom-rod-right", "bucket-rod", "stick-rod"):
        try: d.delete_item(d.shape(100, v, sid))
        except KeyError: pass
d.commit()
