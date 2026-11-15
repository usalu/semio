"""Scan: filled paths whose subpaths overlap (non-trivial area, not strictly nested) and closed rings that cross themselves.
python3 author-72-scan.py rings.json > scan.json ; prints a table."""
import json, sys
from shapely.geometry import Polygon, LineString
from shapely.ops import unary_union

TOL_REL = 0.02  # nested: intersection >= 98 % of the smaller ring
MIN_AREA = 1e-6  # 1 mm2 is trivial; report from 1 cm2 downwards counted as overlap if > 1e-6 m2 and > 1 % of smaller

def signed(r):
    return sum(r[i][0] * r[(i + 1) % len(r)][1] - r[(i + 1) % len(r)][0] * r[i][1] for i in range(len(r))) / 2

hits, selfx, nested = [], [], []
for s in json.load(open(sys.argv[1])):
    rings = [r for r in s["rings"] if len(r) >= 3]
    key = {k: s[k] for k in ("cat", "slug", "scale", "view", "id", "kind", "bone")}
    polys = []
    for r in rings:
        p = Polygon(r)
        polys.append(p)
        ls = LineString(r + [r[0]])
        if not ls.is_simple and abs(signed(r)) > 0:
            selfx.append({**key, "ring": len(polys) - 1, "area": round(abs(signed(r)), 6)})
    for i in range(len(polys)):
        for j in range(i + 1, len(polys)):
            a, b = polys[i].buffer(0), polys[j].buffer(0)
            if a.is_empty or b.is_empty:
                continue
            ia = a.intersection(b).area
            small = min(a.area, b.area)
            if small <= 0 or ia <= MIN_AREA:
                continue
            same = (signed(rings[i]) > 0) == (signed(rings[j]) > 0)
            if ia >= (1 - TOL_REL) * small:
                nested.append({**key, "rings": [i, j], "inner_area": round(small, 6), "same_winding": same})
            else:
                hits.append({**key, "rings": [i, j], "overlap_m2": round(ia, 6), "smaller_m2": round(small, 6), "frac": round(ia / small, 3), "same_winding": same})
json.dump({"overlap": hits, "selfCross": selfx, "nested": nested}, sys.stdout, ensure_ascii=False)
print(f"overlap {len(hits)} self {len(selfx)} nested {len(nested)}", file=sys.stderr)
