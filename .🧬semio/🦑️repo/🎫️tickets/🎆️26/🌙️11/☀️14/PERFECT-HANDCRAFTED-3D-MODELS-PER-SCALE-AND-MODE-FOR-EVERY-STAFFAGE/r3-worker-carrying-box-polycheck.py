"""🔍 Lists the drawn polygons of the round-3 worker shapes that self-intersect, repeat a point or have almost no area (debug aid of the 2D redraw)."""
import json
import sys

def inter(a, b, c, d):
    def o(p, q, r):
        return (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0])
    return o(a, b, c) * o(a, b, d) < 0 and o(c, d, a) * o(c, d, b) < 0

shapes = json.load(open(sys.argv[1]))
for scale, rep in shapes.items():
    for view, lst in rep["geometry"].items():
        for s in lst:
            p = s.get("points")
            if not p or not s.get("closed"):
                continue
            n = len(p)
            area = sum(p[i][0] * p[(i + 1) % n][1] - p[(i + 1) % n][0] * p[i][1] for i in range(n)) / 2
            bad = [(i, j) for i in range(n) for j in range(i + 2, n) if not (i == 0 and j == n - 1) and inter(p[i], p[(i + 1) % n], p[j], p[(j + 1) % n])]
            dup = [i for i in range(n) if p[i] == p[(i + 1) % n]]
            if bad or dup or abs(area) < 1e-6:
                print(scale, view, s["id"], "n", n, "area %.6f" % area, "crossings", bad[:4], "dups", dup)
