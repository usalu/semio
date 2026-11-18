"""Excavator 1:50: pins, bosses, eyes and feet (revolves about the lateral axis, drawn in the elevation as ellipses with their lateral extent) also in the front and the plan; hood grille side slab at its true lateral place; hood top slab in the elevation."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/excavator")
IDS = ["boom-pin", "boom-cyl-foot", "boom-root-boss", "boom-tip-boss", "boom-rod-eye", "stick-cyl-foot", "stick-root-boss", "stick-tip-boss", "stick-rod-eye", "bucket-cyl-foot", "bucket-pin", "bucket-ear"]
for v in ("front", "plan"):
    anchor = d.shape(50, v, "boom-body")
    for sid in reversed(IDS):
        d.insert_after(anchor, d.obj(d.shape(50, "elevation", sid)))
g = d.shape(50, "elevation", "hood-grille")
d.setv(sp.child(sp.child(g, "solid"), "z0"), -1.27); d.setv(sp.child(sp.child(g, "solid"), "z1"), -1.25)
o = dict(d.obj(g)); o["id"] = "hood-grille-top"; o["d"] = "M -1.55,1.25 L -0.55,1.25 L -0.55,1.27 L -1.55,1.27 Z"; o["solid"] = {"kind": "extruded", "sweep": "w", "z0": -0.9, "z1": 0.9}
d.insert_after(g, o)
d.commit()
