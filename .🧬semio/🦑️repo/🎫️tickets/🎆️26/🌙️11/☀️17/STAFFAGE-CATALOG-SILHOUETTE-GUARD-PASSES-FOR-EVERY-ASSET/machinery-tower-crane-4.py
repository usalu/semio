"""Tower crane 1:500 front: the swept pendant paths are as wide as the volume pendants (their pen: 0.065 m and 0.14 m), the sweep of a swept path is its drawn width."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/tower-crane")
for sid, z0, z1 in (("pendant-fore-1", 0.095, 0.16), ("pendant-fore-2", -0.19, -0.05), ("pendant-back", -0.0325, 0.0325)):
    s = sp.child(d.shape(500, "front", sid), "solid")
    d.setv(sp.child(s, "z0"), z0); d.setv(sp.child(s, "z1"), z1)
d.commit()
