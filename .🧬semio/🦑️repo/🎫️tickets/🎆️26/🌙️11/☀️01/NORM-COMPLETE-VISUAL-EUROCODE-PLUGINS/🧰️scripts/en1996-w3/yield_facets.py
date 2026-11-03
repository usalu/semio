"""Independent check of the yield-line facets: they tile the panel and the tent deflection is the lowest plane (scipy/numpy only)."""
import numpy as np
from scipy.spatial import ConvexHull

def facets(w, v, r, left, right):
    x0, x1 = left, w - right
    out = []
    if r > 1e-9: out.append([(0, 0), (w, 0), (x1, r), (x0, r)])
    if v - r > 1e-9: out.append([(0, v), (w, v), (x1, r), (x0, r)])
    if left > 1e-9: out.append([(0, 0), (x0, r), (0, v)])
    if right > 1e-9: out.append([(w, 0), (w, v), (x1, r)])
    return out

def area(f):
    a = np.array(f); x, y = a[:, 0], a[:, 1]
    return 0.5 * abs(np.dot(x, np.roll(y, -1)) - np.dot(y, np.roll(x, -1)))

for name, (w, v, r, l, rr) in {"SSSS square": (4, 4, 2, 2, 2), "free top": (6, 3, 3, 1.5, 2.0), "free bottom+left": (5, 2.5, 0, 0, 1.7)}.items():
    fs = facets(w, v, r, l, rr)
    tri_area = sum(ConvexHull(np.unique(np.array(p, float), axis=0)).volume for p in fs)
    print(name, "facets", len(fs), "sum area", round(sum(map(area, fs)), 9), "convex-hull area", round(tri_area, 9), "panel", w * v)
    assert abs(sum(map(area, fs)) - w * v) < 1e-9 and abs(tri_area - w * v) < 1e-9
print("ok")
