"""Tower crane 1:500: the skip gate under the skip is drawn in the elevation and the front like at 1:200."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/tower-crane")
src = D("🏗️machinery/tower-crane")
for v in ("elevation", "front"):
    c = src.obj(src.shape(200, v, "skip-gate")); c["role"] = "body"
    d.insert_after(d.shape(500, v, "skip-body"), c)
d.commit()
