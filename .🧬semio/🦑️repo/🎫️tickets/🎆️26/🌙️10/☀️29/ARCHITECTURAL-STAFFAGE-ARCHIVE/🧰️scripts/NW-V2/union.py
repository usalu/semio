"""Raster union of hand-designed outlines into merged silhouettes (grid contour tracing + Douglas-Peucker). A merge operation on designed shapes, not a simplifier."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *


def flat(loop):
    return path_points(catmull(loop, True))


def _fill(mask, nx, ny, pts, x0, y0, c):
    ys = [p[1] for p in pts]
    n = len(pts)
    for j in range(max(0, int((min(ys) - y0) / c) - 1), min(ny, int((max(ys) - y0) / c) + 2)):
        y = y0 + (j + 0.5) * c
        xs = []
        for i in range(n):
            a, b = pts[i], pts[(i + 1) % n]
            if (a[1] > y) != (b[1] > y):
                xs.append(a[0] + (y - a[1]) / (b[1] - a[1]) * (b[0] - a[0]))
        xs.sort()
        for k in range(0, len(xs) - 1, 2):
            i0 = max(0, math.ceil((xs[k] - x0) / c - 0.5))
            i1 = min(nx - 1, math.floor((xs[k + 1] - x0) / c - 0.5))
            for i in range(i0, i1 + 1):
                mask[j][i] = 1


def dp(pts, eps):
    if len(pts) < 3:
        return pts
    a, b = pts[0], pts[-1]
    dx, dy = b[0] - a[0], b[1] - a[1]
    L = hypot(dx, dy) or 1e-12
    dm, im = -1, 0
    for i in range(1, len(pts) - 1):
        d = abs((pts[i][0] - a[0]) * dy - (pts[i][1] - a[1]) * dx) / L
        if d > dm:
            dm, im = d, i
    if dm > eps:
        return dp(pts[:im + 1], eps)[:-1] + dp(pts[im:], eps)
    return [a, b]


def morph(mask, nx, ny, r, grow):
    v = 1 if grow else 0
    tmp = [[0] * nx for _ in range(ny)]
    for j in range(ny):
        row = mask[j]
        for i in range(nx):
            hit = False
            for a in range(max(0, i - r), min(nx, i + r + 1)):
                if (row[a] == 1) == bool(grow):
                    hit = True
                    break
            tmp[j][i] = (1 if grow else 0) if hit else (0 if grow else 1)
    out = [[0] * nx for _ in range(ny)]
    for j in range(ny):
        for i in range(nx):
            hit = False
            for b in range(max(0, j - r), min(ny, j + r + 1)):
                if (tmp[b][i] == 1) == bool(grow):
                    hit = True
                    break
            out[j][i] = (1 if grow else 0) if hit else (0 if grow else 1)
    return out


def union(loops, cell=0.01, eps=None, keep_holes=False, min_area=0.0, close=0, open_=0):
    polys = [flat(l) if len(l[0]) >= 2 and not isinstance(l, tuple) else l for l in loops]
    xs = [p[0] for q in polys for p in q]
    ys = [p[1] for q in polys for p in q]
    m = (3 + close) * cell
    x0, y0 = min(xs) - m, min(ys) - m
    nx, ny = int((max(xs) - x0 + m) / cell) + 2, int((max(ys) - y0 + m) / cell) + 2
    mask = [[0] * nx for _ in range(ny)]
    for q in polys:
        _fill(mask, nx, ny, q, x0, y0, cell)
    if close:
        mask = morph(morph(mask, nx, ny, close, True), nx, ny, close, False)
    if open_:
        mask = morph(morph(mask, nx, ny, open_, False), nx, ny, open_, True)
    out = [[0] * nx for _ in range(ny)]
    seen = [[False] * nx for _ in range(ny)]
    stack = [(0, 0)]
    seen[0][0] = True
    while stack:
        i, j = stack.pop()
        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            a, b = i + di, j + dj
            if 0 <= a < nx and 0 <= b < ny and not seen[b][a] and mask[b][a] == 0:
                seen[b][a] = True
                stack.append((a, b))
    for j in range(ny):
        for i in range(nx):
            out[j][i] = 1 if (mask[j][i] == 1 or not seen[j][i]) else 0
    nxt = {}
    def inside(i, j):
        return 0 <= i < nx and 0 <= j < ny and out[j][i] == 1
    for j in range(ny):
        for i in range(nx):
            if not out[j][i]:
                continue
            if not inside(i, j - 1): nxt.setdefault((i, j), []).append((i + 1, j))
            if not inside(i + 1, j): nxt.setdefault((i + 1, j), []).append((i + 1, j + 1))
            if not inside(i, j + 1): nxt.setdefault((i + 1, j + 1), []).append((i, j + 1))
            if not inside(i - 1, j): nxt.setdefault((i, j + 1), []).append((i, j))
    loops_out = []
    while nxt:
        start = next(iter(nxt))
        cur = start
        path = [cur]
        prev_dir = None
        while True:
            outs = nxt.get(cur)
            if not outs:
                break
            if len(outs) > 1 and prev_dir is not None:
                def turn(o):
                    d = (o[0] - cur[0], o[1] - cur[1])
                    return prev_dir[0] * d[1] - prev_dir[1] * d[0]
                nn = max(outs, key=turn)
            else:
                nn = outs[0]
            outs.remove(nn)
            if not outs:
                del nxt[cur]
            prev_dir = (nn[0] - cur[0], nn[1] - cur[1])
            cur = nn
            if cur == start:
                break
            path.append(cur)
        if len(path) > 6:
            pts = [(x0 + p[0] * cell, y0 + p[1] * cell) for p in path]
            area = abs(sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))) / 2
            if area >= min_area:
                far = max(range(len(pts)), key=lambda k: hypot(pts[k][0] - pts[0][0], pts[k][1] - pts[0][1]))
                e = eps or 0.5 * cell
                loops_out.append(dp(pts[:far + 1], e)[:-1] + dp(pts[far:] + [pts[0]], e)[:-1])
    return loops_out
