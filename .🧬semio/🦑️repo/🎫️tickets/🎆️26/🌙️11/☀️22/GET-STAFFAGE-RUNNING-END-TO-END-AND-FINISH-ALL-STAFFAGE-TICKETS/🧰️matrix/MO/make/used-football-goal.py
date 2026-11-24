"""⚽️ Writes the repair ops of `used-football-goal`: maintenance tips the goal 90 degrees onto its posts. At 1:500 the front drew posts and crossbar as one portal bound to the crossbar bone, so the contact anchors `rest-post-right` and `rest-post-left` lay 880 mm from every shape of their posts, and the net cage was a flat front and plan sheet (1,000 mm and 751 mm from the side elevation when tipped). The portal becomes its three members on their own bones, each a solid: the right post the side elevation outline of post and crossbar end swept over the 80 mm of the tube, the left post the same outline on its bone, the crossbar its section swept along its bone in the front and the plan. The net cage (and the back and top net of the finer bands) becomes its side elevation outline swept over the width of the goal."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, extruded, has, like, op, prism, scales, shape, write, GEOMETRY
a = asset("used-football-goal")
ops = []
for k in scales(a):
    for sid in ("mesh-cage", "back-mesh", "top-mesh"):
        for view in ("front", "plan"):
            if has(a, k, view, sid) and has(a, k, "elevation", sid):
                ops.append(prism(a, k, view, sid))
side = shape(a, 500, "elevation", "post-and-crossbar")
frame = shape(a, 500, "front", "goal-frame" if has(a, 500, "front", "goal-frame") else "crossbar")
paint = like(frame, drop=GEOMETRY)
bar = {"kind": "poly", "points": [[0, -0.04], [5.16, -0.04], [5.16, 0.04], [0, 0.04]], "closed": True, "solid": extruded("w", -2.62, -2.54)}
for sid, bone, z0, z1 in (("post-and-crossbar", "post-right", -2.58, -2.5), ("post-left", "post-left", 2.5, 2.58)):
    ops.append({"op": "shape-add", "scale": "500", "view": "front", "after": "mesh-cage", "shape": {**paint, "id": sid, "bone": bone, "kind": "poly", "points": side["points"], "closed": True, "solid": extruded("w", z0, z1)}})
ops.append({"op": "shape-add", "scale": "500", "view": "front", "after": "post-left", "shape": {**paint, "id": "crossbar", "bone": "crossbar", **bar}})
ops.append({"op": "shape-remove", "scale": "500", "view": "front", "id": "goal-frame"})
ops.append(op("500", "plan", "crossbar", like(shape(a, 500, "plan", "crossbar"), drop=GEOMETRY, **bar)))
write("used-football-goal", ops)
