"""🎠️ Authoritative generator of the used playground roundabout: hand-chosen numbers, rig frames, geometry helpers and the whole asset document `🔣️.json` next to this file (`python3 author-roundabout.py`).

World frame of the staffage archive: x ahead, y up, z the asset's left; ground y = 0 is the top of the surrounding surfacing.

@see ../../brief.md — the wave brief
@see /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/README.md — rig, solids and 2D conventions
"""
import json
import math

G = 9.80665

R_PLAT = 0.900
GAP = 0.006
R_TRAY_IN = R_PLAT + GAP
T_TRAY = 0.003
R_TRAY_OUT = R_TRAY_IN + T_TRAY
Y_TRAY_BOT = -0.300
Y_TRAY_TOP_FLOOR = Y_TRAY_BOT + T_TRAY
R_COLLAR = 0.980
T_COLLAR = 0.008
R_APRON = 1.400
T_APRON = 0.040
HOLLOW = (1.020, 1.100, 1.220, 1.320, 0.015)
R_FOUND = 0.300
Y_FOUND_BOT = -0.900

R_HOUSING_FLANGE = 0.150
T_HOUSING_FLANGE = 0.020
R_HOUSING = 0.070
Y_HOUSING_TOP = -0.100

R_SPINDLE = 0.030
Y_SPINDLE_BOT = -0.250
Y_SPINDLE_TOP = -0.090

R_HUB = 0.08415
Y_HUB_BOT = -0.095
Y_SPOKE_BOT = -0.030
Y_SPOKE_TOP = 0.030
SPOKE_W = 0.040
R_SPOKE_END = 0.840
RIM_LEG = 0.006
Y_DECK_BOT = 0.030
Y_DECK_TOP = 0.035

R_COL = 0.05715
Y_COL_TOP = 0.735
Y_CAP_TOP = 0.750

R_TUBE = 0.01685
Y_RAIL = 0.620
R_RUN_END = 0.700
R_BEND = 0.100
R_RAIL_START = 0.030
R_FOOT = 0.040
T_FOOT = 0.008
SAG_C = 0.020

R_RING = 0.450

EYE_SHANK = (0.010, 0.030)
EYE_COLLAR = (0.020, 0.008)
EYE_CENTRE = 0.074
EYE_RING_R = 0.028
EYE_SECTION_R = 0.008
EYE_REST_U = 0.600
EYE_TRAVEL = 0.085
LIFT_TRAVEL = 0.600

RHO_STEEL = 7850.0
RHO_CAST = 7200.0
RHO_CONCRETE = 2400.0
RHO_SURFACING = 950.0
TREAD_KG_M2 = 42.4

RAIL_IDS = ["rail-a", "rail-b", "rail-c", "rail-d"]


def v_add(a, b):
    """➕️ Sum of two 3-vectors."""
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]]


def v_sub(a, b):
    """➖️ Difference of two 3-vectors."""
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def v_dot(a, b):
    """✖️ Dot product of two 3-vectors."""
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def m_mul(a, b):
    """🧮️ Product of two 3x3 matrices given as rows."""
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def m_vec(m, v):
    """🎯️ A 3x3 matrix (rows) applied to a 3-vector."""
    return [sum(m[i][k] * v[k] for k in range(3)) for i in range(3)]


def cs(deg):
    """🔄️ Cosine and sine of an angle in degrees, exact at quarter turns."""
    q = deg / 90.0
    if abs(q - round(q)) < 1e-12:
        return [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)][int(round(q)) % 4]
    r = math.radians(deg)
    return math.cos(r), math.sin(r)


def rot_w(deg):
    """🌀️ Rotation about the joint axis w: u towards v (matrix columns are the images of u, v, w)."""
    c, s = cs(deg)
    return [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]


def rot_v(deg):
    """↪️ Abduction about the normal v: u towards w."""
    c, s = cs(deg)
    return [[c, 0.0, -s], [0.0, 1.0, 0.0], [s, 0.0, c]]


def rot_u(deg):
    """🔁️ Twist about the bone axis u: v towards -w."""
    c, s = cs(deg)
    return [[1.0, 0.0, 0.0], [0.0, c, s], [0.0, -s, c]]


RIG = [
    ("ground", None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("footing", "ground", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("lift", "footing", 0.0, Y_HOUSING_TOP, 0.0, 90.0, 0.0, 0.0),
    ("rotor", "lift", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("platform", "rotor", -Y_HOUSING_TOP, 0.0, 0.0, -90.0, 0.0, 0.0),
    ("column", "platform", 0.0, Y_DECK_TOP, 0.0, 90.0, 0.0, 0.0),
    ("rail-a", "column", Y_RAIL - Y_DECK_TOP, 0.0, 0.0, -90.0, 0.0, 0.0),
    ("rail-b", "column", Y_RAIL - Y_DECK_TOP, 0.0, 0.0, -90.0, 90.0, 0.0),
    ("rail-c", "column", Y_RAIL - Y_DECK_TOP, 0.0, 0.0, -90.0, 180.0, 0.0),
    ("rail-d", "column", Y_RAIL - Y_DECK_TOP, 0.0, 0.0, -90.0, -90.0, 0.0),
    ("eyebolt", "column", EYE_REST_U, 0.0, 0.0, 0.0, 0.0, 0.0),
]


def frames():
    """🦴️ World rest frames {bone: (origin, R)} of the rig, R with the world images of u, v, w as columns: R = R(parent) · Rw · Rv · Ru, o = o(parent) + R(parent) · (x, y, z)."""
    out = {}
    for bid, parent, x, y, z, ang, abd, tw in RIG:
        rest = m_mul(m_mul(rot_w(ang), rot_v(abd)), rot_u(tw))
        if parent is None:
            out[bid] = ([x, y, z], rest)
        else:
            po, pr = out[parent]
            out[bid] = (v_add(po, m_vec(pr, [x, y, z])), m_mul(pr, rest))
    return out


FRAMES = frames()


def local(bone, p):
    """📍️ A world point in the rest frame (u, v, w) of `bone`."""
    o, r = FRAMES[bone]
    d = v_sub(p, o)
    return [sum(r[k][i] * d[k] for k in range(3)) for i in range(3)]


def world(bone, q):
    """🌍️ A bone-local point (u, v, w) in world coordinates at rest."""
    o, r = FRAMES[bone]
    return v_add(o, m_vec(r, q))


def rnd(x, n=6):
    """🔢️ A float rounded for the document, never -0."""
    v = round(float(x), n)
    return 0.0 if v == 0 else v


def pts(points, n=6):
    """🔢️ A rounded list of 2D points."""
    return [[rnd(a, n), rnd(b, n)] for a, b in points]


def quat_from_columns(x, y, z):
    """🧭️ The unit quaternion [x, y, z, w] of the rotation whose matrix columns are the images x, y, z of the section axes."""
    m00, m01, m02 = x[0], y[0], z[0]
    m10, m11, m12 = x[1], y[1], z[1]
    m20, m21, m22 = x[2], y[2], z[2]
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s]
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1.0 + m00 - m11 - m22) * 2
        q = [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    elif m11 > m22:
        s = math.sqrt(1.0 + m11 - m00 - m22) * 2
        q = [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s]
    else:
        s = math.sqrt(1.0 + m22 - m00 - m11) * 2
        q = [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s]
    n = math.sqrt(sum(c * c for c in q))
    return [rnd(c / n, 8) for c in q]


def rail_path(rail, foot_v, arc_steps=8):
    """🛤️ The centre line of one handrail in its bone frame (u radial from the column axis, v up from the rail height): run, bend of radius 0.10 m, vertical leg down to `foot_v`; rail-c carries its 20 mm vandal sag at the elbow."""
    sag = SAG_C if rail == "rail-c" else 0.0
    p0 = (R_RAIL_START, 0.0)
    p1 = (R_RUN_END, -sag)
    h0 = math.atan2(p1[1] - p0[1], p1[0] - p0[0])
    centre = (p1[0] + R_BEND * math.sin(h0), p1[1] - R_BEND * math.cos(h0))
    path = [p0, p1]
    for k in range(1, arc_steps + 1):
        h = h0 + (-math.pi / 2 - h0) * k / arc_steps
        path.append((centre[0] - R_BEND * math.sin(h), centre[1] + R_BEND * math.cos(h)))
    path.append((path[-1][0], foot_v))
    return path


def tangents(path):
    """➡️ Unit tangents of a polyline (central differences, one-sided at the ends)."""
    out = []
    for i in range(len(path)):
        a = path[max(i - 1, 0)]
        b = path[min(i + 1, len(path) - 1)]
        d = (b[0] - a[0], b[1] - a[1])
        n = math.hypot(*d)
        out.append((d[0] / n, d[1] / n))
    return out


def offset_outline(path, r):
    """🧵️ The closed outline of a tube of radius `r` along a planar centre line (both offsets joined at the ends)."""
    t = tangents(path)
    left = [(p[0] - tt[1] * r, p[1] + tt[0] * r) for p, tt in zip(path, t)]
    right = [(p[0] + tt[1] * r, p[1] - tt[0] * r) for p, tt in zip(path, t)]
    return left + right[::-1]


def path_length(path):
    """📏️ Length of a polyline."""
    return sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(path, path[1:]))


def loft_tube(path, r):
    """🪈️ Loft sections of a round tube along a centre line in the (u, v) plane: section x the in-plane normal, y the lateral w, z the tangent."""
    out = []
    for p, tt in zip(path, tangents(path)):
        z = (tt[0], tt[1], 0.0)
        y = (0.0, 0.0, 1.0)
        x = (-tt[1], tt[0], 0.0)
        out.append({"at": [rnd(p[0]), rnd(p[1]), 0.0], "rx": r, "ry": r, "exponent": 2, "rotation": quat_from_columns(x, y, z)})
    return out


def pappus(profile, closed_loop):
    """⚙️ Volume of a revolve of a profile [axial, radial] about its axis (Pappus): a closed loop as it is, an open profile closed through the axis."""
    ring = list(profile)
    if closed_loop and ring[0] == ring[-1]:
        ring = ring[:-1]
    area = 0.0
    cr = 0.0
    n = len(ring)
    for i in range(n):
        a0, r0 = ring[i]
        a1, r1 = ring[(i + 1) % n]
        cross = a0 * r1 - a1 * r0
        area += cross
        cr += (r0 + r1) * cross
    area *= 0.5
    if abs(area) < 1e-15:
        return 0.0
    cr /= 6 * area
    return abs(area) * 2 * math.pi * abs(cr)


def circle_path(cx, cy, r, n=48):
    """⭕️ An SVG subpath of a circle as a polygon of `n` points."""
    p = [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]
    return "M " + " L ".join(f"{rnd(a, 5)} {rnd(b, 5)}" for a, b in p) + " Z"


def annulus_path(r0, r1, n=72):
    """🍩️ An SVG path of an annulus about the origin (outer ring and its hole)."""
    return circle_path(0.0, 0.0, r1, n) + " " + circle_path(0.0, 0.0, r0, n)


def sector_path(r0, r1, a0, a1, n=16):
    """🍰️ An SVG path of an annular sector between radii r0, r1 and angles a0, a1 (degrees, from +x towards +z = paper up)."""
    outer = [(r1 * math.cos(math.radians(a0 + (a1 - a0) * k / n)), r1 * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]
    inner = [(r0 * math.cos(math.radians(a1 - (a1 - a0) * k / n)), r0 * math.sin(math.radians(a1 - (a1 - a0) * k / n))) for k in range(n + 1)]
    return "M " + " L ".join(f"{rnd(a, 5)} {rnd(b, 5)}" for a, b in outer + inner) + " Z"


def ring_profile(axial, radial, r, n=16):
    """🧿️ A closed profile loop [axial, radial] of a round section of radius `r` (a torus or hoop when revolved), first point repeated last."""
    p = [[rnd(axial + r * math.sin(2 * math.pi * k / n)), rnd(radial + r * math.cos(2 * math.pi * k / n))] for k in range(n)]
    return p + [p[0]]


SCALES = [50, 100, 200, 500]
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.030, 500: 0.075}
BANDS = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
PENS = {
    50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7},
    100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5},
    200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
    500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}


def T(en, de):
    """🌍️ A bilingual text, English first."""
    return {"en": en, "de": de}


def revolve(sid, bone, mat, tone, profile, axis, origin=(0.0, 0.0, 0.0), role="body", **flags):
    """🏺️ A revolve solid of a profile [axial, radial]."""
    s = {"id": sid, "kind": "revolve", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": [[rnd(a), rnd(r)] for a, r in profile], "axis": list(axis), "origin": [rnd(c) for c in origin]}
    s.update(flags)
    return s


def box(sid, bone, mat, tone, size, at, role="body", **flags):
    """📦️ A box solid centred at `at` (bone-local)."""
    s = {"id": sid, "kind": "box", "bone": bone, "material": mat, "role": role, "tone": tone, "size": [rnd(c) for c in size], "transform": {"translation": [rnd(c) for c in at]}}
    s.update(flags)
    return s


def ellipsoid(sid, bone, mat, tone, radii, at, role="detail", **flags):
    """🥚️ An ellipsoid solid centred at `at` (bone-local)."""
    s = {"id": sid, "kind": "ellipsoid", "bone": bone, "material": mat, "role": role, "tone": tone, "radii": [rnd(c) for c in radii], "transform": {"translation": [rnd(c) for c in at]}}
    s.update(flags)
    return s


def capsule(sid, bone, mat, tone, p0, p1, r, role="body", **flags):
    """💊️ A capsule solid between two bone-local points."""
    s = {"id": sid, "kind": "capsule", "bone": bone, "material": mat, "role": role, "tone": tone, "p0": [rnd(c) for c in p0], "p1": [rnd(c) for c in p1], "r0": rnd(r), "r1": rnd(r)}
    s.update(flags)
    return s


def loft(sid, bone, mat, tone, sections, role="body", **flags):
    """🪈️ A loft solid through superelliptic sections."""
    s = {"id": sid, "kind": "loft", "bone": bone, "material": mat, "role": role, "tone": tone, "sections": sections, "capStart": True, "capEnd": True}
    s.update(flags)
    return s


def ref(sid, of, **labels):
    """🔗️ A thermal or structural reference to a body solid of the same scale."""
    s = {"id": sid, "of": of}
    s.update(labels)
    return s


def shape(sid, bone, role, z, tone, geom, region=None, filled=True, hatch=None, pen=None, material=None, solid=None, regions=None):
    """✏️ One authored 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    if region is not None:
        s["region"] = region
    if regions is not None:
        s["regions"] = regions
    if material is not None:
        s["material"] = material
    if hatch is not None:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    if solid is not None:
        s["solid"] = solid
    return s


def poly(points, closed=True):
    """🔷️ Polygon geometry."""
    return {"kind": "poly", "points": pts(points), "closed": closed}


def rect(x0, y0, x1, y1):
    """⬛️ Rectangle geometry from two corners."""
    return {"kind": "rect", "x": rnd(min(x0, x1)), "y": rnd(min(y0, y1)), "w": rnd(abs(x1 - x0)), "h": rnd(abs(y1 - y0))}


def circle(cx, cy, r):
    """⭕️ Circle geometry."""
    return {"kind": "ellipse", "cx": rnd(cx), "cy": rnd(cy), "rx": rnd(r), "ry": rnd(r)}


def path(d):
    """〰️ Path geometry."""
    return {"kind": "path", "d": d}


def revolved(bone, axis):
    """🌀️ The revolved 2D solid about the rotor axis through the origin of `bone`."""
    return {"kind": "revolved", "axisBone": bone, "axis": axis, "axisX": 0, "axisZ": 0, "mirror": True}


def extruded(z0, z1):
    """🧱️ The extruded 2D solid swept along w over the rest lateral span z0..z1."""
    return {"kind": "extruded", "sweep": "w", "z0": rnd(z0), "z1": rnd(z1)}


def rect_uv(u0, v0, u1, v1):
    """⬛️ A rectangle as a closed polygon in a bone plane."""
    return [(u0, v0), (u1, v0), (u1, v1), (u0, v1)]


MATERIALS = [
    {"id": "steel-galv-weathered", "title": T("Hot-dip galvanised steel, weathered dull", "Feuerverzinkter Stahl, matt verwittert"), "emissivity": 0.28, "solarAbsorptivity": 0.89, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 355e6, "source": "literature", "reference": "duffie-beckman-2013"},
    {"id": "steel-galv-new", "title": T("Hot-dip galvanised steel, new (replaced handrail)", "Feuerverzinkter Stahl, neu (ersetzter Haltebügel)"), "emissivity": 0.13, "solarAbsorptivity": 0.65, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "duffie-beckman-2013"},
    {"id": "steel-powder-red", "title": T("Powder-coated steel, traffic red, chalked and chipped", "Pulverbeschichteter Stahl, verkehrsrot, ausgekreidet und abgeplatzt"), "emissivity": 0.9, "solarAbsorptivity": 0.62, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "authored-estimate", "reference": "levinson-2007"},
    {"id": "steel-c45", "title": T("Bright turned steel C45 (spindle, eyebolt), greased", "Blanker Drehstahl C45 (Achse, Ringschraube), gefettet"), "emissivity": 0.35, "solarAbsorptivity": 0.6, "densityKgM3": RHO_STEEL, "conductivityWmK": 45, "specificHeatJKgK": 470, "elasticModulusPa": 210e9, "strengthPa": 430e6, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "cast-iron", "title": T("Grey cast iron bearing housing, rust-streaked", "Lagergehäuse aus Grauguss, mit Roststreifen"), "emissivity": 0.8, "solarAbsorptivity": 0.75, "densityKgM3": RHO_CAST, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 110e9, "source": "literature", "reference": "incropera-2018"},
    {"id": "concrete", "title": T("Plain concrete C25/30 (point foundation)", "Unbewehrter Beton C25/30 (Punktfundament)"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": RHO_CONCRETE, "conductivityWmK": 1.65, "specificHeatJKgK": 1000, "elasticModulusPa": 31e9, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "epdm-surfacing", "title": T("Poured EPDM safety surfacing on SBR base, faded green", "Gegossener EPDM-Fallschutzbelag auf SBR-Tragschicht, ausgeblichenes Grün"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": RHO_SURFACING, "conductivityWmK": 0.17, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "din-en-1177-2018"},
]


def apron_profile(s):
    """🟩️ The closed loop [y, r] of the poured surfacing apron with the hollow its users' feet have worn into it."""
    t = 0.075 if s == 500 else T_APRON
    r0, r1, r2, r3, d = HOLLOW
    if s == 50:
        top = [(0.0, R_APRON), (0.0, r3), (-0.010, 1.270), (-d, r2), (-d, r1), (-0.010, 1.060), (0.0, r0), (0.0, R_COLLAR)]
    elif s == 100:
        top = [(0.0, R_APRON), (0.0, r3), (-d, 1.240), (-d, 1.080), (0.0, r0), (0.0, R_COLLAR)]
    else:
        top = [(0.0, R_APRON), (0.0, R_COLLAR)]
    loop = [(-t, R_COLLAR), (-t, R_APRON)] + top
    return loop + [loop[0]]


def tray_profile(s):
    """🥘️ The open profile [y, r] of the galvanised pit tray with its collar (closed through the axis)."""
    collar = {50: T_COLLAR, 100: T_COLLAR, 200: 0.030, 500: 0.075}[s]
    return [(Y_TRAY_BOT, 0.0), (Y_TRAY_BOT, R_TRAY_OUT), (-collar, R_TRAY_OUT), (-collar, R_COLLAR), (0.0, R_COLLAR), (0.0, R_TRAY_IN), (Y_TRAY_TOP_FLOOR, R_TRAY_IN), (Y_TRAY_TOP_FLOOR, 0.0)]


FOUND_PROFILE = [(Y_FOUND_BOT, 0.0), (Y_FOUND_BOT, R_FOUND), (Y_TRAY_BOT, R_FOUND), (Y_TRAY_BOT, 0.0)]


def housing_profile(s):
    """⚙️ The open profile [y, r] of the cast bearing housing on the tray floor."""
    if s in (50, 100):
        return [(Y_TRAY_TOP_FLOOR, 0.0), (Y_TRAY_TOP_FLOOR, R_HOUSING_FLANGE), (Y_TRAY_TOP_FLOOR + T_HOUSING_FLANGE, R_HOUSING_FLANGE), (Y_TRAY_TOP_FLOOR + T_HOUSING_FLANGE, R_HOUSING), (Y_HOUSING_TOP, R_HOUSING), (Y_HOUSING_TOP, 0.0)]
    return [(Y_TRAY_TOP_FLOOR, 0.0), (Y_TRAY_TOP_FLOOR, R_HOUSING), (Y_HOUSING_TOP, R_HOUSING), (Y_HOUSING_TOP, 0.0)]


SPINDLE_PROFILE = [(Y_SPINDLE_BOT + 0.1, 0.0), (Y_SPINDLE_BOT + 0.1, R_SPINDLE), (Y_SPINDLE_TOP + 0.1, R_SPINDLE), (Y_SPINDLE_TOP + 0.1, 0.0)]
ROTOR_500_PROFILE = [(-0.015, 0.0), (-0.015, R_HUB), (0.070, R_HUB), (0.070, 0.0)]
HUB_PROFILE = [(Y_HUB_BOT, 0.0), (Y_HUB_BOT, R_HUB), (Y_DECK_BOT, R_HUB), (Y_DECK_BOT, 0.0)]
RIM_PROFILE = [(Y_SPOKE_BOT, R_SPOKE_END), (Y_SPOKE_BOT, R_PLAT), (Y_DECK_BOT, R_PLAT), (Y_DECK_BOT, R_SPOKE_END), (Y_SPOKE_BOT, R_SPOKE_END)]
DECK_PROFILE = [(Y_DECK_BOT, 0.0), (Y_DECK_BOT, R_PLAT), (Y_DECK_TOP, R_PLAT), (Y_DECK_TOP, 0.0)]
SHELL_100_PROFILE = [(Y_DECK_BOT, 0.0), (Y_DECK_BOT, R_PLAT - RIM_LEG), (Y_SPOKE_BOT, R_PLAT - RIM_LEG), (Y_SPOKE_BOT, R_PLAT), (Y_DECK_TOP, R_PLAT), (Y_DECK_TOP, 0.0)]
DISC_200_PROFILE = [(Y_HUB_BOT, 0.0), (Y_HUB_BOT, R_HUB), (Y_SPOKE_BOT, R_HUB), (Y_SPOKE_BOT, R_PLAT), (Y_DECK_TOP, R_PLAT), (Y_DECK_TOP, 0.0)]
DISC_500_PROFILE = [(-0.040, 0.0), (-0.040, R_PLAT), (Y_DECK_TOP, R_PLAT), (Y_DECK_TOP, 0.0)]
H_COL = Y_COL_TOP - Y_DECK_TOP
H_CAP = Y_CAP_TOP - Y_DECK_TOP


def column_profile(s):
    """🗼️ The open profile [u, r] of the centre column (u up from the deck) with its domed, chamfered or flat cap."""
    if s == 50:
        cap = [(H_COL, R_COL), (H_COL + 0.005, 0.0535), (H_COL + 0.010, 0.044), (H_COL + 0.014, 0.028), (H_CAP, 0.0)]
    elif s == 100:
        cap = [(H_COL, R_COL), (H_CAP, 0.030), (H_CAP, 0.0)]
    else:
        cap = [(H_CAP, R_COL), (H_CAP, 0.0)]
    return [(0.0, 0.0), (0.0, R_COL)] + cap


RING_U = Y_RAIL - Y_DECK_TOP
FOOT_V = {50: Y_DECK_TOP + T_FOOT - Y_RAIL, 100: Y_DECK_TOP - Y_RAIL, 200: Y_DECK_TOP - Y_RAIL, 500: Y_DECK_TOP - Y_RAIL}
ARC_STEPS = {50: 8, 100: 6, 200: 4, 500: 3}
RING_SEGMENTS = {50: 16, 100: 12, 200: 8, 500: 8}


def rail_material(rail):
    """🎨️ The replaced rail-d is bare new galvanised tube, the others the chalked red powder coat."""
    return "steel-galv-new" if rail == "rail-d" else "steel-powder-red"


def rail_region(rail):
    """🌡️ The thermal region of a handrail."""
    return "rail-new" if rail == "rail-d" else "rails"


def rail_tone(rail):
    """🎚️ The grey of a handrail in the tone views."""
    return 0.72 if rail == "rail-d" else 0.42


def eye_profile(n=16):
    """🪝️ The closed profile [axial along w, radial in the u-v plane] of the DIN 580 eye ring."""
    return ring_profile(0.0, EYE_RING_R, EYE_SECTION_R, n)


EYE_SHANK_PROFILE = [(0.0, 0.0), (0.0, EYE_SHANK[0]), (EYE_SHANK[1], EYE_SHANK[0]), (EYE_SHANK[1], EYE_COLLAR[0]), (EYE_SHANK[1] + EYE_COLLAR[1], EYE_COLLAR[0]), (EYE_SHANK[1] + EYE_COLLAR[1], 0.0)]


def body(s):
    """🧊️ The body solids of one scale: each scale its own model for its smallest feature."""
    out = []
    Y = [0.0, 1.0, 0.0]
    U = [1.0, 0.0, 0.0]
    out.append(revolve("apron", "footing", "epdm-surfacing", 0.3, apron_profile(s), Y))
    out.append(revolve("tray", "footing", "steel-galv-weathered", 0.62, tray_profile(s), Y))
    out.append(revolve("foundation", "footing", "concrete", 0.82, FOUND_PROFILE, Y))
    out.append(revolve("housing", "footing", "cast-iron", 0.3, housing_profile(s), Y))
    if s == 500:
        out.append(revolve("rotor-boss", "rotor", "steel-galv-weathered", 0.35, ROTOR_500_PROFILE, U))
        out.append(revolve("platform", "platform", "steel-galv-weathered", 0.55, DISC_500_PROFILE, Y))
    else:
        out.append(revolve("spindle", "rotor", "steel-c45", 0.25, SPINDLE_PROFILE, U))
        if s == 200:
            out.append(revolve("platform", "platform", "steel-galv-weathered", 0.55, DISC_200_PROFILE, Y))
        else:
            out.append(revolve("hub", "platform", "steel-galv-weathered", 0.4, HUB_PROFILE, Y))
            out.append(box("spokes", "platform", "steel-galv-weathered", 0.45, [R_SPOKE_END - R_HUB, Y_SPOKE_TOP - Y_SPOKE_BOT, SPOKE_W], [(R_SPOKE_END + R_HUB) / 2, 0.0, 0.0], repeat={"count": 8, "rotationAxis": [0.0, 1.0, 0.0], "rotationRad": rnd(math.pi / 4, 8), "origin": [0.0, 0.0, 0.0]}))
            if s == 50:
                out.append(revolve("rim", "platform", "steel-galv-weathered", 0.5, RIM_PROFILE, Y))
                out.append(revolve("deck", "platform", "steel-galv-weathered", 0.58, DECK_PROFILE, Y, sheet=True))
            else:
                out.append(revolve("platform", "platform", "steel-galv-weathered", 0.55, SHELL_100_PROFILE, Y))
    out.append(revolve("column", "column", "steel-powder-red", 0.42, column_profile(s), U))
    out.append(revolve("ring", "column", "steel-powder-red", 0.42, ring_profile(RING_U, R_RING, R_TUBE, RING_SEGMENTS[s]), U, slender=True))
    for rail in RAIL_IDS:
        p = rail_path(rail, FOOT_V[s], ARC_STEPS[s])
        flags = {"slender": True} if s == 500 else {}
        out.append(loft(rail, rail, rail_material(rail), rail_tone(rail), loft_tube(p, R_TUBE), **flags))
        if s == 50:
            fu = p[-1][0]
            out.append(revolve(f"foot-{rail[-1]}", rail, rail_material(rail), 0.36, [(FOOT_V[50] - T_FOOT, 0.0), (FOOT_V[50] - T_FOOT, R_FOOT), (FOOT_V[50], R_FOOT), (FOOT_V[50], 0.0)], Y, origin=(fu, 0.0, 0.0)))
    if s in (50, 100):
        out.append(revolve("eye-shank", "eyebolt", "steel-c45", 0.3, EYE_SHANK_PROFILE, U))
        out.append(revolve("eye-ring", "eyebolt", "steel-c45", 0.3, eye_profile(16 if s == 50 else 12), [0.0, 0.0, 1.0], origin=(EYE_CENTRE, 0.0, 0.0)))
    return out


def thermal_list(s, solids):
    """🌡️ The thermal solids of one scale: references to the body where the thermal surface is the body surface, plus the seal band that closes the 6 mm running gap of the pit."""
    region = {"apron": "surfacing", "tray": "pit-tray", "foundation": "foundation", "housing": "underframe", "spindle": "underframe", "rotor-boss": "underframe", "hub": "underframe", "spokes": "underframe", "rim": "deck", "deck": "deck", "platform": "deck", "column": "rails", "ring": "rails"}
    out = []
    for b in solids:
        sid = b["id"]
        if sid.startswith("eye-"):
            continue
        if region.get(sid) == "underframe":
            own = {k: v for k, v in b.items() if k != "id"}
            out.append({"id": f"th-{sid}", **own, "region": "underframe"})
            continue
        if sid in RAIL_IDS or sid.startswith("foot-"):
            rail = sid if sid in RAIL_IDS else "rail-" + sid[-1]
            out.append(ref(f"th-{sid}", sid, region=rail_region(rail)))
        else:
            out.append(ref(f"th-{sid}", sid, region=region[sid]))
    y0 = {50: Y_SPOKE_BOT, 100: -0.025, 200: -0.020, 500: -0.040}[s]
    seal = [(y0, R_PLAT - RIM_LEG), (y0, R_TRAY_OUT), (0.0, R_TRAY_OUT), (0.0, R_PLAT - RIM_LEG), (y0, R_PLAT - RIM_LEG)]
    out.append(revolve("th-seal", "platform", "steel-galv-weathered", 0.5, seal, [0.0, 1.0, 0.0], region="deck"))
    return out


def structure_list(s, solids, com):
    """🏗️ The structural idealisation of one scale: members on their bones with their true sections, joints at the rig joints, the two supports at their anchors and mass markers at the centres of mass."""
    f = FEATURE[s]
    ids = {b["id"] for b in solids}
    Y = [0.0, 1.0, 0.0]
    U = [1.0, 0.0, 0.0]
    out = []
    out.append(revolve("member-footing", "footing", "cast-iron", 0.3, [(Y_FOUND_BOT, 0.0), (Y_FOUND_BOT, R_HOUSING), (Y_HOUSING_TOP, R_HOUSING), (Y_HOUSING_TOP, 0.0)], Y, member="footing", part="member"))
    rr = 0.0375 if s == 500 else R_SPINDLE
    out.append(revolve("member-rotor", "rotor", "steel-c45", 0.25, [(-0.15, 0.0), (-0.15, rr), (0.13, rr), (0.13, 0.0)], U, member="rotor", part="member"))
    if "hub" in ids:
        out.append(ref("joint-hub", "hub", member="platform", part="joint"))
    else:
        out.append(revolve("joint-hub", "platform", "steel-galv-weathered", 0.4, [(-0.040, 0.0), (-0.040, R_HUB), (Y_DECK_TOP, R_HUB), (Y_DECK_TOP, 0.0)], Y, member="platform", part="joint"))
    if "spokes" in ids:
        out.append(ref("member-spokes", "spokes", member="platform", part="member"))
    else:
        out.append(box("member-spokes", "platform", "steel-galv-weathered", 0.45, [R_SPOKE_END - R_HUB, Y_SPOKE_TOP - Y_SPOKE_BOT, SPOKE_W], [(R_SPOKE_END + R_HUB) / 2, 0.0, 0.0], repeat={"count": 8, "rotationAxis": [0.0, 1.0, 0.0], "rotationRad": rnd(math.pi / 4, 8), "origin": [0.0, 0.0, 0.0]}, member="platform", part="member", slender=True))
    out.append(ref("member-deck", "deck" if "deck" in ids else "platform", member="platform", part="member"))
    members_col = {} if s in (50, 100) else {"members": ["eyebolt"]}
    if s in (50, 100):
        out.append(ref("member-column", "column", member="column", part="member"))
    else:
        out.append(revolve("member-column", "column", "steel-powder-red", 0.42, [(0.0, 0.0), (0.0, R_COL), (H_CAP, R_COL), (H_CAP, 0.0)], U, member="column", part="member", **members_col))
    out.append(ref("member-ring", "ring", member="column", part="member"))
    for rail in RAIL_IDS:
        out.append(ref(f"member-{rail}", rail, member=rail, part="member"))
    if s in (50, 100):
        out.append(capsule("member-eyebolt", "eyebolt", "steel-c45", 0.3, [0.0, 0.0, 0.0], [EYE_CENTRE + EYE_RING_R - EYE_SECTION_R, 0.0, 0.0], EYE_SHANK[0], member="eyebolt", part="member"))
    jr = {50: 0.02, 100: 0.025, 200: 0.035, 500: 0.08}[s]
    out.append(revolve("joint-bearing", "footing", "cast-iron", 0.3, [(Y_HOUSING_TOP - jr, 0.0), (Y_HOUSING_TOP - jr, R_HOUSING), (Y_HOUSING_TOP, R_HOUSING), (Y_HOUSING_TOP, 0.0)], Y, member="rotor", part="joint"))
    out.append(revolve("joint-column-foot", "column", "steel-powder-red", 0.42, [(0.0, 0.0), (0.0, R_COL), (jr, R_COL), (jr, 0.0)], U, member="column", part="joint"))
    for rail in RAIL_IDS:
        out.append(ellipsoid(f"joint-{rail}", rail, "steel-powder-red", 0.2, [jr, min(jr, R_TUBE * 1.2) if s in (50, 100) else jr, min(jr, R_TUBE * 1.2) if s in (50, 100) else jr], [R_COL, 0.0, 0.0], member=rail, part="joint"))
    sf = {50: 0.02, 100: 0.03, 200: 0.04, 500: 0.08}[s]
    out.append(box("support-footing", "footing", "concrete", 0.15, [0.2, sf, 0.2], [0.0, Y_FOUND_BOT + sf / 2, 0.0], member="footing", part="support", anchor="footing-fixed"))
    mr = {50: 0.015, 100: 0.02, 200: 0.035, 500: 0.08}[s]
    for bone, (cu, cv, cw) in com.items():
        out.append(ellipsoid(f"mass-{bone}", bone, "steel-galv-weathered", 0.2, [mr, mr, mr], [cu, cv, cw], member=bone, part="mass"))
    return out


def swap(profile):
    """🔀️ A [y, r] profile as (u = r, v = y) points of a horizontal bone (the platform frame)."""
    return [(r, y) for y, r in profile]


def open_ring(profile):
    """✂️ A closed loop without its repeated last point."""
    return profile[:-1] if profile[0] == profile[-1] else profile


def views(s):
    """🖼️ The three authored views of one scale: footing shapes plain in world coordinates, every rotating part as a revolved or extruded solid so it reads right at every twist of the rotor."""
    f = FEATURE[s]
    t_apron = 0.075 if s == 500 else T_APRON
    collar = {50: T_COLLAR, 100: T_COLLAR, 200: 0.030, 500: 0.075}[s]
    el, fr, pl = [], [], []
    tray_outline = [(-R_COLLAR, 0.0), (R_COLLAR, 0.0), (R_COLLAR, -collar), (R_TRAY_OUT, -collar), (R_TRAY_OUT, Y_TRAY_BOT), (-R_TRAY_OUT, Y_TRAY_BOT), (-R_TRAY_OUT, -collar), (-R_COLLAR, -collar)]
    hatch_concrete = "concrete" if s != 500 else None
    tray_half = [(0.0, 0.0), (R_COLLAR, 0.0), (R_COLLAR, -collar), (R_TRAY_OUT, -collar), (R_TRAY_OUT, Y_TRAY_BOT), (0.0, Y_TRAY_BOT)]
    for lst in (el, fr):
        lst.append(shape("foundation", "footing", "body", 2, 0.82, poly(rect_uv(0.0, Y_FOUND_BOT, R_FOUND, Y_TRAY_BOT)), region="foundation", hatch=hatch_concrete, material="concrete", solid=revolved("footing", "v")))
        lst.append(shape("tray", "footing", "body", 2.5, 0.62, poly(tray_half), region="pit-tray", material="steel-galv-weathered", solid=revolved("footing", "v")))
        lst.append(shape("apron", "footing", "body", 3, 0.3, poly(rect_uv(0.0, -t_apron, R_APRON, 0.0)), region="surfacing", material="epdm-surfacing", solid=revolved("footing", "v")))
    if s in (50, 100):
        for lst in (el, fr):
            lst.append(shape("tray-seam", "footing", "detail", 2.6, 0.5, poly(rect_uv(0.0, -0.152, R_TRAY_OUT, -0.148)), region="pit-tray", solid=revolved("footing", "v")))
    rot = []
    if s == 500:
        rot.append(shape("rotor-boss", "rotor", "body", 1.5, 0.35, poly(ROTOR_500_PROFILE), region="underframe", solid=revolved("rotor", "u")))
        rot.append(shape("platform", "platform", "body", 3.5, 0.55, poly(swap(DISC_500_PROFILE)), region="deck", solid=revolved("platform", "v")))
    else:
        rot.append(shape("spindle", "rotor", "body", 1.5, 0.25, poly(SPINDLE_PROFILE), region="underframe", solid=revolved("rotor", "u")))
        if s == 200:
            rot.append(shape("platform", "platform", "body", 1.7, 0.55, poly(swap(DISC_200_PROFILE)), region="deck", solid=revolved("platform", "v")))
        else:
            rot.append(shape("hub", "platform", "body", 1.6, 0.4, poly(swap(HUB_PROFILE)), region="underframe", solid=revolved("platform", "v")))
            if s == 50:
                rot.append(shape("rim", "platform", "body", 1.7, 0.5, poly(rect_uv(0.0, Y_SPOKE_BOT, R_PLAT, Y_DECK_BOT)), region="deck", solid=revolved("platform", "v")))
                rot.append(shape("deck", "platform", "body", 1.8, 0.58, poly(rect_uv(0.0, Y_DECK_BOT, R_PLAT, Y_DECK_TOP)), region="deck", solid=revolved("platform", "v")))
            else:
                rot.append(shape("platform", "platform", "body", 1.7, 0.55, poly(rect_uv(0.0, Y_SPOKE_BOT, R_PLAT, Y_DECK_TOP)), region="deck", solid=revolved("platform", "v")))
    rot.append(shape("column", "column", "body", 5, 0.42, poly(column_profile(s)), region="rails", material="steel-powder-red", solid=revolved("column", "u")))
    rot.append(shape("ring", "column", "body", 6, 0.42, poly(rect_uv(RING_U - R_TUBE, 0.0, RING_U + R_TUBE, R_RING + R_TUBE)), region="rails", solid=revolved("column", "u")))
    if s in (50, 100):
        rot.append(shape("column-foot-rust", "column", "detail", 5.1, 0.18, poly(rect_uv(0.0, -R_COL, 0.03 if s == 50 else 0.04, R_COL)), region="rails", solid=extruded(-R_COL, R_COL)))
    for rail in RAIL_IDS:
        p = rail_path(rail, FOOT_V[s], ARC_STEPS[s])
        fu = p[-1][0]
        rot.append(shape(rail, rail, "body", 5.5, rail_tone(rail), poly(offset_outline(p, R_TUBE)), region=rail_region(rail), material=rail_material(rail), solid=extruded(-R_TUBE, R_TUBE)))
        if s == 50:
            rot.append(shape(f"foot-{rail[-1]}", rail, "detail", 5.6, 0.36, poly(rect_uv(fu - R_FOOT, FOOT_V[50] - T_FOOT, fu + R_FOOT, FOOT_V[50])), region=rail_region(rail), solid=extruded(-R_FOOT, R_FOOT)))
        if s in (50, 100) and rail != "rail-d":
            rot.append(shape(f"rust-{rail[-1]}", rail, "detail", 5.7, 0.15, poly(rect_uv(fu - R_TUBE, FOOT_V[s], fu + R_TUBE, FOOT_V[s] + (0.045 if s == 50 else 0.06))), region="rails", solid=extruded(-R_TUBE, R_TUBE)))
        if s == 50 and rail != "rail-d":
            rot.append(shape(f"rubbed-{rail[-1]}", rail, "texture", 5.8, 0.88, poly(rect_uv(0.12, R_TUBE - 0.007, 0.66, R_TUBE)), region="rails", solid=extruded(-R_TUBE, R_TUBE)))
    if s in (50, 100):
        eye = circle_path(EYE_CENTRE, 0.0, EYE_RING_R + EYE_SECTION_R, 32) + " " + circle_path(EYE_CENTRE, 0.0, EYE_RING_R - EYE_SECTION_R, 32)
        rot.append(shape("eye-ring", "eyebolt", "detail", 4.6, 0.3, path(eye), solid=extruded(-EYE_SECTION_R, EYE_SECTION_R)))
        rot.append(shape("eye-shank", "eyebolt", "detail", 4.7, 0.3, poly(EYE_SHANK_PROFILE), solid=revolved("eyebolt", "u")))
    el += rot
    fr += rot
    if s == 50:
        el.append(shape("sticker", "column", "texture", 5.2, 0.9, poly(rect_uv(0.34, -0.026, 0.40, 0.026)), solid=extruded(-R_COL - 0.002, -R_COL + 0.004)))
        el.append(shape("tag", "column", "texture", 5.2, 0.12, poly([(0.18, -0.03), (0.22, -0.01), (0.20, 0.02), (0.25, 0.035), (0.24, -0.005), (0.19, -0.035)]), solid=extruded(-R_COL - 0.002, -R_COL + 0.004)))
        fr.append(shape("sticker-front", "column", "texture", 5.2, 0.9, poly(rect_uv(0.30, R_COL - 0.004, 0.37, R_COL + 0.002)), solid=extruded(-0.03, 0.03)))
    if s == 500:
        pl.append(shape("floor", "footing", "body", 1, 0.7, circle(0.0, 0.0, R_TRAY_IN), region="pit-tray"))
        pl.append(shape("apron", "footing", "body", 2, 0.3, path(annulus_path(R_TRAY_IN, R_APRON)), regions=["surfacing", "pit-tray"], material="epdm-surfacing"))
    else:
        pl.append(shape("floor", "footing", "body", 1, 0.7, circle(0.0, 0.0, R_TRAY_IN), region="pit-tray"))
        pl.append(shape("apron", "footing", "body", 2, 0.3, path(annulus_path(R_COLLAR, R_APRON)), region="surfacing", material="epdm-surfacing"))
        pl.append(shape("collar", "footing", "body", 3, 0.62, path(annulus_path(R_TRAY_IN, R_COLLAR)), region="pit-tray", material="steel-galv-weathered"))
        role = "texture" if s in (50, 100) else "detail"
        pl.append(shape("worn-hollow", "footing", role, 2.5, 0.16, path(annulus_path(HOLLOW[0] + 0.02, HOLLOW[3] - 0.02)), region="surfacing", hatch="gravel" if s in (50, 100) else None))
        if s in (50, 100):
            pl.append(shape("moss", "footing", "texture", 2.6, 0.45, path(sector_path(1.30, R_APRON - 0.015, 60, 150)), region="surfacing", hatch="moss"))
    deck_bone_shape = "deck" if s == 50 else "platform"
    pl.append(shape(deck_bone_shape, "platform", "body", 4, 0.58, poly(rect_uv(0.0, Y_DECK_BOT, R_PLAT, Y_DECK_TOP)), region="deck", material="steel-galv-weathered", hatch="chequer" if s in (50, 100) else None, solid=revolved("platform", "v")))
    if s != 500:
        pl.append(shape("worn-path", "platform", "texture" if s in (50, 100) else "detail", 4.5, 0.8, path(annulus_path(0.56, 0.85)), region="deck"))
    if s == 50:
        for k, (r, a) in enumerate([(0.62, 20), (0.71, 75), (0.66, 160), (0.78, 205), (0.6, 250), (0.74, 330)]):
            pl.append(shape(f"gum-{k}", "platform", "texture", 4.6, 0.1, circle(r * math.cos(math.radians(a)), r * math.sin(math.radians(a)), 0.012), region="deck"))
    for rail in RAIL_IDS:
        p = rail_path(rail, FOOT_V[s], ARC_STEPS[s])
        fu = p[-1][0]
        pl.append(shape(rail, rail, "body", 5, rail_tone(rail), poly(offset_outline(p, R_TUBE)), region=rail_region(rail), material=rail_material(rail), solid=extruded(-R_TUBE, R_TUBE)))
        if s == 50:
            wx, _, wz = world(rail, [fu, 0.0, 0.0])
            pl.append(shape(f"foot-{rail[-1]}", rail, "detail", 4.9, 0.36, circle(wx, wz, R_FOOT), region=rail_region(rail)))
            if rail != "rail-d":
                pl.append(shape(f"rust-{rail[-1]}", rail, "texture", 5.1, 0.15, circle(wx, wz, R_TUBE + 0.006), region="rails"))
    pl.append(shape("column", "column", "body", 6, 0.42, poly(column_profile(s)), region="rails", material="steel-powder-red", solid=revolved("column", "u")))
    pl.append(shape("ring", "column", "body", 6.5, 0.42, path(annulus_path(R_RING - R_TUBE, R_RING + R_TUBE)), region="rails"))
    if s in (50, 100):
        eye = circle_path(EYE_CENTRE, 0.0, EYE_RING_R + EYE_SECTION_R, 32) + " " + circle_path(EYE_CENTRE, 0.0, EYE_RING_R - EYE_SECTION_R, 32)
        pl.append(shape("eye-ring", "eyebolt", "detail", 5.8, 0.3, path(eye), solid=extruded(-EYE_SECTION_R, EYE_SECTION_R)))
    return {"elevation": el, "front": fr, "plan": pl}


def hatches(s):
    """▤️ The hatches of one scale: chequer plate, concrete, worn-through gravel and moss."""
    out = []
    if s in (50, 100):
        out.append({"id": "chequer", "title": T("Teardrop chequer plate", "Tränenblech"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.6 if s == 50 else 0.5, "penMm": 0.13})
        out.append({"id": "gravel", "title": T("Worn-through base layer (gravel)", "Durchgetretene Tragschicht (Kies)"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.6 if s == 50 else 0.5, "penMm": 0.13})
        out.append({"id": "moss", "title": T("Moss and algae on the shaded edge", "Moos und Algen am schattigen Rand"), "pattern": "dots", "angleDeg": 30, "spacingMm": 0.45, "penMm": 0.13})
    if s != 500:
        out.append({"id": "concrete", "title": T("Concrete, cut below ground", "Beton, unter Gelände"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.8 if s == 50 else 0.6, "penMm": 0.13})
    return out


def point_in(poly_pts, a, r):
    """📌️ Even-odd point-in-polygon test in a profile plane."""
    inside = False
    n = len(poly_pts)
    for i in range(n):
        a0, r0 = poly_pts[i]
        a1, r1 = poly_pts[(i + 1) % n]
        if (r0 > r) != (r1 > r) and a < a0 + (r - r0) * (a1 - a0) / (r1 - r0):
            inside = not inside
    return inside


def revolve_props(profile, closed_loop, n=240):
    """⚖️ Volume and axial centre of a revolve (numerical, on a grid over the profile)."""
    ring = open_ring(profile) if closed_loop else profile
    a_lo, a_hi = min(p[0] for p in ring), max(p[0] for p in ring)
    r_lo, r_hi = 0.0, max(p[1] for p in ring)
    da, dr = (a_hi - a_lo) / n, (r_hi - r_lo) / n
    vol = 0.0
    mom = 0.0
    for i in range(n):
        a = a_lo + (i + 0.5) * da
        for j in range(n):
            r = r_lo + (j + 0.5) * dr
            if point_in(ring, a, r):
                dv = 2 * math.pi * r * da * dr
                vol += dv
                mom += dv * a
    return vol, (mom / vol if vol > 0 else 0.0)


A_RHS = 0.060 * 0.040 - 0.054 * 0.034
A_CHS_COL = math.pi / 4 * (0.1143 ** 2 - 0.1063 ** 2)
A_CHS_RAIL = math.pi / 4 * (0.0337 ** 2 - 0.0277 ** 2)
A_CHS_HUB = math.pi / 4 * (0.1683 ** 2 - 0.1523 ** 2)
A_ANGLE = (0.060 + 0.054) * 0.006
A_SPINDLE = math.pi * R_SPINDLE ** 2


def masses():
    """⚖️ Part masses (kg) and centres from section × density, gathered per bone in the bone's rest frame."""
    parts = {}
    v_apron, y_apron = revolve_props(apron_profile(50), True)
    parts["apron"] = ("footing", v_apron * RHO_SURFACING, (0.0, y_apron))
    floor_m = math.pi * R_TRAY_IN ** 2 * T_TRAY * RHO_STEEL
    wall_m = 2 * math.pi * (R_TRAY_IN + T_TRAY / 2) * (-T_COLLAR - Y_TRAY_BOT) * T_TRAY * RHO_STEEL
    collar_m = math.pi * (R_COLLAR ** 2 - R_TRAY_IN ** 2) * T_COLLAR * RHO_STEEL
    tray_m = floor_m + wall_m + collar_m
    tray_y = (floor_m * (Y_TRAY_BOT + T_TRAY / 2) + wall_m * (Y_TRAY_BOT - T_COLLAR) / 2 + collar_m * (-T_COLLAR / 2)) / tray_m
    parts["tray"] = ("footing", tray_m, (0.0, tray_y))
    v_f, y_f = revolve_props(FOUND_PROFILE, False)
    parts["foundation"] = ("footing", v_f * RHO_CONCRETE, (0.0, y_f))
    v_h, y_h = revolve_props(housing_profile(50), False)
    bore = math.pi * 0.045 ** 2 * (Y_HOUSING_TOP - Y_TRAY_TOP_FLOOR - T_HOUSING_FLANGE)
    parts["housing"] = ("footing", (v_h - bore) * RHO_CAST + 2.0, (0.0, y_h))
    parts["spindle"] = ("rotor", A_SPINDLE * (Y_SPINDLE_TOP - Y_SPINDLE_BOT) * RHO_STEEL, ((Y_SPINDLE_TOP + Y_SPINDLE_BOT) / 2 + 0.1, 0.0))
    hub_m = A_CHS_HUB * (Y_DECK_BOT - Y_HUB_BOT) * RHO_STEEL + 2 * math.pi * R_HUB ** 2 * 0.010 * RHO_STEEL
    parts["hub"] = ("platform", hub_m, (0.0, (Y_DECK_BOT + Y_HUB_BOT) / 2))
    parts["spokes"] = ("platform", 8 * A_RHS * (R_SPOKE_END - R_HUB) * RHO_STEEL, (0.0, 0.0))
    parts["rim"] = ("platform", A_ANGLE * 2 * math.pi * 0.882 * RHO_STEEL, (0.0, 0.012))
    parts["deck"] = ("platform", TREAD_KG_M2 * math.pi * R_PLAT ** 2, (0.0, (Y_DECK_BOT + Y_DECK_TOP) / 2 + 0.001))
    parts["column"] = ("column", A_CHS_COL * H_COL * RHO_STEEL + math.pi * R_COL ** 2 * 0.004 * 1.2 * RHO_STEEL, (H_COL / 2 + 0.006, 0.0))
    parts["ring"] = ("column", A_CHS_RAIL * 2 * math.pi * R_RING * RHO_STEEL, (RING_U, 0.0))
    for rail in RAIL_IDS:
        p = rail_path(rail, FOOT_V[50], 16)
        segs = list(zip(p, p[1:]))
        total = 0.0
        cu = cv = 0.0
        for a, b in segs:
            l = math.hypot(b[0] - a[0], b[1] - a[1])
            u0 = max(a[0], R_COL) if b[0] > a[0] else a[0]
            frac = 1.0 if a[0] >= R_COL else max(0.0, (b[0] - R_COL) / (b[0] - a[0]))
            l *= frac
            total += l
            cu += l * ((u0 + b[0]) / 2 if frac < 1 else (a[0] + b[0]) / 2)
            cv += l * (a[1] + b[1]) / 2
        tube = A_CHS_RAIL * total * RHO_STEEL
        foot = math.pi * R_FOOT ** 2 * T_FOOT * RHO_STEEL
        m = tube + foot
        parts[rail] = (rail, m, ((cu / total * tube + p[-1][0] * foot) / m, (cv / total * tube + (FOOT_V[50] - T_FOOT / 2) * foot) / m))
    parts["eyebolt"] = ("eyebolt", 0.45, (0.05, 0.0))
    bones = {}
    for name, (bone, m, (cu, cv)) in parts.items():
        tot, su, sv = bones.get(bone, (0.0, 0.0, 0.0))
        bones[bone] = (tot + m, su + m * cu, sv + m * cv)
    return parts, {b: (m, su / m, sv / m) for b, (m, su, sv) in bones.items()}


PARTS, BONE_MASS = masses()
TOTAL_KG = sum(m for m, _, _ in BONE_MASS.values())


SECTIONS = {
    "rotor": {"areaM2": A_SPINDLE, "secondMomentM4": math.pi * (2 * R_SPINDLE) ** 4 / 64, "extremeFibreM": R_SPINDLE, "material": "steel-c45", "title": T("Spindle o60 solid C45", "Achse o60 voll, C45")},
    "platform": {"areaM2": A_RHS, "secondMomentM4": (0.040 * 0.060 ** 3 - 0.034 * 0.054 ** 3) / 12, "extremeFibreM": 0.030, "material": "steel-galv-weathered", "title": T("Spoke RHS 60 x 40 x 3 (EN 10219-2), one of eight", "Speiche RHS 60 x 40 x 3 (EN 10219-2), eine von acht")},
    "column": {"areaM2": A_CHS_COL, "secondMomentM4": math.pi / 64 * (0.1143 ** 4 - 0.1063 ** 4), "extremeFibreM": R_COL, "material": "steel-powder-red", "title": T("Column CHS 114.3 x 4 (EN 10219-2)", "Säule CHS 114,3 x 4 (EN 10219-2)")},
    "rail": {"areaM2": A_CHS_RAIL, "secondMomentM4": math.pi / 64 * (0.0337 ** 4 - 0.0277 ** 4), "extremeFibreM": R_TUBE, "material": "steel-powder-red", "title": T("Handrail CHS 33.7 x 3 (EN 10219-2)", "Haltebügel CHS 33,7 x 3 (EN 10219-2)")},
    "eyebolt": {"areaM2": 2.45e-4, "secondMomentM4": math.pi * 0.01765 ** 4 / 64, "extremeFibreM": 0.01, "material": "steel-c45", "title": T("Eyebolt M20 shank, stress area 245 mm2 (DIN 580)", "Ringschraube M20, Spannungsquerschnitt 245 mm2 (DIN 580)")},
}

BONE_INFO = {
    "ground": (T("Reference point on the surfacing at the roundabout axis", "Bezugspunkt auf dem Belag in der Karussellachse"), 0.05, {"type": "fixed"}),
    "footing": (T("Footing left in the ground: poured EPDM apron, galvanised pit tray with collar, cast bearing housing, concrete point foundation", "Im Boden verbleibende Gründung: gegossene EPDM-Umrandung, verzinkte Einbauwanne mit Kragen, gegossenes Lagergehäuse, Beton-Punktfundament"), 0.05, {"type": "fixed"}),
    "lift": (T("Lift of the rotating assembly out of its bearing (crane travel)", "Hub der Drehgruppe aus ihrem Lager (Kranweg)"), 0.05, {"type": "prismatic", "axisDeg": 90, "minM": 0.0, "maxM": 0.8}),
    "rotor": (T("Spindle in the tapered-roller bearing: the vertical axis the platform turns about", "Achse im Kegelrollenlager: die senkrechte Drehachse der Plattform"), 0.13, {"type": "revolute", "continuousTwist": True}),
    "platform": (T("Platform: teardrop tread plate on eight spokes, rim angle and hub", "Plattform: Tränenblech auf acht Speichen, Randwinkel und Nabe"), R_PLAT, {"type": "fixed"}),
    "column": (T("Centre column with domed cap and the ring handrail", "Mittelsäule mit gewölbter Kappe und Haltering"), H_COL, {"type": "fixed"}),
    "rail-a": (T("Handrail ahead", "Haltebügel vorn"), 0.80, {"type": "fixed"}),
    "rail-b": (T("Handrail left", "Haltebügel links"), 0.80, {"type": "fixed"}),
    "rail-c": (T("Handrail behind, its elbow bent down 20 mm", "Haltebügel hinten, Bogen 20 mm heruntergebogen"), 0.80, {"type": "fixed"}),
    "rail-d": (T("Handrail right, replaced in bare galvanised tube", "Haltebügel rechts, in blankem verzinktem Rohr ersetzt"), 0.80, {"type": "fixed"}),
    "eyebolt": (T("DIN 580 M20 lifting eyebolt (stowed in the column cap in service)", "Ringschraube DIN 580 M20 (im Betrieb in der Säulenkappe verstaut)"), 0.11, {"type": "prismatic", "axisDeg": 0, "minM": 0.0, "maxM": EYE_TRAVEL}),
}


def rig_json():
    """🦴️ The rig of the asset with segment masses and member sections."""
    bones = []
    for bid, parent, x, y, z, ang, abd, tw in RIG:
        title, length, joint = BONE_INFO[bid]
        b = {"id": bid, "parent": parent, "title": title, "x": rnd(x), "y": rnd(y)}
        if z:
            b["z"] = rnd(z)
        b["angleDeg"] = rnd(ang)
        if abd:
            b["abductionDeg"] = rnd(abd)
        if tw:
            b["twistDeg"] = rnd(tw)
        b["lengthM"] = rnd(length)
        b["joint"] = joint
        if bid in BONE_MASS:
            m, cu, cv = BONE_MASS[bid]
            b["mass"] = {"kg": rnd(m, 3), "comX": rnd(cu, 4), "comY": rnd(cv, 4), "distribution": "point", "source": "authored-estimate", "reference": "en-10219-2-2019" if bid not in ("footing",) else "iso-10456-2007"}
        key = "rail" if bid.startswith("rail-") else bid
        if key in SECTIONS:
            sec = dict(SECTIONS[key])
            sec = {k: (rnd(v, 12) if isinstance(v, float) else v) for k, v in sec.items()}
            if bid == "rail-d":
                sec["material"] = "steel-galv-new"
            b["section"] = sec
        bones.append(b)
    return {"kind": "mechanical", "root": "ground", "bones": bones}


ANCHORS = [
    {"id": "footing-fixed", "bone": "footing", "x": 0.0, "y": Y_FOUND_BOT, "kind": "support", "title": T("Point foundation bedded in the soil (base and compacted fill around the pit)", "Punktfundament im Erdreich (Sohle und verdichtete Verfüllung um die Wanne)"), "support": "fixed", "footprintLengthM": 0.6, "footprintWidthM": 0.6, "frictionCoefficient": 0.6},
    {"id": "lift-eye", "bone": "eyebolt", "x": rnd(EYE_CENTRE + EYE_RING_R - EYE_SECTION_R), "y": 0.0, "kind": "load", "title": T("Crane hook in the eyebolt (only while the depot lifts the assembly)", "Kranhaken in der Ringschraube (nur während der Bauhof die Drehgruppe hebt)")},
    {"id": "push-point", "bone": "platform", "x": R_PLAT, "y": 0.02, "kind": "load", "title": T("Hands of the pushing adult on the rim", "Hände der anschiebenden erwachsenen Person am Rand"), "z": 0.0},
]

REGIONS = [
    ("deck", T("Teardrop tread plate deck and rim, weathered galvanised", "Tränenblech-Deck und Rand, verwittert verzinkt"), "surface", "steel-galv-weathered", 2.9, 1.8, "horizontal-plate-up", 0.95, ("metal", "steel-galv-weathered", 0.005, T("Tread plate 5/7 mm", "Tränenblech 5/7 mm")), None),
    ("rails", T("Column, ring and three handrails, chalked red powder coat", "Säule, Ring und drei Haltebügel, ausgekreidete rote Pulverbeschichtung"), "surface", "steel-powder-red", 0.95, 0.034, "cylinder", 0.6, ("metal", "steel-powder-red", 0.003, T("Tube wall 3 mm with powder coat", "Rohrwand 3 mm mit Pulverlack")), None),
    ("rail-new", T("Replaced handrail, bare new galvanised tube", "Ersetzter Haltebügel, blankes neues verzinktes Rohr"), "surface", "steel-galv-new", 0.14, 0.034, "cylinder", 0.6, ("metal", "steel-galv-new", 0.003, T("Tube wall 3 mm", "Rohrwand 3 mm")), None),
    ("underframe", T("Spokes, hub, spindle and bearing housing in the pit", "Speichen, Nabe, Achse und Lagergehäuse in der Wanne"), "surface", "steel-galv-weathered", 1.9, 0.06, "horizontal-plate-down", 0.0, ("metal", "steel-galv-weathered", 0.003, T("Hollow-section wall 3 mm", "Hohlprofilwand 3 mm")), T("Under the platform inside the closed pit; seen only when the depot lifts the assembly.", "Unter der Plattform in der geschlossenen Wanne; nur sichtbar, wenn der Bauhof die Drehgruppe hebt.")),
    ("pit-tray", T("Galvanised pit tray and its collar flush with the surfacing", "Verzinkte Einbauwanne und ihr Kragen bündig mit dem Belag"), "surface", "steel-galv-weathered", 5.2, 0.07, "horizontal-plate-up", 0.3, ("metal", "steel-galv-weathered", 0.003, T("Sheet 3 mm", "Blech 3 mm")), None),
    ("surfacing", T("Poured EPDM apron with the worn hollow", "Gegossene EPDM-Umrandung mit ausgetretener Mulde"), "surface", "epdm-surfacing", 6.9, 0.42, "horizontal-plate-up", 0.95, ("polymer", "epdm-surfacing", T_APRON, T("EPDM granulate on SBR base 40 mm", "EPDM-Granulat auf SBR-Tragschicht 40 mm")), None),
    ("foundation", T("Concrete point foundation in the soil", "Beton-Punktfundament im Erdreich"), "surface", "concrete", 1.7, 0.6, "cylinder", 0.0, ("masonry", "concrete", 0.10, T("Concrete skin 100 mm (active depth)", "Betonhaut 100 mm (aktive Tiefe)")), None),
]
CONDUCTIVITY = {m["id"]: m["conductivityWmK"] for m in MATERIALS}
CAPACITY = {m["id"]: m["densityKgM3"] * m["specificHeatJKgK"] for m in MATERIALS}


def regions_json(areas):
    """🔥️ The thermal regions with their single layer, heat capacity and outer area."""
    out = []
    for rid, title, kind, mat, area, lc, geom, sky, (lkind, lmat, th, ltitle), hidden in REGIONS:
        hc = rnd(CAPACITY[lmat] * th, 1)
        r = {"id": rid, "title": title, "kind": kind, "model": "passive", "material": mat, "areaM2": rnd(areas.get(rid, area), 3), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": sky,
             "layers": [{"id": f"{rid}-layer", "title": ltitle, "kind": lkind, "material": lmat, "thicknessM": th, "resistanceM2KW": rnd(th / CONDUCTIVITY[lmat], 8), "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "iso-10456-2007"}],
             "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "iso-10456-2007" if mat != "epdm-surfacing" else "din-en-1177-2018"}
        if hidden is not None:
            r["hidden"] = hidden
        out.append(r)
    return out


def binding(kind):
    """🌬️ The region states of a pose: summer sun on the deck, the buried parts in contact with the soil."""
    deck = {"exposedFraction": 1, "sunlitFraction": 0.92}
    rails = {"exposedFraction": 1, "sunlitFraction": 0.5}
    under = {"exposedFraction": 1, "sunlitFraction": 0}
    if kind == "use":
        deck = {"exposedFraction": 0.93, "sunlitFraction": 0.8, "contact": {"fraction": 0.05, "tempC": 33}}
        rails = {"exposedFraction": 0.97, "sunlitFraction": 0.45, "contact": {"fraction": 0.03, "tempC": 34}}
    if kind == "lifted":
        deck = {"exposedFraction": 1, "sunlitFraction": 0.85}
        under = {"exposedFraction": 1, "sunlitFraction": 0.15}
    return {"regions": {
        "deck": deck, "rails": rails, "rail-new": dict(rails), "underframe": under,
        "pit-tray": {"exposedFraction": 0.12, "sunlitFraction": 0.08 if kind != "lifted" else 0.2, "contact": {"fraction": 0.85, "tempC": 22}},
        "surfacing": {"exposedFraction": 0.52, "sunlitFraction": 0.48, "contact": {"fraction": 0.46, "tempC": 24}},
        "foundation": {"exposedFraction": 0, "sunlitFraction": 0, "contact": {"fraction": 1.0, "tempC": 16}},
    }, "emitters": {}}


def children():
    """🧒️ Four children of 30 kg riding at 0.70 m between the handrails (platform frame, carried with the rotor)."""
    out = []
    for k, a in enumerate([45, 135, 225, 315]):
        out.append({"id": f"child-{k + 1}", "kind": "occupant", "title": T(f"Child {k + 1} riding (30 kg)", f"Kind {k + 1} fährt mit (30 kg)"), "bone": "platform", "x": rnd(0.7 * math.cos(math.radians(a))), "y": Y_DECK_TOP, "z": rnd(0.7 * math.sin(math.radians(a))), "fxN": 0, "fyN": rnd(-30 * G, 3), "source": "authored-estimate", "reference": "din-en-1176-1-2017"})
    return out


USE_TWIST = 30.0
HOOK_N = rnd((TOTAL_KG - BONE_MASS["footing"][0]) * G, 2)


def poses():
    """📥️ In service (default), in use, maintenance and the key poses of the two clips."""
    ca, sa = math.cos(math.radians(USE_TWIST)), math.sin(math.radians(USE_TWIST))
    push = {"id": "adult-push", "kind": "live", "title": T("Adult pushing the rim tangentially (150 N)", "Erwachsene Person schiebt tangential am Rand (150 N)"), "bone": "platform", "x": R_PLAT, "y": 0.02, "z": 0.0, "fxN": rnd(-150 * sa, 2), "fyN": 0, "fzN": rnd(150 * ca, 2), "source": "authored-estimate", "reference": "din-en-1176-5-2020"}
    bearing = {"id": "crane-hook", "kind": "equipment", "title": T("Crane hook lifting the rotating assembly (its weight)", "Kranhaken hebt die Drehgruppe (ihr Gewicht)"), "anchor": "lift-eye", "fxN": 0, "fyN": HOOK_N, "source": "authored-estimate", "reference": "din-580-2010"}
    eye_in = {"eyebolt": {"dx": EYE_TRAVEL}}
    return [
        {"id": "in-service", "title": T("In service", "In Betrieb"), "description": T("Standing empty in the summer sun as the park keeps it: platform 35 mm above the surfacing, eyebolt stowed in the column cap.", "Leer in der Sommersonne, wie der Park es betreibt: Plattform 35 mm über dem Belag, Ringschraube in der Säulenkappe verstaut."), "bones": {}, "ground": "authored", "contacts": ["footing-fixed"], "thermal": binding("service"), "loads": []},
        {"id": "in-use", "title": T("In use", "In Benutzung"), "description": T("Four children of 30 kg ride between the handrails while an adult pushes the rim with 150 N; the platform has turned 30 degrees.", "Vier Kinder zu 30 kg fahren zwischen den Haltebügeln, eine erwachsene Person schiebt mit 150 N am Rand; die Plattform hat sich um 30 Grad gedreht."), "bones": {"rotor": {"twistDeg": USE_TWIST}}, "ground": "authored", "contacts": ["footing-fixed"], "thermal": binding("use"), "loads": children() + [push]},
        {"id": "maintenance", "title": T("Lifted out for maintenance", "Zur Wartung ausgehoben"), "description": T("The depot crane hangs the 203 kg rotating assembly 0.60 m above its pit on the DIN 580 eyebolt to regrease the dry bearing; the footing stays in the ground.", "Der Kran des Bauhofs hält die Drehgruppe von 203 kg an der Ringschraube DIN 580 0,60 m über ihrer Wanne, um das trockene Lager nachzuschmieren; die Gründung bleibt im Boden."), "bones": {"lift": {"dy": LIFT_TRAVEL}, **eye_in}, "ground": "authored", "contacts": ["footing-fixed"], "thermal": binding("lifted"), "loads": [bearing]},
        {"id": "eyebolt-fitted", "title": T("Eyebolt fitted (key)", "Ringschraube eingesetzt (Schlüssel)"), "description": T("The eyebolt is screwed into the column cap; the assembly still turns in its bearing.", "Die Ringschraube ist in die Säulenkappe geschraubt; die Drehgruppe liegt noch im Lager."), "key": True, "bones": dict(eye_in), "ground": "authored", "contacts": ["footing-fixed"], "thermal": binding("service"), "loads": []},
        {"id": "slings-taut", "title": T("Crane takes the load (key)", "Kran übernimmt die Last (Schlüssel)"), "description": T("The hook carries the assembly before it leaves the bearing; the soil carries the footing.", "Der Haken trägt die Drehgruppe, bevor sie das Lager verlässt; der Boden trägt die Gründung."), "key": True, "bones": dict(eye_in), "ground": "authored", "contacts": ["footing-fixed"], "thermal": binding("service"), "loads": [bearing]},
        {"id": "riding", "title": T("Children riding (key)", "Kinder fahren mit (Schlüssel)"), "description": T("Four children of 30 kg ride without being pushed.", "Vier Kinder zu 30 kg fahren ohne Anschub mit."), "key": True, "bones": {}, "ground": "authored", "contacts": ["footing-fixed"], "thermal": binding("use"), "loads": children()},
    ]


SPIN_S = 2.4


def clips():
    """🎞️ The depot lifting the assembly out (maintenance) and one turn of the children's ride (spinning, looping)."""
    spin_keys = [{"t": rnd(SPIN_S * k / 4, 4), "pose": "riding", "ease": "linear", "bones": {"rotor": {"twistDeg": 90.0 * k}}} for k in range(5)]
    return [
        {"id": "maintenance", "title": T("Lifting out for maintenance", "Ausheben zur Wartung"), "description": T("The worker screws the eyebolt into the column cap, the crane takes the load and lifts the rotating assembly 0.60 m out of the pit in 7 s.", "Der Mitarbeiter schraubt die Ringschraube in die Säulenkappe, der Kran übernimmt die Last und hebt die Drehgruppe in 7 s 0,60 m aus der Wanne."), "durationS": 10, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 2.5, "pose": "eyebolt-fitted", "ease": "smooth"}, {"t": 3.0, "pose": "slings-taut", "ease": "smooth"}, {"t": 10, "pose": "maintenance", "ease": "smooth"}],
         "contacts": [{"anchor": "footing-fixed", "fromS": 0, "toS": 10}],
         "phases": [{"id": "fit-eyebolt", "fromS": 0, "toS": 2.5, "title": T("Eyebolt screwed in", "Ringschraube eingeschraubt")}, {"id": "take-load", "fromS": 2.5, "toS": 3.0, "title": T("Crane takes the load", "Kran übernimmt die Last")}, {"id": "lift", "fromS": 3.0, "toS": 10, "title": T("Assembly lifted out of the bearing", "Drehgruppe aus dem Lager gehoben")}],
         "events": [{"t": 3.0, "kind": "load-on", "title": T("The hook carries the assembly, the soil the footing", "Der Haken trägt die Drehgruppe, der Boden die Gründung"), "target": "crane-hook"}, {"t": 3.2, "kind": "motion", "title": T("The spindle leaves the dry bearing", "Die Achse verlässt das trockene Lager")}],
         "tracks": []},
        {"id": "spinning", "title": T("Spinning with children", "Drehen mit Kindern"), "description": T("Four children ride one full turn in 2.4 s (peripheral speed 2.36 m/s at the rim, below the 5 m/s of DIN EN 1176-5); the dry bearing grinds once a turn.", "Vier Kinder fahren eine volle Umdrehung in 2,4 s (Umfangsgeschwindigkeit 2,36 m/s am Rand, unter den 5 m/s der DIN EN 1176-5); das trockene Lager knirscht einmal je Umdrehung."), "durationS": SPIN_S, "fps": 25, "loop": True, "rootMotion": "none",
         "keys": spin_keys, "contacts": [{"anchor": "footing-fixed", "fromS": 0, "toS": SPIN_S}],
         "phases": [{"id": "turn", "fromS": 0, "toS": SPIN_S, "title": T("One turn", "Eine Umdrehung")}],
         "events": [{"t": 0.9, "kind": "motion", "title": T("Bearing grinds where the grease has gone", "Das Lager knirscht, wo das Fett fehlt")}],
         "tracks": []},
    ]


def mm(s, metres):
    """📏️ Paper millimetres of a real length at 1:s, as text."""
    return f"{metres * 1000 / s:.1f}"


def rep_decisions(s):
    """📝️ The eight authored drawing decisions of one scale."""
    w, h = mm(s, 2 * R_APRON), mm(s, Y_CAP_TOP - Y_FOUND_BOT)
    d = {
        50: dict(
            silhouette=T(f"Side and front elevation {w} x {h} mm: the 1.80 m platform 35 mm above the poured apron, the 114.3 mm column with its domed cap, the ring handrail, four bent handrails on foot plates (the one behind sags 20 mm at its elbow), below ground the pit tray with collar and the 0.60 m point foundation; plan: apron, collar, deck with its chequer, column, ring and the four rails as a cross.", f"Seiten- und Vorderansicht {w} x {h} mm: die 1,80-m-Plattform 35 mm über der gegossenen Umrandung, die 114,3-mm-Säule mit gewölbter Kappe, der Haltering, vier gebogene Haltebügel auf Fußplatten (der hintere hängt am Bogen 20 mm durch), unter Gelände die Einbauwanne mit Kragen und das 0,60-m-Punktfundament; Draufsicht: Umrandung, Kragen, Deck mit Tränenmuster, Säule, Ring und die vier Bügel als Kreuz."),
            retained=T("Every part the 7.5 mm feature can show: the 5 mm tread plate over the 60 mm rim band, the hub and the spindle under it, the 80 mm foot plates, the eyebolt in the cap, the 8 mm collar and the 3 mm tray drawn as its outline.", "Jedes Teil, das das Merkmal 7,5 mm zeigen kann: das 5-mm-Tränenblech über dem 60-mm-Randband, Nabe und Achse darunter, die 80-mm-Fußplatten, die Ringschraube in der Kappe, der 8-mm-Kragen und die 3-mm-Wanne als ihr Umriss."),
            merged=T("The eight spokes lie behind the rim band in both elevations and are not drawn separately; the far rails stand behind the column.", "Die acht Speichen liegen in beiden Ansichten hinter dem Randband und werden nicht eigens gezeichnet; die hinteren Bügel stehen hinter der Säule."),
            omitted=T("Bolts of the bearing housing, the grease nipple, weld seams, the drainage gravel in the pit.", "Schrauben des Lagergehäuses, der Schmiernippel, Schweißnähte, der Drainagekies in der Wanne."),
            thinElements=T("The 33.7 mm handrails and the ring are filled strips at true width (0.67 mm); the 6 mm running gap is drawn as the tray floor showing between deck and collar in plan.", "Die 33,7-mm-Haltebügel und der Ring sind gefüllte Streifen in wahrer Breite (0,67 mm); der 6-mm-Laufspalt zeigt sich in der Draufsicht als Wannenboden zwischen Deck und Kragen."),
            hatching=T("Teardrop chequer as a 0.6 mm cross hatch on the deck; the worn ring of the deck left bare; stipple for the concrete below ground; dots for the gravel showing through the worn hollow of the apron and for the moss on its shaded edge.", "Tränenmuster als 0,6-mm-Kreuzschraffur auf dem Deck; der abgelaufene Ring des Decks bleibt frei; Punktierung für den Beton unter Gelände; Punkte für den durchscheinenden Kies in der ausgetretenen Mulde der Umrandung und für das Moos an ihrem schattigen Rand."),
            styles=T("Body pen 0.5, detail 0.25 (foot plates, rust collars, eyebolt, tray seam), texture 0.13 (rubbed paint, sticker, tag, gum spots, worn ring).", "Körperstift 0,5, Detail 0,25 (Fußplatten, Rostkragen, Ringschraube, Wannennaht), Textur 0,13 (abgeriebener Lack, Aufkleber, Tag, Kaugummiflecken, abgelaufener Ring)."),
            overlays=T("Deck and rim = deck; column, ring and three rails = rails; the replaced rail = rail-new; hub and spindle = underframe; tray and collar = pit-tray; apron = surfacing; point foundation = foundation.", "Deck und Rand = deck; Säule, Ring und drei Bügel = rails; der ersetzte Bügel = rail-new; Nabe und Achse = underframe; Wanne und Kragen = pit-tray; Umrandung = surfacing; Punktfundament = foundation.")),
        100: dict(
            silhouette=T(f"Side and front elevation {w} x {h} mm: platform, column with chamfered cap, ring, four handrails standing on the deck, tray and point foundation; plan with apron, collar, deck, column, ring and rails.", f"Seiten- und Vorderansicht {w} x {h} mm: Plattform, Säule mit gefaster Kappe, Ring, vier Haltebügel direkt auf dem Deck, Wanne und Punktfundament; Draufsicht mit Umrandung, Kragen, Deck, Säule, Ring und Bügeln."),
            retained=T("Parts of 15 mm and more: the deck with its rim as one 65 mm band, hub and spindle, rails and ring at true width, the eyebolt (16 mm ring), the rust at the rail feet and at the column foot.", "Teile ab 15 mm: Deck und Rand als ein 65-mm-Band, Nabe und Achse, Bügel und Ring in wahrer Breite, die Ringschraube (16-mm-Ring), der Rost an den Bügelfüßen und am Säulenfuß."),
            merged=T("Tread plate and rim angle are one platform band; foot plates merged into the rail feet.", "Tränenblech und Randwinkel sind ein Plattformband; Fußplatten in die Bügelfüße zusammengezogen."),
            omitted=T("Sticker, tag, rubbed paint and gum spots (below 15 mm), the domed cap profile.", "Aufkleber, Tag, abgeriebener Lack und Kaugummiflecken (unter 15 mm), das Kappenprofil."),
            thinElements=T("Rails and ring 0.34 mm strips at true width; the 8 mm collar drawn with the tray outline.", "Bügel und Ring als 0,34-mm-Streifen in wahrer Breite; der 8-mm-Kragen mit dem Wannenumriss gezeichnet."),
            hatching=T("Chequer cross hatch 0.5 mm (symbolic pitch), dots for the worn hollow and the moss, stipple for the concrete.", "Tränenmuster als Kreuzschraffur 0,5 mm (symbolische Teilung), Punkte für Mulde und Moos, Punktierung für den Beton."),
            styles=T("Body 0.35, detail 0.18, texture 0.13.", "Körper 0,35, Detail 0,18, Textur 0,13."),
            overlays=T("As at 1:50: deck, rails, rail-new, underframe, pit-tray, surfacing, foundation.", "Wie bei 1:50: deck, rails, rail-new, underframe, pit-tray, surfacing, foundation.")),
        200: dict(
            silhouette=T(f"Side and front elevation {w} x {h} mm: the platform as one disc with its hub, the flat-topped column, ring and four rails, the tray and the foundation; plan: apron, collar, deck, worn ring, column, ring and rails.", f"Seiten- und Vorderansicht {w} x {h} mm: die Plattform als eine Scheibe mit Nabe, die flach abgeschlossene Säule, Ring und vier Bügel, Wanne und Fundament; Draufsicht: Umrandung, Kragen, Deck, abgelaufener Ring, Säule, Ring und Bügel."),
            retained=T("Parts of 30 mm and more: platform disc 0.13 m deep with hub, 60 mm spindle, 114 mm column, 33.7 mm rails and ring, the 30 mm collar band.", "Teile ab 30 mm: Plattformscheibe 0,13 m tief mit Nabe, 60-mm-Achse, 114-mm-Säule, 33,7-mm-Bügel und Ring, das 30-mm-Kragenband."),
            merged=T("Deck, rim, spokes and hub are one disc; the collar is widened to the 30 mm feature; the bearing flange merged into the housing.", "Deck, Rand, Speichen und Nabe sind eine Scheibe; der Kragen ist auf das Merkmal 30 mm verbreitert; der Lagerflansch ist ins Gehäuse gezogen."),
            omitted=T("Eyebolt, rust collars, chequer, moss and the dip of the worn hollow (15 mm deep); the hollow stays as a detail ring in plan.", "Ringschraube, Rostkragen, Tränenmuster, Moos und die Tiefe der Mulde (15 mm); die Mulde bleibt als Detailring in der Draufsicht."),
            thinElements=T("Rails and ring 0.17 mm strips at true width.", "Bügel und Ring als 0,17-mm-Streifen in wahrer Breite."),
            hatching=T("Stipple for the concrete only.", "Nur Punktierung für den Beton."),
            styles=T("Body 0.25, detail 0.13.", "Körper 0,25, Detail 0,13."),
            overlays=T("Platform disc = deck, spindle = underframe, the rest as at 1:50.", "Plattformscheibe = deck, Achse = underframe, sonst wie bei 1:50.")),
        500: dict(
            silhouette=T(f"Side and front elevation {w} x {h} mm: platform disc on its boss, column, ring and rails, apron, tray and foundation; plan: apron with collar, tray floor, deck, column, ring and rails.", f"Seiten- und Vorderansicht {w} x {h} mm: Plattformscheibe auf ihrem Nabenblock, Säule, Ring und Bügel, Umrandung, Wanne und Fundament; Draufsicht: Umrandung mit Kragen, Wannenboden, Deck, Säule, Ring und Bügel."),
            retained=T("The 75 mm feature: platform disc 75 mm, column 114 mm, apron drawn 75 mm deep, rails and ring at their true 33.7 mm under the 0.18 mm body pen.", "Das Merkmal 75 mm: Plattformscheibe 75 mm, Säule 114 mm, Umrandung 75 mm tief gezeichnet, Bügel und Ring in wahren 33,7 mm unter dem 0,18-mm-Körperstift."),
            merged=T("Hub and spindle are one boss; apron and collar are one ring in plan; the apron and collar are drawn 75 mm deep (true 40 and 8 mm); the 75 mm platform disc is drawn whole over the apron so it still prints at 1:350.", "Nabe und Achse sind ein Block; Umrandung und Kragen sind in der Draufsicht ein Ring; Umrandung und Kragen 75 mm tief gezeichnet (wahr 40 und 8 mm); die 75-mm-Plattformscheibe ist ganz über der Umrandung gezeichnet, damit sie bei 1:350 noch druckt."),
            omitted=T("All traces of use (below the feature), the eyebolt, the worn ring and hollow.", "Alle Gebrauchsspuren (unter dem Merkmal), die Ringschraube, abgelaufener Ring und Mulde."),
            thinElements=T("Rails and ring are 0.07 mm strips carried by the body pen.", "Bügel und Ring sind 0,07-mm-Streifen, getragen vom Körperstift."),
            hatching=T("None.", "Keine."),
            styles=T("Body pen 0.18 only.", "Nur Körperstift 0,18."),
            overlays=T("Apron ring = surfacing and pit-tray (merged), the rest as at 1:200.", "Umrandungsring = surfacing und pit-tray (zusammengefasst), sonst wie bei 1:200.")),
    }[s]
    return d


def volume_decisions(s):
    """🧾️ What one scale's 3D model keeps, merges and why."""
    body = {
        50: T("Body: the real roundabout part by part. Footing: poured EPDM apron 40 mm on its ring 0.98 to 1.40 m with the hollow worn 15 mm deep between 1.02 and 1.32 m, the 3 mm galvanised pit tray (open revolve, true wall) with its 8 mm collar flush with the surfacing, the cast bearing housing with flange and the 0.60 x 0.60 m point foundation. Rotating assembly: 60 mm spindle, 168 mm hub, eight 60 x 40 RHS spokes (one box repeated eight times about the axis), the L 60 x 60 x 6 rim angle as its 60 x 60 mm envelope ring (the open corner of the angle is inside the pit and closed in the model), the 5 mm tread plate as a declared sheet, the 114.3 mm column with domed cap, the ring handrail as a slender torus and four handrails as lofts along their bent centre lines on 80 mm foot plates, the one behind sagging 20 mm at the elbow; the DIN 580 eyebolt stowed inside the column.", "Körper: das echte Karussell Teil für Teil. Gründung: gegossene EPDM-Umrandung 40 mm auf dem Ring 0,98 bis 1,40 m mit der zwischen 1,02 und 1,32 m 15 mm tief ausgetretenen Mulde, die 3-mm-Einbauwanne (offene Rotation, wahre Wand) mit 8-mm-Kragen bündig mit dem Belag, das gegossene Lagergehäuse mit Flansch und das Punktfundament 0,60 x 0,60 m. Drehgruppe: 60-mm-Achse, 168-mm-Nabe, acht RHS-Speichen 60 x 40 (ein Quader achtmal um die Achse wiederholt), der Randwinkel L 60 x 60 x 6 als sein 60 x 60 mm großer Hüllring (die offene Ecke des Winkels liegt in der Wanne und ist im Modell geschlossen), das 5-mm-Tränenblech als erklärtes Blech, die 114,3-mm-Säule mit gewölbter Kappe, der Haltering als schlanker Torus und vier Haltebügel als Lofts entlang ihrer gebogenen Mittellinien auf 80-mm-Fußplatten, der hintere am Bogen 20 mm durchhängend; die Ringschraube DIN 580 in der Säule verstaut."),
        100: T("Body for the 15 mm feature: apron with a single-step hollow, tray and foundation as at 1:50, housing with flange; deck and rim merged into one platform shell (65 mm), hub, spindle and the eight spokes kept; column with a chamfered cap; rails without foot plates standing on the deck; ring; eyebolt (16 mm ring, 20 mm shank).", "Körper für das Merkmal 15 mm: Umrandung mit einstufiger Mulde, Wanne und Fundament wie bei 1:50, Gehäuse mit Flansch; Deck und Rand zu einer Plattformschale (65 mm) zusammengefasst, Nabe, Achse und die acht Speichen behalten; Säule mit gefaster Kappe; Bügel ohne Fußplatten direkt auf dem Deck; Ring; Ringschraube (16-mm-Ring, 20-mm-Schaft)."),
        200: T("Body for the 30 mm feature: the apron without the 15 mm hollow, the collar widened to 30 mm, the housing without its flange; deck, rim, spokes and hub merged into one platform disc with its hub boss, the spindle kept (60 mm); column flat-topped; rails and ring at their true 33.7 mm; the eyebolt left out (16 mm).", "Körper für das Merkmal 30 mm: die Umrandung ohne die 15-mm-Mulde, der Kragen auf 30 mm verbreitert, das Gehäuse ohne Flansch; Deck, Rand, Speichen und Nabe zu einer Plattformscheibe mit Nabenblock zusammengefasst, die Achse behalten (60 mm); Säule flach abgeschlossen; Bügel und Ring in wahren 33,7 mm; die Ringschraube entfällt (16 mm)."),
        500: T("Body for the 75 mm feature: apron and collar drawn 75 mm deep, tray and foundation, housing; hub and spindle merged into one 168 mm boss on the rotor, the platform a 75 mm disc; column flat-topped; rails and ring kept at their true 33.7 mm as declared slender parts.", "Körper für das Merkmal 75 mm: Umrandung und Kragen 75 mm tief, Wanne und Fundament, Gehäuse; Nabe und Achse zu einem 168-mm-Block am Rotor zusammengefasst, die Plattform eine 75-mm-Scheibe; Säule flach abgeschlossen; Bügel und Ring in wahren 33,7 mm als erklärte schlanke Teile."),
    }[s]
    thermal = T("Thermal: seven single-layer regions, every solid a reference to its body solid except the seal band, a slender hoop that closes the 6 mm running gap between rim and tray in the thermal model, so the pit under the platform is the closed cavity it nearly is and the spokes, hub, spindle and housing are the hidden region underframe; the eyebolt has no region (inside the column).", "Thermik: sieben einschichtige Regionen, jeder Körper ein Verweis auf seinen Körperteil außer dem Dichtband, einem schlanken Reif, der im thermischen Modell den 6-mm-Laufspalt zwischen Rand und Wanne schließt, so dass die Wanne unter der Plattform der fast geschlossene Hohlraum ist und Speichen, Nabe, Achse und Gehäuse die verdeckte Region underframe bilden; die Ringschraube hat keine Region (in der Säule).")
    structure = T("Structure: the load path from the handrails and the deck through the spokes, the hub and the spindle into the bearing and down the housing and the point foundation to the fixed support at its base; members on their bones at their true sections (spindle o60, RHS 60 x 40 spokes, CHS 114.3 column, CHS 33.7 rails, the M20 eyebolt shank" + (")" if s in (50, 100) else ", taken in by the column member at this scale)") + ", joints at the bearing, hub, column foot and rail roots, the support at the base of the point foundation and mass markers at the centres of mass.", "Tragwerk: der Lastpfad von den Haltebügeln und dem Deck über Speichen, Nabe und Achse ins Lager und über Gehäuse und Punktfundament zum festen Auflager an dessen Sohle; Stäbe auf ihren Knochen in wahren Querschnitten (Achse o60, Speichen RHS 60 x 40, Säule CHS 114,3, Bügel CHS 33,7, Schaft der Ringschraube M20" + (")" if s in (50, 100) else ", bei diesem Maßstab vom Säulenstab mitgenommen)") + ", Gelenke an Lager, Nabe, Säulenfuß und Bügelwurzeln, das Auflager an der Sohle des Punktfundaments und Massenmarken in den Schwerpunkten.")
    return [body, thermal, structure]


SUMMARY = T(
    "A flush-mounted municipal platform roundabout of hot-dip galvanised steel after some fifteen years in a German playground: a 1.80 m teardrop tread-plate platform on eight spokes turns on a tapered-roller bearing in a galvanised pit tray set on a concrete point foundation, 35 mm above the poured EPDM surfacing; a red powder-coated centre column carries a ring handrail and four bent handrails. Its use shows: the chequer is worn smooth in a ring where children run, the apron is worn into a 15 mm hollow, one rail is bent 20 mm and another replaced in bare zinc, paint chalks and rusts at the welds, and the dry bearing grinds.",
    "Ein bodenbündiges kommunales Plattformkarussell aus feuerverzinktem Stahl nach rund fünfzehn Jahren auf einem deutschen Spielplatz: eine 1,80-m-Plattform aus Tränenblech auf acht Speichen dreht sich auf einem Kegelrollenlager in einer verzinkten Einbauwanne über einem Beton-Punktfundament, 35 mm über dem gegossenen EPDM-Belag; eine rot pulverbeschichtete Mittelsäule trägt einen Haltering und vier gebogene Haltebügel. Der Gebrauch ist sichtbar: das Tränenmuster ist in einem Ring, wo Kinder laufen, glattgetreten, die Umrandung zu einer 15-mm-Mulde ausgetreten, ein Bügel um 20 mm verbogen und ein anderer in blankem Zink ersetzt, der Lack kreidet und rostet an den Schweißnähten, und das trockene Lager knirscht.")

REFERENCES = [
    {"id": "din-en-1176-1-2017", "citation": "DIN EN 1176-1:2017-12 Playground equipment and surfacing - Part 1: General safety requirements and test methods (finger entrapment between 8 and 25 mm, user loads, impact area)."},
    {"id": "din-en-1176-5-2020", "citation": "DIN EN 1176-5:2020-03 Playground equipment and surfacing - Part 5: Additional specific safety requirements and test methods for carousels (maximum peripheral speed 5 m/s, clearances under and around the rotating platform)."},
    {"id": "din-en-1176-7-2020", "citation": "DIN EN 1176-7:2020-07 Playground equipment and surfacing - Part 7: Guidance on installation, inspection, maintenance and operation (operational and annual main inspection, bearing maintenance)."},
    {"id": "din-en-1177-2018", "citation": "DIN EN 1177:2018-03 Impact attenuating playground surfacing - Methods of test for determination of impact attenuation (poured EPDM on SBR base, layer thickness against critical fall height)."},
    {"id": "din-18034-1-2020", "citation": "DIN 18034-1:2020-10 Playgrounds and outdoor play areas - Part 1: Requirements for planning, construction and operation."},
    {"id": "din-59220-2000", "citation": "DIN 59220:2000-04 Flat products of steel - Hot rolled tear plates - Dimensions, mass and tolerances (5/7 mm tear plate about 42.4 kg/m2)."},
    {"id": "en-10219-2-2019", "citation": "EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (CHS 114.3 x 4, 33.7 x 3, 168.3 x 8; RHS 60 x 40 x 3)."},
    {"id": "en-10056-1-2017", "citation": "EN 10056-1:2017 Structural steel equal and unequal leg angles - Part 1: Dimensions (L 60 x 60 x 6, 5.42 kg/m)."},
    {"id": "din-580-2010", "citation": "DIN 580:2010-08 Lifting eye bolts (M20, working load limit 1.2 t in the axial direction)."},
    {"id": "en-iso-1461-2022", "citation": "EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles - Specifications and test methods."},
    {"id": "en-1992-1-1-2004", "citation": "EN 1992-1-1:2004 Eurocode 2: Design of concrete structures, Part 1-1 (C25/30, Ecm = 31 GPa, Table 3.1)."},
    {"id": "iso-10456-2007", "citation": "EN ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (concrete, steel)."},
    {"id": "duffie-beckman-2013", "citation": "Duffie, J. A., Beckman, W. A. (2013). Solar Engineering of Thermal Processes, 4th ed., Wiley (solar absorptance and emittance of galvanised steel, new 0.65/0.13 and weathered 0.89/0.28)."},
    {"id": "levinson-2007", "citation": "Levinson, R., Berdahl, P., Akbari, H., et al. (2007). Methods of creating solar-reflective nonwhite surfaces and their application to residential roofing materials. Solar Energy Materials and Solar Cells 91, 304-314 (solar absorptance of red pigmented coatings, chalking)."},
    {"id": "incropera-2018", "citation": "Bergman, T. L., Lavine, A. S., Incropera, F. P., DeWitt, D. P. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (properties of steel and cast iron)."},
]

NOTES = T(
    "Catalogue-typical flush-mounted platform roundabout of municipal playground ranges, no brand: platform o1.80 m of 5/7 mm teardrop plate on a rim angle L 60 x 60 x 6 and eight RHS 60 x 40 x 3 spokes welded to a o168.3 x 8 hub, a o60 C45 spindle in a cast housing with two tapered-roller bearings on the floor of a 3 mm galvanised pit tray (inner o1.812 m, 0.30 m deep, 6 mm running gap below the 8 mm finger-entrapment limit of DIN EN 1176-1), tray on a 0.60 x 0.60 m C25/30 point foundation down to frost depth 0.90 m, poured EPDM apron 40 mm to o2.80 m. Centre column CHS 114.3 x 4 to 0.75 m, ring handrail and four handrails CHS 33.7 x 3 with their tops at 0.637 m. All dimensions and masses are authored estimates of the type (section x density, tear plate per DIN 59220), the traces of use are illustrative: rail-c bent 20 mm at the elbow by a mower or vandal, rail-d replaced in bare galvanised tube, chequer worn smooth in the running ring 0.56 to 0.85 m, the apron worn 15 mm deep by pushing feet, moss on the shaded edge, chalked and chipped paint with rust collars at the welds, gum on the deck, a sticker and a tag on the column, a dry bearing that grinds.",
    "Katalogtypisches bodenbündiges Plattformkarussell kommunaler Spielplatzprogramme, ohne Marke: Plattform o1,80 m aus Tränenblech 5/7 mm auf Randwinkel L 60 x 60 x 6 und acht Speichen RHS 60 x 40 x 3 an einer Nabe o168,3 x 8, Achse o60 aus C45 in einem gegossenen Gehäuse mit zwei Kegelrollenlagern auf dem Boden einer 3-mm-Einbauwanne (innen o1,812 m, 0,30 m tief, 6-mm-Laufspalt unter der Fingerfanggrenze 8 mm der DIN EN 1176-1), Wanne auf einem Punktfundament C25/30 0,60 x 0,60 m bis zur Frosttiefe 0,90 m, gegossene EPDM-Umrandung 40 mm bis o2,80 m. Mittelsäule CHS 114,3 x 4 bis 0,75 m, Haltering und vier Haltebügel CHS 33,7 x 3 mit Oberkante 0,637 m. Alle Maße und Massen sind Schätzungen des Typs (Querschnitt x Dichte, Tränenblech nach DIN 59220), die Gebrauchsspuren sind illustrativ: rail-c am Bogen 20 mm durch Mäher oder Vandalismus verbogen, rail-d in blankem verzinktem Rohr ersetzt, Tränenmuster im Laufring 0,56 bis 0,85 m glattgetreten, Umrandung durch schiebende Füße 15 mm tief ausgetreten, Moos am schattigen Rand, ausgekreideter und abgeplatzter Lack mit Rostkragen an den Schweißnähten, Kaugummi auf dem Deck, ein Aufkleber und ein Tag an der Säule, ein trockenes Lager, das knirscht.")

THERMAL_ASSUMPTIONS = [
    T("Reference case: a hot summer afternoon (summer-hot-sun: 32 degC air, 850 W/m2); the dark weathered zinc of the deck (solar absorptance 0.89, emittance 0.28) is the hot surface children touch, the bare new zinc of the replaced rail (0.65, 0.13) reflects more but radiates less, the red powder coat (0.62, 0.90) stays cooler than the deck.", "Bezugsfall: ein heißer Sommernachmittag (summer-hot-sun: 32 degC Luft, 850 W/m2); das dunkle verwitterte Zink des Decks (Absorptionsgrad 0,89, Emissionsgrad 0,28) ist die heiße Fläche, die Kinder berühren, das blanke neue Zink des ersetzten Bügels (0,65, 0,13) reflektiert mehr, strahlt aber weniger ab, die rote Pulverbeschichtung (0,62, 0,90) bleibt kühler als das Deck."),
    T("The pit under the platform is treated as closed: the 6 mm running gap exchanges negligible air, so the spokes, hub, spindle and bearing housing are the hidden region underframe; the buried tray, apron underside and point foundation conduct to the soil (contact 0.85, 0.46 and 1.0 at 22, 24 and 16 degC).", "Die Wanne unter der Plattform gilt als geschlossen: der 6-mm-Laufspalt tauscht vernachlässigbar wenig Luft, daher sind Speichen, Nabe, Achse und Lagergehäuse die verdeckte Region underframe; die eingegrabene Wanne, die Unterseite der Umrandung und das Punktfundament leiten zum Erdreich (Kontakt 0,85, 0,46 und 1,0 bei 22, 24 und 16 degC)."),
    T("The roundabout has no electrical or fuel element: its total power is 0 W.", "Das Karussell hat kein elektrisches oder brennstoffbetriebenes Element: seine Gesamtleistung ist 0 W."),
]

STRUCTURE_ASSUMPTIONS = [
    T("In service the whole roundabout is one rigid body on the fixed support at the base of the point foundation; the platform load goes through the spokes, hub and spindle into the tapered-roller bearing and the housing.", "Im Betrieb ist das ganze Karussell ein starrer Körper auf dem festen Auflager an der Sohle des Punktfundaments; die Plattformlast geht über Speichen, Nabe und Achse ins Kegelrollenlager und das Gehäuse."),
    T("In use four children of 30 kg stand or sit at 0.70 m and an adult pushes the rim tangentially with 150 N (authored service loads, not the design user load of DIN EN 1176-1); at one turn in 2.4 s a child at 0.70 m needs 144 N of grip towards the axis, which the loads, as world vectors, do not carry along in the spinning clip.", "In Benutzung stehen oder sitzen vier Kinder zu 30 kg auf 0,70 m und eine erwachsene Person schiebt tangential mit 150 N am Rand (angesetzte Gebrauchslasten, nicht die Bemessungsnutzerlast der DIN EN 1176-1); bei einer Umdrehung in 2,4 s braucht ein Kind auf 0,70 m 144 N Haltekraft zur Achse, die die Lasten als Weltvektoren im Drehclip nicht mitführen."),
    T("Lifted out, the 203 kg rotating assembly hangs from the crane hook in the DIN 580 eyebolt: the hook is applied as its lifting force (the weight of the assembly, 1.99 kN, ramped in while the slings tighten) because the archive's supports are permanent springs of equal stiffness that would share the weight with the footing; the 654 kg footing stays in the ground on its fixed support, which then carries exactly the footing.", "Ausgehoben hängt die Drehgruppe von 203 kg am Kranhaken in der Ringschraube DIN 580: der Haken ist als seine Hubkraft angesetzt (das Gewicht der Drehgruppe, 1,99 kN, beim Straffen der Anschlagmittel aufgebracht), weil die Auflager des Archivs ständige Federn gleicher Steifigkeit sind, die das Gewicht mit der Gründung teilen würden; die Gründung von 654 kg bleibt auf ihrem festen Auflager im Boden, das dann genau die Gründung trägt."),
    T("Masses are section times density (EN 10219-2, EN 10056-1, DIN 59220, concrete 2400 kg/m3, poured surfacing 950 kg/m3).", "Massen sind Querschnitt mal Dichte (EN 10219-2, EN 10056-1, DIN 59220, Beton 2400 kg/m3, gegossener Belag 950 kg/m3)."),
]


AREAS = {"deck": 2.97, "rails": 0.967, "rail-new": 0.145, "underframe": 1.63, "pit-tray": 5.15, "surfacing": 6.86, "foundation": 1.41}


def com_markers():
    """⚖️ Mass-marker positions per bone (bone-local u, v, w); a bent handrail's centre of mass lies in the air inside its bend, so its marker sits on the tube where the centre line passes nearest to it."""
    out = {}
    for b, (m, cu, cv) in BONE_MASS.items():
        if b in RAIL_IDS:
            p = rail_path(b, FOOT_V[50], 16)
            cu, cv = min(p, key=lambda q: math.hypot(q[0] - cu, q[1] - cv))
        out[b] = (cu, cv, 0.0)
    return out


def representation(s):
    """🗺️ One authored representation."""
    lo, hi = BANDS[s]
    return {"status": "authored", "denominator": s, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[s], "hatches": hatches(s), "geometry": views(s), "decisions": rep_decisions(s)}


def volume(s):
    """🔧️ One authored volume."""
    b = body(s)
    return {"status": "authored", "denominator": s, "body": b, "thermal": thermal_list(s, b), "structure": structure_list(s, b, com_markers()), "decisions": volume_decisions(s)}


def document():
    """📄️ The whole asset document."""
    asset = {
        "id": "used-playground-roundabout",
        "category": "objects",
        "title": T("Used Playground Roundabout", "Gebrauchtes Karussell"),
        "summary": SUMMARY,
        "tags": ["used", "park", "public-space", "play", "roundabout", "carousel", "steel", "galvanised", "powder-coated", "rubber-surfacing", "concrete"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": NOTES},
        "dimensions": {"heightM": rnd(Y_CAP_TOP - Y_FOUND_BOT, 4), "widthM": rnd(2 * R_APRON, 4), "depthM": rnd(2 * R_APRON, 4), "massKg": rnd(TOTAL_KG, 1)},
        "materials": MATERIALS,
        "rig": rig_json(),
        "anchors": ANCHORS,
        "representations": {str(s): representation(s) for s in SCALES},
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": {"environment": {"airTempC": 32, "meanRadiantTempC": 36, "skyTempC": 21, "groundTempC": 50, "relativeHumidity": 0.4, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 850, "diffuseFraction": 0.12, "pressurePa": 101325}, "referenceEnvironment": "summer-hot-sun", "passengers": [], "operating": [], "regions": regions_json(AREAS), "emitters": [], "assumptions": THERMAL_ASSUMPTIONS},
        "structure": {"gravityMS2": G, "dynamics": "inertial", "supports": ["footing-fixed"], "assumptions": STRUCTURE_ASSUMPTIONS, "clinical": False},
        "volumes": {str(s): volume(s) for s in SCALES},
        "presentation": {"defaultPose": "in-service", "defaultClip": "spinning", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    from pathlib import Path
    out = Path(__file__).with_name("🔣️.json")
    out.write_text(json.dumps(document(), ensure_ascii=False, indent=1) + "\n")
    print(f"[DEBUG] wrote {out} ({out.stat().st_size} bytes, {TOTAL_KG:.1f} kg)")
