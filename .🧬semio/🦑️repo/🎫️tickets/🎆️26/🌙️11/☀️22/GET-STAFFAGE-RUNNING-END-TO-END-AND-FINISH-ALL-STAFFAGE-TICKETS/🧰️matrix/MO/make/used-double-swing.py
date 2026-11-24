"""🛝️ Writes the repair ops of `used-double-swing`: the bands 1:200 and 1:500 draw every chain as one stroke, and in the front and the plan that stroke was flat, so the swung seat (pose `in-use`, 30 degrees) left its chains hanging plumb in the plan, 472 mm from where the side elevation shows them. Each chain stroke becomes a solid in the front and the plan of the two bands, as the finer bands have it: the stroke of the side elevation, as thick as the chain of the body (21 mm), swept across over the same 21 mm at the lateral position the front draws, which runs from the hanger to the seat."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, extruded, like, op, shape, write, GEOMETRY
HALF = 0.0105
a = asset("used-double-swing")
LENGTH = {"chain-flat-l": 1.835, "chain-flat-r": 1.835, "chain-cradle-l": 1.35, "chain-cradle-r": 1.35}
ops = []
for k in ("200", "500"):
    for bone, length in LENGTH.items():
        sid = f"{bone}-links"
        u0, u1 = sorted(p[0] for p in shape(a, k, "elevation", sid)["points"])
        front = shape(a, k, "front", sid)
        if "solid" in front:
            solid = front["solid"]
        else:
            (z0, y0), (z1, y1) = sorted(front["points"], key=lambda p: -p[1])
            at = lambda u: z0 + (z1 - z0) * (u - u0) / (u1 - u0)
            solid = extruded("w", at(0) - HALF, at(0) + HALF, (at(length) - HALF, at(length) + HALF))
        for view in ("front", "plan"):
            ops.append(op(k, view, sid, like(shape(a, k, view, sid), drop=GEOMETRY, kind="poly", points=[[u0, -HALF], [u1, -HALF], [u1, HALF], [u0, HALF]], closed=True, solid=solid)))
write("used-double-swing", ops)
