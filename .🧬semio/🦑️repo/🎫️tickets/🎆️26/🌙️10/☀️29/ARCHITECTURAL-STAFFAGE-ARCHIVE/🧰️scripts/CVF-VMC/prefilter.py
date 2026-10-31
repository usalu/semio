"""CVF-VMC scratch: offline approximation of the engine's print suitability (gap between visible outlines, stroke, hatch, thin filled features).
Mirrors 🖨️representation measureOnce: paper mm = metres * 1000/den, y down, hidden lines behind higher-z (or joined same-region) filled body/detail/texture shapes removed,
ink gap = centreline distance - (penA+penB)/2, reported when 0 < ink < minGap. Sampled outlines (step 0.04 mm): a screening tool, the official check decides.
Usage: python3 prefilter.py <asset-json> [scales=500,200,100,50] [--all-poses] [--limit n]"""
import json, math, sys
sys.path.insert(0, __import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from lib import world, ap, path_pts

MIN_STROKE, MIN_GAP, MIN_FEATURE, MIN_HATCH = 0.1, 0.15, 0.2, 0.4
OCC = ("body", "detail", "texture")
STEP = 0.04


def arc_pts(cx, cy, r, a0, a1, n):
    return [(cx + r * math.cos(a0 + (a1 - a0) * i / n), cy + r * math.sin(a0 + (a1 - a0) * i / n)) for i in range(n + 1)]


def local_rings(s):
    k = s["kind"]
    if k == "path":
        d = s["d"]
        subs = [x for x in d.replace("M", "|M").split("|") if x.strip()]
        out = []
        for sub in subs:
            pts = path_pts(sub, 12)
            out.append(pts)
        return out, s["filled"] and True
    if k == "poly":
        return [[tuple(p) for p in s["points"]]], s["closed"]
    if k == "rect":
        x, y, w, h = s["x"], s["y"], s["w"], s["h"]
        r = min(s.get("r", 0), w / 2, h / 2)
        if r <= 0:
            return [[(x, y), (x + w, y), (x + w, y + h), (x, y + h)]], True
        pts = arc_pts(x + w - r, y + r, r, -math.pi / 2, 0, 6) + arc_pts(x + w - r, y + h - r, r, 0, math.pi / 2, 6) + arc_pts(x + r, y + h - r, r, math.pi / 2, math.pi, 6) + arc_pts(x + r, y + r, r, math.pi, 1.5 * math.pi, 6)
        return [pts], True
    if k == "ellipse":
        rot = math.radians(s.get("rotationDeg", 0))
        n = 72
        pts = []
        for i in range(n):
            a = 2 * math.pi * i / n
            px, py = s["rx"] * math.cos(a), s["ry"] * math.sin(a)
            pts.append((s["cx"] + px * math.cos(rot) - py * math.sin(rot), s["cy"] + px * math.sin(rot) + py * math.cos(rot)))
        return [pts], True
    if k == "capsule":
        x0, y0, x1, y1, r0, r1 = s["x0"], s["y0"], s["x1"], s["y1"], s["r0"], s["r1"]
        L = math.hypot(x1 - x0, y1 - y0)
        if L < 1e-9:
            return [arc_pts(x0, y0, r0, 0, 2 * math.pi, 24)[:-1]], True
        cloud = arc_pts(x0, y0, r0, 0, 2 * math.pi, 24)[:-1] + arc_pts(x1, y1, r1, 0, 2 * math.pi, 24)[:-1]
        return [hull(cloud)], True
    raise ValueError(k)


def hull(pts):
    pts = sorted(set(pts))
    if len(pts) < 3:
        return pts
    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lo = []
    for p in pts:
        while len(lo) >= 2 and cross(lo[-2], lo[-1], p) <= 0:
            lo.pop()
        lo.append(p)
    up = []
    for p in reversed(pts):
        while len(up) >= 2 and cross(up[-2], up[-1], p) <= 0:
            up.pop()
        up.append(p)
    return lo[:-1] + up[:-1]


def inside(pt, rings):
    x, y = pt
    c = False
    for ring in rings:
        n = len(ring)
        j = n - 1
        for i in range(n):
            xi, yi = ring[i]
            xj, yj = ring[j]
            if (yi > y) != (yj > y) and x < (xj - xi) * (y - yi) / (yj - yi) + xi:
                c = not c
            j = i
    return c


def seg_dist(p, a, b):
    dx, dy = b[0] - a[0], b[1] - a[1]
    l2 = dx * dx + dy * dy
    t = 0 if l2 == 0 else max(0, min(1, ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / l2))
    return math.hypot(p[0] - a[0] - t * dx, p[1] - a[1] - t * dy)


def near_boundary(pt, rings, eps):
    for ring in rings:
        n = len(ring)
        for i in range(n):
            if seg_dist(pt, ring[i], ring[(i + 1) % n]) < eps:
                return True
    return False


def prepare(asset, rep, pose, den):
    W = world(asset, pose)
    k = 1000.0 / den
    pens = rep["pens"]
    hatches = {h["id"]: h for h in rep["hatches"]}
    bones = {b["id"]: b["parent"] for b in asset["rig"]["bones"]}
    items = []
    shapes = list(enumerate(rep["geometry"]["elevation"]))
    shapes.sort(key=lambda t: (t[1]["z"], t[0]))
    for order, (idx, s) in enumerate(shapes):
        pen = s.get("penMm") or pens.get(s["role"])
        if pen is None or s["role"] == "ground":
            continue
        rings, closed_flag = local_rings(s)
        m = W[s["bone"]]
        prs = []
        for r in rings:
            prs.append([(ap(m, p)[0] * k, -ap(m, p)[1] * k) for p in r])
        kind = s["kind"]
        closed = True if kind in ("rect", "ellipse", "capsule") else (s["closed"] if kind == "poly" else s["d"].strip().upper().endswith("Z"))
        filled = bool(s["filled"]) and closed
        items.append({"id": s["id"], "s": s, "rings": prs, "closed": closed, "filled": filled, "pen": pen, "order": order, "region": s.get("region"), "bone": s["bone"], "parent": bones[s["bone"]], "occ": filled and s["role"] in OCC})
    return items


def joined(a, b):
    return a["region"] is not None and a["region"] == b["region"] and (a["bone"] == b["bone"] or a["parent"] == b["bone"] or b["parent"] == a["bone"])


def box(rings):
    xs = [p[0] for r in rings for p in r]
    ys = [p[1] for r in rings for p in r]
    return (min(xs), min(ys), max(xs), max(ys))


def sample_runs(item, hiders):
    pts = []
    for ring in item["rings"]:
        n = len(ring)
        cnt = n if item["closed"] else n - 1
        for i in range(cnt):
            a, b = ring[i], ring[(i + 1) % n]
            L = math.hypot(b[0] - a[0], b[1] - a[1])
            m = max(1, int(L / STEP))
            for j in range(m + (0 if i < cnt - 1 or item["closed"] else 1)):
                t = j / m
                p = (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)
                hid = False
                for h in hiders:
                    bx = h["box"]
                    if p[0] < bx[0] or p[0] > bx[2] or p[1] < bx[1] or p[1] > bx[3]:
                        continue
                    if inside(p, h["rings"]) and not near_boundary(p, h["rings"], 2e-3):
                        hid = True
                        break
                if not hid:
                    pts.append(p)
    return pts


def thin_features(item, minf=MIN_FEATURE):
    """Approximate feature width of one filled shape by opening with a disk of diameter minf (raster pitch 0.03). Returns the area (mm2) lost relative to the tolerance."""
    rings = item["rings"]
    bx = box(rings)
    pitch = 0.02
    w, h = bx[2] - bx[0], bx[3] - bx[1]
    if w / pitch * h / pitch > 400000:
        pitch = math.sqrt(w * h / 400000)
    nx, ny = int(w / pitch) + 5, int(h / pitch) + 5
    x0, y0 = bx[0] - 2 * pitch, bx[1] - 2 * pitch
    mask = [bytearray(nx) for _ in range(ny)]
    for row in range(ny):
        y = y0 + (row + 0.5) * pitch
        xs = []
        for ring in rings:
            n = len(ring)
            j = n - 1
            for i in range(n):
                a, b = ring[i], ring[j]
                if (a[1] > y) != (b[1] > y):
                    xs.append(a[0] + (y - a[1]) / (b[1] - a[1]) * (b[0] - a[0]))
                j = i
        xs.sort()
        for q in range(0, len(xs) - 1, 2):
            lo = max(0, math.ceil((xs[q] - x0) / pitch - 0.5))
            hi = min(nx - 1, math.floor((xs[q + 1] - x0) / pitch - 0.5))
            for c in range(lo, hi + 1):
                mask[row][c] = 1
    r = minf / 2 / pitch
    ri = int(math.ceil(r))
    offs = [(dx, dy) for dy in range(-ri, ri + 1) for dx in range(-ri, ri + 1) if dx * dx + dy * dy <= r * r]
    core = [bytearray(nx) for _ in range(ny)]
    inside_n = 0
    for y in range(ny):
        for x in range(nx):
            if mask[y][x]:
                inside_n += 1
                ok = True
                for dx, dy in offs:
                    yy, xx = y + dy, x + dx
                    if yy < 0 or yy >= ny or xx < 0 or xx >= nx or not mask[yy][xx]:
                        ok = False
                        break
                if ok:
                    core[y][x] = 1
    cov = [bytearray(nx) for _ in range(ny)]
    for y in range(ny):
        for x in range(nx):
            if core[y][x]:
                for dx, dy in offs:
                    yy, xx = y + dy, x + dx
                    if 0 <= yy < ny and 0 <= xx < nx:
                        cov[yy][xx] = 1
    lost = sum(1 for y in range(ny) for x in range(nx) if mask[y][x] and not cov[y][x]) * pitch * pitch
    tol = max(0.01 * inside_n * pitch * pitch, (minf / 2) ** 2) + 4 * (1 - math.pi / 4) * (minf / 2) ** 2
    return lost, tol


def measure(asset, den, all_poses=True, features=True):
    rep = asset["representations"][str(den)]
    if rep["status"] != "authored":
        return None
    out = {"gap": {}, "feature": {}, "stroke": [], "hatch": []}
    for s in rep["geometry"]["elevation"]:
        pen = s.get("penMm") or rep["pens"].get(s["role"])
        if pen is not None and pen < MIN_STROKE - 1e-9 and s["role"] != "ground":
            out["stroke"].append((s["id"], pen))
    for h in rep["hatches"]:
        if h["penMm"] < MIN_STROKE - 1e-9:
            out["stroke"].append(("hatch " + h["id"], h["penMm"]))
        if h["spacingMm"] < MIN_HATCH - 1e-9:
            out["hatch"].append((h["id"], h["spacingMm"]))
    poses = [p for p in asset["poses"] if not p.get("key")]
    if not all_poses:
        poses = [p for p in poses if p["id"] == asset["presentation"]["defaultPose"]]
    seen = set()
    for pose in poses:
        sig = json.dumps([pose.get("bones"), pose.get("root")], sort_keys=True)
        if sig in seen:
            continue
        seen.add(sig)
        items = prepare(asset, rep, pose, den)
        for it in items:
            it["box"] = box(it["rings"])
        runs = []
        for it in items:
            hiders = [q for q in items if q["occ"] and q is not it and (q["order"] > it["order"] or joined(it, q)) and not (q["box"][2] < it["box"][0] or q["box"][0] > it["box"][2] or q["box"][3] < it["box"][1] or q["box"][1] > it["box"][3])]
            runs.append((it, sample_runs(it, hiders)))
        grid = {}
        C = 0.5
        for ii, (it, pts) in enumerate(runs):
            for p in pts:
                grid.setdefault((int(p[0] // C), int(p[1] // C)), []).append((ii, p))
        best = {}
        for (cx, cy), lst in grid.items():
            cand = []
            for dx in (-1, 0, 1):
                for dy in (-1, 0, 1):
                    cand += grid.get((cx + dx, cy + dy), [])
            for ii, p in lst:
                for jj, q in cand:
                    if jj <= ii:
                        continue
                    d = math.hypot(p[0] - q[0], p[1] - q[1])
                    if d > 0.75:
                        continue
                    ink = d - (runs[ii][0]["pen"] + runs[jj][0]["pen"]) / 2
                    key = (ii, jj)
                    prev = best.get(key)
                    if prev is None or ink < prev[0]:
                        best[key] = (ink, p)
        for (ii, jj), (ink, p) in best.items():
            if 0 < ink < MIN_GAP + 0.005:
                a, b = runs[ii][0]["id"], runs[jj][0]["id"]
                key = "+".join(sorted([a, b]))
                prev = out["gap"].get(key)
                if prev is None or ink < prev[0]:
                    out["gap"][key] = (ink, p[0], p[1], pose["id"])
        if features:
            for it in items:
                if it["filled"]:
                    bx = it["box"]
                    if min(bx[2] - bx[0], bx[3] - bx[1]) < 0.08:
                        out["feature"][it["id"]] = (0.05, pose["id"])
                        continue
                    area = sum(abs(sum(r[i][0] * r[(i + 1) % len(r)][1] - r[(i + 1) % len(r)][0] * r[i][1] for i in range(len(r)))) / 2 for r in it["rings"])
                    perim = sum(math.hypot(r[i][0] - r[(i + 1) % len(r)][0], r[i][1] - r[(i + 1) % len(r)][1]) for r in it["rings"] for i in range(len(r)))
                    if perim > 0 and 4 * area / perim < 1.1 and area < 400:
                        lost, tol = thin_features(it, 0.22)
                        if lost > tol:
                            out["feature"][it["id"]] = (round(lost / tol, 2), pose["id"])
    return out


def sig(s):
    import re
    keep = {k: v for k, v in s.items() if k in ("kind", "bone", "role", "d", "points", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "closed", "rotationDeg")}
    t = json.dumps(keep, sort_keys=True)
    return re.sub(r"-?\d+\.\d+", lambda m: "%.4f" % float(m.group()), t)


def copies(asset):
    reps = {int(k): v for k, v in asset["representations"].items() if v["status"] == "authored"}
    res = []
    dens = sorted(reps, reverse=True)
    for i, c in enumerate(dens):
        for f in dens[i + 1:]:
            a = [sig(s) for s in reps[c]["geometry"]["elevation"]]
            pool = {}
            for s in reps[f]["geometry"]["elevation"]:
                pool[sig(s)] = pool.get(sig(s), 0) + 1
            shared = 0
            for x in a:
                if pool.get(x, 0) > 0:
                    shared += 1
                    pool[x] -= 1
            if a and shared == len(a):
                res.append((c, f, "subset/identical", shared, len(a)))
            elif a and shared > 0:
                res.append((c, f, "partial", shared, len(a)))
    return res


def main():
    f = sys.argv[1]
    scales = [int(x) for x in (sys.argv[2] if len(sys.argv) > 2 and not sys.argv[2].startswith("--") else "500,200,100,50").split(",")]
    all_poses = "--all-poses" in sys.argv or True
    doc = json.load(open(f))
    asset = doc["asset"]
    bad = 0
    for c, f, kind, sh, n in copies(asset):
        print("copies 1:%d vs 1:%d %s shared %d of %d" % (c, f, kind, sh, n))
        if kind != "partial":
            bad += 1
    for den in scales:
        r = measure(asset, den, all_poses)
        if r is None:
            print("1:%d unavailable" % den)
            continue
        n = len(r["gap"]) + len(r["feature"]) + len(r["stroke"]) + len(r["hatch"])
        bad += n
        print("1:%d %s gaps %d features %d stroke %d hatch %d" % (den, "OK" if n == 0 else "FAIL", len(r["gap"]), len(r["feature"]), len(r["stroke"]), len(r["hatch"])))
        for key, (ink, x, y, pose) in sorted(r["gap"].items(), key=lambda t: t[1][0]):
            print("   gap %-40s %.3f @%.1f,%.1f m(%.2f,%.2f) %s" % (key, ink, x, y, x * den / 1000, -y * den / 1000, pose))
        for key, (v, pose) in r["feature"].items():
            print("   feature %-30s thin-area/tol %s %s" % (key, v, pose))
        for x in r["stroke"] + r["hatch"]:
            print("   pen/hatch", x)
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
