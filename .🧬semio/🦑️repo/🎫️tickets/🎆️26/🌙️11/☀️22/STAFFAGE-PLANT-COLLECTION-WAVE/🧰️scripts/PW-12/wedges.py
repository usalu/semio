"""📐️ PW-12 wedge probe: for every pair of drawn stem capsules of a scale and each view, the heights where their projections stop overlapping (thin wedges whose apex amplifies tessellation differences), so the stem layout can be kept clear of them.

`python3 wedges.py <species module> <scale>`"""
import importlib
import math
import sys

mod = importlib.import_module(sys.argv[1])
S = mod.SCALES[sys.argv[2]]
caps = [c for c in S["caps"] if c.role != "thermal-only" and c.region == "stem-bark"]


def at(c, y):
    t = (y - c.p0[1]) / (c.p1[1] - c.p0[1])
    if t < 0 or t > 1:
        return None
    return [c.p0[k] + (c.p1[k] - c.p0[k]) * t for k in range(3)], c.r0 + (c.r1 - c.r0) * t


for view, k in (("elevation", 0), ("front", 2)):
    for i, a in enumerate(caps):
        for b in caps[i + 1:]:
            last = None
            for n in range(0, 300):
                y = 0.02 * n
                pa, pb = at(a, y), at(b, y)
                if pa is None or pb is None:
                    continue
                gap = abs(pa[0][k] - pb[0][k]) - pa[1] - pb[1]
                if last is not None and last < 0 <= gap:
                    print("[DEBUG] %s %s/%s wedge apex at y %.2f, opening %.3f per m" % (view, a.id, b.id, y, (gap - last) / 0.02))
                last = gap
