"""🌞 Author of the used park sun lounger (`used-sun-lounger-bench`): every hand-chosen number of the reference table, the geometry of the lying-surface curve, slats, flat-bar stringers, leg portals, base plates, anchor nuts and strip footings, and the writer of the asset document `🔣️.json` next to this file (rig, anchors, four individually drawn representations, four individually modelled volumes, poses, clips, thermal and structural model).

Run: `python3 author-lounger.py` (writes `🔣️.json`, prints `[DEBUG]` measurements).
@see ../../reports/used-sun-lounger-bench.md — reference table, decisions and proof
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — the contract
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))

LENGTH = 2.000
SLAT_W, SLAT_T, SLAT_L, SLAT_N, SLAT_R = 0.070, 0.040, 0.700, 23, 0.004
BAR_D, BAR_T, BAR_Z = 0.080, 0.012, 0.300
POST, POST_WALL = 0.060, 0.003
PLATE, PLATE_T = 0.150, 0.012
FOOT_RUST = 0.060
FOOTING_X, FOOTING_Z, FOOTING_D = 0.300, 0.800, 0.500
X_HEAD_LEG, X_FOOT_LEG, X_JOINT = 0.500, 1.550, 0.880
NUT_R, NUT_H, NUT_DX = 0.0095, 0.010, 0.050
LIFT = 0.450
RHO_TIMBER, RHO_STEEL, RHO_CONCRETE = 740.0, 7850.0, 2400.0
G = 9.80665
SUNBATHER_KG = 80.0

RAW = [(0.000, 0.725), (0.080, 0.712), (0.640, 0.395), (0.780, 0.346), (0.950, 0.331), (1.250, 0.352), (1.550, 0.357), (1.850, 0.333), (2.000, 0.306)]


def r5(v):
    """🔢 Rounds a number for the document."""
    return round(v + 0.0, 5)


def pchip(points):
    """📈 Fritsch-Carlson monotone cubic through the control points: returns y(x), y'(x)."""
    xs = [p[0] for p in points]
    ys = [p[1] for p in points]
    n = len(xs)
    h = [xs[i + 1] - xs[i] for i in range(n - 1)]
    d = [(ys[i + 1] - ys[i]) / h[i] for i in range(n - 1)]
    m = [0.0] * n
    m[0], m[-1] = d[0], d[-1]
    for i in range(1, n - 1):
        if d[i - 1] * d[i] <= 0:
            m[i] = 0.0
        else:
            w1, w2 = 2 * h[i] + h[i - 1], h[i] + 2 * h[i - 1]
            m[i] = (w1 + w2) / (w1 / d[i - 1] + w2 / d[i])

    def at(x):
        x = min(max(x, xs[0]), xs[-1])
        i = next((k for k in range(n - 1) if x <= xs[k + 1]), n - 2)
        t = (x - xs[i]) / h[i]
        y = (2 * t**3 - 3 * t**2 + 1) * ys[i] + (t**3 - 2 * t**2 + t) * h[i] * m[i] + (-2 * t**3 + 3 * t**2) * ys[i + 1] + (t**3 - t**2) * h[i] * m[i + 1]
        dy = ((6 * t**2 - 6 * t) * ys[i] + (3 * t**2 - 4 * t + 1) * h[i] * m[i] + (-6 * t**2 + 6 * t) * ys[i + 1] + (3 * t**2 - 2 * t) * h[i] * m[i + 1]) / h[i]
        return y, dy

    return at


class Curve:
    """〰 The stringer top: the underside of the slats, a monotone cubic through the control heights."""

    def __init__(self, control):
        self.control = control
        self.f = pchip(control)
        self.x0, self.x1 = control[0][0], control[-1][0]
        self.xs = [self.x0 + (self.x1 - self.x0) * i / 2000 for i in range(2001)]
        self.arc = [0.0]
        for a, b in zip(self.xs, self.xs[1:]):
            self.arc.append(self.arc[-1] + math.hypot(b - a, self.top(b) - self.top(a)))

    def top(self, x):
        """🛏 Height of the stringer top at x."""
        return self.f(x)[0]

    def slope(self, x):
        """📐 Tangent angle at x in radians."""
        return math.atan(self.f(x)[1])

    def offset(self, x, d):
        """↕ The point d along the upward normal from the top at x."""
        a = self.slope(x)
        return (x - d * math.sin(a), self.top(x) + d * math.cos(a))

    def x_at(self, s):
        """🧵 x at arc length s."""
        for i in range(1, len(self.arc)):
            if self.arc[i] >= s:
                f = (s - self.arc[i - 1]) / (self.arc[i] - self.arc[i - 1])
                return self.xs[i - 1] + f * (self.xs[i] - self.xs[i - 1])
        return self.xs[-1]

    def curvature(self, x):
        """🌀 Curvature (positive where the curve sags), from central differences inside the span."""
        e = 2e-3
        x = min(max(x, self.x0 + e), self.x1 - e)
        d1 = (self.top(x + e) - self.top(x - e)) / (2 * e)
        d2 = (self.top(x + e) - 2 * self.top(x) + self.top(x - e)) / (e * e)
        return d2 / (1 + d1 * d1) ** 1.5

    def line(self, x0, x1, d, step=0.02):
        """🧶 Points along the curve offset by d between x0 and x1."""
        count = max(2, int(round(abs(x1 - x0) / step)))
        return [self.offset(x0 + (x1 - x0) * k / count, d) for k in range(count + 1)]


def slat_list(c):
    """🪵 The slats along the curve: centre, angle and corners; a slat over a sag is lifted so it bears on both edges."""
    gap = (c.arc[-1] - SLAT_N * SLAT_W) / (SLAT_N - 1)
    out = []
    for i in range(SLAT_N):
        x = c.x_at(i * (SLAT_W + gap) + SLAT_W / 2)
        a = c.slope(x)
        lift = max(0.0, c.curvature(x) * (SLAT_W / 2) ** 2 / 2)
        n = (-math.sin(a), math.cos(a))
        t = (math.cos(a), math.sin(a))
        cx, cy = x + n[0] * SLAT_T / 2, c.top(x) + lift + n[1] * SLAT_T / 2
        corners = [(cx + t[0] * du + n[0] * dv, cy + t[1] * du + n[1] * dv) for du, dv in [(-SLAT_W / 2, -SLAT_T / 2), (SLAT_W / 2, -SLAT_T / 2), (SLAT_W / 2, SLAT_T / 2), (-SLAT_W / 2, SLAT_T / 2)]]
        out.append({"i": i, "x": cx, "y": cy, "a": a, "corners": corners})
    return out, gap


def calibrated():
    """📏 The control curve stretched so the whole lounger (slats and bars) spans exactly 2.000 m from x = 0."""
    control = RAW
    for _ in range(6):
        c = Curve(control)
        s, _ = slat_list(c)
        pts = [p for sl in s for p in sl["corners"]] + c.line(c.x0, c.x1, -BAR_D)
        lo, hi = min(p[0] for p in pts), max(p[0] for p in pts)
        k = LENGTH / (hi - lo)
        control = [((x - lo) * k, y) for x, y in control]
    return Curve(control)


C = calibrated()
SLATS, SLAT_GAP = slat_list(C)


def min_top(xc, half):
    """⤵ Lowest stringer top over [xc - half, xc + half]."""
    return min(C.top(xc - half + 2 * half * k / 20) for k in range(21))


def bar_bottom(xc, half=POST / 2, depth=BAR_D):
    """⤓ Lowest underside of the bar over the post width."""
    return min(C.offset(xc - half + 2 * half * k / 20, -depth)[1] for k in range(21))


def post_top(xc, size=POST):
    """📏 A post top: 10 mm under the lowest slat bearing over its width."""
    return min_top(xc, size / 2) - 0.010


def area_poly(pts):
    """📐 Shoelace area."""
    return abs(sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(pts, pts[1:] + pts[:1]))) / 2


def perimeter(pts):
    """📏 Perimeter of a closed polygon."""
    return sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(pts, pts[1:] + pts[:1]))


def ccw(pts):
    """🔄 The polygon counter-clockwise."""
    s = sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(pts, pts[1:] + pts[:1]))
    return pts if s > 0 else list(reversed(pts))


def P(pts):
    """✒ Points rounded for the document."""
    return [[r5(x), r5(y)] for x, y in pts]


def T(en, de):
    """🗣 A bilingual text."""
    return {"en": en, "de": de}


def quat_w(a):
    """🌀 Unit quaternion turning u towards v by a about w."""
    return [0, 0, r5(math.sin(a / 2)), r5(math.cos(a / 2))]


def rounded_rect(cx, cy, a, w, h, r, seg=3):
    """⬜ A w x h rectangle about (cx, cy) turned by a with eased corners r."""
    t = (math.cos(a), math.sin(a))
    n = (-math.sin(a), math.cos(a))
    pts = []
    for sx, sy, start in [(1, -1, -90), (1, 1, 0), (-1, 1, 90), (-1, -1, 180)]:
        ox, oy = sx * (w / 2 - r), sy * (h / 2 - r)
        for k in range(seg + 1):
            ang = math.radians(start + 90 * k / seg)
            du, dv = ox + r * math.cos(ang), oy + r * math.sin(ang)
            pts.append((cx + t[0] * du + n[0] * dv, cy + t[1] * du + n[1] * dv))
    return pts


def to_local(origin, angle, p):
    """🧭 World (x, y) to the (u, v) frame of a bone at origin turned by angle."""
    dx, dy = p[0] - origin[0], p[1] - origin[1]
    return (dx * math.cos(angle) + dy * math.sin(angle), -dx * math.sin(angle) + dy * math.cos(angle))


Z_POST = BAR_Z - BAR_T / 2 - POST / 2
LEGS = {"head": X_HEAD_LEG, "foot": X_FOOT_LEG}
SIDES = {"r": -1, "l": 1}

BACK_O = C.offset(C.x0, -BAR_D / 2)
BACK_TIP = C.offset(X_JOINT, -BAR_D / 2)
LEG_TIP = C.offset(C.x1, -BAR_D / 2)
BACK_A = math.atan2(BACK_TIP[1] - BACK_O[1], BACK_TIP[0] - BACK_O[0])
BACK_LEN = math.hypot(BACK_TIP[0] - BACK_O[0], BACK_TIP[1] - BACK_O[1])
LEGB_A = math.atan2(LEG_TIP[1] - BACK_TIP[1], LEG_TIP[0] - BACK_TIP[0])
LEGB_LEN = math.hypot(LEG_TIP[0] - BACK_TIP[0], LEG_TIP[1] - BACK_TIP[1])
BONE_FRAMES = {
    "ground": ((0.0, 0.0), 0.0),
    "frame": ((0.0, 0.0), 0.0),
    "stringer-back": (BACK_O, BACK_A),
    "stringer-leg": (BACK_TIP, LEGB_A),
    "leg-head": ((X_HEAD_LEG, 0.0), math.pi / 2),
    "leg-foot": ((X_FOOT_LEG, 0.0), math.pi / 2),
    "footing-head": ((X_HEAD_LEG, -FOOTING_D), math.pi / 2),
    "footing-foot": ((X_FOOT_LEG, -FOOTING_D), math.pi / 2),
}


def local(bone, p):
    """📍 World point (x, y) in the frame of a bone."""
    o, a = BONE_FRAMES[bone]
    return to_local(o, a, p)


def box(id, bone, material, tone, size, centre, rot=None, role="body", **extra):
    """🧱 A box solid of size [u, v, w] centred at (u, v, w) in the bone frame."""
    s = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": [r5(v) for v in size]}
    tr = {"translation": [r5(v) for v in centre]}
    if rot is not None:
        tr["rotation"] = quat_w(rot)
    s["transform"] = tr
    s.update(extra)
    return s


def wbox(id, material, tone, x0, x1, y0, y1, z0, z1, role="body", **extra):
    """🧱 A world-aligned box on the frame bone."""
    return box(id, "frame", material, tone, [x1 - x0, y1 - y0, z1 - z0], [(x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2], role=role, **extra)


def footing_box(leg, tone):
    """🧱 The strip footing of a leg portal on its footing bone (u up, v towards -x)."""
    return box(f"footing-{leg}", f"footing-{leg}", "concrete-c25", tone, [FOOTING_D, FOOTING_X, FOOTING_Z], [FOOTING_D / 2, 0, 0])


def extrude_xy(id, material, tone, pts, z0, z1, role="body", bone="frame", **extra):
    """🪚 A profile in the (u, v) plane swept along w from z0 to z1."""
    s = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": P(ccw(pts)), "plane": "xy", "from": r5(z0), "to": r5(z1)}
    s.update(extra)
    return s


def extrude_yz(id, material, tone, pts, x0, x1, role="body", bone="frame", **extra):
    """🪚 A profile in the (v, w) plane swept along u from x0 to x1."""
    s = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": P(ccw(pts)), "plane": "yz", "from": r5(x0), "to": r5(x1)}
    s.update(extra)
    return s


def nut(id, x, z, y0):
    """🔩 A hexagon nut M12 as its circumscribed cylinder on a base plate."""
    return {"id": id, "kind": "revolve", "bone": "frame", "material": "steel-rusted", "role": "detail", "tone": 0.22, "profile": [[0, r5(NUT_R)], [r5(NUT_H), r5(NUT_R)]], "axis": [0, 1, 0], "origin": [r5(x), r5(y0), r5(z)]}


def shape(id, bone, role, z, kind, filled=True, tone=0.5, region=None, material=None, hatch=None, pen=None, regions=None, **geom):
    """✏ A 2D shape of one view."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    s.update(geom)
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    return s


def rect(id, bone, role, z, x0, y0, x1, y1, **kw):
    """⬛ A rect shape from two corners."""
    return shape(id, bone, role, z, "rect", x=r5(min(x0, x1)), y=r5(min(y0, y1)), w=r5(abs(x1 - x0)), h=r5(abs(y1 - y0)), **kw)


def poly(id, bone, role, z, pts, closed=True, **kw):
    """🔷 A poly shape."""
    return shape(id, bone, role, z, "poly", points=P(pts), closed=closed, **kw)


def footing_local_rect(leg):
    """🧱 The footing outline in its bone frame for the side elevation."""
    return [(0.0, -FOOTING_X / 2), (FOOTING_D, -FOOTING_X / 2), (FOOTING_D, FOOTING_X / 2), (0.0, FOOTING_X / 2)]


def plan_spans(eased=False):
    """🔭 The part of every slat seen from above: where a higher slat overlaps a lower one in plan (the steep back), the lower one starts at the edge of the higher one. `eased` measures the slats with their eased arrises, as the body of the fine scales has them: two slats whose sharp corners would just overlap in plan then leave the few millimetres of open joint the body shows (silhouette repair SF of 2026-11-23, `catalog-silhouette` 80.6 mm in the plan of 1:50 between slat 3 and slat 4)."""
    outlines = [rounded_rect(sl["x"], sl["y"], sl["a"], SLAT_W, SLAT_T, SLAT_R) if eased else sl["corners"] for sl in SLATS]
    spans = [[min(p[0] for p in o), max(p[0] for p in o)] for o in outlines]
    tops = [max(p[1] for p in o) for o in outlines]
    for i in range(len(spans) - 1):
        if spans[i + 1][0] < spans[i][1]:
            if tops[i] >= tops[i + 1]:
                spans[i + 1][0] = spans[i][1]
            else:
                spans[i][1] = spans[i + 1][0]
    return spans


PLAN_SPANS = plan_spans()
PLAN_SPANS_EASED = plan_spans(True)


def slat_tone(i):
    """🎨 Greyed robinia; the slats under shoulders and hips are worn lighter and smoother."""
    return 0.72 if i in WORN else 0.6


WORN = {4, 5, 6, 9, 10, 11, 12}
CHECKED = {2, 8, 14, 19}

DEBUG = []


class Scale:
    """📐 What one drawing scale keeps: smallest feature, slats as single boards or one band, bar and plate thickness, post feet, nuts, pens."""

    def __init__(self, den):
        self.den = den
        self.feature = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[den]
        self.single_slats = den in (50, 100)
        self.eased = den == 50
        self.bar_t = {50: BAR_T, 100: 0.015, 200: 0.030, 500: 0.075}[den]
        self.plate_t = {50: PLATE_T, 100: 0.015, 200: 0.030, 500: 0.0}[den]
        self.rust_feet = den == 50
        self.nuts = den == 50
        self.post = 0.075 if den == 500 else POST
        self.slab_depth = SLAT_T + BAR_D if den == 500 else SLAT_T
        self.pens = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[den]
        self.range = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}[den]

    def z_post(self):
        """↔ Lateral centre of a post: its outer face against the inner face of the bar."""
        return BAR_Z - self.bar_t / 2 - self.post / 2 if self.den != 500 else BAR_Z - self.post / 2 + 0.0


def build(den):
    """🏗 The representation and the volume of one scale, drawn and modelled from the same parts."""
    sc = Scale(den)
    body, thermal, structure = [], [], []
    elev, front, plan = [], [], []
    zb = BAR_Z - sc.bar_t
    zp = sc.z_post()
    ps = sc.post

    if sc.single_slats:
        for sl in SLATS:
            i = sl["i"]
            sid = f"slat-{i + 1:02d}"
            tone = slat_tone(i)
            extra = {"cornerRadius": SLAT_R} if sc.eased else {}
            body.append(box(sid, "frame", "robinia-weathered", tone, [SLAT_W, SLAT_T, SLAT_L], [sl["x"], sl["y"], 0], rot=sl["a"], **extra))
            thermal.append({"id": f"th-{sid}", "of": sid, "region": "slats"})
            outline = rounded_rect(sl["x"], sl["y"], sl["a"], SLAT_W, SLAT_T, SLAT_R) if sc.eased else sl["corners"]
            elev.append(poly(sid, "frame", "body", 6, outline, tone=tone, region="slats", material="robinia-weathered", pen=0.5 if den == 50 else 0.35))
            xa, xb = (PLAN_SPANS_EASED if sc.eased else PLAN_SPANS)[i]
            plan.append(rect(sid, "frame", "body", 6, xa, -SLAT_L / 2, xb, SLAT_L / 2, tone=tone, region="slats", material="robinia-weathered", hatch="grain" if den == 50 else None, pen=0.5 if den == 50 else 0.35))
        ys = sorted([(min(p[1] for p in sl["corners"]), max(p[1] for p in sl["corners"])) for sl in SLATS])
        band = [list(ys[0])]
        for lo, hi in ys[1:]:
            if lo <= band[-1][1] + 1e-6:
                band[-1][1] = max(band[-1][1], hi)
            else:
                band.append([lo, hi])
        for k, (lo, hi) in enumerate(band):
            front.append(rect(f"slat-band-{k + 1}", "frame", "body", 6, -SLAT_L / 2, lo, SLAT_L / 2, hi, tone=0.62, region="slats", material="robinia-weathered"))
        DEBUG.append(f"[DEBUG] 1:{den} front slat band intervals {[[round(a, 4), round(b, 4)] for a, b in band]}")
        for sl in SLATS[:12]:
            hi = max(p[1] for p in sl["corners"])
            front.append(poly(f"slat-edge-{sl['i'] + 1:02d}", "frame", "detail", 7, [(-SLAT_L / 2, hi), (SLAT_L / 2, hi)], closed=False, filled=False, tone=0.3))
    else:
        depth = sc.slab_depth
        x0, x1 = C.x0, C.x1
        top_line = C.line(x0, x1, SLAT_T, 0.05)
        bottom_line = C.line(x0, x1, SLAT_T - depth, 0.05)
        prof = top_line + list(reversed(bottom_line))
        sid = "slab" if den == 500 else "slat-band"
        tone = 0.6
        body.append(extrude_xy(sid, "robinia-weathered", tone, prof, -SLAT_L / 2, SLAT_L / 2))
        thermal.append({"id": f"th-{sid}", "of": sid, "region": "slats"})
        regs = {"regions": ["slats", "frame"]} if den == 500 else {"region": "slats"}
        elev.append(poly(sid, "frame", "body", 6, prof, tone=tone, material="robinia-weathered", **regs))
        xs = [p[0] for p in prof]
        ysv = [p[1] for p in prof]
        front.append(rect(sid, "frame", "body", 6, -SLAT_L / 2, min(ysv), SLAT_L / 2, max(ysv), tone=0.62, material="robinia-weathered", **regs))
        plan.append(rect(sid, "frame", "body", 6, min(xs), -SLAT_L / 2, max(xs), SLAT_L / 2, tone=tone, material="robinia-weathered", **regs))
        if den == 200:
            for sl in SLATS[1:]:
                xa = min(p[0] for p in sl["corners"])
                plan.append(poly(f"slat-joint-{sl['i'] + 1:02d}", "frame", "detail", 7, [(xa - SLAT_GAP / 2, -SLAT_L / 2), (xa - SLAT_GAP / 2, SLAT_L / 2)], closed=False, filled=False, tone=0.35))
            for sl in SLATS[:10]:
                hi = max(p[1] for p in sl["corners"])
                front.append(poly(f"slat-edge-{sl['i'] + 1:02d}", "frame", "detail", 7, [(-SLAT_L / 2, hi), (SLAT_L / 2, hi)], closed=False, filled=False, tone=0.3))

    if den != 500:
        for part, (xa, xb) in {"back": (C.x0, X_JOINT), "leg": (X_JOINT, C.x1)}.items():
            step = 0.02 if den < 200 else 0.05
            prof = C.line(xa, xb, 0.0, step) + list(reversed(C.line(xa, xb, -BAR_D, step)))
            for side, sgn in SIDES.items():
                sid = f"bar-{part}-{side}"
                z0, z1 = (zb, BAR_Z) if sgn > 0 else (-BAR_Z, -zb)
                body.append(extrude_xy(sid, "steel-coated-weathered", 0.34, prof, z0, z1))
                thermal.append({"id": f"th-{sid}", "of": sid, "region": "frame"})
                structure.append({"id": f"m-{sid}", "of": sid, "member": f"stringer-{part}", "part": "member"})
                if side == "r":
                    elev.append(poly(sid, "frame", "body", 4, prof, tone=0.34, region="frame", material="steel-coated-weathered"))
                yv = [p[1] for p in prof]
                xv = [p[0] for p in prof]
                front.append(rect(sid, "frame", "body", 4, z0, min(yv), z1, max(yv), tone=0.34, region="frame", material="steel-coated-weathered"))
                plan.append(rect(sid, "frame", "body", 3, min(xv), z0, max(xv), z1, tone=0.34, region="frame", material="steel-coated-weathered"))
    else:
        for part, (xa, xb) in {"back": (C.x0, X_JOINT), "leg": (X_JOINT, C.x1)}.items():
            prof = C.line(xa, xb, 0.0, 0.1) + list(reversed(C.line(xa, xb, -BAR_D, 0.1)))
            for side, sgn in SIDES.items():
                sid = f"m-bar-{part}-{side}"
                z0, z1 = (BAR_Z - sc.bar_t, BAR_Z) if sgn > 0 else (-BAR_Z, -BAR_Z + sc.bar_t)
                structure.append(dict(extrude_xy(sid, "steel-coated-weathered", 0.34, prof, z0, z1), member=f"stringer-{part}", part="member"))

    for leg, xl in LEGS.items():
        y_base = sc.plate_t
        if den == 500:
            pt = min(C.offset(xl - ps / 2 + ps * k / 20, SLAT_T - sc.slab_depth)[1] for k in range(21))
            y_foot = 0.075
            for side, sgn in SIDES.items():
                fid = f"foot-{leg}-{side}"
                body.append(wbox(fid, "steel-rusted", 0.26, xl - ps / 2, xl + ps / 2, 0.0, y_foot, sgn * zp - ps / 2, sgn * zp + ps / 2))
                thermal.append({"id": f"th-{fid}", "of": fid, "region": "feet"})
                structure.append({"id": f"m-{fid}", "of": fid, "member": f"leg-{leg}", "part": "member"})
            zi = zp - ps / 2
            ycb = CROSS[leg] - ps / 2
            portal = [(y_foot, -zp - ps / 2), (pt, -zp - ps / 2), (pt, -zi), (ycb + ps, -zi), (ycb + ps, zi), (pt, zi), (pt, zp + ps / 2), (y_foot, zp + ps / 2), (y_foot, zi), (ycb, zi), (ycb, -zi), (y_foot, -zi)]
            pid = f"portal-{leg}"
            body.append(extrude_yz(pid, "steel-coated-weathered", 0.4, portal, xl - ps / 2, xl + ps / 2))
            thermal.append({"id": f"th-{pid}", "of": pid, "region": "frame"})
            structure.append({"id": f"m-{pid}", "of": pid, "member": f"leg-{leg}", "part": "member"})
            elev.append(rect(pid, "frame", "body", 3, xl - ps / 2, y_foot, xl + ps / 2, pt, tone=0.4, region="frame", material="steel-coated-weathered"))
            elev.append(rect(f"foot-{leg}-r", "frame", "body", 3.5, xl - ps / 2, 0.0, xl + ps / 2, y_foot, tone=0.26, region="feet", material="steel-rusted"))
            front.append(poly(pid, "frame", "body", 2 if leg == "head" else 3, [(z, y) for y, z in portal], tone=0.4, region="frame", material="steel-coated-weathered"))
            for side, sgn in SIDES.items():
                front.append(rect(f"foot-{leg}-{side}", "frame", "body", 2.5 if leg == "head" else 3.5, sgn * zp - ps / 2, 0.0, sgn * zp + ps / 2, y_foot, tone=0.26, region="feet", material="steel-rusted"))
                plan.append(rect(f"foot-{leg}-{side}", "frame", "body", 2, xl - ps / 2, sgn * zp - ps / 2, xl + ps / 2, sgn * zp + ps / 2, tone=0.26, region="feet", material="steel-rusted"))
            plan.append(rect(pid, "frame", "body", 2.2, xl - ps / 2, -zp - ps / 2, xl + ps / 2, zp + ps / 2, tone=0.4, region="frame", material="steel-coated-weathered"))
            continue
        pt = post_top(xl)
        for side, sgn in SIDES.items():
            zc = sgn * zp
            plate_id = f"plate-{leg}-{side}"
            body.append(wbox(plate_id, "steel-rusted", 0.26, xl - PLATE / 2, xl + PLATE / 2, 0.0, sc.plate_t, zc - PLATE / 2, zc + PLATE / 2))
            thermal.append({"id": f"th-{plate_id}", "of": plate_id, "region": "feet"})
            structure.append({"id": f"m-{plate_id}", "of": plate_id, "member": f"leg-{leg}", "part": "member"})
            y_up = y_base
            if sc.rust_feet:
                fid = f"post-foot-{leg}-{side}"
                body.append(wbox(fid, "steel-rusted", 0.28, xl - ps / 2, xl + ps / 2, y_base, y_base + RUST[leg], zc - ps / 2, zc + ps / 2))
                thermal.append({"id": f"th-{fid}", "of": fid, "region": "feet"})
                structure.append({"id": f"m-{fid}", "of": fid, "member": f"leg-{leg}", "part": "member"})
                y_up = y_base + RUST[leg]
            post_id = f"post-{leg}-{side}"
            body.append(wbox(post_id, "steel-coated-weathered", 0.4, xl - ps / 2, xl + ps / 2, y_up, pt, zc - ps / 2, zc + ps / 2))
            thermal.append({"id": f"th-{post_id}", "of": post_id, "region": "frame"})
            structure.append({"id": f"m-{post_id}", "of": post_id, "member": f"leg-{leg}", "part": "member"})
            if sc.nuts:
                for k, dx in enumerate((-NUT_DX, NUT_DX)):
                    body.append(nut(f"nut-{leg}-{side}-{k + 1}", xl + dx, zc, sc.plate_t))
            zfront = (2 if leg == "head" else 3)
            if side == "r":
                elev.append(rect(plate_id, "frame", "body", 2, xl - PLATE / 2, 0.0, xl + PLATE / 2, sc.plate_t, tone=0.26, region="feet", material="steel-rusted"))
                if sc.rust_feet:
                    elev.append(rect(f"post-foot-{leg}-r", "frame", "body", 3, xl - ps / 2, y_base, xl + ps / 2, y_base + RUST[leg], tone=0.28, region="feet", material="steel-rusted"))
                elev.append(rect(post_id, "frame", "body", 3, xl - ps / 2, y_up, xl + ps / 2, pt, tone=0.4, region="frame", material="steel-coated-weathered"))
                if sc.nuts:
                    for k, dx in enumerate((-NUT_DX, NUT_DX)):
                        elev.append(rect(f"nut-{leg}-r-{k + 1}", "frame", "detail", 2.5, xl + dx - NUT_R, sc.plate_t, xl + dx + NUT_R, sc.plate_t + NUT_H, tone=0.22, region="feet", material="steel-rusted"))
            front.append(rect(plate_id, "frame", "body", zfront, zc - PLATE / 2, 0.0, zc + PLATE / 2, sc.plate_t, tone=0.26, region="feet", material="steel-rusted"))
            if sc.rust_feet:
                front.append(rect(f"post-foot-{leg}-{side}", "frame", "body", zfront + 0.1, zc - ps / 2, y_base, zc + ps / 2, y_base + RUST[leg], tone=0.28, region="feet", material="steel-rusted"))
            front.append(rect(post_id, "frame", "body", zfront + 0.1, zc - ps / 2, y_up, zc + ps / 2, pt, tone=0.4, region="frame", material="steel-coated-weathered"))
            if sc.nuts:
                front.append(rect(f"nut-{leg}-{side}-1", "frame", "detail", zfront + 0.05, zc - NUT_R, sc.plate_t, zc + NUT_R, sc.plate_t + NUT_H, tone=0.22, region="feet", material="steel-rusted"))
            plan.append(rect(plate_id, "frame", "body", 1.5, xl - PLATE / 2, zc - PLATE / 2, xl + PLATE / 2, zc + PLATE / 2, tone=0.26, region="feet", material="steel-rusted"))
            plan.append(rect(post_id, "frame", "body", 2, xl - ps / 2, zc - ps / 2, xl + ps / 2, zc + ps / 2, tone=0.4, region="frame", material="steel-coated-weathered"))
        cross_id = f"cross-{leg}"
        zi = zp - ps / 2
        body.append(wbox(cross_id, "steel-coated-weathered", 0.38, xl - ps / 2, xl + ps / 2, CROSS[leg] - ps / 2, CROSS[leg] + ps / 2, -zi, zi))
        thermal.append({"id": f"th-{cross_id}", "of": cross_id, "region": "frame"})
        structure.append({"id": f"m-{cross_id}", "of": cross_id, "member": f"leg-{leg}", "part": "member"})
        front.append(rect(cross_id, "frame", "body", (2 if leg == "head" else 3) + 0.1, -zi, CROSS[leg] - ps / 2, zi, CROSS[leg] + ps / 2, tone=0.38, region="frame", material="steel-coated-weathered"))
        plan.append(rect(cross_id, "frame", "body", 2, xl - ps / 2, -zi, xl + ps / 2, zi, tone=0.38, region="frame", material="steel-coated-weathered"))

    for leg, xl in LEGS.items():
        fid = f"footing-{leg}"
        body.append(footing_box(leg, 0.78))
        thermal.append({"id": f"th-{fid}", "of": fid, "region": "footings", "sectionFaces": ["-u", "+v", "-v", "+w", "-w"]})
        structure.append({"id": f"m-{fid}", "of": fid, "member": fid, "part": "member"})
        hatch = "concrete" if den in (50, 100) else None
        elev.append(poly(fid, fid, "body", 1, footing_local_rect(leg), tone=0.78, region="footings", material="concrete-c25", hatch=hatch))
        if leg == "foot":
            front.append(rect(fid, fid, "body", 1, -FOOTING_Z / 2, -FOOTING_D, FOOTING_Z / 2, 0.0, tone=0.78, region="footings", material="concrete-c25", hatch=hatch))
        plan.append(rect(fid, fid, "body", 1, xl - FOOTING_X / 2, -FOOTING_Z / 2, xl + FOOTING_X / 2, FOOTING_Z / 2, tone=0.78, region="footings", material="concrete-c25"))

    js = max(sc.feature * 2, 0.02) if den != 500 else 0.075
    wt = max(sc.bar_t, sc.feature)
    jt = sc.bar_t
    for side, sgn in SIDES.items():
        cx, cy = C.offset(X_JOINT, -BAR_D / 2)
        structure.append(dict(box(f"j-weld-{side}", "frame", "steel-coated-weathered", 0.5, [js, BAR_D, jt], [cx, cy, sgn * (BAR_Z - jt / 2)], rot=C.slope(X_JOINT), role="detail"), member="stringer-leg", part="joint"))
        for leg, xl in LEGS.items():
            if den in (200, 500):
                continue
            pt = post_top(xl)
            zi = sgn * (BAR_Z - sc.bar_t)
            structure.append(dict(wbox(f"j-post-{leg}-{side}", "steel-coated-weathered", 0.5, xl - ps / 2, xl + ps / 2, pt - 0.06, pt, zi - wt / 2, zi + wt / 2, role="detail"), member=f"leg-{leg}", part="joint"))
    for leg, xl in LEGS.items():
        jh = max(0.024, sc.feature) if den != 500 else 0.075
        for side, sgn in SIDES.items():
            zc = sgn * zp
            wj = PLATE if den != 500 else ps
            structure.append(dict(wbox(f"j-base-{leg}-{side}", "steel-rusted", 0.5, xl - wj / 2, xl + wj / 2, -jh / 2, jh / 2, zc - wj / 2, zc + wj / 2, role="detail"), member=f"leg-{leg}", part="joint"))
    mr = {50: 0.025, 100: 0.03, 200: 0.04, 500: 0.0775}[den]
    for bone, (com, _) in MASS_COM.items():
        structure.append(dict({"id": f"mass-{bone}", "kind": "ellipsoid", "bone": "frame", "material": "steel-coated-weathered", "role": "detail", "tone": 0.3, "radii": [mr, mr, mr], "transform": {"translation": [r5(com[0]), r5(com[1]), 0]}}, member=bone, part="mass"))

    if den == 50:
        details_50(elev, front, plan)
    if den == 100:
        details_100(elev, front, plan)
    return sc, body, thermal, structure, elev, front, plan


def details_50(elev, front, plan):
    """🧷 The traces of use at 1:50: weld seam, end checks, graffiti tag and sticker residue on the near bar, rust run and the mower dent on the foot posts, abrasion, chewing gum and a split in the worn slats."""
    a, b = C.offset(X_JOINT, 0.0), C.offset(X_JOINT, -BAR_D)
    elev.append(poly("weld-seam", "frame", "detail", 4.5, [a, b], closed=False, filled=False, tone=0.2))
    for i in sorted(CHECKED):
        sl = SLATS[i]
        t = (math.cos(sl["a"]), math.sin(sl["a"]))
        n = (-math.sin(sl["a"]), math.cos(sl["a"]))
        pts = [(sl["x"] + t[0] * du + n[0] * dv, sl["y"] + t[1] * du + n[1] * dv) for du, dv in [(-0.022, 0.012), (-0.004, 0.002), (0.012, -0.003)]]
        elev.append(poly(f"end-check-{i + 1:02d}", "frame", "texture", 6.5, pts, closed=False, filled=False, tone=0.25))
    tag = []
    for k in range(13):
        x = 1.06 + 0.022 * k
        yb = C.offset(x, -BAR_D)[1]
        yt = C.top(x)
        tag.append((x, yb + 0.022 + (0.034 if k % 2 else 0.0) + 0.006 * math.sin(k)))
    elev.append(poly("graffiti-tag", "frame", "texture", 4.6, tag, closed=False, filled=False, tone=0.15))
    xs = 1.72
    yb = C.offset(xs, -BAR_D)[1]
    elev.append(poly("sticker-residue", "frame", "texture", 4.6, [(xs, yb + 0.02), (xs + 0.055, yb + 0.02 - 0.003), (xs + 0.055, yb + 0.058 - 0.003), (xs, yb + 0.058)], closed=True, filled=False, tone=0.2))
    for leg, xl in LEGS.items():
        for k, dx in enumerate((-0.018, -0.004, 0.012)):
            elev.append(poly(f"rust-run-{leg}-{k + 1}", "frame", "texture", 3.6, [(xl + dx, PLATE_T + RUST[leg] + 0.035 - 0.012 * k), (xl + dx + 0.002, PLATE_T + 0.006)], closed=False, filled=False, tone=0.2))
    elev.append(poly("mower-dent", "frame", "texture", 3.7, [(X_FOOT_LEG + 0.03, PLATE_T + 0.03), (X_FOOT_LEG + 0.018, PLATE_T + 0.041), (X_FOOT_LEG + 0.03, PLATE_T + 0.052)], closed=False, filled=False, tone=0.2))
    for i in sorted(WORN):
        xa, xb = PLAN_SPANS[i]
        for k, zz in enumerate((-0.14, 0.0, 0.12)):
            plan.append(poly(f"abrasion-{i + 1:02d}-{k + 1}", "frame", "texture", 6.5, [(xa + 0.2 * (xb - xa), zz - 0.06), (xa + 0.8 * (xb - xa), zz + 0.05)], closed=False, filled=False, tone=0.45))
    for i, zz in ((16, -0.08), (18, 0.15), (21, 0.02)):
        xm = sum(PLAN_SPANS[i]) / 2
        plan.append(shape(f"chewing-gum-{i + 1:02d}", "frame", "texture", 6.6, "ellipse", filled=False, tone=0.2, cx=r5(xm), cy=r5(zz), rx=0.009, ry=0.008))
    xm = sum(PLAN_SPANS[11]) / 2
    plan.append(poly("split-12", "frame", "texture", 6.6, [(xm - 0.004, -0.31), (xm + 0.003, -0.12), (xm - 0.002, 0.05)], closed=False, filled=False, tone=0.2))
    for leg in LEGS:
        for side, sgn in SIDES.items():
            zc = sgn * Z_POST
            front.append(poly(f"rust-run-front-{leg}-{side}", "frame", "texture", (2 if leg == "head" else 3) + 0.2, [(zc - 0.01, PLATE_T + RUST[leg] + 0.02), (zc - 0.008, PLATE_T + 0.005)], closed=False, filled=False, tone=0.2))


def details_100(elev, front, plan):
    """🧷 The traces of use kept at 1:100: the weld seam and the worn slats by their lighter tone."""
    a, b = C.offset(X_JOINT, 0.0), C.offset(X_JOINT, -BAR_D)
    elev.append(poly("weld-seam", "frame", "detail", 4.5, [a, b], closed=False, filled=False, tone=0.2))


def masses():
    """⚖ Masses of the 1:50 parts with their true sections (hollow 60 x 60 x 3 tubes), per bone, with the centres of mass."""
    slat_kg = SLAT_W * SLAT_T * SLAT_L * RHO_TIMBER
    bars = {}
    for part, (xa, xb) in {"back": (C.x0, X_JOINT), "leg": (X_JOINT, C.x1)}.items():
        prof = C.line(xa, xb, 0.0, 0.005) + list(reversed(C.line(xa, xb, -BAR_D, 0.005)))
        ar = area_poly(prof)
        cx = sum(p[0] for p in prof) / len(prof)
        cy = sum(p[1] for p in prof) / len(prof)
        bars[part] = (ar, cx, cy, prof)
    out = {}
    for part, bone in (("back", "stringer-back"), ("leg", "stringer-leg")):
        ar, cx, cy, _ = bars[part]
        m_bar = 2 * ar * BAR_T * RHO_STEEL
        sl = [s for s in SLATS if (s["x"] < X_JOINT) == (part == "back")]
        m_sl = len(sl) * slat_kg
        mx = (m_bar * cx + sum(slat_kg * s["x"] for s in sl)) / (m_bar + m_sl)
        my = (m_bar * cy + sum(slat_kg * s["y"] for s in sl)) / (m_bar + m_sl)
        out[bone] = {"kg": m_bar + m_sl, "com": (mx, my), "bar": m_bar, "slats": m_sl, "n": len(sl)}
    tube_a = POST**2 - (POST - 2 * POST_WALL) ** 2
    for leg, xl in LEGS.items():
        h = post_top(xl) - PLATE_T
        m_posts = 2 * h * tube_a * RHO_STEEL
        m_caps = 2 * (POST - 2 * POST_WALL) ** 2 * POST_WALL * RHO_STEEL
        lc = 2 * (Z_POST - POST / 2)
        m_cross = lc * tube_a * RHO_STEEL
        m_plates = 2 * PLATE * PLATE * PLATE_T * RHO_STEEL
        m_nuts = 4 * 0.017
        tot = m_posts + m_caps + m_cross + m_plates + m_nuts
        my = (m_posts * (PLATE_T + h / 2) + m_caps * post_top(xl) + m_cross * CROSS[leg] + m_plates * PLATE_T / 2 + m_nuts * (PLATE_T + NUT_H / 2)) / tot
        out[f"leg-{leg}"] = {"kg": tot, "com": (xl, my), "posts": m_posts, "cross": m_cross, "plates": m_plates, "caps": m_caps, "nuts": m_nuts}
    return out


def cross_heights():
    """⚖ The cross tube of each portal sits at the portal's own centre of mass (never below the rusted post feet), so the mass marker lies in the tube."""
    tube_a = POST**2 - (POST - 2 * POST_WALL) ** 2
    out = {}
    for leg, xl in LEGS.items():
        h = post_top(xl) - PLATE_T
        m_posts = 2 * h * tube_a * RHO_STEEL
        m_caps = 2 * (POST - 2 * POST_WALL) ** 2 * POST_WALL * RHO_STEEL
        m_cross = 2 * (Z_POST - POST / 2) * tube_a * RHO_STEEL
        m_plates = 2 * PLATE * PLATE * PLATE_T * RHO_STEEL
        m_nuts = 4 * 0.017
        cy = 0.15
        for _ in range(50):
            cy = max(PLATE_T + FOOT_RUST + POST / 2, (m_posts * (PLATE_T + h / 2) + m_caps * post_top(xl) + m_cross * cy + m_plates * PLATE_T / 2 + m_nuts * (PLATE_T + NUT_H / 2)) / (m_posts + m_caps + m_cross + m_plates + m_nuts))
        out[leg] = cy
    return out


CROSS = cross_heights()
RUST = {leg: cy - POST / 2 - PLATE_T for leg, cy in CROSS.items()}
MASS = masses()
MASS_COM = {b: (v["com"], v["kg"]) for b, v in MASS.items()}
MASS_KG = sum(v["kg"] for v in MASS.values())


def region_areas():
    """🌡 Total outer areas of the four regions on the 1:50 model: faces against parts of the same region left out, faces against other regions counted, the footing only by its top."""
    slat = 2 * (SLAT_W * SLAT_L + SLAT_T * SLAT_L + SLAT_W * SLAT_T) * SLAT_N
    frame = 0.0
    for part, (xa, xb) in {"back": (C.x0, X_JOINT), "leg": (X_JOINT, C.x1)}.items():
        prof = C.line(xa, xb, 0.0, 0.005) + list(reversed(C.line(xa, xb, -BAR_D, 0.005)))
        frame += 2 * (2 * area_poly(prof) + perimeter(prof) * BAR_T)
    frame -= 2 * 2 * BAR_D * BAR_T
    feet = 0.0
    nut = 2 * math.pi * NUT_R * NUT_H + math.pi * NUT_R**2
    for leg, xl in LEGS.items():
        h = post_top(xl) - PLATE_T - RUST[leg]
        contact = post_top(xl) - bar_bottom(xl)
        lc = 2 * (Z_POST - POST / 2)
        frame += 2 * (4 * POST * h + 2 * POST**2 - 2 * POST * contact) - 2 * POST**2 + 4 * POST * lc
        feet += 2 * (2 * PLATE**2 + 4 * PLATE * PLATE_T - POST**2 - 2 * math.pi * NUT_R**2 + 4 * POST * RUST[leg] + POST**2 + 2 * nut)
    footing = 2 * FOOTING_X * FOOTING_Z
    return {"slats": slat, "frame": frame, "feet": feet, "footings": footing}


AREAS = region_areas()

REFS = [
    ("din-en-581-1-2017", "DIN EN 581-1:2017-12 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 1: General safety requirements (contract use includes public parks; edges, gaps and finger traps of slatted seating)."),
    ("din-en-581-2-2015", "DIN EN 581-2:2016-01 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 2: Mechanical safety requirements and test methods for seating (static seat and back loads, durability of seating for contract use; basis of the lying load case)."),
    ("din-en-1991-1-1", "DIN EN 1991-1-1:2010-12 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (unit weights: steel 78.5 kN/m3, concrete 24 kN/m3, hardwood 7-8 kN/m3)."),
    ("din-en-338-2016", "DIN EN 338:2016-07 Structural timber - Strength classes (hardwood class D40: mean modulus of elasticity 13 GPa, mean density 700 kg/m3 for the robinia slats)."),
    ("din-en-350-2016", "DIN EN 350:2016-12 Durability of wood and wood-based products - Testing and classification of the durability to biological agents (Robinia pseudoacacia durability class 1-2, used untreated in ground-near outdoor furniture)."),
    ("wagenfuehr-2007", "Wagenführ, R. (2007). Holzatlas, 6th ed., Fachbuchverlag Leipzig (robinia: density 0.73-0.77 g/cm3 at 12 % moisture, greying to silver within one to two seasons outdoors, end and surface checks)."),
    ("din-en-10058-2018", "DIN EN 10058:2019-02 Hot rolled flat steel bars and steel wide flats for general purposes - Dimensions and tolerances (flat bar 80 x 12 mm, 7.54 kg/m)."),
    ("din-en-10219-2-2019", "DIN EN 10219-2:2019-07 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (SHS 60 x 60 x 3: A = 6.61 cm2 nominal with corner radii, 5.19 kg/m, I = 35.1 cm4)."),
    ("din-en-iso-1461-2022", "DIN EN ISO 1461:2022-09 Hot dip galvanized coatings on fabricated iron and steel articles; with DIN EN 13438:2013 powder organic coatings on galvanised steel (duplex system of municipal street furniture; coating failure first at base plates where water stands)."),
    ("din-en-1992-1-1", "DIN EN 1992-1-1:2011-01 Eurocode 2: Design of concrete structures - General rules (C25/30: Ecm = 31 GPa) with DIN EN 206:2021-06 exposure classes XC4/XF1 for unreinforced garden footings."),
    ("din-en-1492-1-2009", "DIN EN 1492-1:2009-05 Textile slings - Safety - Part 1: Flat woven webbing slings made of man-made fibres for general purpose use (1 t violet sling, colour code and working load limit of the lifting straps)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity, specific heat and density of hardwood, steel and concrete)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity and solar absorptance of oxidised steel, paints, concrete and wood)."),
    ("berdahl-bretz-1997", "Berdahl, P., Bretz, S. E. (1997). Preliminary survey of the solar reflectance of cool roofing materials. Energy and Buildings 25(2), 149-158 (solar reflectance of weathered versus new surfaces: dark grey coatings 0.2-0.3, weathered concrete 0.3-0.4)."),
    ("iso-7730-2005", "ISO 7730:2005 Ergonomics of the thermal environment (skin and light-clothing surface temperatures of a resting person, 32-34 degC)."),
    ("dubois-1916", "Du Bois, D., Du Bois, E. F. (1916). A formula to estimate the approximate surface area if height and weight be known. Archives of Internal Medicine 17, 863-871 (1.9 m2 body surface of an 80 kg adult; about a quarter of it rests on a lounger)."),
    ("park-lounger-catalogues", "Catalogue-typical data of fixed park loungers (Parkliegen, Sonnenliegen) of German municipal street-furniture makers as a product type, no brand: 1.9-2.1 m long, 0.6-0.8 m wide, lying surface 0.35-0.45 m above ground, head end 0.70-0.85 m, back reclined 25-35 degrees, hardwood slats 40-45 x 70-90 mm on two bent flat-bar stringers 80-100 x 10-15 mm, galvanised and powder-coated steel legs on base plates bolted to concrete footings, 80-120 kg (authored estimate)."),
]


def materials():
    """🧱 Weathered robinia, coated steel, rusted steel at the feet and the footing concrete, with the weathered optics stated against new surfaces."""
    return [
        {"id": "robinia-weathered", "title": T("Weathered robinia, greyed", "Verwitterte Robinie, vergraut"), "emissivity": 0.92, "solarAbsorptivity": 0.62, "densityKgM3": RHO_TIMBER, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 13000000000, "strengthPa": 40000000, "source": "literature", "reference": "wagenfuehr-2007"},
        {"id": "steel-coated-weathered", "title": T("Duplex-coated steel, chalked and flaking", "Duplex-beschichteter Stahl, gekreidet und abblätternd"), "emissivity": 0.9, "solarAbsorptivity": 0.78, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "din-en-iso-1461-2022"},
        {"id": "steel-rusted", "title": T("Rusted steel at the feet", "Angerosteter Stahl an den Füßen"), "emissivity": 0.8, "solarAbsorptivity": 0.85, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
        {"id": "concrete-c25", "title": T("Concrete C25/30, weathered top", "Beton C25/30, verwitterte Oberseite"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": RHO_CONCRETE, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 31000000000, "source": "literature", "reference": "iso-10456-2007"},
    ]


def bone(id, parent, title, x, y, angle_deg, length, joint, mass=None, section=None, z=None):
    """🦴 A bone of the rig."""
    b = {"id": id, "parent": parent, "title": title, "x": r5(x), "y": r5(y)}
    if z is not None:
        b["z"] = r5(z)
    b.update({"angleDeg": r5(angle_deg), "lengthM": r5(length), "joint": joint})
    if mass:
        b["mass"] = mass
    if section:
        b["section"] = section
    return b


def seg_mass(bone_id, source="authored-estimate", reference="din-en-1991-1-1"):
    """⚖ The segment mass of a bone at its centre in the bone frame."""
    m = MASS[bone_id]
    u, v = local(bone_id, m["com"])
    return {"kg": r5(m["kg"]), "comX": r5(u), "comY": r5(v), "distribution": "point", "source": source, "reference": reference}


def rig():
    """🌳 Ground, the two footings that stay, the lifting frame (prismatic) with the two stringer runs and the two leg portals."""
    bar_sec = {"areaM2": r5(2 * BAR_D * BAR_T), "secondMomentM4": 1.024e-06, "extremeFibreM": BAR_D / 2, "material": "steel-coated-weathered", "title": T("Two flat bars 80 x 12 mm on edge", "Zwei Flachstähle 80 x 12 mm hochkant")}
    tube_a = POST**2 - (POST - 2 * POST_WALL) ** 2
    tube_i = (POST**4 - (POST - 2 * POST_WALL) ** 4) / 12
    leg_sec = {"areaM2": r5(2 * tube_a), "secondMomentM4": float(f"{2 * tube_i:.4e}"), "extremeFibreM": POST / 2, "material": "steel-coated-weathered", "title": T("Two square hollow sections 60 x 60 x 3 mm", "Zwei Quadratrohre 60 x 60 x 3 mm")}
    footing_sec = {"areaM2": r5(FOOTING_X * FOOTING_Z), "secondMomentM4": r5(FOOTING_Z * FOOTING_X**3 / 12), "extremeFibreM": FOOTING_X / 2, "material": "concrete-c25", "title": T("Strip footing 300 x 800 mm", "Streifenfundament 300 x 800 mm")}
    fixed = {"type": "fixed"}
    return {
        "kind": "structural",
        "root": "ground",
        "bones": [
            bone("ground", None, T("Lawn reference at ground level", "Rasenbezug auf Geländehöhe"), 0, 0, 0, 0, fixed),
            bone("footing-head", "ground", T("Strip footing under the head portal (stays in the lawn)", "Streifenfundament unter dem Kopfjoch (bleibt im Rasen)"), X_HEAD_LEG, -FOOTING_D, 90, FOOTING_D, fixed, section=footing_sec),
            bone("footing-foot", "ground", T("Strip footing under the foot portal (stays in the lawn)", "Streifenfundament unter dem Fußjoch (bleibt im Rasen)"), X_FOOT_LEG, -FOOTING_D, 90, FOOTING_D, fixed, section=footing_sec),
            bone("frame", "ground", T("Lounger frame, lifted off the footings by the depot", "Liegengestell, vom Bauhof von den Fundamenten gehoben"), 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 1.2}),
            bone("stringer-back", "frame", T("Back run of the two bent flat-bar stringers", "Rückenteil der zwei gebogenen Flachstahlholme"), BACK_O[0], BACK_O[1], math.degrees(BACK_A), BACK_LEN, fixed, mass=seg_mass("stringer-back"), section=bar_sec),
            bone("stringer-leg", "stringer-back", T("Seat and leg run of the stringers, butt-welded at the hip", "Sitz- und Beinteil der Holme, an der Hüfte stumpf geschweißt"), BACK_LEN, 0, math.degrees(LEGB_A - BACK_A), LEGB_LEN, fixed, mass=seg_mass("stringer-leg"), section=bar_sec),
            bone("leg-head", "frame", T("Head leg portal: two posts, cross tube, base plates", "Kopfjoch: zwei Pfosten, Querrohr, Fußplatten"), X_HEAD_LEG, 0, 90, post_top(X_HEAD_LEG), fixed, mass=seg_mass("leg-head", "literature", "din-en-10219-2-2019"), section=leg_sec),
            bone("leg-foot", "frame", T("Foot leg portal: two posts, cross tube, base plates", "Fußjoch: zwei Pfosten, Querrohr, Fußplatten"), X_FOOT_LEG, 0, 90, post_top(X_FOOT_LEG), fixed, mass=seg_mass("leg-foot", "literature", "din-en-10219-2-2019"), section=leg_sec),
        ],
    }


def anchors():
    """🪝 The soil bearing under each footing and the four sling points on the stringers next to the portals."""
    out = []
    for leg in LEGS:
        out.append({"id": f"footing-{leg}-base", "bone": f"footing-{leg}", "x": 0, "y": 0, "kind": "support", "title": T(f"Soil bearing under the {leg} footing, 0.5 m deep", f"Bodenauflager unter dem {'Kopf' if leg == 'head' else 'Fuß'}fundament, 0,5 m tief"), "support": "fixed", "footprintLengthM": FOOTING_X, "footprintWidthM": FOOTING_Z})
    for leg, xl in LEGS.items():
        b = "stringer-back" if xl < X_JOINT else "stringer-leg"
        u, v = local(b, C.offset(xl, -BAR_D / 2))
        for side, sgn in SIDES.items():
            out.append({"id": f"sling-{leg}-{side}", "bone": b, "x": r5(u), "y": r5(v), "z": r5(sgn * BAR_Z), "kind": "support", "title": T(f"Sling choked round the {'left' if sgn > 0 else 'right'} stringer at the {leg} portal", f"Hebeband im Schnürgang um den {'linken' if sgn > 0 else 'rechten'} Holm am {'Kopf' if leg == 'head' else 'Fuß'}joch"), "support": "pin", "footprintLengthM": 0.05, "footprintWidthM": BAR_T})
    return out


def binding(slats, frame, feet, footings):
    """🌬 A thermal binding of the four regions."""
    return {"regions": {"slats": slats, "frame": frame, "feet": feet, "footings": footings}, "emitters": {}}


def sunbather_loads():
    """🧍 An 80 kg sunbather lying on his back: trunk, head and arms on the back run, pelvis and legs on the seat and leg run (segment shares after Dempster)."""
    w = SUNBATHER_KG * G
    upper, lower = 0.62 * w, 0.38 * w
    a0, a1 = 0.10, BACK_LEN - 0.02
    b0, b1 = 0.0, 0.92 * LEGB_LEN
    return [
        {"id": "sunbather-trunk", "kind": "occupant", "title": T("Sunbather: head, trunk and arms (62 % of 80 kg)", "Sonnenbadende Person: Kopf, Rumpf und Arme (62 % von 80 kg)"), "fxN": 0, "fyN": 0, "distributed": {"bone": "stringer-back", "fromM": r5(a0), "toM": r5(a1), "wxNPerM": 0, "wyNPerM": r5(-upper / (a1 - a0))}, "source": "authored-estimate", "reference": "din-en-581-2-2015"},
        {"id": "sunbather-legs", "kind": "occupant", "title": T("Sunbather: pelvis and legs (38 % of 80 kg)", "Sonnenbadende Person: Becken und Beine (38 % von 80 kg)"), "fxN": 0, "fyN": 0, "distributed": {"bone": "stringer-leg", "fromM": r5(b0), "toM": r5(b1), "wxNPerM": 0, "wyNPerM": r5(-lower / (b1 - b0))}, "source": "authored-estimate", "reference": "din-en-581-2-2015"},
    ]


BASES = ["footing-head-base", "footing-foot-base"]
SLINGS = ["sling-head-l", "sling-head-r", "sling-foot-l", "sling-foot-r"]


def poses():
    """🧍 In service on its footings, lifted out on four slings by the depot, and a sunbather lying in the midday sun."""
    service = binding({"exposedFraction": 1, "sunlitFraction": 0.55}, {"exposedFraction": 1, "sunlitFraction": 0.3}, {"exposedFraction": 0.8, "sunlitFraction": 0.2, "contact": {"fraction": 0.2, "tempC": 30}}, {"exposedFraction": 0.85, "sunlitFraction": 0.5, "contact": {"fraction": 0.15, "tempC": 30}})
    lifted = binding({"exposedFraction": 1, "sunlitFraction": 0.5}, {"exposedFraction": 1, "sunlitFraction": 0.35}, {"exposedFraction": 1, "sunlitFraction": 0.3}, {"exposedFraction": 1, "sunlitFraction": 0.6})
    lying = binding({"exposedFraction": 0.86, "sunlitFraction": 0.3, "contact": {"fraction": 0.14, "tempC": 33}}, {"exposedFraction": 1, "sunlitFraction": 0.22}, {"exposedFraction": 0.8, "sunlitFraction": 0.2, "contact": {"fraction": 0.2, "tempC": 30}}, {"exposedFraction": 0.85, "sunlitFraction": 0.45, "contact": {"fraction": 0.15, "tempC": 30}})
    return [
        {"id": "in-service", "title": T("In service", "Im Einsatz"), "description": T("Empty lounger on its two strip footings in the lawn of a city park at midday: base plates bolted down with M12 anchors, greyed slats in the sun.", "Leere Liege auf ihren zwei Streifenfundamenten im Rasen eines Stadtparks zur Mittagszeit: Fußplatten mit M12-Ankern verschraubt, vergraute Latten in der Sonne."), "bones": {}, "ground": "authored", "contacts": BASES, "thermal": service, "loads": []},
        {"id": "maintenance", "title": T("Lifted out by the depot", "Vom Bauhof ausgehoben"), "description": T("The anchor nuts are off and the frame hangs 0.45 m above the footings on four flat webbing slings choked round the stringers next to the portals; the footings with their anchor studs stay in the lawn.", "Die Ankermuttern sind ab, das Gestell hängt 0,45 m über den Fundamenten an vier Hebebändern, die neben den Jochen im Schnürgang um die Holme gelegt sind; die Fundamente mit ihren Ankerbolzen bleiben im Rasen."), "bones": {"frame": {"dy": LIFT}}, "ground": "authored", "contacts": SLINGS, "thermal": lifted, "loads": []},
        {"id": "in-use", "title": T("Sunbather lying", "Sonnenbadende Person liegt"), "description": T("An 80 kg adult lies on his back with the head on the raised end; the body covers and shades the worn middle of the slats.", "Ein Erwachsener mit 80 kg liegt auf dem Rücken, den Kopf am hohen Ende; der Körper bedeckt und beschattet die abgenutzte Mitte der Latten."), "bones": {}, "ground": "authored", "contacts": BASES, "thermal": lying, "loads": sunbather_loads()},
    ]


def clips():
    """🎞 The depot lifts the frame out, a sunbather lies down, and the shade of a park tree passes over the lounger."""
    return [
        {"id": "maintenance", "title": T("Lifted out by the depot", "Ausheben durch den Bauhof"), "description": T("The crew takes off the eight anchor nuts, the slings take the weight and the frame rises 0.45 m off the footings, which stay in the lawn.", "Die Kolonne nimmt die acht Ankermuttern ab, die Hebebänder nehmen das Gewicht auf und das Gestell hebt sich 0,45 m von den Fundamenten, die im Rasen bleiben."), "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "step"}, {"t": 2.5, "pose": "in-service", "ease": "smooth"}, {"t": 8, "pose": "maintenance", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 2.5} for a in BASES] + [{"anchor": a, "fromS": 2.5, "toS": 8} for a in SLINGS],
         "phases": [{"id": "unbolt", "fromS": 0, "toS": 2.5, "title": T("Anchor nuts off, slings tensioned", "Ankermuttern ab, Hebebänder gespannt")}, {"id": "lift", "fromS": 2.5, "toS": 8, "title": T("Lift off the footings", "Abheben von den Fundamenten")}],
         "events": [{"t": 2.5, "kind": "contact-off", "title": T("Base plates leave the footings", "Fußplatten verlassen die Fundamente"), "target": "footing-head-base"}, {"t": 2.5, "kind": "contact-on", "title": T("Slings take the frame", "Hebebänder übernehmen das Gestell"), "target": "sling-head-l"}, {"t": 8, "kind": "motion", "title": T("Frame clear for the pallet", "Gestell frei für die Palette")}],
         "tracks": []},
        {"id": "sunbather-lies-down", "title": T("Sunbather lies down", "Sonnenbadende Person legt sich hin"), "description": T("A visitor sits on the seat, swings the legs up and lies back; the load moves from the seat to the back run and the body shades the slats.", "Eine Besucherin setzt sich auf den Sitz, schwingt die Beine hoch und legt sich zurück; die Last wandert vom Sitz auf das Rückenteil, der Körper beschattet die Latten."), "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 4, "pose": "in-use", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 4} for a in BASES],
         "phases": [{"id": "sit", "fromS": 0, "toS": 1.5, "title": T("Sitting down", "Hinsetzen")}, {"id": "lie-back", "fromS": 1.5, "toS": 4, "title": T("Lying back", "Zurücklegen")}],
         "events": [{"t": 1.0, "kind": "load-on", "title": T("Pelvis on the seat", "Becken auf dem Sitz"), "target": "sunbather-legs"}, {"t": 3.0, "kind": "load-on", "title": T("Back on the back run", "Rücken auf dem Rückenteil"), "target": "sunbather-trunk"}],
         "tracks": [{"target": "load:sunbather-legs", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1.0, "v": 1}, {"t": 4, "v": 1}]}, {"target": "load:sunbather-trunk", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1.5, "v": 0}, {"t": 3.0, "v": 1}, {"t": 4, "v": 1}]}]},
        {"id": "tree-shade-passes", "title": T("Tree shade passes", "Baumschatten zieht vorbei"), "description": T("Illustrative afternoon: the shade of a park lime moves over the lounger and the sun returns; the dark coated frame heats and cools faster than the slats.", "Beispielnachmittag: Der Schatten einer Parklinde wandert über die Liege und die Sonne kehrt zurück; das dunkle beschichtete Gestell erwärmt und kühlt sich schneller als die Latten."), "durationS": 60, "fps": 30, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 30, "pose": "in-service", "ease": "smooth"}, {"t": 60, "pose": "in-service", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 60} for a in BASES],
         "phases": [{"id": "shade", "fromS": 0, "toS": 30, "title": T("Shade", "Schatten")}, {"id": "sun", "fromS": 30, "toS": 60, "title": T("Sun", "Sonne")}],
         "events": [{"t": 30, "kind": "thermal", "title": T("Sun back on the lounger", "Sonne wieder auf der Liege")}],
         "tracks": [{"target": f"region:{r}:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": lo}, {"t": 30, "v": hi}, {"t": 60, "v": lo}]} for r, lo, hi in (("slats", 0.08, 0.55), ("frame", 0.05, 0.3), ("feet", 0.03, 0.2), ("footings", 0.1, 0.5))]},
    ]


def thermal():
    """🌡 Four passive regions in the summer sun with weathered optics; no rating, the asset draws 0 W."""
    def region(id, title, mat, area, length, geometry, sky, layer, cap, src, ref):
        return {"id": id, "title": title, "kind": "surface", "model": "passive", "material": mat, "areaM2": r5(area), "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky, "layers": [layer], "heatCapacityJM2K": cap, "source": src, "reference": ref}
    return {
        "environment": {"airTempC": 28, "meanRadiantTempC": 30, "skyTempC": 12, "groundTempC": 36, "relativeHumidity": 0.45, "windMS": 1.2, "windDirectionDeg": 90, "solarWM2": 850, "diffuseFraction": 0.15, "pressurePa": 101325},
        "referenceEnvironment": "summer-hot-sun",
        "passengers": [],
        "operating": [],
        "regions": [
            region("slats", T("Greyed robinia slats", "Vergraute Robinienlatten"), "robinia-weathered", AREAS["slats"], SLAT_W, "horizontal-plate-up", 0.85, {"id": "robinia-slat", "title": T("Robinia slat 40 mm", "Robinienlatte 40 mm"), "kind": "timber", "material": "robinia-weathered", "thicknessM": SLAT_T, "resistanceM2KW": r5(SLAT_T / 0.17), "heatCapacityJM2K": 23700, "source": "literature", "reference": "iso-10456-2007"}, 23700, "literature", "wagenfuehr-2007"),
            region("frame", T("Coated steel frame: stringers, posts, cross tubes", "Beschichtetes Stahlgestell: Holme, Pfosten, Querrohre"), "steel-coated-weathered", AREAS["frame"], BAR_D, "vertical-plate", 0.45, {"id": "steel-section", "title": T("Flat bar 12 mm, tube wall 3 mm", "Flachstahl 12 mm, Rohrwand 3 mm"), "kind": "metal", "material": "steel-coated-weathered", "thicknessM": 0.006, "resistanceM2KW": 0.00012, "heatCapacityJM2K": 21700, "source": "literature", "reference": "iso-10456-2007"}, 21700, "literature", "din-en-iso-1461-2022"),
            region("feet", T("Rusted feet: base plates, post feet, anchor nuts", "Angerostete Füße: Fußplatten, Pfostenfüße, Ankermuttern"), "steel-rusted", AREAS["feet"], PLATE, "horizontal-plate-up", 0.3, {"id": "plate", "title": T("Base plate 12 mm", "Fußplatte 12 mm"), "kind": "metal", "material": "steel-rusted", "thicknessM": PLATE_T, "resistanceM2KW": 0.00024, "heatCapacityJM2K": 43300, "source": "literature", "reference": "iso-10456-2007"}, 43300, "literature", "incropera-2018"),
            region("footings", T("Footing tops flush with the lawn", "Fundamentoberseiten bündig mit dem Rasen"), "concrete-c25", AREAS["footings"], FOOTING_X, "horizontal-plate-up", 0.6, {"id": "concrete", "title": T("Concrete C25/30", "Beton C25/30"), "kind": "masonry", "material": "concrete-c25", "thicknessM": FOOTING_D, "resistanceM2KW": r5(FOOTING_D / 2.0), "heatCapacityJM2K": 120000, "source": "literature", "reference": "iso-10456-2007"}, 120000, "literature", "iso-10456-2007"),
        ],
        "emitters": [],
        "assumptions": [
            T("Passive surfaces in the sun: absorbed short-wave, long-wave exchange with sky and lawn, convection and storage; no emitters, no rating: the lounger draws 0 W.", "Passive Flächen in der Sonne: absorbierte Kurzwelle, Langwellenaustausch mit Himmel und Rasen, Konvektion und Speicherung; keine Emitter, keine Nennleistung: die Liege nimmt 0 W auf."),
            T("Weathering changes the optics: greyed robinia absorbs about 0.62 of the sun against 0.45 when new, the chalked dark grey coating 0.78 against 0.7, rust at the feet 0.85; emissivities 0.8-0.92 (Incropera, Berdahl and Bretz, authored estimates for the weathered state).", "Verwitterung verändert die Optik: vergraute Robinie absorbiert etwa 0,62 der Sonne gegenüber 0,45 im Neuzustand, die gekreidete dunkelgraue Beschichtung 0,78 gegenüber 0,7, Rost an den Füßen 0,85; Emissionsgrade 0,8-0,92 (Incropera, Berdahl und Bretz, Schätzwerte für den verwitterten Zustand)."),
            T("Areal heat capacity rho*cp*d over the half thickness that takes part within one hour (slat 20 mm, bar and tube 6 mm, plate 12 mm, concrete 50 mm).", "Flächenbezogene Wärmekapazität rho*cp*d über die halbe Dicke, die binnen einer Stunde teilnimmt (Latte 20 mm, Flachstahl und Rohr 6 mm, Platte 12 mm, Beton 50 mm)."),
            T("The footings count only their top face; their sides and bottom lie against the soil and are section faces. Contact fractions: plates on the footing tops (0.2 of the feet at 30 degC), a lying sunbather on 0.14 of the slat area (0.55 m2 of 3.9 m2, skin and swimwear at 33 degC, ISO 7730, Du Bois).", "Die Fundamente zählen nur ihre Oberseite; Seiten und Sohle liegen am Boden an und sind Schnittflächen. Kontaktanteile: Platten auf den Fundamenten (0,2 der Füße bei 30 degC), eine liegende Person auf 0,14 der Lattenfläche (0,55 m2 von 3,9 m2, Haut und Badekleidung 33 degC, ISO 7730, Du Bois)."),
            T("Reference case: a hot park afternoon, 28 degC air, 850 W/m2 sun, 1.2 m/s breeze; the archive's summer-hot-sun preset is the reference environment.", "Referenzfall: ein heißer Parknachmittag, 28 degC Luft, 850 W/m2 Sonne, 1,2 m/s Wind; die Archiv-Voreinstellung summer-hot-sun ist die Referenzumgebung."),
        ],
    }


def structure_model():
    """🏗 Quasi-static rigid body: on the footing soil bearings in service and in use, on the four sling pins in maintenance."""
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
        T("The welded frame and the two footings are one rigid body for the solver; each pose names its supports: the soil bearing under both footings (fixed, 0.3 x 0.8 m) in service and in use, the four sling points on the stringers (pins) when the depot lifts it.", "Das geschweißte Gestell und die zwei Fundamente sind für den Löser ein starrer Körper; jede Pose nennt ihre Auflager: das Bodenauflager unter beiden Fundamenten (eingespannt, 0,3 x 0,8 m) im Einsatz und in Benutzung, die vier Anschlagpunkte an den Holmen (Gelenke), wenn der Bauhof sie hebt."),
        T("Masses from the true sections: robinia slats 70 x 40 x 700 mm at 740 kg/m3, flat bars 80 x 12 mm and SHS 60 x 60 x 3 at 7850 kg/m3, base plates 150 x 150 x 12; the site-cast footings (2 x 288 kg) are drawn but not counted in the furniture mass, as for the street lamp's foundation, so the slings carry the frame alone.", "Massen aus den wahren Querschnitten: Robinienlatten 70 x 40 x 700 mm mit 740 kg/m3, Flachstähle 80 x 12 mm und Quadratrohre 60 x 60 x 3 mit 7850 kg/m3, Fußplatten 150 x 150 x 12; die vor Ort betonierten Fundamente (2 x 288 kg) sind gezeichnet, zählen aber nicht zur Möbelmasse, wie das Fundament der Straßenleuchte, so tragen die Hebebänder nur das Gestell."),
        T("Lying load: an 80 kg adult as two line loads, 62 % over the back run and 38 % over the seat and leg run (Dempster segment shares); the EN 581-2 static seat test for contract seating (1300 N) is the strength reference, not modelled.", "Liegelast: ein Erwachsener mit 80 kg als zwei Streckenlasten, 62 % über dem Rückenteil und 38 % über Sitz- und Beinteil (Segmentanteile nach Dempster); die statische Sitzprüfung nach EN 581-2 für den Objektbereich (1300 N) ist die Festigkeitsreferenz, nicht modelliert."),
        T("Wear does not weaken the load path: checked slats, flaking coating and surface rust at the feet are below any section loss; the lounger is still safe and in service.", "Die Abnutzung schwächt den Lastpfad nicht: gerissene Latten, abblätternde Beschichtung und Flugrost an den Füßen liegen unter jedem Querschnittsverlust; die Liege ist weiter sicher und im Einsatz."),
    ]}


REP_DECISIONS = {
    50: {
        "silhouette": T("Lounger 40 x 15 mm above a 10 mm deep footing: the near flat-bar stringer following the lying curve, the 24 slat ends with eased edges riding on it, the two portals with rusted post feet, base plates and anchor nuts, the strip footings under the lawn. Front: the rising slat field over the foot portal, bars edge-on, posts, cross tube and plates. Plan: every slat with the drainage gaps, the bars showing through, the footings beside the slats.", "Liege 40 x 15 mm über 10 mm tiefem Fundament: der nahe Flachstahlholm in der Liegekurve, die 24 Lattenenden mit gebrochenen Kanten darauf, die zwei Joche mit angerosteten Pfostenfüßen, Fußplatten und Ankermuttern, die Streifenfundamente unter dem Rasen. Vorderansicht: das ansteigende Lattenfeld über dem Fußjoch, Holme in Kantenansicht, Pfosten, Querrohr und Platten. Draufsicht: jede Latte mit den Fugen, durchscheinende Holme, die Fundamente neben den Latten."),
        "retained": T("True curve (head 0.76 m, lying surface 0.37 m, back 30-35 degrees), 70 x 40 slats at 18 mm gaps, 80 x 12 bars, 60 mm tubes, 12 mm plates, M12 nuts, the hip weld.", "Wahre Kurve (Kopf 0,76 m, Liegefläche 0,37 m, Lehne 30-35 Grad), Latten 70 x 40 mit 18 mm Fugen, Holme 80 x 12, Rohre 60 mm, Platten 12 mm, Muttern M12, die Hüftnaht."),
        "merged": T("Left and right bars and posts coincide in the side view; the far parts hide behind the near ones.", "Linke und rechte Holme und Pfosten fallen in der Seitenansicht zusammen; die fernen Teile verdecken die nahen."),
        "omitted": T("Countersunk slat screws (8 mm heads), the anchor studs inside the footings, the lawn edge.", "Senkkopfschrauben der Latten (8-mm-Köpfe), die Ankerbolzen im Fundament, die Rasenkante."),
        "thinElements": T("Bars 12 mm (0.24 mm) and plates 12 mm drawn true; slat gaps 18 mm close under the 0.35 mm slat pen.", "Holme 12 mm (0,24 mm) und Platten 12 mm wahr gezeichnet; die 18-mm-Fugen schließen sich unter dem 0,35-mm-Lattenstift."),
        "hatching": T("Robinia grain along the slats in plan; concrete stipple in the footings.", "Robinienmaserung längs der Latten in der Draufsicht; Betonpunktierung in den Fundamenten."),
        "styles": T("Slats 0.6, worn shoulder and hip slats 0.72, bars 0.34, posts 0.4, rusted feet 0.26-0.28, concrete 0.78.", "Latten 0,6, abgenutzte Schulter- und Hüftlatten 0,72, Holme 0,34, Pfosten 0,4, rostige Füße 0,26-0,28, Beton 0,78."),
        "overlays": T("Traces of use as texture: end checks, graffiti tag and sticker residue on the near bar, rust runs and a mower dent at the foot posts, abrasion, a split and chewing gum on the worn slats.", "Gebrauchsspuren als Textur: Endrisse, Graffiti-Tag und Aufkleberreste am nahen Holm, Rostfahnen und eine Mäherdelle an den Fußpfosten, Abrieb, ein Riss und Kaugummi auf den abgenutzten Latten."),
    },
    100: {
        "silhouette": T("Lounger 20 x 7.6 mm: the near stringer, the 24 slat ends as plain boards, the two portals on 15 mm plates, the footings. Front: slat field, bars, portals. Plan: every slat, bars, footings.", "Liege 20 x 7,6 mm: der nahe Holm, die 24 Lattenenden als schlichte Bretter, die zwei Joche auf 15-mm-Platten, die Fundamente. Vorderansicht: Lattenfeld, Holme, Joche. Draufsicht: jede Latte, Holme, Fundamente."),
        "retained": T("True curve, single slats, bar depth 80 mm, posts and cross tubes 60 mm, the hip weld, the lighter worn slats.", "Wahre Kurve, einzelne Latten, Holmhöhe 80 mm, Pfosten und Querrohre 60 mm, die Hüftnaht, die helleren abgenutzten Latten."),
        "merged": T("Post feet into the posts; plates thickened to 15 mm.", "Pfostenfüße in die Pfosten; Platten auf 15 mm verstärkt."),
        "omitted": T("Eased slat edges, nuts, screws, checks, graffiti, stickers, rust runs, chewing gum.", "Gebrochene Lattenkanten, Muttern, Schrauben, Risse, Graffiti, Aufkleber, Rostfahnen, Kaugummi."),
        "thinElements": T("Bars drawn 15 mm (0.15 mm) on their outer face.", "Holme 15 mm (0,15 mm) an ihrer Außenseite gezeichnet."),
        "hatching": T("Concrete stipple in the footings.", "Betonpunktierung in den Fundamenten."),
        "styles": T("As 1:50.", "Wie 1:50."),
        "overlays": T("One region per shape; wear only as slat tone.", "Eine Region je Form; Abnutzung nur als Latten-Tonwert."),
    },
    200: {
        "silhouette": T("Lounger 10 x 3.8 mm: the slat field as one curved band 40 mm thick on the 30 mm stringers, posts, cross tubes, plates and footings. Plan: the band with the slat joints.", "Liege 10 x 3,8 mm: das Lattenfeld als ein gebogenes Band 40 mm dick auf den 30-mm-Holmen, Pfosten, Querrohre, Platten und Fundamente. Draufsicht: das Band mit den Lattenfugen."),
        "retained": T("Curve, head and lying heights, the two portals, the footings.", "Kurve, Kopf- und Liegehöhe, die zwei Joche, die Fundamente."),
        "merged": T("24 slats into one band, gaps closed; plates 30 mm.", "24 Latten zu einem Band, Fugen geschlossen; Platten 30 mm."),
        "omitted": T("Weld seam, nuts, all traces of use but the tone.", "Schweißnaht, Muttern, alle Gebrauchsspuren außer dem Tonwert."),
        "thinElements": T("Bars 30 mm (0.15 mm).", "Holme 30 mm (0,15 mm)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Band 0.6, bars 0.34, posts 0.4, feet 0.26, concrete 0.78.", "Band 0,6, Holme 0,34, Pfosten 0,4, Füße 0,26, Beton 0,78."),
        "overlays": T("Slat joints as detail lines in plan and the back slat edges in front.", "Lattenfugen als Detaillinien in der Draufsicht und die Kanten der Rückenlatten in der Vorderansicht."),
    },
    500: {
        "silhouette": T("Lounger 4 x 1.5 mm: one curved slab 120 mm deep for slats and stringers, two 75 mm leg portals on rusted feet, the footings.", "Liege 4 x 1,5 mm: eine gebogene Platte 120 mm tief für Latten und Holme, zwei 75-mm-Joche auf rostigen Füßen, die Fundamente."),
        "retained": T("Length 2.0 m, head 0.76 m, lying surface 0.37 m, the two portals, the footings 0.5 m deep.", "Länge 2,0 m, Kopf 0,76 m, Liegefläche 0,37 m, die zwei Joche, die Fundamente 0,5 m tief."),
        "merged": T("Slats and stringers into one slab; posts and cross tube into a portal; plates into the 75 mm feet.", "Latten und Holme zu einer Platte; Pfosten und Querrohr zu einem Joch; Platten in die 75-mm-Füße."),
        "omitted": T("Gaps, weld, nuts, traces of use.", "Fugen, Naht, Muttern, Gebrauchsspuren."),
        "thinElements": T("Every part at least 75 mm (0.15 mm).", "Jedes Teil mindestens 75 mm (0,15 mm)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Slab 0.6, portals 0.4, feet 0.26, concrete 0.78.", "Platte 0,6, Joche 0,4, Füße 0,26, Beton 0,78."),
        "overlays": T("The slab carries the slat and frame regions together.", "Die Platte trägt die Latten- und Gestellregion gemeinsam."),
    },
}

VOL_DECISIONS = {
    50: [T("Body for 7.5 mm: 24 robinia slats 70 x 40 x 700 mm with 4 mm eased edges, each turned to the local slope of the lying curve and lifted where the curve sags so it bears on both bar edges; two bent flat bars 80 x 12 mm in two runs butt-welded at the hip (the seam is the boundary between the back and leg solids); two portals of SHS 60 x 60 posts welded against the inner bar faces with a 60 mm cross tube, the bottom 60 mm of each post as rusted foot, 150 x 150 x 12 base plates with two M12 nuts each; two strip footings 300 x 500 x 800 mm flush with the lawn.", "Körper für 7,5 mm: 24 Robinienlatten 70 x 40 x 700 mm mit 4 mm gebrochenen Kanten, jede auf die örtliche Neigung der Liegekurve gedreht und über Mulden angehoben, damit sie auf beiden Holmkanten aufliegt; zwei gebogene Flachstähle 80 x 12 mm in zwei Teilen, an der Hüfte stumpf geschweißt (die Naht ist die Grenze zwischen Rücken- und Beinkörper); zwei Joche aus Quadratrohrpfosten 60 x 60, an die Holminnenseiten geschweißt, mit 60-mm-Querrohr, die unteren 60 mm jedes Pfostens als angerosteter Fuß, Fußplatten 150 x 150 x 12 mit je zwei Muttern M12; zwei Streifenfundamente 300 x 500 x 800 mm bündig mit dem Rasen."),
         T("Thermal references every body part by region (slats, frame, feet, footings; the footing sides and sole are section faces against the soil). Structure: the four bar runs, posts, feet, plates, cross tubes and footings are members, the hip welds, post welds and base anchorages joints, four mass markers at the centres of the slatted runs and of the portals.", "Thermik verweist jeden Körperteil nach Region (Latten, Gestell, Füße, Fundamente; Seiten und Sohle der Fundamente sind Schnittflächen gegen den Boden). Tragwerk: die vier Holmteile, Pfosten, Füße, Platten, Querrohre und Fundamente sind Stäbe, die Hüftnähte, Pfostennähte und Fußverankerungen Knoten, vier Massenmarken in den Schwerpunkten der belatteten Teile und der Joche.")],
    100: [T("1:100 for 15 mm: the 24 slats stay single boards with square edges; the bars are 15 mm thick on their outer face; the rusted post feet merge into the posts, the plates are 15 mm and carry the rust region alone; the nuts are left out.", "1:100 für 15 mm: die 24 Latten bleiben einzelne Bretter mit scharfen Kanten; die Holme sind an ihrer Außenseite 15 mm dick; die rostigen Pfostenfüße gehen in die Pfosten auf, die Platten sind 15 mm und tragen die Rostregion allein; die Muttern entfallen."),
         T("Thermal and structure as at 1:50 on these parts; joints 30 mm, mass markers 60 mm.", "Thermik und Tragwerk wie bei 1:50 auf diesen Teilen; Knoten 30 mm, Massenmarken 60 mm.")],
    200: [T("1:200 for 30 mm: the 18 mm gaps close and the slats become one curved band 40 mm thick swept over 0.70 m; bars 30 x 80 mm, posts and cross tubes 60 mm, plates 30 mm, footings unchanged.", "1:200 für 30 mm: die 18-mm-Fugen schließen sich, die Latten werden ein gebogenes Band 40 mm dick über 0,70 m; Holme 30 x 80 mm, Pfosten und Querrohre 60 mm, Platten 30 mm, Fundamente unverändert."),
         T("Thermal: band, bars, posts, cross tubes, plates and footings by region. Structure: bars, posts, plates, cross tubes and footings as members, welds and anchorages 60 mm, mass markers 80 mm.", "Thermik: Band, Holme, Pfosten, Querrohre, Platten und Fundamente nach Region. Tragwerk: Holme, Pfosten, Platten, Querrohre und Fundamente als Stäbe, Nähte und Verankerungen 60 mm, Massenmarken 80 mm.")],
    500: [T("1:500 for 75 mm: slats and stringers become one curved slab 120 mm deep (slat 40 over bar 80) that keeps the true side silhouette; each portal is one H-shaped extrusion of 75 mm posts and cross tube; the bottom 75 mm of each post is the rusted foot; footings unchanged.", "1:500 für 75 mm: Latten und Holme werden eine gebogene Platte 120 mm tief (Latte 40 über Holm 80), die die wahre Seitenansicht behält; jedes Joch ist eine H-förmige Extrusion aus 75-mm-Pfosten und Querrohr; die unteren 75 mm jedes Pfostens sind der rostige Fuß; Fundamente unverändert."),
         T("Thermal: slab (slats), portals (frame), feet, footings. Structure keeps its own load path: the two flat-bar runs per side as 75 mm members inside the slab, portals, feet and footings as members, 75 mm welds and anchorages, mass markers 155 mm.", "Thermik: Platte (Latten), Joche (Gestell), Füße, Fundamente. Das Tragwerk behält seinen eigenen Lastpfad: die zwei Flachstahlteile je Seite als 75-mm-Stäbe in der Platte, Joche, Füße und Fundamente als Stäbe, 75-mm-Nähte und -Verankerungen, Massenmarken 155 mm.")],
}


def hatches(den):
    """▤ The hatches of a scale."""
    out = []
    if den == 50:
        out.append({"id": "grain", "title": T("Robinia grain", "Robinienmaserung"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.6, "penMm": 0.13})
    if den in (50, 100):
        out.append({"id": "concrete", "title": T("Concrete", "Beton"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.8 if den == 50 else 0.6, "penMm": 0.13})
    return out


def document():
    """📄 The whole asset document."""
    reps, vols = {}, {}
    all_points = []
    for den in (50, 100, 200, 500):
        sc, body, th, st, elev, front, plan = build(den)
        reps[str(den)] = {"status": "authored", "denominator": den, "validRange": {"minDenominator": sc.range[0], "maxDenominator": sc.range[1]}, "pens": sc.pens, "hatches": hatches(den), "geometry": {"elevation": elev, "front": front, "plan": plan}, "decisions": REP_DECISIONS[den]}
        vols[str(den)] = {"status": "authored", "denominator": den, "body": body, "thermal": th, "structure": st, "decisions": VOL_DECISIONS[den]}
        DEBUG.append(f"[DEBUG] 1:{den} body {len(body)} thermal {len(th)} structure {len(st)} shapes {len(elev)}/{len(front)}/{len(plan)}")
    top_y = max(max(p[1] for p in sl["corners"]) for sl in SLATS)
    dims = {"heightM": r5(top_y + FOOTING_D), "widthM": LENGTH, "depthM": FOOTING_Z, "massKg": round(MASS_KG, 1)}
    asset = {
        "id": "used-sun-lounger-bench",
        "category": "furniture",
        "title": T("Used Park Sun Lounger", "Gebrauchte Parkliege"),
        "summary": T(
            "A fixed park sun lounger after many summers in a German city park: 24 greyed robinia slats on two bent flat-bar stringers 2.0 m long and 0.7 m wide, head end 0.76 m, lying surface 0.37 m, on two duplex-coated steel portals bolted to strip footings in the lawn. The slats under shoulders and hips are worn smooth and lighter, some are checked and one is split; the coating chalks and flakes, the feet rust, a mower has dented a foot post and the near stringer carries a graffiti tag and sticker residue. The depot lifts the frame out on four slings and leaves the footings; in use an 80 kg sunbather lies in the midday sun on the hot slats.",
            "Eine fest eingebaute Parkliege nach vielen Sommern in einem deutschen Stadtpark: 24 vergraute Robinienlatten auf zwei gebogenen Flachstahlholmen, 2,0 m lang und 0,7 m breit, Kopfende 0,76 m, Liegefläche 0,37 m, auf zwei duplexbeschichteten Stahljochen, die auf Streifenfundamente im Rasen geschraubt sind. Die Latten unter Schultern und Hüften sind glatt und heller abgenutzt, einige gerissen, eine gespalten; die Beschichtung kreidet und blättert, die Füße rosten, ein Mäher hat einen Fußpfosten verbeult und der nahe Holm trägt ein Graffiti-Tag und Aufkleberreste. Der Bauhof hebt das Gestell an vier Hebebändern aus und lässt die Fundamente zurück; in Benutzung liegt eine sonnenbadende Person mit 80 kg in der Mittagssonne auf den heißen Latten."),
        "tags": ["used", "park", "public-space", "seating", "lounger", "sun-lounger", "street-furniture", "timber", "robinia", "steel", "flat-bar", "footing", "weathered"],
        "provenance": {"author": "Semio staffage content (used park objects wave, row 05)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS], "notes": T(
            f"Side elevation along the lying length, head end at x = 0. Product type after catalogue-typical fixed park loungers: lying curve through hand-set control heights (head 0.725, back reclined about 30 degrees, hip 0.33, calf hump 0.357, foot 0.306 m on the stringer top), {SLAT_N} robinia slats 70 x 40 x 700 mm at {SLAT_GAP * 1000:.0f} mm gaps, two flat bars 80 x 12 mm (EN 10058) at z = +-0.30 m, portals of SHS 60 x 60 x 3 (EN 10219-2) at x = 0.50 and 1.55 m, plates 150 x 150 x 12 with two M12 anchors, strip footings C25/30 300 x 800 x 500 mm. Masses from the sections: slats {sum(MASS[b]['slats'] for b in ('stringer-back', 'stringer-leg')):.1f} kg, bars {sum(MASS[b]['bar'] for b in ('stringer-back', 'stringer-leg')):.1f} kg, portals {MASS['leg-head']['kg'] + MASS['leg-foot']['kg']:.1f} kg, total {MASS_KG:.1f} kg; the footings are drawn and not counted. Nothing is measured; dimensions are literature and authored estimates.",
            f"Seitenansicht entlang der Liegelänge, Kopfende bei x = 0. Produkttyp nach katalogtypischen fest eingebauten Parkliegen: Liegekurve durch von Hand gesetzte Stützhöhen (Kopf 0,725, Lehne etwa 30 Grad geneigt, Hüfte 0,33, Wadenbuckel 0,357, Fuß 0,306 m auf der Holmoberkante), {SLAT_N} Robinienlatten 70 x 40 x 700 mm mit {SLAT_GAP * 1000:.0f} mm Fugen, zwei Flachstähle 80 x 12 mm (EN 10058) bei z = +-0,30 m, Joche aus Quadratrohr 60 x 60 x 3 (EN 10219-2) bei x = 0,50 und 1,55 m, Platten 150 x 150 x 12 mit zwei Ankern M12, Streifenfundamente C25/30 300 x 800 x 500 mm. Massen aus den Querschnitten: Latten {sum(MASS[b]['slats'] for b in ('stringer-back', 'stringer-leg')):.1f} kg, Holme {sum(MASS[b]['bar'] for b in ('stringer-back', 'stringer-leg')):.1f} kg, Joche {MASS['leg-head']['kg'] + MASS['leg-foot']['kg']:.1f} kg, gesamt {MASS_KG:.1f} kg; die Fundamente sind gezeichnet und nicht gezählt. Nichts ist gemessen; Maße sind Literatur- und Schätzwerte.")},
        "dimensions": dims,
        "materials": materials(),
        "rig": rig(),
        "volumes": vols,
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal(),
        "structure": structure_model(),
        "presentation": {"defaultPose": "in-service", "defaultClip": "sunbather-lies-down", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    for line in DEBUG:
        print(line)
    print(f"[DEBUG] curve x0 {C.x0:.4f} x1 {C.x1:.4f} arc {C.arc[-1]:.4f} gap {SLAT_GAP * 1000:.1f} mm")
    print(f"[DEBUG] dims {doc['asset']['dimensions']}")
    print("[DEBUG] masses", {k: round(v["kg"], 2) for k, v in MASS.items()}, "total", round(MASS_KG, 2))
    print("[DEBUG] areas", {k: round(v, 4) for k, v in AREAS.items()})
    print(f"[DEBUG] post tops head {post_top(X_HEAD_LEG):.4f} foot {post_top(X_FOOT_LEG):.4f}; back bone {math.degrees(BACK_A):.2f} deg {BACK_LEN:.4f} m; leg bone {math.degrees(LEGB_A):.2f} deg {LEGB_LEN:.4f} m")
