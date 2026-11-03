"""Shared helpers of content worker VE: asset I/O, shape constructors, rest-pose rig, extents.

Front and plan coordinates are rest-pose model metres (the bone only supplies travel); elevation coordinates are bone-local.
"""
import json
import math
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
T = os.path.dirname(HERE)
ROOT = os.path.abspath(os.path.join(T, *[".."] * 7))
CATALOG = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🚜️vehicles")


def path_of(aid):
    return os.path.join(CATALOG, aid, "🔣️.json")


def load(aid):
    """The pristine asset: the first state this worker saw (cached in orig/), so that every build is idempotent."""
    cache = os.path.join(HERE, "orig", aid + ".json")
    if not os.path.exists(cache):
        os.makedirs(os.path.dirname(cache), exist_ok=True)
        with open(path_of(aid), encoding="utf-8", newline="") as f, open(cache, "w", encoding="utf-8", newline="") as g:
            g.write(f.read())
    with open(cache, encoding="utf-8") as f:
        return json.load(f)


def raw(aid):
    with open(path_of(aid), encoding="utf-8", newline="") as f:
        return f.read()


def dumps(doc):
    return json.dumps(doc, ensure_ascii=False, indent=1)


def write_atomic(path, text):
    """Writes through a temporary file in the same folder and os.replace, so that concurrent readers never see half a file."""
    import tempfile
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(path), suffix=".tmp")
    try:
        with os.fdopen(fd, "w", encoding="utf-8", newline="") as f:
            f.write(text)
        for attempt in range(60):
            try:
                os.replace(tmp, path)
                return
            except OSError:
                import time
                time.sleep(0.5)
        raise OSError("cannot replace " + path)
    finally:
        if os.path.exists(tmp):
            os.remove(tmp)


def save(aid, doc):
    text = dumps(doc)
    original = raw(aid)
    if original.endswith("\n"):
        text += "\n"
    write_atomic(path_of(aid), text)


def r4(v):
    v = round(v, 4)
    return 0.0 if v == 0 else v


# ---------------------------------------------------------------- geometry constructors
def rect(x, y, w, h, r=None, follow=False):
    g = {"kind": "rect", "x": r4(x), "y": r4(y), "w": r4(w), "h": r4(h)}
    if r:
        g["r"] = r4(r)
    if follow:
        g["followTip"] = True
    return g


def box(x0, y0, x1, y1, r=None, follow=False):
    return rect(min(x0, x1), min(y0, y1), abs(x1 - x0), abs(y1 - y0), r, follow)


def ell(cx, cy, rx, ry=None, rot=None):
    g = {"kind": "ellipse", "cx": r4(cx), "cy": r4(cy), "rx": r4(rx), "ry": r4(rx if ry is None else ry)}
    if rot:
        g["rotationDeg"] = rot
    return g


def poly(points, closed=True, follow=False):
    g = {"kind": "poly", "points": [[r4(x), r4(y)] for x, y in points], "closed": closed}
    if follow:
        g["followTip"] = True
    return g


def cap(x0, y0, x1, y1, r0, r1=None, follow=False):
    g = {"kind": "capsule", "x0": r4(x0), "y0": r4(y0), "x1": r4(x1), "y1": r4(y1), "r0": r4(r0), "r1": r4(r0 if r1 is None else r1)}
    if follow:
        g["followTip"] = True
    return g


def path(d):
    return {"kind": "path", "d": d}


def P(*pts):
    """SVG path through points, closed: 'M x,y L x,y ... Z'"""
    return "M " + " L ".join(f"{r4(x)},{r4(y)}" for x, y in pts) + " Z"


def S(id, bone, role, z, geom, tone=0.5, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
    s = dict(geom)
    s["id"] = id
    s["bone"] = bone
    s["role"] = role
    s["z"] = z
    s["filled"] = filled
    s["tone"] = tone
    if region:
        s["region"] = region
    if regions:
        s["regions"] = list(regions)
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    return s


# ---------------------------------------------------------------- rest-pose rig
def rest_world(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    cache = {}

    def solve(bid):
        if bid in cache:
            return cache[bid]
        b = bones[bid]
        a = math.radians(b.get("angleDeg", 0))
        if b["parent"] is None:
            res = (b["x"], b["y"], a)
        else:
            px, py, pa = solve(b["parent"])
            res = (px + b["x"] * math.cos(pa) - b["y"] * math.sin(pa), py + b["x"] * math.sin(pa) + b["y"] * math.cos(pa), pa + a)
        cache[bid] = res
        return res

    return {bid: solve(bid) for bid in bones}


def to_world(w, bone, x, y):
    ox, oy, a = w[bone]
    return (ox + x * math.cos(a) - y * math.sin(a), oy + x * math.sin(a) + y * math.cos(a))


def tip(asset, w, bone):
    ln = next(b for b in asset["rig"]["bones"] if b["id"] == bone)["lengthM"]
    return to_world(w, bone, ln, 0)


# ---------------------------------------------------------------- extents (rest pose)
def _bez(p0, p1, p2, p3, n=24):
    return [tuple((1 - t) ** 3 * p0[i] + 3 * (1 - t) ** 2 * t * p1[i] + 3 * (1 - t) * t * t * p2[i] + t ** 3 * p3[i] for i in (0, 1)) for t in [k / n for k in range(1, n + 1)]]


def _quad(p0, p1, p2, n=16):
    return [tuple((1 - t) ** 2 * p0[i] + 2 * (1 - t) * t * p1[i] + t * t * p2[i] for i in (0, 1)) for t in [k / n for k in range(1, n + 1)]]


def path_points(d):
    toks = re.findall(r"[MLHVCQZmlhvcqz]|-?\d*\.?\d+(?:e-?\d+)?", d)
    pts, i, cmd, cur, start = [], 0, None, (0.0, 0.0), (0.0, 0.0)

    def num():
        nonlocal i
        v = float(toks[i])
        i += 1
        return v

    while i < len(toks):
        if re.match(r"[A-Za-z]", toks[i]):
            cmd = toks[i]
            i += 1
            if cmd in "Zz":
                cur = start
                continue
        rel = cmd.islower()
        c = cmd.upper()
        ox, oy = cur if rel else (0.0, 0.0)
        if c == "M":
            cur = (ox + num(), oy + num())
            start = cur
            pts.append(cur)
            cmd = "l" if rel else "L"
        elif c == "L":
            cur = (ox + num(), oy + num())
            pts.append(cur)
        elif c == "H":
            cur = ((cur[0] if rel else 0) + num(), cur[1])
            pts.append(cur)
        elif c == "V":
            cur = (cur[0], (cur[1] if rel else 0) + num())
            pts.append(cur)
        elif c == "C":
            p1 = (ox + num(), oy + num())
            p2 = (ox + num(), oy + num())
            p3 = (ox + num(), oy + num())
            pts += _bez(cur, p1, p2, p3)
            cur = p3
        elif c == "Q":
            p1 = (ox + num(), oy + num())
            p2 = (ox + num(), oy + num())
            pts += _quad(cur, p1, p2)
            cur = p2
        else:
            raise ValueError("path command " + cmd)
    return pts


def shape_points(s):
    k = s["kind"]
    if k == "rect":
        return [(s["x"], s["y"]), (s["x"] + s["w"], s["y"] + s["h"])]
    if k == "poly":
        return [tuple(p) for p in s["points"]]
    if k == "ellipse":
        rot = math.radians(s.get("rotationDeg", 0))
        hx = math.hypot(s["rx"] * math.cos(rot), s["ry"] * math.sin(rot))
        hy = math.hypot(s["rx"] * math.sin(rot), s["ry"] * math.cos(rot))
        return [(s["cx"] - hx, s["cy"] - hy), (s["cx"] + hx, s["cy"] + hy)]
    if k == "capsule":
        return [(s["x0"] - s["r0"], s["y0"] - s["r0"]), (s["x0"] + s["r0"], s["y0"] + s["r0"]), (s["x1"] - s["r1"], s["y1"] - s["r1"]), (s["x1"] + s["r1"], s["y1"] + s["r1"])]
    if k == "path":
        return path_points(s["d"])
    raise ValueError(k)


def bbox(points):
    xs = [p[0] for p in points]
    ys = [p[1] for p in points]
    return (min(xs), min(ys), max(xs), max(ys))


def elevation_bbox(asset, shapes):
    w = rest_world(asset)
    pts = []
    for s in shapes:
        if s["role"] == "ground":
            continue
        pts += [to_world(w, s["bone"], x, y) for x, y in shape_points(s)]
    return bbox(pts)


def view_bbox(shapes, asset=None, view=None):
    pts = []
    w = rest_world(asset) if asset is not None else None
    for s in shapes:
        if s["role"] == "ground":
            continue
        if "solid" in s:
            wp = [to_world(w, s["bone"], x, y) for x, y in shape_points(s)]
            z0, z1 = s["solid"]["z0"], s["solid"]["z1"]
            b = bbox(wp)
            pts += [(z0, b[1]), (z1, b[3])] if view == "front" else [(b[0], z0), (b[2], z1)]
        else:
            pts += shape_points(s)
    return bbox(pts)


def report(asset, scale):
    g = asset["representations"][scale]["geometry"]
    e = elevation_bbox(asset, g["elevation"])
    f = view_bbox(g["front"], asset, "front") if g.get("front") else None
    p = view_bbox(g["plan"], asset, "plan") if g.get("plan") else None
    d = asset["dimensions"]
    out = [f"  {scale}: elev x[{e[0]:.3f},{e[2]:.3f}] y[{e[1]:.3f},{e[3]:.3f}] w={e[2]-e[0]:.3f}/{d['widthM']} h={e[3]-e[1]:.3f}/{d['heightM']}"]
    if f:
        out.append(f"       front x[{f[0]:.3f},{f[2]:.3f}] y[{f[1]:.3f},{f[3]:.3f}] depth={f[2]-f[0]:.3f}/{d['depthM']} h={f[3]-f[1]:.3f}")
    if p:
        out.append(f"       plan  x[{p[0]:.3f},{p[2]:.3f}] y[{p[1]:.3f},{p[3]:.3f}] w={p[2]-p[0]:.3f} depth={p[3]-p[1]:.3f}")
    return "\n".join(out)


def apply_views(asset, scale, front, plan, decisions=None):
    rep = asset["representations"][scale]
    rep["geometry"]["front"] = front
    rep["geometry"]["plan"] = plan
    order = {"elevation": rep["geometry"]["elevation"], "front": front, "plan": plan}
    rep["geometry"].clear()
    rep["geometry"].update(order)
    if decisions:
        for key, (en, de) in decisions.items():
            cur = rep["decisions"][key]
            cur["en"] = cur["en"].rstrip() + " " + en
            cur["de"] = cur["de"].rstrip() + " " + de


def strip_viewpoints(asset):
    asset["presentation"].pop("viewpoints", None)


# ---------------------------------------------------------------- solid (extruded) rebinding
GEOM_KEYS = {"kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "rotationDeg", "points", "closed", "x0", "y0", "x1", "y1", "r0", "r1", "followTip", "d", "solid"}


def ext(profile, z0, z1):
    g = dict(profile)
    g.pop("followTip", None)
    g["solid"] = {"kind": "extruded", "z0": r4(z0), "z1": r4(z1)}
    return g


def rebind(shapes, rules):
    """rules: id -> None (drop) | (bone, geometry[, overrides])"""
    out = []
    for s in shapes:
        rule = rules.get(s["id"], 0)
        if rule == 0:
            out.append(s)
            continue
        if rule is None:
            continue
        new = dict(rule[1])
        new.update({k: v for k, v in s.items() if k not in GEOM_KEYS})
        new["bone"] = rule[0]
        if len(rule) > 2:
            new.update(rule[2])
        out.append(new)
    return out


def apply_rules(asset, rules):
    for scale, views in rules.items():
        for view, table in views.items():
            geo = asset["representations"][scale]["geometry"]
            geo[view] = rebind(geo[view], table)
