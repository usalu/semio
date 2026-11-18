"""Oil pumpjack 1:50: removes the duplicate crank shaft shapes inserted without solid in the front and plan (the original ones with the extruded solid stay)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/oil-pumpjack")
for v in ("front", "plan"):
    for n in d.at("asset", "representations", "50", "geometry", v)["items"]:
        o = d.obj(n)
        if o["id"] == "crank-hub" and "solid" not in o: d.delete_item(n)
d.commit()
