"""🗑️ Authoring script of `used-litter-bin-post` (run it to regenerate `🔣️.json`). Hand-chosen geometry of `used-litter-bin-post` (world rest metres: x ahead, y up, z left; origin on the post axis at ground level) and the frame helpers that turn it into bone-local 2D shapes and 3D solids.

@see ../../brief.md — the brief of the wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Front and plan views", "Volume"
"""
import json
import math

R_POST = 0.03015
Y_POST0 = -0.50
Y_POST1 = 1.20
R_CAP = 0.0315
Y_CAP1 = 1.215
R_FOOT = 0.15
Y_FOOT0 = -0.60
R_COLLAR = 0.0362
X_BIN = 0.25
R_BIN = 0.175
T_BIN = 0.00125
Y_BIN0 = 0.45
U_FLOOR0 = 0.015
U_FLOOR1 = U_FLOOR0 + T_BIN
U_RUST = 0.07
U_WALL1 = 0.555
U_BEAD = 0.56
R_BEAD = 0.005
U_TOP = 0.565
U_SICKEN = (0.25, 0.40)
DENT = ((0.07, 0.092, 0.005), (0.092, 0.114, 0.011), (0.114, 0.136, 0.015), (0.136, 0.158, 0.011), (0.158, 0.18, 0.005))
DENT_HALF_DEG = 26.0
X_PIVOT = 0.435
Y_PIVOT = 0.43
X_ARM0 = 0.034
X_ARM1 = 0.455
Y_ARM0 = 0.41
Y_ARM1 = 0.45
Z_ARM = 0.02
Y_LCOL = (0.40, 0.46)
Y_UCOL = (0.92, 0.96)
Y_RING = (0.925, 0.955)
R_RING = (0.177, 0.181)
X_STRAP = (0.034, 0.071)
Z_STRAP = 0.02
Z_LUG = (0.0205, 0.0255)
X_LUG = (0.4225, 0.455)
Y_LUG1 = 0.475
R_LUG = 0.02
R_PIN = 0.006
Z_PIN = 0.032
TONGUE = {"x": (0.06, 0.077), "y": (0.955, 0.98), "z": 0.015}
Y_HOOD0 = 1.13
Y_SKIRT1 = 1.19
Y_APEX = 1.28
R_HOOD = 0.215
T_HOOD = 0.0015
BOX = {"h": 0.07, "x": 0.075, "z": 0.07}

BONES = {
    "footing": {"origin": (0.0, 0.0), "theta": -90.0},
    "post": {"origin": (0.0, Y_POST0), "theta": 90.0},
    "carrier": {"origin": (0.0, Y_PIVOT), "theta": 0.0},
    "hood": {"origin": (0.0, Y_HOOD0), "theta": 0.0},
    "liner": {"origin": (X_PIVOT, Y_PIVOT), "theta": 90.0},
    "liner-body": {"origin": (X_BIN, Y_BIN0), "theta": 90.0},
    "litter": {"origin": (X_BIN, Y_BIN0 + U_FLOOR1), "theta": 90.0},
}


def r5(value):
    """🔢 Rounds a coordinate to 0.01 mm and removes negative zero."""
    out = round(value, 5)
    return 0.0 if out == 0 else out


def local(bone, x, y):
    """📐 World rest point (x, y) to the bone-local (u, v) of `bone`."""
    ox, oy = BONES[bone]["origin"]
    t = math.radians(BONES[bone]["theta"])
    dx, dy = x - ox, y - oy
    return (r5(dx * math.cos(t) + dy * math.sin(t)), r5(-dx * math.sin(t) + dy * math.cos(t)))


def lpoly(bone, points):
    """🔷 World polygon to bone-local points."""
    return [list(local(bone, x, y)) for x, y in points]


def arc(cx, cy, r, a0, a1, n):
    """🌙 Points on a circle from angle a0 to a1 (degrees, inclusive)."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def ring(r, n, cx=0.0, cy=0.0, dent=None):
    """⭕ A closed n-gon of radius r about (cx, cy) in world (x, z); `dent` = (depth, half angle deg) pushes it in around azimuth 0 (ahead)."""
    pts = []
    for k in range(n):
        phi = 2 * math.pi * k / n
        rr = r
        if dent is not None:
            depth, half = dent
            d = abs(math.degrees(math.atan2(math.sin(phi), math.cos(phi))))
            if d < half:
                rr = r - depth * math.cos(math.pi * d / (2 * half)) ** 2
        pts.append((cx + rr * math.cos(phi), cy + rr * math.sin(phi)))
    return pts


def to_vw(points):
    """🔁 World (x, z) offsets from the bin axis to the (v, w) profile plane of a u-up bone (v = -x)."""
    return [[r5(-x), r5(z)] for x, z in points]


def shape(sid, bone, role, z, filled, tone, geom, region=None, material=None, hatch=None, pen=None, solid=None):
    """✏️ One authored 2D shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    if region:
        out["region"] = region
    if material:
        out["material"] = material
    if hatch:
        out["hatch"] = hatch
    if pen:
        out["penMm"] = pen
    if solid:
        out["solid"] = solid
    return out


def rect(x0, y0, x1, y1, r=None):
    """⬛ Rectangle geometry from two corners."""
    g = {"kind": "rect", "x": r5(min(x0, x1)), "y": r5(min(y0, y1)), "w": r5(abs(x1 - x0)), "h": r5(abs(y1 - y0))}
    if r:
        g["r"] = r
    return g


def lrect(bone, x0, x1, y0, y1, r=None):
    """⬜ World rectangle to a bone-local rectangle (the bones are axis-aligned at rest)."""
    a = local(bone, x0, y0)
    b = local(bone, x1, y1)
    return rect(a[0], a[1], b[0], b[1], r)


def poly(points, closed=True):
    """🔺 Polygon or polyline geometry."""
    return {"kind": "poly", "points": [[r5(x), r5(y)] for x, y in points], "closed": closed}


def ellipse(cx, cy, rx, ry):
    """🥚 Ellipse geometry."""
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(rx), "ry": r5(ry)}


def path(d):
    """〰️ SVG path geometry."""
    return {"kind": "path", "d": d}


def solid(sid, kind, bone, material, role, tone, **fields):
    """🧊 One volume solid."""
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(fields)
    return out


def rev(sid, bone, material, role, tone, profile, axis, origin, **extra):
    """🌀 A revolve solid."""
    return solid(sid, "revolve", bone, material, role, tone, profile=[[r5(a), r5(b)] for a, b in profile], axis=axis, origin=[r5(c) for c in origin], **extra)


def ext(sid, bone, material, role, tone, profile, plane, frm, to, holes=None, **extra):
    """🧱 An extrude solid."""
    fields = {"profile": [[r5(a), r5(b)] for a, b in profile], "plane": plane, "from": r5(frm), "to": r5(to)}
    if holes:
        fields["holes"] = [[[r5(a), r5(b)] for a, b in h] for h in holes]
    fields.update(extra)
    return solid(sid, "extrude", bone, material, role, tone, **fields)


def box(sid, bone, material, role, tone, size, centre, **extra):
    """📦 A box solid centred at `centre` (bone-local)."""
    return solid(sid, "box", bone, material, role, tone, size=[r5(s) for s in size], transform={"translation": [r5(c) for c in centre]}, **extra)


def ell(sid, bone, material, radii, centre, **extra):
    """🔵 A mass marker ellipsoid."""
    return solid(sid, "ellipsoid", bone, material, "detail", 0.15, radii=[r5(r) for r in radii], transform={"translation": [r5(c) for c in centre]}, **extra)


def circle_profile(cu, cr, rad, n):
    """⚪ A closed revolve loop: a circle of radius `rad` about (axial cu, radial cr), first point repeated."""
    pts = [(cu + rad * math.cos(2 * math.pi * k / n), cr + rad * math.sin(2 * math.pi * k / n)) for k in range(n)]
    return pts + [pts[0]]


def annulus_profile(a0, a1, r0, r1):
    """🔘 A closed revolve loop of a rectangle in (axial, radial): a collar or a band."""
    return [(a0, r0), (a1, r0), (a1, r1), (a0, r1), (a0, r0)]


def hood_profile():
    """🎩 The closed loop of the rain hood shell (axial = y - Y_HOOD0, radial): skirt and flat cone to the pressed apex, 1.5 mm sheet."""
    h_skirt = Y_SKIRT1 - Y_HOOD0
    h_apex = Y_APEX - Y_HOOD0
    slope = (h_apex - h_skirt) / R_HOOD
    lift = T_HOOD / math.cos(math.atan(slope))
    inner_corner = h_skirt - lift + (R_HOOD - (R_HOOD - T_HOOD)) * slope
    return [(0.0, R_HOOD), (h_skirt, R_HOOD), (h_apex, 0.0), (h_apex - lift, 0.0), (inner_corner, R_HOOD - T_HOOD), (0.0, R_HOOD - T_HOOD), (0.0, R_HOOD)]


def hood_outline_xy():
    """🎩 Side and front silhouette of the hood about its axis, as offsets (lateral, y)."""
    return [(-R_HOOD, Y_HOOD0), (R_HOOD, Y_HOOD0), (R_HOOD, Y_SKIRT1), (0.0, Y_APEX), (-R_HOOD, Y_SKIRT1)]


def lug_profile_world():
    """🔩 The hinge lug outline in the sagittal plane (world x, y): welded to the bin front, rounded about the pivot pin."""
    back_y = Y_PIVOT - math.sqrt(R_LUG ** 2 - (X_PIVOT - X_LUG[0]) ** 2)
    pts = [(X_LUG[0], Y_LUG1), (X_LUG[1], Y_LUG1), (X_LUG[1], Y_PIVOT)]
    a_end = math.degrees(math.atan2(back_y - Y_PIVOT, X_LUG[0] - X_PIVOT))
    pts += arc(X_PIVOT, Y_PIVOT, R_LUG, 0.0, a_end, 8)[1:]
    return pts


def arm_profile_world(y0, y1, x0, x1, n=6):
    """🦾 Side outline of the lower support arm with its knuckle rounded about the pivot."""
    r = (y1 - y0) / 2
    yc = (y0 + y1) / 2
    xc = x1 - r
    return [(x0, y0), (xc, y0)] + arc(xc, yc, r, -90, 90, n)[1:-1] + [(xc, y1), (x0, y1)]


FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
SEGMENTS = {50: 64, 100: 48, 200: 32, 500: 24}
ZINC, RUST, CONC, CARD = "zinc-weathered", "steel-rusted", "concrete-weathered", "paperboard"


def tx(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


class Scale:
    """📏 Collects the 2D shapes and 3D solids of one authored scale, built from the one set of numbers above."""

    def __init__(self, s):
        self.s = s
        self.n = SEGMENTS[s]
        self.elev, self.front, self.plan = [], [], []
        self.body, self.thermal, self.structure = [], [], []

    def th(self, sid, region, **extra):
        """🌡️ A thermal reference to a body solid."""
        self.thermal.append({"id": "th-" + sid, "of": sid, "region": region, **extra})

    def st(self, sid, member, part, **extra):
        """🏗️ A structural reference to a body solid."""
        self.structure.append({"id": "st-" + sid, "of": sid, "member": member, "part": part, **extra})


def revolved_liner(profile_u_r):
    """🌀 Front/plan shape geometry of an axisymmetric liner part: its half profile (u, radial) in the liner-body frame, revolved about the bin axis."""
    return poly(profile_u_r), {"kind": "revolved", "axisBone": "liner-body", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}


def build(s):
    """🏗️ All shapes and solids of scale 1:s."""
    k = Scale(s)
    n = k.n
    fine = s == 50
    coarse = s >= 200
    rp = 0.0375 if s == 500 else R_POST
    hatch = "concrete-stipple" if s <= 100 else None

    k.elev.append(shape("footing", "footing", "body", 1, True, 0.62, lrect("footing", -R_FOOT, R_FOOT, Y_FOOT0, 0.0), "footing-concrete", CONC, hatch))
    k.front.append(shape("footing", "footing", "body", 1, True, 0.62, rect(-R_FOOT, Y_FOOT0, R_FOOT, 0.0), "footing-concrete", CONC, hatch))
    k.plan.append(shape("footing", "footing", "body", 1, True, 0.62, ellipse(0, 0, R_FOOT, R_FOOT), "footing-concrete", CONC, hatch))
    k.body.append(rev("footing", "footing", CONC, "body", 0.62, [(0.6, 0), (0.6, R_FOOT), (0, R_FOOT), (0, rp), (-Y_POST0, rp), (-Y_POST0, 0)], [1, 0, 0], [0, 0, 0]))
    k.th("footing", "footing-concrete")
    k.st("footing", "footing", "member")

    top = Y_POST1 if s <= 50 else Y_CAP1
    k.elev.append(shape("post", "post", "body", 2, True, 0.48, lrect("post", -rp, rp, 0.0, top), "frame-zinc", ZINC))
    k.front.append(shape("post", "post", "body", 2, True, 0.48, rect(-rp, 0.0, rp, top), "frame-zinc", ZINC))
    k.plan.append(shape("post", "post", "body", 2, True, 0.48, ellipse(0, 0, rp, rp), "frame-zinc", ZINC))
    k.body.append(rev("post", "post", ZINC, "body", 0.48, [(0, 0), (0, rp), (top - Y_POST0, rp), (top - Y_POST0, 0)], [1, 0, 0], [0, 0, 0]))
    k.th("post", "frame-zinc")
    k.st("post", "post", "member", **({"slender": True} if False else {}))
    if fine:
        cap = [(0, 0), (0, R_CAP), (0.004, R_CAP), (0.008, 0.028), (0.0115, 0.021), (0.0138, 0.012), (0.015, 0)]
        u0 = Y_POST1 - Y_POST0
        k.body.append(rev("post-cap", "post", ZINC, "body", 0.4, [(u0 + a, r) for a, r in cap], [1, 0, 0], [0, 0, 0]))
        side = [(r, Y_POST1 + a) for a, r in cap[1:-1]] + [(0, Y_CAP1)] + [(-r, Y_POST1 + a) for a, r in reversed(cap[1:-1])]
        k.elev.append(shape("post-cap", "post", "body", 2.1, True, 0.4, poly([local("post", x, y) for x, y in side]), "frame-zinc", ZINC))
        k.front.append(shape("post-cap", "post", "body", 2.1, True, 0.4, poly(side), "frame-zinc", ZINC))
        k.plan.append(shape("post-cap", "post", "body", 6.9, True, 0.4, ellipse(0, 0, R_CAP, R_CAP), "frame-zinc", ZINC))
        k.th("post-cap", "frame-zinc")
        k.st("post-cap", "post", "member")

    collars = [("collar-lower", "carrier", Y_LCOL), ("collar-upper", "carrier", Y_UCOL), ("hood-collar", "hood", (Y_HOOD0, Y_SKIRT1))] if s <= 50 else []
    for cid, bone, (y0, y1) in collars:
        oy = BONES[bone]["origin"][1]
        region = "hood-zinc" if bone == "hood" else "frame-zinc"
        k.elev.append(shape(cid, bone, "body", 2.5, True, 0.42, lrect(bone, -R_COLLAR, R_COLLAR, y0, y1), region, ZINC))
        k.front.append(shape(cid, bone, "body", 2.5, True, 0.42, rect(-R_COLLAR, y0, R_COLLAR, y1), region, ZINC))
        k.plan.append(shape(cid, bone, "body", 6.8, True, 0.42, ellipse(0, 0, R_COLLAR, R_COLLAR), region, ZINC))
        k.body.append(rev(cid, bone, ZINC, "body", 0.42, annulus_profile(y0 - oy, y1 - oy, R_POST, R_COLLAR), [0, 1, 0], [0, 0, 0]))
        k.th(cid, region)
        k.st(cid, bone, "joint")

    if s < 500:
        ax0 = X_ARM0 if s <= 50 else R_POST - 0.001
        arm = arm_profile_world(Y_ARM0, Y_ARM1, ax0, X_ARM1, 6 if s <= 100 else 4)
        arm_y0 = Y_ARM0
        za = Z_ARM
    else:
        arm = [(rp - 0.001, Y_ARM1 - 0.075), (X_ARM1, Y_ARM1 - 0.075), (X_ARM1, Y_ARM1), (rp - 0.001, Y_ARM1)]
        arm_y0 = Y_ARM1 - 0.075
        za = 0.0375
    k.elev.append(shape("arm", "carrier", "body", 3, True, 0.4, poly([local("carrier", x, y) for x, y in arm]), "frame-zinc", ZINC))
    k.front.append(shape("arm", "carrier", "body", 3, True, 0.4, rect(-za, arm_y0, za, Y_ARM1), "frame-zinc", ZINC))
    k.plan.append(shape("arm", "carrier", "body", 3, True, 0.4, rect(arm[0][0], -za, X_ARM1, za), "frame-zinc", ZINC))
    k.body.append(ext("arm", "carrier", ZINC, "body", 0.4, [local("carrier", x, y) for x, y in arm], "xy", -za, za))
    k.th("arm", "frame-zinc")
    k.st("arm", "carrier", "member")

    if s <= 200:
        sx0 = X_STRAP[0] if s <= 50 else R_POST - 0.001
        sx1 = X_STRAP[1] if s <= 50 else X_BIN - R_BIN + 0.0005
        zs = Z_STRAP if s <= 100 else 0.02
        k.elev.append(shape("strap", "carrier", "body", 3.4, True, 0.4, lrect("carrier", sx0, sx1, *Y_RING), "frame-zinc", ZINC))
        k.front.append(shape("strap", "carrier", "body", 3.4, True, 0.4, rect(-zs, Y_RING[0], zs, Y_RING[1]), "frame-zinc", ZINC))
        k.plan.append(shape("strap", "carrier", "body", 3.4, True, 0.4, rect(sx0, -zs, sx1, zs), "frame-zinc", ZINC))
        cu, cv = local("carrier", (sx0 + sx1) / 2, sum(Y_RING) / 2)
        k.body.append(box("strap", "carrier", ZINC, "body", 0.4, [sx1 - sx0, Y_RING[1] - Y_RING[0], 2 * zs], [cu, cv, 0]))
        k.th("strap", "frame-zinc")
        k.st("strap", "carrier", "member")
    if fine:
        outer = arc(X_BIN, 0, R_RING[1], 90, 270, 24)
        inner = arc(X_BIN, 0, R_RING[0], 270, 90, 24)
        half = outer + inner
        k.elev.append(shape("ring", "carrier", "body", 3.5, True, 0.45, lrect("carrier", X_BIN - R_RING[1], X_BIN, *Y_RING), "frame-zinc", ZINC))
        k.front.append(shape("ring", "carrier", "body", 3.3, True, 0.45, rect(-R_RING[1], Y_RING[0], R_RING[1], Y_RING[1]), "frame-zinc", ZINC))
        k.plan.append(shape("ring", "carrier", "body", 3.5, True, 0.45, poly(half), "frame-zinc", ZINC))
        v0 = Y_RING[0] - Y_PIVOT
        k.body.append(ext("ring", "carrier", ZINC, "body", 0.45, [(z, x) for x, z in half], "zx", v0, v0 + Y_RING[1] - Y_RING[0]))
        k.th("ring", "frame-zinc")
        k.st("ring", "carrier", "member")
        k.elev.append(shape("pin", "carrier", "detail", 5.9, True, 0.2, ellipse(X_PIVOT, 0, R_PIN, R_PIN), "frame-zinc", ZINC))
        k.front.append(shape("pin", "carrier", "detail", 3.1, True, 0.2, rect(-Z_PIN, Y_PIVOT - R_PIN, Z_PIN, Y_PIVOT + R_PIN), "frame-zinc", ZINC))
        k.plan.append(shape("pin", "carrier", "detail", 3.1, True, 0.2, rect(X_PIVOT - R_PIN, -Z_PIN, X_PIVOT + R_PIN, Z_PIN), "frame-zinc", ZINC))
        k.body.append(rev("pin", "carrier", ZINC, "detail", 0.2, [(-Z_PIN, 0), (-Z_PIN, R_PIN), (Z_PIN, R_PIN), (Z_PIN, 0)], [0, 0, 1], [X_PIVOT, 0, 0]))
        k.th("pin", "frame-zinc")
        k.st("pin", "liner", "joint")
        lug = lug_profile_world()
        for side, (z0, z1) in (("l", Z_LUG), ("r", (-Z_LUG[1], -Z_LUG[0]))):
            prof = [local("liner", x, y) for x, y in lug]
            sol = {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}
            k.elev.append(shape("lug-" + side, "liner", "body", 5.5 if side == "r" else 3.2, True, 0.3, poly(prof), "liner-rust", RUST))
            k.front.append(shape("lug-" + side, "liner", "body", 3.2, True, 0.3, poly(prof), "liner-rust", RUST, solid=sol))
            k.plan.append(shape("lug-" + side, "liner", "body", 3.2, True, 0.3, poly(prof), "liner-rust", RUST, solid=sol))
            k.body.append(ext("lug-" + side, "liner", RUST, "body", 0.3, prof, "xy", z0, z1))
            k.th("lug-" + side, "liner-rust")
    elif s <= 200:
        bx0 = 0.02 if s == 100 else 0.015
        bx1 = 0.04 if s == 100 else 0.045
        bz = 0.01 if s == 100 else 0.015
        k.elev.append(shape("hood-bracket", "hood", "body", 2.6, True, 0.42, lrect("hood", bx0, bx1, Y_HOOD0, Y_SKIRT1), "hood-zinc", ZINC))
        k.front.append(shape("hood-bracket", "hood", "body", 2.6, True, 0.42, rect(-bz, Y_HOOD0, bz, Y_SKIRT1), "hood-zinc", ZINC))
        k.plan.append(shape("hood-bracket", "hood", "body", 2.6, True, 0.42, rect(bx0, -bz, bx1, bz), "hood-zinc", ZINC))
        k.body.append(box("hood-bracket", "hood", ZINC, "body", 0.42, [bx1 - bx0, Y_SKIRT1 - Y_HOOD0, 2 * bz], [(bx0 + bx1) / 2, (Y_SKIRT1 - Y_HOOD0) / 2, 0]))
        k.th("hood-bracket", "hood-zinc")
        k.st("hood-bracket", "hood", "joint")

    u_rust = 0.075 if s == 500 else U_RUST
    top_wall = U_WALL1 if fine else U_TOP
    rb = R_BIN + 0.005 if fine else R_BIN
    k.elev.append(shape("rust-band", "liner-body", "body", 4.0, True, 0.24, lrect("liner-body", X_BIN - R_BIN, X_BIN + R_BIN, Y_BIN0, Y_BIN0 + u_rust), "liner-rust", RUST))
    g, sol = revolved_liner([(0, 0), (0, R_BIN), (u_rust, R_BIN), (u_rust, 0)])
    k.front.append(shape("rust-band", "liner-body", "body", 4.0, True, 0.24, g, "liner-rust", RUST, solid=sol))
    k.plan.append(shape("rust-band", "liner-body", "body", 4.0, True, 0.24, g, "liner-rust", RUST, solid=sol))
    k.body.append(ext("rust-band", "liner-body", RUST, "body", 0.24, to_vw(ring(R_BIN, n)), "yz", 0.0, u_rust, holes=[to_vw(ring(R_BIN - T_BIN, n))]))
    k.th("rust-band", "liner-rust")
    k.st("rust-band", "liner-body", "member")
    k.body.append(rev("floor", "liner-body", RUST, "body", 0.24, [(U_FLOOR0, 0), (U_FLOOR0, R_BIN - T_BIN), (U_FLOOR1, R_BIN - T_BIN), (U_FLOOR1, 0)], [1, 0, 0], [0, 0, 0], sheet=True))
    k.th("floor", "liner-rust")
    k.st("floor", "liner-body", "member")

    if fine:
        front_edge = [(U_WALL1, -R_BIN), (DENT[-1][1], -R_BIN)]
        for u0, u1, d in reversed(DENT):
            front_edge += [(u1, -(R_BIN - d)), (u0, -(R_BIN - d))]
        front_edge += [(DENT[0][0], -R_BIN)]
        wall = [(u_rust, R_BIN), (U_WALL1, R_BIN)] + front_edge
        k.elev.append(shape("wall", "liner-body", "body", 4.1, True, 0.5, poly(wall), "liner-zinc", ZINC))
        for i, (u0, u1, d) in enumerate(DENT):
            k.body.append(ext(f"dent-{i}", "liner-body", ZINC, "body", 0.5, to_vw(ring(R_BIN, n, dent=(d, DENT_HALF_DEG))), "yz", u0, u1, holes=[to_vw(ring(R_BIN - T_BIN, n, dent=(d, DENT_HALF_DEG)))]))
            k.th(f"dent-{i}", "liner-zinc")
            k.st(f"dent-{i}", "liner-body", "member")
        k.body.append(ext("wall", "liner-body", ZINC, "body", 0.5, to_vw(ring(R_BIN, n)), "yz", DENT[-1][1], U_WALL1, holes=[to_vw(ring(R_BIN - T_BIN, n))]))
    else:
        k.elev.append(shape("wall", "liner-body", "body", 4.1, True, 0.5, lrect("liner-body", X_BIN - R_BIN, X_BIN + R_BIN, Y_BIN0 + u_rust, Y_BIN0 + top_wall), "liner-zinc", ZINC))
        k.body.append(ext("wall", "liner-body", ZINC, "body", 0.5, to_vw(ring(R_BIN, n)), "yz", u_rust, top_wall, holes=[to_vw(ring(R_BIN - T_BIN, n))]))
    g, sol = revolved_liner([(u_rust, 0), (u_rust, R_BIN), (top_wall, R_BIN), (top_wall, 0)])
    k.front.append(shape("wall", "liner-body", "body", 4.1, True, 0.5, g, "liner-zinc", ZINC, solid=sol))
    k.plan.append(shape("wall", "liner-body", "body", 4.1, True, 0.5, g, "liner-zinc", ZINC, solid=sol))
    k.th("wall", "liner-zinc")
    k.st("wall", "liner-body", "member")

    if fine:
        k.elev.append(shape("rim-bead", "liner-body", "body", 4.3, True, 0.58, lrect("liner-body", X_BIN - rb, X_BIN + rb, Y_BIN0 + U_WALL1, Y_BIN0 + U_TOP), "liner-zinc", ZINC))
        g, sol = revolved_liner([(U_WALL1, 0), (U_WALL1, rb), (U_TOP, rb), (U_TOP, 0)])
        k.front.append(shape("rim-bead", "liner-body", "body", 4.3, True, 0.58, g, "liner-zinc", ZINC, solid=sol))
        k.plan.append(shape("rim-bead", "liner-body", "body", 4.3, True, 0.58, g, "liner-zinc", ZINC, solid=sol))
        k.body.append(rev("rim-bead", "liner-body", ZINC, "body", 0.58, circle_profile(U_BEAD, R_BIN, R_BEAD, 16), [1, 0, 0], [0, 0, 0]))
        k.th("rim-bead", "liner-zinc")
        k.st("rim-bead", "liner-body", "member")
        for i, us in enumerate(U_SICKEN):
            k.body.append(rev(f"sicken-{i}", "liner-body", ZINC, "detail", 0.5, circle_profile(us, R_BIN - 0.0005, 0.0025, 12), [1, 0, 0], [0, 0, 0], slender=True))
            k.th(f"sicken-{i}", "liner-zinc")
            k.elev.append(shape(f"sicken-{i}", "liner-body", "detail", 4.4, False, 0.3, poly([(us, R_BIN + 0.002), (us, -(R_BIN + 0.002))], False), "liner-zinc", ZINC))
            k.front.append(shape(f"sicken-{i}", "liner-body", "detail", 4.4, False, 0.3, poly([(-(R_BIN + 0.002), Y_BIN0 + us), (R_BIN + 0.002, Y_BIN0 + us)], False), "liner-zinc", ZINC))

    if s <= 100:
        tu0, tv0 = local("liner-body", TONGUE["x"][1], TONGUE["y"][0])
        tu1, tv1 = local("liner-body", TONGUE["x"][0], TONGUE["y"][1])
        tg = rect(tu0, tv0, tu1, tv1)
        tsol = {"kind": "extruded", "sweep": "w", "z0": -TONGUE["z"], "z1": TONGUE["z"]}
        k.elev.append(shape("tongue", "liner-body", "body", 4.5, True, 0.35, tg, "liner-zinc", ZINC))
        k.front.append(shape("tongue", "liner-body", "body", 2.9, True, 0.35, tg, "liner-zinc", ZINC, solid=tsol))
        k.plan.append(shape("tongue", "liner-body", "body", 3.6, True, 0.35, tg, "liner-zinc", ZINC, solid=tsol))
        k.body.append(box("tongue", "liner-body", ZINC, "body", 0.35, [tu1 - tu0, tv1 - tv0, 2 * TONGUE["z"]], [(tu0 + tu1) / 2, (tv0 + tv1) / 2, 0]))
        k.th("tongue", "liner-zinc")
        k.st("tongue", "liner-body", "joint")

    bh = 0.075 if s == 500 else BOX["h"]
    bg = rect(0, -BOX["x"], bh, BOX["x"])
    bsol = {"kind": "extruded", "sweep": "w", "z0": -BOX["z"], "z1": BOX["z"]}
    k.elev.append(shape("litter-box", "litter", "body", 3.9, True, 0.75, bg, "litter-box", CARD))
    k.front.append(shape("litter-box", "litter", "body", 3.9, True, 0.75, bg, "litter-box", CARD, solid=bsol))
    k.plan.append(shape("litter-box", "litter", "body", 3.9, True, 0.75, bg, "litter-box", CARD, solid=bsol))
    k.body.append(box("litter-box", "litter", CARD, "body", 0.75, [bh, 2 * BOX["x"], 2 * BOX["z"]], [bh / 2, 0, 0]))
    k.th("litter-box", "litter-box")

    hood = [(X_BIN + a, y) for a, y in hood_outline_xy()]
    k.elev.append(shape("hood", "hood", "body", 6, True, 0.55, poly([local("hood", x, y) for x, y in hood]), "hood-zinc", ZINC))
    k.front.append(shape("hood", "hood", "body", 6, True, 0.55, poly(hood_outline_xy()), "hood-zinc", ZINC))
    k.plan.append(shape("hood", "hood", "body", 6, True, 0.55, ellipse(X_BIN, 0, R_HOOD, R_HOOD), "hood-zinc", ZINC))
    k.body.append(rev("hood", "hood", ZINC, "body", 0.55, hood_profile(), [0, 1, 0], [X_BIN, 0, 0]))
    k.th("hood", "hood-zinc")
    k.st("hood", "hood", "member")
    return k


def traces(k):
    """🩹 Traces of use and weather as detail and texture strokes: sticker residue, a felt-tip tag, rust runs, the kick dent, strimmer scars, algae on the shaded hood and the stipple of the weathered footing head."""
    s = k.s
    if s > 100:
        return
    zinc = ("liner-zinc", ZINC)
    if s == 50:
        lu = lambda x, y: local("liner-body", x, y)
        k.elev.append(shape("dent-crease", "liner-body", "detail", 4.6, False, 0.3, path("M 0.075 -0.172 C 0.1 -0.163 0.15 -0.163 0.175 -0.172"), *zinc))
        k.elev.append(shape("sticker-residue", "liner-body", "texture", 4.6, False, 0.6, path("M 0.31 -0.06 L 0.37 -0.06 L 0.37 -0.135 L 0.31 -0.135 Z M 0.33 -0.02 L 0.355 0.01 L 0.335 0.04 M 0.40 -0.11 L 0.44 -0.09"), *zinc))
        k.elev.append(shape("tag", "liner-body", "texture", 4.6, False, 0.2, path("M 0.47 0.05 C 0.49 0.0 0.45 -0.03 0.48 -0.07 C 0.5 -0.1 0.46 -0.12 0.49 -0.15"), *zinc))
        k.elev.append(shape("rust-runs", "liner-body", "texture", 4.6, False, 0.2, path("M 0.07 0.12 L 0.105 0.118 M 0.07 0.03 L 0.12 0.031 M 0.07 -0.08 L 0.098 -0.081"), "liner-rust", RUST))
        k.front.append(shape("dent-outline", "liner-body", "detail", 4.6, False, 0.3, ellipse(0.0, Y_BIN0 + 0.125, 0.07, 0.05), *zinc))
        k.front.append(shape("dent-crease", "liner-body", "texture", 4.6, False, 0.3, path(f"M -0.05 {Y_BIN0 + 0.125} C -0.02 {Y_BIN0 + 0.118} 0.02 {Y_BIN0 + 0.132} 0.05 {Y_BIN0 + 0.125}"), *zinc))
        k.front.append(shape("sticker-residue", "liner-body", "texture", 4.6, False, 0.6, path(f"M -0.12 {Y_BIN0 + 0.31} L -0.05 {Y_BIN0 + 0.31} L -0.05 {Y_BIN0 + 0.37} L -0.12 {Y_BIN0 + 0.37} Z M 0.03 {Y_BIN0 + 0.42} L 0.09 {Y_BIN0 + 0.42} L 0.09 {Y_BIN0 + 0.46} L 0.03 {Y_BIN0 + 0.46} Z M -0.02 {Y_BIN0 + 0.33} L 0.0 {Y_BIN0 + 0.35}"), *zinc))
        k.front.append(shape("tag", "liner-body", "texture", 4.6, False, 0.2, path(f"M -0.13 {Y_BIN0 + 0.24} C -0.09 {Y_BIN0 + 0.28} -0.06 {Y_BIN0 + 0.2} -0.02 {Y_BIN0 + 0.25} C 0.02 {Y_BIN0 + 0.29} 0.06 {Y_BIN0 + 0.21} 0.11 {Y_BIN0 + 0.26}"), *zinc))
        k.front.append(shape("rust-runs", "liner-body", "texture", 4.6, False, 0.2, path(f"M -0.1 {Y_BIN0 + 0.07} L -0.098 {Y_BIN0 + 0.11} M 0.02 {Y_BIN0 + 0.07} L 0.021 {Y_BIN0 + 0.125} M 0.12 {Y_BIN0 + 0.07} L 0.118 {Y_BIN0 + 0.1}"), "liner-rust", RUST))
        k.elev.append(shape("strimmer-scars", "post", "texture", 2.2, False, 0.3, path("M 0.03 -0.03 L 0.06 -0.03 M 0.05 0.02 L 0.09 0.02 M 0.11 -0.025 L 0.13 -0.025"), "frame-zinc", ZINC))
        k.front.append(shape("strimmer-scars", "post", "texture", 2.2, False, 0.3, path("M -0.025 0.04 L 0.02 0.045 M -0.02 0.08 L 0.025 0.075 M -0.015 0.12 L 0.01 0.122"), "frame-zinc", ZINC))
        k.elev.append(shape("algae", "hood", "texture", 6.1, False, 0.35, path("M 0.05 0.062 L 0.09 0.078 M 0.07 0.06 L 0.12 0.08 M 0.04 0.03 L 0.04 0.055"), "hood-zinc", ZINC))
        k.plan.append(shape("algae", "hood", "texture", 6.1, False, 0.35, path("M 0.06 -0.05 L 0.11 -0.02 M 0.06 0.04 L 0.12 0.06 M 0.08 0.0 L 0.13 0.01"), "hood-zinc", ZINC))
        k.plan.append(shape("hood-press-lines", "hood", "texture", 6.1, False, 0.4, path(f"M {X_BIN} 0 L {X_BIN + 0.18} 0 M {X_BIN} 0 L {X_BIN} 0.18 M {X_BIN} 0 L {X_BIN} -0.18"), "hood-zinc", ZINC))
        k.elev.append(shape("hood-hem", "hood", "detail", 6.1, False, 0.4, poly([(X_BIN - R_HOOD + 0.01, 0.004), (X_BIN + R_HOOD - 0.01, 0.004)], False), "hood-zinc", ZINC))
        k.front.append(shape("hood-hem", "hood", "detail", 6.1, False, 0.4, poly([(-R_HOOD + 0.01, Y_HOOD0 + 0.004), (R_HOOD - 0.01, Y_HOOD0 + 0.004)], False), "hood-zinc", ZINC))
        k.elev.append(shape("hood-crease", "hood", "detail", 6.1, False, 0.4, poly([(X_BIN - R_HOOD + 0.012, Y_SKIRT1 - Y_HOOD0), (X_BIN + R_HOOD - 0.012, Y_SKIRT1 - Y_HOOD0)], False), "hood-zinc", ZINC))
        k.front.append(shape("hood-crease", "hood", "detail", 6.1, False, 0.4, poly([(-R_HOOD + 0.012, Y_SKIRT1), (R_HOOD - 0.012, Y_SKIRT1)], False), "hood-zinc", ZINC))
        k.plan.append(shape("hood-crease", "hood", "detail", 6.1, False, 0.4, ellipse(X_BIN, 0, R_HOOD - 0.004, R_HOOD - 0.004), "hood-zinc", ZINC))
        k.plan.append(shape("footing-cracks", "footing", "texture", 1.1, False, 0.4, path("M -0.14 0.03 L -0.08 0.01 L -0.05 0.03 M -0.1 -0.1 L -0.06 -0.07"), "footing-concrete", CONC))
    else:
        k.elev.append(shape("sticker-residue", "liner-body", "texture", 4.6, False, 0.6, path("M 0.31 -0.06 L 0.37 -0.06 L 0.37 -0.135 L 0.31 -0.135 Z"), *zinc))
        k.elev.append(shape("dent-crease", "liner-body", "detail", 4.6, False, 0.3, path("M 0.075 -0.172 C 0.1 -0.163 0.15 -0.163 0.175 -0.172"), *zinc))
        k.front.append(shape("dent-outline", "liner-body", "detail", 4.6, False, 0.3, ellipse(0.0, Y_BIN0 + 0.125, 0.07, 0.05), *zinc))
        k.front.append(shape("sticker-residue", "liner-body", "texture", 4.6, False, 0.6, path(f"M -0.12 {Y_BIN0 + 0.31} L -0.05 {Y_BIN0 + 0.31} L -0.05 {Y_BIN0 + 0.37} L -0.12 {Y_BIN0 + 0.37} Z M 0.03 {Y_BIN0 + 0.42} L 0.09 {Y_BIN0 + 0.42} L 0.09 {Y_BIN0 + 0.46} L 0.03 {Y_BIN0 + 0.46} Z"), *zinc))
        k.elev.append(shape("hood-crease", "hood", "detail", 6.1, False, 0.4, poly([(X_BIN - R_HOOD + 0.012, Y_SKIRT1 - Y_HOOD0), (X_BIN + R_HOOD - 0.012, Y_SKIRT1 - Y_HOOD0)], False), "hood-zinc", ZINC))
        k.front.append(shape("hood-crease", "hood", "detail", 6.1, False, 0.4, poly([(-R_HOOD + 0.012, Y_SKIRT1), (R_HOOD - 0.012, Y_SKIRT1)], False), "hood-zinc", ZINC))
        k.elev.append(shape("sicken-lines", "liner-body", "detail", 4.4, False, 0.3, path(" ".join(f"M {u} {R_BIN} L {u} {-R_BIN}" for u in U_SICKEN)), *zinc))
        k.front.append(shape("sicken-lines", "liner-body", "detail", 4.4, False, 0.3, path(" ".join(f"M {-R_BIN} {Y_BIN0 + u} L {R_BIN} {Y_BIN0 + u}" for u in U_SICKEN)), *zinc))


def masses():
    """⚖️ Bone masses from section x density (kg) and their centres in world rest metres; one source for the rig, the mass markers and the report."""
    rho_s, rho_c = 7850.0, 2300.0
    steel_ring = math.pi * (R_POST ** 2 - (R_POST - 0.0032) ** 2)
    out = {}
    v_conc = math.pi * R_FOOT ** 2 * 0.6 - steel_ring * (-Y_POST0)
    out["footing"] = [(v_conc * rho_c, 0.0, -0.3)]
    out["post"] = [(steel_ring * rho_s * (Y_POST1 - Y_POST0), 0.0, (Y_POST0 + Y_POST1) / 2), (0.03, 0.0, Y_POST1 + 0.006)]
    collar = lambda h: math.pi * (R_COLLAR ** 2 - R_POST ** 2) * h * rho_s
    out["carrier"] = [
        (4.21e-4 * (X_ARM1 - X_ARM0) * rho_s, (X_ARM0 + X_ARM1) / 2, Y_PIVOT),
        (collar(Y_LCOL[1] - Y_LCOL[0]), 0.0, sum(Y_LCOL) / 2),
        (collar(Y_UCOL[1] - Y_UCOL[0]), 0.0, sum(Y_UCOL) / 2),
        (2 * 0.004 * 0.03 * (X_STRAP[1] - X_STRAP[0] + 0.03) * rho_s, sum(X_STRAP) / 2, sum(Y_RING) / 2),
        (math.pi * sum(R_RING) / 2 * (R_RING[1] - R_RING[0]) * (Y_RING[1] - Y_RING[0]) * rho_s, X_BIN - 2 * sum(R_RING) / 2 / math.pi, sum(Y_RING) / 2),
        (math.pi * R_PIN ** 2 * 2 * Z_PIN * rho_s, X_PIVOT, Y_PIVOT),
    ]
    wall = math.pi * (R_BIN ** 2 - (R_BIN - T_BIN) ** 2) * U_WALL1 * rho_s
    floor = math.pi * (R_BIN - T_BIN) ** 2 * T_BIN * rho_s
    bead = 0.5 * 2 * math.pi ** 2 * R_BIN * R_BEAD ** 2 * rho_s
    lug_area = 0.0325 * 0.06
    out["liner-body"] = [
        (wall, X_BIN, Y_BIN0 + U_WALL1 / 2),
        (floor, X_BIN, Y_BIN0 + (U_FLOOR0 + U_FLOOR1) / 2),
        (bead, X_BIN, Y_BIN0 + U_BEAD),
        (0.1, sum(TONGUE["x"]) / 2, sum(TONGUE["y"]) / 2),
        (2 * lug_area * 0.005 * rho_s, sum(X_LUG) / 2, (Y_LUG1 + Y_PIVOT) / 2),
    ]
    slant = math.hypot(R_HOOD, Y_APEX - Y_SKIRT1)
    hood_area = math.pi * R_HOOD * slant + 2 * math.pi * R_HOOD * (Y_SKIRT1 - Y_HOOD0)
    out["hood"] = [
        (hood_area * T_HOOD * rho_s, X_BIN, Y_SKIRT1 - 0.01),
        (collar(Y_SKIRT1 - Y_HOOD0), 0.0, (Y_HOOD0 + Y_SKIRT1) / 2),
    ]
    out["litter"] = [(0.08, X_BIN, Y_BIN0 + U_FLOOR1 + BOX["h"] / 2)]
    total = {}
    for bone, parts in out.items():
        m = sum(p[0] for p in parts)
        cx = sum(p[0] * p[1] for p in parts) / m
        cy = sum(p[0] * p[2] for p in parts) / m
        total[bone] = (m, cx, cy)
    return out, total


REFERENCES = [
    ("en-10219-2", "EN 10219-2:2019 Cold formed welded steel structural hollow sections, Part 2: Tolerances, dimensions and sectional properties (CHS 60.3 x 3.2: A = 5.74 cm2, I = 23.5 cm4, 4.51 kg/m; SHS 40 x 40 x 3: A = 4.21 cm2, I = 9.78 cm4, 3.30 kg/m)."),
    ("en-10346", "EN 10346:2015 Continuously hot-dip coated steel flat products for cold forming (DX51D+Z275, the sheet of galvanised litter bins and hoods)."),
    ("en-iso-1461", "EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles (coating 45-85 micrometres on posts and brackets; white rust and the consumption of the zinc layer where water stands)."),
    ("en-1991-1-4", "DIN EN 1991-1-4:2010-12 with DIN EN 1991-1-4/NA:2010-12 Eurocode 1: Wind actions (wind zone 2 inland, peak velocity pressure 0.65 kN/m2 up to 10 m; force coefficients of short cylinders and plates)."),
    ("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1: Actions on structures, Part 1-1: densities, self-weight and imposed loads (steel 78.5 kN/m3, plain concrete 24 kN/m3 nominal)."),
    ("en-1993-1-1", "EN 1993-1-1:2005 Eurocode 3: Design of steel structures, Part 1-1 (E = 210 GPa, S235 fy = 235 MPa, density 7850 kg/m3)."),
    ("din-en-206", "DIN EN 206:2021-06 with DIN 1045-2:2023 Concrete (C20/25 for unreinforced point footings of street furniture, exposure XC2/XF1)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products, hygrothermal properties: design values of conductivity, specific heat and density of steel, concrete and paper products."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of galvanised, oxidised and rusted steel, concrete and paper)."),
    ("duffie-beckman-2013", "Duffie, J. A., Beckman, W. A. (2013). Solar Engineering of Thermal Processes, 4th ed., Wiley (solar absorptance of new and weathered galvanised steel, rusted steel and concrete)."),
    ("street-furniture-catalogue", "Catalogue-typical data of German municipal street-furniture suppliers, 2005-2024, for round 50 l post-mounted litter bins with tilting insert and rain hood (insert diameter 330-360 mm, height 550-600 mm, hot-dip galvanised sheet 1.0-1.5 mm, post 60.3 or 76.1 mm set 500 mm into a concrete point footing, throw-in rim 0.9-1.05 m above ground); a product type, no brand."),
    ("gala-bau-practice", "FLL and GaLaBau practice for the setting of posts of park furnishings: augered point footing 0.30 m in diameter and 0.60 m deep in C20/25, post embedded 0.50 m, footing head finished flush with the surface."),
]


def materials():
    """🧱 The materials with weathered surface data."""
    return [
        {"id": ZINC, "title": tx("Hot-dip galvanised steel, weathered", "Feuerverzinkter Stahl, bewittert"), "emissivity": 0.28, "solarAbsorptivity": 0.78, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 2.1e11, "strengthPa": 2.35e8, "source": "literature", "reference": "duffie-beckman-2013"},
        {"id": RUST, "title": tx("Steel sheet with the zinc consumed, rusted", "Stahlblech mit aufgezehrter Verzinkung, verrostet"), "emissivity": 0.8, "solarAbsorptivity": 0.86, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 2.1e11, "strengthPa": 2.0e8, "source": "literature", "reference": "incropera-2018"},
        {"id": CONC, "title": tx("Concrete C20/25, weathered footing head", "Beton C20/25, bewitterter Fundamentkopf"), "emissivity": 0.92, "solarAbsorptivity": 0.68, "densityKgM3": 2300, "conductivityWmK": 1.65, "specificHeatJKgK": 1000, "elasticModulusPa": 3.0e10, "strengthPa": 2.0e7, "source": "literature", "reference": "iso-10456-2007"},
        {"id": CARD, "title": tx("Paperboard takeaway box, greasy", "Pappschachtel vom Imbiss, fettig"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 700, "conductivityWmK": 0.14, "specificHeatJKgK": 1340, "source": "authored-estimate", "reference": "iso-10456-2007"},
    ]


def rig(total):
    """🦴 The mechanical rig: footing, post, carrier with the pivot, the tilting liner, its contents and the hood."""
    def mass(bone, distribution="point"):
        m, cx, cy = total[bone]
        u, v = local(bone, cx, cy)
        return {"kg": round(m, 3), "comX": u, "comY": v, "distribution": distribution, "source": "literature" if bone != "litter" else "authored-estimate", "reference": "en-10219-2" if bone in ("post", "carrier") else ("din-en-206" if bone == "footing" else "en-10346")}
    sec = lambda a, i, e, mat, en, de: {"areaM2": a, "secondMomentM4": i, "extremeFibreM": e, "material": mat, "title": tx(en, de)}
    a_bin = math.pi * (R_BIN ** 2 - (R_BIN - T_BIN) ** 2)
    i_bin = math.pi / 4 * (R_BIN ** 4 - (R_BIN - T_BIN) ** 4)
    return {"kind": "mechanical", "root": "footing", "bones": [
        {"id": "footing", "parent": None, "title": tx("Concrete point footing, from its head at ground level down to its base", "Betonpunktfundament, vom Kopf in Geländehöhe hinab zur Sohle"), "x": 0, "y": 0, "angleDeg": -90, "lengthM": 0.6, "joint": {"type": "fixed"}, "mass": mass("footing"), "section": sec(r5(math.pi * R_FOOT ** 2), 3.976e-4, R_FOOT, CONC, "Concrete C20/25, 300 mm diameter", "Beton C20/25, Durchmesser 300 mm")},
        {"id": "post", "parent": "footing", "title": tx("Galvanised post, from its embedded foot up to the cap", "Verzinkter Pfosten, vom eingespannten Fuß bis zur Kappe"), "x": -Y_POST0, "y": 0, "angleDeg": 180, "lengthM": Y_POST1 - Y_POST0, "joint": {"type": "fixed"}, "mass": mass("post", "uniform"), "section": sec(5.74e-4, 2.35e-7, R_POST, ZINC, "CHS 60.3 x 3.2 (EN 10219-2)", "KHP 60,3 x 3,2 (EN 10219-2)")},
        {"id": "carrier", "parent": "post", "title": tx("Bin carrier: post clamps, lower support arm to the pivot knuckle, upper half ring", "Behälterträger: Pfostenschellen, unterer Tragarm bis zum Gelenkauge, oberer Halbring"), "x": Y_PIVOT - Y_POST0, "y": 0, "angleDeg": -90, "lengthM": X_ARM1, "joint": {"type": "fixed"}, "mass": mass("carrier"), "section": sec(4.21e-4, 9.78e-8, 0.02, ZINC, "SHS 40 x 40 x 3 (EN 10219-2)", "QHP 40 x 40 x 3 (EN 10219-2)")},
        {"id": "liner", "parent": "carrier", "title": tx("Tilting pivot of the liner on the 12 mm pin", "Kippgelenk des Einsatzes am 12-mm-Bolzen"), "x": X_PIVOT, "y": 0, "angleDeg": 90, "lengthM": 0.02, "joint": {"type": "revolute", "minDeg": -110, "maxDeg": 0}},
        {"id": "liner-body", "parent": "liner", "title": tx("50 l galvanised liner, up its axis from the foot ring to the rolled rim", "50-l-Einsatz aus verzinktem Blech, entlang der Achse vom Fußring bis zum Rollrand"), "x": Y_BIN0 - Y_PIVOT, "y": X_PIVOT - X_BIN, "angleDeg": 0, "lengthM": U_TOP, "joint": {"type": "fixed"}, "mass": mass("liner-body"), "section": sec(r5(a_bin), r5(i_bin), R_BIN, ZINC, "Sheet cylinder 350 x 1.25 mm", "Blechzylinder 350 x 1,25 mm")},
        {"id": "litter", "parent": "liner-body", "title": tx("A crushed takeaway box thrown in, sliding along the liner axis", "Eine zerdrückte Imbissschachtel, entlang der Einsatzachse gleitend"), "x": U_FLOOR1, "y": 0, "angleDeg": 0, "lengthM": BOX["h"], "joint": {"type": "prismatic", "minM": 0, "maxM": 0.6, "axisDeg": 0}, "mass": mass("litter")},
        {"id": "hood", "parent": "post", "title": tx("Rain hood with its post collar, from the post axis ahead to the front of the skirt", "Regenhaube mit Pfostenschelle, von der Pfostenachse nach vorn bis zur Schürze"), "x": Y_HOOD0 - Y_POST0, "y": 0, "angleDeg": -90, "lengthM": X_BIN + R_HOOD, "joint": {"type": "fixed"}, "mass": mass("hood")},
    ]}


def anchors():
    """📍 The support at the footing base and the load points of the poses."""
    return [
        {"id": "footing-base", "bone": "footing", "x": 0.6, "y": 0, "kind": "support", "title": tx("Footing base held by the soil (fixed)", "Fundamentsohle im Boden gehalten (eingespannt)"), "support": "fixed", "footprintLengthM": 0.3, "footprintWidthM": 0.3},
        {"id": "floor-centre", "bone": "liner-body", "x": U_FLOOR1, "y": 0, "kind": "load", "title": tx("Centre of the liner floor", "Mitte des Einsatzbodens")},
        {"id": "wind-liner", "bone": "liner-body", "x": 0.28, "y": 0, "kind": "load", "title": tx("Centre of the wind area of the liner", "Mitte der Windfläche des Einsatzes")},
        {"id": "wind-hood", "bone": "hood", "x": X_BIN, "y": 0.07, "kind": "load", "title": tx("Centre of the wind area of the hood", "Mitte der Windfläche der Haube")},
        {"id": "wind-post", "bone": "post", "x": 0.75, "y": 0, "kind": "load", "title": tx("Centre of the exposed post below the arm", "Mitte des freien Pfostens unter dem Arm")},
    ]


def area_estimates():
    """📐 Total outer areas of the regions (m2), hand-summed from the 1:50 parts and checked against the flood measure of the validator."""
    side = lambda r, h: 2 * math.pi * r * h
    disc = lambda r: math.pi * r * r
    collar = lambda h: side(R_COLLAR, h) + 2 * (disc(R_COLLAR) - disc(R_POST))
    slant = math.hypot(R_HOOD, Y_APEX - Y_SKIRT1)
    return {
        "footing-concrete": side(R_FOOT, 0.6) + disc(R_FOOT) + disc(R_FOOT) - disc(R_POST),
        "frame-zinc": side(R_POST, Y_POST1 - Y_POST0) - side(R_POST, 0.10) + 0.0062 + collar(0.06) + collar(0.04) + 0.16 * (X_ARM1 - X_ARM0) + 0.0016 + side(sum(R_RING) / 2, 0.03) + 2 * math.pi * sum(R_RING) / 2 * 0.004 + 0.0064 + 0.0024,
        "hood-zinc": 2 * (math.pi * R_HOOD * slant + side(R_HOOD, Y_SKIRT1 - Y_HOOD0)) + collar(0.06),
        "liner-zinc": side(R_BIN, U_WALL1 - U_RUST) + side(R_BIN - T_BIN, U_WALL1 - U_RUST) + 0.5 * 4 * math.pi ** 2 * R_BIN * R_BEAD + 0.5 * 2 * 4 * math.pi ** 2 * R_BIN * 0.0025 + 0.004,
        "liner-rust": side(R_BIN, U_RUST) + side(R_BIN - T_BIN, U_RUST) + 2 * disc(R_BIN - T_BIN) + 0.0085,
        "litter-box": 2 * (0.07 * 0.15 + 0.07 * 0.14 + 0.15 * 0.14),
    }


def regions(area):
    """🔥 The thermal regions: passive weathered surfaces in the park sun, no rated power anywhere."""
    lay = lambda lid, en, de, kind, mat, t, lam, rho, cp, src, ref: {"id": lid, "title": tx(en, de), "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": r5(t / lam) if t / lam >= 1e-5 else float(f"{t / lam:.3g}"), "heatCapacityJM2K": round(rho * cp * t), "source": src, "reference": ref}
    def reg(rid, en, de, kind, mat, geom, lc, svf, layer, src="literature", ref="duffie-beckman-2013"):
        return {"id": rid, "title": tx(en, de), "kind": kind, "model": "passive", "material": mat, "areaM2": round(area[rid], 3), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [layer], "heatCapacityJM2K": layer["heatCapacityJM2K"], "source": src, "reference": ref}
    return [
        reg("liner-zinc", "Liner wall, weathered zinc with sticker residue and a tag", "Einsatzwand, bewittertes Zink mit Aufkleberresten und Tag", "surface", ZINC, "cylinder", 0.35, 0.35, lay("liner-sheet", "Galvanised sheet 1.25 mm", "Verzinktes Blech 1,25 mm", "metal", ZINC, T_BIN, 50, 7850, 470, "literature", "en-10346")),
        reg("liner-rust", "Liner foot ring, floor and hinge lugs, zinc consumed and rusted", "Fußring, Boden und Gelenklaschen des Einsatzes, Zink aufgezehrt und verrostet", "surface", RUST, "cylinder", 0.35, 0.3, lay("rusted-sheet", "Rusted sheet 1.25 mm", "Verrostetes Blech 1,25 mm", "metal", RUST, T_BIN, 50, 7850, 470, "literature", "incropera-2018"), ref="incropera-2018"),
        reg("hood-zinc", "Rain hood and its post collar, algae on the shaded rear", "Regenhaube mit Pfostenschelle, Algen an der schattigen Rückseite", "surface", ZINC, "horizontal-plate-up", 0.43, 0.85, lay("hood-sheet", "Galvanised sheet 1.5 mm", "Verzinktes Blech 1,5 mm", "metal", ZINC, T_HOOD, 50, 7850, 470, "literature", "en-10346")),
        reg("frame-zinc", "Post, clamps, support arm and half ring, galvanised and strimmer-scarred", "Pfosten, Schellen, Tragarm und Halbring, verzinkt und von der Motorsense zerkratzt", "surface", ZINC, "cylinder", 0.06, 0.5, lay("tube-wall", "Tube wall 3.2 mm", "Rohrwand 3,2 mm", "metal", ZINC, 0.0032, 50, 7850, 470, "literature", "en-10219-2")),
        reg("footing-concrete", "Concrete point footing, head flush with the trodden ground", "Betonpunktfundament, Kopf bündig mit dem festgetretenen Boden", "surface", CONC, "horizontal-plate-up", 0.3, 0.9, lay("footing-core", "Concrete to the footing axis 150 mm", "Beton bis zur Fundamentachse 150 mm", "masonry", CONC, 0.15, 1.65, 2300, 1000, "literature", "iso-10456-2007"), ref="iso-10456-2007"),
        reg("litter-box", "Takeaway box on the liner floor", "Imbissschachtel auf dem Einsatzboden", "surface", CARD, "horizontal-plate-up", 0.14, 0.05, lay("board", "Paperboard 0.5 mm", "Pappe 0,5 mm", "polymer", CARD, 0.0005, 0.14, 700, 1340, "authored-estimate", "iso-10456-2007"), src="authored-estimate", ref="iso-10456-2007"),
    ]


def binding(state):
    """🌞 Region bindings of a pose: summer noon in the park; the hood shades the liner opening, the footing is in the soil but for its head."""
    sun = {
        "in-service": {"liner-zinc": 0.45, "liner-rust": 0.45, "hood-zinc": 0.9, "frame-zinc": 0.4, "footing-concrete": 0.1, "litter-box": 0.0},
        "maintenance": {"liner-zinc": 0.6, "liner-rust": 0.5, "hood-zinc": 0.9, "frame-zinc": 0.45, "footing-concrete": 0.1, "litter-box": 0.3},
        "in-use": {"liner-zinc": 0.45, "liner-rust": 0.45, "hood-zinc": 0.9, "frame-zinc": 0.4, "footing-concrete": 0.1, "litter-box": 0.0},
    }[state]
    out = {}
    for rid, f in sun.items():
        b = {"exposedFraction": 1, "sunlitFraction": f}
        if rid == "footing-concrete":
            b = {"exposedFraction": 0.1, "sunlitFraction": 0.1, "contact": {"fraction": 0.9, "tempC": 14}}
        if rid == "frame-zinc":
            b = {"exposedFraction": 0.79, "sunlitFraction": f, "contact": {"fraction": 0.21, "tempC": 14}}
        if rid == "litter-box":
            b = {"exposedFraction": 0.8, "sunlitFraction": f, "contact": {"fraction": 0.2, "tempC": 30}}
        out[rid] = b
    return {"regions": out, "emitters": {}}


def poses():
    """🧍 In service in a breeze, tipped out for emptying, and full with a visitor's load."""
    wind = [
        {"id": "wind-on-liner", "kind": "wind", "title": tx("Design gust on the liner (0.65 kN/m2 x 0.7 x 0.20 m2)", "Bemessungsböe auf den Einsatz (0,65 kN/m² x 0,7 x 0,20 m²)"), "anchor": "wind-liner", "fxN": -91, "fyN": 0, "source": "literature", "reference": "en-1991-1-4"},
        {"id": "wind-on-hood", "kind": "wind", "title": tx("Design gust on the hood (0.65 kN/m2 x 1.0 x 0.045 m2)", "Bemessungsböe auf die Haube (0,65 kN/m² x 1,0 x 0,045 m²)"), "anchor": "wind-hood", "fxN": -29, "fyN": 0, "source": "literature", "reference": "en-1991-1-4"},
        {"id": "wind-on-post", "kind": "wind", "title": tx("Design gust on the free post (0.65 kN/m2 x 1.2 x 0.036 m2)", "Bemessungsböe auf den freien Pfosten (0,65 kN/m² x 1,2 x 0,036 m²)"), "anchor": "wind-post", "fxN": -28, "fyN": 0, "source": "literature", "reference": "en-1991-1-4"},
    ]
    contents = [
        {"id": "litter-contents", "kind": "live", "title": tx("50 l of park litter, 8 kg (160 kg/m3 loose, wet paper and bottles)", "50 l Parkabfall, 8 kg (160 kg/m³ lose, nasses Papier und Flaschen)"), "anchor": "floor-centre", "fxN": 0, "fyN": -78.5, "source": "authored-estimate"},
        {"id": "hand-press", "kind": "imposed", "title": tx("A visitor pressing the heap down at the rim (0.15 kN)", "Ein Besucher drückt den Haufen am Rand nach unten (0,15 kN)"), "anchor": "floor-centre", "fxN": 0, "fyN": -150, "source": "authored-estimate"},
    ]
    base = {"ground": "authored", "contacts": ["footing-base"]}
    return [
        {"id": "in-service", "title": tx("In service", "Im Einsatz"), "description": tx("Locked in its carrier under the rain hood, a takeaway box on the floor, a design gust from ahead; the 0.5 m post stands in its concrete point footing.", "Im Träger unter der Regenhaube verriegelt, eine Imbissschachtel auf dem Boden, eine Bemessungsböe von vorn; der Pfosten steht 0,5 m tief im Betonpunktfundament."), "bones": {}, **base, "thermal": binding("in-service"), "loads": wind},
        {"id": "maintenance", "title": tx("Tipped out for emptying", "Zum Leeren ausgekippt"), "description": tx("The depot worker has opened the triangular-key latch and tipped the liner 105 degrees forward on its pivot pin; the last box slides out of the mouth into the cart. Post, carrier and hood stay in place.", "Der Bauhofmitarbeiter hat den Dreikantverschluss geöffnet und den Einsatz am Gelenkbolzen um 105 Grad nach vorn gekippt; die letzte Schachtel rutscht aus der Öffnung in den Wagen. Pfosten, Träger und Haube bleiben stehen."), "bones": {"liner": {"angleDeg": -105}, "litter": {"dx": 0.5}}, **base, "thermal": binding("maintenance"), "loads": []},
        {"id": "in-use", "title": tx("Full, in use", "Voll, in Benutzung"), "description": tx("Full on a summer Sunday: 8 kg of litter on the floor and a visitor pressing the heap down to fit one more box.", "Voll an einem Sommersonntag: 8 kg Abfall auf dem Boden und ein Besucher, der den Haufen nach unten drückt, damit noch eine Schachtel hineinpasst."), "bones": {}, **base, "thermal": binding("in-use"), "loads": contents},
    ]


def clips():
    """🎞 The depot's emptying round and a visitor filling the bin."""
    c = [{"anchor": "footing-base", "fromS": 0, "toS": 0}]
    return [
        {"id": "maintenance", "title": tx("Emptying by the depot", "Leeren durch den Bauhof"), "description": tx("Unlock, tip the liner forward on its pivot until the box slides out, hold while the litter falls into the cart, swing back and lock.", "Entriegeln, den Einsatz am Gelenk nach vorn kippen, bis die Schachtel herausrutscht, halten, während der Abfall in den Wagen fällt, zurückschwenken und verriegeln."), "durationS": 7, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 2.5, "pose": "maintenance", "ease": "smooth"}, {"t": 4.5, "pose": "maintenance", "ease": "smooth"}, {"t": 7, "pose": "in-service", "ease": "smooth"}],
         "contacts": [{**c[0], "toS": 7}],
         "phases": [{"id": "tip-out", "fromS": 0, "toS": 2.5, "title": tx("Tipping out", "Auskippen")}, {"id": "pour", "fromS": 2.5, "toS": 4.5, "title": tx("Pouring into the cart", "In den Wagen schütten")}, {"id": "swing-back", "fromS": 4.5, "toS": 7, "title": tx("Swinging back and locking", "Zurückschwenken und verriegeln")}],
         "events": [{"t": 0.2, "kind": "motion", "title": tx("Latch opened with the triangular key", "Verschluss mit dem Dreikantschlüssel geöffnet")}, {"t": 2.5, "kind": "motion", "title": tx("Liner fully tipped", "Einsatz ganz gekippt")}, {"t": 6.9, "kind": "motion", "title": tx("Latch locked", "Verschluss verriegelt")}],
         "tracks": []},
        {"id": "filling", "title": tx("A visitor filling the bin", "Ein Besucher befüllt den Behälter"), "description": tx("A takeaway box is dropped through the throw-in gap and falls onto the floor; over the afternoon the bin fills to its 50 l and a visitor presses the heap down.", "Eine Imbissschachtel wird durch den Einwurfspalt geworfen und fällt auf den Boden; im Lauf des Nachmittags füllt sich der Behälter auf seine 50 l, und ein Besucher drückt den Haufen nach unten."), "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "in", "bones": {"litter": {"dx": 0.58}}}, {"t": 0.35, "pose": "in-service", "ease": "smooth"}, {"t": 4, "pose": "in-use", "ease": "smooth"}],
         "contacts": [{**c[0], "toS": 4}],
         "phases": [{"id": "drop", "fromS": 0, "toS": 0.35, "title": tx("Box falling", "Schachtel fällt")}, {"id": "filling", "fromS": 0.35, "toS": 4, "title": tx("Filling up", "Füllen")}],
         "events": [{"t": 0.35, "kind": "contact-on", "title": tx("Box lands on the floor", "Schachtel landet auf dem Boden")}, {"t": 4, "kind": "load-on", "title": tx("Bin full", "Behälter voll"), "target": "litter-contents"}],
         "tracks": [{"target": "load:litter-contents", "ease": "smooth", "keys": [{"t": 0.35, "v": 0}, {"t": 4, "v": 1}]}, {"target": "load:hand-press", "ease": "smooth", "keys": [{"t": 3.2, "v": 0}, {"t": 4, "v": 1}]}]},
    ]


PENS = {
    50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7},
    100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5},
    200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35},
    500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}
BANDS = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def rep_decisions(s):
    """📝 The eight authored decisions of the 2D representation at 1:s."""
    d = {
        50: dict(
            silhouette=tx("Side elevation 12.3 x 37.6 mm on paper: the 0.30 m footing below ground, the 60.3 mm post with its domed cap, the three clamps, the 40 mm support arm with its pivot knuckle, the half ring and strap, the 50 l liner with its rusted foot ring, the kick dent stepped into its front edge, the rolled rim and the lock tongue, the hinge lugs, and the flat-coned rain hood. Front and plan are the same parts seen from ahead and from above; the liner, its lugs, tongue and the box carry revolved and extruded solids, so they tilt right in the three views.", "Seitenansicht 12,3 x 37,6 mm auf dem Papier: das 0,30-m-Fundament unter Gelände, der 60,3-mm-Pfosten mit Kuppelkappe, die drei Schellen, der 40-mm-Tragarm mit Gelenkauge, Halbring und Lasche, der 50-l-Einsatz mit verrostetem Fußring, die in die Vorderkante gestufte Trittdelle, Rollrand und Verschlusszunge, die Gelenklaschen und die flach kegelige Regenhaube. Vorder- und Draufsicht zeigen dieselben Teile von vorn und von oben; Einsatz, Laschen, Zunge und Schachtel tragen gedrehte und extrudierte Körper und kippen deshalb in allen drei Ansichten richtig."),
            retained=tx("Every part of the object, the pressed stiffening beads as lines, the hood hem and crease, the pivot pin, the takeaway box on the floor.", "Jedes Teil des Gegenstands, die gepressten Sicken als Linien, Haubensaum und -knick, der Gelenkbolzen, die Imbissschachtel auf dem Boden."),
            merged=tx("Nothing merged; the post below ground lies behind the footing and is not drawn separately.", "Nichts verschmolzen; der Pfosten unter Gelände liegt hinter dem Fundament und wird nicht eigens gezeichnet."),
            omitted=tx("The triangular-key lock cylinder (14 mm, inside the strap), the drain holes of the floor and the bolts of the clamps.", "Der Dreikant-Schließzylinder (14 mm, in der Lasche), die Ablauflöcher des Bodens und die Schrauben der Schellen."),
            thinElements=tx("The 1.25 mm sheet is drawn as the filled outline of the liner; the pressed beads (2.5 mm proud) and the hood hem are 0.25 mm detail lines; sticker residue, the felt-tip tag, rust runs, strimmer scars and algae are 0.13 mm texture strokes.", "Das 1,25-mm-Blech ist die gefüllte Kontur des Einsatzes; die Sicken (2,5 mm erhaben) und der Haubensaum sind 0,25-mm-Detaillinien; Aufkleberreste, Filzstift-Tag, Rostläufer, Kratzer der Motorsense und Algen sind 0,13-mm-Texturstriche."),
            hatching=tx("Concrete stipple (dots at 0.8 mm) on the footing in all three views.", "Betonpunktierung (Punkte im Abstand 0,8 mm) auf dem Fundament in allen drei Ansichten."),
            styles=tx("Hood 0.55, liner 0.5, rim 0.58, rust band and lugs 0.24-0.3, frame 0.4-0.48, footing 0.62, box 0.75; body pen 0.5, detail 0.25, texture 0.13.", "Haube 0,55, Einsatz 0,5, Rand 0,58, Rostband und Laschen 0,24-0,3, Träger 0,4-0,48, Fundament 0,62, Schachtel 0,75; Körperstift 0,5, Detail 0,25, Textur 0,13."),
            overlays=tx("liner-zinc: wall, dent, rim, beads, tongue; liner-rust: foot ring, floor, lugs; frame-zinc: post, cap, clamps, arm, strap, ring, pin; hood-zinc: hood and hood collar; footing-concrete; litter-box.", "liner-zinc: Wand, Delle, Rand, Sicken, Zunge; liner-rust: Fußring, Boden, Laschen; frame-zinc: Pfosten, Kappe, Schellen, Arm, Lasche, Ring, Bolzen; hood-zinc: Haube und Haubenschelle; footing-concrete; litter-box."),
        ),
        100: dict(
            silhouette=tx("Side elevation 6.2 x 18.8 mm: footing, post, support arm with knuckle, strap to the liner, liner with foot ring and tongue, hood with a 20 mm bracket block to the post, the box on the floor; front and plan likewise.", "Seitenansicht 6,2 x 18,8 mm: Fundament, Pfosten, Tragarm mit Auge, Lasche zum Einsatz, Einsatz mit Fußring und Zunge, Haube mit 20-mm-Halteblock am Pfosten, die Schachtel auf dem Boden; Vorder- und Draufsicht ebenso."),
            retained=tx("Footing, post, arm, strap, liner, rust band, tongue, hood, box; the dent, sticker residue, beads and hood crease as strokes.", "Fundament, Pfosten, Arm, Lasche, Einsatz, Rostband, Zunge, Haube, Schachtel; Delle, Aufkleberreste, Sicken und Haubenknick als Striche."),
            merged=tx("The rolled rim (10 mm) into the liner wall, the cap into the post, the three 6 mm clamps and the half ring (4 mm) into the post and the strap; the hood collar becomes a 20 mm bracket block.", "Der Rollrand (10 mm) in die Einsatzwand, die Kappe in den Pfosten, die drei 6-mm-Schellen und der Halbring (4 mm) in Pfosten und Lasche; die Haubenschelle wird ein 20-mm-Halteblock."),
            omitted=tx("Hinge lugs and pin (5 and 12 mm), the tag, rust runs, strimmer scars and algae.", "Gelenklaschen und Bolzen (5 und 12 mm), Tag, Rostläufer, Kratzer der Motorsense und Algen."),
            thinElements=tx("Dent outline, beads and hood crease are 0.18 mm detail lines; sticker residue 0.13 mm texture.", "Dellenkontur, Sicken und Haubenknick sind 0,18-mm-Detaillinien; Aufkleberreste 0,13-mm-Textur."),
            hatching=tx("Concrete stipple on the footing.", "Betonpunktierung auf dem Fundament."),
            styles=tx("Tones as 1:50; body pen 0.35, detail 0.18, texture 0.13.", "Töne wie 1:50; Körperstift 0,35, Detail 0,18, Textur 0,13."),
            overlays=tx("Regions as 1:50; the bracket block is hood-zinc.", "Bereiche wie 1:50; der Halteblock gehört zu hood-zinc."),
        ),
        200: dict(
            silhouette=tx("Side elevation 3.1 x 9.4 mm: footing, post, arm, strap, liner with its rust band, hood with a 30 mm bracket, the box.", "Seitenansicht 3,1 x 9,4 mm: Fundament, Pfosten, Arm, Lasche, Einsatz mit Rostband, Haube mit 30-mm-Halter, die Schachtel."),
            retained=tx("The masses that read at 30 mm: footing, post, arm, strap, liner, rust band, hood, box.", "Die bei 30 mm lesbaren Massen: Fundament, Pfosten, Arm, Lasche, Einsatz, Rostband, Haube, Schachtel."),
            merged=tx("Tongue (17 mm) into the strap, rim into the wall, the collars into the post.", "Zunge (17 mm) in die Lasche, Rand in die Wand, die Schellen in den Pfosten."),
            omitted=tx("Every detail and texture stroke; the dent is 0.075 mm on paper.", "Jeder Detail- und Texturstrich; die Delle misst 0,075 mm auf dem Papier."),
            thinElements=tx("None below the 30 mm feature except the 1.25 mm liner sheet, which is the filled outline.", "Keine unter dem 30-mm-Merkmal außer dem 1,25-mm-Blech des Einsatzes, das die gefüllte Kontur ist."),
            hatching=tx("None.", "Keine."),
            styles=tx("Tones as 1:50; body pen 0.25.", "Töne wie 1:50; Körperstift 0,25."),
            overlays=tx("Regions as 1:50.", "Bereiche wie 1:50."),
        ),
        500: dict(
            silhouette=tx("Side elevation 1.2 x 3.8 mm: footing, the post drawn at the 75 mm print floor, a 75 mm arm block under the liner, the liner with a 75 mm rust band, the hood, the box.", "Seitenansicht 1,2 x 3,8 mm: Fundament, der Pfosten an der 75-mm-Druckgrenze, ein 75-mm-Armblock unter dem Einsatz, der Einsatz mit 75-mm-Rostband, die Haube, die Schachtel."),
            retained=tx("Footing, post, arm, liner, rust band, hood, box.", "Fundament, Pfosten, Arm, Einsatz, Rostband, Haube, Schachtel."),
            merged=tx("Strap, ring, tongue and bracket are gone into the post, arm and hood: the thickened post (75 mm) reaches the hood skirt.", "Lasche, Ring, Zunge und Halter sind in Pfosten, Arm und Haube aufgegangen: der verdickte Pfosten (75 mm) reicht bis an die Haubenschürze."),
            omitted=tx("Every stroke below the 75 mm feature.", "Jeder Strich unter dem 75-mm-Merkmal."),
            thinElements=tx("The post (60.3 mm) and the arm (40 mm) would be 0.12 and 0.08 mm on paper: both are drawn at 0.15 mm (75 mm), thickened towards each other.", "Pfosten (60,3 mm) und Arm (40 mm) wären auf dem Papier 0,12 und 0,08 mm: beide sind mit 0,15 mm (75 mm) gezeichnet, zueinander hin verdickt."),
            hatching=tx("None.", "Keine."),
            styles=tx("Tones as 1:50; body pen 0.18.", "Töne wie 1:50; Körperstift 0,18."),
            overlays=tx("Regions as 1:50.", "Bereiche wie 1:50."),
        ),
    }[s]
    return d


def vol_decisions(s):
    """🧊 The volume decisions at 1:s."""
    return {
        50: [tx("Body 1:50 for 7.5 mm: augered C20/25 footing 0.30 x 0.60 m with the post bore, CHS 60.3 post with domed cap, three 6 mm clamps, SHS 40 arm with its knuckle and 12 mm pin, 30 x 4 mm half ring and strap, the liner as true 1.25 mm sheet bands (a 70 mm rusted foot ring, five stepped bands of the 15 mm kick dent, the wall), the rolled rim bead, two pressed beads as slender hoops, a 1.25 mm floor declared as sheet, the two 5 mm hinge lugs and the lock tongue, the 1.5 mm hood shell with its flat cone, and the takeaway box.", "Körper 1:50 für 7,5 mm: gebohrtes Fundament C20/25 0,30 x 0,60 m mit Pfostenbohrung, KHP-60,3-Pfosten mit Kuppelkappe, drei 6-mm-Schellen, QHP-40-Arm mit Auge und 12-mm-Bolzen, Halbring und Lasche 30 x 4 mm, der Einsatz aus echten 1,25-mm-Blechbändern (ein 70-mm-Rostfußring, fünf gestufte Bänder der 15-mm-Trittdelle, die Wand), der Rollrand, zwei Sicken als schlanke Reifen, ein 1,25-mm-Boden als Blech deklariert, die zwei 5-mm-Gelenklaschen und die Verschlusszunge, die 1,5-mm-Haubenschale mit flachem Kegel und die Imbissschachtel."), tx("Thermal 1:50: every body solid by reference in its region; structure 1:50: footing, post, arm, strap, ring and liner bands as members, clamps, pin and tongue as joints, a mass marker per loaded bone and the fixed block under the footing base.", "Thermik 1:50: jeder Körper als Verweis in seinem Bereich; Tragwerk 1:50: Fundament, Pfosten, Arm, Lasche, Ring und Einsatzbänder als Stäbe, Schellen, Bolzen und Zunge als Gelenke, je belastetem Knochen eine Massenmarke und der Einspannblock unter der Fundamentsohle.")],
        100: [tx("Body 1:100 for 15 mm: the rim merged into a plain wall to 1.015 m, the dent left to the drawing, the cap into the post; clamps, ring, lugs and pin (6, 4, 5 and 12 mm) are below the feature and go into the post, a strap to the liner and a 20 mm hood bracket block.", "Körper 1:100 für 15 mm: der Rand in eine glatte Wand bis 1,015 m verschmolzen, die Delle der Zeichnung überlassen, die Kappe im Pfosten; Schellen, Ring, Laschen und Bolzen (6, 4, 5 und 12 mm) liegen unter dem Merkmal und gehen in Pfosten, eine Lasche zum Einsatz und einen 20-mm-Haubenhalteblock auf."), tx("Thermal 1:100 by reference; structure 1:100: members as at 1:50 without the ring, the pivot as a 15 mm pin joint, masses and the footing block.", "Thermik 1:100 als Verweis; Tragwerk 1:100: Stäbe wie 1:50 ohne Ring, das Gelenk als 15-mm-Bolzen, Massen und der Fundamentblock.")],
        200: [tx("Body 1:200 for 30 mm: footing, post, arm, a 40 x 30 mm strap, the liner walls with the 70 mm rust band and the sheet floor, the hood shell, a 30 mm bracket block, the box; the tongue (17 mm) is merged into the strap.", "Körper 1:200 für 30 mm: Fundament, Pfosten, Arm, eine Lasche 40 x 30 mm, die Einsatzwände mit 70-mm-Rostband und Blechboden, die Haubenschale, ein 30-mm-Halteblock, die Schachtel; die Zunge (17 mm) ist in der Lasche aufgegangen."), tx("Thermal 1:200 by reference, only the total area is compared; structure 1:200: members, a 30 mm pin joint, masses and the footing block.", "Thermik 1:200 als Verweis, verglichen wird nur die Gesamtfläche; Tragwerk 1:200: Stäbe, ein 30-mm-Bolzengelenk, Massen und der Fundamentblock.")],
        500: [tx("Body 1:500 for 75 mm: footing, the post at 75 mm (the print floor) reaching the hood skirt, the arm as a 75 mm block under the liner, the liner with a 75 mm rust band and its sheet floor, the hood shell, the box at 75 mm.", "Körper 1:500 für 75 mm: Fundament, der Pfosten mit 75 mm (Druckgrenze) bis an die Haubenschürze, der Arm als 75-mm-Block unter dem Einsatz, der Einsatz mit 75-mm-Rostband und Blechboden, die Haubenschale, die Schachtel mit 75 mm."), tx("Thermal 1:500 by reference, total area only; structure 1:500: the post and the arm keep their true 60.3 mm and 40 mm sections as slender members inside the thickened body, the liner walls, footing and hood as members, a 75 mm pin joint, masses and the footing block.", "Thermik 1:500 als Verweis, nur die Gesamtfläche; Tragwerk 1:500: Pfosten und Arm behalten ihre wahren Querschnitte 60,3 mm und 40 mm als schlanke Stäbe im verdickten Körper, Einsatzwände, Fundament und Haube als Stäbe, ein 75-mm-Bolzengelenk, Massen und der Fundamentblock.")],
    }[s]


def structure_extras(k, total):
    """🏗️ Coarse true-section members, the pivot joint, the mass markers and the support block of scale k."""
    s, f = k.s, FEATURE[k.s]
    if s == 500:
        k.structure = [e for e in k.structure if e["of"] not in ("post", "arm")]
        k.structure.append(rev("st-post-true", "post", ZINC, "body", 0.48, [(0, 0), (0, R_POST), (Y_CAP1 - Y_POST0, R_POST), (Y_CAP1 - Y_POST0, 0)], [1, 0, 0], [0, 0, 0], slender=True, member="post", part="member"))
        au, av = local("carrier", (R_POST + X_ARM1) / 2, (Y_ARM0 + Y_ARM1) / 2)
        k.structure.append(box("st-arm-true", "carrier", ZINC, "body", 0.4, [X_ARM1 - R_POST, 0.04, 0.04], [au, av, 0], slender=True, member="carrier", part="member"))
    if s > 50:
        rj = max(f / 2, R_PIN)
        zj = max(f / 2, Z_PIN)
        k.structure.append(rev("st-pivot", "carrier", ZINC, "detail", 0.2, [(-zj, 0), (-zj, rj), (zj, rj), (zj, 0)], [0, 0, 1], [X_PIVOT, 0, 0], member="carrier", part="joint"))
    mr = {50: 0.02, 100: 0.025, 200: 0.03, 500: 0.04}[s]
    for bone, (m, cx, cy) in total.items():
        u, v = local(bone, cx, cy)
        r = min(mr, BOX["h"] / 2 - 0.001) if bone == "litter" and s < 500 else mr
        k.structure.append(ell("mass-" + bone, bone, CONC if bone == "footing" else (CARD if bone == "litter" else ZINC), [r, r, r], [u, v, 0], member=bone, part="mass"))
    k.structure.append(box("support-footing-base", "footing", CONC, "detail", 0.1, [f, 0.3, 0.3], [0.6 - f / 2, 0, 0], member="footing", part="support", anchor="footing-base"))


def document():
    """📄 The whole asset document."""
    parts, total = masses()
    area = area_estimates()
    reps, vols = {}, {}
    for s in (50, 100, 200, 500):
        k = build(s)
        traces(k)
        structure_extras(k, total)
        hatches = [{"id": "concrete-stipple", "title": tx("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8 if s == 50 else 0.6, "penMm": 0.13}] if s <= 100 else []
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": BANDS[s][0], "maxDenominator": BANDS[s][1]}, "pens": PENS[s], "hatches": hatches, "geometry": {"elevation": k.elev, "front": k.front, "plan": k.plan}, "decisions": rep_decisions(s)}
        vols[str(s)] = {"status": "authored", "denominator": s, "body": k.body, "thermal": k.thermal, "structure": k.structure, "decisions": vol_decisions(s)}
    mass_total = round(sum(t[0] for t in total.values()), 2)
    asset = {
        "id": "used-litter-bin-post",
        "category": "objects",
        "title": tx("Used Post-Mounted Litter Bin", "Gebrauchter Pfostenabfallbehälter"),
        "summary": tx("A 50 l galvanised steel litter bin of the municipal post-mounted type after years in a park: the liner tilts forward on a pivot pin for emptying under a fixed flat-coned rain hood, its foot ring has rusted through the zinc, a kick dent sits low on the front, sticker residue and a felt-tip tag cover the wall, and the 60.3 mm post stands 0.5 m deep in a concrete point footing.", "Ein 50-l-Abfallbehälter aus verzinktem Stahlblech in der kommunalen Pfostenbauweise nach Jahren im Park: der Einsatz kippt zum Leeren an einem Gelenkbolzen nach vorn unter einer festen, flach kegeligen Regenhaube, sein Fußring ist durch das Zink gerostet, unten vorn sitzt eine Trittdelle, Aufkleberreste und ein Filzstift-Tag bedecken die Wand, und der 60,3-mm-Pfosten steht 0,5 m tief in einem Betonpunktfundament."),
        "tags": ["used", "park", "public-space", "waste", "litter-bin", "post-mounted", "galvanised-steel", "rain-hood", "tilting-liner", "concrete-footing", "street-furniture"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES], "notes": tx(f"Catalogue-typical 50 l round post bin: liner 350 mm x 565 mm in 1.25 mm DX51D+Z275, flat-coned hood 430 mm in 1.5 mm sheet, post CHS 60.3 x 3.2 set 0.50 m into a 0.30 x 0.60 m C20/25 point footing, throw-in rim at 1.015 m with a 115 mm gap under the hood. Masses from section x density: footing {total['footing'][0]:.1f} kg, post {total['post'][0]:.2f} kg, carrier {total['carrier'][0]:.2f} kg, liner {total['liner-body'][0]:.2f} kg, hood {total['hood'][0]:.2f} kg, box {total['litter'][0]:.2f} kg. Nothing is measured.", f"Katalogtypischer 50-l-Rundbehälter am Pfosten: Einsatz 350 mm x 565 mm aus 1,25 mm DX51D+Z275, flach kegelige Haube 430 mm aus 1,5-mm-Blech, Pfosten KHP 60,3 x 3,2, 0,50 m tief in einem Punktfundament C20/25 0,30 x 0,60 m, Einwurfrand auf 1,015 m mit 115 mm Spalt unter der Haube. Massen aus Querschnitt x Dichte: Fundament {total['footing'][0]:.1f} kg, Pfosten {total['post'][0]:.2f} kg, Träger {total['carrier'][0]:.2f} kg, Einsatz {total['liner-body'][0]:.2f} kg, Haube {total['hood'][0]:.2f} kg, Schachtel {total['litter'][0]:.2f} kg. Nichts ist gemessen.")},
        "dimensions": {"heightM": round(Y_APEX - Y_FOOT0, 4), "widthM": round(X_BIN + R_HOOD + R_FOOT, 4), "depthM": round(2 * R_HOOD, 4), "massKg": mass_total},
        "materials": materials(),
        "rig": rig(total),
        "volumes": vols,
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": {
            "minimumAirPassageM": 0.002,
            "environment": {"airTempC": 27, "meanRadiantTempC": 31, "skyTempC": 14, "groundTempC": 33, "relativeHumidity": 0.45, "windMS": 1.2, "windDirectionDeg": 180, "solarWM2": 850, "diffuseFraction": 0.15, "pressurePa": 101325},
            "passengers": [], "operating": [], "emitters": [],
            "regions": regions(area),
            "assumptions": [
                tx("Passive weathered surfaces at summer noon in a park: absorbed sun, long-wave exchange with sky and ground, convection and the heat capacity of the sheet; the bin draws no power (0 W) and carries no rating.", "Passive verwitterte Flächen zur Sommermittagszeit im Park: absorbierte Sonne, Langwellenaustausch mit Himmel und Boden, Konvektion und die Wärmekapazität des Blechs; der Behälter nimmt keine Leistung auf (0 W) und trägt keine Nennleistung."),
                tx("Weathered zinc absorbs more and emits more than new zinc (solar absorptance about 0.65 new, 0.78 weathered; emissivity about 0.1 new, 0.28 weathered, Duffie & Beckman, Incropera); the rusted foot ring reads 0.86 and 0.8, so it is the hottest part of the liner in the sun.", "Bewittertes Zink absorbiert und strahlt mehr als neues (solarer Absorptionsgrad etwa 0,65 neu, 0,78 bewittert; Emissionsgrad etwa 0,1 neu, 0,28 bewittert, Duffie & Beckman, Incropera); der verrostete Fußring liegt bei 0,86 und 0,8 und ist in der Sonne der heißeste Teil des Einsatzes."),
                tx("The hood shades the opening and the upper liner; the footing is in soil contact at 14 degC over 90 % of its surface and only its head sees the sky. Fractions are scenario data.", "Die Haube beschattet Öffnung und oberen Einsatz; das Fundament steht auf 90 % seiner Fläche bei 14 °C im Erdreich, nur sein Kopf sieht den Himmel. Die Anteile sind Szenariodaten."),
            ],
        },
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["footing-base"], "assumptions": [
            tx("The post is clamped in the point footing and the soil holds the footing: the restraint is idealised as one fixed support at the footing base; the bedding along the 0.6 m depth is not resolved.", "Der Pfosten ist im Punktfundament eingespannt und das Erdreich hält das Fundament: die Einspannung ist als ein festes Auflager an der Fundamentsohle idealisiert; die Bettung über die 0,6 m Tiefe ist nicht aufgelöst."),
            tx("The liner hangs on the pivot pin and is locked by its tongue on the half ring; in service it is one rigid body with the carrier, tipped out it is carried by the pin alone.", "Der Einsatz hängt am Gelenkbolzen und ist mit seiner Zunge am Halbring verriegelt; im Einsatz bildet er mit dem Träger einen starren Körper, ausgekippt trägt ihn allein der Bolzen."),
            tx("Wind after DIN EN 1991-1-4 with the German annex, zone 2 inland, peak pressure 0.65 kN/m2, force coefficients 0.7 (short cylinder), 1.0 (hood) and 1.2 (post); litter and hand loads are authored estimates.", "Wind nach DIN EN 1991-1-4 mit deutschem Anhang, Zone 2 Binnenland, Böengeschwindigkeitsdruck 0,65 kN/m², Kraftbeiwerte 0,7 (kurzer Zylinder), 1,0 (Haube) und 1,2 (Pfosten); Abfall- und Handlasten sind Schätzungen."),
        ], "clinical": False},
        "presentation": {"defaultPose": "in-service", "defaultClip": "maintenance", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    out = HERE = __import__("pathlib").Path(__file__).parent / "🔣️.json"
    out.write_text(json.dumps(document(), ensure_ascii=False, indent=1) + "\n")
    print("[DEBUG] wrote", out)
