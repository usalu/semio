import json, math, os, subprocess, sys

T = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(T, "🗑️generated", "author-08")
S = os.path.join(T, "../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
CAT = os.path.normpath(os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"))
SCALES = ["50", "100", "200", "500"]
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}


def bun():
    base = os.path.expanduser("~/Library/Caches/semio/runtimes/bun")
    for d in sorted(os.listdir(base)):
        p = os.path.join(base, d, "bun")
        if os.path.exists(p):
            return p
    raise SystemExit("no bun")


def asset_file(cat, slug):
    for d in os.listdir(CAT):
        if d.endswith(cat):
            return os.path.join(CAT, d, slug, "🔣️.json")
    raise SystemExit("no category " + cat)


def load_asset(cat, slug):
    return json.load(open(asset_file(cat, slug)))["asset"]


def src_path(cat, slug):
    return os.path.join(T, "volumes", cat, slug + ".json")


def load_src(cat, slug):
    return json.load(open(src_path(cat, slug)))


def save_src(cat, slug, d):
    json.dump(d, open(src_path(cat, slug), "w"), ensure_ascii=False, separators=(",", ":"))


def profiles(slug, scale):
    return json.load(open(os.path.join(GEN, f"{slug}.p{scale}.json")))


def r5(x):
    return round(float(x), 5)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def sub(a, b):
    return [x - y for x, y in zip(a, b)]


def add(a, b):
    return [x + y for x, y in zip(a, b)]


def mul(a, k):
    return [x * k for x in a]


def norm(a):
    l = math.sqrt(dot(a, a)) or 1.0
    return [x / l for x in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def quat_basis(x, y, z):
    m = [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]]
    tr = m[0][0] + m[1][1] + m[2][2]
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, 0.25 * s]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [0.25 * s, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, 0.25 * s, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, 0.25 * s, (m[1][0] - m[0][1]) / s]
    n = math.sqrt(sum(c * c for c in q))
    return [r5(c / n) for c in q]


def rotations(ats):
    out = []
    n = len(ats)
    for i in range(n):
        a = ats[max(0, i - 1)]
        b = ats[min(n - 1, i + 1)]
        t = norm(sub(b, a))
        zc = [0, 0, 1]
        x = sub(zc, mul(t, dot(zc, t)))
        if math.sqrt(dot(x, x)) < 1e-3:
            x = [1, 0, 0]
            x = sub(x, mul(t, dot(x, t)))
        x = norm(x)
        y = cross(t, x)
        out.append(quat_basis(x, y, t))
    return out


def loft(sid, bone, mat, tone, role, stations, exp=2.2, cap=(True, True), **extra):
    ats = [[r5(s[0]), r5(s[1]), r5(s[2])] for s in stations]
    rots = rotations(ats)
    secs = []
    for s, a, q in zip(stations, ats, rots):
        sec = {"at": a, "rx": r5(max(s[3], 0.002)), "ry": r5(max(s[4], 0.002)), "exponent": s[5] if len(s) > 5 else exp, "rotation": q}
        secs.append(sec)
    d = {"id": sid, "bone": bone, "material": mat, "role": role, "tone": tone, "kind": "loft", "sections": secs, "capStart": cap[0], "capEnd": cap[1]}
    d.update(extra)
    return d


def capsule(sid, bone, mat, tone, role, p0, p1, r0, r1=None, **extra):
    d = {"id": sid, "bone": bone, "material": mat, "role": role, "tone": tone, "kind": "capsule", "p0": [r5(c) for c in p0], "p1": [r5(c) for c in p1], "r0": r5(r0), "r1": r5(r0 if r1 is None else r1)}
    d.update(extra)
    return d


def ellipsoid(sid, bone, mat, tone, role, centre, radii, rotation=None, **extra):
    d = {"id": sid, "bone": bone, "material": mat, "role": role, "tone": tone, "kind": "ellipsoid", "radii": [r5(c) for c in radii], "transform": {"translation": [r5(c) for c in centre]}}
    if rotation is not None:
        d["transform"]["rotation"] = rotation
    d.update(extra)
    return d


def box(sid, bone, mat, tone, role, centre, size, rotation=None, corner=None, **extra):
    d = {"id": sid, "bone": bone, "material": mat, "role": role, "tone": tone, "kind": "box", "size": [r5(c) for c in size], "transform": {"translation": [r5(c) for c in centre]}}
    if rotation is not None:
        d["transform"]["rotation"] = rotation
    if corner is not None:
        d["cornerRadius"] = r5(corner)
    d.update(extra)
    return d


def shrink(s, t):
    """Inward offset of a solid by t metres (nested thermal layer shells)."""
    c = json.loads(json.dumps(s))
    k = c["kind"]
    if k == "box":
        c["size"] = [r5(max(v - 2 * t, 0.002)) for v in c["size"]]
        if "cornerRadius" in c:
            c["cornerRadius"] = r5(max(c["cornerRadius"] - t, 0))
    elif k == "ellipsoid":
        c["radii"] = [r5(max(v - t, 0.002)) for v in c["radii"]]
    elif k == "capsule":
        c["r0"] = r5(max(c["r0"] - t, 0.002))
        c["r1"] = r5(max(c["r1"] - t, 0.002))
    elif k == "loft":
        for sec in c["sections"]:
            sec["rx"] = r5(max(sec["rx"] - t, 0.002))
            sec["ry"] = r5(max(sec["ry"] - t, 0.002))
    elif k == "revolve":
        c["profile"] = [[a, r5(max(r - t, 0.0))] if r > t else [a, 0.0] for a, r in c["profile"]]
    elif k == "extrude":
        c["profile"] = inset(c["profile"], t)
        c["holes"] = [inset(h, -t) for h in c.get("holes", [])] if c.get("holes") else c.get("holes")
        if c.get("holes") is None:
            c.pop("holes", None)
        c["from"] = r5(c["from"] + t)
        c["to"] = r5(c["to"] - t)
        if c["to"] <= c["from"]:
            mid = (c["from"] + c["to"]) / 2
            c["from"], c["to"] = r5(mid - 0.001), r5(mid + 0.001)
    return c


def scale_lat(s, f):
    c = json.loads(json.dumps(s))
    k = c["kind"]
    if k == "box":
        c["size"] = [r5(c["size"][0]), r5(c["size"][1] * f), r5(c["size"][2] * f)] if False else [r5(v * f) for v in c["size"]]
    elif k == "ellipsoid":
        c["radii"] = [r5(v * f) for v in c["radii"]]
    elif k == "capsule":
        c["r0"] = r5(c["r0"] * f)
        c["r1"] = r5(c["r1"] * f)
    elif k == "loft":
        for sec in c["sections"]:
            sec["rx"] = r5(sec["rx"] * f)
            sec["ry"] = r5(sec["ry"] * f)
    elif k == "revolve":
        c["profile"] = [[a, r5(r * f)] for a, r in c["profile"]]
    return c


def inset(poly, t):
    n = len(poly)
    if n < 3:
        return poly
    ar = 0
    for i in range(n):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % n]
        ar += x0 * y1 - x1 * y0
    sgn = 1 if ar > 0 else -1
    out = []
    for i in range(n):
        p0, p1, p2 = poly[i - 1], poly[i], poly[(i + 1) % n]
        e1 = norm([p1[0] - p0[0], p1[1] - p0[1], 0])
        e2 = norm([p2[0] - p1[0], p2[1] - p1[1], 0])
        n1 = [-e1[1] * sgn, e1[0] * sgn]
        n2 = [-e2[1] * sgn, e2[0] * sgn]
        bis = [n1[0] + n2[0], n1[1] + n2[1]]
        l = bis[0] ** 2 + bis[1] ** 2
        if l < 1e-9:
            out.append([p1[0], p1[1]])
            continue
        k = 2 * t / l
        out.append([r5(p1[0] + bis[0] * k), r5(p1[1] + bis[1] * k)])
    return out


def measure(cat, slug):
    r = subprocess.run([bun(), "author-08-measure.ts", f"{cat}/{slug}"], cwd=T, capture_output=True, text=True)
    return json.loads(r.stdout)


def check(cat, slug, scale=None):
    cmd = [bun(), "volumes-check.ts", f"{cat}/{slug}", "--json"]
    if scale:
        cmd += ["--scale", str(scale)]
    r = subprocess.run(cmd, cwd=T, capture_output=True, text=True)
    return json.loads(r.stdout)


class Prof:
    def __init__(self, slug, scale):
        self.d = profiles(slug, scale)
        self.f = self.d["bones"]

    def shapes(self, view, bone, roles=("body",), include=None, exclude=(), open_ok=True):
        def pick(filled):
            out = []
            for s in self.d[view]:
                if s["bone"] != bone or s["filled"] != filled or s["role"] not in roles or s["id"] in exclude:
                    continue
                if include is not None and s["id"] not in include:
                    continue
                out.append(s)
            return out
        out = pick(True)
        if not out and open_ok:
            out = pick(False)
        return out

    def local(self, view, bone, p):
        f = self.f[bone]
        o, u, v, w = f["o"], f["u"], f["v"], f["w"]
        if view == "elevation":
            P = [p[0], p[1], o[2]]
        elif view == "front":
            P = [o[0], p[1], p[0]]
        else:
            P = [p[0], o[1], p[1]]
        d = sub(P, o)
        return dot(d, u), dot(d, v), dot(d, w)

    def has_shapes(self, bone):
        return any(x["bone"] == bone for v in ("elevation", "front", "plan") for x in self.d[v])

    def rings_local(self, view, bone, **kw):
        out = []
        for s in self.shapes(view, bone, **kw):
            for ring in s["rings"]:
                out.append(([self.local(view, bone, p) for p in ring], bool(s["filled"] or s["closed"])))
        return out


def span_at(rings, a0, idx):
    lo, hi = math.inf, -math.inf
    for ring, closed in rings:
        n = len(ring)
        for i in range(n if closed else n - 1):
            p, q = ring[i], ring[(i + 1) % n]
            if (p[0] - a0) * (q[0] - a0) <= 0 and p[0] != q[0]:
                t = (a0 - p[0]) / (q[0] - p[0])
                c = p[idx] + t * (q[idx] - p[idx])
                lo, hi = min(lo, c), max(hi, c)
            elif p[0] == a0 and q[0] == a0:
                lo, hi = min(lo, p[idx], q[idx]), max(hi, p[idx], q[idx])
    return (lo, hi) if lo <= hi else None


def urange(rings):
    xs = [p[0] for r, _ in rings for p in r]
    return (min(xs), max(xs)) if xs else None


def rdp(rows, tol, chans):
    n = len(rows)
    if n <= 2:
        return list(range(n))
    keep = {0, n - 1}
    stack = [(0, n - 1)]
    while stack:
        i, j = stack.pop()
        worst, wk = 0.0, None
        for k in range(i + 1, j):
            t = (rows[k][0] - rows[i][0]) / ((rows[j][0] - rows[i][0]) or 1)
            e = max(abs(rows[k][c] - (rows[i][c] + t * (rows[j][c] - rows[i][c]))) for c in chans)
            if e > worst:
                worst, wk = e, k
        if wk is not None and worst > tol:
            keep.add(wk)
            stack += [(i, wk), (wk, j)]
    return sorted(keep)


def bone_profile(P, bone, roles=("body",), include=None, exclude=(), nst=64, wview=None, trim=0.0):
    kw = dict(roles=roles, include=include, exclude=exclude)
    el = P.rings_local("elevation", bone, **kw)
    if not el:
        kw2 = dict(kw, roles=("body", "detail"))
        el = P.rings_local("elevation", bone, **kw2)
    vfrom = 1
    if not el or urange(el) is None:
        for vw in ("front", "plan"):
            el = P.rings_local(vw, bone, **kw) or P.rings_local(vw, bone, **dict(kw, roles=("body", "detail")))
            if el:
                vfrom = 2
                break
    if not el:
        return None
    a0, a1 = urange(el)
    f = P.f[bone]
    if wview is None:
        wview = "front" if abs(f["u"][1]) >= 0.5 else "plan"
    wr = P.rings_local(wview, bone, **kw)
    if not wr:
        other = "plan" if wview == "front" else "front"
        wr = P.rings_local(other, bone, **kw)
    el_open = all(not c for _, c in el)
    rows = []
    eps = 1e-5
    lo, hi = a0 + trim + eps, a1 - trim - eps
    for i in range(nst):
        a = lo + (hi - lo) * i / (nst - 1)
        sv = span_at(el, a, vfrom)
        if sv is None:
            continue
        sw = span_at(wr, a, 2) if wr else None
        if sw is None:
            h = (sv[1] - sv[0]) / 2
            sw = (-h, h) if not wr else None
        if sw is None:
            continue
        rx = max((sw[1] - sw[0]) / 2, 0.001)
        ry = max((sv[1] - sv[0]) / 2, 0.001)
        if el_open or vfrom == 2:
            ry, sv = rx, (-rx, rx)
        rows.append([a, (sv[0] + sv[1]) / 2, (sw[0] + sw[1]) / 2, rx, ry])
    return rows or None


def simplify(rows, tol, maxn=None):
    idx = rdp(rows, tol, [1, 2, 3, 4])
    if maxn and len(idx) > maxn:
        lo = tol
        while len(idx) > maxn and lo < 1:
            lo *= 1.3
            idx = rdp(rows, lo, [1, 2, 3, 4])
    return [rows[i] for i in idx]


def clamp_feature(s, ft):
    if s.get("slender"):
        return s
    fix_profile(s, ft)
    if s["kind"] == "loft" and len(s["sections"]) >= 2:
        a0, a1 = s["sections"][0]["at"][0], s["sections"][-1]["at"][0]
        need = ft + 0.003
        if a1 - a0 < need:
            d = (need - (a1 - a0)) / 2
            s["sections"][0]["at"][0] = r5(a0 - d)
            s["sections"][-1]["at"][0] = r5(a1 + d)
    m = ft / 2 + 0.0015
    k = s["kind"]
    if k == "ellipsoid":
        s["radii"] = [r5(max(v, m)) for v in s["radii"]]
    elif k == "capsule":
        s["r0"] = r5(max(s["r0"], m))
        s["r1"] = r5(max(s["r1"], m))
    elif k == "box":
        s["size"] = [r5(max(v, ft + 0.0012)) for v in s["size"]]
    elif k == "loft":
        for sec in s["sections"]:
            sec["rx"] = r5(max(sec["rx"], m))
            sec["ry"] = r5(max(sec["ry"], m))
    return s


def hull(pts):
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in pts))
    if len(pts) < 3:
        return pts
    def cr(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lo, up = [], []
    for p in pts:
        while len(lo) >= 2 and cr(lo[-2], lo[-1], p) <= 0:
            lo.pop()
        lo.append(p)
    for p in reversed(pts):
        while len(up) >= 2 and cr(up[-2], up[-1], p) <= 0:
            up.pop()
        up.append(p)
    return [list(p) for p in lo[:-1] + up[:-1]]


def self_intersects(poly):
    n = len(poly)
    def seg(a, b, c, d):
        def o(p, q, r):
            return (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0])
        return o(a, b, c) * o(a, b, d) < 0 and o(c, d, a) * o(c, d, b) < 0
    for i in range(n):
        for j in range(i + 2, n):
            if i == 0 and j == n - 1:
                continue
            if seg(poly[i], poly[(i + 1) % n], poly[j], poly[(j + 1) % n]):
                return True
    return False


def fix_profile(s, ft):
    if s["kind"] == "extrude":
        pr = s["profile"]
        if self_intersects(pr):
            s["profile"] = [[r5(a), r5(b)] for a, b in hull(pr)]
            s.pop("holes", None)
        xs = [p[0] for p in s["profile"]]; ys = [p[1] for p in s["profile"]]
        need = ft + 0.0024
        for k, (lo, hi) in enumerate([(min(xs), max(xs)), (min(ys), max(ys))]):
            if hi - lo < need:
                c = (lo + hi) / 2
                f = need / max(hi - lo, 1e-6)
                for p in s["profile"]:
                    p[k] = r5(c + (p[k] - c) * f)
        if s["to"] - s["from"] < need:
            c = (s["to"] + s["from"]) / 2
            s["from"], s["to"] = r5(c - need / 2), r5(c + need / 2)
    return s
