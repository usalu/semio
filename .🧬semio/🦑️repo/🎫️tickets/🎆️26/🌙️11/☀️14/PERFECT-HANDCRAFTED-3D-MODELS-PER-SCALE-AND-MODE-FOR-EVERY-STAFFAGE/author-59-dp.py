"""Douglas-Peucker for closed polygons: dp(points, eps) -> reduced ring (a generalisation of a finer scale's outline for a coarser scale)."""
import math
def dp(pts0, eps):
    n = len(pts0)
    i0 = max(range(n), key=lambda i: pts0[i][0])
    ring = pts0[i0:] + pts0[:i0]
    ring = ring + [ring[0]]
    pts = ring
    def rec(a, b):
        best, bi = 0, -1
        (x0, y0), (x1, y1) = pts[a], pts[b]
        l = math.hypot(x1 - x0, y1 - y0) or 1e-9
        for i in range(a + 1, b):
            d = abs((x1 - x0) * (y0 - pts[i][1]) - (x0 - pts[i][0]) * (y1 - y0)) / l
            if d > best:
                best, bi = d, i
        return rec(a, bi) + [bi] + rec(bi, b) if best > eps else []
    h = len(ring) // 2
    keep = sorted({0, h, len(ring) - 1} | set(rec(0, h)) | set(rec(h, len(ring) - 1)))
    return [[round(ring[k][0], 5), round(ring[k][1], 5)] for k in keep][:-1]
