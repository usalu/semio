"""🏛️ Ops writer of `reclaimed-cast-iron-column`: the whole asset turns by 90 degrees when the column is set up, the bearers tip over and the slings swing up by 55 degrees when the load is taken up. At 1:500 the front and the plan did not draw the slings (front 492 mm below, plan 841 mm shorter than the side elevation): each sling is added there as a stroke as a solid, a sliver about its side elevation stroke over the 80 mm the two legs span. The bearers were flat in front and plan at every scale: the plan rectangle becomes the side elevation section swept laterally, the front outline with its V notch is swept along the bearer, and the grain strokes of the plan of 1:50 become ribbons on the top face: `python3 make/reclaimed-cast-iron-column.py` writes its ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("reclaimed-cast-iron-column")
ops = solidify(a, {b["id"] for b in a["rig"]["bones"]}, quiet=True, loose=0.999)
frames = rest(a)
for scale in scales(a):
    for cradle in ("cradle-a", "cradle-b"):
        x = frames[cradle][0]
        ops.append(own_sweep(a, scale, "front", cradle, x - 0.08, x + 0.08))
for cradle in ("cradle-a", "cradle-b"):
    x = frames[cradle][0]
    for i in range(1, 5):
        s = shape(a, "50", "plan", f"{cradle}-grain-{i}")
        if "solid" in s:
            ops.append(op("50", "plan", s["id"], s))
            continue
        ops.append(op("50", "plan", s["id"], sliver(s, [(px - x, py) for px, py in s["points"]], -0.002, 0.0, sweep="u")))
for sling in ("sling-a", "sling-b"):
    stroke = sliver(shape(a, "500", "elevation", sling), shape(a, "500", "elevation", sling)["points"], -0.04, 0.04)
    for view in ("front", "plan"):
        ops.append({"op": "shape-add", "scale": "500", "view": view, "after": "cradle-b", "shape": like(stroke, z=0.3)})
write("reclaimed-cast-iron-column", ops)
