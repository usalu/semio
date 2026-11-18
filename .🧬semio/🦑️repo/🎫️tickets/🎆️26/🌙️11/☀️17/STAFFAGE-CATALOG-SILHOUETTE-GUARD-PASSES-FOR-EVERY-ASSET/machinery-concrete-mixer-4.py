"""Concrete mixer truck 1:50: the drum seam line ends 15 mm inside the drum outline, so its pen does not stand out of the silhouette."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/concrete-mixer-truck")
d.setv(sp.child(d.shape(50, "plan", "drum-seam"), "points"), [[-1.0109, -1.105], [-1.0109, 1.105]])
d.setv(sp.child(d.shape(50, "elevation", "drum-seam"), "points"), [[3, -1.105], [3, 1.105]])
n = d.shape(50, "front", "drum-seam")
d.setv(sp.child(n, "rx"), 1.105); d.setv(sp.child(n, "ry"), 1.0717)
d.commit()
