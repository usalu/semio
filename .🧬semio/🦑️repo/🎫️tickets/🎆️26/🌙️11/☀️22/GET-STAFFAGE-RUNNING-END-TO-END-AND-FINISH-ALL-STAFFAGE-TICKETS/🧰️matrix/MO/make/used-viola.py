"""🎻️ Writes the repair ops of `used-viola`: the plan drew only the top of the viola body, which covers the ribs and the back while the viola lies in its case. Carried, the case stands on end and the plan looks along the viola: the top is then a band 20 mm thick, and the contact anchor `collarbone` on the back lay 19 mm off it. The plan of every band gets the ribs and the back as the solids they are: the outline of the top swept over the thickness the side elevation draws for each, painted under the top as layers of their own (z 0.8 and 0.9 between the case bed and the top), so nothing changes while the viola lies flat."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, box, extruded, geometry, like, scales, shape, write, GEOMETRY
a = asset("used-viola")
ops = []
for k in scales(a):
    plan = [s["id"] for s in a["representations"][k]["geometry"]["plan"] if s["id"] not in ("viola-back", "viola-ribs")]
    after = plan[plan.index("viola-top") - 1]
    top = shape(a, k, "plan", "viola-top")
    for sid, z in (("viola-back", 0.8), ("viola-ribs", 0.9)):
        b = box(shape(a, k, "elevation", sid))
        ops.append({"op": "shape-add", "scale": k, "view": "plan", "after": after, "shape": like(shape(a, k, "front", sid), drop=GEOMETRY, z=z, **geometry(top), solid=extruded("v", b[2], b[3]))})
        after = sid
write("used-viola", ops)
