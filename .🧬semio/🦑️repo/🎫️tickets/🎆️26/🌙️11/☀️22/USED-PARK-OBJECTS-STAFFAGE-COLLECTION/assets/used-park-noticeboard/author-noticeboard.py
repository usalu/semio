"""📌 Authoring script of `used-park-noticeboard` (run it to regenerate `🔣️.json`): the hand-chosen numbers of a municipal timber notice board (Schaukasten) on two larch posts with a gable roof, and the helpers that turn each part, a prism along the lateral axis, into its three 2D views and its 3D body, thermal and structure solids per scale.

World rest metres: x ahead (the glazed face looks ahead), y up, z left; origin on the ground midway between the two posts.

@see ../../brief.md — the brief of the wave
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Front and plan views", "Solid placements", "Volume"
"""
import json
import math
import pathlib

T25 = math.tan(math.radians(25.0))
C25 = math.cos(math.radians(25.0))
ZP = 0.81
PH = 0.06
Y_APEX = 2.13
Y_TIMBER0 = 0.060
FOOT = {"x": 0.20, "z": 0.20, "d": 0.80}
SHOE = {"t": 0.008, "w": 0.05, "y0": 0.052, "y1": 0.22}
STEM = {"ro": 0.01685, "ri": 0.01365, "y0": -0.30}
RAFTER = {"x": 0.27, "h": 0.15 / C25, "z0": 0.87, "z1": 0.90}
DECK = {"x": 0.28, "d": 0.031 / C25, "z": 0.96}
CAP = {"x": 0.08, "c": 0.008, "z": 0.96}
CASE = {"x0": -0.045, "x1": 0.045, "y0": 0.95, "y1": 1.95, "z": 0.75, "t": 0.028}
BACK = {"x0": -0.045, "x1": -0.030}
PIN = {"x0": -0.030, "x1": -0.018}
POSTER = {"x0": -0.018, "x1": -0.0175, "yc": 1.45, "h": 0.841, "w": 1.189}
DOOR = {"x0": 0.045, "x1": 0.080, "rail": 0.06, "stile": 0.06}
GLASS = {"x0": 0.0595, "x1": 0.06626, "y0": 1.00, "y1": 1.90, "z": 0.70}
LOCK = {"x1": 0.088, "y0": 0.965, "y1": 0.995, "z": 0.015}
HINGE_R = 0.0045
H = (0.08, 1.95)
P_STRUT = (0.0, 1.55)
D_STRUT = (0.03, 1.80)
Z_STRUT = 0.70
R_STRUT = 0.009
R_ROD = 0.004
L_CYL = (-0.008, 0.20)
L_ROD = 0.25
OPEN_DEG = 75.0
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def ridge(x):
    """🏠 Height of the rafter top line (the underside of the roof deck) at forward position x."""
    return Y_APEX - abs(x) * T25


def rot(p, deg):
    """🔄 Point p turned counter-clockwise by deg about the origin."""
    a = math.radians(deg)
    return (p[0] * math.cos(a) - p[1] * math.sin(a), p[0] * math.sin(a) + p[1] * math.cos(a))


def door_point(p, deg):
    """🚪 Where the door point p (world rest) lies with the door opened by deg about the hinge H."""
    q = rot((p[0] - H[0], p[1] - H[1]), deg)
    return (H[0] + q[0], H[1] + q[1])


def strut_state(deg):
    """🛞 World angle and length of the gas strut from its case pivot to its door pivot with the door opened by deg."""
    d = door_point(D_STRUT, deg)
    return math.degrees(math.atan2(d[1] - P_STRUT[1], d[0] - P_STRUT[0])), math.hypot(d[0] - P_STRUT[0], d[1] - P_STRUT[1])


STRUT0_DEG, STRUT0_L = strut_state(0.0)
ROD0 = STRUT0_L - L_ROD

BONES = {
    "base": {"parent": None, "o": (0.0, 0.0), "t": 0.0, "z": 0.0, "len": 0.0},
    "footing-l": {"parent": "base", "o": (0.0, 0.0), "t": -90.0, "z": ZP, "len": FOOT["d"]},
    "footing-r": {"parent": "base", "o": (0.0, 0.0), "t": -90.0, "z": -ZP, "len": FOOT["d"]},
    "post-l": {"parent": "footing-l", "o": (0.0, 0.0), "t": 90.0, "z": ZP, "len": Y_APEX},
    "post-r": {"parent": "footing-r", "o": (0.0, 0.0), "t": 90.0, "z": -ZP, "len": Y_APEX},
    "case": {"parent": "post-l", "o": (0.0, CASE["y0"]), "t": 90.0, "z": 0.0, "len": CASE["y1"] - CASE["y0"]},
    "door": {"parent": "case", "o": H, "t": -90.0, "z": 0.0, "len": CASE["y1"] - CASE["y0"]},
    "roof": {"parent": "post-l", "o": (0.0, Y_APEX), "t": 0.0, "z": 0.0, "len": DECK["x"]},
    "strut-l": {"parent": "case", "o": P_STRUT, "t": STRUT0_DEG, "z": Z_STRUT, "len": L_CYL[1]},
    "strut-r": {"parent": "case", "o": P_STRUT, "t": STRUT0_DEG, "z": -Z_STRUT, "len": L_CYL[1]},
}
for _side in ("l", "r"):
    _a = math.radians(STRUT0_DEG)
    BONES[f"rod-{_side}"] = {"parent": f"strut-{_side}", "o": (P_STRUT[0] + ROD0 * math.cos(_a), P_STRUT[1] + ROD0 * math.sin(_a)), "t": STRUT0_DEG, "z": BONES[f"strut-{_side}"]["z"], "len": L_ROD}
MOVING = {"door", "strut-l", "strut-r", "rod-l", "rod-r"}


def r5(value):
    """🔢 Rounds a coordinate to 0.01 mm and removes negative zero."""
    out = round(value, 5)
    return 0.0 if out == 0 else out


def local(bone, x, y):
    """📐 World rest point (x, y) to the bone-local (u, v) of `bone`."""
    ox, oy = BONES[bone]["o"]
    t = math.radians(BONES[bone]["t"])
    dx, dy = x - ox, y - oy
    return (r5(dx * math.cos(t) + dy * math.sin(t)), r5(-dx * math.sin(t) + dy * math.cos(t)))


def world(bone, u, v):
    """🌍 Bone-local (u, v) of `bone` to the world rest point (x, y)."""
    ox, oy = BONES[bone]["o"]
    t = math.radians(BONES[bone]["t"])
    return (ox + u * math.cos(t) - v * math.sin(t), oy + u * math.sin(t) + v * math.cos(t))


def tx(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def rect_pts(x0, x1, y0, y1):
    """⬛ The four corners of an axis-aligned world rectangle."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def is_rect(pts):
    """📏 True when the polygon is an axis-aligned rectangle of four corners."""
    if len(pts) != 4:
        return False
    xs = sorted({round(p[0], 6) for p in pts})
    ys = sorted({round(p[1], 6) for p in pts})
    return len(xs) == 2 and len(ys) == 2


def bbox(pts):
    """📦 The bounding box (x0, x1, y0, y1) of points."""
    return (min(p[0] for p in pts), max(p[0] for p in pts), min(p[1] for p in pts), max(p[1] for p in pts))


def area_centroid(pts):
    """⚖️ Shoelace area and centroid of a simple polygon."""
    a = cx = cy = 0.0
    for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1]):
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return abs(a), (cx / (6 * a), cy / (6 * a))


def g_rect(x0, y0, x1, y1):
    """⬜ Rectangle shape geometry from two corners."""
    return {"kind": "rect", "x": r5(min(x0, x1)), "y": r5(min(y0, y1)), "w": r5(abs(x1 - x0)), "h": r5(abs(y1 - y0))}


def g_poly(points, closed=True):
    """🔺 Polygon or polyline shape geometry."""
    return {"kind": "poly", "points": [[r5(x), r5(y)] for x, y in points], "closed": closed}


def g_path(d):
    """〰️ SVG path shape geometry."""
    return {"kind": "path", "d": d}


def g_ellipse(cx, cy, rx, ry):
    """🥚 Ellipse shape geometry."""
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(rx), "ry": r5(ry)}


def shape(sid, bone, role, order, filled, tone, geom, region, material, hatch=None, solid=None, pen=None):
    """✏️ One authored 2D shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": round(order, 3), "filled": filled, "tone": tone}
    out.update(geom)
    out["region"] = region
    out["material"] = material
    if hatch:
        out["hatch"] = hatch
    if pen:
        out["penMm"] = pen
    if solid:
        out["solid"] = solid
    return out


GREY, FRESH, PAINT, PLY, SOFT, PAPER, GLASSM, DECKM, MOSS, ZINC, TIZN, CONC, STRUTM = (
    "larch-weathered", "larch-fresh", "larch-painted", "plywood-painted", "softboard", "poster-paper", "laminated-glass", "roof-deck", "moss", "zinc-weathered", "titanium-zinc", "concrete-weathered", "gas-spring")
DENSITY = {GREY: 600, FRESH: 600, PAINT: 600, PLY: 550, SOFT: 250, PAPER: 1100, GLASSM: 2339, DECKM: 664, MOSS: 300, ZINC: 7850, TIZN: 7200, CONC: 2300, STRUTM: 4500}


class Part:
    """🧩 One physical part: a prism along the lateral axis (world profile `pts` swept from z0 to z1), a tube or rod along a bone, or a pin across it."""

    def __init__(self, pid, bone, region, material, tone, pts=None, z0=0.0, z1=0.0, role="body", order=1.0, sheet=False, slender=False, hatch=None, cyl=None, pin=None, envelope=False, hole=None):
        self.hole = hole
        self.id, self.bone, self.region, self.material, self.tone = pid, bone, region, material, tone
        self.pts, self.z0, self.z1, self.role, self.order = pts, z0, z1, role, order
        self.sheet, self.slender, self.hatch, self.cyl, self.pin, self.envelope = sheet, slender, hatch, cyl, pin, envelope

    def flags(self):
        """🚩 The declarations of the solid."""
        out = {}
        if self.sheet:
            out["sheet"] = True
        if self.slender:
            out["slender"] = True
        if self.envelope:
            out["envelope"] = True
        return out

    def volume_com(self):
        """⚖️ Volume (m3) and world rest centre (x, y, z) of the part."""
        if self.cyl:
            a0, a1, ro, ri = self.cyl
            u = (a0 + a1) / 2
            x, y = world(self.bone, u, 0.0)
            return math.pi * (ro * ro - ri * ri) * (a1 - a0), (x, y, BONES[self.bone]["z"])
        if self.pin:
            u, v, r, w0, w1 = self.pin
            x, y = world(self.bone, u, v)
            return math.pi * r * r * (w1 - w0), (x, y, BONES[self.bone]["z"] + (w0 + w1) / 2)
        a, (cx, cy) = area_centroid(self.pts)
        if self.hole:
            x0, x1, y0, y1 = bbox(self.pts)
            hz, hy0, hy1 = self.hole
            return ((self.z1 - self.z0) * (y1 - y0) - 2 * hz * (hy1 - hy0)) * (x1 - x0), (cx, cy, (self.z0 + self.z1) / 2)
        return a * (self.z1 - self.z0), (cx, cy, (self.z0 + self.z1) / 2)


def depth_orders(p):
    """🎚 Painting orders of part p in the side elevation (nearer the viewer on the right, smaller z, paints later) and the front elevation (further ahead paints later)."""
    bz = BONES[p.bone]["z"]
    if p.cyl:
        z0 = bz - p.cyl[2]
        xmax = max(world(p.bone, p.cyl[0], 0)[0], world(p.bone, p.cyl[1], 0)[0]) + p.cyl[2]
    elif p.pin:
        z0 = bz + p.pin[3]
        xmax = world(p.bone, p.pin[0], p.pin[1])[0] + p.pin[2]
    else:
        z0, xmax = p.z0, max(x for x, _ in p.pts)
    return round(10 - z0, 4), round(10 + xmax, 4)


def emit(k, p):
    """🖨️ The three 2D shapes and the body, thermal and structure solids of part p at scale k."""
    b = p.bone
    bz = BONES[b]["z"]
    moving = b in MOVING
    oe, of_ = depth_orders(p)
    n = {50: 48, 100: 36, 200: 24, 500: 16}[k.s]
    if p.cyl:
        a0, a1, ro, ri = p.cyl
        prof = [(a0, ri), (a1, ri), (a1, ro), (a0, ro), (a0, ri)] if ri > 0 else [(a0, 0), (a0, ro), (a1, ro), (a1, 0)]
        k.body.append({"id": p.id, "kind": "revolve", "bone": b, "material": p.material, "role": p.role, "tone": p.tone, "profile": [[r5(x), r5(y)] for x, y in prof], "axis": [1, 0, 0], "origin": [0, 0, 0], **p.flags()})
        el = g_rect(a0, -ro, a1, ro)
        k.elev.append(shape(p.id, b, p.role, oe, True, p.tone, el, p.region, p.material))
        if moving:
            sol = {"kind": "extruded", "sweep": "w", "z0": r5(bz - ro), "z1": r5(bz + ro)}
            k.front.append(shape(p.id, b, p.role, of_, True, p.tone, el, p.region, p.material, solid=sol))
            k.plan.append(shape(p.id, b, p.role, p.order, True, p.tone, el, p.region, p.material, solid=sol))
        else:
            (x0, y0), (x1, y1) = world(b, a0, 0), world(b, a1, 0)
            k.front.append(shape(p.id, b, p.role, of_, True, p.tone, g_rect(bz - ro, min(y0, y1), bz + ro, max(y0, y1)), p.region, p.material))
            k.plan.append(shape(p.id, b, p.role, p.order, True, p.tone, g_ellipse(x0, bz, ro, ro), p.region, p.material))
    elif p.pin:
        u, v, r, w0, w1 = p.pin
        k.body.append({"id": p.id, "kind": "revolve", "bone": b, "material": p.material, "role": p.role, "tone": p.tone, "profile": [[r5(w0), 0], [r5(w0), r5(r)], [r5(w1), r5(r)], [r5(w1), 0]], "axis": [0, 0, 1], "origin": [r5(u), r5(v), 0], **p.flags()})
        x, y = world(b, u, v)
        k.elev.append(shape(p.id, b, p.role, oe, True, p.tone, g_ellipse(u, v, r, r), p.region, p.material))
        k.front.append(shape(p.id, b, p.role, of_, True, p.tone, g_rect(bz + w0, y - r, bz + w1, y + r), p.region, p.material))
        k.plan.append(shape(p.id, b, p.role, p.order, True, p.tone, g_rect(x - r, bz + w0, x + r, bz + w1), p.region, p.material))
    elif p.hole:
        x0, x1, y0, y1 = bbox(p.pts)
        hz, hy0, hy1 = p.hole
        wu = lambda z, y: [r5(z - bz), local(b, x0, y)[0]]
        prof = [wu(p.z0, y0), wu(p.z1, y0), wu(p.z1, y1), wu(p.z0, y1)]
        hole = [wu(-hz, hy0), wu(hz, hy0), wu(hz, hy1), wu(-hz, hy1)]
        va, vb = sorted((local(b, x0, y0)[1], local(b, x1, y0)[1]))
        k.body.append({"id": p.id, "kind": "extrude", "bone": b, "material": p.material, "role": p.role, "tone": p.tone, "profile": prof, "holes": [hole], "plane": "zx", "from": r5(va), "to": r5(vb), **p.flags()})
        lp = [local(b, x, y) for x, y in p.pts]
        u0, u1, v0, v1 = bbox(lp)
        el = g_rect(u0, v0, u1, v1)
        k.elev.append(shape(p.id, b, p.role, oe, True, p.tone, el, p.region, p.material))
        if moving:
            for mid, (ya, yb, za, zb) in (("top", (hy1, y1, -hz, hz)), ("bottom", (y0, hy0, -hz, hz)), ("l", (y0, y1, hz, p.z1)), ("r", (y0, y1, p.z0, -hz))):
                ua, va_ = local(b, x0, ya)
                ub, vb_ = local(b, x1, yb)
                sol = {"kind": "extruded", "sweep": "w", "z0": r5(za), "z1": r5(zb)}
                k.front.append(shape(f"{p.id}-{mid}", b, p.role, of_, True, p.tone, g_rect(ua, va_, ub, vb_), p.region, p.material, solid=sol))
            k.plan.append(shape(p.id, b, p.role, p.order, True, p.tone, el, p.region, p.material, solid={"kind": "extruded", "sweep": "w", "z0": r5(p.z0), "z1": r5(p.z1)}))
        else:
            d = f"M {r5(p.z0)} {r5(y0)} L {r5(p.z1)} {r5(y0)} L {r5(p.z1)} {r5(y1)} L {r5(p.z0)} {r5(y1)} Z M {r5(-hz)} {r5(hy0)} L {r5(hz)} {r5(hy0)} L {r5(hz)} {r5(hy1)} L {r5(-hz)} {r5(hy1)} Z"
            k.front.append(shape(p.id, b, p.role, of_, True, p.tone, g_path(d), p.region, p.material))
            k.plan.append(shape(p.id, b, p.role, p.order, True, p.tone, g_rect(x0, p.z0, x1, p.z1), p.region, p.material))
    else:
        lp = [local(b, x, y) for x, y in p.pts]
        if is_rect(p.pts):
            u0, u1, v0, v1 = bbox(lp)
            k.body.append({"id": p.id, "kind": "box", "bone": b, "material": p.material, "role": p.role, "tone": p.tone, "size": [r5(u1 - u0), r5(v1 - v0), r5(p.z1 - p.z0)], "transform": {"translation": [r5((u0 + u1) / 2), r5((v0 + v1) / 2), r5((p.z0 + p.z1) / 2 - bz)]}, **p.flags()})
            el = g_rect(u0, v0, u1, v1)
        else:
            k.body.append({"id": p.id, "kind": "extrude", "bone": b, "material": p.material, "role": p.role, "tone": p.tone, "profile": [[r5(u), r5(v)] for u, v in lp], "plane": "xy", "from": r5(p.z0 - bz), "to": r5(p.z1 - bz), **p.flags()})
            el = g_poly(lp)
        k.elev.append(shape(p.id, b, p.role, oe, True, p.tone, el, p.region, p.material, hatch=p.hatch))
        if moving:
            sol = {"kind": "extruded", "sweep": "w", "z0": r5(p.z0), "z1": r5(p.z1)}
            k.front.append(shape(p.id, b, p.role, of_, True, p.tone, el, p.region, p.material, solid=sol))
            k.plan.append(shape(p.id, b, p.role, p.order, True, p.tone, el, p.region, p.material, solid=sol))
        else:
            x0, x1, y0, y1 = bbox(p.pts)
            k.front.append(shape(p.id, b, p.role, of_, True, p.tone, g_rect(p.z0, y0, p.z1, y1), p.region, p.material, hatch=p.hatch))
            k.plan.append(shape(p.id, b, p.role, p.order, True, p.tone, g_rect(x0, p.z0, x1, p.z1), p.region, p.material))
    k.thermal.append({"id": "th-" + p.id, "of": p.id, "region": p.region})
    k.structure.append({"id": "st-" + p.id, "of": p.id, "member": b, "part": "member"})


def chevron(x, top, depth):
    """⛰️ A gable chevron profile from -x to x: top line `top(x)` and the same line `depth` lower, plumb ends."""
    return [(-x, top(x) - depth), (-x, top(x)), (0.0, top(0.0)), (x, top(x)), (x, top(x) - depth), (0.0, top(0.0) - depth)]


def deck_top(x):
    """🏠 Height of the top of the roof deck (felt surface) at forward position x."""
    return ridge(x) + DECK["d"]


def parts(s):
    """🧱 The hand-chosen parts of scale 1:s; each scale is its own model for its smallest feature."""
    f = FEATURE[s]
    out = []
    for side, sg, post_region, post_mat, post_tone in (("l", 1, "timber-grey", GREY, 0.55), ("r", -1, "post-fresh", FRESH, 0.78)):
        zc = sg * ZP
        conc_hatch = "concrete-stipple" if s <= 100 else None
        out.append(Part(f"footing-{side}", f"footing-{side}", "footing-concrete", CONC, 0.62, rect_pts(-FOOT["x"], FOOT["x"], -FOOT["d"], 0.0), zc - FOOT["z"], zc + FOOT["z"], order=1, hatch=conc_hatch))
        pb = f"post-{side}"
        y_timber = Y_TIMBER0 if s < 500 else 0.075
        out.append(Part(pb, pb, post_region, post_mat, post_tone, [(-PH, y_timber), (PH, y_timber), (PH, ridge(PH)), (0.0, Y_APEX), (-PH, ridge(PH))], zc - PH, zc + PH, order=3, hatch="timber-grain" if s == 50 else None))
        if s == 50:
            out.append(Part(f"stem-{side}", pb, "steel-zinc", ZINC, 0.45, cyl=(STEM["y0"], SHOE["y0"], STEM["ro"], STEM["ri"]), order=2))
            out.append(Part(f"shoe-plate-{side}", pb, "steel-zinc", ZINC, 0.45, rect_pts(-PH - SHOE["t"], PH + SHOE["t"], SHOE["y0"], Y_TIMBER0), zc - SHOE["w"], zc + SHOE["w"], order=2))
            for leg, x0 in (("front", PH), ("back", -PH - SHOE["t"])):
                out.append(Part(f"shoe-{leg}-{side}", pb, "steel-zinc", ZINC, 0.45, rect_pts(x0, x0 + SHOE["t"], Y_TIMBER0, SHOE["y1"]), zc - SHOE["w"], zc + SHOE["w"], order=3.5))
        elif s < 500:
            out.append(Part(f"stem-{side}", pb, "steel-zinc", ZINC, 0.45, cyl=(STEM["y0"], SHOE["y0"], STEM["ro"], STEM["ri"] if s == 100 else 0.0), order=2))
            out.append(Part(f"shoe-{side}", pb, "steel-zinc", ZINC, 0.45, rect_pts(-PH - SHOE["t"], PH + SHOE["t"], SHOE["y0"], SHOE["y1"]), zc - SHOE["w"], zc + SHOE["w"], order=3.5))
        else:
            out.append(Part(f"shoe-{side}", pb, "steel-zinc", ZINC, 0.45, rect_pts(-PH, PH, 0.0, 0.075), zc - PH, zc + PH, order=2))
        rz0, rz1 = (RAFTER["z0"], RAFTER["z1"]) if s < 500 else (RAFTER["z0"], RAFTER["z0"] + f)
        out.append(Part(f"rafter-{side}", pb, "timber-grey", GREY, 0.5, chevron(RAFTER["x"], ridge, RAFTER["h"]), *(sorted((sg * rz0, sg * rz1))), order=5, hatch="timber-grain" if s == 50 else None))

    cz, ct = CASE["z"], CASE["t"]
    if s <= 100:
        out.append(Part("case-top", "case", "case-paint", PAINT, 0.36, rect_pts(CASE["x0"], CASE["x1"], CASE["y1"] - ct, CASE["y1"]), -cz, cz, order=4))
        out.append(Part("case-bottom", "case", "case-paint", PAINT, 0.36, rect_pts(CASE["x0"], CASE["x1"], CASE["y0"], CASE["y0"] + ct), -cz, cz, order=4))
        for side, sg in (("l", 1), ("r", -1)):
            out.append(Part(f"case-side-{side}", "case", "case-paint", PAINT, 0.36, rect_pts(CASE["x0"], CASE["x1"], CASE["y0"] + ct, CASE["y1"] - ct), *(sorted((sg * (cz - ct), sg * cz))), order=4))
    else:
        out.append(Part("case-ring", "case", "case-paint", PAINT, 0.36, rect_pts(CASE["x0"], CASE["x1"], CASE["y0"], CASE["y1"]), -cz, cz, order=4, hole=(cz - ct, CASE["y0"] + ct, CASE["y1"] - ct)))
    iy0, iy1, iz = CASE["y0"] + ct, CASE["y1"] - ct, cz - ct
    out.append(Part("back-panel", "case", "case-paint", PLY, 0.4, rect_pts(BACK["x0"], BACK["x1"], iy0, iy1), -iz, iz, order=4.1, sheet=s >= 200))
    out.append(Part("pin-board", "case", "notice-face", SOFT, 0.7, rect_pts(PIN["x0"], PIN["x1"], iy0, iy1), -iz, iz, order=4.2, sheet=s >= 100))
    if s == 50:
        out.append(Part("park-map", "case", "notice-face", PAPER, 0.84, rect_pts(POSTER["x0"], POSTER["x1"], POSTER["yc"] - POSTER["h"] / 2, POSTER["yc"] + POSTER["h"] / 2), -POSTER["w"] / 2, POSTER["w"] / 2, order=4.3, sheet=True))
    if s <= 100:
        out.append(Part("hinge", "case", "steel-zinc", ZINC, 0.4, pin=(*local("case", *H), HINGE_R, -cz, cz), order=6.6, slender=True))
        for side, sg in (("l", 1), ("r", -1)):
            bw = 0.013 if s == 50 else 0.015
            out.append(Part(f"strut-mount-{side}", "case", "steel-zinc", ZINC, 0.4, rect_pts(P_STRUT[0] - 0.01, P_STRUT[0] + 0.01, P_STRUT[1] - 0.01, P_STRUT[1] + 0.01), *(sorted((sg * (Z_STRUT + R_STRUT), sg * (Z_STRUT + R_STRUT + bw)))), order=4.5))

    dx0, dx1 = DOOR["x0"], DOOR["x1"] if s < 500 else 0.12
    rl, st = (DOOR["rail"], DOOR["stile"]) if s < 500 else (0.10, 0.10)
    if s < 500:
        out.append(Part("door-top-rail", "door", "case-paint", PAINT, 0.34, rect_pts(dx0, dx1, CASE["y1"] - rl, CASE["y1"]), -(cz - st), cz - st, order=6))
        out.append(Part("door-bottom-rail", "door", "case-paint", PAINT, 0.34, rect_pts(dx0, dx1, CASE["y0"], CASE["y0"] + rl), -(cz - st), cz - st, order=6))
        for side, sg in (("l", 1), ("r", -1)):
            out.append(Part(f"door-stile-{side}", "door", "case-paint", PAINT, 0.34, rect_pts(dx0, dx1, CASE["y0"], CASE["y1"]), *(sorted((sg * (cz - st), sg * cz))), order=6))
    else:
        out.append(Part("door-frame", "door", "case-paint", PAINT, 0.34, rect_pts(dx0, dx1, CASE["y0"], CASE["y1"]), -cz, cz, order=6, hole=(cz - st, CASE["y0"] + rl, CASE["y1"] - rl)))
    gx0, gx1 = (GLASS["x0"], GLASS["x1"]) if s < 500 else (0.0825, 0.08926)
    out.append(Part("glazing", "door", "glazing", GLASSM, 0.86, rect_pts(gx0, gx1, GLASS["y0"], GLASS["y1"]), -GLASS["z"], GLASS["z"], role="glazing", order=5, sheet=True))
    if s == 50:
        out.append(Part("lock", "door", "steel-zinc", ZINC, 0.25, rect_pts(DOOR["x1"], LOCK["x1"], LOCK["y0"], LOCK["y1"]), -LOCK["z"], LOCK["z"], order=6.5))
    if s <= 100:
        for side, sg in (("l", 1), ("r", -1)):
            out.append(Part(f"door-mount-{side}", "door", "steel-zinc", ZINC, 0.4, rect_pts(D_STRUT[0], DOOR["x0"], D_STRUT[1] - 0.01, D_STRUT[1] + 0.01), *(sorted((sg * (Z_STRUT - R_STRUT), sg * (Z_STRUT + R_STRUT)))), order=6.2))
            out.append(Part(f"strut-body-{side}", f"strut-{side}", "steel-zinc", STRUTM, 0.3, cyl=(L_CYL[0], L_CYL[1], R_STRUT, 0.0), order=4.6))
            out.append(Part(f"strut-rod-{side}", f"rod-{side}", "steel-zinc", STRUTM, 0.5, cyl=(0.0, L_ROD, R_ROD, 0.0), order=4.55, slender=True))

    dd = DECK["d"] if s < 500 else 0.075 / C25
    out.append(Part("roof-deck", "roof", "roof-felt", DECKM, 0.24, chevron(DECK["x"], deck_top, dd), -DECK["z"], DECK["z"], order=7))
    if s <= 100:
        cc = CAP["c"] if s == 50 else f
        out.append(Part("ridge-cap", "roof", "steel-zinc", TIZN, 0.5, chevron(CAP["x"], lambda x: deck_top(x) + cc, cc), -CAP["z"], CAP["z"], order=7.2))
    if s == 50:
        bumps = [0.008, 0.013, 0.010, 0.016, 0.011, 0.014, 0.009]
        xs = [-0.12 - 0.15 * i / (len(bumps) - 1) for i in range(len(bumps))]
        moss = [(x, deck_top(x) - 0.002) for x in xs] + [(x, deck_top(x) + h) for x, h in reversed(list(zip(xs, bumps)))]
        out.append(Part("moss-a", "roof", "roof-felt", MOSS, 0.46, moss, -0.92, -0.35, order=7.3))
        out.append(Part("moss-b", "roof", "roof-felt", MOSS, 0.46, moss, 0.05, 0.62, order=7.3))
    return out


class Scale:
    """📏 Collects the 2D shapes and 3D solids of one authored scale."""

    def __init__(self, s):
        self.s = s
        self.elev, self.front, self.plan = [], [], []
        self.body, self.thermal, self.structure = [], [], []


def build(s):
    """🏗️ All shapes and solids of scale 1:s."""
    k = Scale(s)
    for p in parts(s):
        emit(k, p)
    return k


def masses():
    """⚖️ Bone masses from volume x density of the 1:50 parts (kg) with their world centres; the struts are lumped into the case that holds them."""
    acc = {}
    for p in parts(50):
        vol, (x, y, z) = p.volume_com()
        m = vol * DENSITY[p.material]
        bone = "case" if p.bone.startswith(("strut-", "rod-")) else p.bone
        acc.setdefault(bone, []).append((m, x, y, z, p.id))
    total = {}
    for bone, items in acc.items():
        m = sum(i[0] for i in items)
        total[bone] = (m, sum(i[0] * i[1] for i in items) / m, sum(i[0] * i[2] for i in items) / m, sum(i[0] * i[3] for i in items) / m)
    return acc, total


def tag_line(path_d):
    """🖊️ Normalises an SVG path string."""
    return " ".join(path_d.split())


def traces(k):
    """🩹 Traces of use and weather as detail and texture strokes: the spider crack in the laminated glass, the faded map print, flaking paint, drying checks and a tag on the old post, sticker residue, rust runs at the shoes, strimmer scars, felt laps and moss on the shaded rear slope."""
    s = k.s
    zl = ZP
    if s <= 100:
        cz, cy = -0.40, 1.22
        rays = [(-0.62, 1.55), (-0.18, 1.62), (0.05, 1.30), (-0.05, 1.05), (-0.55, 1.03), (-0.66, 1.20)] if s == 50 else [(-0.62, 1.55), (0.05, 1.30), (-0.55, 1.03)]
        d = " ".join(f"M {cz} {cy} L {r5((cz + x) / 2 + 0.02)} {r5((cy + y) / 2 - 0.015)} L {x} {y}" for x, y in rays)
        if s == 50:
            d += f" M {cz - 0.06} {cy} A 0.06 0.05 0 1 1 {cz + 0.06} {cy} M {cz - 0.13} {cy + 0.02} A 0.13 0.11 0 0 1 {cz + 0.12} {cy + 0.04}"
        k.front.append(shape("glass-crack", "door", "texture", 10.1, False, 0.2, g_path(tag_line(d)), "glazing", GLASSM, pen=0.13 if s == 50 else 0.1))
        my0, my1 = POSTER["yc"] - POSTER["h"] / 2, POSTER["yc"] + POSTER["h"] / 2
        mw = POSTER["w"] / 2
        if s == 50:
            dm = (f"M {-mw + 0.03} {my1 - 0.09} L {mw - 0.03} {my1 - 0.09} "
                  f"M {-mw + 0.05} {my0 + 0.12} C -0.3 1.30 -0.1 1.62 0.18 1.48 C 0.35 1.40 0.42 1.22 {mw - 0.05} 1.18 "
                  f"M -0.42 1.62 C -0.28 1.55 -0.25 1.38 -0.36 1.22 "
                  f"M 0.12 1.20 C 0.20 1.17 0.30 1.21 0.26 1.28 C 0.22 1.34 0.10 1.30 0.12 1.20 Z "
                  f"M {mw - 0.30} {my0 + 0.04} L {mw - 0.04} {my0 + 0.04} L {mw - 0.04} {my0 + 0.20} L {mw - 0.30} {my0 + 0.20} Z")
            k.front.append(shape("map-print", "case", "texture", 9.99, False, 0.62, g_path(tag_line(dm)), "notice-face", PAPER))
            dp = " ".join(f"M {z0} {y0} L {z0 + 0.03} {y0 - 0.012} L {z0 + 0.055} {y0 + 0.006} L {z0 + 0.02} {y0 + 0.018} Z" for z0, y0 in ((0.52, 0.975), (-0.10, 0.968), (0.71, 1.42), (-0.735, 1.70), (0.30, 1.925)))
            k.front.append(shape("paint-flakes", "door", "detail", 10.09, True, 0.6, g_path(dp), "case-paint", PAINT))
            k.front.append(shape("checks", "post-l", "detail", 10.061, False, 0.3, g_path(f"M {zl - 0.02} 0.45 C {zl - 0.025} 0.8 {zl - 0.015} 1.2 {zl - 0.022} 1.62 M {zl + 0.03} 1.05 C {zl + 0.026} 1.3 {zl + 0.034} 1.5 {zl + 0.03} 1.8"), "timber-grey", GREY))
            k.front.append(shape("tag", "post-l", "texture", 10.062, False, 0.15, g_path(f"M {zl - 0.045} 0.62 C {zl - 0.01} 0.70 {zl + 0.02} 0.55 {zl + 0.045} 0.66 M {zl - 0.04} 0.74 C {zl} 0.80 {zl + 0.03} 0.70 {zl + 0.04} 0.79 M {zl - 0.03} 0.57 L {zl + 0.035} 0.86"), "timber-grey", GREY))
            k.front.append(shape("sticker-residue", "post-l", "texture", 10.062, False, 0.7, g_path(f"M {zl - 0.035} 1.30 L {zl + 0.03} 1.30 L {zl + 0.03} 1.37 L {zl - 0.035} 1.37 Z M {zl - 0.02} 1.40 L {zl + 0.015} 1.42 L {zl + 0.005} 1.45"), "timber-grey", GREY))
            for side, zc in (("l", ZP), ("r", -ZP)):
                k.front.append(shape(f"shoe-bolts-{side}", f"post-{side}", "detail", 10.07, False, 0.2, g_path(f"M {zc + 0.01} 0.12 A 0.01 0.01 0 1 1 {zc - 0.01} 0.12 A 0.01 0.01 0 1 1 {zc + 0.01} 0.12 M {zc + 0.01} 0.18 A 0.01 0.01 0 1 1 {zc - 0.01} 0.18 A 0.01 0.01 0 1 1 {zc + 0.01} 0.18"), "steel-zinc", ZINC))
                k.front.append(shape(f"rust-runs-{side}", f"post-{side}", "texture", 10.071, False, 0.3, g_path(f"M {zc - 0.003} 0.11 L {zc - 0.004} 0.075 M {zc + 0.004} 0.17 L {zc + 0.005} 0.135"), "steel-zinc", ZINC))
                k.front.append(shape(f"strimmer-scars-{side}", f"post-{side}", "texture", 10.061, False, 0.35, g_path(f"M {zc - 0.055} 0.25 L {zc - 0.02} 0.27 M {zc + 0.01} 0.24 L {zc + 0.05} 0.26 M {zc - 0.04} 0.30 L {zc - 0.005} 0.31"), "post-fresh" if side == "r" else "timber-grey", FRESH if side == "r" else GREY))
            k.front.append(shape("felt-laps", "roof", "texture", 10.29, False, 0.2, g_path(f"M {-DECK['z'] + 0.01} {r5(deck_top(0.18))} L {DECK['z'] - 0.01} {r5(deck_top(0.18))}"), "roof-felt", DECKM))
            k.elev.append(shape("rafter-bolts", "post-r", "detail", 10.91, False, 0.2, g_path(" ".join(f"M {y + 0.012} {0.0} A 0.012 0.012 0 1 1 {y - 0.012} 0.0 A 0.012 0.012 0 1 1 {y + 0.012} 0.0" for y in (1.93, 2.03))), "timber-grey", GREY))
            k.elev.append(shape("strimmer-scars", "post-r", "texture", 10.88, False, 0.35, g_path("M 0.24 0.05 L 0.26 0.02 M 0.25 -0.01 L 0.27 -0.045 M 0.29 0.03 L 0.30 0.0"), "post-fresh", FRESH))
            k.elev.append(shape("drip-line", "roof", "detail", 10.97, False, 0.3, g_poly([local("roof", DECK["x"] - 0.004, deck_top(DECK["x"]) - DECK["d"] + 0.004), local("roof", DECK["x"] - 0.004, deck_top(DECK["x"]) - 0.004)], False), "roof-felt", DECKM))
            k.elev.append(shape("lock-cylinder", "door", "detail", 10.76, False, 0.2, g_poly([local("door", 0.084, 0.972), local("door", 0.084, 0.988)], False), "steel-zinc", ZINC))
            k.plan.append(shape("felt-laps", "roof", "texture", 7.5, False, 0.2, g_path(f"M 0.18 {-DECK['z'] + 0.01} L 0.18 {DECK['z'] - 0.01} M -0.18 {-DECK['z'] + 0.01} L -0.18 {DECK['z'] - 0.01}"), "roof-felt", DECKM))
            k.plan.append(shape("algae", "roof", "texture", 7.6, False, 0.4, g_path("M -0.27 -0.30 L -0.20 -0.22 M -0.26 0.68 L -0.19 0.75 M -0.25 -0.05 L -0.17 0.02"), "roof-felt", DECKM))
            k.plan.append(shape("footing-cracks-l", "footing-l", "texture", 1.1, False, 0.4, g_path(f"M 0.12 {ZP + 0.15} L 0.16 {ZP + 0.10} L 0.18 {ZP + 0.12}"), "footing-concrete", CONC))
        else:
            k.front.append(shape("map-print", "case", "texture", 9.99, False, 0.62, g_path(f"M {-mw} {my0} L {mw} {my0} L {mw} {my1} L {-mw} {my1} Z M {-mw + 0.05} {my0 + 0.12} C -0.3 1.30 -0.1 1.62 0.18 1.48 C 0.35 1.40 0.42 1.22 {mw - 0.05} 1.18"), "notice-face", SOFT))
            k.front.append(shape("checks", "post-l", "detail", 10.061, False, 0.3, g_path(f"M {zl - 0.02} 0.45 C {zl - 0.025} 0.8 {zl - 0.015} 1.2 {zl - 0.022} 1.62"), "timber-grey", GREY))
            k.front.append(shape("tag", "post-l", "texture", 10.062, False, 0.15, g_path(f"M {zl - 0.045} 0.62 C {zl - 0.01} 0.70 {zl + 0.02} 0.55 {zl + 0.045} 0.66 M {zl - 0.03} 0.57 L {zl + 0.035} 0.86"), "timber-grey", GREY))
            k.elev.append(shape("moss-edge", "roof", "detail", 10.97, False, 0.4, g_poly([local("roof", x, deck_top(x) - 0.006) for x in (-0.12, -0.195, -0.27)], False), "roof-felt", DECKM))
            k.plan.append(shape("moss", "roof", "texture", 7.6, False, 0.45, g_path("M -0.12 -0.92 L -0.27 -0.92 M -0.12 -0.35 L -0.27 -0.35 M -0.12 0.05 L -0.27 0.05 M -0.12 0.62 L -0.27 0.62 M -0.2 -0.9 L -0.2 -0.37 M -0.2 0.07 L -0.2 0.6"), "roof-felt", DECKM))
    elif s == 200:
        my0, my1 = POSTER["yc"] - POSTER["h"] / 2, POSTER["yc"] + POSTER["h"] / 2
        mw = POSTER["w"] / 2
        k.front.append(shape("map-outline", "case", "detail", 9.99, False, 0.62, g_path(f"M {-mw} {my0} L {mw} {my0} L {mw} {my1} L {-mw} {my1} Z"), "notice-face", SOFT))


PENS = {
    50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7},
    100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5},
    200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35},
    500: {"body": 0.18, "detail": None, "texture": None, "glazing": 0.13, "ground": 0.25},
}
BANDS = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

REFERENCES = [
    ("street-furniture-catalogue", "Catalogue-typical data of German municipal park and street-furniture suppliers, 1990-2024, for timber notice boards (Schaukasten) on two posts with a gable roof: case 1200-1800 x 900-1200 x 100-150 mm for an A0 landscape poster, top-hinged glazed door on two gas springs with a cylinder lock, larch or pine posts 100-140 mm square on U-shaped post anchors set in concrete point footings, case bottom edge 0.9-1.0 m above ground; a product type, no brand."),
    ("din-32975", "DIN 32975:2009-12 Gestaltung visueller Informationen im oeffentlichen Raum zur barrierefreien Nutzung (eye heights 1.60 m standing and 1.20 m seated, reading distance and contrast of public information)."),
    ("din-18040-3", "DIN 18040-3:2014-12 Barrierefreies Bauen, Planungsgrundlagen, Teil 3: Oeffentlicher Verkehrs- und Freiraum (information elements in public open space, heights of 0.85 to 1.60 m for readable content)."),
    ("din-68800-2", "DIN 68800-2:2022-02 Holzschutz, Teil 2: Vorbeugende bauliche Massnahmen im Hochbau (constructive wood protection: posts kept clear of the ground on steel post anchors, roof overhang over the case)."),
    ("en-338", "EN 338:2016 Structural timber, strength classes (C24: bending strength 24 MPa, mean modulus 11 GPa; European larch graded to C24 after EN 1912)."),
    ("en-350", "EN 350:2016 Durability of wood and wood-based products (European larch, durability class 3-4, mean density about 590 kg/m3 at 12 % moisture)."),
    ("en-636", "EN 636:2012+A1:2015 Plywood, specifications (EN 636-3 plywood for exterior use, the back panel of the case)."),
    ("en-572-1", "EN 572-1:2012+A1:2016 and EN ISO 12543-2:2021 Glass in building: soda-lime float glass (2500 kg/m3) and laminated safety glass with a PVB interlayer, whose fragments stay bonded after fracture."),
    ("en-13707", "EN 13707:2013 Flexible sheets for waterproofing, reinforced bitumen sheets for roof waterproofing (sanded bitumen felt about 4 mm, 4.5 kg/m2)."),
    ("en-988", "EN 988:1996 Zinc and zinc alloys, specification for rolled flat products for building (titanium zinc 0.7 mm ridge capping)."),
    ("en-iso-1461", "EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles (post anchors and fittings; white rust where water stands)."),
    ("din-en-206", "DIN EN 206:2021-06 with DIN 1045-2:2023 Concrete (C20/25 point footings of street furniture, exposure XC2/XF1, founded frost-free at 0.80 m)."),
    ("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1, Part 1-1: densities, self-weight and imposed loads (timber, steel, concrete, glass)."),
    ("en-1991-1-4", "DIN EN 1991-1-4:2010-12 with DIN EN 1991-1-4/NA:2010-12 Eurocode 1: Wind actions (zone 2 inland, peak velocity pressure 0.65 kN/m2 up to 10 m; force coefficient 1.80 of a signboard after 7.4.3, 2.1 of a square post after 7.6)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products, hygrothermal properties: design values of conductivity, specific heat and density of softwood, plywood, fibreboard, glass, bitumen, steel and concrete."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of wood, paint, glass, roofing, galvanised steel, concrete and paper)."),
    ("duffie-beckman-2013", "Duffie, J. A., Beckman, W. A. (2013). Solar Engineering of Thermal Processes, 4th ed., Wiley (solar absorptance of paints, weathered galvanised steel and concrete; transmittance and absorptance of glass)."),
    ("feist-hon-1984", "Feist, W. C., Hon, D. N.-S. (1984). Chemistry of weathering and protection. In: Rowell, R. M. (ed.), The Chemistry of Solid Wood, Advances in Chemistry 207, American Chemical Society, 401-451 (the silver-grey weathered layer, checking and erosion of exposed softwood)."),
]


def materials():
    """🧱 The materials, weathered surfaces with their own optical data."""
    m = lambda mid, en, de, eps, alpha, rho, lam, cp, e=None, fs=None, src="literature", ref="iso-10456-2007": {k: v for k, v in {"id": mid, "title": tx(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": lam, "specificHeatJKgK": cp, "elasticModulusPa": e, "strengthPa": fs, "source": src, "reference": ref}.items() if v is not None}
    return [
        m(GREY, "European larch, weathered silver-grey and checked", "Europäische Lärche, silbergrau verwittert und rissig", 0.9, 0.7, 600, 0.13, 1600, 1.1e10, 2.4e7, "authored-estimate", "feist-hon-1984"),
        m(FRESH, "European larch, fresh planed (replaced post)", "Europäische Lärche, frisch gehobelt (ersetzter Pfosten)", 0.9, 0.55, 600, 0.13, 1600, 1.1e10, 2.4e7, "authored-estimate", "en-350"),
        m(PAINT, "Larch with dark green opaque paint, chalked and flaking", "Lärche mit dunkelgrünem Deckanstrich, kreidend und abblätternd", 0.9, 0.86, 600, 0.13, 1600, 1.1e10, 2.4e7, "literature", "duffie-beckman-2013"),
        m(PLY, "Exterior plywood 15 mm, painted", "Außensperrholz 15 mm, gestrichen", 0.9, 0.86, 550, 0.13, 1600, 8.0e9, 2.0e7, "literature", "iso-10456-2007"),
        m(SOFT, "Wood fibre softboard pin board", "Holzfaser-Weichfaserplatte als Pinnwand", 0.9, 0.6, 250, 0.05, 2100, None, None, "literature", "iso-10456-2007"),
        m(PAPER, "Laminated paper poster, sun-faded", "Kaschiertes Papierplakat, von der Sonne ausgebleicht", 0.9, 0.42, 1100, 0.2, 1340, None, None, "authored-estimate", "incropera-2018"),
        m(GLASSM, "Laminated safety glass 3 + 0.76 PVB + 3 mm, cracked", "Verbundsicherheitsglas 3 + 0,76 PVB + 3 mm, gesprungen", 0.84, 0.12, 2339, 0.9, 820, 7.0e10, None, "literature", "en-572-1"),
        m(DECKM, "Roof deck: sanded bitumen felt on 27 mm larch boards", "Dachaufbau: besandete Bitumenbahn auf 27-mm-Lärchenschalung", 0.91, 0.88, 664, 0.14, 1600, 1.1e10, 2.4e7, "literature", "en-13707"),
        m(MOSS, "Moss cushion, damp", "Moospolster, feucht", 0.95, 0.8, 300, 0.4, 3000, None, None, "authored-estimate", "incropera-2018"),
        m(ZINC, "Hot-dip galvanised steel, weathered", "Feuerverzinkter Stahl, bewittert", 0.28, 0.78, 7850, 50, 470, 2.1e11, 2.35e8, "literature", "duffie-beckman-2013"),
        m(TIZN, "Titanium zinc 0.7 mm, patinated", "Titanzink 0,7 mm, patiniert", 0.25, 0.7, 7200, 110, 390, 9.0e10, 1.5e8, "literature", "en-988"),
        m(CONC, "Concrete C20/25, weathered footing head", "Beton C20/25, bewitterter Fundamentkopf", 0.92, 0.68, 2300, 1.65, 1000, 3.0e10, 2.0e7, "literature", "iso-10456-2007"),
        m(STRUTM, "Gas spring, steel tube with oil and nitrogen (effective density)", "Gasdruckfeder, Stahlrohr mit Öl und Stickstoff (wirksame Dichte)", 0.3, 0.6, 4500, 50, 470, 2.1e11, 2.35e8, "authored-estimate", "en-1991-1-1"),
    ]


def rig(total):
    """🦴 The mechanical rig: two footings and posts, the case between them with its top-hinged door and gas springs, and the roof."""
    def mass(bone, src="literature", ref="en-1991-1-1"):
        mm, cx, cy, _ = total[bone]
        u, v = local(bone, cx, cy)
        return {"kg": round(mm, 3), "comX": u, "comY": v, "distribution": "point", "source": src, "reference": ref}
    sec = lambda a, i, e, mat, en, de: {"areaM2": a, "secondMomentM4": i, "extremeFibreM": e, "material": mat, "title": tx(en, de)}
    out = []
    for bid, b in BONES.items():
        bone = {"id": bid, "parent": b["parent"]}
        if b["parent"] is None:
            bone.update({"x": 0.0, "y": 0.0, "angleDeg": 0.0})
        else:
            pb = BONES[b["parent"]]
            u, v = local(b["parent"], *b["o"])
            bone.update({"x": u, "y": v, "angleDeg": r5(b["t"] - pb["t"])})
            if b["z"] != pb["z"]:
                bone["z"] = r5(b["z"] - pb["z"])
        bone["lengthM"] = r5(b["len"])
        out.append(bone)
    by = {b["id"]: b for b in out}
    for side, en_side, de_side in (("l", "left", "linken"), ("r", "right", "rechten")):
        by[f"footing-{side}"].update(title=tx(f"Concrete point footing of the {en_side} post, from its head at ground level down to its base", f"Betonpunktfundament des {de_side} Pfostens, vom Kopf in Geländehöhe hinab zur Sohle"), joint={"type": "fixed"}, mass=mass(f"footing-{side}", ref="din-en-206"), section=sec(0.16, 2.1333e-3, 0.2, CONC, "Concrete C20/25, 400 x 400 mm", "Beton C20/25, 400 x 400 mm"))
        by[f"post-{side}"].update(title=tx(f"{'Weathered' if side == 'l' else 'Replaced fresh'} larch post 120 x 120 mm on its galvanised U-anchor, from the footing head up to the gable apex, with its rafter pair", f"{'Verwitterter' if side == 'l' else 'Ersetzter frischer'} Lärchenpfosten 120 x 120 mm auf seinem verzinkten U-Pfostenträger, vom Fundamentkopf bis zur Giebelspitze, mit seinem Sparrenpaar"), joint={"type": "fixed"}, mass=mass(f"post-{side}", ref="en-350"), section=sec(0.0144, 1.728e-5, 0.06, GREY if side == "l" else FRESH, "Larch C24, 120 x 120 mm", "Lärche C24, 120 x 120 mm"))
        by[f"strut-{side}"].update(title=tx(f"Body of the {en_side} gas spring, from its ball socket on the case side", f"Zylinder der {de_side} Gasdruckfeder, von ihrer Kugelpfanne an der Kastenseite"), joint={"type": "revolute", "minDeg": -35, "maxDeg": 0}, section=sec(7.775e-5, 2.67e-9, R_STRUT, STRUTM, "Gas spring tube 18 x 1.5 mm", "Gasfederrohr 18 x 1,5 mm"))
        by[f"rod-{side}"].update(title=tx(f"Piston rod of the {en_side} gas spring, sliding out to the door bracket", f"Kolbenstange der {de_side} Gasdruckfeder, bis zum Türbeschlag ausfahrend"), joint={"type": "prismatic", "minM": -0.01, "maxM": 0.16, "axisDeg": 0}, section=sec(5.027e-5, 2.01e-10, R_ROD, STRUTM, "Piston rod 8 mm", "Kolbenstange 8 mm"))
    by["base"].update(title=tx("Ground reference midway between the two posts", "Bodenbezug mittig zwischen den beiden Pfosten"), joint={"type": "fixed"})
    a_case = 2 * CASE["t"] * 0.09 + 0.015 * 1.444 + 0.012 * 1.444
    by["case"].update(title=tx("Painted larch case 1500 x 1000 x 90 mm with the plywood back, the pin board and the park map, from its bottom edge up", "Gestrichener Lärchenkasten 1500 x 1000 x 90 mm mit Sperrholzrücken, Pinnwand und Parkplan, von der Unterkante aufwärts"), joint={"type": "fixed"}, mass=mass("case", ref="en-636"), section=sec(r5(a_case), 3.2e-6, 0.045, PAINT, "Two side boards 28 x 90 mm with back panel and pin board", "Zwei Seitenbretter 28 x 90 mm mit Rückwand und Pinnwand"))
    by["door"].update(title=tx("Top-hinged glazed door, from the piano hinge down to the lock rail", "Oben angeschlagene Glastür, vom Stangenscharnier hinab zum Schlossriegel"), joint={"type": "revolute", "minDeg": 0, "maxDeg": OPEN_DEG}, mass=mass("door", ref="en-572-1"), section=sec(0.0042, 4.3e-7, 0.0175, PAINT, "Two stiles 60 x 35 mm", "Zwei Rahmenhölzer 60 x 35 mm"))
    by["roof"].update(title=tx("Gable roof: larch board deck under bitumen felt with its zinc ridge, from the ridge ahead to the front eave", "Satteldach: Lärchenschalung unter Bitumenbahn mit Zinkfirst, vom First nach vorn zur Traufe"), joint={"type": "fixed"}, mass=mass("roof", ref="en-13707"), section=sec(0.0595, 4.77e-6, 0.0155, DECKM, "Deck 1920 x 31 mm", "Schalung 1920 x 31 mm"))
    return {"kind": "mechanical", "root": "base", "bones": out}


def anchors():
    """📍 The supports at the footing bases and the load points of the poses."""
    out = []
    for side, en, de in (("l", "left", "links"), ("r", "right", "rechts")):
        out.append({"id": f"footing-base-{side}", "bone": f"footing-{side}", "x": FOOT["d"], "y": 0, "kind": "support", "title": tx(f"Footing base {en} held by the soil (fixed)", f"Fundamentsohle {de} im Boden gehalten (eingespannt)"), "support": "fixed", "footprintLengthM": 0.4, "footprintWidthM": 0.4})
        u, v = local(f"post-{side}", PH, 0.5)
        out.append({"id": f"wind-post-{side}", "bone": f"post-{side}", "x": u, "y": v, "kind": "load", "title": tx(f"Centre of the free {en} post below the case", f"Mitte des freien Pfostens {de} unter dem Kasten")})
    u, v = local("case", DOOR["x1"], 1.45)
    out.append({"id": "wind-case", "bone": "case", "x": u, "y": v, "kind": "load", "title": tx("Centre of the glazed face", "Mitte der Glasfront")})
    u, v = local("roof", DECK["x"], 2.05)
    out.append({"id": "wind-roof", "bone": "roof", "x": u, "y": v, "kind": "load", "title": tx("Centre of the wind area of the roof", "Mitte der Windfläche des Dachs")})
    u, v = local("door", DOOR["x1"], 0.98)
    out.append({"id": "lean-rail", "bone": "door", "x": u, "y": v, "kind": "load", "title": tx("Front of the lock rail where a reader leans", "Vorderseite des Schlossriegels, an den sich ein Leser lehnt")})
    u, v = local("case", PIN["x1"], 1.70)
    out.append({"id": "pin-point", "bone": "case", "x": u, "y": v, "kind": "load", "title": tx("Pin board where a notice is pinned", "Pinnwand, an der ein Aushang angeheftet wird")})
    return out


def area_estimates():
    """📐 Outer areas of the regions (m2), hand-summed face by face from the 1:50 parts and checked against the flood measure: faces against the park air and faces in contact with another region count; faces inside the closed case, faces shared by parts of one region and faces under the steel anchors and the rafters do not."""
    ph = Y_APEX - PH * T25 - Y_TIMBER0
    post = 2 * 0.12 * ph + 2 * (0.12 * ph + 0.12 * PH * T25 / 1) + 2 * (PH / C25) * 0.12 + 0.12 * 0.12
    post -= 2 * 0.10 * (SHOE["y1"] - Y_TIMBER0) + 0.12 * 0.10 + 0.12 * RAFTER["h"]
    limb = RAFTER["x"] / C25
    rafter = 2 * (2 * limb * 0.15) - 0.12 * RAFTER["h"] + 2 * 2 * limb * 0.03 + 2 * RAFTER["h"] * 0.03
    dz, cz = CASE["x1"] - CASE["x0"], 2 * CASE["z"]
    dd = DOOR["x1"] - DOOR["x0"]
    ow, oh = cz - 2 * DOOR["stile"], 1.0 - 2 * DOOR["rail"]
    case = 2 * cz * dz + 2 * 1.0 * dz + cz * 1.0 + (cz * 1.0 - ow * oh) + 2 * (ow + oh) * (DOOR["x1"] - GLASS["x1"]) + 2 * cz * dd + 2 * 1.0 * dd
    gw, gh = 2 * GLASS["z"], GLASS["y1"] - GLASS["y0"]
    iz, ih = 2 * (CASE["z"] - CASE["t"]), 1.0 - 2 * CASE["t"]
    sl = DECK["x"] / C25
    deck = 2 * 2 * sl * 2 * DECK["z"] + 2 * DECK["d"] * 2 * DECK["z"] + 2 * 2 * DECK["x"] * DECK["d"]
    moss_edges = 2 * (2 * 0.012 * 0.165 + 2 * 0.012 * 0.57)
    cap = 2 * (2 * (CAP["x"] / C25) * 2 * CAP["z"]) + 2 * CAP["c"] * 2 * CAP["z"] + 2 * 2 * CAP["x"] * CAP["c"]
    plate = 2 * 0.136 * 0.10 + 2 * (0.136 + 0.10) * SHOE["t"]
    legs = 2 * (2 * 0.10 * 0.16 + 2 * (0.16 + 0.10) * SHOE["t"])
    stem = math.pi * 2 * STEM["ro"] * (SHOE["y0"] - STEM["y0"])
    hinge = math.pi * 2 * HINGE_R * 2 * CASE["z"]
    lock = 0.03 * 0.03 + 4 * 0.03 * 0.008
    return {
        "timber-grey": post + 2 * rafter,
        "post-fresh": post,
        "case-paint": case,
        "glazing": gw * gh + 2 * (gw + gh) * (GLASS["x1"] - GLASS["x0"]),
        "notice-face": iz * ih + 2 * (iz + ih) * (PIN["x1"] - PIN["x0"]),
        "roof-felt": deck + moss_edges,
        "steel-zinc": cap + 2 * (plate + legs + stem) + hinge + lock,
        "footing-concrete": 2 * (2 * 0.4 * 0.4 + 4 * 0.4 * FOOT["d"]) - 2 * math.pi * STEM["ro"] ** 2,
    }


def regions(area):
    """🔥 The thermal regions: passive weathered surfaces of timber, paint, glass, felt, steel and concrete, and the faded map behind the glass; the board draws no power."""
    lay = lambda lid, en, de, kind, mat, t, lam, rho, cp, src="literature", ref="iso-10456-2007": {"id": lid, "title": tx(en, de), "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": float(f"{t / lam:.4g}"), "heatCapacityJM2K": round(rho * cp * t), "source": src, "reference": ref}
    def reg(rid, en, de, kind, mat, geom, lc, svf, layer, src="literature", ref="iso-10456-2007", extra=None):
        out = {"id": rid, "title": tx(en, de), "kind": kind, "model": "passive", "material": mat, "areaM2": round(area[rid], 3), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [layer], "heatCapacityJM2K": layer["heatCapacityJM2K"], "source": src, "reference": ref}
        out.update(extra or {})
        return out
    return [
        reg("timber-grey", "Old left post and the four rafters, larch weathered silver-grey, checked and tagged", "Alter linker Pfosten und die vier Sparren, Lärche silbergrau verwittert, rissig und besprüht", "surface", GREY, "vertical-plate", 2.07, 0.5, lay("post-core", "Larch to the core of the post 60 mm", "Lärche bis zum Pfostenkern 60 mm", "timber", GREY, 0.06, 0.13, 600, 1600), "authored-estimate", "feist-hon-1984"),
        reg("post-fresh", "Right post, replaced in fresh larch after a mower strike", "Rechter Pfosten, nach einem Mäheranprall in frischer Lärche ersetzt", "surface", FRESH, "vertical-plate", 2.07, 0.5, lay("post-core-fresh", "Fresh larch to the core of the post 60 mm", "Frische Lärche bis zum Pfostenkern 60 mm", "timber", FRESH, 0.06, 0.13, 600, 1600), "authored-estimate", "en-350"),
        reg("case-paint", "Case, back and door frame in dark green paint, chalked and flaking", "Kasten, Rückwand und Türrahmen in dunkelgrünem Anstrich, kreidend und abblätternd", "surface", PAINT, "vertical-plate", 1.0, 0.4, lay("board", "Painted larch board 28 mm", "Gestrichenes Lärchenbrett 28 mm", "timber", PAINT, 0.028, 0.13, 600, 1600), "literature", "duffie-beckman-2013"),
        reg("glazing", "Laminated glass with a spider crack held by its interlayer", "Verbundglas mit einem Spinnennetzsprung, von der Folie gehalten", "glazing", GLASSM, "vertical-plate", 0.9, 0.35, lay("laminate", "Laminated glass 6.76 mm", "Verbundglas 6,76 mm", "glass", GLASSM, 0.00676, 0.9, 2339, 820, "literature", "en-572-1"), "literature", "en-572-1"),
        reg("notice-face", "Pin board with the sun-faded park map behind the glass", "Pinnwand mit dem von der Sonne ausgebleichten Parkplan hinter dem Glas", "surface", PAPER, "vertical-plate", 0.94, 0.05, lay("softboard", "Softboard 12 mm", "Weichfaserplatte 12 mm", "insulation", SOFT, 0.012, 0.05, 250, 2100), "authored-estimate", "iso-10456-2007"),
        reg("roof-felt", "Roof deck in bitumen felt, moss and algae on the shaded rear slope", "Dachschalung mit Bitumenbahn, Moos und Algen am schattigen hinteren Hang", "surface", DECKM, "horizontal-plate-up", 0.31, 0.9, lay("deck", "Bitumen felt 4 mm on larch boards 27 mm", "Bitumenbahn 4 mm auf Lärchenschalung 27 mm", "timber", DECKM, 0.031, 0.1411, 664, 1600, "literature", "en-13707"), "literature", "en-13707"),
        reg("steel-zinc", "Galvanised post anchors, hinge, lock, zinc ridge and gas springs", "Verzinkte Pfostenträger, Scharnier, Schloss, Zinkfirst und Gasdruckfedern", "surface", ZINC, "cylinder", 0.1, 0.4, lay("plate", "Steel plate 8 mm", "Stahlblech 8 mm", "metal", ZINC, 0.008, 50, 7850, 470, "literature", "en-iso-1461"), "literature", "duffie-beckman-2013"),
        reg("footing-concrete", "Two concrete point footings, heads flush with the lawn", "Zwei Betonpunktfundamente, Köpfe bündig mit dem Rasen", "surface", CONC, "horizontal-plate-up", 0.4, 0.9, lay("footing-core", "Concrete to the footing axis 200 mm", "Beton bis zur Fundamentachse 200 mm", "masonry", CONC, 0.2, 1.65, 2300, 1000), "literature", "iso-10456-2007"),
    ]


SUN = {
    "in-service": {"timber-grey": 0.4, "post-fresh": 0.4, "case-paint": 0.35, "glazing": 0.8, "notice-face": 0.45, "roof-felt": 0.5, "steel-zinc": 0.35, "footing-concrete": 0.05},
    "maintenance": {"timber-grey": 0.35, "post-fresh": 0.35, "case-paint": 0.45, "glazing": 0.7, "notice-face": 0.55, "roof-felt": 0.5, "steel-zinc": 0.35, "footing-concrete": 0.05},
    "in-use": {"timber-grey": 0.4, "post-fresh": 0.4, "case-paint": 0.3, "glazing": 0.6, "notice-face": 0.35, "roof-felt": 0.5, "steel-zinc": 0.35, "footing-concrete": 0.05},
    "design-gust": {k: 0.0 for k in ("timber-grey", "post-fresh", "case-paint", "glazing", "notice-face", "roof-felt", "steel-zinc", "footing-concrete")},
}


def binding(state):
    """🌞 Region bindings of a pose: a summer afternoon in the park; the footings stand in the soil, the posts on their anchors, the map rests on the back panel."""
    out = {}
    for rid, f in SUN[state].items():
        b = {"exposedFraction": 1, "sunlitFraction": f}
        if rid == "footing-concrete":
            b = {"exposedFraction": 0.1, "sunlitFraction": f, "contact": {"fraction": 0.9, "tempC": 14}}
        elif rid == "roof-felt" and state == "design-gust":
            b = {"exposedFraction": 1, "sunlitFraction": 0, "wettedness": 0.6}
        out[rid] = b
    return {"regions": out, "emitters": {}}


def door_bones(deg):
    """🚪 Bone offsets of door and both gas springs with the door opened by deg."""
    a, length = strut_state(deg)
    off = {"door": {"angleDeg": r5(deg)}}
    for side in ("l", "r"):
        off[f"strut-{side}"] = {"angleDeg": r5(a - STRUT0_DEG)}
        off[f"rod-{side}"] = {"dx": r5(length - STRUT0_L)}
    return off


def poses():
    """🧍 In service, opened by the depot to change the notices, leaned on by a reader, and in a design gust."""
    q, cf = 0.65, 1.8
    gust = [
        {"id": "wind-on-case", "kind": "wind", "title": tx("Design gust on the glazed face (0.65 kN/m2 x 1.8 x 1.50 m2)", "Bemessungsböe auf die Glasfront (0,65 kN/m² x 1,8 x 1,50 m²)"), "anchor": "wind-case", "fxN": -round(q * cf * 1.5 * 1000), "fyN": 0, "source": "literature", "reference": "en-1991-1-4"},
        {"id": "wind-on-roof", "kind": "wind", "title": tx("Design gust on the roof above the case (0.65 kN/m2 x 1.8 x 0.42 m2)", "Bemessungsböe auf das Dach über dem Kasten (0,65 kN/m² x 1,8 x 0,42 m²)"), "anchor": "wind-roof", "fxN": -round(q * cf * 0.42 * 1000), "fyN": 0, "source": "literature", "reference": "en-1991-1-4"},
        {"id": "wind-on-post-l", "kind": "wind", "title": tx("Design gust on the free left post (0.65 kN/m2 x 2.1 x 0.107 m2)", "Bemessungsböe auf den freien linken Pfosten (0,65 kN/m² x 2,1 x 0,107 m²)"), "anchor": "wind-post-l", "z": 0, "fxN": -146, "fyN": 0, "source": "literature", "reference": "en-1991-1-4"},
        {"id": "wind-on-post-r", "kind": "wind", "title": tx("Design gust on the free right post (0.65 kN/m2 x 2.1 x 0.107 m2)", "Bemessungsböe auf den freien rechten Pfosten (0,65 kN/m² x 2,1 x 0,107 m²)"), "anchor": "wind-post-r", "z": 0, "fxN": -146, "fyN": 0, "source": "literature", "reference": "en-1991-1-4"},
    ]
    breeze = {"id": "breeze-on-case", "kind": "wind", "title": tx("Afternoon breeze on the glazed face (0.10 kN/m2 x 1.8 x 1.50 m2)", "Nachmittagsbrise auf die Glasfront (0,10 kN/m² x 1,8 x 1,50 m²)"), "anchor": "wind-case", "fxN": -270, "fyN": 0, "source": "authored-estimate", "reference": "en-1991-1-4"}
    lean = {"id": "reader-lean", "kind": "occupant", "title": tx("A reader leaning a hand on the lock rail (0.20 kN back, 0.05 kN down)", "Ein Leser stützt eine Hand auf den Schlossriegel (0,20 kN nach hinten, 0,05 kN nach unten)"), "anchor": "lean-rail", "fxN": -200, "fyN": -50, "source": "authored-estimate", "reference": "en-1991-1-1"}
    pin = {"id": "pin-press", "kind": "imposed", "title": tx("A depot worker pressing a drawing pin into the board (0.05 kN)", "Ein Bauhofmitarbeiter drückt eine Reißzwecke in die Pinnwand (0,05 kN)"), "anchor": "pin-point", "fxN": -50, "fyN": 0, "source": "authored-estimate", "reference": "en-1991-1-1"}
    base = {"ground": "authored", "contacts": ["footing-base-l", "footing-base-r"]}
    return [
        {"id": "in-service", "title": tx("In service", "Im Einsatz"), "description": tx("Locked on a calm summer afternoon: the faded park map behind the cracked laminated glass, the case on its two larch posts, the posts on galvanised U-anchors in their concrete point footings.", "Verschlossen an einem ruhigen Sommernachmittag: der ausgebleichte Parkplan hinter dem gesprungenen Verbundglas, der Kasten zwischen seinen zwei Lärchenpfosten, die Pfosten auf verzinkten U-Trägern in ihren Betonpunktfundamenten."), "bones": {}, **base, "thermal": binding("in-service"), "loads": []},
        {"id": "maintenance", "title": tx("Opened to change the notices", "Zum Wechseln der Aushänge geöffnet"), "description": tx("A depot worker has unlocked the door and the two gas springs hold it open 75 degrees under the roof while he pins a new notice to the board.", "Ein Bauhofmitarbeiter hat die Tür aufgeschlossen, die zwei Gasdruckfedern halten sie 75 Grad offen unter dem Dach, während er einen neuen Aushang an die Pinnwand heftet."), "bones": door_bones(OPEN_DEG), **base, "thermal": binding("maintenance"), "loads": [pin]},
        {"id": "in-use", "title": tx("Read by a visitor", "Von einer Besucherin gelesen"), "description": tx("A visitor reading the map leans one hand on the lock rail in an afternoon breeze.", "Eine Besucherin liest den Plan und stützt sich in einer Nachmittagsbrise mit einer Hand auf den Schlossriegel."), "bones": {}, **base, "thermal": binding("in-use"), "loads": [lean, breeze]},
        {"id": "design-gust", "title": tx("Design gust", "Bemessungsböe"), "description": tx("Closed in a storm from ahead after rain: the design gust on the glazed face, the roof and the free posts, carried by the two clamped footings.", "Geschlossen in einem Sturm von vorn nach Regen: die Bemessungsböe auf Glasfront, Dach und freie Pfosten, getragen von den zwei eingespannten Fundamenten."), "bones": {}, **base, "thermal": binding("design-gust"), "loads": gust},
    ]


def ease_open(t):
    """📈 Gas-spring opening profile: a smooth start and a damped end over the unit interval."""
    return t * t * (3 - 2 * t)


def clips():
    """🎞 The depot opening the door to change the notices, and an afternoon of reading."""
    contacts = lambda d: [{"anchor": f"footing-base-{s}", "fromS": 0, "toS": d} for s in ("l", "r")]
    keys = [{"t": 0, "pose": "in-service", "ease": "linear"}]
    for i in range(1, 13):
        t = 0.4 + 2.6 * i / 12
        keys.append({"t": round(t, 3), "pose": "in-service", "ease": "linear", "bones": door_bones(OPEN_DEG * ease_open(i / 12))})
    keys[-1] = {"t": 3.0, "pose": "maintenance", "ease": "linear"}
    keys.append({"t": 7.0, "pose": "maintenance", "ease": "linear"})
    for i in range(1, 13):
        t = 7.0 + 3.0 * i / 12
        keys.append({"t": round(t, 3), "pose": "in-service", "ease": "linear", "bones": door_bones(OPEN_DEG * (1 - ease_open(i / 12)))})
    keys[-1] = {"t": 10.0, "pose": "in-service", "ease": "linear"}
    rattle = door_bones(0.35)
    read_keys = [
        {"t": 0, "pose": "in-service", "ease": "smooth"},
        {"t": 2.5, "pose": "in-use", "ease": "smooth"},
        {"t": 5.6, "pose": "in-use", "ease": "smooth"},
        {"t": 6.0, "pose": "in-use", "ease": "smooth", "bones": rattle},
        {"t": 6.4, "pose": "in-use", "ease": "smooth"},
        {"t": 9.5, "pose": "in-use", "ease": "smooth"},
        {"t": 12, "pose": "in-service", "ease": "smooth"},
    ]
    return [
        {"id": "maintenance", "title": tx("Changing the notices", "Aushänge wechseln"), "description": tx("Unlock, let the gas springs lift the door to 75 degrees, pin a new notice to the board, pull the door down and lock it.", "Aufschließen, die Gasdruckfedern heben die Tür auf 75 Grad, einen neuen Aushang an die Pinnwand heften, die Tür herunterziehen und abschließen."), "durationS": 10, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": keys, "contacts": contacts(10),
         "phases": [{"id": "unlock", "fromS": 0, "toS": 0.4, "title": tx("Unlocking", "Aufschließen")}, {"id": "lift", "fromS": 0.4, "toS": 3.0, "title": tx("Door lifting on the gas springs", "Tür hebt sich an den Gasdruckfedern")}, {"id": "pin", "fromS": 3.0, "toS": 7.0, "title": tx("Pinning a notice", "Aushang anheften")}, {"id": "close", "fromS": 7.0, "toS": 10.0, "title": tx("Pulling down and locking", "Herunterziehen und abschließen")}],
         "events": [{"t": 0.3, "kind": "motion", "title": tx("Cylinder lock opened", "Zylinderschloss geöffnet")}, {"t": 3.0, "kind": "motion", "title": tx("Door fully open", "Tür ganz offen")}, {"t": 4.0, "kind": "load-on", "title": tx("Pin pressed in", "Reißzwecke eingedrückt"), "target": "pin-press"}, {"t": 4.6, "kind": "load-off", "title": tx("Pin seated", "Reißzwecke sitzt"), "target": "pin-press"}, {"t": 9.9, "kind": "motion", "title": tx("Lock snapped shut", "Schloss eingerastet")}],
         "tracks": [{"target": "load:pin-press", "ease": "linear", "keys": [{"t": 3.0, "v": 0}, {"t": 4.0, "v": 0}, {"t": 4.2, "v": 1}, {"t": 4.6, "v": 1}, {"t": 4.8, "v": 0}]}]},
        {"id": "reading", "title": tx("Reading in the afternoon", "Lesen am Nachmittag"), "description": tx("A visitor steps up, leans a hand on the lock rail and studies the map while the roof shadow creeps down the glass; a gust rattles the door in its worn lock.", "Eine Besucherin tritt heran, stützt eine Hand auf den Schlossriegel und studiert den Plan, während der Dachschatten über das Glas herabwandert; eine Böe lässt die Tür im ausgeschlagenen Schloss klappern."), "durationS": 12, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": read_keys, "contacts": contacts(12),
         "phases": [{"id": "step-up", "fromS": 0, "toS": 2.5, "title": tx("Stepping up", "Herantreten")}, {"id": "read", "fromS": 2.5, "toS": 9.5, "title": tx("Reading", "Lesen")}, {"id": "leave", "fromS": 9.5, "toS": 12, "title": tx("Walking on", "Weitergehen")}],
         "events": [{"t": 2.5, "kind": "load-on", "title": tx("Hand on the rail", "Hand am Riegel"), "target": "reader-lean"}, {"t": 6.0, "kind": "motion", "title": tx("Door rattles in the lock play", "Tür klappert im Schlossspiel")}, {"t": 9.5, "kind": "load-off", "title": tx("Hand off the rail", "Hand vom Riegel"), "target": "reader-lean"}],
         "tracks": [{"target": "load:breeze-on-case", "ease": "smooth", "keys": [{"t": 2.5, "v": 0.6}, {"t": 5.6, "v": 0.6}, {"t": 6.0, "v": 1.6}, {"t": 6.4, "v": 0.6}, {"t": 9.5, "v": 0.6}]},
                    {"target": "region:glazing:sunlitFraction", "ease": "linear", "keys": [{"t": 0, "v": 0.8}, {"t": 12, "v": 0.45}]},
                    {"target": "region:notice-face:sunlitFraction", "ease": "linear", "keys": [{"t": 0, "v": 0.45}, {"t": 12, "v": 0.2}]}]},
    ]


def rep_decisions(s):
    """📝 The eight authored decisions of the 2D representation at 1:s."""
    return {
        50: dict(
            silhouette=tx("Front elevation 40.4 x 59.3 mm on paper is the reading face: the two 400 mm footings below ground, the galvanised U-anchors, the weathered left post and the fresh right post, the four-board case with its door frame, the cracked glass over the faded A0 map, the lock, the piano hinge, the roof deck with its zinc ridge and the rafters at the gables. The side elevation shows the gable chevron of rafters and deck, the door 20 mm ahead of the posts and the moss on the rear slope; the plan shows the roof over everything but the footing corners. Door and gas springs carry extruded solids so they open right in all three views.", "Die Vorderansicht 40,4 x 59,3 mm ist die Leseseite: die zwei 400-mm-Fundamente unter Gelände, die verzinkten U-Träger, der verwitterte linke und der frische rechte Pfosten, der Kasten aus vier Brettern mit seinem Türrahmen, das gesprungene Glas über dem ausgebleichten A0-Plan, Schloss, Stangenscharnier, die Dachschalung mit Zinkfirst und die Sparren an den Giebeln. Die Seitenansicht zeigt den Giebelwinkel aus Sparren und Schalung, die Tür 20 mm vor den Pfosten und das Moos am hinteren Hang; die Draufsicht zeigt das Dach über allem außer den Fundamentecken. Tür und Gasdruckfedern tragen extrudierte Körper und öffnen sich so in allen drei Ansichten richtig."),
            retained=tx("Every part: footings, stems, base plates and legs of the anchors, posts, rafters, the four case boards, back panel, pin board and map, door rails and stiles, glass, lock rosette, hinge, both gas springs with their brackets, roof deck, ridge cap and two moss cushions.", "Jedes Teil: Fundamente, Stiele, Grundplatten und Laschen der Träger, Pfosten, Sparren, die vier Kastenbretter, Rückwand, Pinnwand und Plan, Türriegel und -rahmenhölzer, Glas, Schlossrosette, Scharnier, beide Gasdruckfedern mit Beschlägen, Dachschalung, Firstkappe und zwei Moospolster."),
            merged=tx("The bitumen felt is drawn with the boards as one 31 mm deck; the glazing beads are in the door frame.", "Die Bitumenbahn ist mit der Schalung als ein 31-mm-Dach gezeichnet; die Glasleisten stecken im Türrahmen."),
            omitted=tx("Screws, the M12 bolts as shafts (their heads are circles), the pins of the notices and the felt nails.", "Schrauben, die M12-Bolzen als Schäfte (ihre Köpfe sind Kreise), die Reißzwecken der Aushänge und die Pappnägel."),
            thinElements=tx("The 0.5 mm map and the 6.76 mm glass are filled outlines of their true thickness; the spider crack, the map print, the tag, the sticker residue, rust runs, strimmer scars, felt laps and algae are 0.13 mm texture strokes; drying checks, bolt heads, the drip line and the lock cylinder are 0.25 mm detail lines; flaked paint is filled detail in a paler tone.", "Der 0,5-mm-Plan und das 6,76-mm-Glas sind gefüllte Konturen ihrer wahren Dicke; Spinnennetzsprung, Plandruck, Tag, Aufkleberreste, Rostläufer, Kratzer der Motorsense, Bahnenstöße und Algen sind 0,13-mm-Texturstriche; Schwindrisse, Bolzenköpfe, Tropfkante und Schließzylinder sind 0,25-mm-Detaillinien; abgeblätterter Lack ist gefülltes Detail in hellerem Ton."),
            hatching=tx("Concrete stipple on the footings; vertical grain lines at 1.2 mm on posts and rafters.", "Betonpunktierung auf den Fundamenten; senkrechte Maserungslinien im Abstand 1,2 mm auf Pfosten und Sparren."),
            styles=tx("Felt 0.24, door frame 0.34, case 0.36, back 0.4, hinge and mounts 0.4, anchors 0.45, map 0.84, glass 0.86, old post 0.55, fresh post 0.78, rafters 0.5, moss 0.46, footing 0.62; body pen 0.5, detail 0.25, texture 0.13, glazing 0.18.", "Bahn 0,24, Türrahmen 0,34, Kasten 0,36, Rückwand 0,4, Scharnier und Beschläge 0,4, Träger 0,45, Plan 0,84, Glas 0,86, alter Pfosten 0,55, frischer Pfosten 0,78, Sparren 0,5, Moos 0,46, Fundament 0,62; Körperstift 0,5, Detail 0,25, Textur 0,13, Glas 0,18."),
            overlays=tx("timber-grey: old post and rafters; post-fresh: replaced post; case-paint: case boards, back, door frame; glazing; notice-face: pin board and map; roof-felt: deck and moss; steel-zinc: anchors, hinge, lock, ridge, gas springs and their mounts; footing-concrete.", "timber-grey: alter Pfosten und Sparren; post-fresh: ersetzter Pfosten; case-paint: Kastenbretter, Rückwand, Türrahmen; glazing; notice-face: Pinnwand und Plan; roof-felt: Schalung und Moos; steel-zinc: Träger, Scharnier, Schloss, First, Gasdruckfedern und Beschläge; footing-concrete."),
        ),
        100: dict(
            silhouette=tx("Front elevation 20.2 x 29.7 mm: footings, anchors as one block and stem, both posts, rafters, case boards, door frame, glass, pin board, hinge, gas springs with their brackets and the roof deck; side elevation and plan likewise.", "Vorderansicht 20,2 x 29,7 mm: Fundamente, Träger als ein Block mit Stiel, beide Pfosten, Sparren, Kastenbretter, Türrahmen, Glas, Pinnwand, Scharnier, Gasdruckfedern mit Beschlägen und die Dachschalung; Seitenansicht und Draufsicht ebenso."),
            retained=tx("All parts of 15 mm and more; the crack, the map outline and its main path, a drying check, the tag and the moss edge as strokes.", "Alle Teile ab 15 mm; Sprung, Planumriss mit dem Hauptweg, ein Schwindriss, der Tag und die Moosgrenze als Striche."),
            merged=tx("Base plate and legs of each anchor into one 136 x 160 mm block; the map into the pin board; the ridge cap (8 mm) and the moss cushions into the deck.", "Grundplatte und Laschen jedes Trägers in einen Block 136 x 160 mm; der Plan in die Pinnwand; die Firstkappe (8 mm) und die Moospolster in die Schalung."),
            omitted=tx("The lock rosette (8 mm proud), bolt heads, rust runs, sticker residue, strimmer scars, paint flakes and felt laps.", "Die Schlossrosette (8 mm vorstehend), Bolzenköpfe, Rostläufer, Aufkleberreste, Kratzer, Lackabplatzungen und Bahnenstöße."),
            thinElements=tx("Glass and pin board are declared thin sheets at their true 6.76 and 12 mm; the hinge (9 mm) and the piston rods (8 mm) keep their true sections as slender parts.", "Glas und Pinnwand sind als dünne Platten mit ihren wahren 6,76 und 12 mm erklärt; Scharnier (9 mm) und Kolbenstangen (8 mm) behalten als schlanke Teile ihre wahren Querschnitte."),
            hatching=tx("Concrete stipple on the footings.", "Betonpunktierung auf den Fundamenten."),
            styles=tx("Tones as 1:50; body pen 0.35, detail 0.18, texture 0.13, glazing 0.13.", "Töne wie 1:50; Körperstift 0,35, Detail 0,18, Textur 0,13, Glas 0,13."),
            overlays=tx("Regions as 1:50.", "Bereiche wie 1:50."),
        ),
        200: dict(
            silhouette=tx("Front elevation 10.1 x 14.8 mm: footings, anchor blocks on their stems, posts, rafters, the case as one ring, back panel, pin board, door frame, glass and roof deck.", "Vorderansicht 10,1 x 14,8 mm: Fundamente, Trägerblöcke auf ihren Stielen, Pfosten, Sparren, der Kasten als ein Ring, Rückwand, Pinnwand, Türrahmen, Glas und Dachschalung."),
            retained=tx("The masses that read at 30 mm; the outline of the map as a detail line.", "Die bei 30 mm lesbaren Massen; der Umriss des Plans als Detaillinie."),
            merged=tx("The four 28 mm case boards into one hollow ring of the case; the stems into solid 33.7 mm bars.", "Die vier 28-mm-Kastenbretter in einen hohlen Kastenring; die Stiele in volle 33,7-mm-Stäbe."),
            omitted=tx("Hinge, gas springs and their brackets (9 to 18 mm, inside the case or at its edge) and every texture stroke; in the opened pose the door is drawn on its hinge line without the springs.", "Scharnier, Gasdruckfedern und Beschläge (9 bis 18 mm, im Kasten oder an seiner Kante) und jeder Texturstrich; in der geöffneten Stellung ist die Tür an ihrer Scharnierlinie ohne die Federn gezeichnet."),
            thinElements=tx("Back panel, pin board and glass are declared sheets at their true thickness.", "Rückwand, Pinnwand und Glas sind als Platten in wahrer Dicke erklärt."),
            hatching=tx("None.", "Keine."),
            styles=tx("Tones as 1:50; body pen 0.25, detail 0.13.", "Töne wie 1:50; Körperstift 0,25, Detail 0,13."),
            overlays=tx("Regions as 1:50.", "Bereiche wie 1:50."),
        ),
        500: dict(
            silhouette=tx("Front elevation 4.0 x 5.9 mm: footings, a 75 mm steel block under each post, posts, rafters at 75 mm, the case ring, the door frame thickened to 75 mm ahead, glass, pin board, back panel and a 75 mm roof deck.", "Vorderansicht 4,0 x 5,9 mm: Fundamente, ein 75-mm-Stahlblock unter jedem Pfosten, Pfosten, Sparren mit 75 mm, der Kastenring, der nach vorn auf 75 mm verdickte Türrahmen, Glas, Pinnwand, Rückwand und eine 75-mm-Dachschalung."),
            retained=tx("Footings, anchors, posts, rafters, case, door, glass, board and roof.", "Fundamente, Träger, Pfosten, Sparren, Kasten, Tür, Glas, Pinnwand und Dach."),
            merged=tx("Anchor, stem and plate into one block; the door rails and stiles into one ring 75 mm deep with members widened to 100 mm (0.2 mm on paper); the 31 mm deck thickened downwards to 75 mm, taking the post heads in.", "Träger, Stiel und Platte in einen Block; Türriegel und -rahmenhölzer in einen 75 mm tiefen Ring mit auf 100 mm verbreiterten Hölzern (0,2 mm auf dem Papier); die 31-mm-Schalung nach unten auf 75 mm verdickt, die Pfostenköpfe aufnehmend."),
            omitted=tx("Every stroke below the 75 mm feature.", "Jeder Strich unter dem 75-mm-Merkmal."),
            thinElements=tx("Glass, pin board and back panel are declared sheets; the rafters (30 mm) are thickened outwards to 75 mm.", "Glas, Pinnwand und Rückwand sind erklärte Platten; die Sparren (30 mm) sind nach außen auf 75 mm verdickt."),
            hatching=tx("None.", "Keine."),
            styles=tx("Tones as 1:50; body pen 0.18, glazing 0.13.", "Töne wie 1:50; Körperstift 0,18, Glas 0,13."),
            overlays=tx("Regions as 1:50.", "Bereiche wie 1:50."),
        ),
    }[s]


def vol_decisions(s):
    """🧊 The volume decisions at 1:s."""
    return {
        50: [tx("Body 1:50 for 7.5 mm: two C20/25 footings 0.40 x 0.40 x 0.80 m, galvanised U-anchors (CHS 33.7 stem, 8 mm plate and legs), larch posts 120 x 120 mm cut to the 25 degree gable, chevron rafter pairs 30 x 150 mm, the case of four 28 mm boards with a 15 mm back panel, a 12 mm pin board and the 0.5 mm A0 map as a declared sheet, the door of rails and stiles 60 x 35 mm with 6.76 mm laminated glass declared as a sheet, the lock rosette, the 9 mm piano hinge, two gas springs (18 mm bodies, 8 mm slender rods) on their brackets, the 31 mm roof deck, the 8 mm zinc ridge and two moss cushions 8 to 16 mm thick on the rear slope.", "Körper 1:50 für 7,5 mm: zwei Fundamente C20/25 0,40 x 0,40 x 0,80 m, verzinkte U-Träger (KHP-33,7-Stiel, 8-mm-Platte und -Laschen), Lärchenpfosten 120 x 120 mm mit 25-Grad-Giebelschnitt, winkelförmige Sparrenpaare 30 x 150 mm, der Kasten aus vier 28-mm-Brettern mit 15-mm-Rückwand, 12-mm-Pinnwand und dem 0,5-mm-A0-Plan als erklärter Platte, die Tür aus durchgehenden Rahmenhölzern und Riegeln dazwischen, 60 x 35 mm mit 6,76-mm-Verbundglas als erklärter Platte, die Schlossrosette, das 9-mm-Stangenscharnier, zwei Gasdruckfedern (18-mm-Zylinder, schlanke 8-mm-Stangen) an ihren Beschlägen, die 31-mm-Dachschalung, der 8-mm-Zinkfirst und zwei 8 bis 16 mm dicke Moospolster am hinteren Hang."), tx("Thermal and structure 1:50 by reference to the body; structure adds a mass marker per loaded bone and a support block at each footing base.", "Thermik und Tragwerk 1:50 als Verweis auf den Körper; das Tragwerk ergänzt je belastetem Knochen einen Massenpunkt und an jeder Fundamentsohle einen Auflagerblock.")],
        100: [tx("Body 1:100 for 15 mm: the anchor plate and legs merged into one block on a CHS 33.7 stem, the map into the pin board (a declared 12 mm sheet), ridge cap, moss and lock rosette left to the deck and the drawing; hinge, gas springs and 15 mm brackets kept.", "Körper 1:100 für 15 mm: Trägerplatte und -laschen zu einem Block auf einem KHP-33,7-Stiel verschmolzen, der Plan in der Pinnwand (erklärte 12-mm-Platte), Firstkappe, Moos und Schlossrosette der Schalung und der Zeichnung überlassen; Scharnier, Gasdruckfedern und 15-mm-Beschläge bleiben."), tx("Thermal and structure 1:100 by reference, with mass markers and support blocks.", "Thermik und Tragwerk 1:100 als Verweis, mit Massenpunkten und Auflagerblöcken.")],
        200: [tx("Body 1:200 for 30 mm: the four case boards as one hollow ring, the stems as full 33.7 mm bars, back panel, pin board and glass as declared sheets; hinge, gas springs and brackets are below the feature and left out (the door meets the case along its back face).", "Körper 1:200 für 30 mm: die vier Kastenbretter als ein hohler Ring, die Stiele als volle 33,7-mm-Stäbe, Rückwand, Pinnwand und Glas als erklärte Platten; Scharnier, Gasdruckfedern und Beschläge liegen unter dem Merkmal und entfallen (die Tür liegt mit ihrer Rückseite am Kasten an)."), tx("Thermal 1:200 by reference, total area compared; the glass and the pin board are thermal slabs that close the case (glass back to the door, 21 mm; pin board forward to the case middle, 30 mm), because the 6.76 mm pane is thinner than the 7.5 mm flood cell; structure with the gas springs as slender members on their pivot joints, mass markers and support blocks.", "Thermik 1:200 als Verweis, verglichen wird die Gesamtfläche; Glas und Pinnwand sind thermische Scheiben, die den Kasten schließen (Glas zurück bis zur Tür, 21 mm; Pinnwand nach vorn bis zur Kastenmitte, 30 mm), weil die 6,76-mm-Scheibe dünner als die 7,5-mm-Flutzelle ist; Tragwerk mit den Gasdruckfedern als schlanke Stäbe an ihren Gelenken, Massenpunkten und Auflagerblöcken.")],
        500: [tx("Body 1:500 for 75 mm: a 120 x 75 mm steel block under each post, posts from 75 mm up, rafters 75 mm thick outwards, the case ring 90 mm deep, the door as one ring 75 mm deep ahead of the case with its members widened from 60 to 100 mm so that they print at 0.2 mm, the roof deck thickened to 75 mm downwards, glass, pin board and back panel as declared sheets.", "Körper 1:500 für 75 mm: ein Stahlblock 120 x 75 mm unter jedem Pfosten, Pfosten ab 75 mm, Sparren nach außen 75 mm dick, der Kastenring 90 mm tief, die Tür als ein 75 mm tiefer Ring vor dem Kasten, seine Hölzer von 60 auf 100 mm verbreitert, damit sie mit 0,2 mm drucken, die Dachschalung nach unten auf 75 mm verdickt, Glas, Pinnwand und Rückwand als erklärte Platten."), tx("Thermal 1:500 by reference, total area only; glass and pin board are thermal slabs (44 and 70 mm) that fill and close the case against the 19 mm flood cell; structure with the gas springs as slender members at their true 18 and 8 mm on 75 mm pivot joints, mass markers and support blocks.", "Thermik 1:500 als Verweis, nur die Gesamtfläche; Glas und Pinnwand sind thermische Scheiben (44 und 70 mm), die den Kasten gegen die 19-mm-Flutzelle füllen und schließen; Tragwerk mit den Gasdruckfedern als schlanke Stäbe in wahren 18 und 8 mm an 75-mm-Gelenken, Massenpunkten und Auflagerblöcken.")],
    }[s]


def structure_extras(k, total):
    """🏗️ The hinge joint, the gas-spring pivots, the mass markers and the support blocks of scale k."""
    s, f = k.s, FEATURE[k.s]
    hu, hv = local("case", *H)
    rj = max(f / 2, HINGE_R)
    k.structure.append({"id": "st-hinge-joint", "kind": "revolve", "bone": "case", "material": ZINC, "role": "detail", "tone": 0.2, "profile": [[-0.7, 0], [-0.7, r5(rj)], [0.7, r5(rj)], [0.7, 0]], "axis": [0, 0, 1], "origin": [hu, hv, 0], "member": "door", "part": "joint"})
    for side, sg in (("l", 1), ("r", -1)):
        if s >= 200:
            k.structure.append({"id": f"st-strut-body-{side}", "kind": "revolve", "bone": f"strut-{side}", "material": STRUTM, "role": "detail", "tone": 0.3, "profile": [[L_CYL[0], 0], [L_CYL[0], R_STRUT], [L_CYL[1], R_STRUT], [L_CYL[1], 0]], "axis": [1, 0, 0], "origin": [0, 0, 0], "slender": True, "member": f"strut-{side}", "part": "member"})
            k.structure.append({"id": f"st-strut-rod-{side}", "kind": "revolve", "bone": f"rod-{side}", "material": STRUTM, "role": "detail", "tone": 0.5, "profile": [[0, 0], [0, R_ROD], [L_ROD, R_ROD], [L_ROD, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0], "slender": True, "member": f"rod-{side}", "part": "member"})
        j = max(f, 0.02)
        zc = sg * (CASE["z"] - CASE["t"] + Z_STRUT) / 2
        jz = CASE["z"] - CASE["t"] - Z_STRUT + 0.004
        k.structure.append({"id": f"st-strut-pivot-{side}", "kind": "box", "bone": f"strut-{side}", "material": ZINC, "role": "detail", "tone": 0.2, "size": [r5(j), r5(j), r5(max(j, jz))], "transform": {"translation": [0, 0, r5(zc - sg * Z_STRUT)]}, "member": f"strut-{side}", "part": "joint"})
        du, dv = local("door", *D_STRUT)
        k.structure.append({"id": f"st-door-pivot-{side}", "kind": "box", "bone": "door", "material": ZINC, "role": "detail", "tone": 0.2, "size": [r5(j), r5(max(j, DOOR["x0"] - D_STRUT[0] + 0.004)), r5(j)], "transform": {"translation": [du, r5(dv + max(j, DOOR["x0"] - D_STRUT[0] + 0.004) / 2), r5(sg * Z_STRUT)]}, "member": f"rod-{side}", "part": "joint"})
    mr = {50: 0.02, 100: 0.025, 200: 0.03, 500: 0.04}[s]
    for bone, (m, cx, cy, cz) in total.items():
        u, v = local(bone, cx, cy)
        k.structure.append({"id": "mass-" + bone, "kind": "ellipsoid", "bone": bone, "material": CONC if bone.startswith("footing") else ZINC, "role": "detail", "tone": 0.15, "radii": [mr, mr, mr], "transform": {"translation": [u, v, r5(cz - BONES[bone]["z"])]}, "member": bone, "part": "mass"})
    for side in ("l", "r"):
        k.structure.append({"id": f"support-footing-{side}", "kind": "box", "bone": f"footing-{side}", "material": CONC, "role": "detail", "tone": 0.1, "size": [f, 0.4, 0.4], "transform": {"translation": [r5(FOOT["d"] - f / 2), 0, 0]}, "member": f"footing-{side}", "part": "support", "anchor": f"footing-base-{side}"})


def thermal_closure(k):
    """🧱 At 1:200 and 1:500 the 6.76 mm glass and the 12 mm pin board are thinner than two cells of the air flood: their thermal solids become closed slabs from the glass back to the door and from the pin board forward into the case, so the closed case stays closed for the area measure."""
    if k.s < 200:
        return
    gx1 = GLASS["x1"] if k.s == 200 else 0.08926
    px1 = 0.0 if k.s == 200 else 0.04
    gu0, gv0 = local("door", DOOR["x0"], GLASS["y1"])
    gu1, gv1 = local("door", gx1, GLASS["y0"])
    pu0, pv0 = local("case", px1, CASE["y0"] + CASE["t"])
    pu1, pv1 = local("case", PIN["x0"], CASE["y1"] - CASE["t"])
    iz = CASE["z"] - CASE["t"]
    slabs = {
        "th-glazing": {"id": "th-glazing", "kind": "box", "bone": "door", "material": GLASSM, "role": "glazing", "tone": 0.86, "size": [r5(abs(gu1 - gu0)), r5(abs(gv1 - gv0)), r5(2 * GLASS["z"])], "transform": {"translation": [r5((gu0 + gu1) / 2), r5((gv0 + gv1) / 2), 0]}, "region": "glazing"},
        "th-pin-board": {"id": "th-pin-board", "kind": "box", "bone": "case", "material": SOFT, "role": "body", "tone": 0.7, "size": [r5(abs(pu1 - pu0)), r5(abs(pv1 - pv0)), r5(2 * iz)], "transform": {"translation": [r5((pu0 + pu1) / 2), r5((pv0 + pv1) / 2), 0]}, "region": "notice-face"},
    }
    k.thermal = [slabs.get(t["id"], t) for t in k.thermal]


def hatches(s):
    """▤ The hatches of scale 1:s."""
    out = []
    if s <= 100:
        out.append({"id": "concrete-stipple", "title": tx("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8 if s == 50 else 0.6, "penMm": 0.13})
    if s == 50:
        out.append({"id": "timber-grain", "title": tx("Timber grain", "Holzmaserung"), "pattern": "lines", "angleDeg": 90, "spacingMm": 1.2, "penMm": 0.1})
    return out


def document():
    """📄 The whole asset document."""
    acc, total = masses()
    area = area_estimates()
    reps, vols = {}, {}
    for s in (50, 100, 200, 500):
        k = build(s)
        traces(k)
        structure_extras(k, total)
        thermal_closure(k)
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": BANDS[s][0], "maxDenominator": BANDS[s][1]}, "pens": PENS[s], "hatches": hatches(s), "geometry": {"elevation": k.elev, "front": k.front, "plan": k.plan}, "decisions": rep_decisions(s)}
        vols[str(s)] = {"status": "authored", "denominator": s, "body": k.body, "thermal": k.thermal, "structure": k.structure, "decisions": vol_decisions(s)}
    mass_total = round(sum(t[0] for t in total.values()), 2)
    top = deck_top(0.0) + CAP["c"]
    asset = {
        "id": "used-park-noticeboard",
        "category": "objects",
        "title": tx("Used Park Notice Board", "Gebrauchter Schaukasten"),
        "summary": tx("A lockable timber notice board of the municipal park type after many years: a 1.50 x 1.00 m case on two larch posts under a gable roof, its top-hinged door lifted by two gas springs, the laminated glass held together around a spider crack over a sun-faded park map, the dark green paint flaking, the old left post grey, checked and tagged, the right post replaced in fresh larch, moss on the shaded rear slope, the posts on galvanised U-anchors in concrete point footings.", "Ein abschließbarer Schaukasten aus Holz in der kommunalen Parkbauweise nach vielen Jahren: ein Kasten 1,50 x 1,00 m auf zwei Lärchenpfosten unter einem Satteldach, die oben angeschlagene Tür von zwei Gasdruckfedern gehoben, das Verbundglas um einen Spinnennetzsprung herum zusammengehalten über einem von der Sonne ausgebleichten Parkplan, der dunkelgrüne Anstrich blättert, der alte linke Pfosten grau, rissig und besprüht, der rechte Pfosten in frischer Lärche ersetzt, Moos am schattigen hinteren Hang, die Pfosten auf verzinkten U-Trägern in Betonpunktfundamenten."),
        "tags": ["used", "park", "public-space", "signage", "notice-board", "showcase", "timber", "larch", "laminated-glass", "gas-spring", "gable-roof", "concrete-footing", "street-furniture"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES], "notes": tx(
            f"Catalogue-typical park notice board: case 1500 x 1000 x 90 mm of 28 mm larch boards with a 35 mm door, bottom edge at 0.95 m and centre at 1.45 m (DIN 32975 eye heights 1.20 and 1.60 m), A0 map 1189 x 841 mm, larch posts 120 x 120 mm to the gable apex at 2.13 m, roof 25 degrees, 0.56 x 1.92 m, C20/25 point footings 0.40 x 0.40 x 0.80 m. Masses from volume x density of the 1:50 parts: footings {total['footing-l'][0]:.1f} kg each, posts {total['post-l'][0]:.2f} and {total['post-r'][0]:.2f} kg with their anchors and rafters, case {total['case'][0]:.2f} kg with board, map and gas springs, door {total['door'][0]:.2f} kg, roof {total['roof'][0]:.2f} kg. Nothing is measured.",
            f"Katalogtypischer Park-Schaukasten: Kasten 1500 x 1000 x 90 mm aus 28-mm-Lärchenbrettern mit 35-mm-Tür, Unterkante 0,95 m, Mitte 1,45 m (Augenhöhen 1,20 und 1,60 m nach DIN 32975), A0-Plan 1189 x 841 mm, Lärchenpfosten 120 x 120 mm bis zur Giebelspitze in 2,13 m, Dach 25 Grad, 0,56 x 1,92 m, Punktfundamente C20/25 0,40 x 0,40 x 0,80 m. Massen aus Volumen x Dichte der 1:50-Teile: Fundamente je {total['footing-l'][0]:.1f} kg, Pfosten {total['post-l'][0]:.2f} und {total['post-r'][0]:.2f} kg mit Trägern und Sparren, Kasten {total['case'][0]:.2f} kg mit Pinnwand, Plan und Gasdruckfedern, Tür {total['door'][0]:.2f} kg, Dach {total['roof'][0]:.2f} kg. Nichts ist gemessen.")},
        "dimensions": {"heightM": round(top + FOOT["d"], 4), "widthM": round(2 * DECK["x"], 4), "depthM": round(2 * (ZP + FOOT["z"]), 4), "massKg": mass_total},
        "materials": materials(),
        "rig": rig(total),
        "volumes": vols,
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": {
            "environment": {"airTempC": 27, "meanRadiantTempC": 31, "skyTempC": 14, "groundTempC": 33, "relativeHumidity": 0.45, "windMS": 1.2, "windDirectionDeg": 180, "solarWM2": 850, "diffuseFraction": 0.15, "pressurePa": 101325},
            "passengers": [], "operating": [], "emitters": [],
            "regions": regions(area),
            "assumptions": [
                tx("Passive weathered surfaces on a summer afternoon in a park: absorbed sun, long-wave exchange with sky and ground, convection and the heat capacity of the boards; the board draws no power (0 W) and carries no rating.", "Passive verwitterte Flächen an einem Sommernachmittag im Park: absorbierte Sonne, Langwellenaustausch mit Himmel und Boden, Konvektion und die Wärmekapazität der Bretter; der Kasten nimmt keine Leistung auf (0 W) und trägt keine Nennleistung."),
                tx("Weathering changes the optics: the dark green paint, chalked, still absorbs about 0.86 of the sun; silver-grey larch about 0.70, the fresh replaced post about 0.55; the sun-faded map about 0.42 against about 0.6 when printed; weathered zinc 0.78 at an emissivity of 0.28 (new about 0.65 and 0.1). Sources Duffie & Beckman, Incropera, Feist & Hon; the timber values are estimates.", "Die Bewitterung verändert die Optik: der kreidende dunkelgrüne Anstrich absorbiert noch etwa 0,86 der Sonne, silbergraue Lärche etwa 0,70, der frisch ersetzte Pfosten etwa 0,55, der ausgebleichte Plan etwa 0,42 gegen etwa 0,6 frisch gedruckt; bewittertes Zink 0,78 bei einem Emissionsgrad von 0,28 (neu etwa 0,65 und 0,1). Quellen Duffie & Beckman, Incropera, Feist & Hon; die Holzwerte sind Schätzungen."),
                tx("The closed case is a small greenhouse: sun through the glass heats the map, which lies on the pin board against the back panel, so only its face and edges exchange with the air of the case; the inside of the case is closed, so only the outer faces of case and door exchange with the park air. Fractions are scenario data.", "Der geschlossene Kasten ist ein kleines Gewächshaus: die Sonne durch das Glas erwärmt den Plan, der auf der Pinnwand an der Rückwand anliegt, sodass nur seine Fläche und Kanten mit der Luft im Kasten tauschen; das Innere des Kastens ist geschlossen, nur die Außenflächen von Kasten und Tür tauschen mit der Parkluft. Die Anteile sind Szenariodaten."),
            ],
        },
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["footing-base-l", "footing-base-r"], "assumptions": [
            tx("Each post stands on a galvanised U-anchor whose stem is cast 0.30 m into a C20/25 point footing; the soil holds the footings, idealised as one fixed support at each footing base; the bedding along the 0.8 m depth is not resolved.", "Jeder Pfosten steht auf einem verzinkten U-Träger, dessen Stiel 0,30 m in ein Punktfundament C20/25 einbetoniert ist; das Erdreich hält die Fundamente, idealisiert als je ein festes Auflager an der Fundamentsohle; die Bettung über die 0,8 m Tiefe ist nicht aufgelöst."),
            tx("The case is screwed between the posts and the roof bolted to their rafters, so posts, case and roof are one frame; the door hangs on its piano hinge and, opened, is held by two gas springs of about 2 x 150 N; the struts are lumped into the mass of the case.", "Der Kasten ist zwischen die Pfosten geschraubt und das Dach an ihre Sparren gebolzt, Pfosten, Kasten und Dach bilden so einen Rahmen; die Tür hängt an ihrem Stangenscharnier und wird geöffnet von zwei Gasdruckfedern mit etwa 2 x 150 N gehalten; die Federn sind der Masse des Kastens zugeschlagen."),
            tx("Wind after DIN EN 1991-1-4 with the German annex, zone 2 inland, peak pressure 0.65 kN/m2, force coefficient 1.80 of a signboard and 2.1 of a square post; reader and pin loads are authored estimates; timber C24 after EN 338.", "Wind nach DIN EN 1991-1-4 mit deutschem Anhang, Zone 2 Binnenland, Böengeschwindigkeitsdruck 0,65 kN/m², Kraftbeiwert 1,80 einer Anzeigetafel und 2,1 eines quadratischen Pfostens; Leser- und Reißzweckenlast sind Schätzungen; Holz C24 nach EN 338."),
        ], "clinical": False},
        "presentation": {"defaultPose": "in-service", "defaultClip": "maintenance", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    out = pathlib.Path(__file__).parent / "🔣️.json"
    doc = document()
    out.write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    acc, total = masses()
    print("[DEBUG] wrote", out, "mass", doc["asset"]["dimensions"])
    for bone, (m, cx, cy, cz) in total.items():
        print(f"[DEBUG] {bone:10s} {m:8.2f} kg  com ({cx:.3f}, {cy:.3f}, {cz:.3f})")
    print(f"[DEBUG] strut rest {STRUT0_DEG:.2f} deg {STRUT0_L:.4f} m; open {strut_state(OPEN_DEG)}")
