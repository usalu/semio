"""🍖️ Writes the repair ops of `charcoal-grill`: the pose `wheeled-tilted` tips the whole grill 20 degrees back onto its wheels, and the three legs were flat plan shapes carried at one height, so the plan kept the front foot 95 mm behind the place the side elevation and the contact anchor `leg-front` give it. The three legs become the inclined tubes they are in the plan of every band: the section of the leg in the (v, w) plane of the bowl bone (its height and lateral run, as the front view draws it) swept along u, the sweep position running with the height from the foot to the socket as the side elevation draws it. The front leg lies in the centre plane and is its side elevation outline swept across its own thickness. A leg that a coarse band draws as one stroke is swept over the 25 mm of the tube, so that it stays a stroke as wide as the leg of the body."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, extruded, like, op, scales, shape, solid_of, write, GEOMETRY
a = asset("charcoal-grill")
LENGTH = 0.74
HAIR = 0.0125
ops = []
for k in scales(a):
    for sid, twin in (("leg-front", "leg-front"), ("leg-left", "leg-wheel"), ("leg-right", "leg-wheel")):
        e = shape(a, k, "elevation", twin)
        (u0, v0), (u1, v1) = ((e["x0"], e["y0"]), (e["x1"], e["y1"])) if e["kind"] == "capsule" else e["points"]
        if v0 > v1:
            u0, v0, u1, v1 = u1, v1, u0, v0
        p = shape(a, k, "plan", sid)
        if "solid" in p:
            ends = sorted(([p["x0"], p["y0"]], [p["x1"], p["y1"]]) if p["kind"] == "capsule" else p["points"])
            w0, w1 = ends[0][1], ends[1][1]
        else:
            (xa, wa), (xb, wb) = ((p["x0"], p["y0"]), (p["x1"], p["y1"])) if p["kind"] == "capsule" else p["points"]
            w0, w1 = (wa, wb) if abs(xa - u0) < abs(xb - u0) else (wb, wa)
        r = p.get("r0", HAIR)
        if sid == "leg-front":
            ops.append(op(k, "plan", sid, solid_of(p, e, extruded("w", -r, r))))
            continue
        slope = (u1 - u0) / (v1 - v0)
        at = lambda v: u0 + slope * (v - v0)
        solid = extruded("u", at(0) - r, at(0) + r, (at(LENGTH) - r, at(LENGTH) + r))
        geom = {"kind": "capsule", "x0": v0, "y0": w0, "x1": v1, "y1": w1, "r0": r, "r1": r} if p["kind"] == "capsule" else {"kind": "poly", "points": [[v0, w0], [v1, w1]], "closed": False}
        ops.append(op(k, "plan", sid, like(p, drop=GEOMETRY, **geom, solid=solid)))
write("charcoal-grill", ops)
