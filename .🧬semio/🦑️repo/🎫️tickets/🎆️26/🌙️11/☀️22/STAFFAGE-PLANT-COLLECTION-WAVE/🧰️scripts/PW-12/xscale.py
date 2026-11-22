"""🔍️ PW-12 cross-scale probe: rasterises the authored 3D primitives of two scales of a species in one view and reports the boundary
distances (p95, worst places), so coarse lobes can be placed by hand where they miss the finer silhouette.

`python3 xscale.py <species module> <fine scale> <coarse scale> <elevation|front|plan>`"""
import importlib
import math
import sys
from collections import deque

CELL = 0.025


def axes(view):
    return {"elevation": (0, 1), "front": (2, 1), "plan": (0, 2)}[view]


def mask(S, view, box):
    a, b = axes(view)
    x0, y0, nx, ny = box
    m = bytearray(nx * ny)
    prims = []
    for lb in S["lobes"]:
        prims.append(("e", lb.c[a], lb.c[b], lb.rad[a], lb.rad[b]))
    for c in S.get("caps", []):
        if c.role != "thermal-only":
            prims.append(("c", c.p0[a], c.p0[b], c.p1[a], c.p1[b], c.r0, c.r1))
    rc = S["collar"]
    h = rc[2] if len(rc) > 2 else 0.15
    if view == "plan":
        prims.append(("e", 0, 0, rc[0], rc[0]))
    else:
        prims.append(("c", 0, 0, 0, h, rc[0], rc[1]))
    for j in range(ny):
        y = y0 + (j + 0.5) * CELL
        for i in range(nx):
            x = x0 + (i + 0.5) * CELL
            for p in prims:
                if p[0] == "e":
                    if ((x - p[1]) / p[3]) ** 2 + ((y - p[2]) / p[4]) ** 2 <= 1:
                        m[j * nx + i] = 1
                        break
                else:
                    _, ax, ay, bx, by, r0, r1 = p
                    dx, dy = bx - ax, by - ay
                    L2 = dx * dx + dy * dy or 1e-12
                    t = max(0, min(1, ((x - ax) * dx + (y - ay) * dy) / L2))
                    if math.hypot(x - ax - t * dx, y - ay - t * dy) <= r0 + (r1 - r0) * t:
                        m[j * nx + i] = 1
                        break
    return m


def boundary(m, nx, ny):
    out = []
    for j in range(ny):
        for i in range(nx):
            if m[j * nx + i] and any(not (0 <= i + di < nx and 0 <= j + dj < ny) or not m[(j + dj) * nx + i + di] for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1))):
                out.append((i, j))
    return out


def dist_field(cells, nx, ny):
    d = [1e9] * (nx * ny)
    q = deque()
    for i, j in cells:
        d[j * nx + i] = 0
        q.append((i, j))
    while q:
        i, j = q.popleft()
        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            a, b = i + di, j + dj
            if 0 <= a < nx and 0 <= b < ny and d[b * nx + a] > d[j * nx + i] + 1:
                d[b * nx + a] = d[j * nx + i] + 1
                q.append((a, b))
    return d


def main():
    mod = importlib.import_module(sys.argv[1])
    fine, coarse, view = mod.SCALES[sys.argv[2]], mod.SCALES[sys.argv[3]], sys.argv[4]
    box = (-2.4, -2.4 if view == "plan" else -0.1, int(4.8 / CELL), int((4.8 if view == "plan" else 4.6) / CELL))
    nx, ny = box[2], box[3]
    A, B = mask(fine, view, box), mask(coarse, view, box)
    ba, bb = boundary(A, nx, ny), boundary(B, nx, ny)
    da, db = dist_field(ba, nx, ny), dist_field(bb, nx, ny)
    ds = sorted([db[j * nx + i] * CELL for i, j in ba] + [da[j * nx + i] * CELL for i, j in bb])
    print("[DEBUG] p95 %.3f max %.3f" % (ds[int(0.95 * (len(ds) - 1))], ds[-1]))
    worst = sorted([(db[j * nx + i] * CELL, "fine", box[0] + i * CELL, box[1] + j * CELL) for i, j in ba] + [(da[j * nx + i] * CELL, "coarse", box[0] + i * CELL, box[1] + j * CELL) for i, j in bb], reverse=True)
    seen = []
    for w in worst:
        if all(math.hypot(w[2] - s[2], w[3] - s[3]) > 0.3 for s in seen):
            seen.append(w)
        if len(seen) >= 8:
            break
    for w in seen:
        print("[DEBUG] %.3f m on the %s boundary at (%.2f, %.2f)" % w)


main()
