"""🧱️ Ops writer of `reclaimed-cobblestone-bag`: the eight laid sett patches turn by 180 degrees about the vertical between the bag and the bed; halfway the plan shows them across while the side elevation and the front drew flat rectangles that became a line. The side elevation and front rectangle of every patch becomes the outline of the patch (the convex hull of its plan path) swept over the sett height, the same rectangle at rest: `python3 make/reclaimed-cobblestone-bag.py` writes its ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *
a = asset("reclaimed-cobblestone-bag")
frames, ops = rest3(a), []
for scale in scales(a):
    for p in [f"{side}{i}" for side in "rl" for i in range(1, 5)]:
        x, y, z, g = frames[f"patch-{p}"]
        runs = segments(shape(a, scale, "plan", f"pl-{p}")["d"])
        ring = [[round(u, 6), round(w, 6)] for u, w in hull([(px - x, pz - z) for run in runs for px, pz in run])]
        for view, name in (("elevation", f"el-{p}"), ("front", f"fr-{p}")):
            ops.append(op(scale, view, name, like(shape(a, scale, view, name), drop=GEOMETRY, kind="poly", points=ring, closed=True, solid=extruded("v", 0, 0.1))))
write("reclaimed-cobblestone-bag", ops)
