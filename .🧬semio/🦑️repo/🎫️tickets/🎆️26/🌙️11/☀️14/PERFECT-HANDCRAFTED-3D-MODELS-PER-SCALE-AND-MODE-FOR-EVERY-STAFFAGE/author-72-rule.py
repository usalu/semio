"""Where do the two fill rules disagree? python3 author-72-rule.py rings.json > rule.json
For every filled path / closed poly: faces of the arrangement of all rings; w = winding number of a face; non-zero fills w != 0, even-odd fills odd w.
disagree = area of faces with w != 0 and even w (non-zero fills, even-odd leaves a hole)."""
import json, sys
from shapely.geometry import LineString, Point
from shapely.ops import unary_union, polygonize

def wind(pt, rings):
    x, y = pt; w = 0
    for r in rings:
        for i in range(len(r)):
            (x0, y0), (x1, y1) = r[i], r[(i + 1) % len(r)]
            if (y0 <= y) != (y1 <= y):
                t = (y - y0) / (y1 - y0)
                if x0 + t * (x1 - x0) > x:
                    w += 1 if y1 > y0 else -1
    return w

out = []
for s in json.load(open(sys.argv[1])):
    rings = []
    for r in s["rings"]:
        r = [tuple(p) for p in r]
        d = [p for i, p in enumerate(r) if i == 0 or p != r[i - 1]]
        if len(d) > 1 and d[0] == d[-1]: d.pop()
        if len(d) >= 3: rings.append(d)
    if not rings: continue
    lines = unary_union([LineString(r + [r[0]]) for r in rings])
    dis, nz, eo = 0.0, 0.0, 0.0
    for f in polygonize(lines):
        w = wind(f.representative_point().coords[0], rings)
        a = f.area
        if w != 0: nz += a
        if w % 2: eo += a
        if w != 0 and w % 2 == 0: dis += a
    if dis > 1e-6:
        out.append({k: s[k] for k in ("cat", "slug", "scale", "view", "id", "kind", "bone")} | {"rings": len(rings), "disagree_m2": round(dis, 6), "nonzero_m2": round(nz, 6), "evenodd_m2": round(eo, 6)})
json.dump(out, sys.stdout, ensure_ascii=False)
print(len(out), file=sys.stderr)
