"""🌬️ Writes the repair ops of `heat-pump-outdoor-unit`: the plan was the cover over the casing at all four scales. At 1:50 it gets the four hinge knuckles that stand 9 mm behind the cover (two on the casing, two on the cover, 0.18 mm on paper), at 1:100 the two knuckles of the cover that its body keeps, 1:200 and 1:500 stay the cover alone (allowlisted as one drawing, D47)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write, GEOMETRY
a = asset("heat-pump-outdoor-unit")
ops = []
for scale in (50, 100):
    lid = shape(a, scale, "plan", "lid")
    for side, z0, z1 in (("l", 0.34, 0.4), ("r", -0.4, -0.34)):
        ops.append({"op": "shape-add", "scale": str(scale), "view": "plan", "after": "casing", "shape": like(lid, drop=GEOMETRY + ("penMm",), id=f"lid-knuckle-{side}", role="detail", z=2.4, tone=0.3, kind="ellipse", cx=0, cy=0, rx=0.009, ry=0.009, solid={"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1})})
casing = shape(a, 50, "plan", "casing")
for side, z in (("l", 0.28), ("r", -0.34)):
    ops.append({"op": "shape-add", "scale": "50", "view": "plan", "after": "casing", "shape": like(casing, drop=GEOMETRY + ("penMm",), id=f"hinge-knuckle-{side}", role="detail", z=2.4, tone=0.3, kind="rect", x=-0.244, y=z, w=0.018, h=0.06)})
ops += [
    {"op": "note", "scale": "50", "decision": "retained", "en": "Plan: the four hinge knuckles behind the cover.", "de": "Draufsicht: die vier Scharnierrollen hinter dem Deckel."},
    {"op": "note", "scale": "100", "decision": "retained", "en": "Plan: the two hinge knuckles of the cover.", "de": "Draufsicht: die zwei Scharnierrollen des Deckels."},
]
write("heat-pump-outdoor-unit", ops)
