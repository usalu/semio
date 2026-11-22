"""🔍️ Compares the product's 3D projection with the drawn rings of one probe dump and lists where they differ."""
import json
import sys
from shapely.geometry import Polygon
from shapely.ops import unary_union

d = json.load(open(sys.argv[1]))
def poly(ring):
    pts = [(p["x"], p["y"]) if isinstance(p, dict) else tuple(p) for p in ring]
    return Polygon(pts).buffer(0) if len(pts) > 2 else Polygon()
vol = unary_union([poly(r) for g in d["volume"] for r in (g if isinstance(g[0], list) else [g])])
drawn = unary_union([poly(r) for s in d["shapes"] if s["filled"] and s["rings"] for r in s["rings"]])
print("[DEBUG] volume area", round(vol.area, 4), "drawn area", round(drawn.area, 4), "iou", round(vol.intersection(drawn).area / vol.union(drawn).area, 4))
for name, g in (("3D-only", vol.difference(drawn)), ("2D-only", drawn.difference(vol))):
    parts = sorted(getattr(g, "geoms", [g]), key=lambda q: -q.area)[:8]
    for q in parts:
        if q.area > 1e-5:
            print("[DEBUG]", name, round(q.area, 5), [round(v, 3) for v in q.bounds])


def evenodd(rings):
    """⚫️ Even-odd fill of a ring list."""
    out = Polygon()
    for r in rings:
        out = out.symmetric_difference(poly(r))
    return out


import numpy as np
drawn2 = unary_union([evenodd(s["rings"]) for s in d["shapes"] if s["filled"] and s["rings"]])
def samples(g, step=0.005):
    pts = []
    for q in getattr(g, "geoms", [g]):
        for ring in [q.exterior, *q.interiors]:
            n = max(2, int(ring.length / step))
            pts += [ring.interpolate(i / n, normalized=True) for i in range(n)]
    return pts
for name, a, b in (("3D->2D", vol, drawn2), ("2D->3D", drawn2, vol)):
    bb = b.boundary
    ds = [(p.distance(bb), p.x, p.y) for p in samples(a)]
    ds.sort()
    arr = np.array([x[0] for x in ds])
    print("[DEBUG]", name, "p95", round(float(np.percentile(arr, 95)), 4), "max", round(arr.max(), 4), "worst", [tuple(round(v, 3) for v in x) for x in ds[-5:]])
    far = [x for x in ds if x[0] > 0.03]
    if far:
        xs = np.array([x[1] for x in far]); ys = np.array([x[2] for x in far])
        print("[DEBUG]  far count", len(far), "of", len(ds), "x range", round(xs.min(), 2), round(xs.max(), 2), "y range", round(ys.min(), 3), round(ys.max(), 3))
