"""Diesel generator: door line, door handle and end door sit flush inside the container envelope (wall plate at z +-1.22 m, end at x 3.03 m) so the extents equal the declared 6.06 x 2.44 m."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/diesel-generator")
def tr(sc, sid, axis, face):
    n = d.solid(sc, sid); o = d.obj(n); t = o["transform"]["translation"]; s = o["size"]
    t[axis] = round(face - (1 if face > 0 else -1) * s[axis] / 2, 5)
    d.setv(sp.child(sp.child(n, "transform"), "translation"), t)
for sc in (200, 100, 50):
    tr(sc, "door-line", 2, -1.22)
    try: tr(sc, "end-door", 0, 3.03)
    except KeyError: pass
tr(50, "door-handle", 2, -1.22)
d.commit()
