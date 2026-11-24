"""🛁️ Writes the repair ops of `used-bathtub` (ops/used-bathtub.json) from the catalog file before the repair: the front view of 1:50 shows the open 52 mm waste hole as an opening and the rim with its round corners.

The stored bath leans back 10 degrees and the front view looks into it, so the line of sight through the hole passes every band of the wall and of the inside. Each of these bands was a side profile swept across the width, a shape that cannot carry a hole. It becomes a prism swept through its depth (sweep `v`) whose outline lies in the plane of the bath (length by width): the rectangle that projects to the same extent in the leaning front view as the side profile did, with a slot along the sight line (a stadium between the points where the sight line enters and leaves the band, 40 mm in radius so that the pen of its edge stays under the floor sheet). The floor sheet, the nearest shape, is the same kind of prism through its 4 mm with the round hole itself. Hole and outline are one ring, joined by a slit of no width: seen along the sweep the union of the swept edges is the outline with its hole, seen across it the closed band, so the bath lying flat is drawn as before. The dark square that stood for the hole is removed. The rim, four strips with square corners, is one such prism through its 22 mm with the outline of the rim plate of the body (corners of 50 mm outside and 120 mm inside). The line views draw an edge of the rim only where it lies on the edge of another shape of the shell, which gave two loose bars along the straight sides: the first inside band now reaches 2 mm under the rim on both sides, and the inner edge of the rim is a 0.25 mm line of its own (role `texture`, an unfilled outline in the upper half millimetre of the rim along the opening the leaning rim shows), whole in every view."""
import math, os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, write

SLUG, SCALE, VIEW = "used-bathtub", "50", "front"
X0, R, SLOT = 0.35, 0.026, 0.040
a = asset(SLUG)
tub = next(b for b in a["rig"]["bones"] if b["id"] == "tub")
sin, cos = math.sin(math.radians(tub["angleDeg"])), math.cos(math.radians(tub["angleDeg"]))
height = lambda u, v: u * sin + v * cos
shapes = a["representations"][SCALE]["geometry"][VIEW]
floor = next(s for s in shapes if s["id"] == "floor")
y_hole = max(p[1] for p in floor["points"])
r6 = lambda v: round(v, 6)
sight = lambda v: X0 + (v - y_hole) * (-cos / sin)


def stadium(x_a, x_b, r, n=12):
    """💊️ The outline of a slot along the bath between two centres, clockwise from its point nearest the foot end."""
    lo, hi = min(x_a, x_b), max(x_a, x_b)
    foot = [[r6(lo - r * math.cos(math.pi * i / n)), r6(r * math.sin(math.pi * i / n))] for i in range(n // 2 + 1)]
    head = [[r6(hi + r * math.sin(math.pi * i / n)), r6(r * math.cos(math.pi * i / n))] for i in range(n + 1)]
    back = [[r6(lo - r * math.sin(math.pi * i / n)), r6(-r * math.cos(math.pi * i / n))] for i in range(n // 2)]
    return foot + head + back


def keyhole(x_lo, x_hi, z0, z1, hole):
    """🗝️ A rectangle of the plane of the bath with a hole as one ring: along the foot edge to its middle, to the hole, around it and back."""
    return [[r6(x_lo), 0.0], [r6(x_lo), z0], [r6(x_hi), z0], [r6(x_hi), z1], [r6(x_lo), z1], [r6(x_lo), 0.0]] + hole + [hole[0]]


ops = []
for s in shapes:
    solid = s.get("solid")
    if s["id"] == "waste-hole" or s.get("kind") != "poly" or solid is None or solid.get("sweep") != "w" or s["role"] == "texture" or s["bone"] != "tub":
        continue
    hs = [height(*p) for p in s["points"]]
    v0, v1 = min(p[1] for p in s["points"]), max(p[1] for p in s["points"])
    if not (solid["z0"] < -R and solid["z1"] > R and min(hs) < height(X0, y_hole) - R and max(hs) > height(X0, y_hole) + R):
        continue
    x_lo, x_hi = (min(hs) - v1 * cos) / sin, (max(hs) - v0 * cos) / sin
    hole = [[r6(X0 - R * math.cos(2 * math.pi * i / 24)), r6(R * math.sin(2 * math.pi * i / 24))] for i in range(24)] if s["id"] == "floor" else stadium(sight(v0), sight(v1), SLOT)
    ops.append({"op": "shape-set", "scale": SCALE, "view": VIEW, "id": s["id"], "set": {"points": keyhole(x_lo, x_hi, solid["z0"], solid["z1"], hole), "solid": {"kind": "extruded", "sweep": "v", "z0": r6(v0), "z1": r6(v1)}}})
ops.append({"op": "shape-remove", "scale": SCALE, "view": VIEW, "id": "waste-hole"})
rim = next(s for s in a["volumes"][SCALE]["body"] if s["id"] == "rim")
outer, inner = [[p[1], p[0]] for p in rim["profile"]], [[p[1], p[0]] for p in rim["holes"][0]]
old = next(s for s in shapes if s["id"] in ("rim-side-l", "rim"))
ops.append({"op": "shape-add", "scale": SCALE, "view": VIEW, "after": "rim-end-head", "shape": {**{k: v for k, v in old.items() if k not in ("points", "solid")}, "id": "rim", "points": outer + inner[::-1] + [inner[-1], outer[-1]], "solid": {"kind": "extruded", "sweep": "v", "z0": -0.022, "z1": 0.0}}})
def cut(subject, clipper):
    """✂️ The intersection of two convex polygons of the same orientation (Sutherland-Hodgman)."""
    out = subject
    for i, c0 in enumerate(clipper):
        c1 = clipper[(i + 1) % len(clipper)]
        side = lambda p: (c1[0] - c0[0]) * (p[1] - c0[1]) - (c1[1] - c0[1]) * (p[0] - c0[0])
        new = []
        for j, p in enumerate(out):
            q = out[(j + 1) % len(out)]
            if side(p) >= 0:
                new.append(p)
            if (side(p) > 0 and side(q) < 0) or (side(p) < 0 and side(q) > 0):
                k = side(p) / (side(p) - side(q))
                new.append([p[0] + k * (q[0] - p[0]), p[1] + k * (q[1] - p[1])])
        out = new
    return out


seen = lambda v: [[z, height(x, v)] for x, z in inner]
area = lambda ring: sum(ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1] for i in range(len(ring)))
near, far = seen(-0.022), seen(0.0)
if area(near) < 0:
    near, far = near[::-1], far[::-1]
opening = [[r6((h + 0.022 * cos) / sin), r6(z)] for z, h in cut(near, far)]
ops.append({"op": "shape-add", "scale": SCALE, "view": VIEW, "after": "rim", "shape": {**{k: v for k, v in old.items() if k not in ("points", "solid", "hatch", "penMm")}, "id": "rim-inner-edge", "role": "texture", "z": 6.2, "filled": False, "penMm": 0.25, "points": opening, "solid": {"kind": "extruded", "sweep": "v", "z0": -0.022, "z1": -0.0215}}})
op = next(o for o in ops if o.get("id") == "inside-1")
op["set"]["points"] = [[u, w + (0.002 if w > 0.3 else -0.002 if w < -0.3 else 0)] for u, w in op["set"]["points"]]
for sid in ("rim-side-l", "rim-side-r", "rim-end-foot", "rim-end-head"):
    ops.append({"op": "shape-remove", "scale": SCALE, "view": VIEW, "id": sid})
write(SLUG, ops)
print(f"{SLUG}: {len(ops)} ops")
