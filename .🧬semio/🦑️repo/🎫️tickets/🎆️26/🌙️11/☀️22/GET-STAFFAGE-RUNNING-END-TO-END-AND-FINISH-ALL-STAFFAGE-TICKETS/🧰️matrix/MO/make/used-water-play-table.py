"""💧️ Writes the repair ops of `used-water-play-table`: maintenance swings the Archimedes screw 40 degrees up, almost level. The plan drew no foot block of the screw mount, because the inclined tube covers it in service, and the front drew it flat, so the plan of the lifted screw ended 95 mm behind the foot the side elevation shows beyond the tube end. The foot becomes one solid in the front and the plan of every band: its side elevation outline swept over the width the front draws; in the plan it is painted under the tube and comes out beyond its end when the screw is lifted."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, extruded, geometry, lateral, like, op, scales, shape, write, GEOMETRY
a = asset("used-water-play-table")
ops = []
for k in scales(a):
    z0, z1 = lateral(a, k, "front", "foot")
    solid = {**geometry(shape(a, k, "elevation", "foot")), "solid": extruded("w", z0, z1)}
    front = shape(a, k, "front", "foot")
    ops.append(op(k, "front", "foot", like(front, drop=GEOMETRY, **solid)))
    plan = [s["id"] for s in a["representations"][k]["geometry"]["plan"] if s["id"] != "foot"]
    at = plan.index("tube")
    under = round(shape(a, k, "plan", "tube")["z"] - 0.05, 4)
    ops.append({"op": "shape-add", "scale": k, "view": "plan", **({"after": plan[at - 1]} if at > 0 else {}), "shape": like(front, drop=GEOMETRY, z=under, **solid)})
write("used-water-play-table", ops)
