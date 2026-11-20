"""🧰 Vacuum cleaner kit: the measures of the real canister cleaner (one place for 3D and 2D), the canister's side, front and plan outlines, and small builders for solids and 2D shapes.

Frames: body = world at rest (x ahead, y up, z lateral, left +). Chain bones (hose-1..10, grip, wand, nozzle): u along the bone, v up when the bone points ahead, w = z.
"""
import json
import math
from pathlib import Path

T = Path(__file__).resolve().parent
R = T.parents[6]
CATALOG = R / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/vacuum-cleaner/🔣️.json"
INDEX = R / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"
SOURCE = T / "volumes/appliances/vacuum-cleaner.json"
GEN = T / "🗑️generated/r3-vacuum-cleaner"
SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
STACK = [-0.5, -0.5, -0.5, 0.5]
AXIAL = [0.0, -0.70711, 0.0, 0.70711]
BUMPER = (0.170, 0.188)

HOSE_N = 10
HOSE_SEG = 0.15
HOSE_R = 0.018
SOCKET = (0.27, 0.125)
GRIP_L = 0.20
WAND_X = 0.18
WAND_L = 0.86
WHEEL = (-0.17, 0.06)
WHEEL_R = 0.06
WHEEL_W = 0.025
WHEEL_Z = 0.1375
CASTER = (0.18, 0.025)
CASTER_R = 0.025
NOZZLE_FLOOR = -0.047
NOZZLE_HALF = 0.1425
PARTING = 0.185
LID_TOP = 0.245
HANDLE_TOP = 0.282

SHELL_KEYS = [
    (0.035, -0.200, 0.195, 0.090),
    (0.042, -0.222, 0.218, 0.105),
    (0.052, -0.236, 0.234, 0.115),
    (0.065, -0.245, 0.244, 0.121),
    (0.080, -0.249, 0.249, 0.124),
    (0.095, -0.250, 0.250, 0.125),
    (0.165, -0.250, 0.250, 0.125),
    (0.185, -0.247, 0.246, 0.124),
]
LID_KEYS = [
    (0.185, -0.247, 0.246, 0.124),
    (0.200, -0.244, 0.240, 0.122),
    (0.215, -0.236, 0.226, 0.117),
    (0.228, -0.222, 0.200, 0.108),
    (0.238, -0.200, 0.160, 0.094),
    (0.243, -0.170, 0.115, 0.078),
    (0.245, -0.140, 0.080, 0.066),
]
PLAN_E = 3.0


def r5(v):
    """🔢 Round every number of a JSON value to 0.01 mm."""
    if isinstance(v, float):
        return round(v, 5) + 0.0
    if isinstance(v, list):
        return [r5(x) for x in v]
    if isinstance(v, dict):
        return {k: r5(x) for k, x in v.items()}
    return v


def interp(keys, y, col):
    """📈 Monotone cubic (Fritsch-Carlson) interpolation of column `col` of the key table at height y."""
    ys = [k[0] for k in keys]
    vs = [k[col] for k in keys]
    if y <= ys[0]:
        return vs[0]
    if y >= ys[-1]:
        return vs[-1]
    n = len(ys)
    d = [(vs[i + 1] - vs[i]) / (ys[i + 1] - ys[i]) for i in range(n - 1)]
    m = [d[0]] + [0.0 if d[i - 1] * d[i] <= 0 else (d[i - 1] + d[i]) / 2 for i in range(1, n - 1)] + [d[-1]]
    for i in range(n - 1):
        if abs(d[i]) < 1e-12:
            m[i] = m[i + 1] = 0.0
            continue
        a, b = m[i] / d[i], m[i + 1] / d[i]
        s = a * a + b * b
        if s > 9:
            t = 3 / math.sqrt(s)
            m[i], m[i + 1] = t * a * d[i], t * b * d[i]
    i = max(k for k in range(n - 1) if ys[k] <= y)
    h = ys[i + 1] - ys[i]
    t = (y - ys[i]) / h
    h00, h10, h01, h11 = 2 * t ** 3 - 3 * t ** 2 + 1, t ** 3 - 2 * t ** 2 + t, -2 * t ** 3 + 3 * t ** 2, t ** 3 - t ** 2
    return h00 * vs[i] + h10 * h * m[i] + h01 * vs[i + 1] + h11 * h * m[i + 1]


def plan_at(keys, y):
    """📐 Rear x, front x and half width of a canister part at height y."""
    return interp(keys, y, 1), interp(keys, y, 2), interp(keys, y, 3)


def stack_sections(keys, heights, grow=0.0, e=PLAN_E):
    """🥞 Horizontal superelliptic plan sections of a canister part, stacked at the given heights (section x = lateral, y = ahead)."""
    out = []
    for y in heights:
        xr, xf, w = plan_at(keys, y)
        out.append({"at": [(xr + xf) / 2, y, 0.0], "rx": w + grow, "ry": (xf - xr) / 2 + grow, "exponent": e, "rotation": STACK})
    return out


def superellipse(a, b, e, n=48, cx=0.0, cy=0.0):
    """⭕ Points of |x/a|^e + |y/b|^e = 1, counter-clockwise from +x."""
    pts = []
    for i in range(n):
        t = 2 * math.pi * i / n
        c, s = math.cos(t), math.sin(t)
        pts.append((cx + a * math.copysign(abs(c) ** (2 / e), c), cy + b * math.copysign(abs(s) ** (2 / e), s)))
    return pts


def rounded_rect(x0, y0, x1, y1, r, n=4):
    """▢ Counter-clockwise rounded rectangle polygon."""
    r = min(r, (x1 - x0) / 2, (y1 - y0) / 2)
    pts = []
    for cx, cy, a0 in ((x1 - r, y0 + r, -90), (x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180)):
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            pts.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    return pts


def circle(r, cx=0.0, cy=0.0, n=32):
    """◯ Counter-clockwise circle polygon."""
    return [(cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)) for i in range(n)]


def area2(pts):
    """➕ Signed doubled area of a polygon."""
    return sum(pts[i][0] * pts[i - 1][1] - pts[i - 1][0] * pts[i][1] for i in range(len(pts)))


def ccw(pts):
    """🔄 The polygon counter-clockwise."""
    return list(pts) if area2(pts) < 0 else list(reversed(pts))


def side_outline(keys, y0, y1, dy=0.004):
    """🖼️ Side elevation outline (x, y) of a stacked canister part between y0 and y1: front edge up, rear edge down."""
    n = max(2, int(round((y1 - y0) / dy)))
    ys = [y0 + (y1 - y0) * i / n for i in range(n + 1)]
    front = [(plan_at(keys, y)[1], y) for y in ys]
    rear = [(plan_at(keys, y)[0], y) for y in reversed(ys)]
    return front + rear


def front_outline(keys, y0, y1, dy=0.004):
    """🖼️ Front elevation outline (lateral, y) of a stacked canister part."""
    n = max(2, int(round((y1 - y0) / dy)))
    ys = [y0 + (y1 - y0) * i / n for i in range(n + 1)]
    right = [(plan_at(keys, y)[2], y) for y in ys]
    left = [(-plan_at(keys, y)[2], y) for y in reversed(ys)]
    return right + left


def plan_outline(keys, y0, y1, n=72):
    """🖼️ Plan outline (x ahead, lateral) of a stacked canister part: the envelope of its sections, sampled by angle."""
    ys = [y0 + (y1 - y0) * i / 40 for i in range(41)]
    out = []
    for i in range(n):
        t = 2 * math.pi * i / n
        c, s = math.cos(t), math.sin(t)
        best = None
        for y in ys:
            xr, xf, w = plan_at(keys, y)
            a, b, cx = (xf - xr) / 2, w, (xf + xr) / 2
            px = cx + a * math.copysign(abs(c) ** (2 / PLAN_E), c)
            pz = b * math.copysign(abs(s) ** (2 / PLAN_E), s)
            score = px * c + pz * s
            if best is None or score > best[0]:
                best = (score, px, pz)
        out.append((best[1], best[2]))
    return out


def offset(pts, d):
    """↗️ Polygon offset outward by d (miter at each vertex, counter-clockwise input)."""
    P = ccw(pts)
    n = len(P)
    out = []
    for i in range(n):
        a, b, c = P[i - 1], P[i], P[(i + 1) % n]
        e1 = (b[0] - a[0], b[1] - a[1])
        e2 = (c[0] - b[0], c[1] - b[1])
        l1, l2 = math.hypot(*e1), math.hypot(*e2)
        n1 = (e1[1] / l1, -e1[0] / l1)
        n2 = (e2[1] / l2, -e2[0] / l2)
        m = (n1[0] + n2[0], n1[1] + n2[1])
        lm = math.hypot(*m)
        m = (m[0] / lm, m[1] / lm)
        k = d / max(0.3, m[0] * n1[0] + m[1] * n1[1])
        out.append((b[0] + m[0] * k, b[1] + m[1] * k))
    return out


def simplify(pts, tol, closed=True):
    """✂️ Douglas-Peucker on a polyline or ring."""
    if len(pts) < 4:
        return list(pts)
    if closed:
        i0 = max(range(len(pts)), key=lambda i: (pts[i][0] - pts[0][0]) ** 2 + (pts[i][1] - pts[0][1]) ** 2)
        a = simplify(pts[: i0 + 1], tol, False)
        b = simplify(pts[i0:] + [pts[0]], tol, False)
        return a[:-1] + b[:-1]

    def dist(p, a, b):
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy)
        if L < 1e-12:
            return math.hypot(p[0] - a[0], p[1] - a[1])
        return abs(dx * (a[1] - p[1]) - dy * (a[0] - p[0])) / L

    k, dmax = 0, 0.0
    for i in range(1, len(pts) - 1):
        d = dist(pts[i], pts[0], pts[-1])
        if d > dmax:
            k, dmax = i, d
    if dmax <= tol:
        return [pts[0], pts[-1]]
    return simplify(pts[: k + 1], tol, False)[:-1] + simplify(pts[k:], tol, False)


def path_d(rings):
    """🖊️ SVG-like path data of closed rings (the 2D `path` shape)."""
    out = []
    for ring in rings:
        out.append("M " + " L ".join(f"{fmt(x)} {fmt(y)}" for x, y in ring) + " Z")
    return " ".join(out)


def fmt(v):
    """🔤 A number for path data."""
    s = f"{v:.5f}".rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


def solid(kind, sid, bone, material, role="body", tone=0.5, **kw):
    """🧊 A volume solid with its common members first."""
    return r5({"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **kw})


def revolve(sid, bone, material, profile, axis, origin=(0, 0, 0), role="body", tone=0.5, **kw):
    """🌀 Revolve of [axial, radial] about axis through origin."""
    return solid("revolve", sid, bone, material, role, tone, profile=[list(p) for p in profile], axis=list(axis), origin=list(origin), **kw)


def tube_profile(a0, a1, r):
    """🧪 Open revolve profile of a plain cylinder from a0 to a1 (closed through the axis)."""
    return [(a0, 0.0), (a0, r), (a1, r), (a1, 0.0)]


def ring_profile(a0, a1, r0, r1):
    """💍 Closed revolve loop of a ring (bore r0, outside r1)."""
    return [(a0, r0), (a0, r1), (a1, r1), (a1, r0), (a0, r0)]


def extrude(sid, bone, material, plane, profile, z0, z1, holes=None, role="body", tone=0.5, **kw):
    """🏗️ Extrude of a counter-clockwise profile with clockwise-agnostic holes."""
    d = {"profile": [list(p) for p in ccw(profile)], "plane": plane, "from": z0, "to": z1}
    if holes:
        d["holes"] = [[list(p) for p in ccw(h)] for h in holes]
    return solid("extrude", sid, bone, material, role, tone, **d, **kw)


def loft(sid, bone, material, sections, caps=(True, True), role="body", tone=0.5, **kw):
    """🧵 Loft of superelliptic sections."""
    return solid("loft", sid, bone, material, role, tone, sections=sections, capStart=caps[0], capEnd=caps[1], **kw)


def axial_loft(stations, e=2.0):
    """🧵 Sections of a loft along a bone's u axis: (u, half extent along w, half extent along v[, v offset])."""
    out = []
    for st in stations:
        u, rw, rv = st[:3]
        dv = st[3] if len(st) > 3 else 0.0
        out.append({"at": [u, dv, 0.0], "rx": rw, "ry": rv, "exponent": e, "rotation": AXIAL})
    return out


def lateral_loft(stations, e=2.0):
    """🧵 Sections of a loft along w: (w, half extent along u, half extent along v, u centre, v centre)."""
    return [{"at": [cu, cv, w], "rx": ru, "ry": rv, "exponent": e} for w, ru, rv, cu, cv in stations]


def capsule(sid, bone, material, p0, p1, r0, r1=None, role="body", tone=0.5, **kw):
    """💊 Capsule."""
    return solid("capsule", sid, bone, material, role, tone, p0=list(p0), p1=list(p1), r0=r0, r1=r0 if r1 is None else r1, **kw)


def ellipsoid(sid, bone, material, at, radii, role="body", tone=0.5, **kw):
    """🥚 Ellipsoid."""
    return solid("ellipsoid", sid, bone, material, role, tone, radii=list(radii), transform={"translation": list(at)}, **kw)


def box(sid, bone, material, at, size, corner=0.0, role="body", tone=0.5, **kw):
    """📦 Box."""
    extra = {"cornerRadius": corner} if corner > 0 else {}
    return solid("box", sid, bone, material, role, tone, size=list(size), transform={"translation": list(at)}, **extra, **kw)


def ref(sid, of, **kw):
    """🔗 Thermal or structural reference to a body solid of the same scale."""
    return {"id": sid, "of": of, **kw}


def thermal(s, region, layer=None, sid=None, **kw):
    """🌡️ A thermal solid from a solid definition."""
    d = dict(s)
    if sid:
        d["id"] = sid
    d["region"] = region
    if layer:
        d["layer"] = layer
    d.update(kw)
    return d


def member(s, bone, part, sid=None, **kw):
    """🏗️ A structural solid from a solid definition."""
    d = dict(s)
    if sid:
        d["id"] = sid
    d["member"] = bone
    d["part"] = part
    d.update(kw)
    return d


def text(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


def load(path):
    """📥 JSON file."""
    return json.loads(Path(path).read_text(encoding="utf-8"))
