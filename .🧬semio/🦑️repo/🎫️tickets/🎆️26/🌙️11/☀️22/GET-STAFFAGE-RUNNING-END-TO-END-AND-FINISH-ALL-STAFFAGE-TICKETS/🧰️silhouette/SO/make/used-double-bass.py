"""🎻️ Writes the repair ops of `used-double-bass`: at 1:50, 1:100 and 1:200 the neck sleeve of the cover is drawn in the plan as lateral slices that follow the width of its loft (centre 0.11 m beside the bone, half-width `ry` of the sections) instead of one block of 0.27 m, whose square end stood 60 to 74 mm outside the rounded sleeve; the side elevation is unchanged; at 1:100 and 1:200 the widths step by 0.15 mm on paper, and no slice is shorter than that, so no slit narrower than the print gap opens between two slices. At 1:50 the heel of the neck inside the cover, which that block had hidden, ends its full width where the cover band ends and goes on at the width of the neck (it stood 26 mm outside the body)."""
import math, os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, shape, solid, write
a = asset("used-double-bass")
ops = []
for scale, step in (("50", 0.008), ("100", 0.015), ("200", 0.03)):
    sections = solid(a, scale, "cover-sleeve")["sections"]
    pieces = []
    for p, q in zip(sections, sections[1:]):
        n = max(1, math.ceil(abs(q["ry"] - p["ry"]) / step))
        if scale != "50":
            n = max(1, min(n, math.floor((q["at"][0] - p["at"][0]) / step)))
        for k in range(n):
            t0, t1 = k / n, (k + 1) / n
            lerp = lambda key, t: p[key] + (q[key] - p[key]) * t
            u0, u1 = p["at"][0] + (q["at"][0] - p["at"][0]) * t0, p["at"][0] + (q["at"][0] - p["at"][0]) * t1
            half = lerp("ry", (t0 + t1) / 2)
            if scale != "50":
                half = round(half / step) * step
            centre = p["at"][2]
            pieces.append(([[round(u0, 6), round(-lerp("rx", t0), 6)], [round(u1, 6), round(-lerp("rx", t1), 6)], [round(u1, 6), round(lerp("rx", t1), 6)], [round(u0, 6), round(lerp("rx", t0), 6)]], round(centre - half, 6), round(centre + half, 6)))
    for view in ("plan",):
        base = {k: v for k, v in shape(a, scale, view, "cover-sleeve").items() if k not in ("points", "solid")}
        previous = None
        for i, (points, z0, z1) in enumerate(pieces):
            sid = "cover-sleeve" if i == 0 else f"cover-sleeve-{i}"
            piece = {**base, "id": sid, "points": points, "solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}}
            ops.append({"op": "shape", "scale": scale, "view": view, "id": sid, "shape": piece} if i == 0 else {"op": "shape-add", "scale": scale, "view": view, "after": previous, "shape": piece})
            previous = sid
HEEL = [[1.085, -0.045], [1.1, -0.044], [1.13, -0.042], [1.1414, -0.04143], [1.1414, 0.04143], [1.13, 0.042], [1.1, 0.044], [1.085, 0.045]]
for view in ("plan",):
    heel = shape(a, "50", view, "neck-heel")
    ops.append({"op": "shape-set", "scale": "50", "view": view, "id": "neck-heel", "set": {"points": HEEL}})
    ops.append({"op": "shape-add", "scale": "50", "view": view, "after": "neck-heel", "shape": {**heel, "id": "neck-heel-neck", "points": [HEEL[3], [1.17, -0.04], [1.17, 0.04], HEEL[4]], "solid": {**heel["solid"], "z1": 0.013}}})
write("used-double-bass", ops)
