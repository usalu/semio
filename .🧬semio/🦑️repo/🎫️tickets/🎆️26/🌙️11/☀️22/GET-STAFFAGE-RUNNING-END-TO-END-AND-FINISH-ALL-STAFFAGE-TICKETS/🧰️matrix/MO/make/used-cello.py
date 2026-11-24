"""🎻️ Writes the repair ops of `used-cello`: the pose `carried` leans the case 5 degrees ahead on the back, and the plan drew the two shells as flat outlines at half height, so the head end of the lid, which the side elevation shows 0.22 m ahead, stayed 42 mm short in the plan. The plan of every band gets the neck end of the lid (from 0.82 m along the lid) as a solid: its side elevation outline swept over the 0.212 m the front shows there. It lies under the outline of the lid shell while the case stands and leans out ahead of it when the case is carried."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, extruded, like, scales, shape, write, GEOMETRY
a = asset("used-cello")
FROM, HALF = 0.82, 0.106
ops = []
for k in scales(a):
    side = shape(a, k, "elevation", "case-lid-side")
    neck = [p for p in side["points"] if p[0] >= FROM - 1e-9]
    assert sum(1 for p in neck if abs(p[0] - FROM) < 1e-9) == 2, k
    shell = shape(a, k, "plan", "case-lid-shell")
    piece = like(shell, drop=GEOMETRY, id="case-lid-side", z=round(shell["z"] - 0.05, 4), kind="poly", points=neck, closed=True, solid=extruded("w", -HALF, HALF))
    ops.append({"op": "shape-add", "scale": k, "view": "plan", "after": "case-back-shell", "shape": piece})
write("used-cello", ops)
