import math
from lib import *

D2R = math.pi / 180


def polar(r, deg):
    return (r * math.cos(deg * D2R), r * math.sin(deg * D2R))


def ribbon(center, hw, closed_tip=True):
    """Outline around a polyline; hw is a list of half widths per point."""
    n = len(center)
    left, right = [], []
    for i, (x, y) in enumerate(center):
        if i == 0:
            dx, dy = center[1][0] - x, center[1][1] - y
        elif i == n - 1:
            dx, dy = x - center[i - 1][0], y - center[i - 1][1]
        else:
            dx, dy = center[i + 1][0] - center[i - 1][0], center[i + 1][1] - center[i - 1][1]
        L = math.hypot(dx, dy) or 1.0
        nx, ny = -dy / L, dx / L
        left.append((x + nx * hw[i], y + ny * hw[i]))
        right.append((x - nx * hw[i], y - ny * hw[i]))
    return left + right[::-1]


def lerp(a, b, t):
    return a + (b - a) * t


def seg(p, q, n):
    return [(lerp(p[0], q[0], i / n), lerp(p[1], q[1], i / n)) for i in range(n + 1)]


def bow(p, q, n, bulge):
    out = []
    dx, dy = q[0] - p[0], q[1] - p[1]
    L = math.hypot(dx, dy) or 1.0
    nx, ny = -dy / L, dx / L
    for i in range(n + 1):
        t = i / n
        k = bulge * math.sin(math.pi * t)
        out.append((lerp(p[0], q[0], t) + nx * k, lerp(p[1], q[1], t) + ny * k))
    return out


def prof(n, base, mid, tip, peak=0.45):
    out = []
    for i in range(n + 1):
        t = i / n
        if t <= peak:
            w = lerp(base, mid, math.sin(0.5 * math.pi * t / peak))
        else:
            w = lerp(mid, tip, ((t - peak) / (1 - peak)) ** 1.3)
        out.append(w)
    return out


def lobe(id, bone, cx, cy, rx, ry, n, depth, tone, z, phase=0.0, seed=0, region=None, regions=None, hatch=None, rot=0.0, steps=None, role="body"):
    pts = lobed(cx, cy, rx, ry, n, depth, phase=phase, seed=seed, steps=steps or n * 5, rot=rot)
    return poly(id, bone, role, z, tone, pts, region=region, regions=regions, hatch=hatch)


def arcs(prefix, bone, cx, cy, rx, ry, count, seed, tone, z, role="texture", span=(60, 110), size=(0.22, 0.34), region=None, followTip=False):
    import random
    rnd = random.Random(seed)
    out = []
    for i in range(count):
        t = rnd.random() * 2 * math.pi
        k = 0.15 + 0.6 * math.sqrt(rnd.random())
        px, py = cx + rx * k * math.cos(t), cy + ry * k * math.sin(t) * 0.8
        rad = rx * (size[0] + (size[1] - size[0]) * rnd.random())
        a0 = rnd.uniform(200, 250)
        a1 = a0 + rnd.uniform(*span)
        pts = [(px + rad * math.cos((a0 + (a1 - a0) * j / 5) * D2R), py + rad * 0.6 * math.sin((a0 + (a1 - a0) * j / 5) * D2R)) for j in range(6)]
        out.append(poly(f"{prefix}-{i + 1}", bone, role, z + 0.001 * i, tone, pts, closed=False, followTip=followTip, region=region))
    return out


def dots(prefix, bone, centres, r, tone, z, region, role="detail"):
    return [ellipse(f"{prefix}-{i + 1}", bone, role, z, tone, x, y, r, r, region=region) for i, (x, y) in enumerate(centres)]


def union_outline(ells, cx0, cy0, n=90, step=0.02, rmax=30.0):
    """Outline of a union of axis-aligned ellipses (cx, cy, rx, ry) found by rays from a point inside it."""
    pts = []
    for i in range(n):
        t = 2 * math.pi * i / n
        dx, dy = math.cos(t), math.sin(t)
        far = 0.0
        for k in range(1, int(rmax / step)):
            r = k * step
            x, y = cx0 + r * dx, cy0 + r * dy
            if any(((x - ex) / erx) ** 2 + ((y - ey) / ery) ** 2 <= 1 for ex, ey, erx, ery in ells):
                far = r
        pts.append((cx0 + far * dx, cy0 + far * dy))
    return pts


def install(asset, views, notes):
    """views: {scale:int -> (front, plan)}, notes: {scale -> (en, de)} appended to the decisions."""
    doc = load(asset)
    a = doc["asset"]
    for sc, (front, plan) in views.items():
        rep = a["representations"][str(sc)]
        assert rep["status"] == "authored"
        ids = [s["id"] for s in front]
        assert len(ids) == len(set(ids)), ("dup front", sc, [i for i in ids if ids.count(i) > 1][:5])
        ids = [s["id"] for s in plan]
        assert len(ids) == len(set(ids)), ("dup plan", sc, [i for i in ids if ids.count(i) > 1][:5])
        g = rep["geometry"]
        rep["geometry"] = {"elevation": g["elevation"], "front": front, "plan": plan}
        for k in g:
            if k not in rep["geometry"]:
                rep["geometry"][k] = g[k]
        en, de = notes[sc]
        for key in ("silhouette",):
            t = rep["decisions"][key]
            marker = " Front and plan: "
            if marker in t["en"]:
                t["en"] = t["en"].split(marker)[0]
                t["de"] = t["de"].split(" Vorder- und Draufsicht: ")[0]
            t["en"] = t["en"] + marker + en
            t["de"] = t["de"] + " Vorder- und Draufsicht: " + de
    a["presentation"].pop("viewpoints", None)
    save(asset, doc)


def extents(asset, scales=None):
    """Rest-pose extents of the three views per scale, ground excluded, against the dimensions."""
    doc = load(asset)
    a = doc["asset"]
    r, bones = rest(doc)
    dim = a["dimensions"]
    print(asset, "dims w/d/h", dim["widthM"], dim["depthM"], dim["heightM"])
    for sc, rep in a["representations"].items():
        if rep.get("status") != "authored" or (scales and int(sc) not in scales):
            continue
        g = rep["geometry"]

        def bb(view, plan=False, front=False):
            pts = []
            for s in g[view]:
                if s["role"] == "ground":
                    continue
                if view == "elevation":
                    pts += world_poly(r, s)
                else:
                    pts += local_pts(s)
            return bbox(pts) if pts else None

        e, f, p = bb("elevation"), bb("front"), bb("plan")
        tol = max(0.02, 0.03 * max(dim["widthM"], dim["depthM"], dim["heightM"]))
        line = f"  {sc:>4} tol {tol:.3f}"
        if e:
            line += f" E x[{e[0]:.2f},{e[2]:.2f}] y[{e[1]:.2f},{e[3]:.2f}]"
        if p:
            line += f" P x[{p[0]:.2f},{p[2]:.2f}] y[{p[1]:.2f},{p[3]:.2f}]"
        if f:
            line += f" F x[{f[0]:.2f},{f[2]:.2f}] y[{f[1]:.2f},{f[3]:.2f}]"
        bad = []
        if e and p:
            if abs(e[0] - p[0]) > tol or abs(e[2] - p[2]) > tol:
                bad.append("plan-x")
            if abs((p[3] - p[1]) - dim["depthM"]) > tol:
                bad.append("depth")
        if e and f:
            if abs(e[1] - f[1]) > tol or abs(e[3] - f[3]) > tol:
                bad.append("front-y")
        if f and p:
            if abs(f[0] - p[1]) > tol or abs(f[2] - p[3]) > tol:
                bad.append("lateral")
        if e and abs((e[2] - e[0]) - dim["widthM"]) > tol:
            bad.append("width")
        if e and abs((e[3] - 0) - dim["heightM"]) > tol:
            bad.append("height")
        print(line, "BAD:" + ",".join(bad) if bad else "ok")


def local_pts(s):
    k = s["kind"]
    if k == "poly":
        return [tuple(p) for p in s["points"]]
    if k == "rect":
        return [(s["x"], s["y"]), (s["x"] + s["w"], s["y"] + s["h"])]
    if k == "ellipse":
        return [(s["cx"] - s["rx"], s["cy"] - s["ry"]), (s["cx"] + s["rx"], s["cy"] + s["ry"])]
    if k == "capsule":
        return [(s["x0"] - s["r0"], s["y0"] - s["r0"]), (s["x0"] + s["r0"], s["y0"] + s["r0"]), (s["x1"] - s["r1"], s["y1"] - s["r1"]), (s["x1"] + s["r1"], s["y1"] + s["r1"])]
    if k == "path":
        return path_points(s["d"])
    return []
