import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🧱️components/external-venetian-blind")
for sc in (100, 200):
    for g in ("guide-left", "guide-right"):
        d.set(d.solid(sc, g), "slender", False)
for i in range(17):
    d.set(d.shape(100, "elevation", f"slat-{i}"), "role", "texture")
d.commit()
