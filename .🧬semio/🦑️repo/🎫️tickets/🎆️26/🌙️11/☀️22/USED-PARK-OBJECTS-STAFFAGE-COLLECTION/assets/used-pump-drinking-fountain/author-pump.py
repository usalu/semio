"""🚰 Authoring script of `used-pump-drinking-fountain` (run it to regenerate `🔣️.json`): a cast-iron lever pump (Schwengelpumpe) on its stand, spouting into a granite trough on a concrete bedding slab.

World rest metres: x ahead (the spout direction), y up, z left; origin on the pump axis at ground level. Every 2D shape and 3D solid of every scale is generated from the one set of numbers below.

@see ../../brief.md — the brief of the wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Front and plan views", "Volume"
"""
import json
import math
import pathlib

X_SLAB = (-0.22, 1.10)
Z_SLAB = 0.36
Y_SLAB0 = -0.25

R_FLANGE = 0.15
R_PIPE = 0.02415
T_PIPE = 0.00368
Y_COLUMN1 = 0.66
Y_HEAD1 = 0.975
R_HEAD = 0.065
R_BORE = 0.040
R_HFLANGE = 0.09
Y_CAP1 = 1.02
R_CAP = 0.075
Y_GLAND1 = 1.04
R_GLAND = 0.022
R_ROD = 0.008
PIVOT = (-0.10, 1.10)
R_PIN = 0.008
Z_PIN = 0.036
LEVER_DEG = 165.0
LEVER_STROKE_DEG = 35.0
U_ROD_PIN = -0.10 / math.cos(math.radians(LEVER_DEG)) * -1.0
U_GRIP = (0.50, 0.64)
Z_LEVER = 0.015
CHEEK_Z = (0.018, 0.030)
Y_ROD0 = 0.78
U_ROD_EYE = PIVOT[1] + U_ROD_PIN * math.sin(math.radians(LEVER_DEG)) - Y_ROD0
HEAD_LIFT = 0.30

X_TROUGH = (0.15, 1.05)
Z_TROUGH = 0.30
Y_RIM = 0.50
T_WALL = 0.08
Y_FLOOR = 0.10
Y_NOTCH = 0.43
Z_NOTCH = 0.03
CHIP = 0.045
Y_WATER = 0.42
GAP_WATER = 0.0005

X_JET = 0.30
Y_MOUTH = 0.70
JET_RISE = Y_MOUTH - 0.415
JET_SEG = 0.095
JET_R = (0.0125, 0.0115, 0.0105, 0.010)

T_ALGAE = 0.002

BONES = {
    "footing": {"origin": (0.0, 0.0), "theta": -90.0},
    "column": {"origin": (0.0, 0.0), "theta": 90.0},
    "head": {"origin": (0.0, Y_COLUMN1), "theta": 90.0},
    "lever": {"origin": PIVOT, "theta": LEVER_DEG},
    "rod": {"origin": (0.0, Y_ROD0), "theta": 90.0},
    "trough": {"origin": (X_TROUGH[0], 0.0), "theta": 0.0},
    "water": {"origin": (X_TROUGH[0] + T_WALL, Y_FLOOR), "theta": 0.0},
}
JETS = []
for _k in range(3):
    _top = Y_MOUTH - JET_SEG * _k
    JETS.append({"id": f"jet-{_k + 1}", "y0": _top - JET_SEG - JET_RISE, "r1": JET_R[_k], "r0": JET_R[_k + 1]})
    BONES[f"jet-{_k + 1}"] = {"origin": (X_JET, _top - JET_SEG - JET_RISE), "theta": 90.0}

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
SEGMENTS = {50: 48, 100: 36, 200: 24, 500: 16}
PAINT, RUST, GRIP, GRANITE, ALGAE, WATER, CONC = "cast-iron-painted", "cast-iron-rusted", "cast-iron-polished", "granite-weathered", "algae-biofilm", "water", "concrete-weathered"
STEEL, PIPE, PISTON = "steel-rod-greased", "galvanised-pipe", "piston-brass-leather"
REGION_OF = {PAINT: "pump-paint", RUST: "pump-rust", GRIP: "grip-bare", GRANITE: "trough-granite", ALGAE: "algae-film", WATER: "water", CONC: "slab-concrete", STEEL: "pump-rust", PISTON: "pump-rust"}


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


def world(bone, u, v):
    """🌍 Bone-local (u, v) of `bone` to the world rest point (x, y)."""
    ox, oy = BONES[bone]["origin"]
    t = math.radians(BONES[bone]["theta"])
    return (ox + u * math.cos(t) - v * math.sin(t), oy + u * math.sin(t) + v * math.cos(t))


def lpts(bone, points):
    """🔷 World polygon to bone-local points."""
    return [local(bone, x, y) for x, y in points]


def arc(cx, cy, r, a0, a1, n):
    """🌙 Points on a circle from angle a0 to a1 (degrees, inclusive)."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def circle_pts(cx, cy, r, n):
    """⚪ A closed n-gon (without repeating the first point)."""
    return arc(cx, cy, r, 0, 360, n)[:-1]


def tx(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def shape(sid, bone, role, z, filled, tone, geom, material, hatch=None, pen=None, solid=None, region=True):
    """✏️ One authored 2D shape; the region follows the material."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    if region and REGION_OF.get(material):
        out["region"] = REGION_OF[material]
    out["material"] = material
    if hatch:
        out["hatch"] = hatch
    if pen:
        out["penMm"] = pen
    if solid:
        out["solid"] = solid
    return out


def rect(x0, y0, x1, y1):
    """⬛ Rectangle geometry from two corners."""
    return {"kind": "rect", "x": r5(min(x0, x1)), "y": r5(min(y0, y1)), "w": r5(abs(x1 - x0)), "h": r5(abs(y1 - y0))}


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


def rev(sid, bone, material, role, tone, profile, axis=(1, 0, 0), origin=(0, 0, 0), **extra):
    """🌀 A revolve solid."""
    return solid(sid, "revolve", bone, material, role, tone, profile=[[r5(a), r5(b)] for a, b in profile], axis=list(axis), origin=[r5(c) for c in origin], **extra)


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


def mirror_outline(profile):
    """🪞 The side silhouette (offset, height) of a part revolved about a vertical axis from its outer half profile (height, radius) bottom to top."""
    right = [(r, h) for h, r in profile]
    left = [(-r, h) for h, r in reversed(profile)]
    return right + left


def ring_volume(profile):
    """🧮 Volume (m3) and axial centroid of the body revolved from a closed (axial, radial) polygon (Pappus, fan triangulation)."""
    pts = list(profile)
    if pts[0] == pts[-1]:
        pts = pts[:-1]
    else:
        pts = [(pts[0][0], 0.0)] + pts + [(pts[-1][0], 0.0)]
    vol, mom = 0.0, 0.0
    a0, r0 = pts[0]
    for i in range(1, len(pts) - 1):
        (a1, r1), (a2, r2) = pts[i], pts[i + 1]
        area = 0.5 * ((a1 - a0) * (r2 - r0) - (a2 - a0) * (r1 - r0))
        ca, cr = (a0 + a1 + a2) / 3, (r0 + r1 + r2) / 3
        vol += area * cr
        mom += area / 12 * (a0 * r0 + a1 * r1 + a2 * r2 + 9 * ca * cr)
    return abs(2 * math.pi * vol), mom / vol


def poly_area(points):
    """📏 Area and centroid of a simple polygon."""
    a, cx, cy = 0.0, 0.0, 0.0
    n = len(points)
    for i in range(n):
        x0, y0 = points[i]
        x1, y1 = points[(i + 1) % n]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a *= 0.5
    return abs(a), cx / (6 * a), cy / (6 * a)


BASE50 = [(0.0, R_PIPE + 0.001), (0.0, R_FLANGE), (0.022, R_FLANGE), (0.022, 0.11), (0.05, 0.095), (0.10, 0.080), (0.10, 0.063), (0.05, 0.083), (0.034, 0.098), (0.012, 0.098), (0.012, R_PIPE + 0.001), (0.0, R_PIPE + 0.001)]
BASE_OUT = {50: [(0.0, R_FLANGE), (0.022, R_FLANGE), (0.022, 0.11), (0.05, 0.095), (0.10, 0.080)], 100: [(0.0, R_FLANGE), (0.022, R_FLANGE), (0.022, 0.11), (0.10, 0.080)], 200: [(0.0, R_FLANGE), (0.03, R_FLANGE), (0.03, 0.11), (0.10, 0.08)], 500: [(0.0, R_FLANGE), (0.04, R_FLANGE), (0.10, 0.08)]}
SHAFT50 = [(0.10, 0.075), (0.615, 0.066), (0.63, R_HFLANGE), (Y_COLUMN1, R_HFLANGE), (Y_COLUMN1, 0.054), (0.615, 0.054), (0.10, 0.063), (0.10, 0.075)]
SHAFT_OUT = {50: [(0.10, 0.075), (0.615, 0.066), (0.63, R_HFLANGE), (Y_COLUMN1, R_HFLANGE)], 100: [(0.10, 0.075), (0.63, 0.066), (0.63, R_HFLANGE), (Y_COLUMN1, R_HFLANGE)], 200: [(0.10, 0.075), (0.63, 0.066), (0.63, R_HFLANGE), (Y_COLUMN1, R_HFLANGE)], 500: [(0.10, 0.078), (0.585, 0.07), (0.585, R_HFLANGE), (Y_COLUMN1, R_HFLANGE)]}
HEAD50 = [(0.0, R_BORE), (0.0, R_HFLANGE), (0.03, R_HFLANGE), (0.03, R_HEAD), (Y_HEAD1 - Y_COLUMN1, R_HEAD), (Y_HEAD1 - Y_COLUMN1, R_BORE), (0.0, R_BORE)]
CAP50 = [(Y_HEAD1 - Y_COLUMN1, 0.0115), (Y_HEAD1 - Y_COLUMN1, R_CAP), (1.012 - Y_COLUMN1, R_CAP), (Y_CAP1 - Y_COLUMN1, 0.067), (Y_CAP1 - Y_COLUMN1, 0.0115), (Y_HEAD1 - Y_COLUMN1, 0.0115)]
BANDS = ((0.715, 0.737), (0.945, 0.967))
R_BAND = 0.073


def head_outline(s):
    """🎩 Outer half profile (world height, radius) of head body and cap at scale s, bottom to top, without the gland."""
    if s == 500:
        return [(Y_COLUMN1, 0.075), (Y_CAP1, 0.075)]
    if s == 200:
        return [(Y_COLUMN1, R_HFLANGE), (0.69, R_HFLANGE), (0.69, 0.066), (Y_HEAD1, 0.066), (Y_HEAD1, R_CAP), (Y_CAP1, R_CAP)]
    r = 0.066 if s == 100 else R_HEAD
    return [(Y_COLUMN1, R_HFLANGE), (0.69, R_HFLANGE), (0.69, r), (Y_HEAD1, r), (Y_HEAD1, R_CAP), (1.012 if s == 50 else Y_CAP1, R_CAP)] + ([(Y_CAP1, 0.067)] if s == 50 else [])


def spout_parts(s):
    """🚿 Side profiles (world x, y) of the spout body and its rusted, limescaled lip at scale s, and the lateral half width."""
    yb = lambda x: 0.745 - 0.045 * (x - 0.045) / (0.28 - 0.045)
    yt = lambda x: 0.825 - 0.085 * (x - 0.045) / (0.32 - 0.045)
    xs = 0.245
    if s == 500:
        return [(0.045, 0.70), (xs, 0.70), (xs, 0.80), (0.045, 0.80)], [(xs, 0.70), (0.32, 0.70), (0.32, 0.775), (xs, 0.775)], 0.0375
    body = [(0.045, 0.745), (xs, yb(xs)), (xs, yt(xs)), (0.045, 0.825)]
    lip = [(xs, yb(xs)), (0.28, Y_MOUTH), (0.32, Y_MOUTH), (0.32, 0.74), (xs, yt(xs))]
    return body, lip, 0.03


def cheek_profile(s):
    """🦻 Side profile (world x, y) of the lever bracket cheek(s) at scale s, rounded about the pivot."""
    if s == 500:
        return [(-0.135, Y_CAP1), (-0.06, Y_CAP1), (-0.06, 1.135), (-0.135, 1.135)]
    n = {50: 10, 100: 6, 200: 4}[s]
    r = 0.026 if s < 200 else 0.03
    pts = [(-0.066, Y_CAP1), (-0.030, Y_CAP1), (-0.040, 1.06), (PIVOT[0] + r, PIVOT[1])]
    pts += arc(PIVOT[0], PIVOT[1], r, 0, 250, n)[1:]
    return pts


def lever_profile(s):
    """🦾 Side profile of the lever bar in the lever frame (u along the lever towards the grip, v its normal) at scale s."""
    if s == 500:
        return [(-0.12, -0.0375), (U_GRIP[0], -0.0375), (U_GRIP[0], 0.0375), (-0.12, 0.0375)]
    if s == 200:
        return [(-0.12, -0.018), (U_GRIP[0], -0.018), (U_GRIP[0], 0.018), (-0.12, 0.018)]
    if s == 100:
        return [(-0.12, -0.017), (0.0, -0.022), (U_GRIP[0], -0.016), (U_GRIP[0], 0.016), (0.0, 0.022), (-0.12, 0.017)]
    eye = arc(U_ROD_PIN, 0.0, 0.017, 90, 270, 8)
    bottom = [(-0.06, -0.019), (-0.024, -0.021)] + arc(0.0, 0.0, 0.024, -120, -60, 4)[1:-1] + [(0.024, -0.021), (0.10, -0.019), (U_GRIP[0], -0.015)]
    top = [(U_GRIP[0], 0.015), (0.10, 0.019), (0.024, 0.021)] + arc(0.0, 0.0, 0.024, 60, 120, 4)[1:-1] + [(-0.024, 0.021), (-0.06, 0.019)]
    return eye + bottom + top


def grip_profile(s):
    """✊ Half profile (u along the lever, radius) of the turned grip at scale s."""
    if s == 50:
        return [(U_GRIP[0], 0.0), (U_GRIP[0], 0.015), (0.52, 0.0175), (0.60, 0.0185), (0.625, 0.017), (0.637, 0.012), (U_GRIP[1], 0.005), (U_GRIP[1], 0.0)]
    if s == 100:
        return [(U_GRIP[0], 0.0), (U_GRIP[0], 0.016), (0.60, 0.0185), (0.632, 0.014), (U_GRIP[1], 0.0)]
    return [(U_GRIP[0], 0.0), (U_GRIP[0], 0.0175), (U_GRIP[1], 0.0175), (U_GRIP[1], 0.0)]


def grip_side(prof):
    """✊ Closed side outline (u, v) of the grip from its half profile."""
    pts = [(u, r) for u, r in prof if r > 0]
    return [(prof[0][0], 0.0)] + pts + [(prof[-1][0], 0.0)] + [(u, -r) for u, r in reversed(pts)]


def trough_c(chip, notch):
    """🪨 The C-shaped plan (w, u) of the upper trough rim with the overflow notch through the front wall and the chipped front-right corner."""
    L = X_TROUGH[1] - X_TROUGH[0]
    W = Z_TROUGH
    iw, u0, u1 = W - T_WALL, T_WALL, L - T_WALL
    out = [(Z_NOTCH, L), (W, L), (W, 0.0), (-W, 0.0)]
    out += [(-W, L - chip), (-W + chip, L)] if chip else [(-W, L)]
    if notch:
        out += [(-Z_NOTCH, L), (-Z_NOTCH, u1), (-iw, u1), (-iw, u0), (iw, u0), (iw, u1), (Z_NOTCH, u1)]
    return out


class Scale:
    """📏 Collects the 2D shapes and 3D solids of one authored scale, built from the one set of numbers above."""

    def __init__(self, s):
        self.s = s
        self.n = SEGMENTS[s]
        self.elev, self.front, self.plan = [], [], []
        self.body, self.thermal, self.structure = [], [], []

    def add(self, sol, member=None, part=None, thermal=True, **sx):
        """➕ A body solid with its thermal reference (region from its material) and optionally its structural reference."""
        self.body.append(sol)
        if thermal:
            self.thermal.append({"id": "th-" + sol["id"], "of": sol["id"], "region": REGION_OF[sol["material"]]})
        if member:
            self.structure.append({"id": "st-" + sol["id"], "of": sol["id"], "member": member, "part": part, **sx})

    def views(self, sid, bone, role, z, tone, mat, elev=None, front=None, plan=None, hatch=None, solid_w=None, zp=None):
        """🖼️ The same part in the three views; `solid_w` = (z0, z1) draws front and plan as the extrusion of the elevation profile."""
        if elev is not None:
            self.elev.append(shape(sid, bone, role, z, True, tone, elev, mat, hatch))
        if solid_w is not None:
            sol = {"kind": "extruded", "sweep": "w", "z0": r5(solid_w[0]), "z1": r5(solid_w[1])}
            self.front.append(shape(sid, bone, role, z, True, tone, elev, mat, hatch, solid=sol))
            self.plan.append(shape(sid, bone, role, zp if zp is not None else z, True, tone, elev, mat, hatch, solid=sol))
            return
        if front is not None:
            self.front.append(shape(sid, bone, role, z, True, tone, front, mat, hatch))
        if plan is not None:
            self.plan.append(shape(sid, bone, role, zp if zp is not None else z, True, tone, plan, mat, hatch))


def vertical_part(k, sid, bone, mat, tone, outer, z, zp, solid_profile, hatch=None, plan_r=None, x_axis=0.0, member=None, part="member", thermal=True):
    """🗼 A part revolved about a vertical bone: its elevation and front outline, its plan circle and its revolve solid (axial = height above the bone origin)."""
    oy = BONES[bone]["origin"][1]
    side = mirror_outline(outer)
    k.views(sid, bone, "body", z, tone, mat, elev=poly(lpts(bone, [(x_axis + a, h) for a, h in side])), front=poly([(a, h) for a, h in side]), plan=ellipse(x_axis, 0, plan_r or max(r for _, r in outer), plan_r or max(r for _, r in outer)), hatch=hatch, zp=zp)
    k.add(rev(sid, bone, mat, "body", tone, [(a - (oy if solid_profile is outer else 0.0), r) for a, r in solid_profile], (1, 0, 0), (0, 0, 0)), member, part, thermal)


def build(s):
    """🏗️ All shapes and solids of scale 1:s."""
    k = Scale(s)
    n = k.n
    f = FEATURE[s]
    fine = s == 50
    conc_h = "concrete-dots" if s <= 100 else None
    gran_h = "granite-stipple" if s <= 100 else None

    k.views("slab", "footing", "body", 1, 0.66, CONC, elev=poly(lpts("footing", [(X_SLAB[0], Y_SLAB0), (X_SLAB[1], Y_SLAB0), (X_SLAB[1], 0.0), (X_SLAB[0], 0.0)])), front=rect(-Z_SLAB, Y_SLAB0, Z_SLAB, 0.0), plan=rect(X_SLAB[0], -Z_SLAB, X_SLAB[1], Z_SLAB), hatch=conc_h, zp=0.5)
    cu, cv = local("footing", sum(X_SLAB) / 2, Y_SLAB0 / 2)
    k.add(box("slab", "footing", CONC, "body", 0.66, [-Y_SLAB0, X_SLAB[1] - X_SLAB[0], 2 * Z_SLAB], [cu, cv, 0]), "footing", "member")

    base_out = BASE_OUT[s]
    if s <= 100:
        base_solid = [(a, r) for a, r in BASE50] if fine else [(0.0, R_PIPE + 0.001), (0.0, R_FLANGE), (0.022, R_FLANGE), (0.022, 0.11), (0.10, 0.080), (0.10, 0.065), (0.03, 0.098), (0.015, 0.098), (0.015, R_PIPE + 0.001), (0.0, R_PIPE + 0.001)]
    else:
        base_solid = [(0.0, 0.0)] + base_out + [(0.10, 0.0)]
    side = mirror_outline(base_out)
    k.views("column-foot", "column", "body", 2, 0.3, RUST, elev=poly(lpts("column", side)), front=poly(side), plan=ellipse(0, 0, R_FLANGE, R_FLANGE), zp=2)
    k.add(rev("column-foot", "column", RUST, "body", 0.3, base_solid), "column", "member")

    shaft_out = SHAFT_OUT[s]
    if s == 50:
        shaft_solid = SHAFT50
    elif s == 100:
        shaft_solid = [(0.10, 0.075), (0.63, 0.066), (0.63, R_HFLANGE), (Y_COLUMN1, R_HFLANGE), (Y_COLUMN1, 0.054), (0.10, 0.063), (0.10, 0.075)]
    else:
        shaft_solid = [(shaft_out[0][0], 0.0)] + shaft_out + [(Y_COLUMN1, 0.0)]
    side = mirror_outline(shaft_out)
    k.views("column-shaft", "column", "body", 2.2, 0.42, PAINT, elev=poly(lpts("column", side)), front=poly(side), plan=ellipse(0, 0, R_HFLANGE, R_HFLANGE), zp=2.2)
    k.add(rev("column-shaft", "column", PAINT, "body", 0.42, shaft_solid), "column", "member")

    if fine:
        k.body.append(rev("riser-pipe", "column", PIPE, "detail", 0.3, [(Y_SLAB0, R_PIPE - T_PIPE), (Y_SLAB0, R_PIPE), (0.69, R_PIPE), (0.69, R_PIPE - T_PIPE), (Y_SLAB0, R_PIPE - T_PIPE)]))
        k.body.append(box("anchor-nut", "column", RUST, "detail", 0.25, [0.013, 0.024, 0.024], [0.0285, -0.0919, 0.0919], repeat={"count": 4, "rotationRad": r5(math.pi / 2), "rotationAxis": [1, 0, 0], "origin": [0, 0, 0]}))
        k.thermal.append({"id": "th-anchor-nut", "of": "anchor-nut", "region": "pump-rust"})
        k.structure.append({"id": "st-anchor-nut", "of": "anchor-nut", "member": "column", "part": "joint"})
        for i, (cx, cz) in enumerate(((0.0919, 0.0919), (-0.0919, 0.0919), (-0.0919, -0.0919), (0.0919, -0.0919))):
            k.plan.append(shape(f"anchor-nut-{i}", "column", "detail", 2.4, True, 0.25, rect(cx - 0.012, cz - 0.012, cx + 0.012, cz + 0.012), RUST))
        for i, c in enumerate((0.0919, -0.0919)):
            k.elev.append(shape(f"anchor-nut-{i}", "column", "detail", 2.4, True, 0.25, poly(lpts("column", [(c - 0.012, 0.022), (c + 0.012, 0.022), (c + 0.012, 0.035), (c - 0.012, 0.035)])), RUST))
            k.front.append(shape(f"anchor-nut-{i}", "column", "detail", 2.4, True, 0.25, rect(c - 0.012, 0.022, c + 0.012, 0.035), RUST))

    hout = head_outline(s)
    if s == 50:
        hsolid = HEAD50
    elif s == 100:
        hsolid = [(0.0, R_BORE), (0.0, R_HFLANGE), (0.03, R_HFLANGE), (0.03, 0.066), (Y_HEAD1 - Y_COLUMN1, 0.066), (Y_HEAD1 - Y_COLUMN1, R_BORE), (0.0, R_BORE)]
    else:
        hsolid = [(0.0, 0.0)] + [(h - Y_COLUMN1, r) for h, r in hout] + [(Y_CAP1 - Y_COLUMN1, 0.0)]
    body_out = [p for p in hout if p[0] <= Y_HEAD1] if s <= 100 else hout
    side = mirror_outline(body_out)
    k.views("head-body", "head", "body", 3, 0.45, PAINT, elev=poly(lpts("head", side)), front=poly(side), plan=ellipse(0, 0, R_HFLANGE, R_HFLANGE), zp=3)
    k.add(rev("head-body", "head", PAINT, "body", 0.45, hsolid), "head", "member")
    if s <= 100:
        cap_out = [p for p in hout if p[0] >= Y_HEAD1 - 1e-9]
        cap_solid = CAP50 if fine else [(Y_HEAD1 - Y_COLUMN1, R_ROD + 0.0005), (Y_HEAD1 - Y_COLUMN1, R_CAP), (Y_CAP1 - Y_COLUMN1, R_CAP), (Y_CAP1 - Y_COLUMN1, R_ROD + 0.0005), (Y_HEAD1 - Y_COLUMN1, R_ROD + 0.0005)]
        side = mirror_outline(cap_out if fine else [(Y_HEAD1, R_CAP), (Y_CAP1, R_CAP)])
        k.views("cap", "head", "body", 3.4, 0.5, PAINT, elev=poly(lpts("head", side)), front=poly(side), plan=ellipse(0, 0, R_CAP, R_CAP), zp=3.4)
        k.add(rev("cap", "head", PAINT, "body", 0.5, cap_solid), "head", "member")
    if fine:
        for i, (y0, y1) in enumerate(BANDS):
            prof = [(y0 - Y_COLUMN1, R_HEAD - 0.002), (y0 - Y_COLUMN1, R_BAND), (y1 - Y_COLUMN1, R_BAND), (y1 - Y_COLUMN1, R_HEAD - 0.002), (y0 - Y_COLUMN1, R_HEAD - 0.002)]
            side = [(R_BAND, y0), (R_BAND, y1), (-R_BAND, y1), (-R_BAND, y0)]
            k.views(f"band-{i}", "head", "body", 3.2, 0.5, PAINT, elev=poly(lpts("head", side)), front=poly(side), plan=ellipse(0, 0, R_BAND, R_BAND), zp=3.2)
            k.add(rev(f"band-{i}", "head", PAINT, "body", 0.5, prof, slender=True))
        k.body.append(box("flange-bolt", "head", PAINT, "detail", 0.35, [0.008, 0.016, 0.016], [0.034, -0.0552, 0.0552], repeat={"count": 4, "rotationRad": r5(math.pi / 2), "rotationAxis": [1, 0, 0], "origin": [0, 0, 0]}))
        k.thermal.append({"id": "th-flange-bolt", "of": "flange-bolt", "region": "pump-paint"})
        k.structure.append({"id": "st-flange-bolt", "of": "flange-bolt", "member": "head", "part": "joint"})
        for i, (cx, cz) in enumerate(((0.0552, 0.0552), (-0.0552, 0.0552), (-0.0552, -0.0552), (0.0552, -0.0552))):
            k.plan.append(shape(f"flange-bolt-{i}", "head", "detail", 3.05, True, 0.35, rect(cx - 0.008, cz - 0.008, cx + 0.008, cz + 0.008), PAINT))
        for i, c in enumerate((0.0552, -0.0552)):
            k.elev.append(shape(f"flange-bolt-{i}", "head", "detail", 3.05, True, 0.35, poly(lpts("head", [(c - 0.008, 0.69), (c + 0.008, 0.69), (c + 0.008, 0.698), (c - 0.008, 0.698)])), PAINT))
            k.front.append(shape(f"flange-bolt-{i}", "head", "detail", 3.05, True, 0.35, rect(c - 0.008, 0.69, c + 0.008, 0.698), PAINT))
        gl = [(Y_CAP1 - Y_COLUMN1, R_ROD + 0.0005), (Y_CAP1 - Y_COLUMN1, R_GLAND), (Y_GLAND1 - Y_COLUMN1, R_GLAND - 0.003), (Y_GLAND1 - Y_COLUMN1, R_ROD + 0.0005), (Y_CAP1 - Y_COLUMN1, R_ROD + 0.0005)]
        side = [(R_GLAND, Y_CAP1), (R_GLAND - 0.003, Y_GLAND1), (-(R_GLAND - 0.003), Y_GLAND1), (-R_GLAND, Y_CAP1)]
        k.views("gland", "head", "body", 3.6, 0.3, STEEL, elev=poly(lpts("head", side)), front=poly(side), plan=ellipse(0, 0, R_GLAND, R_GLAND), zp=3.6)
        k.add(rev("gland", "head", STEEL, "body", 0.3, gl), "head", "joint")

    body, lip, hw = spout_parts(s)
    for sid, prof, mat, tone in (("spout", body, PAINT, 0.45), ("spout-lip", lip, RUST, 0.3)):
        lp = lpts("head", prof)
        k.views(sid, "head", "body", 3.5, tone, mat, elev=poly(lp), solid_w=(-hw, hw), zp=3.5)
        k.add(ext(sid, "head", mat, "body", tone, lp, "xy", -hw, hw), "head", "member")

    cp = lpts("head", cheek_profile(s))
    if fine:
        for side_id, (z0, z1) in (("cheek-l", CHEEK_Z), ("cheek-r", (-CHEEK_Z[1], -CHEEK_Z[0]))):
            k.views(side_id, "head", "body", 3.7 if side_id == "cheek-r" else 3.3, 0.45, PAINT, elev=poly(cp), solid_w=(z0, z1), zp=3.7)
            k.add(ext(side_id, "head", PAINT, "body", 0.45, cp, "xy", z0, z1), "head", "member")
    else:
        hz = 0.0375 if s == 500 else CHEEK_Z[1]
        k.views("bracket", "head", "body", 3.7, 0.45, PAINT, elev=poly(cp), solid_w=(-hz, hz), zp=3.7)
        k.add(ext("bracket", "head", PAINT, "body", 0.45, cp, "xy", -hz, hz), "head", "member")
    if s <= 100:
        pu, pv = local("head", *PIVOT)
        k.views("pivot-pin", "head", "detail", 3.8, 0.2, STEEL, elev=ellipse(pu, pv, R_PIN, R_PIN), front=rect(-Z_PIN, PIVOT[1] - R_PIN, Z_PIN, PIVOT[1] + R_PIN), plan=rect(PIVOT[0] - R_PIN, -Z_PIN, PIVOT[0] + R_PIN, Z_PIN), zp=4.2)
        k.add(rev("pivot-pin", "head", STEEL, "detail", 0.2, [(-Z_PIN, 0), (-Z_PIN, R_PIN), (Z_PIN, R_PIN), (Z_PIN, 0)], (0, 0, 1), (pu, pv, 0)), "lever", "joint")

    lp = lever_profile(s)
    zl = 0.0375 if s == 500 else (0.016 if s == 200 else Z_LEVER)
    k.views("lever", "lever", "body", 4, 0.48, PAINT, elev=poly(lp), solid_w=(-zl, zl), zp=4)
    k.add(ext("lever", "lever", PAINT, "body", 0.48, lp, "xy", -zl, zl), "lever", "member")
    if s == 500:
        gp = [(U_GRIP[0], -0.0375), (U_GRIP[1], -0.0375), (U_GRIP[1], 0.0375), (U_GRIP[0], 0.0375)]
        k.views("grip", "lever", "body", 4.1, 0.7, GRIP, elev=poly(gp), solid_w=(-0.0375, 0.0375), zp=4.1)
        k.add(ext("grip", "lever", GRIP, "body", 0.7, gp, "xy", -0.0375, 0.0375), "lever", "member")
    else:
        gprof = grip_profile(s)
        gr = max(r for _, r in gprof)
        k.views("grip", "lever", "body", 4.1, 0.7, GRIP, elev=poly(grip_side(gprof)), solid_w=(-gr, gr), zp=4.1)
        k.add(rev("grip", "lever", GRIP, "body", 0.7, gprof, (1, 0, 0), (0, 0, 0)), "lever", "member")
    if fine:
        pin = circle_pts(U_ROD_PIN, 0.0, 0.006, 12)
        k.views("rod-pin", "lever", "detail", 4.3, 0.2, STEEL, elev=poly(pin), solid_w=(-0.03, 0.03), zp=4.3)
        k.add(rev("rod-pin", "lever", STEEL, "detail", 0.2, [(-0.03, 0), (-0.03, 0.006), (0.03, 0.006), (0.03, 0)], (0, 0, 1), (U_ROD_PIN, 0, 0)), "lever", "joint")

    if s <= 100:
        top = U_ROD_EYE - 0.02
        k.views("rod", "rod", "body", 3.9, 0.32, STEEL, elev=poly(lpts("rod", [(R_ROD, Y_ROD0 + 0.04), (R_ROD, Y_ROD0 + top), (-R_ROD, Y_ROD0 + top), (-R_ROD, Y_ROD0 + 0.04)])), front=rect(-R_ROD, Y_ROD0 + 0.04, R_ROD, Y_ROD0 + top), plan=ellipse(0, 0, R_ROD, R_ROD), zp=3.9)
        k.add(rev("rod", "rod", STEEL, "body", 0.32, [(0.04, 0), (0.04, R_ROD), (top, R_ROD), (top, 0)]), "rod", "member")
    if s <= 200:
        u0, u1 = U_ROD_EYE - 0.02, U_ROD_EYE + 0.02
        cz = 0.025
        k.views("clevis", "rod", "body", 4.4, 0.32, STEEL, elev=rect(u0, -0.015, u1, 0.015), front=rect(-cz, Y_ROD0 + u0, cz, Y_ROD0 + u1), plan=rect(-0.015, -cz, 0.015, cz), zp=4.4)
        k.add(box("clevis", "rod", STEEL, "body", 0.32, [u1 - u0, 0.03, 2 * cz], [(u0 + u1) / 2, 0, 0]), "rod", "joint")
    if fine:
        k.body.append(rev("piston", "rod", PISTON, "detail", 0.4, [(0.0, 0.0), (0.0, R_BORE - 0.0005), (0.04, R_BORE - 0.0005), (0.04, 0.0)]))

    L = X_TROUGH[1] - X_TROUGH[0]
    chip = CHIP if s <= 200 else 0.0
    notch = s <= 200
    k.views("trough", "trough", "body", 5, 0.7, GRANITE, elev=rect(0.0, 0.0, L, Y_RIM), front=rect(-Z_TROUGH, 0.0, Z_TROUGH, Y_RIM), plan=poly([(X_TROUGH[0], -Z_TROUGH), (X_TROUGH[1] - chip, -Z_TROUGH), (X_TROUGH[1], -Z_TROUGH + chip), (X_TROUGH[1], Z_TROUGH), (X_TROUGH[0], Z_TROUGH)] if chip else [(X_TROUGH[0], -Z_TROUGH), (X_TROUGH[1], -Z_TROUGH), (X_TROUGH[1], Z_TROUGH), (X_TROUGH[0], Z_TROUGH)]), hatch=gran_h, zp=5)
    ring_out = [(-Z_TROUGH, 0.0), (Z_TROUGH, 0.0), (Z_TROUGH, L), (-Z_TROUGH, L)]
    ring_in = [(-(Z_TROUGH - T_WALL), T_WALL), (Z_TROUGH - T_WALL, T_WALL), (Z_TROUGH - T_WALL, L - T_WALL), (-(Z_TROUGH - T_WALL), L - T_WALL)]
    k.add(box("trough-floor", "trough", GRANITE, "body", 0.7, [L, Y_FLOOR, 2 * Z_TROUGH], [L / 2, Y_FLOOR / 2, 0]), "trough", "member")
    if notch:
        k.add(ext("trough-walls", "trough", GRANITE, "body", 0.7, ring_out, "zx", Y_FLOOR, Y_NOTCH, holes=[ring_in]), "trough", "member")
        k.add(ext("trough-rim", "trough", GRANITE, "body", 0.7, trough_c(chip, True), "zx", Y_NOTCH, Y_RIM), "trough", "member")
    else:
        k.add(ext("trough-walls", "trough", GRANITE, "body", 0.7, ring_out, "zx", Y_FLOOR, Y_RIM, holes=[ring_in]), "trough", "member")

    al = algae_patch(s)
    k.elev.append(shape("algae", "trough", "body", 5.5, True, 0.38, poly(al), ALGAE))
    k.body.append(ext("algae", "trough", ALGAE, "body", 0.38, al, "xy", -Z_TROUGH - T_ALGAE, -Z_TROUGH, sheet=True))
    k.thermal.append({"id": "th-algae", "of": "algae", "region": "algae-film"})

    wl = L - 2 * T_WALL - 2 * GAP_WATER
    wh = Y_WATER - Y_FLOOR - GAP_WATER
    wz = Z_TROUGH - T_WALL - GAP_WATER
    x0w = X_TROUGH[0] + T_WALL + GAP_WATER
    k.views("water", "water", "body", 4.6, 0.86, WATER, elev=rect(GAP_WATER, GAP_WATER, GAP_WATER + wl, Y_WATER - Y_FLOOR), front=rect(-wz, Y_FLOOR + GAP_WATER, wz, Y_WATER), plan=rect(x0w, -wz, x0w + wl, wz), hatch="water-lines" if s <= 100 else None, zp=5.6)
    k.add(box("water", "water", WATER, "body", 0.86, [wl, wh, 2 * wz], [GAP_WATER + wl / 2, GAP_WATER + wh / 2, 0]))

    if s <= 100:
        for j in JETS:
            r0, r1 = (j["r0"], j["r1"]) if fine else (0.012, 0.012)
            prof = [(0.0, 0.0), (0.0, r0), (JET_SEG + 0.001, r1), (JET_SEG + 0.001, 0.0)]
            y0 = j["y0"]
            k.views(j["id"], j["id"], "body", 4.8, 0.9, WATER, elev=poly([(0.0, r0), (JET_SEG + 0.001, r1), (JET_SEG + 0.001, -r1), (0.0, -r0)]), front=poly([(r0, y0), (r1, y0 + JET_SEG + 0.001), (-r1, y0 + JET_SEG + 0.001), (-r0, y0)]), plan=ellipse(X_JET, 0, r1, r1), zp=3.45)
            k.add(rev(j["id"], j["id"], WATER, "body", 0.9, prof))
    return k


def algae_patch(s):
    """🦠 The patch of wet algae on the shaded right face of the trough (trough frame u, v), ragged at 1:50 and 1:100, generalised coarser."""
    if s >= 200:
        return [(0.02, 0.02), (0.86, 0.02), (0.86, 0.20), (0.55, 0.24), (0.25, 0.20), (0.02, 0.26)]
    return [(0.02, 0.02), (0.86, 0.02), (0.86, 0.17), (0.80, 0.21), (0.70, 0.18), (0.61, 0.24), (0.50, 0.21), (0.41, 0.26), (0.30, 0.22), (0.20, 0.27), (0.10, 0.23), (0.02, 0.28)]


def traces(k):
    """🩹 Traces of use and weather as detail and texture strokes: flaking paint and rust on the pump, sticker residue, limescale at the spout lip, the polished grip, the overflow notch and the chipped corner, a crack, a felt-tip tag, moss along the foot of the trough, chewing gum on the rim, leaves on the water and the wet patch on the slab."""
    s = k.s
    if s > 100:
        if s == 200:
            k.front.append(shape("notch-edge", "trough", "detail", 5.2, False, 0.4, poly([(-Z_NOTCH, Y_RIM), (-Z_NOTCH, Y_NOTCH), (Z_NOTCH, Y_NOTCH), (Z_NOTCH, Y_RIM)], False), GRANITE))
        return
    fine = s == 50
    paint, rust, gran = PAINT, RUST, GRANITE
    k.front.append(shape("notch-edge", "trough", "detail", 5.2, False, 0.4, poly([(-Z_NOTCH, Y_RIM), (-Z_NOTCH, Y_NOTCH), (Z_NOTCH, Y_NOTCH), (Z_NOTCH, Y_RIM)], False), gran))
    k.front.append(shape("chip-edge", "trough", "detail", 5.2, False, 0.4, poly([(-Z_TROUGH + CHIP, Y_RIM), (-Z_TROUGH + CHIP, Y_NOTCH)], False), gran))
    k.plan.append(shape("notch-edge", "trough", "detail", 5.2, False, 0.4, path(f"M {X_TROUGH[1] - T_WALL} {-Z_NOTCH} L {X_TROUGH[1]} {-Z_NOTCH} M {X_TROUGH[1] - T_WALL} {Z_NOTCH} L {X_TROUGH[1]} {Z_NOTCH}"), gran))
    k.plan.append(shape("rim-inner", "trough", "detail", 5.2, False, 0.4, poly([(X_TROUGH[0] + T_WALL, -(Z_TROUGH - T_WALL)), (X_TROUGH[1] - T_WALL, -(Z_TROUGH - T_WALL)), (X_TROUGH[1] - T_WALL, Z_TROUGH - T_WALL), (X_TROUGH[0] + T_WALL, Z_TROUGH - T_WALL)], True), gran))
    k.elev.append(shape("crack", "trough", "detail", 5.6, False, 0.3, path("M 0.84 0.5 L 0.80 0.43 L 0.815 0.36 L 0.78 0.29"), gran))
    k.elev.append(shape("tag", "trough", "texture", 5.6, False, 0.25, path("M 0.30 0.43 C 0.33 0.47 0.36 0.40 0.39 0.45 C 0.42 0.49 0.45 0.41 0.48 0.46 M 0.52 0.42 L 0.55 0.47 L 0.58 0.42"), gran))
    k.elev.append(shape("moss", "trough", "texture", 5.6, False, 0.3, path("M 0.0 0.012 L 0.05 0.02 L 0.1 0.01 L 0.16 0.022 L 0.24 0.012 L 0.32 0.02 L 0.42 0.01 L 0.52 0.02 L 0.64 0.012 L 0.76 0.02 L 0.9 0.012"), ALGAE))
    k.elev.append(shape("flaking", "column", "texture", 2.6, False, 0.3, path("M 0.22 -0.02 L 0.25 -0.045 L 0.28 -0.03 L 0.26 -0.005 Z M 0.40 0.01 L 0.43 -0.02 L 0.45 0.005 Z M 0.52 -0.05 L 0.55 -0.02 L 0.57 -0.04 Z"), paint))
    k.elev.append(shape("sticker", "column", "texture", 2.6, False, 0.3, rect(0.30, 0.01, 0.36, 0.05), paint))
    k.elev.append(shape("lever-wear", "lever", "texture", 4.6, False, 0.3, path("M 0.52 -0.008 L 0.61 -0.009 M 0.53 0.006 L 0.60 0.007"), GRIP))
    lu, lv = local("head", 0.30, Y_MOUTH + 0.005)
    k.elev.append(shape("limescale", "head", "texture", 3.9, False, 0.25, path(f"M {lu} {lv - 0.02} L {lu - 0.004} {lv - 0.01} L {lu} {lv} L {lu - 0.005} {lv + 0.01}"), rust))
    k.front.append(shape("overflow-streak", "trough", "texture", 5.6, False, 0.3, path("M 0.0 0.43 C 0.01 0.35 -0.012 0.25 0.004 0.15 C 0.012 0.1 0.0 0.06 0.006 0.02 M -0.02 0.42 C -0.025 0.36 -0.015 0.3 -0.022 0.24"), ALGAE))
    k.front.append(shape("flaking", "column", "texture", 2.6, False, 0.3, path("M -0.04 0.25 L -0.01 0.27 L 0.0 0.24 Z M 0.02 0.45 L 0.05 0.47 L 0.045 0.43 Z"), paint))
    if fine:
        k.elev.append(shape("rust-runs", "column", "texture", 2.6, False, 0.25, path("M 0.10 0.06 L 0.13 0.062 M 0.10 -0.04 L 0.14 -0.041 M 0.10 0.0 L 0.12 0.001"), rust))
        k.elev.append(shape("flaking-head", "head", "texture", 3.9, False, 0.3, path("M 0.08 0.02 L 0.11 -0.01 L 0.14 0.015 Z M 0.20 -0.03 L 0.23 -0.05 L 0.24 -0.02 Z"), paint))
        lc = X_TROUGH[1] - X_TROUGH[0] - CHIP
        k.elev.append(shape("chip-edge", "trough", "detail", 5.6, False, 0.35, poly([(lc, Y_RIM), (lc, Y_NOTCH)], False), gran))
        k.plan.append(shape("gum", "trough", "texture", 5.3, False, 0.2, path("M 0.30 -0.27 m -0.006 0 a 0.006 0.006 0 1 0 0.012 0 a 0.006 0.006 0 1 0 -0.012 0 M 0.62 0.26 m -0.006 0 a 0.006 0.006 0 1 0 0.012 0 a 0.006 0.006 0 1 0 -0.012 0 M 1.0 0.1 m -0.006 0 a 0.006 0.006 0 1 0 0.012 0 a 0.006 0.006 0 1 0 -0.012 0"), gran))
        k.plan.append(shape("leaves", "water", "texture", 5.7, False, 0.3, path("M 0.55 0.10 C 0.58 0.13 0.62 0.13 0.64 0.10 C 0.61 0.08 0.58 0.08 0.55 0.10 Z M 0.80 -0.12 C 0.82 -0.09 0.86 -0.09 0.87 -0.12 C 0.85 -0.14 0.82 -0.14 0.80 -0.12 Z"), WATER))
        k.plan.append(shape("wet-patch", "footing", "texture", 0.6, False, 0.3, path(f"M {X_TROUGH[1]} -0.2 C 1.08 -0.18 1.09 -0.05 1.085 0.08 C 1.08 0.18 1.07 0.22 {X_TROUGH[1]} 0.24"), CONC))
        k.plan.append(shape("flaking-cap", "head", "texture", 3.95, False, 0.3, path("M -0.04 0.03 L -0.02 0.05 L -0.01 0.035 Z M 0.03 -0.04 L 0.05 -0.03 L 0.04 -0.05 Z"), paint))


def masses():
    """⚖️ Bone masses from volume x density (kg) and their centres in world rest metres; one source for the rig, the mass markers and the report."""
    rho_ci, rho_st, rho_g, rho_c, rho_w = 7200.0, 7850.0, 2800.0, 2300.0, 1000.0
    out = {}
    out["footing"] = [((X_SLAB[1] - X_SLAB[0]) * 2 * Z_SLAB * -Y_SLAB0 * rho_c, sum(X_SLAB) / 2, Y_SLAB0 / 2)]
    vb, cb = ring_volume(BASE50)
    vs, cs = ring_volume(SHAFT50)
    vp = math.pi * (R_PIPE ** 2 - (R_PIPE - T_PIPE) ** 2) * (0.69 - Y_SLAB0)
    out["column"] = [(vb * rho_ci, 0.0, cb), (vs * rho_ci, 0.0, cs), (vp * rho_st, 0.0, (0.69 + Y_SLAB0) / 2), (4 * 0.013 * 0.024 * 0.024 * rho_st, 0.0, 0.0285)]
    vh, ch = ring_volume(HEAD50)
    vc, cc = ring_volume(CAP50)
    body, lip, hw = spout_parts(50)
    sb = poly_area(body)
    sl = poly_area(lip)
    cheek = poly_area(cheek_profile(50))
    band = math.pi * (R_BAND ** 2 - R_HEAD ** 2) * 0.022
    out["head"] = [
        (vh * rho_ci, 0.0, Y_COLUMN1 + ch), (vc * rho_ci, 0.0, Y_COLUMN1 + cc),
        (0.55 * sb[0] * 2 * hw * rho_ci, sb[1], sb[2]), (0.55 * sl[0] * 2 * hw * rho_ci, sl[1], sl[2]),
        (2 * cheek[0] * (CHEEK_Z[1] - CHEEK_Z[0]) * rho_ci, cheek[1], cheek[2]),
        (2 * band * rho_ci, 0.0, 0.84), (4 * 0.008 * 0.016 * 0.016 * rho_st, 0.0, 0.694),
        (math.pi * (R_GLAND ** 2 - R_ROD ** 2) * 0.02 * rho_st, 0.0, 1.03), (math.pi * R_PIN ** 2 * 2 * Z_PIN * rho_st, PIVOT[0], PIVOT[1]),
    ]
    la = poly_area(lever_profile(50))
    gv, gc = ring_volume(grip_profile(50))
    gx, gy = world("lever", gc, 0.0)
    lx, ly = world("lever", la[1], la[2])
    px, py = world("lever", U_ROD_PIN, 0.0)
    out["lever"] = [(la[0] * 2 * Z_LEVER * rho_ci, lx, ly), (gv * rho_ci, gx, gy), (math.pi * 0.006 ** 2 * 0.06 * rho_st, px, py)]
    eye = (U_ROD_EYE - 0.02)
    out["rod"] = [
        (math.pi * R_ROD ** 2 * (eye - 0.04) * rho_st, 0.0, Y_ROD0 + (eye + 0.04) / 2),
        (0.04 * 0.03 * 0.05 * 0.7 * rho_st, 0.0, Y_ROD0 + U_ROD_EYE),
        (math.pi * (R_BORE - 0.0005) ** 2 * 0.04 * 0.55 * 8500, 0.0, Y_ROD0 + 0.02),
    ]
    L = X_TROUGH[1] - X_TROUGH[0]
    W = 2 * Z_TROUGH
    inner = (L - 2 * T_WALL) * (W - 2 * T_WALL) * (Y_RIM - Y_FLOOR)
    notch = 2 * Z_NOTCH * T_WALL * (Y_RIM - Y_NOTCH)
    chip = 0.5 * CHIP * CHIP * (Y_RIM - Y_NOTCH)
    al = poly_area(algae_patch(50))
    out["trough"] = [(L * W * Y_RIM * rho_g - inner * rho_g - notch * rho_g - chip * rho_g, X_TROUGH[0] + L / 2, 0.22), (al[0] * T_ALGAE * 1050, X_TROUGH[0] + al[1], al[2])]
    wl = L - 2 * T_WALL - 2 * GAP_WATER
    wh = Y_WATER - Y_FLOOR - GAP_WATER
    wz = Z_TROUGH - T_WALL - GAP_WATER
    out["water"] = [(wl * wh * 2 * wz * rho_w, X_TROUGH[0] + T_WALL + GAP_WATER + wl / 2, Y_FLOOR + GAP_WATER + wh / 2)]
    total = {}
    for bone, parts in out.items():
        m = sum(p[0] for p in parts)
        total[bone] = (m, sum(p[0] * p[1] for p in parts) / m, sum(p[0] * p[2] for p in parts) / m)
    return out, total


REFERENCES = [
    ("din-en-1561", "DIN EN 1561:2012-01 Founding, grey cast irons (EN-GJL-200: density 7200 kg/m3, E about 100 GPa, tensile strength 200 MPa), the material of pump stands, pump heads and levers."),
    ("en-1492-2", "EN 1492-2:2000+A1:2008 Textile slings - Safety - Part 2: Roundslings made of man-made fibres for general purpose use (working load limit 1 t, mode factor 0.8 choked)."),
    ("din-en-1176-1", "DIN EN 1176-1:2017-12 Playground equipment and surfacing, Part 1: General safety requirements and test methods (water play equipment on playgrounds, user loads, finger and head entrapment, protruding parts)."),
    ("din-en-1991-1-4", "DIN EN 1991-1-4:2010-12 with DIN EN 1991-1-4/NA:2010-12 Eurocode 1: Wind actions (wind zone 2 inland, peak velocity pressure 0.65 kN/m2 up to 10 m; force coefficient of a short circular cylinder)."),
    ("din-en-1991-1-1", "DIN EN 1991-1-1:2010-12 Eurocode 1: Actions on structures, Part 1-1: densities, self-weight and imposed loads (concrete 24 kN/m3 nominal, water 10 kN/m3)."),
    ("din-en-206", "DIN EN 206:2021-06 with DIN 1045-2:2023-08 Concrete (C25/30, exposure XC4/XF3 for a bedding slab that stays wet)."),
    ("din-en-1467", "DIN EN 1467:2012-10 Natural stone, rough blocks, and DIN EN 1936:2007-02 Natural stone test methods, apparent density (granite 2600-2800 kg/m3) for granite troughs."),
    ("en-10255", "EN 10255:2004+A1:2007 Non-alloy steel tubes suitable for welding and threading (DN 40 medium series: 48.3 x 3.68 mm, 4.05 kg/m), the galvanised riser pipe of hand pumps."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products, hygrothermal properties: design values of conductivity, specific heat and density of cast iron, steel, concrete, crystalline natural stone and water."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of painted, oxidised and polished iron, stone, concrete and water)."),
    ("duffie-beckman-2013", "Duffie, J. A., Beckman, W. A. (2013). Solar Engineering of Thermal Processes, 4th ed., Wiley (solar absorptance of dark paints, weathered paint, rusted iron, granite, concrete and water bodies)."),
    ("fll-wasserspiel", "FLL (Forschungsgesellschaft Landschaftsentwicklung Landschaftsbau): guidance on water play areas on playgrounds (hand pumps with troughs and channels, drinking-water quality, winter draining of the pump)."),
    ("hand-pump-catalogue", "Catalogue-typical data of German playground and park hand-lever pumps (Schwengelpumpen) 1990-2024: grey cast-iron stand 0.6-0.8 m with bolted base flange, pump head 75-80 mm bore with spout and forked lever bracket, lever 0.6-0.8 m, head 25-35 kg, stand 35-60 kg, on granite troughs 0.8-1.2 x 0.5-0.7 x 0.4-0.5 m; a product type, no brand."),
]


def materials():
    """🧱 The materials, weathered surfaces where the use left them."""
    m = lambda i, en, de, e, a, rho, lam, cp, src, ref, E=None, fs=None: {k: v for k, v in {"id": i, "title": tx(en, de), "emissivity": e, "solarAbsorptivity": a, "densityKgM3": rho, "conductivityWmK": lam, "specificHeatJKgK": cp, "elasticModulusPa": E, "strengthPa": fs, "source": src, "reference": ref}.items() if v is not None}
    return [
        m(PAINT, "Grey cast iron with old green alkyd paint, chalked and flaking", "Grauguss mit alter grüner Alkydfarbe, gekreidet und abblätternd", 0.9, 0.8, 7200, 50, 500, "literature", "duffie-beckman-2013", 1.0e11, 2.0e8),
        m(RUST, "Grey cast iron with the paint gone, rusted", "Grauguss ohne Farbe, verrostet", 0.8, 0.86, 7200, 50, 500, "literature", "incropera-2018", 1.0e11, 2.0e8),
        m(GRIP, "Grey cast iron polished by hands, greasy", "Grauguss, von Händen blank poliert, fettig", 0.35, 0.6, 7200, 50, 500, "authored-estimate", "incropera-2018", 1.0e11, 2.0e8),
        m(STEEL, "Steel piston rod and pins, greased and rusty", "Stahl-Kolbenstange und Bolzen, gefettet und rostig", 0.6, 0.7, 7850, 50, 470, "authored-estimate", "incropera-2018", 2.1e11, 2.35e8),
        m(PIPE, "Galvanised steel riser pipe DN 40", "Verzinktes Steigrohr DN 40", 0.28, 0.65, 7850, 50, 470, "literature", "en-10255", 2.1e11, 2.35e8),
        m(PISTON, "Brass piston with leather cup", "Messingkolben mit Ledermanschette", 0.5, 0.5, 8500, 110, 380, "authored-estimate", "iso-10456-2007"),
        m(GRANITE, "Granite, weathered and greyed", "Granit, verwittert und vergraut", 0.9, 0.58, 2800, 3.5, 1000, "literature", "iso-10456-2007", 5.0e10, 1.5e7),
        m(ALGAE, "Wet green algae film on stone", "Nasser grüner Algenbelag auf Stein", 0.95, 0.85, 1050, 0.55, 3800, "authored-estimate", "incropera-2018"),
        m(WATER, "Well water standing in the trough", "Brunnenwasser im Trog", 0.96, 0.8, 1000, 0.6, 4180, "literature", "incropera-2018"),
        m(CONC, "Concrete C25/30 bedding slab, weathered", "Betonbettung C25/30, verwittert", 0.92, 0.65, 2300, 1.65, 1000, "literature", "iso-10456-2007", 3.1e10, 2.5e7),
    ]


def sec(a, i, e, mat, en, de):
    """📐 A member section."""
    return {"areaM2": r5(a) if a >= 1e-4 else float(f"{a:.4g}"), "secondMomentM4": float(f"{i:.4g}"), "extremeFibreM": e, "material": mat, "title": tx(en, de)}


def rig(total):
    """🦴 The mechanical rig: bedding slab, pump stand, the liftable pump head with lever and piston rod, the trough, its water and three slices of the falling jet."""
    def mass(bone):
        m, cx, cy = total[bone]
        u, v = local(bone, cx, cy)
        ref = {"footing": "din-en-206", "trough": "din-en-1467", "water": "din-en-1991-1-1"}.get(bone, "din-en-1561")
        return {"kg": round(m, 3), "comX": u, "comY": v, "distribution": "uniform" if bone in ("footing", "column", "trough", "water") else "point", "source": "literature", "reference": ref}
    ro, ri = 0.0705, 0.0585
    tr_a = 0.6 * 0.1 + 2 * 0.08 * 0.4
    pivot_local = local("head", *PIVOT)
    bones = [
        {"id": "footing", "parent": None, "title": tx("Concrete bedding slab under pump and trough, from its top at ground level down to its base", "Betonbettung unter Pumpe und Trog, von der Oberkante in Geländehöhe hinab zur Sohle"), "x": 0, "y": 0, "angleDeg": -90, "lengthM": -Y_SLAB0, "joint": {"type": "fixed"}, "mass": mass("footing"), "section": sec((X_SLAB[1] - X_SLAB[0]) * 2 * Z_SLAB, 2 * Z_SLAB * (X_SLAB[1] - X_SLAB[0]) ** 3 / 12, (X_SLAB[1] - X_SLAB[0]) / 2, CONC, "Concrete C25/30 slab 1.32 x 0.72 m", "Betonplatte C25/30 1,32 x 0,72 m")},
        {"id": "column", "parent": "footing", "title": tx("Cast-iron pump stand with the riser pipe, from the base flange up to the top flange", "Gusseiserner Pumpenständer mit Steigrohr, vom Fußflansch bis zum Kopfflansch"), "x": 0, "y": 0, "angleDeg": 180, "lengthM": Y_COLUMN1, "joint": {"type": "fixed"}, "mass": mass("column"), "section": sec(math.pi * (ro ** 2 - ri ** 2), math.pi / 4 * (ro ** 4 - ri ** 4), ro, PAINT, "Cast tube 141 x 12 mm (EN-GJL-200)", "Gussrohr 141 x 12 mm (EN-GJL-200)")},
        {"id": "head", "parent": "column", "title": tx("Pump head: cylinder with spout, cap and lever bracket, lifted off its flange for maintenance", "Pumpenkopf: Zylinder mit Auslauf, Deckel und Schwengelbock, zur Wartung vom Flansch gehoben"), "x": Y_COLUMN1, "y": 0, "angleDeg": 0, "lengthM": Y_GLAND1 - Y_COLUMN1, "joint": {"type": "prismatic", "minM": 0, "maxM": 0.4, "axisDeg": 0}, "mass": mass("head"), "section": sec(math.pi * (R_HEAD ** 2 - R_BORE ** 2), math.pi / 4 * (R_HEAD ** 4 - R_BORE ** 4), R_HEAD, PAINT, "Cast cylinder 130 x 25 mm, bore 80 mm", "Gusszylinder 130 x 25 mm, Bohrung 80 mm")},
        {"id": "lever", "parent": "head", "title": tx("Pump lever (Schwengel) on its pivot pin, from the pivot back to the grip", "Pumpenschwengel am Drehbolzen, vom Drehpunkt nach hinten bis zum Griff"), "x": pivot_local[0], "y": pivot_local[1], "angleDeg": LEVER_DEG - 90, "lengthM": U_GRIP[1], "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 36}, "mass": mass("lever"), "section": sec(0.03 * 0.04, 0.03 * 0.04 ** 3 / 12, 0.02, PAINT, "Cast lever bar 30 x 40 mm", "Gussschwengel 30 x 40 mm")},
        {"id": "rod", "parent": "head", "title": tx("Piston with its rod and clevis, sliding in the cylinder", "Kolben mit Stange und Gabelkopf, im Zylinder gleitend"), "x": Y_ROD0 - Y_COLUMN1, "y": 0, "angleDeg": 0, "lengthM": r5(U_ROD_EYE), "joint": {"type": "prismatic", "minM": 0, "maxM": 0.065, "axisDeg": 0}, "mass": mass("rod"), "section": sec(math.pi * R_ROD ** 2, math.pi * (2 * R_ROD) ** 4 / 64, R_ROD, STEEL, "Steel rod 16 mm", "Stahlstange 16 mm")},
        {"id": "trough", "parent": "footing", "title": tx("Granite trough, from its back wall at the pump ahead to its front wall", "Granittrog, von der Rückwand an der Pumpe nach vorn zur Vorderwand"), "x": 0, "y": X_TROUGH[0], "angleDeg": 90, "lengthM": X_TROUGH[1] - X_TROUGH[0], "joint": {"type": "fixed"}, "mass": mass("trough"), "section": sec(tr_a, 0.00284, 0.321, GRANITE, "Granite U-section 0.60 x 0.50 m, walls 80 mm, floor 100 mm", "Granit-U-Querschnitt 0,60 x 0,50 m, Wände 80 mm, Boden 100 mm")},
        {"id": "water", "parent": "trough", "title": tx("Water standing in the trough up to just under the overflow notch", "Wasser im Trog bis knapp unter die Überlaufkerbe"), "x": T_WALL, "y": Y_FLOOR, "angleDeg": 0, "lengthM": 0.05, "joint": {"type": "fixed"}, "mass": mass("water")},
    ]
    for j in JETS:
        lu, lv = local("water", X_JET, j["y0"])
        bones.append({"id": j["id"], "parent": "water", "title": tx(f"Slice {j['id'][-1]} of the falling jet (counted from the spout), sunk in the trough water while the pump is still", f"Scheibe {j['id'][-1]} des fallenden Strahls (vom Auslauf gezählt), im Trogwasser versunken, solange die Pumpe ruht"), "x": lu, "y": lv, "angleDeg": 90, "lengthM": 0.05, "joint": {"type": "prismatic", "minM": 0, "maxM": r5(JET_RISE), "axisDeg": 90}})
    return {"kind": "mechanical", "root": "footing", "bones": bones}


def anchors():
    """📍 Supports at the slab base and at the lifting point of the head, and the load points of the poses."""
    su, sv = local("footing", sum(X_SLAB) / 2, Y_SLAB0)
    gu = 0.57
    return [
        {"id": "slab-base", "bone": "footing", "x": su, "y": sv, "kind": "support", "title": tx("Base of the bedding slab held by the soil (fixed)", "Sohle der Betonbettung im Boden gehalten (eingespannt)"), "support": "fixed", "footprintLengthM": X_SLAB[1] - X_SLAB[0], "footprintWidthM": 2 * Z_SLAB},
        {"id": "head-lift", "bone": "head", "x": Y_GLAND1 - Y_COLUMN1, "y": 0, "kind": "load", "title": tx("Lifting point of the head: round sling choked under the cap, on the hook of the depot crane", "Anschlagpunkt des Kopfs: Rundschlinge unter dem Deckel geschnürt, am Haken des Bauhofkrans")},
        {"id": "grip-push", "bone": "lever", "x": gu, "y": 0, "kind": "load", "title": tx("Hand on the grip of the lever", "Hand am Griff des Schwengels")},
        {"id": "piston-load", "bone": "rod", "x": 0.02, "y": 0, "kind": "load", "title": tx("Piston face lifting the water column", "Kolbenfläche, die die Wassersäule hebt")},
        {"id": "rim-lean", "bone": "trough", "x": X_TROUGH[1] - X_TROUGH[0] - T_WALL / 2, "y": Y_RIM, "kind": "load", "title": tx("Hands of a drinking child on the front rim", "Hände eines trinkenden Kindes auf dem vorderen Rand")},
        {"id": "wind-pump", "bone": "column", "x": 0.52, "y": 0, "kind": "load", "title": tx("Centre of the wind area of stand and head", "Mitte der Windfläche von Ständer und Kopf")},
    ]


def regions(area):
    """🔥 The thermal regions: passive weathered surfaces and water in the park sun, no rated power anywhere."""
    lay = lambda lid, en, de, kind, mat, t, lam, rho, cp, src, ref: {"id": lid, "title": tx(en, de), "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": float(f"{t / lam:.3g}"), "heatCapacityJM2K": round(rho * cp * t), "source": src, "reference": ref}
    def reg(rid, en, de, kind, mat, geom, lc, svf, layer, src="literature", ref="duffie-beckman-2013"):
        return {"id": rid, "title": tx(en, de), "kind": kind, "model": "passive", "material": mat, "areaM2": round(area[rid], 3), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [layer], "heatCapacityJM2K": layer["heatCapacityJM2K"], "source": src, "reference": ref}
    return [
        reg("pump-paint", "Pump stand, head, spout and lever in chalked green paint, flaking", "Pumpenständer, Kopf, Auslauf und Schwengel in gekreideter grüner Farbe, abblätternd", "surface", PAINT, "cylinder", 0.14, 0.6, lay("casting", "Grey cast iron 12 mm", "Grauguss 12 mm", "metal", PAINT, 0.012, 50, 7200, 500, "literature", "din-en-1561")),
        reg("pump-rust", "Rusted foot of the stand, spout lip with limescale, rod, pins and gland", "Verrosteter Ständerfuß, Auslauflippe mit Kalk, Stange, Bolzen und Stopfbuchse", "surface", RUST, "cylinder", 0.1, 0.5, lay("rusted-casting", "Rusted cast iron 12 mm", "Verrostetes Gusseisen 12 mm", "metal", RUST, 0.012, 50, 7200, 500, "literature", "din-en-1561"), ref="incropera-2018"),
        reg("grip-bare", "Lever grip polished bare by hands", "Schwengelgriff, von Händen blank poliert", "surface", GRIP, "cylinder", 0.035, 0.6, lay("grip-iron", "Cast iron grip 17 mm (radius)", "Gussgriff 17 mm (Radius)", "metal", GRIP, 0.017, 50, 7200, 500, "authored-estimate", "din-en-1561"), src="authored-estimate", ref="incropera-2018"),
        reg("trough-granite", "Granite trough, greyed, with a chipped corner and a tag", "Granittrog, vergraut, mit abgeschlagener Ecke und Tag", "surface", GRANITE, "vertical-plate", 0.5, 0.6, lay("stone", "Granite to the middle of the wall 40 mm", "Granit bis zur Wandmitte 40 mm", "masonry", GRANITE, 0.04, 3.5, 2800, 1000, "literature", "iso-10456-2007"), ref="iso-10456-2007"),
        reg("algae-film", "Wet algae on the shaded right face of the trough", "Nasse Algen an der schattigen rechten Trogseite", "surface", ALGAE, "vertical-plate", 0.4, 0.45, lay("biofilm", "Algae film 2 mm, water-soaked", "Algenbelag 2 mm, wassergesättigt", "polymer", ALGAE, T_ALGAE, 0.55, 1050, 3800, "authored-estimate", "incropera-2018"), src="authored-estimate", ref="incropera-2018"),
        reg("water", "Well water in the trough and the falling jet", "Brunnenwasser im Trog und der fallende Strahl", "water", WATER, "horizontal-plate-up", 0.44, 0.85, lay("water-body", "Water 160 mm (half the depth)", "Wasser 160 mm (halbe Tiefe)", "polymer", WATER, 0.16, 0.6, 1000, 4180, "literature", "incropera-2018"), ref="incropera-2018"),
        reg("slab-concrete", "Concrete bedding slab, top flush with the gravel, wet in front of the trough", "Betonbettung, Oberkante bündig mit dem Kies, vor dem Trog nass", "surface", CONC, "horizontal-plate-up", 0.72, 0.9, lay("slab-core", "Concrete to the middle of the slab 125 mm", "Beton bis zur Plattenmitte 125 mm", "masonry", CONC, 0.125, 1.65, 2300, 1000, "literature", "iso-10456-2007"), ref="iso-10456-2007"),
    ]


AREA = {"pump-paint": 0.62, "pump-rust": 0.224, "grip-bare": 0.016, "trough-granite": 3.3, "algae-film": 0.355, "water": 1.4, "slab-concrete": 2.8}


def binding(state):
    """🌞 Region bindings of a pose: summer noon in the park; the water stands in the trough or runs cold from the well."""
    sun = {"pump-paint": 0.5, "pump-rust": 0.5, "grip-bare": 0.6, "trough-granite": 0.45, "algae-film": 0.0, "water": 0.75, "slab-concrete": 0.4}
    out = {rid: {"exposedFraction": 1, "sunlitFraction": f} for rid, f in sun.items()}
    out["trough-granite"] = {"exposedFraction": 0.43, "sunlitFraction": 0.45, "wettedness": 0.1, "contact": {"fraction": 0.47, "tempC": 18}}
    out["algae-film"] = {"exposedFraction": 0.5, "sunlitFraction": 0.0, "wettedness": 0.7, "contact": {"fraction": 0.5, "tempC": 18}}
    out["water"] = {"exposedFraction": 0.23, "sunlitFraction": 0.75, "wettedness": 1.0, "contact": {"fraction": 0.77, "tempC": 18}}
    out["slab-concrete"] = {"exposedFraction": 0.12, "sunlitFraction": 0.4, "wettedness": 0.05, "contact": {"fraction": 0.88, "tempC": 15}}
    out["pump-paint"] = {"exposedFraction": 0.9, "sunlitFraction": 0.5, "contact": {"fraction": 0.1, "tempC": 12}}
    if state == "maintenance":
        out["pump-paint"] = {"exposedFraction": 1, "sunlitFraction": 0.55}
        out["pump-rust"] = {"exposedFraction": 1, "sunlitFraction": 0.55}
    if state == "in-use":
        out["trough-granite"] = {"exposedFraction": 0.43, "sunlitFraction": 0.45, "wettedness": 0.35, "contact": {"fraction": 0.47, "tempC": 14}}
        out["water"] = {"exposedFraction": 0.25, "sunlitFraction": 0.75, "wettedness": 1.0, "contact": {"fraction": 0.8, "tempC": 10}}
        out["pump-rust"] = {"exposedFraction": 1, "sunlitFraction": 0.5, "wettedness": 0.3}
        out["slab-concrete"] = {"exposedFraction": 0.12, "sunlitFraction": 0.4, "wettedness": 0.2, "contact": {"fraction": 0.88, "tempC": 15}}
        out["grip-bare"] = {"exposedFraction": 0.6, "sunlitFraction": 0.5, "contact": {"fraction": 0.4, "tempC": 33}}
    return {"regions": out, "emitters": {}}


def stroke(t):
    """🔁 The lever offset (degrees) of the pumping clip at time t: three priming strokes then steady strokes of 1.2 s, rest at the ends."""
    if t <= 0.3 or t >= 5.4:
        return 0.0
    phase = (t - 0.3) / 1.2
    return LEVER_STROKE_DEG * 0.5 * (1 - math.cos(2 * math.pi * phase))


def rod_dx(angle):
    """🪛 Lift of the piston rod (m) for a lever offset (degrees): the pin on the lever, 0.1035 m ahead of the pivot."""
    return U_ROD_PIN * (math.sin(math.radians(LEVER_DEG + angle)) - math.sin(math.radians(LEVER_DEG)))


JET_ON = (1.0, 1.04, 1.08)
JET_OFF = (5.5, 5.54, 5.58)
FRAME = 1 / 30


def jet_dx(k, t):
    """💧 Rise of jet slice k (m) at time t: it jumps out of the trough water into its place under the spout within one frame, top slice first."""
    on, off = JET_ON[k], JET_OFF[k]
    if t <= on or t >= off + FRAME:
        return 0.0
    if t < on + FRAME:
        return JET_RISE * (t - on) / FRAME
    if t <= off:
        return JET_RISE
    return JET_RISE * (1 - (t - off) / FRAME)


def poses():
    """🧍 In service in a breeze, the head lifted off by the depot, and a child pumping and drinking."""
    wind = [{"id": "wind-on-pump", "kind": "wind", "title": tx("Design gust on stand and head (0.65 kN/m2 x 0.9 x 0.14 m2)", "Bemessungsböe auf Ständer und Kopf (0,65 kN/m² x 0,9 x 0,14 m²)"), "anchor": "wind-pump", "fxN": -82, "fyN": 0, "source": "literature", "reference": "din-en-1991-1-4"}]
    use = [
        {"id": "handle-push", "kind": "imposed", "title": tx("A child pressing the grip down (0.2 kN)", "Ein Kind drückt den Griff nach unten (0,2 kN)"), "anchor": "grip-push", "fxN": 0, "fyN": -200, "source": "authored-estimate", "reference": "din-en-1176-1"},
        {"id": "piston-water", "kind": "live", "title": tx("Water column of 5 m suction and lift on the 80 mm piston (0.25 kN)", "Wassersäule von 5 m Saughöhe auf dem 80-mm-Kolben (0,25 kN)"), "anchor": "piston-load", "fxN": 0, "fyN": -247, "source": "authored-estimate"},
        {"id": "rim-lean", "kind": "imposed", "title": tx("A second child leaning on the front rim to drink (0.3 kN)", "Ein zweites Kind stützt sich zum Trinken auf den vorderen Rand (0,3 kN)"), "anchor": "rim-lean", "fxN": 30, "fyN": -300, "source": "authored-estimate", "reference": "din-en-1176-1"},
    ]
    base = {"ground": "authored"}
    hook = [{"id": "crane-hook", "kind": "equipment", "title": tx("Crane hook carrying head, lever and piston on the round sling", "Kranhaken trägt Kopf, Schwengel und Kolben an der Rundschlinge"), "anchor": "head-lift", "fxN": 0, "fyN": round(sum(round(masses()[1][b][0], 3) for b in ("head", "lever", "rod")) * 9.80665, 1), "source": "authored-estimate", "reference": "en-1492-2"}]
    jets_on = {j["id"]: {"dy": r5(JET_RISE)} for j in JETS}
    return [
        {"id": "in-service", "title": tx("In service", "Im Einsatz"), "description": tx("The pump stands still on its stand behind the granite trough, the lever up, the trough full to just under the overflow notch, a design gust from ahead; the jet slices rest unseen in the water.", "Die Pumpe steht still auf ihrem Ständer hinter dem Granittrog, der Schwengel oben, der Trog bis knapp unter die Überlaufkerbe voll, eine Bemessungsböe von vorn; die Strahlscheiben ruhen unsichtbar im Wasser."), "bones": {}, **base, "contacts": ["slab-base"], "thermal": binding("in-service"), "loads": wind},
        {"id": "maintenance", "title": tx("Head lifted off", "Kopf abgehoben"), "description": tx(f"The depot has undone the four flange bolts and lifts the pump head with lever and piston {HEAD_LIFT:.2f} m off the stand on a round sling under the cap, to change the leather cup; the riser pipe stands out of the top flange.", f"Der Bauhof hat die vier Flanschschrauben gelöst und hebt den Pumpenkopf mit Schwengel und Kolben an einer unter dem Deckel geschnürten Rundschlinge {HEAD_LIFT:.2f} m vom Ständer, um die Ledermanschette zu wechseln; das Steigrohr ragt aus dem Kopfflansch."), "bones": {"head": {"dx": HEAD_LIFT}}, **base, "contacts": ["slab-base"], "thermal": binding("maintenance"), "loads": hook},
        {"id": "sling-taut", "key": True, "title": tx("Sling taut", "Schlinge gespannt"), "description": tx("Key of the maintenance clip: the hook carries the head before it leaves the flange; the slab carries stand and trough.", "Schlüssel des Wartungs-Clips: der Haken trägt den Kopf, bevor er den Flansch verlässt; die Bettung trägt Ständer und Trog."), "bones": {}, **base, "contacts": ["slab-base"], "thermal": binding("in-service"), "loads": hook},
        {"id": "in-use", "title": tx("Pumping and drinking", "Pumpen und trinken"), "description": tx(f"A child has pressed the grip down {LEVER_STROKE_DEG:.0f} degrees, the piston rises {rod_dx(LEVER_STROKE_DEG) * 1000:.0f} mm, cold well water gushes from the spout into the trough, and a second child leans on the front rim to drink.", f"Ein Kind hat den Griff um {LEVER_STROKE_DEG:.0f} Grad hinuntergedrückt, der Kolben steigt um {rod_dx(LEVER_STROKE_DEG) * 1000:.0f} mm, kaltes Brunnenwasser schießt aus dem Auslauf in den Trog, und ein zweites Kind stützt sich zum Trinken auf den vorderen Rand."), "bones": {"lever": {"angleDeg": LEVER_STROKE_DEG}, "rod": {"dx": r5(rod_dx(LEVER_STROKE_DEG))}, **jets_on}, **base, "contacts": ["slab-base"], "thermal": binding("in-use"), "loads": use},
    ]


def clips():
    """🎞 The depot lifting the head off and back, and a child pumping water into the trough."""
    times = sorted(set([round(0.1 * i, 4) for i in range(61)] + [round(t, 4) for t in JET_ON + JET_OFF] + [round(t + FRAME, 4) for t in JET_ON + JET_OFF]))
    keys = []
    for t in times:
        a = stroke(t)
        bones = {"lever": {"angleDeg": r5(a)}, "rod": {"dx": r5(rod_dx(a))}}
        for k, j in enumerate(JETS):
            bones[j["id"]] = {"dy": r5(jet_dx(k, t))}
        keys.append({"t": t, "pose": "in-use", "ease": "linear", "bones": bones})
    downs = []
    for c in range(4):
        t0 = 0.3 + 1.2 * c
        downs += [{"t": round(t0, 3), "v": 0}, {"t": round(t0 + 0.3, 3), "v": 1}, {"t": round(t0 + 0.6, 3), "v": 1}, {"t": round(t0 + 0.75, 3), "v": 0}]
    downs.append({"t": 6, "v": 0})
    return [
        {"id": "maintenance", "title": tx("Lifting the head off", "Kopf abheben"), "description": tx("The depot undoes the flange bolts, the crane lifts the head with lever and piston off the stand, holds it while the cup is changed, and sets it back on the flange.", "Der Bauhof löst die Flanschschrauben, der Kran hebt den Kopf mit Schwengel und Kolben vom Ständer, hält ihn, während die Manschette gewechselt wird, und setzt ihn wieder auf den Flansch."), "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 1.0, "pose": "in-service", "ease": "smooth"}, {"t": 1.5, "pose": "sling-taut", "ease": "smooth"}, {"t": 4, "pose": "maintenance", "ease": "smooth"}, {"t": 5.5, "pose": "maintenance", "ease": "smooth"}, {"t": 8, "pose": "sling-taut", "ease": "smooth"}],
         "contacts": [{"anchor": "slab-base", "fromS": 0, "toS": 8}],
         "phases": [{"id": "unbolt", "fromS": 0, "toS": 1.5, "title": tx("Undoing the flange bolts", "Flanschschrauben lösen")}, {"id": "lift", "fromS": 1.5, "toS": 4, "title": tx("Lifting the head", "Kopf heben")}, {"id": "service", "fromS": 4, "toS": 5.5, "title": tx("Changing the leather cup", "Ledermanschette wechseln")}, {"id": "set-back", "fromS": 5.5, "toS": 8, "title": tx("Setting the head back", "Kopf zurücksetzen")}],
         "events": [{"t": 0.3, "kind": "motion", "title": tx("Flange bolts undone", "Flanschschrauben gelöst")}, {"t": 1.5, "kind": "load-on", "title": tx("Sling on the crane hook takes the head", "Schlinge am Kranhaken übernimmt den Kopf"), "target": "crane-hook"}, {"t": 8, "kind": "motion", "title": tx("Head back on its flange", "Kopf wieder auf dem Flansch")}],
         "tracks": []},
        {"id": "pumping", "title": tx("Pumping water", "Wasser pumpen"), "description": tx("A child works the lever: after the first stroke the cold water gushes from the spout into the trough and keeps running with every stroke of 1.2 s; when the child lets go the last water falls from the lip.", "Ein Kind bewegt den Schwengel: nach dem ersten Hub schießt das kalte Wasser aus dem Auslauf in den Trog und läuft mit jedem Hub von 1,2 s weiter; lässt das Kind los, fällt das letzte Wasser von der Lippe."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": keys,
         "contacts": [{"anchor": "slab-base", "fromS": 0, "toS": 6}],
         "phases": [{"id": "priming", "fromS": 0, "toS": 1.0, "title": tx("First stroke, priming", "Erster Hub, Ansaugen")}, {"id": "flowing", "fromS": 1.0, "toS": 5.5, "title": tx("Water flowing", "Wasser fließt")}, {"id": "dripping", "fromS": 5.5, "toS": 6, "title": tx("Last water falling", "Letztes Wasser fällt")}],
         "events": [{"t": 1.0, "kind": "motion", "title": tx("Water reaches the spout", "Wasser erreicht den Auslauf")}, {"t": 5.4, "kind": "load-off", "title": tx("Child lets go of the grip", "Kind lässt den Griff los"), "target": "handle-push"}],
         "tracks": [{"target": "load:handle-push", "ease": "linear", "keys": downs}, {"target": "load:piston-water", "ease": "linear", "keys": downs}, {"target": "load:rim-lean", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 6, "v": 0}]}]},
    ]


PENS = {
    50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7},
    100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5},
    200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35},
    500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}
BANDS_RANGE = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def rep_decisions(s):
    """📝 The eight authored decisions of the 2D representation at 1:s."""
    return {
        50: dict(
            silhouette=tx("Side elevation 36 x 31 mm on paper: the 0.25 m bedding slab below ground, the cast stand with its rusted foot flange and anchor nuts, the 12 mm walled shaft, the head with its bolted flange, two cast bands, the cap and gland, the spout with its rusted lip, the forked lever bracket with the pivot pin, the lever with its turned grip, the piston rod and clevis, the granite trough with the algae patch on its shaded face, and the water. Front and plan are the same parts from ahead and from above; spout, bracket cheeks, lever, grip and pins carry extruded solids, so they turn right in the three views.", "Seitenansicht 36 x 31 mm auf dem Papier: die 0,25-m-Betonbettung unter Gelände, der Gussständer mit verrostetem Fußflansch und Ankermuttern, der Schaft mit 12 mm Wand, der Kopf mit verschraubtem Flansch, zwei Gussringen, Deckel und Stopfbuchse, der Auslauf mit verrosteter Lippe, der gegabelte Schwengelbock mit Drehbolzen, der Schwengel mit gedrechseltem Griff, Kolbenstange und Gabelkopf, der Granittrog mit dem Algenfleck auf der schattigen Seite und das Wasser. Vorder- und Draufsicht zeigen dieselben Teile von vorn und von oben; Auslauf, Bockwangen, Schwengel, Griff und Bolzen tragen extrudierte Körper und drehen sich daher in allen drei Ansichten richtig."),
            retained=tx("Every visible part, the overflow notch and the chipped corner, the anchor nuts and flange bolts, the three slices of the jet (hidden in the water while the pump rests).", "Jedes sichtbare Teil, Überlaufkerbe und abgeschlagene Ecke, Ankermuttern und Flanschschrauben, die drei Strahlscheiben (bei ruhender Pumpe im Wasser verborgen)."),
            merged=tx("Nothing merged; the riser pipe and the piston are inside the stand and the cylinder and are not drawn.", "Nichts verschmolzen; Steigrohr und Kolben liegen in Ständer und Zylinder und werden nicht gezeichnet."),
            omitted=tx("The foot valve below the slab, the drain hole of the trough and the threads.", "Das Fußventil unter der Bettung, das Ablaufloch des Trogs und die Gewinde."),
            thinElements=tx("The 2 mm algae film is drawn as the filled patch on the trough face; flaking paint, rust runs, the sticker, the felt-tip tag, moss, limescale, chewing gum and leaves are 0.13 mm texture strokes; the notch, the chipped edge, the rim edge and the crack are 0.25 mm detail lines.", "Der 2-mm-Algenbelag ist der gefüllte Fleck auf der Trogseite; abblätternde Farbe, Rostläufer, der Aufkleber, das Filzstift-Tag, Moos, Kalk, Kaugummi und Laub sind 0,13-mm-Texturstriche; Kerbe, abgeschlagene Kante, Randkante und Riss sind 0,25-mm-Detaillinien."),
            hatching=tx("Granite stipple on the trough, concrete dots on the slab, horizontal lines on the water.", "Granitpunktierung auf dem Trog, Betonpunkte auf der Bettung, waagrechte Linien auf dem Wasser."),
            styles=tx("Granite 0.7, algae 0.38, water 0.86, slab 0.66, paint 0.42-0.5, rust 0.3, grip 0.7; body pen 0.5, detail 0.25, texture 0.13.", "Granit 0,7, Algen 0,38, Wasser 0,86, Bettung 0,66, Farbe 0,42-0,5, Rost 0,3, Griff 0,7; Körperstift 0,5, Detail 0,25, Textur 0,13."),
            overlays=tx("pump-paint: shaft, head, bands, cap, spout, bracket, lever, bolts; pump-rust: foot, anchor nuts, spout lip, gland, pins, rod, clevis; grip-bare; trough-granite; algae-film; water: trough water and jet; slab-concrete.", "pump-paint: Schaft, Kopf, Ringe, Deckel, Auslauf, Bock, Schwengel, Schrauben; pump-rust: Fuß, Ankermuttern, Auslauflippe, Stopfbuchse, Bolzen, Stange, Gabelkopf; grip-bare; trough-granite; algae-film; water: Trogwasser und Strahl; slab-concrete."),
        ),
        100: dict(
            silhouette=tx("Side elevation 18 x 15 mm: slab, stand with foot flange, head with flange and cap, spout and lip, one bracket block, pivot pin, lever and grip, rod and clevis, trough with notch and chipped corner, algae patch, water, jet slices; front and plan likewise.", "Seitenansicht 18 x 15 mm: Bettung, Ständer mit Fußflansch, Kopf mit Flansch und Deckel, Auslauf und Lippe, ein Bockblock, Drehbolzen, Schwengel und Griff, Stange und Gabelkopf, Trog mit Kerbe und abgeschlagener Ecke, Algenfleck, Wasser, Strahlscheiben; Vorder- und Draufsicht ebenso."),
            retained=tx("All parts of 15 mm and more; notch, chipped edge, crack, tag, two flakes and the overflow streak as strokes.", "Alle Teile ab 15 mm; Kerbe, abgeschlagene Kante, Riss, Tag, zwei Abplatzungen und der Überlaufstreifen als Striche."),
            merged=tx("The two 12 mm bracket cheeks and the lever between them into one 60 mm bracket block, the gland into the cap, the 8 mm cast bands into a 1 mm wider cylinder.", "Die zwei 12-mm-Bockwangen und der Schwengel dazwischen in einen 60-mm-Bockblock, die Stopfbuchse in den Deckel, die 8-mm-Gussringe in einen 1 mm dickeren Zylinder."),
            omitted=tx("Anchor nuts and flange bolts (13 and 8 mm), the 12 mm rod pin, gum, leaves, rust runs.", "Ankermuttern und Flanschschrauben (13 und 8 mm), der 12-mm-Stangenbolzen, Kaugummi, Laub, Rostläufer."),
            thinElements=tx("The algae film is the filled patch; notch and crack are 0.18 mm detail lines, tag and flakes 0.13 mm texture.", "Der Algenbelag ist der gefüllte Fleck; Kerbe und Riss sind 0,18-mm-Detaillinien, Tag und Abplatzungen 0,13-mm-Textur."),
            hatching=tx("Granite stipple, concrete dots, water lines.", "Granitpunktierung, Betonpunkte, Wasserlinien."),
            styles=tx("Tones as 1:50; body pen 0.35, detail 0.18, texture 0.13.", "Töne wie 1:50; Körperstift 0,35, Detail 0,18, Textur 0,13."),
            overlays=tx("Regions as 1:50; the bracket block is pump-paint.", "Bereiche wie 1:50; der Bockblock gehört zu pump-paint."),
        ),
        200: dict(
            silhouette=tx("Side elevation 9 x 7.7 mm: slab, stand, head with cap as one body, spout and lip, bracket block, lever and grip, clevis, trough with notch and chipped corner, algae patch, water.", "Seitenansicht 9 x 7,7 mm: Bettung, Ständer, Kopf mit Deckel als ein Körper, Auslauf und Lippe, Bockblock, Schwengel und Griff, Gabelkopf, Trog mit Kerbe und abgeschlagener Ecke, Algenfleck, Wasser."),
            retained=tx("The masses that read at 30 mm.", "Die bei 30 mm lesbaren Massen."),
            merged=tx("The foot flange thickened to 30 mm, cap and gland into the head, the pivot pin into the bracket.", "Der Fußflansch auf 30 mm verdickt, Deckel und Stopfbuchse in den Kopf, der Drehbolzen in den Bock."),
            omitted=tx("The 16 mm piston rod and the 25 mm jet (below the feature; the jet is a thin line of water at this size), every texture stroke.", "Die 16-mm-Kolbenstange und der 25-mm-Strahl (unter dem Merkmal; der Strahl ist in dieser Größe ein dünner Wasserfaden), jeder Texturstrich."),
            thinElements=tx("The lever (32 x 36 mm) and the grip (35 mm) stay at their true sections; the algae film is the filled patch.", "Schwengel (32 x 36 mm) und Griff (35 mm) behalten ihre wahren Querschnitte; der Algenbelag ist der gefüllte Fleck."),
            hatching=tx("None.", "Keine."),
            styles=tx("Tones as 1:50; body pen 0.25, detail 0.13.", "Töne wie 1:50; Körperstift 0,25, Detail 0,13."),
            overlays=tx("Regions as 1:50.", "Bereiche wie 1:50."),
        ),
        500: dict(
            silhouette=tx("Side elevation 3.6 x 3.1 mm: slab, stand with a 40 mm foot flange and bell, head as one 150 mm cylinder, spout and lip at 75 mm, a 75 mm bracket block, the lever and grip at 75 mm, trough, algae patch, water.", "Seitenansicht 3,6 x 3,1 mm: Bettung, Ständer mit 40-mm-Fußflansch und Glocke, Kopf als ein 150-mm-Zylinder, Auslauf und Lippe mit 75 mm, ein 75-mm-Bockblock, Schwengel und Griff mit 75 mm, Trog, Algenfleck, Wasser."),
            retained=tx("Slab, stand, head, spout, lip, bracket, lever, grip, trough, algae, water.", "Bettung, Ständer, Kopf, Auslauf, Lippe, Bock, Schwengel, Griff, Trog, Algen, Wasser."),
            merged=tx("Head flange, bands and cap into one cylinder; the clevis into the lever; notch and chip into the trough rim.", "Kopfflansch, Ringe und Deckel in einen Zylinder; der Gabelkopf in den Schwengel; Kerbe und Ecke in den Trogrand."),
            omitted=tx("Rod, pins, jet and every stroke below the 75 mm feature.", "Stange, Bolzen, Strahl und jeder Strich unter dem 75-mm-Merkmal."),
            thinElements=tx("Spout (60 mm), lever (30 x 40 mm) and grip (35 mm) would be 0.12, 0.08 and 0.07 mm on paper: all are drawn at the 0.15 mm print floor (75 mm).", "Auslauf (60 mm), Schwengel (30 x 40 mm) und Griff (35 mm) wären 0,12, 0,08 und 0,07 mm auf dem Papier: alle sind an der Druckgrenze von 0,15 mm (75 mm) gezeichnet."),
            hatching=tx("None.", "Keine."),
            styles=tx("Tones as 1:50; body pen 0.18.", "Töne wie 1:50; Körperstift 0,18."),
            overlays=tx("Regions as 1:50.", "Bereiche wie 1:50."),
        ),
    }[s]


def vol_decisions(s):
    """🧊 The volume decisions at 1:s."""
    return {
        50: [tx("Body 1:50 for 7.5 mm: C25/30 bedding slab 1.32 x 0.72 x 0.25 m; the cast stand as two hollow revolves with 12 mm walls (rusted foot flange 300 mm with four M16 anchor nuts, painted shaft 150 to 132 mm with its top flange), the DN 40 riser pipe inside; the head as a hollow cylinder 130 mm with 80 mm bore and bolted flange, two cast bands (declared slender hoops), the cap and gland; the spout and its rusted lip extruded from their side profile, two 12 mm bracket cheeks, the 16 mm pivot pin, the cast lever with boss and eye, the turned grip, the 12 mm rod pin, the 16 mm rod with clevis and the brass piston; the granite trough as floor slab, wall ring and C-shaped rim with the overflow notch and the chipped corner; the 2 mm algae film as a declared sheet; the water inset 0.5 mm; three slices of the jet.", "Körper 1:50 für 7,5 mm: Betonbettung C25/30 1,32 x 0,72 x 0,25 m; der Gussständer als zwei hohle Rotationskörper mit 12 mm Wand (verrosteter Fußflansch 300 mm mit vier M16-Ankermuttern, lackierter Schaft 150 bis 132 mm mit Kopfflansch), innen das Steigrohr DN 40; der Kopf als hohler Zylinder 130 mm mit 80-mm-Bohrung und verschraubtem Flansch, zwei Gussringe (als schlanke Reifen erklärt), Deckel und Stopfbuchse; Auslauf und verrostete Lippe aus ihrem Seitenprofil extrudiert, zwei 12-mm-Bockwangen, der 16-mm-Drehbolzen, der Gussschwengel mit Nabe und Auge, der gedrechselte Griff, der 12-mm-Stangenbolzen, die 16-mm-Stange mit Gabelkopf und der Messingkolben; der Granittrog als Bodenplatte, Wandring und C-förmiger Rand mit Überlaufkerbe und abgeschlagener Ecke; der 2-mm-Algenbelag als erklärte Platte; das Wasser 0,5 mm eingerückt; drei Strahlscheiben."), tx("Thermal 1:50 by reference to the body solids, one region per weathered surface; riser pipe and piston are inside closed cavities and carry no region. Structure 1:50: members along footing, stand, head, lever, rod and trough, joints at the anchor nuts, flange bolts, gland, pivot and rod pins and clevis, one mass marker per bone with mass, the support block at the slab base (the lifting point is a contact of the maintenance pose only, not a permanent support).", "Thermik 1:50 als Verweis auf die Körper, ein Bereich je verwitterter Oberfläche; Steigrohr und Kolben liegen in geschlossenen Hohlräumen und tragen keinen Bereich. Tragwerk 1:50: Stäbe entlang Bettung, Ständer, Kopf, Schwengel, Stange und Trog, Gelenke an Ankermuttern, Flanschschrauben, Stopfbuchse, Dreh- und Stangenbolzen und Gabelkopf, ein Massenpunkt je Knochen mit Masse, der Auflagerblock an der Bettungssohle (der Anschlagpunkt ist nur ein Kontakt der Wartungsstellung, kein ständiges Auflager).")],
        100: [tx("Body 1:100 for 15 mm: hollow stand and head with simplified profiles, the bands merged into a 1 mm thicker cylinder, the gland into the cap, the two cheeks and the lever into one 60 mm bracket block; nuts, bolts, the rod pin, the riser pipe and the piston are below the feature or hidden and left out; trough with notch and chip, algae sheet, water and three constant 24 mm jet slices.", "Körper 1:100 für 15 mm: hohler Ständer und Kopf mit vereinfachten Profilen, die Ringe in einen 1 mm dickeren Zylinder, die Stopfbuchse in den Deckel, die zwei Wangen und der Schwengel in einen 60-mm-Bockblock verschmolzen; Muttern, Schrauben, der Stangenbolzen, das Steigrohr und der Kolben liegen unter dem Merkmal oder verborgen und entfallen; Trog mit Kerbe und Ecke, Algenplatte, Wasser und drei gleich dicke 24-mm-Strahlscheiben."), tx("Thermal 1:100 by reference; structure 1:100: members as 1:50, the pivot pin and the clevis as joints, a 15 mm rod pin joint, joint rings at both flanges, masses and the slab support.", "Thermik 1:100 als Verweis; Tragwerk 1:100: Stäbe wie 1:50, Drehbolzen und Gabelkopf als Gelenke, ein 15-mm-Stangenbolzengelenk, Gelenkringe an beiden Flanschen, Massen und das Bettungsauflager.")],
        200: [tx("Body 1:200 for 30 mm: stand and head solid (the bore is hidden), the foot flange 30 mm, head with cap as one revolve, spout and lip, bracket block with the pin merged, lever 32 x 36 mm, grip, clevis (the rod is left to the structure), trough with notch and chip, algae sheet, water; the jet is below the feature and not modelled.", "Körper 1:200 für 30 mm: Ständer und Kopf voll (die Bohrung ist verborgen), Fußflansch 30 mm, Kopf mit Deckel als ein Rotationskörper, Auslauf und Lippe, Bockblock mit eingeschmolzenem Bolzen, Schwengel 32 x 36 mm, Griff, Gabelkopf (die Stange bleibt dem Tragwerk), Trog mit Kerbe und Ecke, Algenplatte, Wasser; der Strahl liegt unter dem Merkmal und wird nicht modelliert."), tx("Thermal 1:200 by reference, only the total area is compared; structure 1:200: members, the 16 mm rod as a declared slender member at its true section, 30 mm pivot and rod pin joints, flange joint rings, masses, supports.", "Thermik 1:200 als Verweis, verglichen wird nur die Gesamtfläche; Tragwerk 1:200: Stäbe, die 16-mm-Stange als erklärter schlanker Stab mit wahrem Querschnitt, 30-mm-Dreh- und Stangenbolzengelenke, Flanschgelenkringe, Massen, Auflager.")],
        500: [tx("Body 1:500 for 75 mm: slab, stand with a 40 mm foot and a bell, a 150 mm head cylinder with the cap merged, spout and lip as 75 mm blocks, a 75 mm bracket block, the lever and grip as 75 mm bars, the trough as floor and one wall ring (notch and chip are below the feature), algae sheet, water.", "Körper 1:500 für 75 mm: Bettung, Ständer mit 40-mm-Fuß und Glocke, ein 150-mm-Kopfzylinder mit eingeschmolzenem Deckel, Auslauf und Lippe als 75-mm-Blöcke, ein 75-mm-Bockblock, Schwengel und Griff als 75-mm-Stäbe, der Trog als Boden und ein Wandring (Kerbe und Ecke liegen unter dem Merkmal), Algenplatte, Wasser."), tx("Thermal 1:500 by reference, total area only; structure 1:500: the lever and the rod keep their true 30 x 40 mm and 16 mm sections as declared slender members inside the thickened body, the other members by reference, 75 mm pivot and rod pin joints, masses and supports.", "Thermik 1:500 als Verweis, nur Gesamtfläche; Tragwerk 1:500: Schwengel und Stange behalten ihre wahren Querschnitte 30 x 40 mm und 16 mm als erklärte schlanke Stäbe im verdickten Körper, die übrigen Stäbe als Verweis, 75-mm-Dreh- und Stangenbolzengelenke, Massen und Auflager.")],
    }[s]


def structure_extras(k, total):
    """🏗️ Coarse true-section members, pins and flange joints, the mass markers and the support blocks of scale k."""
    s, f = k.s, FEATURE[k.s]
    pu, pv = local("head", *PIVOT)
    if s >= 200:
        k.structure.append(rev("st-rod-true", "rod", STEEL, "body", 0.32, [(0.04, 0), (0.04, R_ROD), (U_ROD_EYE, R_ROD), (U_ROD_EYE, 0)], slender=True, member="rod", part="member"))
    if s == 500:
        k.structure = [e for e in k.structure if e.get("of") not in ("lever", "grip")]
        k.structure.append(box("st-lever-true", "lever", PAINT, "body", 0.48, [U_GRIP[1] + 0.12, 0.04, 0.03], [(U_GRIP[1] - 0.12) / 2, 0, 0], slender=True, member="lever", part="member"))
    if s > 100:
        rj = f / 2
        k.structure.append(rev("st-pivot", "head", STEEL, "detail", 0.2, [(-max(rj, Z_PIN), 0), (-max(rj, Z_PIN), rj), (max(rj, Z_PIN), rj), (max(rj, Z_PIN), 0)], (0, 0, 1), (pu, pv, 0), member="lever", part="joint"))
    if s > 50:
        rj = max(f / 2, 0.006)
        k.structure.append(rev("st-rod-pin", "lever", STEEL, "detail", 0.2, [(-0.03 if s < 500 else -0.0375, 0), (-0.03 if s < 500 else -0.0375, rj), (0.03 if s < 500 else 0.0375, rj), (0.03 if s < 500 else 0.0375, 0)], (0, 0, 1), (U_ROD_PIN, 0, 0), member="rod", part="joint"))
        h = max(f, 0.02)
        k.structure.append(rev("st-foot-joint", "column", RUST, "detail", 0.2, [(0.0, 0.0), (0.0, R_FLANGE), (h, R_FLANGE), (h, 0.0)], member="column", part="joint"))
        k.structure.append(rev("st-flange-joint", "head", PAINT, "detail", 0.2, [(-h / 2, 0.0), (-h / 2, R_HFLANGE), (h / 2, R_HFLANGE), (h / 2, 0.0)], member="head", part="joint"))
    L = X_TROUGH[1] - X_TROUGH[0]
    k.structure.append(box("st-trough-bed", "trough", GRANITE, "detail", 0.2, [L, f, 2 * Z_TROUGH], [L / 2, 0.0, 0.0], member="trough", part="joint"))
    big = {50: 0.02, 100: 0.025, 200: 0.03, 500: 0.04}[s]
    small = {50: 0.012, 100: 0.012, 200: 0.016, 500: 0.04}[s]
    for bone, (m, cx, cy) in total.items():
        u, v = local(bone, cx, cy)
        r = small if bone in ("lever", "rod") else big
        mat = {"footing": CONC, "trough": GRANITE, "water": WATER, "rod": STEEL}.get(bone, PAINT)
        k.structure.append(ell("mass-" + bone, bone, mat, [r, r, r], [u, v, 0], member=bone, part="mass"))
    su, sv = local("footing", sum(X_SLAB) / 2, Y_SLAB0)
    k.structure.append(box("support-slab-base", "footing", CONC, "detail", 0.1, [f, 0.4, 0.4], [su - f / 2, sv, 0], member="footing", part="support", anchor="slab-base"))


def bbox():
    """📦 The rest-pose envelope (x, y, z) from the 1:50 geometry: slab, trough and the lever grip."""
    xs, ys = [], []
    for u, r in grip_profile(50):
        for sgn in (1, -1):
            x, y = world("lever", u, sgn * r)
            xs.append(x)
            ys.append(y)
    for u, v in lever_profile(50):
        x, y = world("lever", u, v)
        xs.append(x)
        ys.append(y)
    xs += [X_SLAB[0], X_SLAB[1]]
    ys += [Y_SLAB0]
    return (min(xs), max(xs)), (min(ys), max(ys)), (-Z_SLAB, Z_SLAB)


def document():
    """📄 The whole asset document."""
    parts, total = masses()
    reps, vols = {}, {}
    for s in (50, 100, 200, 500):
        k = build(s)
        traces(k)
        structure_extras(k, total)
        hatches = [{"id": "concrete-dots", "title": tx("Concrete dots", "Betonpunkte"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8 if s == 50 else 0.6, "penMm": 0.13}, {"id": "granite-stipple", "title": tx("Granite stipple", "Granitpunktierung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.7 if s == 50 else 0.5, "penMm": 0.13}, {"id": "water-lines", "title": tx("Water surface lines", "Wasserlinien"), "pattern": "lines", "angleDeg": 0, "spacingMm": 1.2 if s == 50 else 0.8, "penMm": 0.13}] if s <= 100 else []
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": BANDS_RANGE[s][0], "maxDenominator": BANDS_RANGE[s][1]}, "pens": PENS[s], "hatches": hatches, "geometry": {"elevation": k.elev, "front": k.front, "plan": k.plan}, "decisions": rep_decisions(s)}
        vols[str(s)] = {"status": "authored", "denominator": s, "body": k.body, "thermal": k.thermal, "structure": k.structure, "decisions": vol_decisions(s)}
    mass_total = round(sum(t[0] for t in total.values()), 2)
    (x0, x1), (y0, y1), (z0, z1) = bbox()
    m = {b: v[0] for b, v in total.items()}
    pump = m["column"] + m["head"] + m["lever"] + m["rod"]
    asset = {
        "id": "used-pump-drinking-fountain",
        "category": "objects",
        "title": tx("Used Hand Pump Fountain", "Gebrauchte Schwengelpumpe"),
        "summary": tx("A grey cast-iron lever pump of the German park and playground type after decades of use: an 80 mm bore pump head with a long spout and a forked lever bracket on a fluted stand, bolted to a concrete bedding slab behind a 0.9 m granite trough. The green paint is chalked and flaking, the foot and the spout lip have rusted and carry limescale, the grip is polished bare by hands, the trough has a chipped corner, a tag, moss at its foot and wet algae on its shaded side, and it stands full of water to its overflow notch.", "Eine Schwengelpumpe aus Grauguss in der deutschen Park- und Spielplatzbauart nach Jahrzehnten im Gebrauch: ein Pumpenkopf mit 80 mm Bohrung, langem Auslauf und gegabeltem Schwengelbock auf einem Gussständer, verschraubt auf einer Betonbettung hinter einem 0,9 m langen Granittrog. Die grüne Farbe ist gekreidet und blättert ab, Fuß und Auslauflippe sind verrostet und verkalkt, der Griff ist von Händen blank poliert, der Trog hat eine abgeschlagene Ecke, ein Tag, Moos am Fuß und nasse Algen an der schattigen Seite und steht bis zur Überlaufkerbe voll Wasser."),
        "tags": ["used", "park", "public-space", "water", "play", "hand-pump", "lever-pump", "cast-iron", "granite-trough", "concrete-footing", "water-play", "street-furniture"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES], "notes": tx(f"Catalogue-typical playground hand pump: stand 0.66 m with 300 mm foot flange, head 130 mm with 80 mm bore up to the cap at 1.02 m, spout mouth at 0.70 m over the trough, lever pivot at 1.10 m, lever 0.64 m with a ratio of 6.2 to the rod pin, piston stroke {rod_dx(LEVER_STROKE_DEG) * 1000:.0f} mm; granite trough 0.90 x 0.60 x 0.50 m with 80 mm walls and a 100 mm floor; C25/30 bedding slab 1.32 x 0.72 x 0.25 m. Masses from volume x density: slab {m['footing']:.1f} kg, stand with riser {m['column']:.1f} kg, head {m['head']:.1f} kg, lever {m['lever']:.2f} kg, rod and piston {m['rod']:.2f} kg (pump {pump:.1f} kg), trough {m['trough']:.1f} kg, water {m['water']:.1f} kg; total {mass_total:.1f} kg. The jet mass (0.1 kg) is neglected.", f"Katalogtypische Spielplatzpumpe: Ständer 0,66 m mit 300-mm-Fußflansch, Kopf 130 mm mit 80 mm Bohrung bis zum Deckel auf 1,02 m, Auslaufmündung auf 0,70 m über dem Trog, Schwengeldrehpunkt auf 1,10 m, Schwengel 0,64 m mit einer Übersetzung von 6,2 zum Stangenbolzen, Kolbenhub {rod_dx(LEVER_STROKE_DEG) * 1000:.0f} mm; Granittrog 0,90 x 0,60 x 0,50 m mit 80 mm Wänden und 100 mm Boden; Betonbettung C25/30 1,32 x 0,72 x 0,25 m. Massen aus Volumen x Dichte: Bettung {m['footing']:.1f} kg, Ständer mit Steigrohr {m['column']:.1f} kg, Kopf {m['head']:.1f} kg, Schwengel {m['lever']:.2f} kg, Stange und Kolben {m['rod']:.2f} kg (Pumpe {pump:.1f} kg), Trog {m['trough']:.1f} kg, Wasser {m['water']:.1f} kg; gesamt {mass_total:.1f} kg. Die Masse des Strahls (0,1 kg) ist vernachlässigt.")},
        "dimensions": {"heightM": round(y1 - y0, 4), "widthM": round(x1 - x0, 4), "depthM": round(z1 - z0, 4), "massKg": mass_total},
        "materials": materials(),
        "rig": rig(total),
        "volumes": vols,
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": {
            "environment": {"airTempC": 27, "meanRadiantTempC": 31, "skyTempC": 14, "groundTempC": 32, "relativeHumidity": 0.5, "windMS": 1.5, "windDirectionDeg": 180, "solarWM2": 850, "diffuseFraction": 0.15, "pressurePa": 101325},
            "passengers": [], "operating": [], "emitters": [],
            "regions": regions(AREA),
            "assumptions": [
                tx("Passive weathered surfaces and water at summer noon in a park: absorbed sun, long-wave exchange with sky and ground, convection, evaporation from the wetted surfaces and the heat capacity of the parts; the pump draws no power (0 W) and carries no rating.", "Passive verwitterte Flächen und Wasser zur Sommermittagszeit im Park: absorbierte Sonne, Langwellenaustausch mit Himmel und Boden, Konvektion, Verdunstung von den benetzten Flächen und die Wärmekapazität der Teile; die Pumpe nimmt keine Leistung auf (0 W) und trägt keine Nennleistung."),
                tx("The chalked green paint absorbs 0.8 of the sun (new dark green alkyd about 0.88, Duffie & Beckman), so stand and head are the hot parts; rusted iron 0.86 and 0.8, the hand-polished grip only 0.35 emissivity (Incropera), weathered granite 0.58, wet algae 0.85 with water evaporating, the water body 0.8 (6 % reflected, the rest absorbed in 0.3 m of water and at the stone floor).", "Die gekreidete grüne Farbe absorbiert 0,8 der Sonne (neues dunkelgrünes Alkyd etwa 0,88, Duffie & Beckman), daher sind Ständer und Kopf die heißen Teile; verrostetes Eisen 0,86 und 0,8, der handpolierte Griff nur 0,35 Emissionsgrad (Incropera), verwitterter Granit 0,58, nasse Algen 0,85 mit verdunstendem Wasser, der Wasserkörper 0,8 (6 % reflektiert, der Rest in 0,3 m Wasser und am Steinboden absorbiert)."),
                tx("The trough water stands at about 18 degC against the stone; while a child pumps, well water of 10 degC flows in (contact fraction 0.8). The slab is 88 % in soil contact at 15 degC. Fractions are scenario data.", "Das Trogwasser steht bei etwa 18 °C am Stein; während ein Kind pumpt, fließt Brunnenwasser von 10 °C zu (Kontaktanteil 0,8). Die Bettung hat zu 88 % Erdkontakt bei 15 °C. Die Anteile sind Szenariodaten."),
            ],
        },
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["slab-base"], "assumptions": [
            tx("The soil holds the bedding slab: the bedding is idealised as one fixed support at the middle of the slab base; pump stand and trough stand on the slab (the stand bolted with four M16 anchors, the trough on a mortar bed).", "Das Erdreich hält die Betonbettung: die Bettung ist als ein festes Auflager in der Mitte der Sohle idealisiert; Pumpenständer und Trog stehen auf der Bettung (der Ständer mit vier M16-Ankern verschraubt, der Trog im Mörtelbett)."),
            tx("In maintenance the crane hook is applied as its lifting force at the lifting point, the weight of head, lever and piston (0.40 kN), because supports of the archive are springs of equal stiffness that would share the whole weight with the slab base; the slab base then carries slab, stand, trough and water alone.", "In der Wartung ist der Kranhaken als seine Hubkraft am Anschlagpunkt angesetzt, das Gewicht von Kopf, Schwengel und Kolben (0,40 kN), weil Auflager des Archivs Federn gleicher Steifigkeit sind, die das ganze Gewicht mit der Bettungssohle teilen würden; die Bettungssohle trägt dann allein Bettung, Ständer, Trog und Wasser."),
            tx("Wind after DIN EN 1991-1-4 with the German annex, zone 2 inland, peak pressure 0.65 kN/m2 and a force coefficient of 0.9 on stand and head; the lever, the water column on the piston and the child at the rim are authored estimates after the user loads of DIN EN 1176-1.", "Wind nach DIN EN 1991-1-4 mit deutschem Anhang, Zone 2 Binnenland, Böengeschwindigkeitsdruck 0,65 kN/m² und Kraftbeiwert 0,9 auf Ständer und Kopf; Schwengel, Wassersäule auf dem Kolben und das Kind am Rand sind Schätzungen nach den Nutzerlasten der DIN EN 1176-1."),
        ], "clinical": False},
        "presentation": {"defaultPose": "in-service", "defaultClip": "pumping", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    out = pathlib.Path(__file__).parent / "🔣️.json"
    doc = document()
    out.write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    parts, total = masses()
    print("[DEBUG] wrote", out)
    print("[DEBUG] masses", {b: round(v[0], 2) for b, v in total.items()}, "total", doc["asset"]["dimensions"])
