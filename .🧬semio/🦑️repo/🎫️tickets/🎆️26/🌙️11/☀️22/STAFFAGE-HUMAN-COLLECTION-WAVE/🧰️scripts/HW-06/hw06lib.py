"""🧰️ Shared helpers of HW-06: catalog IO, the rig chain of `🔨️modules/🦴️rig` (rest and posed frames), per-bone rescaling of a
derived figure (rig, 2D shapes of all views, 3D solids, anchors) and small geometry builders for new parts.

See `🎓️teaching/🏛️architecture/🧍️staffage/README.md` (Rest placement of a bone, Solid placements, Volume)."""
import copy
import json
import math
import os
import re

R = "/Users/ueli/Documents/semio"
S = f"{R}/🎓️teaching/🏛️architecture/🧍️staffage"
HUMANS = f"{S}/🗂️catalog/👤️humans"
T = f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
SCALES = ["500", "200", "100", "50"]
VIEWS = ["elevation", "front", "plan"]


def load(slug):
    with open(f"{HUMANS}/{slug}/🔣️.json", encoding="utf-8") as f:
        return json.load(f)


def save(slug, doc):
    os.makedirs(f"{HUMANS}/{slug}", exist_ok=True)
    path = f"{HUMANS}/{slug}/🔣️.json"
    text = json.dumps(doc, ensure_ascii=False, indent=2) + "\n"
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)
    return path


def t(en, de):
    return {"en": en, "de": de}


def one_text(texts):
    """🧷️ Joins several Texts into the one Text a schema member such as provenance.notes holds."""
    return {language: " ".join(text[language] for text in texts) for language in ("en", "de")}


def rnd(value, digits=5):
    if isinstance(value, float):
        r = round(value, digits)
        return 0.0 if r == 0 else r
    if isinstance(value, list):
        return [rnd(v, digits) for v in value]
    if isinstance(value, dict):
        return {k: rnd(v, digits) for k, v in value.items()}
    return value


# ---------------------------------------------------------------- rig chain (mirror of 🦴️rig `chain`)

def _sc(deg):
    r = math.radians(deg)
    return math.sin(r), math.cos(r)


def _comb(a, ka, b, kb):
    return [a[i] * ka + b[i] * kb for i in range(3)]


def turned(axes, angle, abduction, twist):
    u, v, w = axes
    ts, tc = _sc(angle)
    u1 = _comb(u, tc, v, ts)
    v1 = _comb(u, -ts, v, tc)
    ps, pc = _sc(abduction)
    u2 = _comb(u1, pc, w, ps)
    w2 = _comb(u1, -ps, w, pc)
    ws, wc = _sc(twist)
    return u2, _comb(v1, wc, w2, -ws), _comb(v1, ws, w2, wc)


def effective(bone, off):
    off = off or {}
    j = bone["joint"]
    if j["type"] == "revolute":
        def cl(v, lo, hi):
            if lo is not None and v < lo: return lo
            if hi is not None and v > hi: return hi
            return v
        return (cl(off.get("angleDeg", 0), j.get("minDeg"), j.get("maxDeg")), cl(off.get("abductionDeg", 0), -180, 180),
                cl(off.get("twistDeg", 0), j.get("twistMinDeg", -180), j.get("twistMaxDeg", 180)), 0.0, 0.0)
    if j["type"] == "prismatic":
        s, c = _sc(j.get("axisDeg", 0))
        d = off.get("dx", 0) * c + off.get("dy", 0) * s
        d = max(j.get("minM", -1e9), min(j.get("maxM", 1e9), d))
        return 0.0, 0.0, 0.0, d * c, d * s
    return 0.0, 0.0, 0.0, 0.0, 0.0


def order(bones):
    by = {b["id"]: b for b in bones}
    out, seen = [], set()

    def visit(b):
        if b["id"] in seen: return
        if b["parent"] is not None: visit(by[b["parent"]])
        seen.add(b["id"]); out.append(b)
    for b in bones: visit(b)
    return out


def chain(bones, offsets=None, root=(0.0, 0.0, 0.0)):
    """Frames {id: {o,u,v,w}} exactly as `solveChain` (root = x, y, angleDeg)."""
    offsets = offsets or {}
    rs, rc = _sc(root[2])
    base = {"o": [root[0], root[1], 0.0], "u": [rc, rs, 0.0], "v": [-rs, rc, 0.0], "w": [0.0, 0.0, 1.0]}
    res = {}
    for b in order(bones):
        p = res[b["parent"]] if b["parent"] is not None else base
        a, ab, tw, dx, dy = effective(b, offsets.get(b["id"]))
        along = _comb(p["u"], b["x"] + dx, p["v"], b["y"] + dy)
        side = b.get("z", 0)
        o = [p["o"][i] + along[i] + p["w"][i] * side for i in range(3)]
        ab0, tw0 = b.get("abductionDeg", 0), b.get("twistDeg", 0)
        axes = (p["u"], p["v"], p["w"])
        if ab0 == 0 and tw0 == 0: u, v, w = turned(axes, b["angleDeg"] + a, ab, tw)
        elif tw0 == 0 and a == 0: u, v, w = turned(axes, b["angleDeg"], ab0 + ab, tw)
        elif a == 0 and ab == 0: u, v, w = turned(axes, b["angleDeg"], ab0, tw0 + tw)
        else: u, v, w = turned(turned(axes, b["angleDeg"], ab0, tw0), a, ab, tw)
        res[b["id"]] = {"o": o, "u": u, "v": v, "w": w, "L": b["lengthM"]}
    return res


def to_world(fr, p):
    return [fr["o"][i] + p[0] * fr["u"][i] + p[1] * fr["v"][i] + (p[2] if len(p) > 2 else 0) * fr["w"][i] for i in range(3)]


def to_local(fr, q):
    d = [q[i] - fr["o"][i] for i in range(3)]
    return [sum(d[i] * fr[a][i] for i in range(3)) for a in ("u", "v", "w")]


def mat(fr):
    return [[fr["u"][i], fr["v"][i], fr["w"][i]] for i in range(3)]


def mmul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def mT(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def euler_of(m):
    """(angle, abduction, twist) with turned(I, a, b, c) == columns of m (u, v, w)."""
    u = [m[0][0], m[1][0], m[2][0]]
    b = math.degrees(math.asin(max(-1, min(1, u[2]))))
    a = math.degrees(math.atan2(u[1], u[0]))
    best = None
    for tw in [x * 0.5 for x in range(-360, 361)]:
        uu, vv, ww = turned(([1, 0, 0], [0, 1, 0], [0, 0, 1]), a, b, tw)
        err = sum((vv[i] - m[i][1]) ** 2 for i in range(3))
        if best is None or err < best[0]: best = (err, tw)
    lo, hi = best[1] - 0.5, best[1] + 0.5
    for _ in range(60):
        m1, m2 = lo + (hi - lo) / 3, hi - (hi - lo) / 3
        e = [sum((turned(([1, 0, 0], [0, 1, 0], [0, 0, 1]), a, b, x)[1][i] - m[i][1]) ** 2 for i in range(3)) for x in (m1, m2)]
        if e[0] < e[1]: hi = m2
        else: lo = m1
    return a, b, (lo + hi) / 2


# ---------------------------------------------------------------- per-bone rescale of a derived figure

_NUM = re.compile(r"-?\d*\.?\d+(?:[eE][-+]?\d+)?")


def _scale_path(d, fx):
    """Scale an SVG path by `fx((x, y)) -> (x, y)` for absolute M/L/C/Q/Z commands only."""
    tokens = re.findall(r"[MLCQZHVAmlcqzhva]|-?\d*\.?\d+(?:[eE][-+]?\d+)?", d)
    out, nums, cmd = [], [], None

    def flush():
        if cmd is None: return
        if cmd in "Zz": out.append(cmd); return
        assert cmd in "MLCQ", f"path command {cmd} unsupported"
        pts = []
        for i in range(0, len(nums), 2):
            x, y = fx((nums[i], nums[i + 1]))
            pts.append(f"{rnd(x, 5)} {rnd(y, 5)}")
        out.append(cmd + " " + " ".join(pts))
    for tok in tokens:
        if re.match(r"[A-Za-z]", tok):
            flush(); cmd, nums = tok, []
        else:
            nums.append(float(tok))
    flush()
    return " ".join(out)


def _map_shape(sh, fx):
    k = sh["kind"]
    if k == "poly": sh["points"] = [list(fx(p)) for p in sh["points"]]
    elif k == "path": sh["d"] = _scale_path(sh["d"], fx)
    elif k == "capsule":
        sh["x0"], sh["y0"] = fx((sh["x0"], sh["y0"])); sh["x1"], sh["y1"] = fx((sh["x1"], sh["y1"]))
    elif k == "ellipse": sh["cx"], sh["cy"] = fx((sh["cx"], sh["cy"]))
    elif k == "rect": sh["x"], sh["y"] = fx((sh["x"], sh["y"]))
    else: raise ValueError(k)


def _scale_sizes(sh, f):
    k = sh["kind"]
    if k == "capsule": sh["r0"] *= f; sh["r1"] *= f
    elif k == "ellipse": sh["rx"] *= f; sh["ry"] *= f
    elif k == "rect":
        sh["w"] *= f; sh["h"] *= f
        if "r" in sh: sh["r"] *= f


def rescale_shapes(asset, factor, old, new):
    """Every shape of every authored scale follows its bone: elevation and solid shapes are bone-local (× f), plain front and plan
    shapes are world rest coordinates carried about the bone's rest origin (new origin + f · (p − old origin))."""
    for key, rep in asset["representations"].items():
        if rep["status"] != "authored": continue
        for view, shapes in rep["geometry"].items():
            for sh in shapes:
                b = sh["bone"]; f = factor.get(b, 1.0)
                o0, o1 = old[b]["o"], new[b]["o"]
                if view == "elevation" or "solid" in sh:
                    _map_shape(sh, lambda p: (p[0] * f, p[1] * f)); _scale_sizes(sh, f)
                else:
                    ia, ib = (2, 1) if view == "front" else (0, 2)
                    _map_shape(sh, lambda p: (o1[ia] + f * (p[0] - o0[ia]), o1[ib] + f * (p[1] - o0[ib]))); _scale_sizes(sh, f)
                sol = sh.get("solid")
                if sol is None: continue
                if sol["kind"] == "revolved":
                    fa = factor.get(sol["axisBone"], 1.0)
                    sol["axisX"] *= fa
                    if "axisZ" in sol: sol["axisZ"] *= fa
                elif sol["sweep"] == "w":
                    oz0, oz1 = o0[2], o1[2]
                    sol["z0"] = oz1 + f * (sol["z0"] - oz0); sol["z1"] = oz1 + f * (sol["z1"] - oz0)
                    if "zTip" in sol: sol["zTip"] = [oz1 + f * (z - oz0) for z in sol["zTip"]]
                else:
                    sol["z0"] *= f; sol["z1"] *= f
                    if "zTip" in sol: sol["zTip"] = [z * f for z in sol["zTip"]]


FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}


def thinness(so):
    k = so["kind"]
    if k == "loft": return max(2 * min(s["rx"], s["ry"]) for s in so["sections"])
    if k == "ellipsoid": return 2 * min(so["radii"])
    if k == "capsule": return 2 * max(so["r0"], so["r1"])
    if k == "box": return min(so["size"])
    if k == "extrude": return abs(so["to"] - so["from"])
    if k == "revolve": return 2 * max(p[1] for p in so["profile"])
    return 1.0


def scale_solid(so, f, feature=0.0):
    """Positions × f; cross-sections × f as well, unless that would take a solid that met the smallest feature of its scale below
    it: then its cross-section keeps exactly the feature (the authored floor of that scale)."""
    if "of" in so: return
    k = so["kind"]
    fc = f
    if feature and not so.get("slender") and not so.get("sheet"):
        th = thinness(so)
        if th >= feature * 0.999 and th * f < feature: fc = feature / th
    if k == "box":
        so["size"] = [x * fc for x in so["size"]]
        if "cornerRadius" in so: so["cornerRadius"] *= fc
    elif k == "ellipsoid": so["radii"] = [x * fc for x in so["radii"]]
    elif k == "capsule":
        so["p0"] = [x * f for x in so["p0"]]; so["p1"] = [x * f for x in so["p1"]]; so["r0"] *= fc; so["r1"] *= fc
    elif k == "revolve":
        so["profile"] = [[a * f, b * fc] for a, b in so["profile"]]; so["origin"] = [x * f for x in so["origin"]]
    elif k == "extrude":
        so["profile"] = [[a * f, b * f] for a, b in so["profile"]]
        if "holes" in so: so["holes"] = [[[a * f, b * f] for a, b in h] for h in so["holes"]]
        mid = (so["from"] + so["to"]) / 2; half = (so["to"] - so["from"]) / 2
        so["from"] = mid * f - half * fc; so["to"] = mid * f + half * fc
    elif k == "polyhedron": so["vertices"] = [[x * f for x in v] for v in so["vertices"]]
    elif k == "loft":
        for s in so["sections"]:
            s["at"] = [x * f for x in s["at"]]; s["rx"] *= fc; s["ry"] *= fc
    else: raise ValueError(k)
    tr = so.get("transform")
    if tr and "translation" in tr: tr["translation"] = [x * f for x in tr["translation"]]
    rp = so.get("repeat")
    if rp:
        if "translation" in rp: rp["translation"] = [x * f for x in rp["translation"]]
        if "origin" in rp: rp["origin"] = [x * f for x in rp["origin"]]


def rescale_volumes(asset, factor):
    for key, vol in asset["volumes"].items():
        if vol["status"] != "authored": continue
        for lst in ("body", "thermal", "structure"):
            for so in vol[lst]:
                if "of" in so: continue
                fs = [factor.get(w["bone"], 1.0) for w in so.get("weights", [])] or [factor.get(so["bone"], 1.0)]
                f = sum(fs) / len(fs)
                scale_solid(so, f, FEATURE[key])


def rescale(asset, factor, root_fix=None, old=None):
    """Scale bone lengths by their own factor, child offsets by the parent's factor, then every shape, solid, anchor, centre of mass
    and pose `dy`/`dx`. `root_fix(bone)` may overwrite the placement of the bones whose parent keeps factor 1 (returns x, y)."""
    bones = asset["rig"]["bones"]
    old = old or chain(bones)
    for b in bones:
        f = factor.get(b["id"], 1.0)
        fp = factor.get(b["parent"], 1.0) if b["parent"] else 1.0
        b["lengthM"] *= f
        b["x"] *= fp; b["y"] *= fp
        if "z" in b: b["z"] *= fp
        if "mass" in b:
            b["mass"]["comX"] *= f; b["mass"]["comY"] *= f
        if root_fix:
            fixed = root_fix(b)
            if fixed: b["x"], b["y"] = fixed
    new = chain(bones)
    rescale_shapes(asset, factor, old, new)
    rescale_volumes(asset, factor)
    for a in asset["anchors"]:
        f = factor.get(a["bone"], 1.0)
        a["x"] *= f; a["y"] *= f
        if "z" in a: a["z"] *= f
    return old, new


def walk_text(node, fn):
    if isinstance(node, dict):
        if set(node.keys()) == {"en", "de"} and all(isinstance(v, str) for v in node.values()):
            return fn(node)
        return {k: walk_text(v, fn) for k, v in node.items()}
    if isinstance(node, list): return [walk_text(v, fn) for v in node]
    return node


def drop_bones(asset, ids):
    """Remove bones and everything bound to them (shapes, solids, anchors, references of solids, pose and clip offsets)."""
    ids = set(ids)
    asset["rig"]["bones"] = [b for b in asset["rig"]["bones"] if b["id"] not in ids]
    for rep in asset["representations"].values():
        if rep["status"] != "authored": continue
        for view in rep["geometry"]:
            rep["geometry"][view] = [s for s in rep["geometry"][view] if s["bone"] not in ids]
    for vol in asset["volumes"].values():
        if vol["status"] != "authored": continue
        gone = {s["id"] for s in vol["body"] if s["bone"] in ids}
        vol["body"] = [s for s in vol["body"] if s["id"] not in gone]
        for lst in ("thermal", "structure"):
            vol[lst] = [s for s in vol[lst] if not ("of" in s and s["of"] in gone) and not ("bone" in s and s["bone"] in ids) and s.get("member") not in ids]
            for s in vol[lst]:
                if "members" in s:
                    s["members"] = [m for m in s["members"] if m not in ids]
                    if not s["members"]: del s["members"]
    asset["anchors"] = [a for a in asset["anchors"] if a["bone"] not in ids]
    for p in asset["poses"]:
        p["bones"] = {k: v for k, v in p["bones"].items() if k not in ids}
    for c in asset["clips"]:
        for k in c.get("keys", []):
            if "bones" in k: k["bones"] = {b: v for b, v in k["bones"].items() if b not in ids}


# ---------------------------------------------------------------- new parts: sampled solids, projections, hulls

def hull(points):
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) < 3: return pts

    def cross(o, a, b): return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0: lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0: upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def _sphere_dirs(n=10):
    out = []
    for i in range(n + 1):
        th = math.pi * i / n
        for j in range(2 * n):
            ph = 2 * math.pi * j / (2 * n)
            out.append((math.cos(th), math.sin(th) * math.cos(ph), math.sin(th) * math.sin(ph)))
    return out


_DIRS = _sphere_dirs()


def samples(so):
    """Surface sample points of a solid in its bone-local frame (box, ellipsoid, capsule, loft; translation of `transform` only)."""
    k = so["kind"]; pts = []
    if k == "ellipsoid":
        r = so["radii"]; pts = [(d[0] * r[0], d[1] * r[1], d[2] * r[2]) for d in _DIRS]
    elif k == "capsule":
        ls = so.get("lateralScale", 1)
        for p, r in ((so["p0"], so["r0"]), (so["p1"], so["r1"])):
            pts += [(p[0] + d[0] * r, p[1] + d[1] * r, p[2] + d[2] * r * ls) for d in _DIRS]
    elif k == "box":
        sx, sy, sz = [x / 2 for x in so["size"]]
        pts = [(a * sx, b * sy, c * sz) for a in (-1, 1) for b in (-1, 1) for c in (-1, 1)]
    elif k == "loft":
        for s in so["sections"]:
            q = s.get("rotation", [0, 0, 0, 1]); n = s.get("exponent", 2)
            for j in range(48):
                ph = 2 * math.pi * j / 48
                c, sn = math.cos(ph), math.sin(ph)
                x = s["rx"] * math.copysign(abs(c) ** (2 / n), c); y = s["ry"] * math.copysign(abs(sn) ** (2 / n), sn)
                v = qrot(q, (x, y, 0))
                pts.append((s["at"][0] + v[0], s["at"][1] + v[1], s["at"][2] + v[2]))
    elif k == "extrude":
        plane = so["plane"]
        for a, b in so["profile"]:
            for c in (so["from"], so["to"]):
                pts.append((a, b, c) if plane == "xy" else (c, a, b) if plane == "yz" else (b, c, a))
    else: raise ValueError(k)
    tf = so.get("transform", {})
    sc = tf.get("scale", [1, 1, 1]); q = tf.get("rotation", [0, 0, 0, 1]); tr = tf.get("translation", [0, 0, 0])
    out = []
    for p in pts:
        p = (p[0] * sc[0], p[1] * sc[1], p[2] * sc[2])
        if q != [0, 0, 0, 1]: p = qrot(q, p)
        out.append((p[0] + tr[0], p[1] + tr[1], p[2] + tr[2]))
    return out


def qrot(q, v):
    x, y, z, w = q
    ix = w * v[0] + y * v[2] - z * v[1]; iy = w * v[1] + z * v[0] - x * v[2]; iz = w * v[2] + x * v[1] - y * v[0]; iw = -x * v[0] - y * v[1] - z * v[2]
    return (ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x)


def project(points_local, fr, view):
    """Bone-local points to the paper of a view: elevation keeps (u, v); front and plan use the given (rest) world frame."""
    if view == "elevation": return [(p[0], p[1]) for p in points_local]
    out = []
    for p in points_local:
        q = to_world(fr, p)
        out.append((q[2], q[1]) if view == "front" else (q[0], q[2]))
    return out


def poly_shape(sid, bone, points, **kw):
    sh = {"id": sid, "bone": bone, "role": kw.get("role", "body"), "z": kw.get("z", 5), "filled": kw.get("filled", True), "tone": kw.get("tone", 0.5)}
    for key in ("region", "regions", "material", "hatch", "penMm"):
        if key in kw: sh[key] = kw[key]
    sh.update({"kind": "poly", "points": [[rnd(x), rnd(y)] for x, y in points], "closed": kw.get("closed", True)})
    return sh


def hull_shape(sid, so, fr, view, **kw):
    """The filled outline of a solid (or of several solids on one bone) in one view: the convex hull of its projected samples."""
    sols = so if isinstance(so, list) else [so]
    pts = []
    for s in sols: pts += project(samples(s), fr, view)
    return poly_shape(sid, sols[0]["bone"], hull(pts), **kw)


BUN = f"{R}/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
HERE = os.path.dirname(os.path.abspath(__file__))


def probe(slug, pose=None):
    import subprocess
    args = [BUN, f"{HERE}/probe.ts", f"{HUMANS}/{slug}/🔣️.json"] + ([pose] if pose else [])
    out = subprocess.run(args, cwd=R, capture_output=True, text=True, check=True).stdout
    return json.loads(out)


def finalize(slug, doc):
    """Save, evaluate the default pose of every authored scale and write the measured extents (1:50 drawing) and the rig mass
    into `dimensions`, so the asset file states what its drawings show."""
    a = doc["asset"]
    a["dimensions"]["massKg"] = round(sum(b.get("mass", {}).get("kg", 0) for b in a["rig"]["bones"]), 3)
    save(slug, doc)
    p = probe(slug)
    rows = p["rows"]
    ref = rows.get("50") or next(iter(rows.values()))
    a["dimensions"]["heightM"] = round(ref["height"], 3)
    a["dimensions"]["widthM"] = round(ref["width"], 3)
    a["dimensions"]["depthM"] = round(ref["front"]["horizontal"], 3)
    return save(slug, doc), p


def nelder_mead(f, x0, step=5.0, iters=2000, tol=1e-10):
    """Plain Nelder-Mead minimiser (pose solving for grips and handles)."""
    n = len(x0)
    pts = [list(x0)] + [[x0[j] + (step if j == i else 0) for j in range(n)] for i in range(n)]
    vals = [f(p) for p in pts]
    for _ in range(iters):
        order_ = sorted(range(n + 1), key=lambda i: vals[i]); pts = [pts[i] for i in order_]; vals = [vals[i] for i in order_]
        if abs(vals[-1] - vals[0]) < tol: break
        c = [sum(p[j] for p in pts[:-1]) / n for j in range(n)]
        xr = [c[j] + (c[j] - pts[-1][j]) for j in range(n)]; fr = f(xr)
        if fr < vals[0]:
            xe = [c[j] + 2 * (c[j] - pts[-1][j]) for j in range(n)]; fe = f(xe)
            pts[-1], vals[-1] = (xe, fe) if fe < fr else (xr, fr)
        elif fr < vals[-2]: pts[-1], vals[-1] = xr, fr
        else:
            xc = [c[j] + 0.5 * (pts[-1][j] - c[j]) for j in range(n)]; fc = f(xc)
            if fc < vals[-1]: pts[-1], vals[-1] = xc, fc
            else:
                for i in range(1, n + 1):
                    pts[i] = [pts[0][j] + 0.5 * (pts[i][j] - pts[0][j]) for j in range(n)]; vals[i] = f(pts[i])
    i = min(range(n + 1), key=lambda i: vals[i])
    return pts[i], vals[i]


def mirror_offsets(bones_l):
    """Left offsets to right: abduction and twist change sign (the rig mirrors z)."""
    out = {}
    for b, off in bones_l.items():
        r = b[:-2] + "-r" if b.endswith("-l") else b
        out[r] = {k: (-v if k in ("abductionDeg", "twistDeg") else v) for k, v in off.items()}
    return out


def near(view, q):
    return -q[2] if view == "elevation" else q[0] if view == "front" else q[1]


def solid_points_world(so, fr):
    try: return [to_world(fr, p) for p in samples(so)]
    except ValueError:
        if so["kind"] == "revolve":
            return [to_world(fr, [a, b, 0]) for a, b in so["profile"]] + [to_world(fr, [a, 0, b]) for a, b in so["profile"]] + [to_world(fr, [a, 0, -b]) for a, b in so["profile"]] + [to_world(fr, [a, -b, 0]) for a, b in so["profile"]]
        if so["kind"] == "polyhedron": return [to_world(fr, v) for v in so["vertices"]]
        raise


def assign_z(asset, new_solids, key_bones=None):
    """Paint order (`z`) of new shapes at rest: each new shape goes right under the lowest authored shape of a nearer bone (bone depth =
    nearest point of its body solids at rest at the same scale) and new shapes among themselves follow their own nearest point."""
    rest = chain(asset["rig"]["bones"])
    for key in SCALES:
        rep = asset["representations"][key]; vol = asset["volumes"][key]
        mine = {s["id"]: s for s in new_solids[key]}
        bone_pts = {}
        for so in vol["body"]:
            if so["id"] in mine or so["bone"] not in rest: continue
            bone_pts.setdefault(so["bone"], []).extend(solid_points_world(so, rest[so["bone"]]))
        for view, shapes in rep["geometry"].items():
            old = [s for s in shapes if s["id"] not in mine and s["role"] != "ground"]
            def bnear(b):
                pts = bone_pts.get(b) or [rest[b]["o"]]
                return max(near(view, q) for q in pts)
            depth_old = [(s["z"], bnear(s["bone"])) for s in old]
            zs = sorted(s["z"] for s in old) or [0]
            new = [s for s in shapes if s["id"] in mine]
            dn = {id(s): max(near(view, q) for q in solid_points_world(mine[s["id"]], rest[s["bone"]])) for s in new}
            for rank, s in enumerate(sorted(new, key=lambda s: dn[id(s)])):
                d = dn[id(s)]
                above = [z for z, bd in depth_old if bd > d + 0.002]
                base = (min(above) - 0.5) if above else (max(zs) + 1)
                s["z"] = round(base + rank * 0.001, 4)
