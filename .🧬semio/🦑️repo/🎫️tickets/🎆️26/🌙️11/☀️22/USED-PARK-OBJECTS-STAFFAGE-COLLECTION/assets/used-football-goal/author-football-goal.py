"""⚽️ Authoritative author script of `used-football-goal`: states every hand-chosen number of the welded steel 5 x 2 m Bolzplatz goal and writes `🔣️.json` next to it.

Run: python3 author-football-goal.py  (prints the bone masses, region areas and solid counts the document is built on).

@see ../../brief.md
@see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
G = 9.80665
RHO_S = 7850.0
RHO_C = 2400.0
RHO_SOIL = 1700.0

B = 0.08
TB = 0.004
RO = 0.008
ZP = 2.54
ZO = 2.58
ZI = 2.50
YC0 = 2.0
YC1 = 2.08
SAG = 0.03
YSP = -0.45
YRU = 0.12
D48 = 0.0483
T48 = 0.0029
R = D48 / 2
XPB = -0.04
XR = -1.5
XT = -0.8
LUGT = 0.008
YB = LUGT + R
YT = YC0 - R
MT = 0.008
SL_O = 0.09
SL_I = 0.083
SL_B = -0.46
FF = 0.4
FFD = 0.6
FR = 0.3
FRD = 0.4
PX0, PX1, PZ = -1.25, 1.25, 2.2
PBOT = -0.1
HX, HRX, HRZ, HD = 0.25, 0.95, 1.7, 0.06
BALL_R = 0.11
BALL = (-0.9, BALL_R, 1.6)
BALL_KG = 0.43
STRIKE_Z = 1.6

A_SHS = (B * B - (B - 2 * TB) ** 2) - (4 - math.pi) * (RO ** 2 - TB ** 2)
I_SHS = 1.11e-6
A_CHS = math.pi / 4 * (D48 ** 2 - (D48 - 2 * T48) ** 2)
I_CHS = math.pi / 64 * (D48 ** 4 - (D48 - 2 * T48) ** 4)
WIRE_D = 0.004
MESH_KG_M2 = 2 * 20 * math.pi * (WIRE_D / 2) ** 2 * RHO_S
MESH_AREA_PER_M2 = 2 * 20 * math.pi * WIRE_D

DHAT = (XT - XR, YT - YB)
DLEN = math.hypot(*DHAT)
DHAT = (DHAT[0] / DLEN, DHAT[1] / DLEN)
NHAT = (-DHAT[1], DHAT[0])
S_B = (LUGT - (YB + NHAT[1] * R)) / DHAT[1]
S_T = (YC0 - (YB + NHAT[1] * R)) / DHAT[1]
BULGE = (0.05, 0.95, 1.1, 0.6, 0.9)


def t(en, de):
    """🗣️ One bilingual text."""
    return {"en": en, "de": de}


def r(x, n=5):
    """🎯️ Rounds a number for the document."""
    v = round(x + 0.0, n)
    return 0.0 if v == 0 else v


def rv(p, n=5):
    """📍️ Rounds a vector."""
    return [r(c, n) for c in p]


def sag(z):
    """〰️ Permanent downward set of the crossbar at lateral position z: 30 mm at mid-span from youths hanging on it, nothing over the posts."""
    return SAG * math.cos(math.pi * z / (2 * ZI)) ** 2 if abs(z) < ZI else 0.0


def hollow(x, z):
    """🕳️ Depth of the worn hollow in the goal mouth below the turf at (x, z): 60 mm where the keeper stands and shots are taken, fading to the lawn."""
    q = ((x - HX) / HRX) ** 2 + (z / HRZ) ** 2
    return HD * (1 - q) ** 2 if q < 1 else 0.0


def back_point(s, off):
    """📐️ A point of the sloping back plane: s along the rear leg from its rear corner, off outward along its normal."""
    return (XR + DHAT[0] * s + NHAT[0] * off, YB + DHAT[1] * s + NHAT[1] * off)


def bulge(s, z):
    """🥅️ Outward bulge of the back mesh where years of shots into the left corner pushed it out (50 mm at the centre)."""
    a, s0, z0, rs, rz = BULGE
    q = ((s - s0) / rs) ** 2 + ((z - z0) / rz) ** 2
    return a * (1 - q) ** 2 if q < 1 else 0.0


FRAMES = {
    "ground": ((0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)),
    "goal-mouth": ((0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)),
    "ball-track": (BALL, (1, 0, 0), (0, 1, 0), (0, 0, 1)),
    "ball": (BALL, (1, 0, 0), (0, 1, 0), (0, 0, 1)),
    "goal-lift": ((0.04, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)),
    "goal": ((0.04, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)),
    "post-right": ((0, YSP, -ZP), (0, 1, 0), (-1, 0, 0), (0, 0, 1)),
    "post-left": ((0, YSP, ZP), (0, 1, 0), (-1, 0, 0), (0, 0, 1)),
    "crossbar": ((0, (YC0 + YC1) / 2, -ZO), (0, 0, 1), (0, 1, 0), (-1, 0, 0)),
    "side-right": ((XR, YB, -ZP), (1, 0, 0), (0, 1, 0), (0, 0, 1)),
    "side-left": ((XR, YB, ZP), (1, 0, 0), (0, 1, 0), (0, 0, 1)),
    "rear-bottom": ((XR, YB, -ZP), (0, 0, 1), (0, 1, 0), (-1, 0, 0)),
    "rear-top": ((XT, YT, -ZP), (0, 0, 1), (0, 1, 0), (-1, 0, 0)),
    "mesh": ((0.04, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)),
}


def loc(bone, p):
    """🧭️ A world point in the rest frame (u, v, w) of a bone."""
    o, u, v, w = FRAMES[bone]
    d = [p[i] - o[i] for i in range(3)]
    return [sum(d[i] * a[i] for i in range(3)) for a in (u, v, w)]


def loc2(bone, x, y):
    """🧭️ A side-elevation world point (x, y) in the (u, v) plane of a sagittal bone."""
    q = loc(bone, (x, y, FRAMES[bone][0][2]))
    return (q[0], q[1])


def lsize(bone, size):
    """📦️ The bone-local extents of a world axis-aligned box."""
    _, u, v, w = FRAMES[bone]
    return [sum(abs(a[i]) * size[i] for i in range(3)) for a in (u, v, w)]


def solid_box(i, bone, mat, tone, lo, hi, role="body", **kw):
    """📦️ A box solid from its world corners."""
    c = [(lo[k] + hi[k]) / 2 for k in range(3)]
    s = {"id": i, "kind": "box", "bone": bone, "material": mat, "role": role, "tone": tone, "size": rv(lsize(bone, [hi[k] - lo[k] for k in range(3)])), "transform": {"translation": rv(loc(bone, c))}}
    s.update(kw)
    return s


def solid_capsule(i, bone, mat, tone, p0, p1, rad, role="body", **kw):
    """💊️ A capsule solid from its world end points."""
    s = {"id": i, "kind": "capsule", "bone": bone, "material": mat, "role": role, "tone": tone, "p0": rv(loc(bone, p0)), "p1": rv(loc(bone, p1)), "r0": r(rad), "r1": r(rad)}
    s.update(kw)
    return s


def solid_ball(i, bone, mat, tone, c, rad, role="body", **kw):
    """⚪️ An ellipsoid (sphere) solid at a world centre."""
    s = {"id": i, "kind": "ellipsoid", "bone": bone, "material": mat, "role": role, "tone": tone, "radii": [r(rad)] * 3, "transform": {"translation": rv(loc(bone, c))}}
    s.update(kw)
    return s


def solid_extrude(i, bone, mat, tone, profile, plane, a, b, role="body", holes=None, **kw):
    """🧊️ An extrude solid in bone-local profile coordinates."""
    s = {"id": i, "kind": "extrude", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": [rv(p) for p in profile], "plane": plane, "from": r(a), "to": r(b)}
    if holes:
        s["holes"] = [[rv(p) for p in h] for h in holes]
    s.update(kw)
    return s


def qmul(a, b):
    """✖️ Quaternion product a*b ([x, y, z, w])."""
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


LIMB = [0.5, 0.5, 0.5, 0.5]


def loft_path(i, bone, mat, tone, pts, rx, ry, exponent=2, role="body", **kw):
    """🪈️ A loft through bone-local points in the (u, v) plane, each section normal to the local tangent (limb rotation turned about w)."""
    secs = []
    for k, p in enumerate(pts):
        a = pts[max(k - 1, 0)]
        b = pts[min(k + 1, len(pts) - 1)]
        th = math.atan2(b[1] - a[1], b[0] - a[0])
        q = qmul([0, 0, math.sin(th / 2), math.cos(th / 2)], LIMB)
        secs.append({"at": rv([p[0], p[1], p[2] if len(p) > 2 else 0]), "rx": r(rx), "ry": r(ry), "exponent": exponent, "rotation": rv(q, 6)})
    s = {"id": i, "kind": "loft", "bone": bone, "material": mat, "role": role, "tone": tone, "sections": secs, "capStart": True, "capEnd": True}
    s.update(kw)
    return s


def orient(verts, tris):
    """🔄️ Flips a closed triangle list outward when its signed volume is negative; returns triangles and volume."""
    vol = 0.0
    for a, b, c in tris:
        p, q, s = verts[a], verts[b], verts[c]
        vol += (p[0] * (q[1] * s[2] - q[2] * s[1]) - p[1] * (q[0] * s[2] - q[2] * s[0]) + p[2] * (q[0] * s[1] - q[1] * s[0])) / 6
    if vol < 0:
        tris = [[a, c, b] for a, b, c in tris]
    return tris, abs(vol)


def tri_area(p, q, s):
    """📐️ Area of a triangle."""
    u = [q[k] - p[k] for k in range(3)]
    w = [s[k] - p[k] for k in range(3)]
    c = [u[1] * w[2] - u[2] * w[1], u[2] * w[0] - u[0] * w[2], u[0] * w[1] - u[1] * w[0]]
    return math.sqrt(sum(x * x for x in c)) / 2


def grid_solid(top, bot):
    """🧱️ A closed polyhedron between two grids of equal topology (top[i][j], bot[i][j]); returns vertices, triangles, top area and volume."""
    n, m = len(top), len(top[0])
    idx = lambda g, i, j: g * n * m + i * m + j
    verts = [p for g in (top, bot) for row in g for p in row]
    tris = []
    area = 0.0
    for i in range(n - 1):
        for j in range(m - 1):
            a, b, c, d = idx(0, i, j), idx(0, i, j + 1), idx(0, i + 1, j + 1), idx(0, i + 1, j)
            tris += [[a, b, c], [a, c, d]]
            area += tri_area(verts[a], verts[b], verts[c]) + tri_area(verts[a], verts[c], verts[d])
            a, b, c, d = idx(1, i, j), idx(1, i, j + 1), idx(1, i + 1, j + 1), idx(1, i + 1, j)
            tris += [[a, c, b], [a, d, c]]
    loop = [(i, 0) for i in range(n)] + [(n - 1, j) for j in range(1, m)] + [(i, m - 1) for i in range(n - 2, -1, -1)] + [(0, j) for j in range(m - 2, 0, -1)]
    directed = {(p, q) for tri in tris for p, q in ((tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0]))}
    for k in range(len(loop)):
        (i0, j0), (i1, j1) = loop[k], loop[(k + 1) % len(loop)]
        a, b, c, d = idx(0, i0, j0), idx(0, i1, j1), idx(1, i1, j1), idx(1, i0, j0)
        tris += [[b, a, d], [b, d, c]] if (a, b) in directed else [[a, b, c], [a, c, d]]
    tris, vol = orient(verts, tris)
    return verts, tris, area, vol


def patch_mesh(nx, nz, dished):
    """🟫️ The worn goal-mouth soil sample as world vertices: lawn level at the edges, the hollow dished into its top, a flat cut 0.1 m below."""
    xs = [PX0 + (PX1 - PX0) * k / (nx - 1) for k in range(nx)]
    zs = [-PZ + 2 * PZ * k / (nz - 1) for k in range(nz)]
    top = [[(x, -hollow(x, z) if dished else 0.0, z) for z in zs] for x in xs]
    bot = [[(x, PBOT, z) for z in zs] for x in xs]
    return grid_solid(top, bot)


ZM = ZP + R + MT


def back_mesh(ns, nz, bulged):
    """🥅️ The sloping back mesh panel as world vertices, 8 mm thick on the outside of the rear legs, bulged where the shots land."""
    ss = [S_B + (S_T - S_B) * k / (ns - 1) for k in range(ns)]
    zs = [-ZM + 2 * ZM * k / (nz - 1) for k in range(nz)]
    inner, outer = [], []
    for s in ss:
        ri, ro = [], []
        for z in zs:
            b = bulge(s, z) if bulged else 0.0
            xi, yi = back_point(s, R + b)
            xo, yo = back_point(s, R + b + MT)
            ri.append((xi, yi, z))
            ro.append((xo, yo, z))
        inner.append(ri)
        outer.append(ro)
    return grid_solid(outer, inner)


def to_local_mesh(bone, verts):
    """🧭️ World vertices in a bone frame."""
    return [rv(loc(bone, p), 6) for p in verts]


def polyhedron(i, bone, mat, tone, verts, tris, role="body", **kw):
    """🔺️ A polyhedron solid from world vertices."""
    s = {"id": i, "kind": "polyhedron", "bone": bone, "material": mat, "role": role, "tone": tone, "vertices": to_local_mesh(bone, verts), "triangles": tris}
    s.update(kw)
    return s


def hull(points):
    """🫧️ Convex hull of 2D points (monotone chain), counter-clockwise."""
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) < 3:
        return pts
    cr = lambda o, a, b: (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lo, up = [], []
    for p in pts:
        while len(lo) >= 2 and cr(lo[-2], lo[-1], p) <= 0:
            lo.pop()
        lo.append(p)
    for p in reversed(pts):
        while len(up) >= 2 and cr(up[-2], up[-1], p) <= 0:
            up.pop()
        up.append(p)
    return lo[:-1] + up[:-1]


def poly_area(p):
    """📐️ Area of a simple polygon."""
    return abs(sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))) / 2


def perimeter(p):
    """📏️ Perimeter of a closed polygon."""
    return sum(math.dist(p[i], p[(i + 1) % len(p)]) for i in range(len(p)))


P0 = (XPB, YB)
P1 = (XR, YB)
P2 = (XT, YT)
P3 = (XPB, YT)
BOW = [P0, P1, P2, P3]
BOW_SEGS = [(P0, P1, "ground-bar"), (P1, P2, "rear-leg"), (P2, P3, "top-bar")]
BOW_LEN = sum(math.dist(a, b) for a, b, _ in BOW_SEGS)
SIDE_MESH = [(XPB, LUGT), back_point(S_B, R)[0:1] + (LUGT,), (back_point(S_T, R)[0], YC0), (XPB, YC0)]
TOP_MESH_X = (back_point(S_T, R + MT)[0], XPB)
BACK_MESH_PLAN_X = (back_point(S_B, R + MT)[0], TOP_MESH_X[0])


def fillet(points, rad, n):
    """🪝️ A polyline with its inner corners rounded by arcs of radius rad (n points per arc)."""
    out = [points[0]]
    for k in range(1, len(points) - 1):
        a, b, c = points[k - 1], points[k], points[k + 1]
        d1 = (a[0] - b[0], a[1] - b[1])
        d2 = (c[0] - b[0], c[1] - b[1])
        l1, l2 = math.hypot(*d1), math.hypot(*d2)
        d1 = (d1[0] / l1, d1[1] / l1)
        d2 = (d2[0] / l2, d2[1] / l2)
        ang = math.acos(max(-1, min(1, d1[0] * d2[0] + d1[1] * d2[1])))
        tlen = rad / math.tan(ang / 2)
        s0 = (b[0] + d1[0] * tlen, b[1] + d1[1] * tlen)
        s1 = (b[0] + d2[0] * tlen, b[1] + d2[1] * tlen)
        for j in range(n + 1):
            f = j / n
            m = ((1 - f) ** 2 * s0[0] + 2 * f * (1 - f) * b[0] + f * f * s1[0], (1 - f) ** 2 * s0[1] + 2 * f * (1 - f) * b[1] + f * f * s1[1])
            out.append(m)
    out.append(points[-1])
    return out


SEG_ROT = None

M_POST = A_SHS * (YC0 - YSP) * RHO_S
M_CROSS = A_SHS * 2 * ZO * RHO_S
M_LUG = 0.1 * 0.1 * LUGT * RHO_S
M_BOW = A_CHS * BOW_LEN * RHO_S + M_LUG
M_REAR = A_CHS * 2 * ZP * RHO_S
SIDE_MESH_AREA = poly_area(SIDE_MESH)
TOP_MESH_AREA = (TOP_MESH_X[1] - TOP_MESH_X[0]) * 2 * ZM
BACK_MESH_AREA = (S_T - S_B) * 2 * ZM
MESH_PANEL_AREA = 2 * SIDE_MESH_AREA + TOP_MESH_AREA + BACK_MESH_AREA
M_MESH = MESH_PANEL_AREA * MESH_KG_M2 * 1.04
M_GOAL = 2 * M_POST + M_CROSS + 2 * M_BOW + 2 * M_REAR + M_MESH
M_TOTAL = M_GOAL + BALL_KG
M_FOOT = 2 * FF * FF * FFD * RHO_C + 2 * FR * FR * FRD * RHO_C


def bow_com():
    """⚖️ Centre of mass of a side bow in its bone frame (tube length-weighted, lug included)."""
    sx = sy = 0.0
    for a, b, _ in BOW_SEGS:
        L = math.dist(a, b)
        sx += A_CHS * RHO_S * L * (a[0] + b[0]) / 2
        sy += A_CHS * RHO_S * L * (a[1] + b[1]) / 2
    sx += M_LUG * XR
    sy += M_LUG * LUGT / 2
    return loc2("side-right", sx / M_BOW, sy / M_BOW)


def mesh_com():
    """⚖️ Centre of mass of the mesh panels in the mesh bone frame (area-weighted panel centroids)."""
    side_c = (sum(p[0] for p in SIDE_MESH) / 4, sum(p[1] for p in SIDE_MESH) / 4)
    top_c = ((TOP_MESH_X[0] + TOP_MESH_X[1]) / 2, YC0 + MT / 2)
    back_c = back_point((S_B + S_T) / 2, R + MT / 2)
    ws = [(2 * SIDE_MESH_AREA, side_c), (TOP_MESH_AREA, top_c), (BACK_MESH_AREA, back_c)]
    tot = sum(w for w, _ in ws)
    return loc2("mesh", sum(w * c[0] for w, c in ws) / tot, sum(w * c[1] for w, c in ws) / tot)


REFS = [
    ("en-748-2013", "EN 748:2013+A1:2018 Playing field equipment - Football goals - Functional, safety requirements and test methods (goal sizes including the 5 x 2 m small-sided goal measured between the inner edges of posts and crossbar; posts and crossbar of equal width and depth; stability test: a horizontal force of 1100 N at the middle of the crossbar must not overturn the goal; strength test of the crossbar under a vertical load at mid-span)."),
    ("en-16579-2018", "EN 16579:2018 Playing field equipment - Portable and fixed goals - Functional, safety requirements and test methods (anchorage of goals in ground sleeves, entrapment of fingers in net fixings, stability)."),
    ("din-18035-1-2018", "DIN 18035-1:2018 Sports grounds - Part 1: Outdoor play and athletics areas, planning and dimensions (small-sided pitches and kick-about areas with 5 x 2 m goals)."),
    ("en-10219-2-2006", "EN 10219-2:2006 Cold formed welded structural hollow sections of non-alloy and fine grain steels - Part 2: Tolerances, dimensions and sectional properties (SHS 80 x 80 x 4: A 11.7 cm2, 9.22 kg/m, I 111 cm4, outer corner radius 2t; CHS 48.3 x 2.9: A 4.14 cm2, 3.25 kg/m, I 10.7 cm4)."),
    ("en-10025-2-2019", "EN 10025-2:2019 Hot rolled products of structural steels - Part 2: Non-alloy structural steels (S235: yield strength 235 MPa, E 210 GPa)."),
    ("en-iso-1461-2022", "EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles (zinc coating of goal frames, sleeves and mesh; the dull grey patina that forms outdoors within a few years)."),
    ("en-1991-1-1-2002", "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: densities, self-weight, imposed loads (Table A.1: plain concrete 24 kN/m3; Table A.4: steel 77-78.5 kN/m3)."),
    ("din-en-206-2021", "DIN EN 206:2021 and DIN 1045-2:2023 Concrete - Specification, performance, production and conformity (C20/25, exposure XC2/XF1 for unreinforced point foundations of sports equipment in the ground)."),
    ("ifab-laws-2024", "IFAB (2024). Laws of the Game 2024/25, Law 2 The Ball (size 5: circumference 68-70 cm, mass 410-450 g at the start of the match)."),
    ("asai-2002", "Asai, T., Carre, M. J., Akatsuka, T., Haake, S. J. (2002). The curve kick of a football I: impact with the foot. Sports Engineering 5(4), 183-192 (foot-ball contact about 9 ms, ball speeds of 25-30 m/s for full shots)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (Table A.11: total hemispherical emissivity of oxidised steel about 0.79, of galvanised sheet 0.2-0.3 new, of concrete and soil 0.9-0.95; solar absorptivity of metals and paints)."),
    ("berdahl-bretz-1997", "Berdahl, P., Bretz, S. E. (1997). Preliminary survey of the solar reflectance of cool roofing materials. Energy and Buildings 25(2), 149-158 (white coatings reflect about 0.8 new and lose 0.1-0.2 of their reflectance by chalking and soiling within a few years outdoors)."),
    ("oke-1987", "Oke, T. R. (1987). Boundary Layer Climates, 2nd ed., Routledge (Table 1.1: albedo of dry bare soil 0.05-0.40, emissivity 0.90-0.98; grass albedo 0.16-0.26)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel 50 W/mK, 7800 kg/m3, 450 J/kgK; concrete 2400 kg/m3, 2.0 W/mK, 1000 J/kgK; clay or silt 1.5 W/mK, 1200-1800 kg/m3)."),
    ("municipal-sports-goal-catalogues", "Municipal sports-ground and playground equipment catalogues of German manufacturers (catalogue-typical data, no brand): welded steel Bolzplatz goals 5 x 2 m, hot-dip galvanised and white powder-coated, posts and crossbar 80 x 80 mm square tube, net bows of 48.3 mm tube, rigid welded steel mesh 50 x 50 x 4 mm or steel chain netting instead of a rope net for vandal resistance, posts in galvanised ground sleeves in concrete point foundations, the rear corners anchored to small footings; 180-300 kg."),
]


def mat(i, en, de, eps, alpha, rho, k, cp, e=None, fs=None, src="literature", ref="iso-10456-2007"):
    """🧱️ One material."""
    m = {"id": i, "title": t(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp}
    if e:
        m["elasticModulusPa"] = e
    if fs:
        m["strengthPa"] = fs
    m["source"] = src
    m["reference"] = ref
    return m


MATERIALS = [
    mat("steel-white-coated", "Galvanised S235 steel, white powder coat chalked and flaking", "Verzinkter Stahl S235, weiße Pulverbeschichtung kreidend und abplatzend", 0.9, 0.35, RHO_S, 50, 460, 210e9, 235e6, "authored-estimate", "berdahl-bretz-1997"),
    mat("steel-galvanised-weathered", "Hot-dip galvanised S235 steel with dull zinc patina", "Feuerverzinkter Stahl S235 mit matter Zinkpatina", 0.45, 0.7, RHO_S, 50, 460, 210e9, 235e6, "authored-estimate", "incropera-2018"),
    mat("steel-rusted", "Steel with rust where the coating is gone", "Stahl mit Rost, wo die Beschichtung fehlt", 0.82, 0.82, RHO_S, 50, 460, 210e9, 235e6, "literature", "incropera-2018"),
    mat("steel-mesh-galvanised", "Welded galvanised steel mesh 50 x 50 x 4 mm", "Verzinkte Stahl-Schweißgittermatte 50 x 50 x 4 mm", 0.45, 0.7, RHO_S, 50, 460, 210e9, 400e6, "authored-estimate", "en-iso-1461-2022"),
    mat("concrete-c20", "Concrete C20/25 of the point foundations", "Beton C20/25 der Punktfundamente", 0.92, 0.6, RHO_C, 2.0, 1000, 30e9, 20e6, "literature", "iso-10456-2007"),
    mat("soil-compacted", "Compacted bare soil where the turf is worn away", "Verdichteter nackter Boden, wo der Rasen abgetreten ist", 0.93, 0.78, RHO_SOIL, 1.5, 900, None, None, "literature", "oke-1987"),
    mat("football-pu", "Scuffed polyurethane football cover", "Abgeschabte Polyurethan-Balloberfläche", 0.9, 0.55, 1100, 0.2, 1500, None, None, "authored-estimate", "ifab-laws-2024"),
]

W = {"paint": "steel-white-coated", "galv": "steel-galvanised-weathered", "rust": "steel-rusted", "mesh": "steel-mesh-galvanised", "conc": "concrete-c20", "soil": "soil-compacted", "ball": "football-pu"}
TONE = {"paint": 0.92, "galv": 0.6, "rust": 0.32, "mesh": 0.84, "conc": 0.66, "soil": 0.5, "ball": 0.88, "sleeve": 0.45}
REG = {"paint": "painted-frame", "galv": "galvanised-frame", "rust": "rust-feet", "mesh": "steel-mesh", "conc": "footings", "soil": "goal-mouth", "ball": "football"}


def sec(area, inertia, fibre, material, en, de):
    """📏️ A member section."""
    return {"areaM2": r(area, 7), "secondMomentM4": r(inertia, 10), "extremeFibreM": r(fibre, 5), "material": material, "title": t(en, de)}


def mass(kg, comx, comy, dist, ref):
    """⚖️ A segment mass."""
    return {"kg": r(kg, 2), "comX": r(comx, 4), "comY": r(comy, 4), "distribution": dist, "source": "literature", "reference": ref}


SHS_SEC = lambda: sec(A_SHS, I_SHS, B / 2, "steel-white-coated", "Square hollow section 80 x 80 x 4 mm, S235 (EN 10219-2)", "Quadratrohr 80 x 80 x 4 mm, S235 (EN 10219-2)")
CHS_SEC = lambda: sec(A_CHS, I_CHS, R, "steel-galvanised-weathered", "Circular hollow section 48.3 x 2.9 mm, S235 (EN 10219-2)", "Rundrohr 48,3 x 2,9 mm, S235 (EN 10219-2)")
BOW_C = bow_com()
MESH_C = mesh_com()


def bone(i, parent, en, de, x, y, angle, length, joint, z=None, abd=None, m=None, s=None):
    """🦴️ One bone."""
    b = {"id": i, "parent": parent, "title": t(en, de), "x": r(x), "y": r(y)}
    if z is not None:
        b["z"] = r(z)
    b["angleDeg"] = angle
    if abd is not None:
        b["abductionDeg"] = abd
    b["lengthM"] = r(length)
    b["joint"] = joint
    if m:
        b["mass"] = m
    if s:
        b["section"] = s
    return b


FIXED = {"type": "fixed"}
RIG = {
    "kind": "structural",
    "root": "ground",
    "bones": [
        bone("ground", None, "Point foundations and ground sleeves (stay in the ground)", "Punktfundamente und Bodenhülsen (bleiben im Boden)", 0, 0, 0, 0, FIXED),
        bone("goal-mouth", "ground", "Worn goal-mouth soil sample", "Abgetretene Bodenprobe im Torraum", 0, 0, 0, 0, FIXED),
        bone("ball-track", "goal-mouth", "Ball travel along the pitch", "Ballweg entlang des Spielfelds", BALL[0], BALL[1], 0, 0.05, {"type": "prismatic", "axisDeg": 0, "minM": -1.0, "maxM": 6.5}, z=BALL[2]),
        bone("ball", "ball-track", "Size 5 street football", "Straßenfußball Größe 5", 0, 0, 0, 0.1, {"type": "prismatic", "axisDeg": 90, "minM": -0.05, "maxM": 2.2}, m=mass(BALL_KG, 0, 0, "point", "ifab-laws-2024")),
        bone("goal-lift", "ground", "Lift out of the ground sleeves", "Heben aus den Bodenhülsen", 0.04, 0, 0, 0.05, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.6}),
        bone("goal", "goal-lift", "Welded goal frame, tipping about the front edge of the post feet", "Geschweißter Torrahmen, kippend um die Vorderkante der Pfostenfüße", 0, 0, 0, 0.05, {"type": "revolute", "minDeg": -92, "maxDeg": 0}),
        bone("post-right", "goal", "Right goal post with ground spigot", "Rechter Torpfosten mit Einsteckrohr", -0.04, YSP, 90, YC0 - YSP, FIXED, z=-ZP, m=mass(M_POST, (YC0 - YSP) / 2, 0, "uniform", "en-10219-2-2006"), s=SHS_SEC()),
        bone("post-left", "goal", "Left goal post with ground spigot", "Linker Torpfosten mit Einsteckrohr", -0.04, YSP, 90, YC0 - YSP, FIXED, z=ZP, m=mass(M_POST, (YC0 - YSP) / 2, 0, "uniform", "en-10219-2-2006"), s=SHS_SEC()),
        bone("crossbar", "goal", "Crossbar, bent 30 mm at mid-span", "Querlatte, in Feldmitte 30 mm durchgebogen", -0.04, (YC0 + YC1) / 2, 0, 2 * ZO, FIXED, z=-ZO, abd=90, m=mass(M_CROSS, ZO, -SAG / 2, "uniform", "en-10219-2-2006"), s=SHS_SEC()),
        bone("side-right", "goal", "Right net bow (ground bar, rear leg, top bar)", "Rechter Netzbügel (Bodenrohr, hinteres Bein, Oberrohr)", XR - 0.04, YB, 0, XPB - XR, FIXED, z=-ZP, m=mass(M_BOW, BOW_C[0], BOW_C[1], "point", "en-10219-2-2006"), s=CHS_SEC()),
        bone("side-left", "goal", "Left net bow (ground bar, rear leg, top bar)", "Linker Netzbügel (Bodenrohr, hinteres Bein, Oberrohr)", XR - 0.04, YB, 0, XPB - XR, FIXED, z=ZP, m=mass(M_BOW, BOW_C[0], BOW_C[1], "point", "en-10219-2-2006"), s=CHS_SEC()),
        bone("rear-bottom", "goal", "Rear ground bar", "Hinteres Bodenrohr", XR - 0.04, YB, 0, 2 * ZP, FIXED, z=-ZP, abd=90, m=mass(M_REAR, ZP, 0, "uniform", "en-10219-2-2006"), s=CHS_SEC()),
        bone("rear-top", "goal", "Rear top bar", "Hinteres Oberrohr", XT - 0.04, YT, 0, 2 * ZP, FIXED, z=-ZP, abd=90, m=mass(M_REAR, ZP, 0, "uniform", "en-10219-2-2006"), s=CHS_SEC()),
        bone("mesh", "goal", "Welded steel mesh panels (top, back, sides)", "Stahl-Gittermatten (oben, hinten, seitlich)", 0, 0, 0, 0.05, FIXED, m=mass(M_MESH, MESH_C[0], MESH_C[1], "point", "en-iso-1461-2022")),
    ],
}


def anchor(i, b, x, y, kind, en, de, z=None, **kw):
    """📍️ One anchor."""
    a = {"id": i, "bone": b, "x": r(x), "y": r(y)}
    if z is not None:
        a["z"] = r(z)
    a["kind"] = kind
    a["title"] = t(en, de)
    a.update(kw)
    return a


ANCHORS = [
    anchor("sleeve-right", "post-right", -YSP, 0, "support", "Right post clamped in its ground sleeve at turf level", "Rechter Pfosten in seiner Bodenhülse auf Rasenhöhe eingespannt", support="fixed", footprintLengthM=B, footprintWidthM=B),
    anchor("sleeve-left", "post-left", -YSP, 0, "support", "Left post clamped in its ground sleeve at turf level", "Linker Pfosten in seiner Bodenhülse auf Rasenhöhe eingespannt", support="fixed", footprintLengthM=B, footprintWidthM=B),
    anchor("lug-right", "side-right", 0, -YB, "support", "Right rear lug bolted to its anchor footing", "Rechte hintere Lasche auf ihr Ankerfundament geschraubt", support="pin", footprintLengthM=0.1, footprintWidthM=0.1, frictionCoefficient=0.5),
    anchor("lug-left", "side-left", 0, -YB, "support", "Left rear lug bolted to its anchor footing", "Linke hintere Lasche auf ihr Ankerfundament geschraubt", support="pin", footprintLengthM=0.1, footprintWidthM=0.1, frictionCoefficient=0.5),
    anchor("hands-post-right", "post-right", -YSP + 1.0, 0, "support", "Two depot workers holding the right post at 1 m", "Zwei Bauhofmitarbeiter halten den rechten Pfosten auf 1 m", support="pin"),
    anchor("hands-post-left", "post-left", -YSP + 1.0, 0, "support", "Two depot workers holding the left post at 1 m", "Zwei Bauhofmitarbeiter halten den linken Pfosten auf 1 m", support="pin"),
    anchor("hands-rear-right", "side-right", 0.1, 0, "support", "A depot worker lifting the right rear corner", "Ein Bauhofmitarbeiter hebt die rechte hintere Ecke", support="pin"),
    anchor("hands-rear-left", "side-left", 0.1, 0, "support", "A depot worker lifting the left rear corner", "Ein Bauhofmitarbeiter hebt die linke hintere Ecke", support="pin"),
    anchor("rest-post-right", "post-right", -YSP + 1.0, -B / 2, "contact", "Front face of the right post lying on the turf", "Vorderseite des rechten Pfostens auf dem Rasen liegend", support="contact", normalDeg=90, footprintLengthM=YC0, footprintWidthM=B, frictionCoefficient=0.5),
    anchor("rest-post-left", "post-left", -YSP + 1.0, -B / 2, "contact", "Front face of the left post lying on the turf", "Vorderseite des linken Pfostens auf dem Rasen liegend", support="contact", normalDeg=90, footprintLengthM=YC0, footprintWidthM=B, frictionCoefficient=0.5),
    anchor("rest-crossbar", "crossbar", ZO, 0, "contact", "Front face of the crossbar lying on the turf", "Vorderseite der Querlatte auf dem Rasen liegend", z=-B / 2, support="contact", normalDeg=90, footprintLengthM=2 * ZO, footprintWidthM=B, frictionCoefficient=0.5),
    anchor("ball-ground", "ball", 0, -BALL_R, "contact", "Ball resting on the ground", "Ball auf dem Boden liegend", support="contact", normalDeg=90, footprintLengthM=0.05, footprintWidthM=0.05, frictionCoefficient=0.6),
    anchor("crossbar-centre", "crossbar", ZO, 0, "load", "Middle of the crossbar, front face (inspection pull)", "Mitte der Querlatte, Vorderseite (Prüfzug)", z=-B / 2),
    anchor("crossbar-strike", "crossbar", ZO + STRIKE_Z, -0.02, "load", "Where the shot hits the crossbar, 1.6 m left of the centre", "Wo der Schuss die Latte trifft, 1,6 m links der Mitte", z=-B / 2),
    anchor("crossbar-grip", "crossbar", ZO, B / 2, "load", "Where a youth hangs on the crossbar", "Wo sich ein Jugendlicher an die Latte hängt"),
]

REGION_IDS = [REG[k] for k in ("paint", "galv", "rust", "mesh", "conc", "soil", "ball")]


def bind(paint, galv, rust, mesh_, conc, soil, ball):
    """🌡️ The region bindings of one pose."""
    return {"regions": dict(zip(REGION_IDS, [paint, galv, rust, mesh_, conc, soil, ball])), "emitters": {}}


def rb(e, s, **kw):
    """🌞️ One region binding."""
    b = {"exposedFraction": e, "sunlitFraction": s}
    b.update(kw)
    return b


TH_SERVICE = bind(rb(1.0, 0.45), rb(1.0, 0.4), rb(1.0, 0.4), rb(1.0, 0.45), rb(1.0, 0.7), rb(1.0, 0.75), rb(0.97, 0.5, contact={"fraction": 0.03, "tempC": 38}))
TH_LYING = bind(rb(0.75, 0.5, contact={"fraction": 0.25, "tempC": 30}), rb(1.0, 0.55), rb(1.0, 0.5), rb(1.0, 0.6), rb(1.0, 0.85), rb(1.0, 0.55), rb(0.97, 0.5, contact={"fraction": 0.03, "tempC": 38}))
TH_USE = bind(rb(1.0, 0.45), rb(1.0, 0.4), rb(1.0, 0.4), rb(1.0, 0.45), rb(1.0, 0.7), rb(1.0, 0.75), rb(1.0, 0.6))
SH = G


def load(i, kind, en, de, anchor_, fx, fy, src="authored-estimate", ref="en-748-2013"):
    """🧲️ One applied load at an anchor."""
    return {"id": i, "kind": kind, "title": t(en, de), "anchor": anchor_, "fxN": r(fx, 1), "fyN": r(fy, 1), "source": src, "reference": ref}


BALL_V = 25.0
BALL_E = 0.6
BALL_DT = 0.009
STRIKE_N = BALL_KG * BALL_V * (1 + BALL_E) / BALL_DT
TIP_N = 1100.0
HANG_N = 75 * G * 1.5
SERVICE_CONTACTS = ["sleeve-right", "sleeve-left", "lug-right", "lug-left", "ball-ground"]
LIFT_DY = 0.5

POSES = [
    {"id": "in-service", "title": t("In service on the kick-about pitch", "Im Einsatz auf dem Bolzplatz"), "description": t("The welded goal stands in its two ground sleeves with the rear lugs bolted to their anchor footings, on a summer afternoon; a worn ball lies in the back of the goal and the goal mouth is trodden into a bare hollow.", "Das geschweißte Tor steht in seinen zwei Bodenhülsen, die hinteren Laschen auf ihre Ankerfundamente geschraubt, an einem Sommernachmittag; ein abgenutzter Ball liegt hinten im Tor und der Torraum ist zu einer kahlen Mulde getreten."),
     "bones": {}, "ground": "authored", "contacts": SERVICE_CONTACTS, "thermal": TH_SERVICE, "loads": []},
    {"id": "maintenance", "title": t("Laid face-down for pitch renovation", "Zur Platzpflege nach vorn umgelegt"), "description": t("The depot crew has unbolted the rear lugs, lifted the goal 0.5 m out of its sleeves and tipped it forward onto the turf, posts and crossbar lying face-down and the mesh cage standing up, so the sleeves can be cleaned and the goal mouth re-turfed; the foundations stay in the ground.", "Der Bauhof hat die hinteren Laschen gelöst, das Tor 0,5 m aus seinen Hülsen gehoben und nach vorn auf den Rasen gekippt, Pfosten und Latte liegen mit der Vorderseite unten und der Gitterkorb steht hoch, damit die Hülsen gereinigt und der Torraum neu besodet werden kann; die Fundamente bleiben im Boden."),
     "bones": {"goal": {"angleDeg": -90}}, "ground": "authored", "contacts": ["rest-post-right", "rest-post-left", "rest-crossbar", "ball-ground"], "thermal": TH_LYING, "loads": []},
    {"id": "in-use", "title": t("Shot rattling the crossbar", "Lattenkracher"), "description": t("A full shot at about 25 m/s hits the underside edge of the crossbar 1.6 m left of the centre: the 430 g ball loads the bar for about 9 ms with almost 2 kN, the dent and flaking paint of years of such shots are on that spot.", "Ein voller Schuss mit etwa 25 m/s trifft die Unterkante der Latte 1,6 m links der Mitte: der 430-g-Ball belastet die Latte etwa 9 ms lang mit fast 2 kN, die Delle und die abgeplatzte Farbe jahrelanger Treffer sitzen genau dort."),
     "bones": {"ball-track": {"dx": r(B / 2 + BALL_R - BALL[0] + 0.0)}, "ball": {"dy": r(YC0 - 0.03 - BALL[1])}}, "ground": "authored", "contacts": SERVICE_CONTACTS[:4], "thermal": TH_USE,
     "loads": [load("ball-strike", "live", "Ball impact on the crossbar (0.43 kg at 25 m/s, restitution 0.6, 9 ms contact)", "Ballaufprall auf die Latte (0,43 kg mit 25 m/s, Stoßzahl 0,6, 9 ms Kontakt)", "crossbar-strike", -STRIKE_N, 0, ref="asai-2002")]},
    {"id": "tipping-check", "title": t("Tipping check by the inspector", "Kippprüfung durch den Prüfer"), "description": t("At the annual inspection the inspector pulls the middle of the crossbar towards the pitch with a strap and a force gauge until 1100 N, the stability test force of EN 748; the goal, clamped in its sleeves and bolted at the rear, does not tip.", "Bei der jährlichen Prüfung zieht der Prüfer die Mitte der Latte mit Gurt und Kraftmesser bis 1100 N zum Spielfeld, die Standsicherheits-Prüfkraft der EN 748; das in seinen Hülsen eingespannte und hinten verschraubte Tor kippt nicht."),
     "bones": {}, "ground": "authored", "contacts": SERVICE_CONTACTS, "thermal": TH_SERVICE,
     "loads": [load("inspection-pull", "imposed", "Horizontal stability test force at the middle of the crossbar (EN 748)", "Horizontale Standsicherheits-Prüfkraft in der Mitte der Latte (EN 748)", "crossbar-centre", TIP_N, 0, src="literature")]},
    {"id": "crossbar-hang", "title": t("Youth hanging on the crossbar", "Jugendlicher an der Latte hängend"), "description": t("A 75 kg youth jumps up and swings from the middle of the crossbar, about 1.5 times the body weight at the bottom of the swing: this misuse, repeated over years, bent the bar 30 mm.", "Ein 75 kg schwerer Jugendlicher springt hoch und schwingt an der Mitte der Latte, etwa das 1,5-fache des Körpergewichts im Tiefpunkt: dieser Missbrauch hat die Latte über Jahre um 30 mm durchgebogen."),
     "bones": {}, "ground": "authored", "contacts": SERVICE_CONTACTS, "thermal": TH_SERVICE,
     "loads": [load("youth-hanging", "occupant", "Youth hanging and swinging on the crossbar (75 kg, dynamic factor 1.5)", "Jugendlicher hängt und schwingt an der Latte (75 kg, dynamischer Faktor 1,5)", "crossbar-grip", 0, -HANG_N)]},
    {"id": "lifted", "key": True, "title": t("Lifted out of the sleeves", "Aus den Hülsen gehoben"), "description": t("Four depot workers hold the goal 0.5 m up, the spigots clear of the sleeves.", "Vier Bauhofmitarbeiter halten das Tor 0,5 m hoch, die Einsteckrohre frei über den Hülsen."),
     "bones": {"goal-lift": {"dy": LIFT_DY}}, "ground": "authored", "contacts": ["hands-post-right", "hands-post-left", "hands-rear-right", "hands-rear-left", "ball-ground"], "thermal": TH_SERVICE, "loads": []},
]

KICK_DX = 6.0
CLIPS = [
    {"id": "lift-out", "title": t("Lifting out and laying down", "Ausheben und Umlegen"), "description": t("The crew unbolts the rear lugs, four workers lift the goal straight out of its sleeves, tip it forward and lay it face-down on the turf in front of the foundations.", "Der Trupp löst die hinteren Laschen, vier Mitarbeiter heben das Tor senkrecht aus seinen Hülsen, kippen es nach vorn und legen es mit der Vorderseite nach unten vor den Fundamenten auf den Rasen."),
     "durationS": 40, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "linear"}, {"t": 8, "pose": "in-service", "ease": "linear"}, {"t": 14, "pose": "lifted", "ease": "linear"},
              {"t": 20, "pose": "lifted", "ease": "linear", "bones": {"goal-lift": {"dy": 0.45}, "goal": {"angleDeg": -30}}},
              {"t": 26, "pose": "lifted", "ease": "linear", "bones": {"goal-lift": {"dy": 0.27}, "goal": {"angleDeg": -60}}},
              {"t": 32, "pose": "maintenance", "ease": "linear"}, {"t": 40, "pose": "maintenance", "ease": "linear"}],
     "contacts": [{"anchor": "sleeve-right", "fromS": 0, "toS": 9}, {"anchor": "sleeve-left", "fromS": 0, "toS": 9}, {"anchor": "lug-right", "fromS": 0, "toS": 8}, {"anchor": "lug-left", "fromS": 0, "toS": 8},
                  {"anchor": "hands-post-right", "fromS": 8, "toS": 32}, {"anchor": "hands-post-left", "fromS": 8, "toS": 32}, {"anchor": "hands-rear-right", "fromS": 8, "toS": 32}, {"anchor": "hands-rear-left", "fromS": 8, "toS": 32},
                  {"anchor": "rest-post-right", "fromS": 31.5, "toS": 40}, {"anchor": "rest-post-left", "fromS": 31.5, "toS": 40}, {"anchor": "rest-crossbar", "fromS": 31.5, "toS": 40}, {"anchor": "ball-ground", "fromS": 0, "toS": 40}],
     "phases": [{"id": "unbolt", "fromS": 0, "toS": 8, "title": t("Rear lugs unbolted", "Hintere Laschen gelöst")}, {"id": "lift", "fromS": 8, "toS": 14, "title": t("Lift out of the sleeves", "Aus den Hülsen heben")}, {"id": "tip", "fromS": 14, "toS": 32, "title": t("Tip forward and lay down", "Nach vorn kippen und ablegen")}, {"id": "lying", "fromS": 32, "toS": 40, "title": t("Lying face-down", "Umgelegt")}],
     "events": [{"t": 8, "kind": "contact-on", "title": t("Workers take the weight", "Mitarbeiter übernehmen das Gewicht"), "target": "hands-post-right"}, {"t": 9, "kind": "contact-off", "title": t("Spigots leave the sleeves", "Einsteckrohre verlassen die Hülsen"), "target": "sleeve-right"}, {"t": 31.5, "kind": "contact-on", "title": t("Posts and crossbar touch the turf", "Pfosten und Latte berühren den Rasen"), "target": "rest-crossbar"}],
     "tracks": []},
    {"id": "shot-on-crossbar", "title": t("Shot on the crossbar", "Schuss an die Latte"), "description": t("A ball kicked from 6 m flies at about 25 m/s, rattles the crossbar 1.6 m left of the centre, drops into the worn goal mouth and rolls into the back of the goal.", "Ein aus 6 m geschossener Ball fliegt mit etwa 25 m/s, knallt 1,6 m links der Mitte an die Latte, fällt in den abgetretenen Torraum und rollt hinten ins Tor."),
     "durationS": 1.6, "fps": 60, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "linear", "bones": {"ball-track": {"dx": KICK_DX}, "ball": {"dy": 0}}},
              {"t": 0.12, "pose": "in-service", "ease": "linear", "bones": {"ball-track": {"dx": 3.6}, "ball": {"dy": 1.25}}},
              {"t": 0.24, "pose": "in-use", "ease": "linear"},
              {"t": 0.5, "pose": "in-service", "ease": "linear", "bones": {"ball-track": {"dx": 1.15}, "ball": {"dy": 0.9}}},
              {"t": 0.72, "pose": "in-service", "ease": "linear", "bones": {"ball-track": {"dx": 0.75}, "ball": {"dy": 0.0}}},
              {"t": 0.95, "pose": "in-service", "ease": "linear", "bones": {"ball-track": {"dx": 0.45}, "ball": {"dy": 0.28}}},
              {"t": 1.15, "pose": "in-service", "ease": "linear", "bones": {"ball-track": {"dx": 0.25}, "ball": {"dy": 0.0}}},
              {"t": 1.6, "pose": "in-service", "ease": "out"}],
     "contacts": [{"anchor": "sleeve-right", "fromS": 0, "toS": 1.6}, {"anchor": "sleeve-left", "fromS": 0, "toS": 1.6}, {"anchor": "lug-right", "fromS": 0, "toS": 1.6}, {"anchor": "lug-left", "fromS": 0, "toS": 1.6}, {"anchor": "ball-ground", "fromS": 0, "toS": 0.02}, {"anchor": "ball-ground", "fromS": 0.72, "toS": 0.74}, {"anchor": "ball-ground", "fromS": 1.15, "toS": 1.6}],
     "phases": [{"id": "flight", "fromS": 0, "toS": 0.24, "title": t("Shot in flight", "Schuss im Flug"), "ballistic": True}, {"id": "rebound", "fromS": 0.24, "toS": 1.15, "title": t("Rebound and bounces", "Abpraller und Aufsetzer"), "ballistic": True}, {"id": "roll", "fromS": 1.15, "toS": 1.6, "title": t("Rolling into the goal", "Ins Tor rollen")}],
     "events": [{"t": 0.24, "kind": "load-on", "title": t("Ball hits the crossbar", "Ball trifft die Latte"), "target": "ball-strike"}, {"t": 0.25, "kind": "load-off", "title": t("Ball leaves the crossbar", "Ball verlässt die Latte"), "target": "ball-strike"}, {"t": 0.72, "kind": "contact-on", "title": t("Ball bounces in the goal mouth", "Ball setzt im Torraum auf"), "target": "ball-ground"}],
     "tracks": [{"target": "load:ball-strike", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 0.235, "v": 0}, {"t": 0.24, "v": 1}, {"t": 0.249, "v": 1}, {"t": 0.255, "v": 0}, {"t": 1.6, "v": 0}]}]},
    {"id": "tipping-check", "title": t("Annual tipping check", "Jährliche Kippprüfung"), "description": t("The inspector tensions the strap on the middle of the crossbar to 1100 N, holds it and releases it; the goal does not move in its sleeves.", "Der Prüfer spannt den Gurt an der Lattenmitte auf 1100 N, hält und löst ihn; das Tor bewegt sich in seinen Hülsen nicht."),
     "durationS": 12, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 4, "pose": "tipping-check", "ease": "smooth"}, {"t": 9, "pose": "tipping-check", "ease": "smooth"}, {"t": 12, "pose": "in-service", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 12} for a in SERVICE_CONTACTS],
     "phases": [{"id": "tension", "fromS": 0, "toS": 4, "title": t("Strap tensioned", "Gurt gespannt")}, {"id": "hold", "fromS": 4, "toS": 9, "title": t("Test force held", "Prüfkraft gehalten")}, {"id": "release", "fromS": 9, "toS": 12, "title": t("Released", "Gelöst")}],
     "events": [{"t": 4, "kind": "load-on", "title": t("1100 N reached", "1100 N erreicht"), "target": "inspection-pull"}, {"t": 12, "kind": "load-off", "title": t("Strap released", "Gurt gelöst"), "target": "inspection-pull"}],
     "tracks": [{"target": "load:inspection-pull", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 4, "v": 1}, {"t": 9, "v": 1}, {"t": 12, "v": 0}]}]},
]


def layer(i, en, de, kind, m, thick, k, rho, cp, ref="iso-10456-2007"):
    """🧶️ One region layer with its one-hour areal heat capacity rho*cp*min(d, sqrt(a*3600))."""
    a = k / (rho * cp)
    return {"id": i, "title": t(en, de), "kind": kind, "material": m, "thicknessM": thick, "resistanceM2KW": r(thick / k, 6), "heatCapacityJM2K": round(rho * cp * min(thick, math.sqrt(a * 3600))), "source": "literature", "reference": ref}


L_PAINT = layer("shs-wall", "SHS wall 4 mm under the powder coat", "Quadratrohrwand 4 mm unter der Pulverbeschichtung", "metal", W["paint"], TB, 50, RHO_S, 460)
L_GALV = layer("chs-wall", "CHS wall 2.9 mm, galvanised", "Rundrohrwand 2,9 mm, verzinkt", "metal", W["galv"], T48, 50, RHO_S, 460)
L_RUST = layer("rusted-wall", "Rusting SHS wall 4 mm", "Rostende Quadratrohrwand 4 mm", "metal", W["rust"], TB, 50, RHO_S, 460)
L_MESH = layer("wire", "Mesh wire 4 mm", "Gitterdraht 4 mm", "metal", W["mesh"], WIRE_D, 50, RHO_S, 460)
L_CONC = layer("foundation-concrete", "Concrete of the foundations", "Fundamentbeton", "masonry", W["conc"], FFD, 2.0, RHO_C, 1000)
L_SOIL = layer("compacted-soil", "Compacted soil", "Verdichteter Boden", "masonry", W["soil"], 0.1, 1.5, RHO_SOIL, 900)
L_BALL = layer("ball-cover", "Polyurethane cover and bladder", "Polyurethanhülle und Blase", "polymer", W["ball"], 0.004, 0.2, 1100, 1500, "ifab-laws-2024")


def region(i, en, de, m, area, lc, geom, svf, lay, src, ref, **kw):
    """🔥️ One passive region."""
    g = {"id": i, "title": t(en, de), "kind": "surface", "model": "passive", "material": m, "areaM2": r(area, 3)}
    g.update(kw)
    g.update({"characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": src, "reference": ref})
    return g


def shape(i, bone_, role, z, filled, tone, geom, key, **kw):
    """✏️ One 2D shape bound to a bone, region and material of the part key."""
    s = {"id": i, "bone": bone_, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    if key:
        s["region"] = REG[key]
        s["material"] = W[key]
    s.update(kw)
    return s


def g_rect(x0, y0, x1, y1):
    """⬛️ Rect geometry from two corners."""
    return {"kind": "rect", "x": r(min(x0, x1)), "y": r(min(y0, y1)), "w": r(abs(x1 - x0)), "h": r(abs(y1 - y0))}


def g_poly(points, closed=True):
    """🔺️ Poly geometry."""
    return {"kind": "poly", "points": [rv(p) for p in points], "closed": closed}


def g_cap(a, b, rad):
    """💊️ Capsule geometry."""
    return {"kind": "capsule", "x0": r(a[0]), "y0": r(a[1]), "x1": r(b[0]), "y1": r(b[1]), "r0": r(rad), "r1": r(rad)}


def g_ell(cx, cy, rx, ry):
    """⚪️ Ellipse geometry."""
    return {"kind": "ellipse", "cx": r(cx), "cy": r(cy), "rx": r(rx), "ry": r(ry)}


def g_path(segs):
    """〰️ Path geometry of open strokes (list of point lists)."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r(p[0])} {r(p[1])}" for p in seg) for seg in segs)}


def el(bone_, pts):
    """🧭️ World side-elevation points in a sagittal bone's (u, v) plane."""
    return [loc2(bone_, *p) for p in pts]


def el_rect(bone_, x0, y0, x1, y1):
    """⬛️ A world elevation rectangle as a poly in a bone frame."""
    return g_poly(el(bone_, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]))


def crossbar_front(n):
    """〰️ The front-view outline of the bent crossbar (world z, y)."""
    zs = [-ZO, -ZI] + [-ZI + 2 * ZI * k / n for k in range(1, n)] + [ZI, ZO]
    top = [(z, YC1 - sag(z)) for z in zs]
    bot = [(z, YC0 - sag(z)) for z in reversed(zs)]
    return top + bot


def side_mesh_profile(bone_):
    """🥅️ The side mesh trapezoid in a bone's (u, v)."""
    return el(bone_, SIDE_MESH)


def back_mesh_elev(bulged, ns=13, nz=25):
    """🥅️ The side-elevation hull of the back mesh (with its bulge) in the mesh bone frame."""
    v, _, _, _ = back_mesh(ns, nz, bulged)
    return el("mesh", hull([(p[0], p[1]) for p in v]))


def dish_profile_x(n):
    """🕳️ The hollow profile along x at z = 0 (side elevation)."""
    return [(PX0 + (PX1 - PX0) * k / n, -hollow(PX0 + (PX1 - PX0) * k / n, 0)) for k in range(n + 1)]


def dish_profile_z(n):
    """🕳️ The hollow profile across the goal mouth at x = HX (front view)."""
    return [(-PZ + 2 * PZ * k / n, -hollow(HX, -PZ + 2 * PZ * k / n)) for k in range(n + 1)]


def dish_contour(level, n=36):
    """🕳️ A plan contour of the hollow at a depth level (world x, z)."""
    q = 1 - math.sqrt(level / HD)
    a, b = HRX * math.sqrt(q), HRZ * math.sqrt(q)
    return [(HX + a * math.cos(2 * math.pi * k / n), b * math.sin(2 * math.pi * k / n)) for k in range(n)]


SIDES = (("right", -1), ("left", 1))


class Scale:
    """🏗️ Collects the 3D lists and the 2D views of one scale, part by part, so the two never drift apart."""

    def __init__(self, s):
        self.s = s
        self.body, self.th, self.st = [], [], []
        self.e, self.f, self.p = [], [], []

    def add(self, solid, key, section=None, member=None, part="member"):
        """➕️ A body solid with its thermal reference and, when it bears, its structure reference."""
        self.body.append(solid)
        ref = {"id": "th-" + solid["id"], "of": solid["id"], "region": REG[key]}
        if section:
            ref["sectionFaces"] = section
        self.th.append(ref)
        if member:
            self.st.append({"id": "st-" + solid["id"], "of": solid["id"], "member": member, "part": part})


def build(s):
    """🧱️ One scale: body, thermal, structure solids and the three drawings from the same world numbers."""
    k = Scale(s)
    fine = s <= 100
    S5 = ["+u", "-u", "+w", "-w", "-v"]
    S6 = S5 + ["+v"]
    for side, sg in SIDES:
        zc = sg * ZP
        if s <= 100:
            prof = [[zc - FF / 2, -FF / 2], [zc + FF / 2, -FF / 2], [zc + FF / 2, FF / 2], [zc - FF / 2, FF / 2]]
            hole = [[zc - SL_O / 2, -SL_O / 2], [zc + SL_O / 2, -SL_O / 2], [zc + SL_O / 2, SL_O / 2], [zc - SL_O / 2, SL_O / 2]]
            slb = SL_B if s == 50 else YSP - 0.015
            k.add(solid_extrude(f"footing-{side}", "ground", W["conc"], TONE["conc"], prof, "zx", slb, 0, holes=[hole]), "conc", S5)
            k.add(solid_box(f"footing-base-{side}", "ground", W["conc"], TONE["conc"], (-FF / 2, -FFD, zc - FF / 2), (FF / 2, slb, zc + FF / 2)), "conc", S6)
            hin = [[zc - SL_I / 2, -SL_I / 2], [zc + SL_I / 2, -SL_I / 2], [zc + SL_I / 2, SL_I / 2], [zc - SL_I / 2, SL_I / 2]]
            k.add(solid_extrude(f"sleeve-{side}", "ground", W["galv"], TONE["sleeve"], hole, "zx", YSP, 0, holes=[hin]), "conc", S5)
            k.add(solid_box(f"sleeve-plate-{side}", "ground", W["galv"], TONE["sleeve"], (-SL_O / 2, slb, zc - SL_O / 2), (SL_O / 2, YSP, zc + SL_O / 2)), "conc", S6)
        else:
            k.add(solid_box(f"footing-{side}", "ground", W["conc"], TONE["conc"], (-FF / 2, -FFD, zc - FF / 2), (FF / 2, 0, zc + FF / 2)), "conc", S5)
        k.add(solid_box(f"footing-rear-{side}", "ground", W["conc"], TONE["conc"], (XR - FR / 2, -FRD, zc - FR / 2), (XR + FR / 2, 0, zc + FR / 2)), "conc", S5)
    grid = {50: (21, 23), 100: (13, 15), 200: (9, 9)}
    if s in grid:
        pv, pt_, parea, _ = patch_mesh(*grid[s], True)
        k.add(polyhedron("goal-mouth", "goal-mouth", W["soil"], TONE["soil"], pv, pt_), "soil", S5)
    else:
        k.add(solid_box("goal-mouth", "goal-mouth", W["soil"], TONE["soil"], (PX0, PBOT, -PZ), (PX1, 0, PZ)), "soil", S5)
    k.add(solid_ball("ball", "ball", W["ball"], TONE["ball"], BALL, BALL_R), "ball")
    for side, sg in SIDES:
        b = "post-" + side
        lo, hi = (-B / 2, None, sg * ZP - B / 2), (B / 2, None, sg * ZP + B / 2)
        cr = {"cornerRadius": RO} if s == 50 else {}
        k.add(solid_box(f"spigot-{side}", b, W["galv"], TONE["galv"], (lo[0], YSP, lo[2]), (hi[0], 0, hi[2]), **cr), "galv", None, b, "joint")
        k.add(solid_box(f"rust-foot-{side}", b, W["rust"], TONE["rust"], (lo[0], 0, lo[2]), (hi[0], YRU, hi[2]), **cr), "rust", None, b)
        k.add(solid_box(f"post-{side}", b, W["paint"], TONE["paint"], (lo[0], YRU, lo[2]), (hi[0], YC0, hi[2]), **cr), "paint", None, b)
    if s <= 100:
        n = 20 if s == 50 else 10
        us = [0, ZO - ZI] + [ZO - ZI + 2 * ZI * j / n for j in range(1, n)] + [ZO + ZI, 2 * ZO]
        pts = [(u, -sag(u - ZO), 0) for u in us]
        k.add(loft_path("crossbar", "crossbar", W["paint"], TONE["paint"], pts, B / 2, B / 2, 8 if s == 50 else 6), "paint", None, "crossbar")
    else:
        k.add(solid_box("crossbar", "crossbar", W["paint"], TONE["paint"], (-B / 2, YC0, -ZO), (B / 2, YC1, ZO)), "paint", None, "crossbar")
    for side, sg in SIDES:
        b = "side-" + side
        zc = sg * ZP
        if s < 500:
            for a, c, nm in BOW_SEGS:
                k.add(solid_capsule(f"{nm}-{side}", b, W["galv"], TONE["galv"], (a[0], a[1], zc), (c[0], c[1], zc), R), "galv", None, b)
        else:
            pts = [loc(b, (p[0], p[1], zc)) for p in fillet(BOW, 0.12, 4)]
            k.add(loft_path(f"bow-{side}", b, W["galv"], TONE["galv"], pts, R, R, slender=True), "galv", None, b)
        lug = {50: (0, LUGT), 100: (LUGT - 0.015, LUGT), 200: (LUGT - 0.03, LUGT)}
        if s in lug:
            k.add(solid_box(f"lug-{side}", b, W["galv"], TONE["galv"], (XR - 0.05, lug[s][0], zc - 0.05), (XR + 0.05, lug[s][1], zc + 0.05)), "galv", None, b, "joint")
    for nm, (x, y) in (("rear-bottom", P1), ("rear-top", P2)):
        k.add(solid_capsule(nm, nm, W["galv"], TONE["galv"], (x, y, -ZP), (x, y, ZP), R, **({"slender": True} if s == 500 else {})), "galv", None, nm)
    for side, sg in SIDES:
        z0, z1 = sorted((sg * (ZP + R), sg * ZM))
        k.add(solid_extrude(f"side-mesh-{side}", "mesh", W["mesh"], TONE["mesh"], side_mesh_profile("mesh"), "xy", z0, z1, sheet=True), "mesh")
    k.add(solid_box("top-mesh", "mesh", W["mesh"], TONE["mesh"], (TOP_MESH_X[0], YC0, -ZM), (TOP_MESH_X[1], YC0 + MT, ZM), sheet=True), "mesh")
    if s <= 100:
        mv, mt, _, _ = back_mesh(*((13, 25) if s == 50 else (11, 21)), True)
        k.add(polyhedron("back-mesh", "mesh", W["mesh"], TONE["mesh"], mv, mt), "mesh")
    else:
        mv, mt, _, _ = back_mesh(3, 3, False)
        k.add(polyhedron("back-mesh", "mesh", W["mesh"], TONE["mesh"], mv, mt, sheet=True), "mesh")
    joint_r = {50: 0.03, 100: 0.03, 200: 0.035, 500: 0.06}[s]
    for side, sg in SIDES:
        zc = sg * ZP
        b = "side-" + side
        k.st.append(solid_box(f"j-corner-{side}", "crossbar", W["paint"], 0.3, (-B / 2 - 0.002, YC0 - (0.03 if s == 500 else 0.01), zc - B / 2 - 0.002), (B / 2 + 0.002, YC0 + (0.05 if s == 500 else 0.03), zc + B / 2 + 0.002), role="detail", member="crossbar", part="joint"))
        for nm, p in (("weld-bottom", P0), ("weld-top", P3)):
            k.st.append(solid_ball(f"j-{nm}-{side}", b, W["galv"], 0.3, (p[0], p[1], zc), joint_r, role="detail", member=b, part="joint"))
        for nm, p, m in (("rear-bottom", P1, "rear-bottom"), ("rear-top", P2, "rear-top")):
            k.st.append(solid_ball(f"j-{nm}-{side}", m, W["galv"], 0.3, (p[0], p[1], zc), joint_r, role="detail", member=m, part="joint"))
    mr = {50: 0.03, 100: 0.035, 200: 0.045, 500: 0.08}[s]
    for side, sg in SIDES:
        k.st.append(solid_ball(f"mass-post-{side}", "post-" + side, W["paint"], 0.3, (0, (YSP + YC0) / 2, sg * ZP), mr, role="detail", member="post-" + side, part="mass"))
        c = BOW_C
        k.st.append(solid_ball(f"mass-side-{side}", "side-" + side, W["galv"], 0.3, (c[0] + XR, c[1] + YB, sg * ZP), mr, role="detail", member="side-" + side, part="mass"))
    k.st.append(solid_ball("mass-crossbar", "crossbar", W["paint"], 0.3, (0, (YC0 + YC1) / 2 - SAG / 2, 0), mr, role="detail", member="crossbar", part="mass"))
    k.st.append(solid_ball("mass-rear-bottom", "rear-bottom", W["galv"], 0.3, (XR, YB, 0), mr, role="detail", member="rear-bottom", part="mass"))
    k.st.append(solid_ball("mass-rear-top", "rear-top", W["galv"], 0.3, (XT, YT, 0), mr, role="detail", member="rear-top", part="mass"))
    k.st.append(solid_ball("mass-mesh", "mesh", W["mesh"], 0.3, (MESH_C[0] + 0.04, MESH_C[1], 0), mr, role="detail", member="mesh", part="mass"))
    k.st.append(solid_ball("mass-ball", "ball", W["ball"], 0.3, BALL, min(mr, 0.05), role="detail", member="ball", part="mass"))
    draw(k)
    return k


def grid_side(step):
    """🥅️ Typified mesh wires of the side panel in the side elevation (world x, y), clipped to the panel."""
    xb = lambda y: back_point(S_B, R)[0] + (y - LUGT) * (back_point(S_T, R)[0] - back_point(S_B, R)[0]) / (YC0 - LUGT)
    segs = [[(XPB, y), (xb(y), y)] for y in [LUGT + step * i for i in range(1, int((YC0 - LUGT) / step) + 1)] if y < YC0 - 0.01]
    for i in range(1, int((XPB - back_point(S_B, R)[0]) / step) + 1):
        x = XPB - step * i
        yt = YC0 if x >= back_point(S_T, R)[0] else LUGT + (x - back_point(S_B, R)[0]) * (YC0 - LUGT) / (back_point(S_T, R)[0] - back_point(S_B, R)[0])
        if yt - LUGT > 0.02:
            segs.append([(x, LUGT), (x, yt)])
    return segs


def grid_back(step):
    """🥅️ Typified mesh wires of the back panel seen through the goal mouth (world z, y)."""
    segs = [[(-ZM, y), (ZM, y)] for y in [LUGT + step * i for i in range(1, int((YC0 - LUGT) / step) + 1)] if y < YC0 - 0.01]
    segs += [[(z, LUGT), (z, YC0)] for z in [-ZI + step * i for i in range(0, int(2 * ZI / step) + 1)]]
    return segs


def draw500(k):
    """✏️ The 1:500 drawings, redrawn for 75 mm: the frame as one outline, the bows and rear bars as true thin tubes, the mesh cage as one transparent field per view."""
    e, f, p = k.e, k.f, k.p
    e.append(shape("footing-front", "ground", "body", 1, True, TONE["conc"], g_rect(-FF / 2, -FFD, FF / 2, 0), "conc"))
    e.append(shape("footing-rear", "ground", "body", 1, True, TONE["conc"], g_rect(XR - FR / 2, -FRD, XR + FR / 2, 0), "conc"))
    e.append(shape("soil-sample", "goal-mouth", "body", 1.2, True, TONE["soil"], g_rect(PX0, PBOT, PX1, 0), "soil"))
    e.append(shape("mesh-cage", "mesh", "glazing", 2, True, TONE["mesh"], g_poly(el("mesh", [(XPB, LUGT), back_point(S_B, R + MT), back_point(S_T, R + MT), (TOP_MESH_X[0], YC0 + MT), (XPB, YC0 + MT)])), "mesh"))
    e.append(shape("bow", "side-right", "body", 3, False, TONE["galv"], g_poly(el("side-right", fillet(BOW, 0.12, 4)), False), "galv", penMm=0.1))
    e.append(shape("rust-foot", "post-right", "body", 4, True, TONE["rust"], el_rect("post-right", -B / 2, 0, B / 2, YRU), "rust"))
    e.append(shape("post-and-crossbar", "post-right", "body", 4, True, TONE["paint"], el_rect("post-right", -B / 2, YRU, B / 2, YC1), "paint"))
    e.append(shape("ball", "ball", "body", 6, True, TONE["ball"], g_ell(0, 0, BALL_R, BALL_R), "ball"))
    for side, sg in SIDES:
        zc = sg * ZP
        f.append(shape(f"footing-{side}", "ground", "body", 1, True, TONE["conc"], g_rect(zc - FF / 2, -FFD, zc + FF / 2, 0), "conc"))
        f.append(shape("rust-foot-" + side, "post-" + side, "body", 4, True, TONE["rust"], g_rect(zc - B / 2, 0, zc + B / 2, YRU), "rust"))
    f.append(shape("soil-sample", "goal-mouth", "body", 1.2, True, TONE["soil"], g_rect(-PZ, PBOT, PZ, 0), "soil"))
    f.append(shape("mesh-cage", "mesh", "glazing", 2, True, TONE["mesh"], g_rect(-ZM, LUGT, ZM, YC0), "mesh"))
    for nm, (x, y) in (("rear-bottom", P1), ("rear-top", P2)):
        f.append(shape(nm, nm, "body", 3, False, TONE["galv"], g_poly([(-ZP, y), (ZP, y)], False), "galv", penMm=0.1))
    f.append(shape("goal-frame", "crossbar", "body", 5, True, TONE["paint"], g_poly([(-ZO, YRU), (-ZO, YC1), (ZO, YC1), (ZO, YRU), (ZI, YRU), (ZI, YC0), (-ZI, YC0), (-ZI, YRU)]), "paint"))
    f.append(shape("ball", "ball", "body", 6, True, TONE["ball"], g_ell(BALL[2], BALL[1], BALL_R, BALL_R), "ball"))
    for side, sg in SIDES:
        zc = sg * ZP
        p.append(shape(f"footing-{side}", "ground", "body", 1, True, TONE["conc"], g_rect(-FF / 2, zc - FF / 2, FF / 2, zc + FF / 2), "conc"))
        p.append(shape(f"footing-rear-{side}", "ground", "body", 1, True, TONE["conc"], g_rect(XR - FR / 2, zc - FR / 2, XR + FR / 2, zc + FR / 2), "conc"))
    p.append(shape("soil-sample", "goal-mouth", "body", 1.2, True, TONE["soil"], g_rect(PX0, -PZ, PX1, PZ), "soil"))
    p.append(shape("ball", "ball", "body", 1.5, True, TONE["ball"], g_ell(BALL[0], BALL[2], BALL_R, BALL_R), "ball"))
    p.append(shape("mesh-cage", "mesh", "glazing", 2, True, TONE["mesh"], g_rect(BACK_MESH_PLAN_X[0], -ZM, TOP_MESH_X[1], ZM), "mesh"))
    for side, sg in SIDES:
        p.append(shape("bow-" + side, "side-" + side, "body", 3, False, TONE["galv"], g_poly([(XR, sg * ZP), (XPB, sg * ZP)], False), "galv", penMm=0.1))
    for nm, (x, y) in (("rear-bottom", P1), ("rear-top", P2)):
        p.append(shape(nm, nm, "body", 3.1, False, TONE["galv"], g_poly([(x, -ZP), (x, ZP)], False), "galv", penMm=0.1))
    p.append(shape("crossbar", "crossbar", "body", 5, True, TONE["paint"], g_rect(-B / 2, -ZO, B / 2, ZO), "paint"))


def draw(k):
    """✏️ The three drawings of one scale, from the same world numbers as its solids."""
    s = k.s
    if s == 500:
        return draw500(k)
    e, f, p = k.e, k.f, k.p
    hc = {"hatch": "concrete-dots"} if s <= 100 else {}
    hs = {"hatch": "soil-dots"} if s == 50 else {}
    e.append(shape("footing-front", "ground", "body", 1, True, TONE["conc"], g_rect(-FF / 2, -FFD, FF / 2, 0), "conc", **hc))
    e.append(shape("footing-rear", "ground", "body", 1, True, TONE["conc"], g_rect(XR - FR / 2, -FRD, XR + FR / 2, 0), "conc", **hc))
    e.append(shape("goal-mouth", "goal-mouth", "body", 1.2, True, TONE["soil"], g_rect(PX0, PBOT, PX1, 0), "soil", **hs))
    if s <= 200:
        e.append(shape("hollow-profile", "goal-mouth", "texture", 1.3, False, 0.3, g_poly(dish_profile_x(24 if s == 50 else 10), False), "soil"))
    e.append(shape("ball", "ball", "body", 6, True, TONE["ball"], g_ell(0, 0, BALL_R, BALL_R), "ball"))
    e.append(shape("back-mesh", "mesh", "glazing", 2, True, TONE["mesh"], g_poly(back_mesh_elev(s <= 100)), "mesh"))
    e.append(shape("side-mesh", "mesh", "glazing", 2.1, True, TONE["mesh"], g_poly(side_mesh_profile("mesh")), "mesh"))
    e.append(shape("top-mesh", "mesh", "glazing", 2.2, True, TONE["mesh"], el_rect("mesh", TOP_MESH_X[0], YC0, TOP_MESH_X[1], YC0 + MT), "mesh"))
    sb = "side-right"
    for a, c, nm in BOW_SEGS:
        e.append(shape(nm, sb, "body", 3, True, TONE["galv"], g_cap(loc2(sb, *a), loc2(sb, *c), R), "galv"))
    if s <= 200:
        e.append(shape("lug", sb, "body", 3.1, True, TONE["galv"], el_rect(sb, XR - 0.05, 0 if s == 50 else LUGT - (0.015 if s == 100 else 0.03), XR + 0.05, LUGT), "galv"))
    pb = "post-right"
    if s < 500:
        e.append(shape("spigot", pb, "detail", 1.5, False, 0.4, el_rect(pb, -B / 2, YSP, B / 2, 0), "galv"))
    if s <= 100:
        e.append(shape("sleeve", "ground", "detail", 1.4, False, 0.35, g_rect(-SL_O / 2, SL_B if s == 50 else YSP - 0.015, SL_O / 2, 0), "conc"))
    e.append(shape("rust-foot", pb, "body", 4, True, TONE["rust"], el_rect(pb, -B / 2, 0, B / 2, YRU), "rust"))
    e.append(shape("post", pb, "body", 4, True, TONE["paint"], el_rect(pb, -B / 2, YRU, B / 2, YC0), "paint"))
    e.append(shape("crossbar-end", pb, "body", 4.1, True, TONE["paint"], el_rect(pb, -B / 2, YC0, B / 2, YC1), "paint"))
    if s == 50:
        e.append(shape("paint-flakes", pb, "texture", 4.2, False, 0.4, g_path([el(pb, [(-0.02, 0.35), (0.01, 0.37)]), el(pb, [(-0.03, 0.62), (0.0, 0.6)]), el(pb, [(0.0, 1.05), (0.03, 1.07)]), el(pb, [(-0.025, 1.71), (0.015, 1.74)]), el(pb, [(-0.03, 2.03), (0.02, 2.05)])]), "paint"))
        e.append(shape("mesh-wires", "mesh", "texture", 2.15, False, 0.5, g_path([el("mesh", g) for g in grid_side(0.1)]), "mesh"))
        e.append(shape("mower-dents", sb, "texture", 3.2, False, 0.3, g_path([el(sb, [(-1.32, 0.05), (-1.24, 0.045)]), el(sb, [(-0.72, 0.05), (-0.63, 0.047)]), el(sb, [(-0.35, 0.05), (-0.28, 0.046)])]), "galv"))
        e.append(shape("ball-seams", "ball", "texture", 6.1, False, 0.2, g_path([[(-0.06, 0.04), (0.0, 0.075), (0.06, 0.04), (0.04, -0.035), (-0.04, -0.035), (-0.06, 0.04)], [(0.0, 0.075), (0.0, 0.11)], [(0.06, 0.04), (0.1, 0.05)], [(-0.06, 0.04), (-0.1, 0.05)], [(0.04, -0.035), (0.07, -0.085)], [(-0.04, -0.035), (-0.07, -0.085)]]), "ball"))
    for side, sg in SIDES:
        zc = sg * ZP
        f.append(shape(f"footing-{side}", "ground", "body", 1, True, TONE["conc"], g_rect(zc - FF / 2, -FFD, zc + FF / 2, 0), "conc", **hc))
        if s <= 100:
            f.append(shape(f"sleeve-{side}", "ground", "detail", 1.4, False, 0.35, g_rect(zc - SL_O / 2, SL_B if s == 50 else YSP - 0.015, zc + SL_O / 2, 0), "conc"))
    f.append(shape("goal-mouth", "goal-mouth", "body", 1.2, True, TONE["soil"], g_rect(-PZ, PBOT, PZ, 0), "soil", **hs))
    if s <= 200:
        f.append(shape("hollow-profile", "goal-mouth", "texture", 1.3, False, 0.3, g_poly(dish_profile_z(24 if s == 50 else 10), False), "soil"))
    f.append(shape("back-mesh", "mesh", "glazing", 2, True, TONE["mesh"], g_rect(-ZM, LUGT, ZM, YC0), "mesh"))
    for nm, (x, y) in (("rear-bottom", P1), ("rear-top", P2)):
        f.append(shape(nm, nm, "body", 3, True, TONE["galv"], g_cap((-ZP, y), (ZP, y), R), "galv"))
    for side, sg in SIDES:
        zc = sg * ZP
        b = "post-" + side
        if s < 500:
            f.append(shape("spigot-" + side, b, "detail", 1.5, False, 0.4, g_rect(zc - B / 2, YSP, zc + B / 2, 0), "galv"))
        f.append(shape("rust-foot-" + side, b, "body", 4, True, TONE["rust"], g_rect(zc - B / 2, 0, zc + B / 2, YRU), "rust"))
        f.append(shape("post-" + side, b, "body", 4, True, TONE["paint"], g_rect(zc - B / 2, YRU, zc + B / 2, YC0), "paint"))
        if s <= 200:
            f.append(shape("lug-" + side, "side-" + side, "body", 3.5, True, TONE["galv"], g_rect(zc - 0.05, 0 if s == 50 else LUGT - (0.015 if s == 100 else 0.03), zc + 0.05, LUGT), "galv"))
    f.append(shape("crossbar", "crossbar", "body", 5, True, TONE["paint"], g_poly(crossbar_front(20 if s == 50 else 10) if s <= 100 else [(-ZO, YC0), (ZO, YC0), (ZO, YC1), (-ZO, YC1)]), "paint"))
    f.append(shape("ball", "ball", "body", 6, True, TONE["ball"], g_ell(BALL[2], BALL[1], BALL_R, BALL_R), "ball"))
    if s == 50:
        f.append(shape("mesh-wires", "mesh", "texture", 2.05, False, 0.5, g_path(grid_back(0.1)), "mesh"))
        f.append(shape("graffiti-tag", "post-right", "texture", 4.3, False, 0.25, g_path([[(-ZP - 0.03, 0.9), (-ZP + 0.0, 0.98), (-ZP + 0.025, 0.88), (-ZP + 0.03, 1.02)], [(-ZP - 0.03, 1.08), (-ZP + 0.03, 1.12)]]), "paint"))
        f.append(shape("sticker", "post-left", "texture", 4.3, True, 0.7, g_rect(ZP - 0.03, 1.32, ZP + 0.03, 1.4), "paint"))
        f.append(shape("weld-rust", "crossbar", "detail", 5.1, True, 0.38, g_ell(-ZP, YC0 - 0.005, 0.04, 0.012), "rust"))
        f.append(shape("weld-rust-left", "crossbar", "detail", 5.1, True, 0.38, g_ell(ZP, YC0 - 0.005, 0.04, 0.012), "rust"))
        f.append(shape("strike-flakes", "crossbar", "texture", 5.2, False, 0.4, g_path([[(STRIKE_Z - 0.05, YC0 + 0.012 - sag(STRIKE_Z)), (STRIKE_Z + 0.06, YC0 + 0.016 - sag(STRIKE_Z))], [(STRIKE_Z - 0.02, YC0 + 0.03 - sag(STRIKE_Z)), (STRIKE_Z + 0.03, YC0 + 0.028 - sag(STRIKE_Z))]]), "paint"))
        f.append(shape("mesh-bulge", "mesh", "texture", 2.3, False, 0.35, g_poly([(BULGE[2] + 0.75 * BULGE[4] * math.cos(a / 18 * math.pi), back_point(BULGE[1], 0)[1] + 0.75 * BULGE[3] * DHAT[1] * math.sin(a / 18 * math.pi)) for a in range(36)]), "mesh"))
    for side, sg in SIDES:
        zc = sg * ZP
        p.append(shape(f"footing-{side}", "ground", "body", 1, True, TONE["conc"], g_rect(-FF / 2, zc - FF / 2, FF / 2, zc + FF / 2), "conc", **hc))
        p.append(shape(f"footing-rear-{side}", "ground", "body", 1, True, TONE["conc"], g_rect(XR - FR / 2, zc - FR / 2, XR + FR / 2, zc + FR / 2), "conc", **hc))
    p.append(shape("goal-mouth", "goal-mouth", "body", 1.2, True, TONE["soil"], g_rect(PX0, -PZ, PX1, PZ), "soil", **hs))
    if s <= 200:
        for lv in ((0.015, 0.045) if s <= 100 else (0.03,)):
            p.append(shape(f"hollow-{int(lv * 1000)}", "goal-mouth", "detail", 1.3, False, 0.3, g_poly(dish_contour(lv)), "soil"))
    p.append(shape("ball", "ball", "body", 1.5, True, TONE["ball"], g_ell(BALL[0], BALL[2], BALL_R, BALL_R), "ball"))
    p.append(shape("back-mesh", "mesh", "glazing", 2, True, TONE["mesh"], g_rect(BACK_MESH_PLAN_X[0], -ZM, BACK_MESH_PLAN_X[1], ZM), "mesh"))
    p.append(shape("top-mesh", "mesh", "glazing", 2.1, True, TONE["mesh"], g_rect(TOP_MESH_X[0], -ZM, TOP_MESH_X[1], ZM), "mesh"))
    for side, sg in SIDES:
        zc = sg * ZP
        b = "side-" + side
        p.append(shape("bow-" + side, b, "body", 3, True, TONE["galv"], g_cap((XR, zc), (XPB, zc), R), "galv"))
        if s <= 200:
            p.append(shape("lug-" + side, b, "body", 2.9, True, TONE["galv"], g_rect(XR - 0.05, zc - 0.05, XR + 0.05, zc + 0.05), "galv"))
    for nm, (x, y) in (("rear-bottom", P1), ("rear-top", P2)):
        p.append(shape(nm, nm, "body", 3.1, True, TONE["galv"], g_cap((x, -ZP), (x, ZP), R), "galv"))
    p.append(shape("crossbar", "crossbar", "body", 5, True, TONE["paint"], g_rect(-B / 2, -ZO, B / 2, ZO), "paint"))
    if s == 50:
        p.append(shape("crossbar-flakes", "crossbar", "texture", 5.1, False, 0.4, g_path([[(-0.02, STRIKE_Z - 0.06), (0.01, STRIKE_Z + 0.05)], [(-0.01, -0.4), (0.02, -0.3)], [(-0.025, 0.55), (0.0, 0.62)]]), "paint"))
        p.append(shape("lug-bolts", "side-right", "texture", 3.2, False, 0.25, g_path([[(XR - 0.012, sg * ZP), (XR + 0.012, sg * ZP)] for _, sg in SIDES]), "galv"))


def decisions(sil, ret, mer, omi, thin, hat, sty):
    """📝️ The eight decision texts of a representation."""
    return {"silhouette": sil, "retained": ret, "merged": mer, "omitted": omi, "thinElements": thin, "hatching": hat, "styles": sty,
            "overlays": t("Every shape carries one region (painted-frame, galvanised-frame, rust-feet, steel-mesh, footings, goal-mouth or football) and its material, so the thermal view paints the white posts and crossbar, the bare galvanised bows, the rusting feet, the mesh, the foundations, the worn soil and the ball apart.", "Jede Form trägt eine Region (painted-frame, galvanised-frame, rust-feet, steel-mesh, footings, goal-mouth oder football) und ihr Material, so dass die thermische Ansicht die weißen Pfosten und die Latte, die blanken verzinkten Bügel, die rostenden Füße, das Gitter, die Fundamente, den abgetretenen Boden und den Ball getrennt färbt.")}


HATCH = {
    50: [{"id": "concrete-dots", "title": t("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1},
         {"id": "soil-dots", "title": t("Soil stipple", "Bodenpunktierung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}],
    100: [{"id": "concrete-dots", "title": t("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.7, "penMm": 0.1}],
    200: [],
    500: [],
}
PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.2, "ground": 0.7},
        100: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.2, "ground": 0.5},
        200: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.2, "ground": 0.35},
        500: {"body": 0.2, "detail": None, "texture": None, "glazing": 0.2, "ground": 0.25}}
RANGE = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

REP_DEC = {
    50: decisions(
        t("Goal 103 x 42 mm on paper with its whole construction: in the side elevation the right post with its rusting foot and the hidden spigot in the sleeve and the 0.6 m foundation, the crossbar end, the net bow of ground bar, sloping rear leg and top bar, the lug on its rear anchor footing, the side and back mesh with the back panel bulged out 50 mm by shots, the soil sample with the 60 mm hollow as a hidden line and the ball in the back of the goal. Front: both posts, the crossbar sagging 30 mm at mid-span, the rear bars seen through the mouth against the back mesh, graffiti, sticker, rust at the welds and flaking paint where the shots land. Plan: crossbar, top and back mesh, bows, rear bars, lugs, the four foundations and the contours of the worn hollow.", "Tor 103 x 42 mm auf dem Papier mit seiner ganzen Konstruktion: in der Seitenansicht der rechte Pfosten mit rostendem Fuß, dem verdeckten Einsteckrohr in der Hülse und dem 0,6-m-Fundament, das Lattenende, der Netzbügel aus Bodenrohr, schrägem hinterem Bein und Oberrohr, die Lasche auf ihrem hinteren Ankerfundament, Seiten- und Rückgitter mit dem von Schüssen 50 mm ausgebeulten Rückfeld, die Bodenprobe mit der 60-mm-Mulde als verdeckte Linie und der Ball hinten im Tor. Vorderansicht: beide Pfosten, die in Feldmitte 30 mm durchhängende Latte, die hinteren Rohre durch die Toröffnung vor dem Rückgitter, Graffiti, Aufkleber, Rost an den Schweißnähten und abgeplatzte Farbe, wo die Schüsse treffen. Draufsicht: Latte, Ober- und Rückgitter, Bügel, hintere Rohre, Laschen, die vier Fundamente und die Höhenlinien der abgetretenen Mulde."),
        t("Every tube at its true section, the 30 mm set of the crossbar, the 50 mm bulge of the back mesh, the rusting 120 mm feet, the lugs, sleeves and foundations, the 60 mm hollow, the ball.", "Jedes Rohr in seinem wahren Querschnitt, die 30-mm-Durchbiegung der Latte, die 50-mm-Beule des Rückgitters, die rostenden 120-mm-Füße, Laschen, Hülsen und Fundamente, die 60-mm-Mulde, der Ball."),
        t("The two posts, the two bows and the two foundations coincide in the side elevation and are drawn once; the mesh is drawn as transparent glazing panels with its 50 mm grid as a cross hatch, not wire by wire.", "Die beiden Pfosten, die beiden Bügel und die beiden Fundamente decken sich in der Seitenansicht und werden einmal gezeichnet; das Gitter ist als transparente Verglasungsfläche mit seinem 50-mm-Raster als Kreuzschraffur gezeichnet, nicht Draht für Draht."),
        t("The mesh clips, the anchor bolts below their heads, the weld seams and the inside of the hollow sections.", "Die Gitterklammern, die Ankerschrauben unter ihren Köpfen, die Schweißnähte und das Innere der Hohlprofile."),
        t("The 8 mm mesh panels are filled glazing shapes; the hidden spigot and sleeve are outlines; flaking paint, mower dents, graffiti and ball seams are texture strokes.", "Die 8-mm-Gitterfelder sind gefüllte Verglasungsformen; verdecktes Einsteckrohr und Hülse sind Umrisse; abgeplatzte Farbe, Mähdellen, Graffiti und Ballnähte sind Texturstriche."),
        t("Concrete stipple every 0.8 mm, soil stipple every 0.6 mm, mesh cross hatch every 1.0 mm (50 mm).", "Betonpunktierung alle 0,8 mm, Bodenpunktierung alle 0,6 mm, Gitter-Kreuzschraffur alle 1,0 mm (50 mm)."),
        t("White coat 0.92, bare zinc 0.6, rust 0.32, mesh 0.84 transparent, concrete 0.66, soil 0.5, ball 0.88.", "Weiße Beschichtung 0,92, blankes Zink 0,6, Rost 0,32, Gitter 0,84 transparent, Beton 0,66, Boden 0,5, Ball 0,88.")),
    100: decisions(
        t("Goal 52 x 27 mm: posts with rust feet, the sagging crossbar, the bows, lugs, sleeves and foundations, the bulged back mesh, the hollow as hidden line and contours, the ball.", "Tor 52 x 27 mm: Pfosten mit Rostfüßen, die durchhängende Latte, Bügel, Laschen, Hülsen und Fundamente, das ausgebeulte Rückgitter, die Mulde als verdeckte Linie und Höhenlinien, der Ball."),
        t("The 30 mm sag (0.3 mm), the 50 mm bulge, the 120 mm rust feet, the 90 mm sleeves.", "Die 30-mm-Durchbiegung (0,3 mm), die 50-mm-Beule, die 120-mm-Rostfüße, die 90-mm-Hülsen."),
        t("Lug plate and bolt merged into one 15 mm seat; mesh grid typified to a 100 mm cross hatch.", "Laschenblech und Schraube zu einem 15-mm-Auflager zusammengefasst; Gitterraster zu einer 100-mm-Kreuzschraffur typisiert."),
        t("Corner radii of the tubes, graffiti, stickers, flaking paint and dents (below 0.15 mm or texture).", "Eckradien der Rohre, Graffiti, Aufkleber, abgeplatzte Farbe und Dellen (unter 0,15 mm oder Textur)."),
        t("Mesh panels as filled glazing; spigots and sleeves as outlines.", "Gitterfelder als gefüllte Verglasung; Einsteckrohre und Hülsen als Umrisse."),
        t("Concrete stipple every 0.7 mm, mesh cross hatch every 1.0 mm.", "Betonpunktierung alle 0,7 mm, Gitter-Kreuzschraffur alle 1,0 mm."),
        t("Tones as at 1:50.", "Töne wie bei 1:50.")),
    200: decisions(
        t("Goal 26 x 13.5 mm: posts, a straight crossbar, the bows and rear bars, the flat mesh panels as transparent fields, foundations, the soil sample with the hollow line and the ball.", "Tor 26 x 13,5 mm: Pfosten, eine gerade Latte, Bügel und hintere Rohre, die ebenen Gitterfelder als transparente Flächen, Fundamente, die Bodenprobe mit der Muldenlinie und der Ball."),
        t("The true 80 mm and 48 mm tubes, the rust feet, the lugs as 30 mm seats, the hollow outline.", "Die wahren 80-mm- und 48-mm-Rohre, die Rostfüße, die Laschen als 30-mm-Auflager, der Umriss der Mulde."),
        t("Sleeves merged into the foundations; the bulge and the 30 mm sag generalised away (one smallest feature).", "Hülsen in die Fundamente aufgegangen; Beule und 30-mm-Durchbiegung weggelassen (ein kleinstes Merkmal)."),
        t("Hatches, texture, sleeve outlines.", "Schraffuren, Textur, Hülsenumrisse."),
        t("Mesh as glazing tone only.", "Gitter nur als Verglasungston."),
        t("No hatches.", "Keine Schraffuren."),
        t("Tones as at 1:50, mesh lighter.", "Töne wie bei 1:50, Gitter heller.")),
    500: decisions(
        t("Goal 10.3 x 5.4 mm: the white frame, the bows and rear bars as thin true tubes, the mesh cage as transparent fields, foundations, a flat soil sample and the ball.", "Tor 10,3 x 5,4 mm: der weiße Rahmen, Bügel und hintere Rohre als dünne wahre Rohre, der Gitterkorb als transparente Flächen, Fundamente, eine ebene Bodenprobe und der Ball."),
        t("Goal mouth 5 x 2 m, depth 1.5 m, the foundations, the rust feet as a dark band.", "Toröffnung 5 x 2 m, Tiefe 1,5 m, die Fundamente, die Rostfüße als dunkles Band."),
        t("The bow of each side is one bent tube; the hollow is generalised into the flat soil sample (60 mm below the 75 mm feature).", "Der Bügel jeder Seite ist ein gebogenes Rohr; die Mulde ist in die ebene Bodenprobe verallgemeinert (60 mm unter dem 75-mm-Merkmal)."),
        t("Lugs, sleeves, hollow lines, texture.", "Laschen, Hülsen, Muldenlinien, Textur."),
        t("Tubes at their true 48 mm as filled capsules.", "Rohre in ihren wahren 48 mm als gefüllte Kapseln."),
        t("No hatches.", "Keine Schraffuren."),
        t("Tones as at 1:50.", "Töne wie bei 1:50.")),
}

VOL_DEC = {
    50: [t("Body, authored for 7.5 mm: posts as rounded 80 x 80 mm boxes (8 mm corner radius) split into the hidden 0.45 m spigot, the 120 mm rusting foot and the white shaft; the crossbar as one loft of 24 rounded-square sections (exponent 8) following its 30 mm permanent set; each net bow as three 48.3 mm capsules (ground bar, rear leg, top bar) welded to the back of the post, the rear bars as 5.08 m capsules, 8 mm galvanised lugs on the rear anchor footings; the mesh as declared 8 mm sheets on the sides and top and the back panel as a closed polyhedron of 13 x 25 stations bulged 50 mm by shots; footings with sleeve holes, square sleeves with bottom plates, the soil sample as a 21 x 23 heightfield with the 60 mm hollow, the ball.", "Körper, für 7,5 mm gebaut: Pfosten als gerundete 80 x 80 mm Quader (8 mm Eckradius), geteilt in das verdeckte 0,45-m-Einsteckrohr, den rostenden 120-mm-Fuß und den weißen Schaft; die Latte als ein Loft aus 24 gerundet-quadratischen Querschnitten (Exponent 8) entlang ihrer 30-mm-Durchbiegung; jeder Netzbügel als drei 48,3-mm-Kapseln (Bodenrohr, hinteres Bein, Oberrohr) hinten an den Pfosten geschweißt, die hinteren Rohre als 5,08-m-Kapseln, 8-mm-Laschen auf den hinteren Ankerfundamenten; das Gitter als deklarierte 8-mm-Häute an den Seiten und oben und das Rückfeld als geschlossenes Polyeder aus 13 x 25 Stationen, von Schüssen 50 mm ausgebeult; Fundamente mit Hülsenlöchern, Vierkanthülsen mit Bodenblechen, die Bodenprobe als 21 x 23-Höhenfeld mit der 60-mm-Mulde, der Ball."),
         t("Thermal: seven passive regions as the surfaces of their body solids; the faces of foundations, sleeves and soil sample against the surrounding ground are section faces; the mesh is an exchange-area region (wire surface, not panel surface). Structure: posts, crossbar, bows and rear bars are members along their bones, the spigots and lugs the joints by which the goal is clamped and bolted, welds at every frame corner are joints, masses sit at the centres; the supports are the pose contacts (sleeves and lugs in service, hands while lifting, the turf when laid down).", "Thermik: sieben passive Regionen als Oberflächen ihrer Körper; die Flächen von Fundamenten, Hülsen und Bodenprobe gegen das umgebende Erdreich sind Schnittflächen; das Gitter ist eine Austauschflächen-Region (Drahtoberfläche, nicht Feldfläche). Tragwerk: Pfosten, Latte, Bügel und hintere Rohre sind Glieder entlang ihrer Knochen, Einsteckrohre und Laschen die Gelenke, über die das Tor eingespannt und verschraubt ist, Schweißpunkte an jeder Rahmenecke sind Gelenke, Massen sitzen in den Schwerpunkten; die Auflager sind die Posenkontakte (Hülsen und Laschen im Einsatz, Hände beim Heben, der Rasen im umgelegten Zustand).")],
    100: [t("Body, authored anew for 15 mm: posts as sharp-edged boxes (the 8 mm radius is below the feature), the crossbar loft with 14 sections (exponent 6) still showing its 30 mm set, bows and rear bars unchanged in section, the lugs merged with their bolt into 15 mm seats, the back mesh polyhedron with 9 x 13 stations keeping the 50 mm bulge, the soil heightfield with 13 x 15 stations, sleeves and footings as at 1:50.", "Körper, neu für 15 mm gebaut: Pfosten als scharfkantige Quader (der 8-mm-Radius liegt unter dem Merkmal), der Lattenloft mit 14 Querschnitten (Exponent 6), der die 30-mm-Durchbiegung noch zeigt, Bügel und hintere Rohre im Querschnitt unverändert, die Laschen mit ihrer Schraube zu 15-mm-Auflagern zusammengefasst, das Rückgitter-Polyeder mit 9 x 13 Stationen und der 50-mm-Beule, das Bodenhöhenfeld mit 13 x 15 Stationen, Hülsen und Fundamente wie bei 1:50."),
          t("Thermal and structure as at 1:50, joints and masses resized to the scale.", "Thermik und Tragwerk wie bei 1:50, Gelenke und Massen auf den Maßstab angepasst.")],
    200: [t("Body, authored anew for 30 mm: the crossbar as a straight 80 mm box (its 30 mm set is one feature), the sleeves merged into plain foundation blocks with the spigots in them, the back mesh as a flat declared 8 mm sheet (the bulge left out), the soil sample as a 9 x 9 heightfield that keeps the 60 mm hollow, lugs as 30 mm seats.", "Körper, neu für 30 mm gebaut: die Latte als gerader 80-mm-Quader (ihre 30-mm-Durchbiegung ist ein Merkmal), die Hülsen in einfache Fundamentblöcke mit den Einsteckrohren darin aufgegangen, das Rückgitter als ebene deklarierte 8-mm-Haut (die Beule weggelassen), die Bodenprobe als 9 x 9-Höhenfeld, das die 60-mm-Mulde behält, Laschen als 30-mm-Auflager."),
          t("Thermal: one surface per region; structure as at 1:50 with 35 mm weld joints.", "Thermik: eine Fläche je Region; Tragwerk wie bei 1:50 mit 35-mm-Schweißgelenken.")],
    500: [t("Body, authored anew for 75 mm: each net bow as one bent tube (a loft with filleted corners, declared slender at its true 48.3 mm), the rear bars as slender capsules, the posts and the straight crossbar at their true 80 mm, the mesh as declared sheets, the soil sample as a flat 0.1 m slab (the 60 mm hollow is below the feature), foundation blocks, the ball; the lugs are left out.", "Körper, neu für 75 mm gebaut: jeder Netzbügel als ein gebogenes Rohr (ein Loft mit ausgerundeten Ecken, schlank deklariert in seinen wahren 48,3 mm), die hinteren Rohre als schlanke Kapseln, Pfosten und gerade Latte in ihren wahren 80 mm, das Gitter als deklarierte Häute, die Bodenprobe als ebene 0,1-m-Platte (die 60-mm-Mulde liegt unter dem Merkmal), Fundamentblöcke, der Ball; die Laschen sind weggelassen."),
          t("Thermal: one surface per region (only the total area is compared at this scale); structure: the members by reference, 60 mm weld joints, mass markers.", "Thermik: eine Fläche je Region (bei diesem Maßstab wird nur die Gesamtfläche verglichen); Tragwerk: die Glieder als Verweise, 60-mm-Schweißgelenke, Massenmarker.")],
}


def representation(s, k):
    """🖨️ The authored representation of one scale."""
    return {"status": "authored", "denominator": s, "validRange": {"minDenominator": RANGE[s][0], "maxDenominator": RANGE[s][1]}, "pens": PENS[s], "hatches": HATCH[s],
            "geometry": {"elevation": k.e, "front": k.f, "plan": k.p}, "decisions": REP_DEC[s]}


SCALES = {s: build(s) for s in (50, 100, 200, 500)}

_, _, PATCH_AREA, PATCH_VOL = patch_mesh(21, 23, True)
_, _, BACK_TOP_AREA, _ = back_mesh(13, 25, True)
PERIM_R = 4 * B - 8 * RO + 2 * math.pi * RO
AREA = {
    "painted-frame": 2 * (PERIM_R * (YC0 - YRU)) + 4 * B * 2 * ZO + 2 * B * B,
    "galvanised-frame": math.pi * D48 * (2 * BOW_LEN + 2 * 2 * ZP) + 2 * (0.01 + 4 * 0.1 * LUGT),
    "rust-feet": 2 * (PERIM_R * YRU + 2 * (B * B - (4 - math.pi) * RO ** 2)),
    "steel-mesh": MESH_PANEL_AREA * MESH_AREA_PER_M2,
    "footings": 2 * (FF * FF - SL_I ** 2) + 2 * (FR * FR - 0.01),
    "goal-mouth": PATCH_AREA,
    "football": 4 * math.pi * BALL_R ** 2,
}

THERMAL = {
    "environment": {"airTempC": 28, "meanRadiantTempC": 32, "skyTempC": 14, "groundTempC": 40, "relativeHumidity": 0.4, "windMS": 2.0, "windDirectionDeg": 225, "solarWM2": 820, "diffuseFraction": 0.15, "pressurePa": 101325},
    "referenceEnvironment": "summer-hot-sun",
    "passengers": [],
    "operating": [],
    "regions": [
        region("painted-frame", "White powder-coated posts and crossbar, chalked", "Weiß pulverbeschichtete Pfosten und Latte, kreidend", W["paint"], AREA["painted-frame"], B, "cylinder", 0.5, L_PAINT, "authored-estimate", "berdahl-bretz-1997"),
        region("galvanised-frame", "Bare galvanised net bows, rear bars and lugs", "Blank verzinkte Netzbügel, hintere Rohre und Laschen", W["galv"], AREA["galvanised-frame"], D48, "cylinder", 0.5, L_GALV, "authored-estimate", "incropera-2018"),
        region("rust-feet", "Rusting post feet where strimmers took the coat off", "Rostende Pfostenfüße, wo Freischneider die Beschichtung abgetragen haben", W["rust"], AREA["rust-feet"], B, "cylinder", 0.45, L_RUST, "literature", "incropera-2018"),
        region("steel-mesh", "Welded steel mesh 50 x 50 x 4 mm (wire surface)", "Stahl-Schweißgitter 50 x 50 x 4 mm (Drahtoberfläche)", W["mesh"], AREA["steel-mesh"], WIRE_D, "cylinder", 0.55, L_MESH, "authored-estimate", "en-iso-1461-2022", areaBasis="exchange"),
        region("footings", "Tops of the concrete foundations and sleeve rims", "Oberseiten der Betonfundamente und Hülsenränder", W["conc"], AREA["footings"], FF, "horizontal-plate-up", 0.95, L_CONC, "literature", "iso-10456-2007"),
        region("goal-mouth", "Bare compacted soil of the worn goal mouth", "Nackter verdichteter Boden des abgetretenen Torraums", W["soil"], AREA["goal-mouth"], 2.0, "horizontal-plate-up", 0.85, L_SOIL, "literature", "oke-1987"),
        region("football", "Worn size 5 football", "Abgenutzter Fußball Größe 5", W["ball"], AREA["football"], 2 * BALL_R, "sphere", 0.6, L_BALL, "authored-estimate", "ifab-laws-2024"),
    ],
    "emitters": [],
    "assumptions": [
        t("Passive surfaces only: absorbed sun, long-wave exchange with sky and ground, convection and heat capacity; the goal draws no power (0 W) and has no emitters.", "Nur passive Flächen: absorbierte Sonne, Langwellenaustausch mit Himmel und Boden, Konvektion und Wärmekapazität; das Tor nimmt keine Leistung auf (0 W) und hat keine Emitter."),
        t("Weathered surfaces differ from new ones: the white powder coat chalked and soiled absorbs about 0.35 of the sun instead of about 0.2 new (Berdahl and Bretz 1997); hot-dip zinc turns from a bright spangle (emissivity about 0.25) to a dull grey patina absorbing about 0.7 with an emissivity near 0.45; where strimmers and wet turf took the coat off the post feet, rust absorbs about 0.8 with an emissivity near 0.8 (Incropera, oxidised steel); the trodden goal mouth is dry bare soil absorbing about 0.78 instead of the 0.75 reflected by the grass around it being 0.2 (Oke 1987).", "Verwitterte Flächen unterscheiden sich von neuen: die kreidende, verschmutzte weiße Pulverbeschichtung absorbiert etwa 0,35 der Sonne statt etwa 0,2 im Neuzustand (Berdahl und Bretz 1997); Feuerverzinkung wird von glänzendem Zinkblumenmuster (Emissionsgrad etwa 0,25) zu matt grauer Patina mit etwa 0,7 Absorption und einem Emissionsgrad um 0,45; wo Freischneider und nasser Rasen die Beschichtung von den Pfostenfüßen genommen haben, absorbiert Rost etwa 0,8 bei einem Emissionsgrad um 0,8 (Incropera, oxidierter Stahl); der abgetretene Torraum ist trockener nackter Boden mit etwa 0,78 Absorption, während der Rasen ringsum etwa 0,2 reflektiert (Oke 1987)."),
        t("The thermal story of a summer afternoon: the thin-walled tubes (2.9 and 4 mm, 13-16 kJ/m2K) follow the sun within minutes, so the bare galvanised bows get noticeably hotter than the white posts, while the 60 mm hollow of bare soil is the hottest surface of the goal mouth and the foundations below the turf stay at ground temperature.", "Die thermische Geschichte eines Sommernachmittags: die dünnwandigen Rohre (2,9 und 4 mm, 13-16 kJ/m2K) folgen der Sonne binnen Minuten, so dass die blank verzinkten Bügel spürbar heißer werden als die weißen Pfosten, während die 60-mm-Mulde aus nacktem Boden die heißeste Fläche des Torraums ist und die Fundamente unter dem Rasen auf Bodentemperatur bleiben."),
        t("areaM2 is the outer surface of each region's 1:50 solids: tubes without their capped bores, the hidden spigots in the sleeves not counted, the foundations and the soil sample only by their tops (their faces against the surrounding ground are section faces); the mesh carries the surface of its wires (50 x 50 mm grid of 4 mm wire: 0.50 m2 per m2 of panel) as an exchange area. Sunlit fractions are scenario data of each pose.", "areaM2 ist die Außenfläche der 1:50-Körper jeder Region: Rohre ohne ihre verschlossenen Bohrungen, die verdeckten Einsteckrohre in den Hülsen nicht gezählt, Fundamente und Bodenprobe nur mit ihren Oberseiten (ihre Flächen gegen das umgebende Erdreich sind Schnittflächen); das Gitter trägt die Oberfläche seiner Drähte (50 x 50-mm-Raster aus 4-mm-Draht: 0,50 m2 je m2 Feld) als Austauschfläche. Besonnte Anteile sind Szenariodaten jeder Pose."),
    ],
}

STRUCTURE = {
    "gravityMS2": G,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        t(f"In service the welded goal ({M_GOAL:.0f} kg) is clamped by its two 0.45 m spigots in galvanised ground sleeves set in C20/25 point foundations 0.4 x 0.4 x 0.6 m (fixed supports at turf level) and bolted by its rear lugs to two anchor footings 0.3 x 0.3 x 0.4 m (pins); the foundations ({M_FOOT:.0f} kg of concrete) stay in the ground and are not part of the mass.", f"Im Einsatz ist das geschweißte Tor ({M_GOAL:.0f} kg) mit seinen zwei 0,45-m-Einsteckrohren in verzinkten Bodenhülsen in C20/25-Punktfundamenten 0,4 x 0,4 x 0,6 m eingespannt (feste Auflager auf Rasenhöhe) und mit seinen hinteren Laschen auf zwei Ankerfundamenten 0,3 x 0,3 x 0,4 m verschraubt (Gelenke); die Fundamente ({M_FOOT:.0f} kg Beton) bleiben im Boden und gehören nicht zur Masse."),
        t("Tipping check: EN 748 requires that a horizontal force of 1100 N at the middle of the crossbar does not overturn the goal; with the posts clamped in their sleeves the force goes into the sleeves as a moment of about 2.3 kNm, which the foundations carry into the soil. Crossbar hang: a 75 kg youth swinging at mid-span with a dynamic factor of 1.5 (about 1.1 kN) bends the 80 x 80 x 4 bar (W 27.8 cm3) to about 140 MPa between clamped corners, above the elastic range only with repeated jerks and cold-formed corners: hence the permanent 30 mm set.", "Kippprüfung: EN 748 verlangt, dass eine horizontale Kraft von 1100 N in der Lattenmitte das Tor nicht umwirft; mit den in ihren Hülsen eingespannten Pfosten geht die Kraft als Moment von etwa 2,3 kNm in die Hülsen, das die Fundamente in den Boden abtragen. Lattenhang: ein 75 kg schwerer Jugendlicher, der in Feldmitte mit dynamischem Faktor 1,5 schwingt (etwa 1,1 kN), beansprucht die Latte 80 x 80 x 4 (W 27,8 cm3) zwischen eingespannten Ecken mit etwa 140 MPa, über dem elastischen Bereich nur durch wiederholte Rucke und kaltverformte Ecken: daher die bleibende Durchbiegung von 30 mm."),
        t(f"Ball impact: a 430 g ball at 25 m/s with a restitution of 0.6 and a contact time of 9 ms gives a mean impact force of {STRIKE_N:.0f} N on the crossbar; it is a short pulse, treated here quasi-statically. Maintenance: four depot workers carry the goal by the posts at 1 m and by the rear corners while it is lifted out and tipped forward; laid face-down the posts and crossbar rest on the turf.", f"Ballaufprall: ein 430-g-Ball mit 25 m/s, Stoßzahl 0,6 und 9 ms Kontaktzeit ergibt eine mittlere Aufprallkraft von {STRIKE_N:.0f} N auf die Latte; als kurzer Stoß wird sie hier quasistatisch behandelt. Wartung: vier Bauhofmitarbeiter tragen das Tor an den Pfosten auf 1 m und an den hinteren Ecken, während es ausgehoben und nach vorn gekippt wird; umgelegt liegen Pfosten und Latte auf dem Rasen."),
        t("Members: posts and crossbar SHS 80 x 80 x 4 (11.7 cm2), net bows and rear bars CHS 48.3 x 2.9 (4.14 cm2), S235, welded; the mesh panels (50 x 50 x 4 mm, 3.95 kg/m2 plus 4 % for clips and edge wires) are non-bearing infill and appear only as a mass; masses are section times length times 7850 kg/m3.", "Glieder: Pfosten und Latte Quadratrohr 80 x 80 x 4 (11,7 cm2), Netzbügel und hintere Rohre Rundrohr 48,3 x 2,9 (4,14 cm2), S235, geschweißt; die Gitterfelder (50 x 50 x 4 mm, 3,95 kg/m2 plus 4 % für Klammern und Randdrähte) sind nichttragende Füllung und erscheinen nur als Masse; Massen sind Querschnitt mal Länge mal 7850 kg/m3."),
    ],
    "clinical": False,
}

TAGS = ["used", "park", "public-space", "sport", "play", "football", "goal", "bolzplatz", "steel", "galvanised", "welded-mesh", "ground-sleeve", "weathered"]

SUMMARY = t(
    f"A welded steel Bolzplatz goal of the municipal catalogue type, 5.0 x 2.0 m clear between white powder-coated 80 x 80 mm posts and crossbar, with galvanised 48.3 mm net bows 1.5 m deep and rigid 50 x 50 mm steel mesh instead of a net, {M_GOAL:.0f} kg, clamped in two ground sleeves in concrete point foundations and bolted at the rear. Years on a kick-about pitch bent the crossbar 30 mm from youths hanging on it, pushed the back mesh out 50 mm where the shots land, chalked and flaked the paint, rusted the post feet and trod the goal mouth into a bare 60 mm hollow; the depot lifts it out of its sleeves and lays it face-down, and the inspector pulls 1100 N on the crossbar to check it cannot tip.",
    f"Ein geschweißtes Bolzplatztor aus Stahl vom Typ der kommunalen Kataloge, 5,0 x 2,0 m lichte Weite zwischen weiß pulverbeschichteten Pfosten und Latte 80 x 80 mm, mit verzinkten 48,3-mm-Netzbügeln von 1,5 m Tiefe und starrem 50 x 50-mm-Stahlgitter statt Netz, {M_GOAL:.0f} kg, in zwei Bodenhülsen in Betonpunktfundamenten eingespannt und hinten verschraubt. Jahre auf dem Bolzplatz haben die Latte durch daran hängende Jugendliche um 30 mm durchgebogen, das Rückgitter dort, wo die Schüsse einschlagen, um 50 mm ausgebeult, die Farbe kreiden und abplatzen lassen, die Pfostenfüße rosten lassen und den Torraum zu einer kahlen 60-mm-Mulde getreten; der Bauhof hebt es aus seinen Hülsen und legt es nach vorn um, und der Prüfer zieht mit 1100 N an der Latte, um zu prüfen, dass es nicht kippen kann.")

PROVENANCE = {
    "author": "Semio staffage content (used park objects wave)",
    "method": "handcrafted",
    "created": "2026-11-22",
    "references": [{"id": i, "citation": c} for i, c in REFS],
    "notes": t(
        f"Catalogue type of German municipal welded steel Bolzplatz goals (no brand): clear opening 5.00 x 2.00 m (EN 748, DIN 18035-1), posts and crossbar SHS 80 x 80 x 4 S235 hot-dip galvanised and white powder-coated, net bows and rear bars CHS 48.3 x 2.9 galvanised, 1.5 m deep at the ground and 0.8 m at the top, welded mesh 50 x 50 x 4 mm, spigots 0.45 m in galvanised sleeves 90 x 90 mm in C20/25 foundations 0.4 x 0.4 x 0.6 m, rear lugs on footings 0.3 x 0.3 x 0.4 m. Masses section x length x 7850 kg/m3: posts 2 x {M_POST:.1f} kg, crossbar {M_CROSS:.1f} kg, bows 2 x {M_BOW:.1f} kg, rear bars 2 x {M_REAR:.1f} kg, mesh {MESH_PANEL_AREA:.2f} m2 x 3.95 kg/m2 x 1.04 = {M_MESH:.1f} kg; goal {M_GOAL:.1f} kg plus the 0.43 kg ball = {M_TOTAL:.1f} kg; the foundations ({M_FOOT:.0f} kg) are not included. Traces of use: crossbar set 30 mm, back mesh bulged 50 mm, post feet rusting over 120 mm, paint chalked and flaking at the strike spot, graffiti and a sticker, mower dents in the ground bars, a 60 mm hollow worn into the goal mouth. All numbers are literature or authored estimates; nothing is measured.",
        f"Katalogtyp deutscher kommunaler geschweißter Bolzplatztore aus Stahl (ohne Marke): lichte Öffnung 5,00 x 2,00 m (EN 748, DIN 18035-1), Pfosten und Latte Quadratrohr 80 x 80 x 4 S235 feuerverzinkt und weiß pulverbeschichtet, Netzbügel und hintere Rohre Rundrohr 48,3 x 2,9 verzinkt, unten 1,5 m und oben 0,8 m tief, Schweißgitter 50 x 50 x 4 mm, Einsteckrohre 0,45 m in verzinkten Hülsen 90 x 90 mm in C20/25-Fundamenten 0,4 x 0,4 x 0,6 m, hintere Laschen auf Fundamenten 0,3 x 0,3 x 0,4 m. Massen Querschnitt x Länge x 7850 kg/m3: Pfosten 2 x {M_POST:.1f} kg, Latte {M_CROSS:.1f} kg, Bügel 2 x {M_BOW:.1f} kg, hintere Rohre 2 x {M_REAR:.1f} kg, Gitter {MESH_PANEL_AREA:.2f} m2 x 3,95 kg/m2 x 1,04 = {M_MESH:.1f} kg; Tor {M_GOAL:.1f} kg plus der 0,43-kg-Ball = {M_TOTAL:.1f} kg; die Fundamente ({M_FOOT:.0f} kg) sind nicht enthalten. Gebrauchsspuren: Latte 30 mm durchgebogen, Rückgitter 50 mm ausgebeult, Pfostenfüße auf 120 mm rostend, Farbe am Trefferpunkt kreidend und abplatzend, Graffiti und ein Aufkleber, Mähdellen in den Bodenrohren, eine 60-mm-Mulde im Torraum. Alle Zahlen sind Literatur- oder Schätzwerte; nichts ist gemessen."),
}

XMIN = min(PX0, XR - FR / 2)
XMAX = PX1
ASSET = {
    "id": "used-football-goal",
    "category": "objects",
    "title": t("Used Mini Football Goal", "Gebrauchtes Bolzplatztor"),
    "summary": SUMMARY,
    "tags": TAGS,
    "provenance": PROVENANCE,
    "dimensions": {"heightM": r(YC1 + FFD), "widthM": r(XMAX - XMIN), "depthM": r(2 * (ZP + FF / 2)), "massKg": r(M_TOTAL, 1)},
    "materials": MATERIALS,
    "rig": RIG,
    "anchors": ANCHORS,
    "representations": {str(s): representation(s, SCALES[s]) for s in (50, 100, 200, 500)},
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": THERMAL,
    "structure": STRUCTURE,
    "volumes": {str(s): {"status": "authored", "denominator": s, "body": SCALES[s].body, "thermal": SCALES[s].th, "structure": SCALES[s].st, "decisions": VOL_DEC[s]} for s in (50, 100, 200, 500)},
    "presentation": {"defaultPose": "in-service", "defaultClip": "shot-on-crossbar", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

if __name__ == "__main__":
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, separators=(",", ":"))
        fh.write("\n")
    print(f"[DEBUG] A_SHS {A_SHS * 1e4:.3f} cm2 A_CHS {A_CHS * 1e4:.3f} cm2 I_CHS {I_CHS * 1e8:.2f} cm4 mesh {MESH_KG_M2:.3f} kg/m2")
    print(f"[DEBUG] masses post {M_POST:.2f} cross {M_CROSS:.2f} bow {M_BOW:.2f} rear {M_REAR:.2f} mesh {M_MESH:.2f} ({MESH_PANEL_AREA:.3f} m2) goal {M_GOAL:.2f} total {M_TOTAL:.2f} footings {M_FOOT:.0f}")
    print(f"[DEBUG] strike {STRIKE_N:.0f} N hang {HANG_N:.0f} N S_B {S_B:.4f} S_T {S_T:.4f}")
    for kk, v in AREA.items():
        print(f"[DEBUG] area {kk}: {v:.4f}")
    for s, k in SCALES.items():
        print(f"[DEBUG] 1:{s} body {len(k.body)} thermal {len(k.th)} structure {len(k.st)} 2D {len(k.e)}/{len(k.f)}/{len(k.p)}")
    print(f"[DEBUG] dims {ASSET['dimensions']}")
