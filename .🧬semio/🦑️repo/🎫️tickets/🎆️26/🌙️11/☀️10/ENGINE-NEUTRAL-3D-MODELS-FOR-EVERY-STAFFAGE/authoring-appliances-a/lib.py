"""Authoring helpers: explicit solid constructors in the bone-local u, v, w frame, one asset per script."""
import json, os, math

T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
AX = {"u": (1, 0, 0), "v": (0, 1, 0), "w": (0, 0, 1)}


def r5(x):
    if isinstance(x, float):
        y = round(x, 5)
        return 0.0 if y == 0 else y
    if isinstance(x, (list, tuple)):
        return [r5(i) for i in x]
    if isinstance(x, dict):
        return {k: r5(v) for k, v in x.items()}
    return x


class Ctx:
    def __init__(self, bone, material, region=None, role="body", tone=0.5):
        self.d = dict(bone=bone, material=material, region=region, role=role, tone=tone)

    def __call__(self, **kw):
        c = Ctx.__new__(Ctx)
        c.d = {**self.d, **kw}
        return c


def mk(sid, kind, ctx, geom, tr=None, rot=None, scale=None, repeat=None, maxd=None, weights=None, regions=None):
    c = ctx.d if isinstance(ctx, Ctx) else ctx
    s = {"id": sid, "kind": kind, "bone": c["bone"], "material": c["material"], "role": c["role"], "tone": c["tone"]}
    if regions:
        s["regions"] = regions
    elif c.get("region"):
        s["region"] = c["region"]
    if maxd is not None:
        s["maxDenominator"] = maxd
    s.update(geom)
    t = {}
    if scale is not None:
        t["scale"] = list(scale)
    if rot is not None:
        t["rotation"] = list(rot)
    if tr is not None and any(abs(x) > 1e-12 for x in tr):
        t["translation"] = list(tr)
    if t:
        s["transform"] = t
    if repeat is not None:
        s["repeat"] = repeat
    if weights is not None:
        s["weights"] = weights
    return r5(s)


def qz(deg):
    a = math.radians(deg) / 2
    return [0, 0, math.sin(a), math.cos(a)]


def qy(deg):
    a = math.radians(deg) / 2
    return [0, math.sin(a), 0, math.cos(a)]


def qx(deg):
    a = math.radians(deg) / 2
    return [math.sin(a), 0, 0, math.cos(a)]


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw*bx+ax*bw+ay*bz-az*by, aw*by-ax*bz+ay*bw+az*bx, aw*bz+ax*by-ay*bx+az*bw, aw*bw-ax*bx-ay*by-az*bz]


def box(sid, ctx, lo, hi, cr=None, **kw):
    size = [hi[i] - lo[i] for i in range(3)]
    c = [(hi[i] + lo[i]) / 2 for i in range(3)]
    g = {"size": size}
    if cr:
        assert cr <= min(size) / 2 + 1e-12, (sid, cr, size)
        g["cornerRadius"] = cr
    tr = kw.pop("tr", None)
    return mk(sid, "box", ctx, g, tr=[c[i] + (tr[i] if tr else 0) for i in range(3)], **kw)


def ell(sid, ctx, c, radii, **kw):
    return mk(sid, "ellipsoid", ctx, {"radii": list(radii)}, tr=c, **kw)


def cap(sid, ctx, p0, p1, r0, r1=None, ls=None, **kw):
    g = {"p0": list(p0), "p1": list(p1), "r0": r0, "r1": r0 if r1 is None else r1}
    if ls is not None:
        g["lateralScale"] = ls
    return mk(sid, "capsule", ctx, g, **kw)


def rev(sid, ctx, profile, axis="v", origin=(0, 0, 0), ls=None, sweep=None, **kw):
    g = {"profile": [list(p) for p in profile], "axis": list(AX[axis]) if isinstance(axis, str) else list(axis), "origin": list(origin)}
    if sweep is not None:
        g["sweepRad"] = sweep
    if ls is not None:
        g["lateralScale"] = ls
    return mk(sid, "revolve", ctx, g, **kw)


def cyl(sid, ctx, axis, c, length, radius, radius1=None, ls=None, **kw):
    """Cylinder/frustum about axis 'u'|'v'|'w' through c, centred at c along the axis, optional second radius."""
    h = length / 2
    o = list(c)
    i = "uvw".index(axis)
    o[i] -= h
    prof = [[0, radius], [length, radius if radius1 is None else radius1]]
    return rev(sid, ctx, prof, axis=axis, origin=o, ls=ls, **kw)


def basis_q(x, y, z):
    m = [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return [round(c, 6) for c in q]


LOFT_Q = {
    "u": basis_q((0, 0, 1), (0, -1, 0), (1, 0, 0)),
    "v": basis_q((0, 0, 1), (1, 0, 0), (0, 1, 0)),
    "w": [0, 0, 0, 1],
}


def sect(axis, t, ra, rb, e=None, ca=0, cb=0):
    """Loft section along bone axis 'u'|'v'|'w' at t. ra, rb are the half extents on the first and second lateral axis:
    u: ra on w, rb on v (offsets ca along w, cb along v); v: ra on w, rb on u (ca along w, cb along u); w: ra on u, rb on v (ca along u, cb along v).
    The section frame is given explicitly so that the model and the checker agree."""
    at = {"u": [t, cb, ca], "v": [cb, t, ca], "w": [ca, cb, t]}[axis]
    s = {"at": at, "rx": ra, "ry": rb}
    if e is not None:
        s["exponent"] = e
    if axis != "w":
        s["rotation"] = LOFT_Q[axis]
    return s


def loft(sid, ctx, sections, cs=True, ce=True, **kw):
    return mk(sid, "loft", ctx, {"sections": sections, "capStart": cs, "capEnd": ce}, **kw)


def lf(sid, ctx, axis, rows, cs=True, ce=True, **kw):
    """rows: (t, ra, rb[, e[, ca[, cb]]])"""
    return loft(sid, ctx, [sect(axis, *r) for r in rows], cs, ce, **kw)


def ext(sid, ctx, profile, plane, a, b, holes=None, taper=None, **kw):
    g = {"profile": [list(p) for p in profile], "plane": plane, "from": a, "to": b}
    if holes:
        g["holes"] = [[list(p) for p in h] for h in holes]
    if taper:
        g["taperTo"] = list(taper)
    return mk(sid, "extrude", ctx, g, **kw)


def frustum_y(sid, ctx, y0, y1, hu0, hu1, hw0=None, hw1=None, e=12, c=(0, 0), **kw):
    """Rectangular frustum along v: half extents along u (hu) and w (hw) at y0 -> y1; c = (w, u) centre offsets."""
    hw0 = hu0 if hw0 is None else hw0
    hw1 = hu1 if hw1 is None else hw1
    return lf(sid, ctx, "v", [(y0, hw0, hu0, e, c[0], c[1]), (y1, hw1, hu1, e, c[0], c[1])], **kw)


def text(en, de):
    return {"en": en, "de": de}


def write(slug, solids, decisions, category="appliances"):
    d = os.path.join(T, "volumes", category)
    os.makedirs(d, exist_ok=True)
    ids = [s["id"] for s in solids]
    assert len(ids) == len(set(ids)), [i for i in ids if ids.count(i) > 1]
    path = os.path.join(d, slug + ".json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump({"volume": {"status": "authored", "solids": solids, "decisions": decisions}}, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(slug, len(solids), "solids ->", path)


PLANE = {"u": "yz", "v": "zx", "w": "xy"}


def ngon(r, n=48, cx=0.0, cy=0.0, ry=None):
    ry = r if ry is None else ry
    return [[cx + r * math.cos(2 * math.pi * k / n), cy + ry * math.sin(2 * math.pi * k / n)] for k in range(n)]


def pipe(sid, ctx, axis, c, length, ro, ri, n=48, **kw):
    """Hollow cylinder (tube) about axis through c, centred at c along the axis: extrude of a ring with a hole.
    Profile coordinates: u -> (v, w); v -> (w, u); w -> (u, v)."""
    i = "uvw".index(axis)
    h = length / 2
    cc = {"u": (c[1], c[2]), "v": (c[2], c[0]), "w": (c[0], c[1])}[axis]
    return ext(sid, ctx, ngon(ro, n, *cc), PLANE[axis], c[i] - h, c[i] + h, holes=[ngon(ri, n, *cc)], **kw)


def rrect(x0, y0, x1, y1, r, n=6):
    """Counter-clockwise rounded rectangle polygon."""
    pts = []
    for (cx, cy, a0) in ((x1 - r, y0 + r, -90), (x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180)):
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            pts.append([cx + r * math.cos(a), cy + r * math.sin(a)])
    return pts


def hexagon(cx, cy, r, a0=30):
    return [[cx + r * math.cos(math.radians(a0 + 60 * k)), cy + r * math.sin(math.radians(a0 + 60 * k))] for k in range(6)]


def asset_shapes(slug, view="elevation", scale="50"):
    import glob
    f = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*appliances/%s/*.json" % slug)[0]
    a = json.load(open(f))["asset"]
    return {s["id"]: s for s in a["representations"][scale]["geometry"][view]}


def path_points(d):
    import re
    out = []
    for sub in [x for x in d.split("M") if x.strip()]:
        nums = [float(t) for t in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", sub)]
        out.append([[nums[i], nums[i + 1]] for i in range(0, len(nums) - 1, 2)])
    return out
