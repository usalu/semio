"""🔍 Finds self-intersections and repeated points in the outlines of `author-piano.py`."""
import importlib.util
import sys

spec = importlib.util.spec_from_file_location("a", "author-piano.py")
a = importlib.util.module_from_spec(spec)
spec.loader.exec_module(a)


def segs_cross(p, q, r, s):
    def o(a_, b_, c_):
        return (b_[0] - a_[0]) * (c_[1] - a_[1]) - (b_[1] - a_[1]) * (c_[0] - a_[0])
    d1, d2, d3, d4 = o(r, s, p), o(r, s, q), o(p, q, r), o(p, q, s)
    return (d1 * d2 < 0) and (d3 * d4 < 0)


def check(name, poly):
    n = len(poly)
    bad = []
    for i in range(n):
        a0, a1 = poly[i], poly[(i + 1) % n]
        if abs(a0[0] - a1[0]) < 1e-9 and abs(a0[1] - a1[1]) < 1e-9:
            bad.append(("dup", i, a0))
        for j in range(i + 2, n):
            if i == 0 and j == n - 1:
                continue
            b0, b1 = poly[j], poly[(j + 1) % n]
            if segs_cross(a0, a1, b0, b1):
                bad.append(("cross", i, j, a0, b0))
    print(name, len(poly), "issues", len(bad), bad[:4])


for per in (4, 2, 1):
    check(f"rim {per}", a.rim_ring(per))
    check(f"inner {per}", a.inner_region(per, -0.32))
    check(f"plate {per}", a.inner_region(per, -0.43, 0.012))
    check(f"strings {per}", a.inner_region(per, -0.40, 0.04))
    check(f"outer {per}", a.outer_outline(per))
