"""🏀️ Writes the repair ops of `used-basketball-hoop`: the pose `maintenance` lowers the pole 50 degrees about the foot hinge. The backboard stood in the plan as a flat strip 43 mm deep and the chain net as flat strokes, so the plan kept the board 345 mm and the net 185 mm short of the side elevation. In the plan every part of the board becomes its side elevation outline swept over its width. Every chain of the net becomes a ribbon as wide as its body solid (7 mm) in the front and in the plan: the stroke of the side elevation, placed laterally where the front draws it, so the net is one cone of chains in the three views at every angle; the band 1:500 draws the net as its cone, a body of revolution about the net bone. The pole and the arm (tubes), the foot flange, the end plate and the strut of the arm were flat in the front and the plan too (the lowered pole stayed a circle in the plan); the tubes become bodies of revolution about their bones, the plates and the strut their side elevation outline swept over their width."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, extruded, has, like, op, prism, scales, turned, segments, shape, write, GEOMETRY
a = asset("used-basketball-hoop")
if not has(a, 50, "front", "chain-net"):
    sys.exit("used-basketball-hoop: the net is chains already, ops kept")
LENGTH = 0.4
WIRE = 0.0035
ops = []
for k in scales(a):
    for sid in ("backboard-plate", "frame-bottom", "frame-top", "frame-side", "frame-mid", "board-frame", "backboard"):
        if has(a, k, "plan", sid):
            ops.append(prism(a, k, "plan", sid))
    for view in ("front", "plan"):
        for sid, bone in (("pole-tube", "pole"), ("pole-cap", "pole"), ("arm-tube", "arm")):
            if has(a, k, view, sid):
                ops.append(turned(a, k, view, sid, bone))
        for sid in ("foot-flange", "pole-foot", "arm-end-plate", "arm-strut"):
            if has(a, k, view, sid):
                ops.append(prism(a, k, view, sid, half=0.02415))
    if has(a, k, "elevation", "net-cone"):
        cone = shape(a, k, "elevation", "net-cone")
        for view in ("front", "plan"):
            ops.append(op(k, view, "net-cone", like(shape(a, k, view, "net-cone"), drop=GEOMETRY, kind="poly", points=cone["points"], closed=True, solid={"kind": "revolved", "axisBone": "net", "axis": "u", "axisX": 0, "mirror": True})))
        continue
    side = segments(shape(a, k, "elevation", "chain-net")["d"])
    for view in ("front", "plan"):
        base = shape(a, k, view, "chain-net")
        across = [[p[0] for p in run] for run in segments(shape(a, k, "front", "chain-net")["d"])]
        after = "chain-net"
        for i, ((u0, v0), (u1, v1)) in enumerate(side):
            w0, w1 = across[i]
            along_v = view == "plan" and abs(v1 - v0) < abs(w1 - w0)
            a0, a1, b0, b1 = (w0, w1, v0, v1) if along_v else (v0, v1, w0, w1)
            line = lambda u: b0 + (b1 - b0) * (u - u0) / (u1 - u0)
            piece = like(base, drop=GEOMETRY, id=f"chain-net-{i}", kind="poly", points=[[u0, a0], [u1, a1]], closed=False, solid=extruded("v" if along_v else "w", line(0) - WIRE, line(0) + WIRE, (line(LENGTH) - WIRE, line(LENGTH) + WIRE)))
            ops.append({"op": "shape-add", "scale": k, "view": view, "after": after, "shape": piece})
            after = piece["id"]
        ops.append({"op": "shape-remove", "scale": k, "view": view, "id": "chain-net"})
write("used-basketball-hoop", ops)
