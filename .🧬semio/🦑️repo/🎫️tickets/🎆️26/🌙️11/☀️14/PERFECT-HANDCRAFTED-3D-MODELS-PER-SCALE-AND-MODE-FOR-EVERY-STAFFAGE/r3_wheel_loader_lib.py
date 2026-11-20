"""🧰 Solid and drawing helpers of the round-3 wheel loader: every solid is stated by hand in its bone frame; a solid that is a prism, a body of revolution or a loft also gives its own true orthographic outline for the three 2D views."""
import math

FIXED = {"rear-frame": (0.0, 0.0), "front-frame": (0.0, 0.0)}
WHEEL = {}


def r4(x):
    return round(float(x), 4) + 0.0


def P(pts):
    return [[r4(a), r4(b)] for a, b in pts]


def area(pts):
    return sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts))) / 2


def ccw(pts):
    pts = [tuple(p) for p in pts]
    return pts if area(pts) > 0 else pts[::-1]


def arc(cx, cy, r, a0, a1, n):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def strip(pts, t):
    """➰ A plate of thickness t along a polyline: the polyline and its offset to the left, mitred."""
    out = []
    for i, p in enumerate(pts):
        a, b = pts[max(i - 1, 0)], pts[min(i + 1, len(pts) - 1)]
        n = []
        for q0, q1 in ((a, p), (p, b)):
            d = math.hypot(q1[0] - q0[0], q1[1] - q0[1])
            if d > 1e-9:
                n.append((-(q1[1] - q0[1]) / d, (q1[0] - q0[0]) / d))
        mx, my = sum(v[0] for v in n) / len(n), sum(v[1] for v in n) / len(n)
        k = t / max(mx * n[0][0] + my * n[0][1], 0.3)
        out.append((p[0] + mx * k, p[1] + my * k))
    return list(pts) + out[::-1]


def sup(cx, cy, rx, ry, e, n=64):
    out = []
    for i in range(n):
        a = 2 * math.pi * i / n
        c, s = math.cos(a), math.sin(a)
        out.append((cx + rx * math.copysign(abs(c) ** (2 / e), c), cy + ry * math.copysign(abs(s) ** (2 / e), s)))
    return out


class Ctx:
    def __init__(self, bone, material, role="body", tone=0.45, region=None, member=None):
        self.d = dict(bone=bone, material=material, role=role, tone=tone, region=region, member=member)

    def __call__(self, **kw):
        c = Ctx(**self.d)
        c.d.update(kw)
        return c


def base(sid, kind, c, **kw):
    s = dict(id=sid, kind=kind, bone=c.d["bone"], material=c.d["material"], role=c.d["role"], tone=c.d["tone"])
    s.update(kw)
    s["_r"], s["_m"] = c.d["region"], c.d["member"]
    return s


def mirror(s, a, b):
    s["repeat"] = {"count": 2, "translation": [0, 0, r4(-(a + b))]}
    return s


def ext(sid, c, profile, a, b, plane="xy", holes=None, mir=False):
    s = base(sid, "extrude", c, profile=P(ccw(profile)), plane=plane, **{"from": r4(a), "to": r4(b)})
    if holes:
        s["holes"] = [P(ccw(h)) for h in holes]
    return mirror(s, a, b) if mir else s


def box(sid, c, lo, hi, radius=None):
    s = base(sid, "box", c, size=[r4(hi[i] - lo[i]) for i in range(3)], transform={"translation": [r4((hi[i] + lo[i]) / 2) for i in range(3)]})
    if radius:
        s["cornerRadius"] = radius
    return s


def rev(sid, c, profile, axis, origin, repeat=None):
    s = base(sid, "revolve", c, profile=P(profile), axis={"u": [1, 0, 0], "v": [0, 1, 0], "w": [0, 0, 1]}[axis], origin=[r4(x) for x in origin])
    if repeat:
        s["repeat"] = repeat
    return s


def cyl(sid, c, axis, origin, length, r):
    return rev(sid, c, [(0, 0), (0, r), (length, r), (length, 0)], axis, origin)


def loft(sid, c, sections, e=2.0, upright=False):
    rot = {"rotation": [0, 0.7071067811865476, 0, 0.7071067811865476]} if upright else {}
    return base(sid, "loft", c, sections=[dict(at=[r4(x) for x in at], rx=r4(rx), ry=r4(ry), exponent=e, **rot) for at, rx, ry in sections], capStart=True, capEnd=True)


def cap(sid, c, p0, p1, r, slender=True):
    s = base(sid, "capsule", c, p0=[r4(x) for x in p0], p1=[r4(x) for x in p1], r0=r, r1=r)
    if slender:
        s["slender"] = True
    return s


def ell(sid, c, at, radii):
    return base(sid, "ellipsoid", c, radii=list(radii), transform={"translation": [r4(x) for x in at]})


class Sheet:
    """📄 The three 2D shape lists of one scale."""

    def __init__(self):
        self.v = {"elevation": [], "front": [], "plan": []}

    def add(self, view, sid, bone, role, z, tone, geom, filled=True, region=None, material=None, solid=None, pen=None):
        ids = {s["id"] for s in self.v[view]}
        assert sid not in ids, (view, sid)
        s = dict(geom)
        s.update(id=sid, bone=bone, role=role, z=z, filled=filled, tone=tone)
        if region:
            s["region"] = region
        if material:
            s["material"] = material
        if solid:
            s["solid"] = solid
        if pen:
            s["penMm"] = pen
        self.v[view].append(s)

    def out(self, twins=True):
        """🪞 The lists in layer order; with `twins` every filled closed detail gets an unfilled outline right above it, because the line views merge the inner outlines of filled shapes of one region."""
        res = {}
        for k, v in self.v.items():
            rows = list(v)
            if twins:
                for s in v:
                    if s["role"] == "detail" and s["filled"] and s.get("closed", True):
                        t = {a: b for a, b in s.items() if a not in ("region", "material")}
                        t.update(id=s["id"] + "-edge", filled=False, z=round(s["z"] + 0.004, 3))
                        rows.append(t)
            res[k] = sorted(rows, key=lambda s: s["z"])
        return res


def poly(pts, closed=True):
    return dict(kind="poly", points=P(pts), closed=closed)


def rect(x0, y0, x1, y1):
    return dict(kind="rect", x=r4(min(x0, x1)), y=r4(min(y0, y1)), w=r4(abs(x1 - x0)), h=r4(abs(y1 - y0)))


def circle(cx, cy, r):
    return dict(kind="ellipse", cx=r4(cx), cy=r4(cy), rx=r4(r), ry=r4(r))


def draw(sh, s, z, views="efp", sid=None, role=None, tone=None, filled=True, pen=None):
    """✏️ The true outline of one solid in the wanted views (e, f, p); z is the layer in (elevation, front, plan)."""
    sid, bone = sid or s["id"], s["bone"]
    role, tone = role or s["role"], s["tone"] if tone is None else tone
    kw = dict(filled=filled, region=s.get("_r"), material=s["material"], pen=pen)
    fixed = bone in FIXED or bone in WHEEL
    ox, oy = WHEEL.get(bone, (0.0, 0.0))
    copies = [0.0] + ([s["repeat"]["translation"][2]] if "repeat" in s and "translation" in s["repeat"] else [])
    names = [sid] if len(copies) == 1 else [sid + "-l", sid + "-r"]
    k = s["kind"]
    if k == "extrude" and s["plane"] == "xy":
        pr, a, b = s["profile"], s["from"], s["to"]
        us, vs = [p[0] for p in pr], [p[1] for p in pr]
        if "e" in views:
            sh.add("elevation", sid, bone, role, z[0], tone, poly(pr), **kw)
        for n, dz in zip(names, copies):
            if fixed:
                if "f" in views:
                    sh.add("front", n, bone, role, z[1], tone, rect(a + dz, min(vs) + oy, b + dz, max(vs) + oy), **kw)
                if "p" in views:
                    sh.add("plan", n, bone, role, z[2], tone, rect(min(us) + ox, a + dz, max(us) + ox, b + dz), **kw)
            else:
                hint = dict(kind="extruded", sweep="w", z0=r4(a + dz), z1=r4(b + dz))
                if "f" in views:
                    sh.add("front", n, bone, role, z[1], tone, poly(pr), solid=hint, **kw)
                if "p" in views:
                    sh.add("plan", n, bone, role, z[2], tone, poly(pr), solid=hint, **kw)
    elif k == "extrude" and s["plane"] == "zx":
        pr, a, b = s["profile"], s["from"], s["to"]
        ws, us = [p[0] for p in pr], [p[1] for p in pr]
        if "e" in views:
            sh.add("elevation", sid, bone, role, z[0], tone, rect(min(us), a, max(us), b), **kw)
        if "f" in views:
            sh.add("front", sid, bone, role, z[1], tone, rect(min(ws), a, max(ws), b), **kw)
        if "p" in views:
            sh.add("plan", sid, bone, role, z[2], tone, poly([(p[1], p[0]) for p in pr]), **kw)
    elif k == "box":
        t, h = s["transform"]["translation"], [x / 2 for x in s["size"]]
        if "e" in views:
            sh.add("elevation", sid, bone, role, z[0], tone, rect(t[0] - h[0], t[1] - h[1], t[0] + h[0], t[1] + h[1]), **kw)
        for n, dz in zip(names, copies):
            if "f" in views:
                sh.add("front", n, bone, role, z[1], tone, rect(t[2] - h[2] + dz, t[1] - h[1] + oy, t[2] + h[2] + dz, t[1] + h[1] + oy), **kw)
            if "p" in views:
                sh.add("plan", n, bone, role, z[2], tone, rect(t[0] - h[0] + ox, t[2] - h[2] + dz, t[0] + h[0] + ox, t[2] + h[2] + dz), **kw)
    elif k == "revolve":
        pr, o = s["profile"], s["origin"]
        ax = "uvw"[s["axis"].index(1)]
        rm = max(p[1] for p in pr)
        a0, a1 = min(p[0] for p in pr), max(p[0] for p in pr)
        outer = [p for p in pr if True]
        if pr[0] == pr[-1]:
            top = max(range(len(pr)), key=lambda i: pr[i][1])
            ring = sorted({(p[0], max(q[1] for q in pr if abs(q[0] - p[0]) < 1e-9)) for p in pr})
            outer = ring
        side = [(p[0], p[1]) for p in outer] + [(p[0], -p[1]) for p in outer[::-1] if p[1] > 1e-9]
        if ax == "w":
            if "e" in views:
                sh.add("elevation", sid, bone, role, z[0], tone, circle(o[0], o[1], rm), **kw)
            if fixed:
                if "f" in views:
                    sh.add("front", sid, bone, role, z[1], tone, poly([(o[2] + p[0], o[1] + oy + p[1]) for p in side]), **kw)
                if "p" in views:
                    sh.add("plan", sid, bone, role, z[2], tone, poly([(o[0] + ox + p[1], o[2] + p[0]) for p in side]), **kw)
            else:
                hint = dict(kind="extruded", sweep="w", z0=r4(o[2] + a0), z1=r4(o[2] + a1))
                for v, zz in (("front", z[1]), ("plan", z[2])):
                    if v[0] in views:
                        sh.add(v, sid, bone, role, zz, tone, circle(o[0], o[1], rm), solid=hint, **kw)
        elif ax == "v":
            if "e" in views:
                sh.add("elevation", sid, bone, role, z[0], tone, poly([(o[0] + p[1], o[1] + p[0]) for p in side]), **kw)
            if "f" in views:
                sh.add("front", sid, bone, role, z[1], tone, poly([(o[2] + p[1], o[1] + p[0]) for p in side]), **kw)
            if "p" in views:
                sh.add("plan", sid, bone, role, z[2], tone, circle(o[0], o[2], rm), **kw)
        else:
            pts = [(o[0] + p[0], o[1] + p[1]) for p in side]
            if "e" in views:
                sh.add("elevation", sid, bone, role, z[0], tone, poly(pts), **kw)
            if fixed:
                if "f" in views:
                    sh.add("front", sid, bone, role, z[1], tone, circle(o[2], o[1], rm), **kw)
                if "p" in views:
                    sh.add("plan", sid, bone, role, z[2], tone, poly([(o[0] + p[0], o[2] + p[1]) for p in side]), **kw)
            else:
                hint = dict(kind="extruded", sweep="w", z0=r4(o[2] - rm), z1=r4(o[2] + rm))
                for v, zz in (("front", z[1]), ("plan", z[2])):
                    if v[0] in views:
                        sh.add(v, sid, bone, role, zz, tone, poly(pts), solid=hint, **kw)
    elif k == "loft":
        sec, e = s["sections"], s["sections"][0].get("exponent", 2)
        along_u = abs(sec[-1]["at"][0] - sec[0]["at"][0]) > abs(sec[-1]["at"][1] - sec[0]["at"][1])
        big = max(sec, key=lambda q: q["rx"] * q["ry"])
        if along_u:
            if "e" in views:
                sh.add("elevation", sid, bone, role, z[0], tone, poly([(q["at"][0], q["at"][1] + q["ry"]) for q in sec] + [(q["at"][0], q["at"][1] - q["ry"]) for q in sec[::-1]]), **kw)
            if "f" in views:
                sh.add("front", sid, bone, role, z[1], tone, poly(sup(big["at"][2], big["at"][1], big["rx"], big["ry"], e)), **kw)
            if "p" in views:
                sh.add("plan", sid, bone, role, z[2], tone, poly([(q["at"][0], q["at"][2] + q["rx"]) for q in sec] + [(q["at"][0], q["at"][2] - q["rx"]) for q in sec[::-1]]), **kw)
        else:
            if "e" in views:
                sh.add("elevation", sid, bone, role, z[0], tone, poly([(q["at"][0] + q["rx"], q["at"][1]) for q in sec] + [(q["at"][0] - q["rx"], q["at"][1]) for q in sec[::-1]]), **kw)
            if "f" in views:
                sh.add("front", sid, bone, role, z[1], tone, poly([(q["at"][2] + q["ry"], q["at"][1]) for q in sec] + [(q["at"][2] - q["ry"], q["at"][1]) for q in sec[::-1]]), **kw)
            if "p" in views:
                sh.add("plan", sid, bone, role, z[2], tone, poly(sup(big["at"][0], big["at"][2], big["rx"], big["ry"], e)), **kw)
    elif k == "capsule":
        p0, p1, r = s["p0"], s["p1"], s["r0"]

        def bar(view, a, b, zz):
            d = math.hypot(b[0] - a[0], b[1] - a[1])
            if d < 1e-6:
                sh.add(view, sid, bone, role, zz, tone, circle(a[0], a[1], r), **kw)
            else:
                n = (-(b[1] - a[1]) / d * r, (b[0] - a[0]) / d * r)
                sh.add(view, sid, bone, role, zz, tone, poly([(a[0] - n[0], a[1] - n[1]), (b[0] - n[0], b[1] - n[1]), (b[0] + n[0], b[1] + n[1]), (a[0] + n[0], a[1] + n[1])]), **kw)

        if "e" in views:
            bar("elevation", (p0[0], p0[1]), (p1[0], p1[1]), z[0])
        if "f" in views:
            bar("front", (p0[2], p0[1]), (p1[2], p1[1]), z[1])
        if "p" in views:
            bar("plan", (p0[0], p0[2]), (p1[0], p1[2]), z[2])
    else:
        raise SystemExit("draw: " + k)
    return s
