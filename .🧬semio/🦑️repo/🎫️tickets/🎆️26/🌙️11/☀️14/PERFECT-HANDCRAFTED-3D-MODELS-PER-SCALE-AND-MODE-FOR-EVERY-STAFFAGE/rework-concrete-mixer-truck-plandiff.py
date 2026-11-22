"""🗺️ Where the 1:50 plan drawing and the projected body of machinery/concrete-mixer-truck disagree: rasters both on a 15 mm grid (chute bones left out, they are placed by the pose) and lists the connected difference blobs.

Usage: python3 rework-concrete-mixer-truck-plandiff.py [scale]   (after r3-project2d.ts --view plan)
"""
import json, re, sys

scale = sys.argv[1] if len(sys.argv) > 1 else "50"
C = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/concrete-mixer-truck/🔣️.json"
a = json.load(open(C))["asset"]
pj = json.load(open(f"🗑️generated/r3-project2d/machinery/concrete-mixer-truck.s{scale}.plan.json"))
H = 0.015
X0, Y0, NX, NY = -4.7, -1.4, int(9.5 / H), int(2.8 / H)


def rings(s):
    k = s["kind"]
    if k == "rect":
        return [[(s["x"], s["y"]), (s["x"] + s["w"], s["y"]), (s["x"] + s["w"], s["y"] + s["h"]), (s["x"], s["y"] + s["h"])]]
    if k == "poly" or (k == "path" and "points" in s):
        return [[tuple(p) for p in s["points"]]]
    if k == "ellipse":
        import math
        return [[(s["cx"] + s["rx"] * math.cos(i * 0.2618), s["cy"] + s["ry"] * math.sin(i * 0.2618)) for i in range(24)]]
    if k == "path":
        d = s.get("d", "")
        if re.search(r"[ACQSacqs]", d):
            return None
        out = []
        for sub in re.split(r"[Mm]", d):
            n = [float(t) for t in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", sub)]
            if len(n) >= 6:
                out.append(list(zip(n[0::2], n[1::2])))
        return out
    return None


def paint(grid, ring):
    ys = [p[1] for p in ring]
    for j in range(max(0, int((min(ys) - Y0) / H)), min(NY, int((max(ys) - Y0) / H) + 1)):
        y = Y0 + (j + 0.5) * H
        xs = []
        for (x1, y1), (x2, y2) in zip(ring, ring[1:] + ring[:1]):
            if (y1 > y) != (y2 > y):
                xs.append(x1 + (y - y1) / (y2 - y1) * (x2 - x1))
        xs.sort()
        for x1, x2 in zip(xs[0::2], xs[1::2]):
            for i in range(max(0, int((x1 - X0) / H + 0.5)), min(NX, int((x2 - X0) / H + 0.5))):
                grid[j][i] ^= 1


def raster(shapes):
    total = [[0] * NX for _ in range(NY)]
    for s in shapes:
        rs = rings(s)
        if rs is None:
            print("unparsed", s.get("id"))
            continue
        g = [[0] * NX for _ in range(NY)]
        for r in rs:
            paint(g, r)
        for j in range(NY):
            row, t = g[j], total[j]
            for i in range(NX):
                if row[i]:
                    t[i] = 1
    return total


B = raster([s for b in pj["bones"] if not b["bone"].startswith("chute") for s in b["shapes"]])
D = raster([s for s in a["representations"][scale]["geometry"]["plan"] if not s["bone"].startswith("chute") and s.get("filled")])
for name, U, W in (("body not drawn", B, D), ("drawn not body", D, B)):
    diff = [[1 if U[j][i] and not W[j][i] else 0 for i in range(NX)] for j in range(NY)]
    blobs = []
    for j in range(NY):
        for i in range(NX):
            if diff[j][i]:
                stack, cells = [(j, i)], []
                diff[j][i] = 0
                while stack:
                    cj, ci = stack.pop()
                    cells.append((cj, ci))
                    for nj, ni in ((cj + 1, ci), (cj - 1, ci), (cj, ci + 1), (cj, ci - 1)):
                        if 0 <= nj < NY and 0 <= ni < NX and diff[nj][ni]:
                            diff[nj][ni] = 0
                            stack.append((nj, ni))
                js, is_ = [c[0] for c in cells], [c[1] for c in cells]
                w = min(max(js) - min(js) + 1, max(is_) - min(is_) + 1)
                blobs.append((len(cells) * H * H, w * H, X0 + min(is_) * H, Y0 + min(js) * H, X0 + (max(is_) + 1) * H, Y0 + (max(js) + 1) * H))
    for b in sorted(blobs, reverse=True)[:12]:
        if b[1] > 0.03:
            print(name, "area %.3f thin %.3f bbox x %.2f..%.2f z %.2f..%.2f" % (b[0], b[1], b[2], b[4], b[3], b[5]))
