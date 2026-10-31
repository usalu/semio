"""Scratch approximation of the white-sliver measure (pure python chamfer distance transforms) to locate narrow white areas; usage: python3 sliver.py <module> <den> [pose]"""
import sys, importlib, math
from render import flat
from lib import ap

mod, den = sys.argv[1], int(sys.argv[2])
posename = sys.argv[3] if len(sys.argv) > 3 else None
m = importlib.import_module(mod)
a = m.asset()
rp = next(p for p in a["poses"] if p["id"] == (posename or a["presentation"]["defaultPose"]))
W = m.rig.world(rp["bones"])
r = a["representations"][str(den)]
k = 1000.0 / den
GAP = 0.15
pitch = GAP / 4
items = []
xs, ys = [], []
for s in r["geometry"]["elevation"]:
    if s["role"] == "ground":
        continue
    pts, closed = flat(s)
    wp = [(ap(W[s["bone"]], p)[0] * k, ap(W[s["bone"]], p)[1] * k) for p in pts]
    pen = s.get("penMm") or r["pens"].get(s["role"]) or r["pens"]["body"]
    items.append((s, wp, closed, pen))
    for p in wp:
        xs.append(p[0]); ys.append(p[1])
x0, x1, y0, y1 = min(xs) - 1, max(xs) + 1, min(ys) - 1, max(ys) + 1
nx, ny = int((x1 - x0) / pitch) + 1, int((y1 - y0) / pitch) + 1
ink = [[0] * nx for _ in range(ny)]


def fill(poly):
    for row in range(ny):
        y = y0 + (row + 0.5) * pitch
        xsl = []
        n = len(poly)
        for i in range(n):
            ax, ay = poly[i]; bx, by = poly[(i + 1) % n]
            if (ay > y) != (by > y):
                xsl.append(ax + (y - ay) / (by - ay) * (bx - ax))
        xsl.sort()
        for j in range(0, len(xsl) - 1, 2):
            c0 = max(0, int(math.ceil((xsl[j] - x0) / pitch - 0.5))); c1 = min(nx - 1, int(math.floor((xsl[j + 1] - x0) / pitch - 0.5)))
            for c in range(c0, c1 + 1):
                ink[row][c] = 1


def stroke(pts, closed, pen):
    half = pen / 2
    n = len(pts)
    segs = n if closed else n - 1
    for i in range(segs):
        ax, ay = pts[i]; bx, by = pts[(i + 1) % n]
        c0 = max(0, int((min(ax, bx) - half - x0) / pitch) - 1); c1 = min(nx - 1, int((max(ax, bx) + half - x0) / pitch) + 1)
        r0 = max(0, int((min(ay, by) - half - y0) / pitch) - 1); r1 = min(ny - 1, int((max(ay, by) + half - y0) / pitch) + 1)
        dx, dy = bx - ax, by - ay
        l2 = dx * dx + dy * dy
        for row in range(r0, r1 + 1):
            py = y0 + (row + 0.5) * pitch
            for c in range(c0, c1 + 1):
                px = x0 + (c + 0.5) * pitch
                t = 0 if l2 == 0 else max(0, min(1, ((px - ax) * dx + (py - ay) * dy) / l2))
                if math.hypot(px - ax - t * dx, py - ay - t * dy) <= half:
                    ink[row][c] = 1


for s, wp, closed, pen in items:
    if closed and s["filled"]:
        fill(wp)
    stroke(wp, closed, pen)


def dt(src):
    INF = 1e9
    d = [[0 if src[y][x] else INF for x in range(nx)] for y in range(ny)]
    for y in range(ny):
        for x in range(nx):
            v = d[y][x]
            if x > 0: v = min(v, d[y][x - 1] + 1)
            if y > 0:
                v = min(v, d[y - 1][x] + 1)
                if x > 0: v = min(v, d[y - 1][x - 1] + 1.414)
                if x < nx - 1: v = min(v, d[y - 1][x + 1] + 1.414)
            d[y][x] = v
    for y in range(ny - 1, -1, -1):
        for x in range(nx - 1, -1, -1):
            v = d[y][x]
            if x < nx - 1: v = min(v, d[y][x + 1] + 1)
            if y < ny - 1:
                v = min(v, d[y + 1][x] + 1)
                if x < nx - 1: v = min(v, d[y + 1][x + 1] + 1.414)
                if x > 0: v = min(v, d[y + 1][x - 1] + 1.414)
            d[y][x] = v
    return d


white = [[1 - ink[y][x] for x in range(nx)] for y in range(ny)]
din = dt(ink)
R = GAP / 2 / pitch
core = [[1 if white[y][x] and din[y][x] >= R else 0 for x in range(nx)] for y in range(ny)]
dc = dt(core)
unc = [[1 if white[y][x] and dc[y][x] > R else 0 for x in range(nx)] for y in range(ny)]
seen = [[0] * nx for _ in range(ny)]
comps = []
for y in range(ny):
    for x in range(nx):
        if unc[y][x] and not seen[y][x]:
            st = [(x, y)]; seen[y][x] = 1; cells = []
            while st:
                cx, cy = st.pop(); cells.append((cx, cy))
                for ddx, ddy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                    qx, qy = cx + ddx, cy + ddy
                    if 0 <= qx < nx and 0 <= qy < ny and unc[qy][qx] and not seen[qy][qx]:
                        seen[qy][qx] = 1; st.append((qx, qy))
            area = len(cells) * pitch * pitch
            mx = sum(c[0] for c in cells) / len(cells); my = sum(c[1] for c in cells) / len(cells)
            comps.append((area, (x0 + (mx + 0.5) * pitch) / k, (y0 + (my + 0.5) * pitch) / k))
comps.sort(reverse=True)
print("allowed", 3 * GAP * GAP, "components", len(comps))
for c in comps[:12]:
    print(f"area {c[0]:.4f} mm2 at model x={c[1]:.2f} y={c[2]:.2f} m")
