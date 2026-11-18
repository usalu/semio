"""Oil pumpjack 1:50: front legs drawn as the hull of the leg loft (they reach down to 0.38 m into the skid, 23 mm beyond the beam), from machinery-hull.ts."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/oil-pumpjack")
H = [[-0.9726, 0.4426], [-0.9691, 0.4096], [-0.9573, 0.4011], [-0.8403, 0.381], [-0.8285, 0.3855], [-0.101, 4.1399], [-0.1038, 4.1678], [-0.1132, 4.1749], [-0.2068, 4.1909], [-0.2162, 4.187]]
d.setv(sp.child(d.shape(50, "front", "leg-right"), "points"), H)
d.setv(sp.child(d.shape(50, "front", "leg-left"), "points"), [[-x, y] for x, y in reversed(H)])
d.commit()
