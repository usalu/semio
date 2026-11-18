"""Tower crane 1:500: the tower head members carry their true section diameter (leg 0.1 m, base bar 0.08 m, tie 0.075 m): an unfilled member whose hull encloses the volume counts as a filled shape of its hull width."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/tower-crane")
for v in ("elevation", "front", "plan"):
    for x in d.val("asset", "representations", "500", "geometry", v):
        i = x["id"]
        if not i.startswith("tower-head-"): continue
        dia = 0.1 if "leg" in i else 0.08 if "frame" in i else 0.075
        d.setv(sp.child(sp.child(d.shape(500, v, i), "section"), "d"), dia)
d.commit()
