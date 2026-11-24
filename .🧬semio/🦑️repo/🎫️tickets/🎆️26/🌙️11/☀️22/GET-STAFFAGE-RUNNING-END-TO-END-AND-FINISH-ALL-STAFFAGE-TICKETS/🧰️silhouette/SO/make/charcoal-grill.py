"""🍖️ Writes the repair ops of `charcoal-grill`: at 1:50 the cap of the front foot is 27 mm across on the 25 mm leg tube instead of 33 mm, so the body from the rim bead to the foot is 0.6064 m wide, within one smallest feature of the declared 0.60 m (it was 0.6095 m); the three views draw the same cap."""
import copy, os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, solid, write
R = 0.0135
a = asset("charcoal-grill")
sections = copy.deepcopy(solid(a, 50, "foot-front")["sections"])
for s in sections:
    s["rx"], s["ry"] = round(R * (0.01679 / 0.0165 if s["at"][1] == 0 else 1), 5), R
G = sections[-1]["rx"]
write("charcoal-grill", [
    {"op": "solid-set", "scale": "50", "id": "foot-front", "set": {"sections": sections}},
    {"op": "shape-set", "scale": "50", "view": "elevation", "id": "foot-front", "set": {"points": [[round(0.3 - G, 5), 0.0], [round(0.3 + G, 5), 0.0], [round(0.2943 + R, 5), 0.03], [round(0.2943 - R, 5), 0.03]]}},
    {"op": "shape-set", "scale": "50", "view": "front", "id": "foot-front", "set": {"points": [[-R, 0.0], [R, 0.0], [R, 0.03], [-R, 0.03]]}},
    {"op": "shape-set", "scale": "50", "view": "plan", "id": "foot-front", "set": {"rx": G, "ry": R}},
])
