"""🪜 Authors the staffage asset `reclaimed-steel-stair`: a dismantled industrial steel stair flight (two UPN 200 stringers, fourteen chequer-plate treads, one side-mounted handrail, flame-cut top cleats) lying on two bearers in a component yard, rigged with a four-leg chain sling, lifted and reinstalled for a storey of 2.80 m.

Every number is stated here; running the script writes `🔣️.json` next to it.

@see ../../brief.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")

STOREY = 2.80
RISERS = 15
TREADS = 14
RISE = STOREY / RISERS
GOING = 0.25
ALPHA = math.atan2(RISE, GOING)
CA, SA, TA = math.cos(ALPHA), math.sin(ALPHA), math.tan(ALPHA)
STEP = math.hypot(GOING, RISE)
PLATE = 0.010
X_TOE = (PLATE - RISE) / TA
X_TOP = TREADS * GOING

DEPTH, FLANGE_B, WEB_S, FLANGE_T, ROOT_R1, TOE_R2 = 0.200, 0.075, 0.0085, 0.0115, 0.0115, 0.006
W_IN = 0.400
W_OUT = W_IN + WEB_S
W_TIP = W_IN + FLANGE_B
ZC = W_IN + 0.0201
UPN_AREA, UPN_I, UPN_KG_M = 32.2e-4, 1910e-8, 25.3

U_TOP = (X_TOP - X_TOE) / CA
U0 = 0.1 / TA
LS = (U_TOP - 0.1 * TA) - U0

TREAD_D, TREAD_T, TREAD_F, CHEEK_H, CHEEK_T = 0.26, 0.005, 0.040, 0.065, 0.008
TREAD_200 = (0.251, 0.036)
PAN_W = 0.392
CHEQUER_KG_M2, PLATE5_KG_M2 = 42.4, 39.25

REST_Y = 0.10 + 0.2
LIFT_DY = 0.60
FLIGHT_V_APEX = 2.15

POST_R, KNEE_R = 0.0212, 0.01685
W_POST = W_TIP + POST_R
U_POSTS = [0.40, 2.30, 4.20]
POST_PITCH = U_POSTS[1] - U_POSTS[0]
V_POST_BOT = -0.18
V_RAIL = 1.0 * CA
V_KNEE = 0.5 * CA
TUBE42_KG_M, TUBE34_KG_M = 2.55, 1.99

CHAIN_R = 0.009
CHAIN_KG_M = 1.40
SHACKLE_R, SHACKLE_r = 0.020, 0.006
ML_R, ML_r = 0.045, 0.008
W_SB, W_ST = 0.425, 0.455

BEARER = (0.12, 0.10, 1.10)
TIMBER_RHO = 450
STEEL_RHO = 7850


def r6(x):
    """🔢 Rounds a number to six decimals for the document."""
    return round(x + 0.0, 6)


def P(points):
    """🧾 Rounds a list of points."""
    return [[r6(a) for a in p] for p in points]


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def i2f(X, Y):
    """🔄 Installed (horizontal X, vertical Y) to flight frame (u along the pitch line from the toe, v its normal)."""
    dx, dy = X - X_TOE, Y - PLATE
    return (dx * CA + dy * SA, -dx * SA + dy * CA)


def f2i(u, v):
    """🔁 Flight frame to installed coordinates."""
    return (X_TOE + u * CA - v * SA, PLATE + u * SA + v * CA)


def u_lo(v):
    """✂️ The u of the floor cut (installed horizontal through the toe) at the height v of the flight frame."""
    return -v / TA


def u_hi(v):
    """✂️ The u of the plumb cut at the landing face at the height v."""
    return U_TOP + v * TA


def fillet(a, b, c, r, n=4):
    """⭕ Replaces the polygon corner b (between a and c) by an arc of radius r with n segments."""
    d1 = (b[0] - a[0], b[1] - a[1])
    l1 = math.hypot(*d1)
    d1 = (d1[0] / l1, d1[1] / l1)
    d2 = (c[0] - b[0], c[1] - b[1])
    l2 = math.hypot(*d2)
    d2 = (d2[0] / l2, d2[1] / l2)
    cosang = -(d1[0] * d2[0] + d1[1] * d2[1])
    theta = math.acos(max(-1, min(1, cosang)))
    t = r / math.tan(theta / 2)
    t1 = (b[0] - d1[0] * t, b[1] - d1[1] * t)
    t2 = (b[0] + d2[0] * t, b[1] + d2[1] * t)
    bis = (-d1[0] + d2[0], -d1[1] + d2[1])
    lb = math.hypot(*bis)
    bis = (bis[0] / lb, bis[1] / lb)
    dist = r / math.sin(theta / 2)
    ctr = (b[0] + bis[0] * dist, b[1] + bis[1] * dist)
    a1 = math.atan2(t1[1] - ctr[1], t1[0] - ctr[0])
    a2 = math.atan2(t2[1] - ctr[1], t2[0] - ctr[0])
    da = a2 - a1
    while da > math.pi:
        da -= 2 * math.pi
    while da < -math.pi:
        da += 2 * math.pi
    return [(ctr[0] + r * math.cos(a1 + da * k / n), ctr[1] + r * math.sin(a1 + da * k / n)) for k in range(n + 1)]


def flange_section(top):
    """📐 The (v, |w|) polygon of one UPN 200 flange with its 8 % inner taper, toe radius r2, root radius r1 and the web band it is welded to (DIN 1026-1)."""
    half = (FLANGE_B - WEB_S) / 2
    v_root = -(FLANGE_T + 0.08 * half)
    v_toe = -(FLANGE_T - 0.08 * half)
    sgn = 1 if top else -1
    base = 0.0 if top else -DEPTH
    def at(v, w):
        return (base + sgn * v, w)
    outer_web = at(0, W_IN)
    outer_tip = at(0, W_TIP)
    toe = at(v_toe, W_TIP)
    root = at(v_root, W_OUT)
    below = at(v_root - 0.05, W_OUT)
    toe_arc = fillet(outer_tip, toe, root, TOE_R2)
    root_arc = fillet(toe, root, below, ROOT_R1)
    end = root_arc[-1]
    pts = [outer_web, outer_tip] + toe_arc + root_arc + [(end[0], W_IN)]
    return pts


def area2(poly):
    """🧮 Twice the signed area of a 2D polygon."""
    return sum(poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1] for i in range(len(poly)))


def ear_clip(poly):
    """✂️ Triangulates a simple counter-clockwise polygon by ear clipping; returns index triples."""
    idx = list(range(len(poly)))
    tris = []
    def inside(p, a, b, c):
        def s(p1, p2, p3):
            return (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])
        d1, d2, d3 = s(p, a, b), s(p, b, c), s(p, c, a)
        return not ((d1 < -1e-15 or d2 < -1e-15 or d3 < -1e-15) and (d1 > 1e-15 or d2 > 1e-15 or d3 > 1e-15))
    guard = 0
    while len(idx) > 3 and guard < 10000:
        guard += 1
        for k in range(len(idx)):
            i0, i1, i2 = idx[k - 1], idx[k], idx[(k + 1) % len(idx)]
            a, b, c = poly[i0], poly[i1], poly[i2]
            cross = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
            if cross <= 1e-14:
                continue
            if any(inside(poly[j], a, b, c) for j in idx if j not in (i0, i1, i2)):
                continue
            tris.append((i0, i1, i2))
            idx.pop(k)
            break
    tris.append(tuple(idx))
    return tris


def prism_polyhedron(section, lo, hi, to_local):
    """🧊 A closed outward polyhedron of a (v, w) section swept along u between the planes u = lo(v) and u = hi(v), mapped to bone-local coordinates by `to_local`."""
    sec = list(section)
    if area2(sec) < 0:
        sec.reverse()
    n = len(sec)
    verts = [to_local(lo(v), v, w) for v, w in sec] + [to_local(hi(v), v, w) for v, w in sec]
    tris = []
    for i in range(n):
        j = (i + 1) % n
        tris += [(i, j, n + j), (i, n + j, n + i)]
    for a, b, c in ear_clip(sec):
        tris.append((a, c, b))
        tris.append((n + a, n + b, n + c))
    def vol(tr):
        s = 0
        for a, b, c in tr:
            p, q, r = verts[a], verts[b], verts[c]
            s += p[0] * (q[1] * r[2] - q[2] * r[1]) - p[1] * (q[0] * r[2] - q[2] * r[0]) + p[2] * (q[0] * r[1] - q[1] * r[0])
        return s / 6
    if vol(tris) < 0:
        tris = [(a, c, b) for a, b, c in tris]
    return [list(map(r6, v)) for v in verts], [list(t) for t in tris], abs(vol(tris))


def circle(cx, cy, r, n=12, start=0.0):
    """⚪ A polygon approximating a circle."""
    return [(cx + r * math.cos(start + 2 * math.pi * k / n), cy + r * math.sin(start + 2 * math.pi * k / n)) for k in range(n)]


def hexagon(cx, cy, af):
    """⬡ A hexagon of width across flats `af`."""
    rr = af / math.sqrt(3)
    return [(cx + rr * math.cos(math.pi / 6 + k * math.pi / 3), cy + rr * math.sin(math.pi / 6 + k * math.pi / 3)) for k in range(6)]


def rot_strip(p0, d, length, half):
    """📏 A rectangle along the unit direction d from p0 over `length`, `half` wide on each side."""
    nx, ny = -d[1], d[0]
    p1 = (p0[0] + d[0] * length, p0[1] + d[1] * length)
    return [(p0[0] + nx * half, p0[1] + ny * half), (p0[0] - nx * half, p0[1] - ny * half), (p1[0] - nx * half, p1[1] - ny * half), (p1[0] + nx * half, p1[1] + ny * half)]


def ccw(poly):
    """🔃 The polygon in counter-clockwise order."""
    return poly if area2(poly) > 0 else list(reversed(poly))


def rect(x0, y0, x1, y1):
    """▭ An axis-aligned rectangle polygon."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def inst_poly(points):
    """🔄 Converts a polygon of installed coordinates to the flight frame."""
    return [i2f(x, y) for x, y in points]


def shift(points, du, dv):
    """➡️ Moves a polygon by (du, dv)."""
    return [(a + du, b + dv) for a, b in points]


def tread_section(k, block=False, depth=TREAD_D, thick=TREAD_F):
    """🪜 The flight-frame section of tread k (1 based): a folded chequer-plate pan, or a block `depth` by `thick` from the nosing."""
    xn, yn = (k - 1) * GOING, k * RISE
    if block:
        return ccw(inst_poly(rect(xn, yn - thick, xn + depth, yn)))
    t = TREAD_T
    return ccw(inst_poly([(xn, yn), (xn + TREAD_D, yn), (xn + TREAD_D, yn - TREAD_F), (xn + TREAD_D - t, yn - TREAD_F), (xn + TREAD_D - t, yn - t), (xn + t, yn - t), (xn + t, yn - TREAD_F), (xn, yn - TREAD_F)]))


def cheek_section(k):
    """🧱 The flight-frame outline of the welded side cheek of tread k."""
    xn, yn = (k - 1) * GOING, k * RISE
    return ccw(inst_poly(rect(xn, yn - CHEEK_H, xn + TREAD_D, yn)))


def bolt_points(k):
    """🔩 The flight-frame centres of the two M12 bolts of tread k."""
    xn, yn = (k - 1) * GOING, k * RISE
    return [i2f(xn + 0.05, yn - 0.03), i2f(xn + 0.21, yn - 0.03)]


def bar_outline(floor):
    """📐 The flight-frame outline of a stringer bar whose floor cut lies at installed height `floor`: top edge on the pitch line, plumb cut at the landing face, bottom edge 200 mm below the top edge."""
    toe = i2f((floor - RISE) / TA, floor)
    heel = i2f((floor - RISE + DEPTH / CA) / TA, floor)
    return ccw([toe, heel, (u_hi(-DEPTH), -DEPTH), (U_TOP, 0.0)])


def plate_profile(thick):
    """🦶 The flight-frame side outline of a base plate `thick` thick on the floor under the floor cut, 60 mm longer than the cut at both ends (thicker than 10 mm it reaches into the stringer end)."""
    xh = X_TOE + DEPTH / SA
    return ccw(inst_poly(rect(X_TOE - 0.06, 0.0, xh + 0.06, thick)))


def cleat_profile(leg):
    """🪝 The flight-frame side outline of the remaining L 100 x 100 x 10 top cleat with its flame-cut horizontal leg stub, legs `leg` thick."""
    x0 = X_TOP
    return ccw(inst_poly([(x0, STOREY - 0.09), (x0 + leg, STOREY - 0.09), (x0 + leg, STOREY), (x0 + leg + 0.025, STOREY), (x0 + leg + 0.025, STOREY + leg), (x0, STOREY + leg)]))


def ring_points(R, r, n=16):
    """⭕ A closed revolve profile [axial, radial]: a circle of radius r at radial distance R."""
    pts = [[r6(r * math.cos(2 * math.pi * k / n)), r6(R + r * math.sin(2 * math.pi * k / n))] for k in range(n)]
    return pts + [pts[0]]


def quat_z(theta):
    """🧭 A unit quaternion turning by theta about the local w axis."""
    return [0.0, 0.0, r6(math.sin(theta / 2)), r6(math.cos(theta / 2))]


def stringer_local(side):
    """🗺️ The map from flight-frame (u, v, w) to the local frame of a stringer bone; side +1 left, -1 right."""
    return lambda u, v, w: (u - U0, v + 0.1, w - side * ZC)


def mat3(m, x):
    """✖️ Applies a 3x3 matrix to a vector."""
    return [sum(m[i][j] * x[j] for j in range(3)) for i in range(3)]


def matmul(a, b):
    """✖️ Multiplies two 3x3 matrices."""
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def rot(angle, abduction=0.0, twist=0.0):
    """🔄 R = Rw(angle) Rv(abduction) Ru(twist) in degrees, as in the product's rig."""
    a, b, t = map(math.radians, (angle, abduction, twist))
    rw = [[math.cos(a), -math.sin(a), 0], [math.sin(a), math.cos(a), 0], [0, 0, 1]]
    rv = [[math.cos(b), 0, -math.sin(b)], [0, 1, 0], [math.sin(b), 0, math.cos(b)]]
    ru = [[1, 0, 0], [0, math.cos(t), math.sin(t)], [0, -math.sin(t), math.cos(t)]]
    return matmul(matmul(rw, rv), ru)


def solve(bones, offsets):
    """🦴 World origin and rotation of every bone for the given offsets."""
    byid = {b["id"]: b for b in bones}
    out = {}
    def frame(bid):
        if bid in out:
            return out[bid]
        b = byid[bid]
        off = offsets.get(bid, {})
        dx, dy = 0.0, 0.0
        if b["joint"]["type"] == "prismatic":
            ax = math.radians(b["joint"].get("axisDeg", 0))
            s = off.get("dx", 0) * math.cos(ax) + off.get("dy", 0) * math.sin(ax)
            dx, dy = s * math.cos(ax), s * math.sin(ax)
        ang = off.get("angleDeg", 0) if b["joint"]["type"] == "revolute" else 0
        abd = off.get("abductionDeg", 0) if b["joint"]["type"] == "revolute" else 0
        if b["parent"] is None:
            po, pr = [0, 0, 0], [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
        else:
            po, pr = frame(b["parent"])
        t = mat3(pr, [b["x"] + dx, b["y"] + dy, b.get("z", 0)])
        o = [po[i] + t[i] for i in range(3)]
        r = matmul(matmul(pr, rot(b["angleDeg"], b.get("abductionDeg", 0), b.get("twistDeg", 0))), rot(ang, abd))
        out[bid] = (o, r)
        return out[bid]
    for b in bones:
        frame(b["id"])
    return out


def world(frames, bone, p):
    """🌍 A bone-local point in world coordinates."""
    o, r = frames[bone]
    q = mat3(r, p)
    return [o[i] + q[i] for i in range(3)]


tread_one = tread_section(1)
cheek_one = cheek_section(1)
bolts_one = bolt_points(1)
web_poly = [(0.0, 0.0), (u_lo(-DEPTH), -DEPTH), (u_hi(-DEPTH), -DEPTH), (U_TOP, 0.0)]

tread_mass = TREAD_D * 2 * PAN_W * CHEQUER_KG_M2 + 2 * TREAD_F * 2 * PAN_W * PLATE5_KG_M2 + 2 * TREAD_D * CHEEK_H * CHEEK_T * STEEL_RHO + 4 * 0.058
base_mass = (DEPTH / SA + 0.12) * 0.10 * PLATE * STEEL_RHO - 2 * math.pi * 0.009 ** 2 * PLATE * STEEL_RHO
cleat_mass = (0.10 * 0.010 + 0.025 * 0.010) * 0.10 * STEEL_RHO
channel_mass = UPN_KG_M * (U_TOP + (u_hi(-DEPTH) - u_lo(-DEPTH))) / 2
stringer_mass = channel_mass + TREADS * tread_mass / 2 + base_mass + cleat_mass

post_len = (V_RAIL - V_POST_BOT) / CA
rail_u0 = U_POSTS[0] + V_RAIL * TA - 0.10
rail_u1 = U_POSTS[-1] + V_RAIL * TA + 0.10
knee_u0 = U_POSTS[0] + V_KNEE * TA
knee_u1 = U_POSTS[-1] + V_KNEE * TA
shoe_len = W_POST - W_OUT
shoe_mass = 0.05 * 0.008 * shoe_len * STEEL_RHO
m_posts = 3 * post_len * TUBE42_KG_M
m_rail = (rail_u1 - rail_u0) * TUBE42_KG_M
m_knee = (knee_u1 - knee_u0) * TUBE34_KG_M
handrail_mass = m_posts + m_rail + m_knee + 6 * shoe_mass + 6 * 0.058
post_mid = [(u + (V_POST_BOT + V_RAIL) / 2 * TA, (V_POST_BOT + V_RAIL) / 2) for u in U_POSTS]
hcu = (m_posts * sum(p[0] for p in post_mid) / 3 + m_rail * (rail_u0 + rail_u1) / 2 + m_knee * (knee_u0 + knee_u1) / 2) / (m_posts + m_rail + m_knee)
hcv = (m_posts * post_mid[0][1] + m_rail * V_RAIL + m_knee * V_KNEE) / (m_posts + m_rail + m_knee)
H_COM = (hcu, V_KNEE)
H_ORIGIN = (U_POSTS[1] + V_POST_BOT * TA, V_POST_BOT)

bearer_mass = BEARER[0] * BEARER[1] * BEARER[2] * TIMBER_RHO
ml_mass = 1.60
shackle_mass = 0.30

com_u = U0 + LS / 2
U_SB, U_ST = round(com_u - 1.15, 3), round(com_u + 1.15, 3)
XB = [round(com_u - 1.38, 3), round(com_u + 1.38, 3)]

legs_def = {
    "sling-lb": (1, U_SB + SHACKLE_R, W_SB, 0.0),
    "sling-lt": (1, U_ST - SHACKLE_R, W_ST, 180.0),
    "sling-rb": (-1, U_SB + SHACKLE_R, -W_SB, 0.0),
    "sling-rt": (-1, U_ST - SHACKLE_R, -W_ST, 180.0),
}
leg_origin = {k: (u, CHAIN_R, w) for k, (s, u, w, a0) in legs_def.items()}

apex = [com_u, FLIGHT_V_APEX, 0.0]
for _ in range(8):
    lens = {k: math.dist(apex, leg_origin[k]) for k in legs_def}
    mass_pts = [(stringer_mass, (U0 + LS / 2, -0.1, ZC)), (stringer_mass, (U0 + LS / 2, -0.1, -ZC)), (handrail_mass, (H_COM[0], H_COM[1], W_POST))]
    for k in legs_def:
        o = leg_origin[k]
        mass_pts.append((CHAIN_KG_M * lens[k], tuple((o[i] + apex[i]) / 2 for i in range(3))))
    mass_pts.append((ml_mass, tuple(apex)))
    mass_pts.append((4 * shackle_mass, (com_u, 0.0, 0.0)))
    mt = sum(m for m, _ in mass_pts)
    apex = [sum(m * p[0] for m, p in mass_pts) / mt, FLIGHT_V_APEX, sum(m * p[2] for m, p in mass_pts) / mt]
lens = {k: math.dist(apex, leg_origin[k]) for k in legs_def}
leg_mass = {k: CHAIN_KG_M * lens[k] for k in legs_def}
leg_off = {}
for k, (s, u, w, a0) in legs_def.items():
    o = leg_origin[k]
    d = [(apex[i] - o[i]) / lens[k] for i in range(3)]
    if a0 == 0:
        th = math.degrees(math.atan2(d[1], d[0]))
    else:
        th = math.degrees(math.atan2(-d[1], -d[0]))
    ph = math.degrees(math.asin(d[2]))
    leg_off[k] = {"angleDeg": round(th, 4), "abductionDeg": round(ph, 4)}

MASS_TOTAL = 2 * stringer_mass + handrail_mass + sum(leg_mass.values()) + ml_mass + 4 * shackle_mass + 2 * bearer_mass

walker_a0 = i2f(0.13, RISE)
walker_travel = (TREADS - 1) * STEP

bones = [
    {"id": "yard", "parent": None, "title": T("Yard floor (frame of the asset)", "Lagerplatzboden (Bezugssystem des Objekts)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "bearer-low", "parent": "yard", "title": T("Bearer at the foot end", "Unterleger am Fußende"), "x": r6(XB[0] - 0.06), "y": 0.05, "angleDeg": 0, "lengthM": 0.12, "joint": {"type": "fixed"}, "mass": {"kg": r6(bearer_mass), "comX": 0.06, "comY": 0, "distribution": "point", "source": "literature", "reference": "en-338"}},
    {"id": "bearer-high", "parent": "yard", "title": T("Bearer at the head end", "Unterleger am Kopfende"), "x": r6(XB[1] - 0.06), "y": 0.05, "angleDeg": 0, "lengthM": 0.12, "joint": {"type": "fixed"}, "mass": {"kg": r6(bearer_mass), "comX": 0.06, "comY": 0, "distribution": "point", "source": "literature", "reference": "en-338"}},
    {"id": "lift", "parent": "yard", "title": T("Lift travel of the flight (crane hook height)", "Hubweg des Laufs (Kranhakenhöhe)"), "x": 0, "y": REST_Y, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.4, "maxM": 1.0}},
    {"id": "flight", "parent": "lift", "title": T("Flight, pivot at the toe of the stringers", "Lauf, Drehpunkt an der Wangenspitze"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 40}},
]
for side, sid, name_en, name_de in [(1, "stringer-l", "Left stringer UPN 200 (centroid axis)", "Linke Wange UPN 200 (Schwerachse)"), (-1, "stringer-r", "Right stringer UPN 200 (centroid axis)", "Rechte Wange UPN 200 (Schwerachse)")]:
    bones.append({"id": sid, "parent": "flight", "title": T(name_en, name_de), "x": r6(U0), "y": -0.1, "z": r6(side * ZC), "angleDeg": 0, "lengthM": r6(LS), "joint": {"type": "fixed"}, "mass": {"kg": r6(stringer_mass), "comX": r6(LS / 2), "comY": 0, "distribution": "uniform", "source": "literature", "reference": "din-1026-1"}, "section": {"areaM2": UPN_AREA, "secondMomentM4": UPN_I, "extremeFibreM": 0.1, "material": "steel-stringer", "title": T("UPN 200 (DIN 1026-1)", "UPN 200 (DIN 1026-1)")}})
bones.append({"id": "handrail", "parent": "stringer-l", "title": T("Handrail with three side-mounted posts and knee rail", "Handlauf mit drei seitlich angeschraubten Pfosten und Knieleiste"), "x": r6(H_ORIGIN[0] - U0), "y": r6(H_ORIGIN[1] + 0.1), "z": r6(W_POST - ZC), "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}, "mass": {"kg": r6(handrail_mass), "comX": r6(H_COM[0] - H_ORIGIN[0]), "comY": r6(H_COM[1] - H_ORIGIN[1]), "distribution": "point", "source": "literature", "reference": "en-10219-2"}})
for k, (s, u, w, a0) in legs_def.items():
    parent = "stringer-l" if s > 0 else "stringer-r"
    where = {"sling-lb": ("left, foot-end shackle", "links, Schäkel am Fußende"), "sling-lt": ("left, head-end shackle", "links, Schäkel am Kopfende"), "sling-rb": ("right, foot-end shackle", "rechts, Schäkel am Fußende"), "sling-rt": ("right, head-end shackle", "rechts, Schäkel am Kopfende")}[k]
    bones.append({"id": k, "parent": parent, "title": T(f"Chain sling leg, {where[0]}", f"Kettenstrang, {where[1]}"), "x": r6(u - U0), "y": r6(CHAIN_R + 0.1), "z": r6(w - s * ZC), "angleDeg": a0, "lengthM": r6(lens[k]), "joint": {"type": "revolute", "minDeg": -90, "maxDeg": 90}, "mass": {"kg": r6(leg_mass[k]), "comX": r6(lens[k] / 2), "comY": 0, "distribution": "uniform", "source": "literature", "reference": "en-818-4"}})
bones.append({"id": "master-link", "parent": "sling-lb", "title": T("Master link of the four-leg sling", "Aufhängering des Vierstranggehänges"), "x": r6(lens["sling-lb"]), "y": 0, "angleDeg": 0, "lengthM": r6(2 * (ML_R + ML_r)), "joint": {"type": "fixed"}, "mass": {"kg": ml_mass, "comX": r6(ML_R + ML_r), "comY": 0, "distribution": "point", "source": "literature", "reference": "en-1677-4"}})
for side, wid in [(1, "walker-l"), (-1, "walker-r")]:
    bones.append({"id": wid, "parent": "stringer-l" if side > 0 else "stringer-r", "title": T(f"Walker load carrier on the {'left' if side > 0 else 'right'} stringer (massless)", f"Lastträger Person auf der {'linken' if side > 0 else 'rechten'} Wange (masselos)"), "x": r6(walker_a0[0] - U0), "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": r6(walker_travel)}})

for b in bones:
    if b["id"] in ("stringer-l", "stringer-r"):
        b["mass"]["kg"] = r6(stringer_mass + 2 * shackle_mass)

MASS_TOTAL = sum(b["mass"]["kg"] for b in bones if "mass" in b)

xh_plate = X_TOE + DEPTH / SA
plate_mid = ((X_TOE - 0.06 + xh_plate + 0.06) / 2, 0.0)
cleat_bear = (X_TOP + PLATE + 0.0125, STOREY)
anchors = []
for side, sid, tag in [(1, "stringer-l", "l"), (-1, "stringer-r", "r")]:
    for bi, (bid, xb) in enumerate(zip(["low", "high"], XB)):
        anchors.append({"id": f"bearer-{bid}-{tag}", "bone": sid, "x": r6(xb - U0), "y": -0.1, "kind": "contact", "title": T(f"{'Left' if side > 0 else 'Right'} stringer on the {'foot-end' if bid == 'low' else 'head-end'} bearer", f"{'Linke' if side > 0 else 'Rechte'} Wange auf dem Unterleger am {'Fußende' if bid == 'low' else 'Kopfende'}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.12, "footprintWidthM": 0.075, "frictionCoefficient": 0.5})
    for which, u, w in [("low", U_SB, W_SB), ("high", U_ST, W_ST)]:
        anchors.append({"id": f"shackle-{which}-{tag}", "bone": sid, "x": r6(u - U0), "y": 0.1, "z": r6(side * w - side * ZC), "kind": "support", "title": T(f"Sling shackle, {'left' if side > 0 else 'right'} {'foot' if which == 'low' else 'head'} end", f"Schäkel des Gehänges, {'links' if side > 0 else 'rechts'} am {'Fußende' if which == 'low' else 'Kopfende'}"), "support": "pin"})
    fu, fv = i2f(*plate_mid)
    anchors.append({"id": f"foot-{tag}", "bone": sid, "x": r6(fu - U0), "y": r6(fv + 0.1), "z": r6(side * 0.4375 - side * ZC), "kind": "support", "title": T(f"{'Left' if side > 0 else 'Right'} base plate on the floor (pin)", f"{'Linke' if side > 0 else 'Rechte'} Fußplatte auf dem Boden (Festlager)"), "support": "pin"})
    cu, cv = i2f(*cleat_bear)
    anchors.append({"id": f"landing-{tag}", "bone": sid, "x": r6(cu - U0), "y": r6(cv + 0.1), "z": r6(side * 0.4375 - side * ZC), "kind": "support", "title": T(f"{'Left' if side > 0 else 'Right'} cut cleat on the new landing angle (roller)", f"{'Linke' if side > 0 else 'Rechte'} abgetrennte Lasche auf dem neuen Podestwinkel (Rollenlager)"), "support": "roller", "normalDeg": 90})
for wid in ["walker-l", "walker-r"]:
    anchors.append({"id": f"{wid}-point", "bone": wid, "x": 0, "y": r6(walker_a0[1] + 0.1), "kind": "load", "title": T(f"Half of a person on the {'left' if wid.endswith('l') else 'right'} stringer", f"Halbe Person auf der {'linken' if wid.endswith('l') else 'rechten'} Wange")})

SIDES = [(1, "l", "stringer-l"), (-1, "r", "stringer-r")]
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.030, 500: 0.075}


def span(a, b):
    """↔️ An ordered pair."""
    return (min(a, b), max(a, b))


def lat(side, w_abs):
    """↔️ The flight-frame lateral position of a distance |w| on one side."""
    return side * w_abs


def to_str(side, pts):
    """🗺️ Flight-frame (u, v) points in the local frame of a stringer bone."""
    return [(u - U0, v + 0.1) for u, v in pts]


def to_hr(pts):
    """🗺️ Flight-frame (u, v) points in the local frame of the handrail bone."""
    return [(u - H_ORIGIN[0], v - H_ORIGIN[1]) for u, v in pts]




def solid(sid, bone, kind, material, role, tone, **kw):
    """🧱 One volume solid."""
    d = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    d.update(kw)
    return d


def xy(sid, bone, material, role, tone, profile, frm, to, **kw):
    """🧱 An extrude of a (u, v) profile along w."""
    a, b = span(frm, to)
    return solid(sid, bone, "extrude", material, role, tone, profile=P(ccw(profile)), plane="xy", **{"from": r6(a), "to": r6(b)}, **kw)


def tube(sid, bone, material, role, tone, r, p0, d, length, **kw):
    """🧪 A solid cylinder (open two-point revolve) from p0 along the unit direction d."""
    return solid(sid, bone, "revolve", material, role, tone, profile=[[0.0, r6(r)], [r6(length), r6(r)]], axis=[r6(d[0]), r6(d[1]), 0.0], origin=[r6(p0[0]), r6(p0[1]), r6(p0[2])], **kw)


def ring(sid, bone, material, role, tone, R, r, axis, origin, **kw):
    """⭕ A torus (closed revolve profile off the axis)."""
    return solid(sid, bone, "revolve", material, role, tone, profile=ring_points(R, r), axis=axis, origin=[r6(x) for x in origin], **kw)


def repeat14():
    """🔁 The fourteen tread positions along the pitch line."""
    return {"count": TREADS, "translation": [r6(STEP), 0.0, 0.0]}


def stringer_parts(scale):
    """🧱 The body solids of both stringers at one scale with their regions."""
    out = []
    for side, t, sid in SIDES:
        to_local = stringer_local(side)
        if scale in (50, 100):
            web_hi = W_OUT if scale == 50 else 0.415
            holes = []
            if scale == 50 and side < 0:
                for uc in U_POSTS:
                    for v in (-0.06, -0.14):
                        holes.append(P(list(reversed(ccw(to_str(side, circle(uc + v * TA, v, 0.007, 12)))))))
            kw = {"holes": holes} if holes else {}
            out.append((xy(f"web-{t}", sid, "steel-stringer", "body", 0.42, to_str(side, web_poly), lat(side, W_IN) - side * ZC, lat(side, web_hi) - side * ZC, **kw), "stringer-steel"))
            if scale == 50:
                for top, name in [(True, "top"), (False, "bottom")]:
                    sec = [(v, lat(side, w)) for v, w in flange_section(top)]
                    verts, tris, _ = prism_polyhedron(sec, u_lo, u_hi, lambda u, v, w, tl=to_local: tl(u, v, w))
                    out.append((solid(f"flange-{name}-{t}", sid, "polyhedron", "steel-stringer", "body", 0.45, vertices=verts, triangles=tris), "stringer-steel"))
            else:
                for vt, vb, name in [(0.0, -0.015, "top"), (-DEPTH + 0.015, -DEPTH, "bottom")]:
                    strip = [(u_lo(vt), vt), (u_hi(vt), vt), (u_hi(vb), vb), (u_lo(vb), vb)]
                    out.append((xy(f"flange-{name}-{t}", sid, "steel-stringer", "body", 0.45, to_str(side, strip), lat(side, 0.415) - side * ZC, lat(side, W_TIP) - side * ZC), "stringer-steel"))
            thick = 0.010 if scale == 50 else 0.015
            lo_x, hi_x = -0.06, (DEPTH / SA) + 0.06
            c0, c1 = span(lat(side, 0.3875) - side * ZC, lat(side, 0.4875) - side * ZC)
            cm = lat(side, 0.4375) - side * ZC
            prof = [(c0, lo_x), (c1, lo_x), (c1, hi_x), (c0, hi_x)]
            holes = [P(circle(cm, lo_x + 0.03, 0.009, 12)), P(circle(cm, hi_x - 0.03, 0.009, 12))]
            out.append((solid(f"base-plate-{t}", sid, "extrude", "steel-stringer", "body", 0.30, profile=P(prof), holes=holes, plane="zx", **{"from": r6(-PLATE), "to": r6(thick - PLATE)}, transform={"rotation": quat_z(-ALPHA), "translation": [r6(-U0), 0.1, 0.0]}), "stringer-steel"))
            out.append((xy(f"cleat-{t}", sid, "steel-stringer", "body", 0.30, to_str(side, cleat_profile(0.010 if scale == 50 else 0.015)), lat(side, 0.3875) - side * ZC, lat(side, 0.4875) - side * ZC), "stringer-steel"))
            if scale == 50:
                for bi, (bu, bv) in enumerate(bolts_one):
                    hx = to_str(side, hexagon(bu, bv, 0.018))
                    out.append((xy(f"bolt-head-{bi + 1}-{t}", sid, "steel-stringer", "detail", 0.25, hx, lat(side, W_OUT) - side * ZC, lat(side, W_OUT + 0.0075) - side * ZC, repeat=repeat14()), "stringer-steel"))
                    out.append((xy(f"nut-{bi + 1}-{t}", sid, "steel-stringer", "detail", 0.25, hx, lat(side, PAN_W - 0.0075) - side * ZC, lat(side, PAN_W) - side * ZC, repeat=repeat14()), "stringer-steel"))
                for which, u, w in [("low", U_SB, W_SB), ("high", U_ST, W_ST)]:
                    o = to_local(u, SHACKLE_R + SHACKLE_r, side * w)
                    out.append((ring(f"shackle-{which}-{t}", sid, "chain-steel", "detail", 0.2, SHACKLE_R, SHACKLE_r, [0.0, 0.0, 1.0], o), "sling-chain"))
        elif scale == 200:
            out.append((xy(f"stringer-{t}", sid, "steel-stringer", "body", 0.42, to_str(side, bar_outline(0.0)), lat(side, W_IN) - side * ZC, lat(side, W_TIP) - side * ZC), "stringer-steel"))
    return out


def tread_parts(scale):
    """🪜 The body solids of the treads at one scale."""
    if scale == 50:
        out = [(xy("tread", "flight", "chequer-steel", "body", 0.62, tread_one, -PAN_W, PAN_W, repeat=repeat14()), "tread-plate")]
        for side, t, sid in SIDES:
            out.append((xy(f"cheek-{t}", "flight", "chequer-steel", "body", 0.55, cheek_one, lat(side, PAN_W), lat(side, W_IN), repeat=repeat14()), "tread-plate"))
        return out
    if scale == 100:
        return [(xy("tread", "flight", "chequer-steel", "body", 0.62, tread_section(1, block=True), -W_IN, W_IN, repeat=repeat14()), "tread-plate")]
    if scale == 200:
        return [(xy("tread", "flight", "chequer-steel", "body", 0.62, tread_section(1, True, *TREAD_200), -W_IN, W_IN, repeat=repeat14()), "tread-plate")]
    return [(xy("flight", "flight", "steel-stringer", "body", 0.5, bar_outline(0.0), -W_TIP, W_TIP), None)]


def handrail_parts(scale):
    """🦺 The body solids of the handrail at one scale."""
    slender = {"slender": True} if scale == 500 else {}
    hb = lambda u, v, w=W_POST: (u - H_ORIGIN[0], v - H_ORIGIN[1], w - W_POST)
    d_post = (SA, CA)
    out = [
        (tube("posts", "handrail", "handrail-yellow", "body", 0.55, POST_R, hb(U_POSTS[0] + V_POST_BOT * TA, V_POST_BOT), d_post, post_len, repeat={"count": 3, "translation": [r6(POST_PITCH), 0.0, 0.0]}, **slender), "handrail-tube"),
        (tube("rail", "handrail", "handrail-yellow", "body", 0.55, POST_R, hb(rail_u0, V_RAIL), (1.0, 0.0), rail_u1 - rail_u0, **({"slender": True} if scale in (200, 500) else {})), "handrail-tube"),
        (tube("knee-rail", "handrail", "handrail-yellow", "body", 0.55, KNEE_R, hb(knee_u0, V_KNEE), (1.0, 0.0), knee_u1 - knee_u0, **({"slender": True} if scale in (200, 500) else {})), "handrail-tube"),
    ]
    if scale == 50:
        for v, name in [(-0.06, "upper"), (-0.14, "lower")]:
            c = hb(U_POSTS[0] + v * TA, v, (W_OUT + W_POST) / 2)
            out.append((solid(f"shoe-{name}", "handrail", "box", "handrail-yellow", "detail", 0.45, size=[0.05, 0.008, r6(shoe_len)], transform={"translation": [r6(c[0]), r6(c[1]), r6(c[2])]}, repeat={"count": 3, "translation": [r6(POST_PITCH), 0.0, 0.0]}), "handrail-tube"))
    return out


def sling_parts(scale):
    """⛓️ The body solids of the four chain legs and the master link at one scale."""
    out = []
    for k in legs_def:
        L = lens[k] + (2 * (ML_R + ML_r) if (k == "sling-lb" and scale in (200, 500)) else 0.0)
        out.append((solid(k, k, "capsule", "chain-steel", "detail", 0.2, p0=[0.0, 0.0, 0.0], p1=[r6(L), 0.0, 0.0], r0=CHAIN_R, r1=CHAIN_R, slender=True), "sling-chain"))
    if scale in (50, 100):
        out.append((ring("master-link", "master-link", "chain-steel", "detail", 0.2, ML_R, ML_r, [0.0, 1.0, 0.0], (ML_R + ML_r, 0.0, 0.0)), "sling-chain"))
    return out


def bearer_parts(scale):
    """🪵 The body solids of the two bearers at one scale."""
    out = []
    for bid in ["bearer-low", "bearer-high"]:
        extra = {"cornerRadius": 0.006} if scale == 50 else {}
        out.append((solid(bid, bid, "box", "bearer-softwood", "body", 0.68, size=list(BEARER), transform={"translation": [0.06, 0.0, 0.0]}, **extra), "bearer-timber"))
    return out


def body_parts(scale):
    """🧊 Every body solid of one scale with its region."""
    return stringer_parts(scale) + tread_parts(scale) + handrail_parts(scale) + sling_parts(scale) + bearer_parts(scale)


MEMBER_OF = {"web": "stringer", "flange": "stringer", "stringer": "stringer", "base-plate": "stringer", "cleat": "stringer", "bolt-head": "stringer", "nut": "stringer", "shackle": "shackle", "tread": "flight", "cheek": "flight", "posts": "handrail", "rail": "handrail", "knee-rail": "handrail", "shoe": "handrail", "master-link": "master-link", "bearer-low": "bearer-low", "bearer-high": "bearer-high"}
JOINTS = {"base-plate", "cleat", "bolt-head", "nut", "shackle", "cheek", "shoe"}


def structure_label(s):
    """🏗️ The member bone and part of one body solid in the structure list."""
    sid = s["id"]
    if sid.startswith("sling-"):
        return sid, "member"
    base = sid.rsplit("-", 1)[0] if sid.endswith(("-l", "-r")) else sid
    for key in sorted(MEMBER_OF, key=len, reverse=True):
        if base.startswith(key):
            member = MEMBER_OF[key]
            if member == "stringer":
                member = "stringer-l" if sid.endswith("-l") else "stringer-r"
            if member == "shackle":
                side = sid[-1]
                member = f"sling-{side}b" if "-low-" in sid else f"sling-{side}t"
            return member, ("joint" if key in JOINTS else "member")
    raise KeyError(sid)


def volume_of(scale):
    """🔧 The authored volume of one scale."""
    parts = body_parts(scale)
    body = [s for s, _ in parts]
    thermal = [{"id": f"th-{s['id']}", "of": s["id"], "region": reg} for s, reg in parts if reg is not None]
    structure = []
    for s, reg in parts:
        if s["id"] == "flight":
            continue
        member, part = structure_label(s)
        entry = {"id": f"st-{s['id']}", "of": s["id"], "member": member, "part": part}
        if s["id"] == "sling-lb" and scale in (200, 500):
            entry["members"] = ["master-link"]
        structure.append(entry)
    if scale == 500:
        for side, t, sid in SIDES:
            bar = xy(f"stringer-bar-{t}", sid, "steel-stringer", "body", 0.42, to_str(side, bar_outline(0.0)), lat(side, W_IN) - side * ZC, lat(side, W_TIP + 0.0001) - side * ZC)
            thermal.append(dict(bar, id=f"th-stringer-{t}", region="stringer-steel"))
            structure.append(dict(bar, id=f"st-stringer-{t}", member=sid, part="member"))
        band = xy("tread-band", "flight", "chequer-steel", "body", 0.6, bar_outline(0.0), -W_IN, W_IN)
        thermal.append(dict(band, id="th-tread-band", region="tread-plate"))
        structure.append(dict(band, id="st-tread-band", member="flight", part="member"))
    radius = {50: 0.015, 100: 0.02, 200: 0.03, 500: 0.045}[scale]
    structure.append(solid("mass-handrail", "handrail", "ellipsoid", "handrail-yellow", "detail", 0.1, radii=[r6(radius)] * 3, transform={"translation": [r6(H_COM[0] - H_ORIGIN[0]), r6(H_COM[1] - H_ORIGIN[1]), 0.0]}, member="handrail", part="mass"))
    return body, thermal, structure, parts


BODY = {s: volume_of(s) for s in (50, 100, 200, 500)}


PENS = {
    50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5},
    100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
    200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25},
    500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18},
}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = {
    50: [{"id": "end-grain", "title": T("Timber end grain", "Hirnholz"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.6, "penMm": 0.1}, {"id": "chequer", "title": T("Teardrop chequer plate", "Tränenblech"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}],
    100: [{"id": "end-grain", "title": T("Timber end grain", "Hirnholz"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}, {"id": "chequer", "title": T("Teardrop chequer plate", "Tränenblech"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.45, "penMm": 0.1}],
    200: [],
    500: [],
}


def hull(points):
    """🧺 The convex hull of a set of 2D points, counter-clockwise."""
    pts = sorted(set(points))
    def half(seq):
        out = []
        for p in seq:
            while len(out) >= 2 and (out[-1][0] - out[-2][0]) * (p[1] - out[-2][1]) - (out[-1][1] - out[-2][1]) * (p[0] - out[-2][0]) <= 0:
                out.pop()
            out.append(p)
        return out
    lower, upper = half(pts), half(list(reversed(pts)))
    return lower[:-1] + upper[:-1]


def shape(sid, bone, pts, role, tone, z, region=None, regions=None, material=None, filled=True, closed=True, hatch=None, pen=None, sol=None):
    """✏️ One authored 2D shape as a polygon."""
    d = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "points": P(pts), "closed": closed}
    if region:
        d["region"] = region
    if regions:
        d["regions"] = regions
    if material:
        d["material"] = material
    if hatch:
        d["hatch"] = hatch
    if pen:
        d["penMm"] = pen
    if sol:
        d["solid"] = sol
    return d


def ext_sol(z0, z1):
    """🌀 The lateral sweep of a front or plan shape between the rest positions z0 and z1."""
    a, b = span(z0, z1)
    return {"kind": "extruded", "sweep": "w", "z0": r6(a), "z1": r6(b)}


def ring_path(sid, bone, cu, cv, R, r, role, tone, z, region):
    """⭕ An annulus as a path with an outer and an inner ring."""
    outer = circle(cu, cv, R + r, 16)
    inner = list(reversed(circle(cu, cv, R - r, 16)))
    d = "M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in outer) + " Z M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in inner) + " Z"
    return {"id": sid, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, "kind": "path", "d": d, "region": region, "material": "chain-steel"}


def leg_strip(k, scale):
    """⛓️ The local outline of a chain leg (with the merged master link at the coarse scales)."""
    L = lens[k] + (2 * (ML_R + ML_r) if (k == "sling-lb" and scale in (200, 500)) else 0.0)
    return rect(-CHAIN_R, -CHAIN_R, L + CHAIN_R, CHAIN_R)


def handrail_strips():
    """🦺 The local outlines of the three posts, the handrail and the knee rail."""
    out = []
    for i, uc in enumerate(U_POSTS):
        out.append((f"post-{i + 1}", to_hr(rot_strip((uc + V_POST_BOT * TA, V_POST_BOT), (SA, CA), post_len, POST_R)), POST_R))
    out.append(("rail", to_hr(rect(rail_u0, V_RAIL - POST_R, rail_u1, V_RAIL + POST_R)), POST_R))
    out.append(("knee-rail", to_hr(rect(knee_u0, V_KNEE - KNEE_R, knee_u1, V_KNEE + KNEE_R)), KNEE_R))
    return out


def flange_strip(top, thick):
    """📏 The (u, v) outline of a flange band `thick` deep with its exact oblique ends."""
    vt, vb = (0.0, -thick) if top else (-DEPTH + thick, -DEPTH)
    return [(u_lo(vt), vt), (u_hi(vt), vt), (u_hi(vb), vb), (u_lo(vb), vb)]


def elevation(scale):
    """🖼️ The side elevation (seen from the right) of one scale in bone-local coordinates."""
    out = []
    hatch = "end-grain" if scale in (50, 100) else None
    for bid in ["bearer-low", "bearer-high"]:
        out.append(shape(f"{bid}", bid, rect(0, -0.05, 0.12, 0.05), "body", 0.68, 8, region="bearer-timber", material="bearer-softwood", hatch=hatch))
    out.append(shape("ground", "bearer-low", [(-0.4 - (XB[0] - 0.06), -0.05), (5.4 - (XB[0] - 0.06), -0.05)], "ground", 0.5, 0, filled=False, closed=False))
    thin = scale == 500
    if scale in (50, 100, 200):
        side, sid = -1, "stringer-r"
        outline = web_poly if scale in (50, 100) else bar_outline(0.0)
        out.append(shape("stringer-r", sid, to_str(side, outline), "body", 0.42, 6, region="stringer-steel", material="steel-stringer"))
        if scale in (50, 100):
            vtoe = -(FLANGE_T - 0.08 * (FLANGE_B - WEB_S) / 2) if scale == 50 else -0.015
            for name, v in [("flange-edge-top-r", vtoe), ("flange-edge-bottom-r", -DEPTH - vtoe)]:
                out.append(shape(name, sid, to_str(side, [(u_lo(v), v), (u_hi(v), v)]), "detail", 0.3, 6.1, region="stringer-steel", filled=False, closed=False))
            out.append(shape("base-plate-r", sid, to_str(side, plate_profile(0.010 if scale == 50 else 0.015)), "body", 0.3, 6.5, region="stringer-steel", material="steel-stringer"))
            out.append(shape("cleat-r", sid, to_str(side, cleat_profile(0.010 if scale == 50 else 0.015)), "body", 0.3, 6.5, region="stringer-steel", material="steel-stringer"))
        if scale == 50:
            for k in range(TREADS):
                for bi, (bu, bv) in enumerate(bolts_one):
                    out.append(shape(f"bolt-{k + 1}-{bi + 1}-r", sid, to_str(side, hexagon(bu + k * STEP, bv, 0.018)), "detail", 0.2, 6.3, region="stringer-steel", material="steel-stringer"))
            for i, uc in enumerate(U_POSTS):
                for j, v in enumerate((-0.06, -0.14)):
                    cu, cv = to_str(side, [(uc + v * TA, v)])[0]
                    out.append({"id": f"old-hole-{i + 1}-{j + 1}", "bone": sid, "role": "detail", "z": 6.3, "filled": True, "tone": 0.05, "kind": "ellipse", "cx": r6(cu), "cy": r6(cv), "rx": 0.007, "ry": 0.007, "region": "stringer-steel"})
            for which, u in [("low", U_SB), ("high", U_ST)]:
                cu, cv = to_str(side, [(u, SHACKLE_R + SHACKLE_r)])[0]
                out.append(ring_path(f"shackle-{which}-r", sid, cu, cv, SHACKLE_R, SHACKLE_r, "detail", 0.2, 7.2, "sling-chain"))
            rust = [[(0.55, -0.07), (0.72, -0.085), (0.86, -0.075)], [(1.6, -0.12), (1.81, -0.135)], [(2.45, -0.06), (2.6, -0.072), (2.78, -0.064)], [(3.3, -0.15), (3.52, -0.16)], [(3.95, -0.09), (4.12, -0.1), (4.25, -0.092)], [(1.05, -0.165), (1.22, -0.172)]]
            for i, line in enumerate(rust):
                out.append(shape(f"rust-{i + 1}", sid, to_str(side, line), "texture", 0.3, 6.2, region="stringer-steel", filled=False, closed=False))
            x0 = X_TOP + 0.010 + 0.025
            jag = inst_poly([(x0, STOREY), (x0 + 0.002, STOREY + 0.0025), (x0 - 0.001, STOREY + 0.005), (x0 + 0.002, STOREY + 0.0075), (x0, STOREY + 0.010)])
            out.append(shape("flame-cut-r", sid, to_str(side, jag), "texture", 0.3, 6.6, region="stringer-steel", filled=False, closed=False))
    if scale == 200:
        first, last = tread_section(1, True, *TREAD_200), tread_section(TREADS, True, *TREAD_200)
        out.append(shape("treads", "flight", hull(first + last), "body", 0.62, 3, region="tread-plate", material="chequer-steel"))
    for k in range(TREADS):
        if scale in (200, 500):
            break
        sec = tread_section(k + 1, block=scale != 50)
        out.append(shape(f"tread-{k + 1:02d}", "flight", sec, "body", 0.62, 3, region="tread-plate", material="chequer-steel"))
    if scale == 500:
        out.append(shape("flight", "flight", bar_outline(0.0), "body", 0.45, 6, regions=["stringer-steel", "tread-plate"], material="steel-stringer"))
    for sid_, pts, rr in handrail_strips():
        out.append(shape(sid_, "handrail", pts, "body", 0.55, 2, region="handrail-tube", material="handrail-yellow", filled=not thin))
    for k in legs_def:
        out.append(shape(k, k, leg_strip(k, scale), "body" if scale in (200, 500) else "detail", 0.2, 7 if k.startswith("sling-r") else 2.5, region="sling-chain", material="chain-steel", filled=not thin))
    if scale in (50, 100):
        out.append(shape("master-link", "master-link", rect(0, -ML_r, 2 * (ML_R + ML_r), ML_r), "detail", 0.2, 2.6, region="sling-chain", material="chain-steel"))
    return out


def front_plan(scale, view):
    """🖼️ The front elevation or the plan of one scale: every part of the flight as a lateral sweep of its side profile, the bearers drawn in rest coordinates."""
    out = []
    thin = scale == 500
    for bid, xb in zip(["bearer-low", "bearer-high"], XB):
        if view == "front":
            out.append(shape(bid, bid, rect(-BEARER[2] / 2, 0, BEARER[2] / 2, BEARER[1]), "body", 0.68, 1, region="bearer-timber", material="bearer-softwood"))
        else:
            out.append(shape(bid, bid, rect(xb - 0.06, -BEARER[2] / 2, xb + 0.06, BEARER[2] / 2), "body", 0.68, 1, region="bearer-timber", material="bearer-softwood"))
    if view == "front":
        out.append(shape("ground", "bearer-low", [(-0.8, 0.0), (0.8, 0.0)], "ground", 0.5, 0, filled=False, closed=False))
    for side, t, sid in SIDES:
        if scale in (50, 100):
            web_hi = W_OUT if scale == 50 else 0.415
            fl = 0.01416 if scale == 50 else 0.015
            out.append(shape(f"web-{t}", sid, to_str(side, web_poly), "body", 0.42, 4, region="stringer-steel", material="steel-stringer", sol=ext_sol(lat(side, W_IN), lat(side, web_hi))))
            for top, name in [(True, "top"), (False, "bottom")]:
                out.append(shape(f"flange-{name}-{t}", sid, to_str(side, flange_strip(top, fl)), "body", 0.45, 4.1, region="stringer-steel", material="steel-stringer", sol=ext_sol(lat(side, web_hi), lat(side, W_TIP))))
            th = 0.010 if scale == 50 else 0.015
            out.append(shape(f"base-plate-{t}", sid, to_str(side, plate_profile(th)), "body", 0.3, 4.2, region="stringer-steel", material="steel-stringer", sol=ext_sol(lat(side, 0.3875), lat(side, 0.4875))))
            out.append(shape(f"cleat-{t}", sid, to_str(side, cleat_profile(th)), "body", 0.3, 4.2, region="stringer-steel", material="steel-stringer", sol=ext_sol(lat(side, 0.3875), lat(side, 0.4875))))
            if scale == 50:
                for which, u, w in [("low", U_SB, W_SB), ("high", U_ST, W_ST)]:
                    cu, cv = to_str(side, [(u, SHACKLE_R + SHACKLE_r)])[0]
                    out.append(shape(f"shackle-{which}-{t}", sid, circle(cu, cv, SHACKLE_R + SHACKLE_r, 12), "detail", 0.2, 6.2 if which == "low" else 6.25, region="sling-chain", material="chain-steel", sol=ext_sol(lat(side, w) - SHACKLE_r, lat(side, w) + SHACKLE_r)))
        elif scale == 200:
            out.append(shape(f"stringer-{t}", sid, to_str(side, bar_outline(0.0)), "body", 0.42, 4, region="stringer-steel", material="steel-stringer", sol=ext_sol(lat(side, W_IN), lat(side, W_TIP))))
    if scale == 500:
        out.append(shape("flight", "flight", bar_outline(0.0), "body", 0.45, 4, regions=["stringer-steel", "tread-plate"], material="steel-stringer", sol=ext_sol(-W_TIP, W_TIP)))
    else:
        hatch = "chequer" if (view == "plan" and scale in (50, 100)) else None
        for k in range(TREADS):
            sec = tread_section(k + 1, True, *TREAD_200) if scale == 200 else tread_section(k + 1, block=scale != 50)
            out.append(shape(f"tread-{k + 1:02d}", "flight", sec, "body", 0.62, 3, region="tread-plate", material="chequer-steel", hatch=hatch, pen=0.1 if scale == 200 else None, sol=ext_sol(-W_IN, W_IN)))
    for sid_, pts, rr in handrail_strips():
        out.append(shape(sid_, "handrail", pts, "body", 0.55, 5, region="handrail-tube", material="handrail-yellow", filled=not thin, sol=ext_sol(W_POST - rr, W_POST + rr)))
    for k, (s, u, w, a0) in legs_def.items():
        out.append(shape(k, k, leg_strip(k, scale), "body" if scale in (200, 500) else "detail", 0.2, 6, region="sling-chain", material="chain-steel", filled=not thin, sol=ext_sol(w - CHAIN_R, w + CHAIN_R)))
    if scale in (50, 100):
        out.append(shape("master-link", "master-link", rect(0, -ML_r, 2 * (ML_R + ML_r), ML_r), "detail", 0.2, 6.5, region="sling-chain", material="chain-steel", sol=ext_sol(W_SB - ML_R - ML_r, W_SB + ML_R + ML_r)))
    return out


AREAS = {"stringer-steel": 6.45, "tread-plate": 8.30, "handrail-tube": 1.47, "bearer-timber": 1.01, "sling-chain": 0.62}
AREA_FILE = os.path.join(HERE, "author-areas.json")
if os.path.exists(AREA_FILE):
    AREAS.update(json.load(open(AREA_FILE)))

materials = [
    {"id": "steel-stringer", "title": T("Steel St 37-2, grey primer weathered to a rust patina", "Stahl St 37-2, graue Grundierung zu Rostpatina verwittert"), "emissivity": 0.88, "solarAbsorptivity": 0.75, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "chequer-steel", "title": T("Teardrop chequer plate, worn to bare rusty steel on the walking line", "Tränenblech, auf der Lauflinie zu blankem, rostigem Stahl abgelaufen"), "emissivity": 0.85, "solarAbsorptivity": 0.82, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "handrail-yellow", "title": T("Steel tube with faded, chipped safety-yellow paint", "Stahlrohr mit verblasstem, abgeplatztem Sicherheitsgelb"), "emissivity": 0.90, "solarAbsorptivity": 0.50, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "bearer-softwood", "title": T("Reclaimed softwood bearer, weathered grey", "Gebrauchter Nadelholz-Unterleger, grau verwittert"), "emissivity": 0.90, "solarAbsorptivity": 0.65, "densityKgM3": TIMBER_RHO, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "authored-estimate", "reference": "en-338"},
    {"id": "chain-steel", "title": T("Grade 8 alloy steel chain, red coating worn", "Güteklasse-8-Kette, rote Beschichtung abgenutzt"), "emissivity": 0.85, "solarAbsorptivity": 0.70, "densityKgM3": 7850, "conductivityWmK": 45, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 800000000, "source": "literature", "reference": "en-818-4"},
]

regions = [
    {"id": "stringer-steel", "title": T("Stringers, base plates, cut cleats and tread bolts", "Wangen, Fußplatten, abgetrennte Laschen und Stufenschrauben"), "kind": "surface", "model": "passive", "material": "steel-stringer", "areaM2": AREAS["stringer-steel"], "characteristicLengthM": 0.2, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 17200, "source": "authored-estimate", "reference": "en-iso-10456"},
    {"id": "tread-plate", "title": T("Chequer-plate tread pans and side cheeks", "Tränenblech-Stufenwannen und Seitenwangen"), "kind": "surface", "model": "passive", "material": "chequer-steel", "areaM2": AREAS["tread-plate"], "characteristicLengthM": 0.26, "geometry": "horizontal-plate-up", "skyViewFactor": 0.9, "layers": [], "heatCapacityJM2K": 9600, "source": "authored-estimate", "reference": "en-iso-10456"},
    {"id": "handrail-tube", "title": T("Handrail, knee rail, posts and post shoes", "Handlauf, Knieleiste, Pfosten und Pfostenlaschen"), "kind": "surface", "model": "passive", "material": "handrail-yellow", "areaM2": AREAS["handrail-tube"], "characteristicLengthM": 0.0424, "geometry": "cylinder", "skyViewFactor": 0.8, "layers": [], "heatCapacityJM2K": 8600, "source": "authored-estimate", "reference": "en-iso-10456"},
    {"id": "bearer-timber", "title": T("Squared-timber bearers", "Kantholz-Unterleger"), "kind": "surface", "model": "passive", "material": "bearer-softwood", "areaM2": AREAS["bearer-timber"], "characteristicLengthM": 0.12, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 25900, "source": "authored-estimate", "reference": "en-iso-13786"},
    {"id": "sling-chain", "title": T("Four-leg chain sling with shackles and master link", "Vierstrang-Kettengehänge mit Schäkeln und Aufhängering"), "kind": "surface", "model": "passive", "material": "chain-steel", "areaM2": AREAS["sling-chain"], "characteristicLengthM": 0.018, "geometry": "cylinder", "skyViewFactor": 0.8, "layers": [], "heatCapacityJM2K": 11000, "source": "authored-estimate", "reference": "en-iso-10456"},
]


def binding(sun, indoor=False):
    """🌤️ The thermal binding of every region for one pose."""
    regs = {
        "stringer-steel": {"exposedFraction": 1, "sunlitFraction": sun * 0.6},
        "tread-plate": {"exposedFraction": 1, "sunlitFraction": sun * 0.7},
        "handrail-tube": {"exposedFraction": 1, "sunlitFraction": sun * 0.6},
        "bearer-timber": {"exposedFraction": 1, "sunlitFraction": sun * 0.4, "contact": {"fraction": 0.24, "tempC": 14}},
        "sling-chain": {"exposedFraction": 1, "sunlitFraction": sun * 0.7},
    }
    for v in regs.values():
        v["sunlitFraction"] = round(v["sunlitFraction"], 3)
    return {"regions": regs, "emitters": {}}


BEARER_CONTACTS = [a["id"] for a in anchors if a["id"].startswith("bearer-")]
SHACKLE_PINS = [a["id"] for a in anchors if a["id"].startswith("shackle-")]
SERVICE = ["foot-l", "foot-r", "landing-l", "landing-r"]
lifted_bones = {"lift": {"dy": LIFT_DY}}
lifted_bones.update(leg_off)
installed = {"lift": {"dy": r6(PLATE - REST_Y)}, "flight": {"angleDeg": round(math.degrees(ALPHA), 4)}}
q_member = 5000 * W_IN * CA
tread_from = i2f(0.0, RISE)[0] - U0
tread_to = i2f((TREADS - 1) * GOING + TREAD_D, TREADS * RISE)[0] - U0
walker_mid = 6 * STEP

poses = [
    {"id": "stored", "title": T("Stored on bearers", "Auf Unterlegern gelagert"), "description": T("The flight lies on the bottom flanges of its two stringers on two squared-timber bearers in the component yard, the four-leg chain sling shackled on and laid along the top flanges for the next pick.", "Der Lauf liegt mit den Unterflanschen seiner beiden Wangen auf zwei Kantholz-Unterlegern im Bauteillager, das Vierstrang-Kettengehänge ist angeschäkelt und für den nächsten Hub auf den Oberflanschen abgelegt."), "bones": {}, "ground": "authored", "contacts": BEARER_CONTACTS, "thermal": binding(1.0), "loads": []},
    {"id": "slung", "key": True, "title": T("Slung, still on the bearers", "Angeschlagen, noch auf den Unterlegern"), "description": T("The crane has taken up the four chain legs to the master link above the centre of mass; the flight still rests on the bearers.", "Der Kran hat die vier Kettenstränge bis zum Aufhängering über dem Schwerpunkt gestrafft; der Lauf liegt noch auf den Unterlegern."), "bones": dict(leg_off), "ground": "authored", "contacts": BEARER_CONTACTS, "thermal": binding(1.0), "loads": []},
    {"id": "lifted", "title": T("Lifted on the chain sling", "Am Kettengehänge angehoben"), "description": T("The flight hangs level 0.60 m above the bearers from the four shackles in its top flanges; the bearers stay on the yard floor.", "Der Lauf hängt waagrecht 0,60 m über den Unterlegern an den vier Schäkeln in seinen Oberflanschen; die Unterleger bleiben auf dem Lagerplatz liegen."), "bones": lifted_bones, "ground": "authored", "contacts": SHACKLE_PINS, "thermal": binding(1.0), "loads": []},
    {"id": "reinstalled", "title": T("Reinstalled, empty", "Wieder eingebaut, leer"), "description": T("Back in service for a storey of 2.80 m: the base plates are pinned to the floor, the cut cleats bear on a new angle at the landing (roller); self-weight only.", "Wieder im Einsatz für ein Geschoss von 2,80 m: die Fußplatten sind am Boden festgelegt, die abgetrennten Laschen liegen auf einem neuen Winkel am Podest auf (Rollenlager); nur Eigengewicht."), "bones": dict(installed), "ground": "authored", "contacts": SERVICE, "thermal": binding(0.2, True), "loads": []},
    {"id": "reinstalled-imposed", "title": T("Reinstalled, full stair load", "Wieder eingebaut, volle Treppenlast"), "description": T("Imposed stair load of category T2, 5.0 kN/m² over the clear width of 0.80 m, 2.0 kN/m per stringer on plan, 1.60 kN/m along each inclined member.", "Treppennutzlast der Kategorie T2, 5,0 kN/m² über die lichte Breite von 0,80 m, 2,0 kN/m je Wange im Grundriss, 1,60 kN/m entlang jedes geneigten Stabs."), "bones": dict(installed), "ground": "authored", "contacts": SERVICE, "thermal": binding(0.2, True), "loads": [{"id": f"imposed-{t}", "kind": "imposed", "title": T(f"Stair load T2 on the {'left' if t == 'l' else 'right'} stringer, 1.60 kN/m", f"Treppenlast T2 auf der {'linken' if t == 'l' else 'rechten'} Wange, 1,60 kN/m"), "fxN": 0, "fyN": 0, "distributed": {"bone": sid, "fromM": r6(tread_from), "toM": r6(tread_to), "wxNPerM": 0, "wyNPerM": r6(-q_member)}, "source": "literature", "reference": "din-en-1991-1-1-na"} for side, t, sid in SIDES]},
    {"id": "reinstalled-walker", "title": T("Reinstalled, person on tread 7", "Wieder eingebaut, Person auf Stufe 7"), "description": T("A person of 0.8 kN stands in the middle of tread 7; each stringer carries half.", "Eine Person von 0,8 kN steht in der Mitte der Stufe 7; jede Wange trägt die Hälfte."), "bones": dict(installed, **{"walker-l": {"dx": r6(walker_mid)}, "walker-r": {"dx": r6(walker_mid)}}), "ground": "authored", "contacts": SERVICE, "thermal": binding(0.2, True), "loads": [{"id": f"walker-{t}", "kind": "occupant", "title": T(f"Half a person, 400 N, on the {'left' if t == 'l' else 'right'} stringer", f"Halbe Person, 400 N, auf der {'linken' if t == 'l' else 'rechten'} Wange"), "anchor": f"walker-{t}-point", "fxN": 0, "fyN": -400.0, "source": "authored-estimate"} for side, t, sid in SIDES]},
]

clips = [
    {"id": "lift", "title": T("Lift off the bearers", "Von den Unterlegern abheben"), "description": T("The rigger hands the four chain legs up to the crane hook, the crane takes up the slack until the master link stands above the centre of mass, then lifts the flight 0.60 m clear of the bearers and holds it.", "Der Anschläger reicht die vier Kettenstränge zum Kranhaken, der Kran strafft sie, bis der Aufhängering über dem Schwerpunkt steht, hebt den Lauf 0,60 m von den Unterlegern ab und hält ihn."), "durationS": 10, "fps": 25, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 3, "pose": "slung", "ease": "smooth"}, {"t": 4, "pose": "slung", "ease": "smooth"}, {"t": 9, "pose": "lifted", "ease": "smooth"}, {"t": 10, "pose": "lifted", "ease": "smooth"}], "contacts": [{"anchor": a, "fromS": 0, "toS": 4} for a in BEARER_CONTACTS] + [{"anchor": a, "fromS": 3, "toS": 10} for a in SHACKLE_PINS], "phases": [{"id": "rig", "fromS": 0, "toS": 3, "title": T("Rigging", "Anschlagen")}, {"id": "take-up", "fromS": 3, "toS": 4, "title": T("Taking up the load", "Last aufnehmen")}, {"id": "hoist", "fromS": 4, "toS": 9, "title": T("Hoisting", "Heben")}, {"id": "hold", "fromS": 9, "toS": 10, "title": T("Holding", "Halten")}], "events": [{"t": 3, "kind": "motion", "title": T("Chain legs taut", "Kettenstränge straff")}, {"t": 4, "kind": "contact-off", "title": T("Flight leaves the bearers", "Lauf verlässt die Unterleger")}], "tracks": []},
    {"id": "walk-up", "title": T("Person walking up the reinstalled flight", "Person geht den wieder eingebauten Lauf hinauf"), "description": T("A person of 0.8 kN climbs from tread 1 to tread 14 in eight seconds; the load point travels along both stringers.", "Eine Person von 0,8 kN steigt in acht Sekunden von Stufe 1 zu Stufe 14; der Lastpunkt wandert entlang beider Wangen."), "durationS": 8, "fps": 25, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "reinstalled-walker", "ease": "linear", "bones": {"walker-l": {"dx": 0}, "walker-r": {"dx": 0}}}, {"t": 8, "pose": "reinstalled-walker", "ease": "linear", "bones": {"walker-l": {"dx": r6(walker_travel)}, "walker-r": {"dx": r6(walker_travel)}}}], "contacts": [{"anchor": a, "fromS": 0, "toS": 8} for a in SERVICE], "phases": [{"id": "climb", "fromS": 0, "toS": 8, "title": T("Climb", "Aufstieg")}], "events": [{"t": 0, "kind": "motion", "title": T("Person on tread 1", "Person auf Stufe 1")}], "tracks": []},
]


def bounds_stored():
    """📦 The rest-pose bounding box of the 1:50 body in world metres, sampled from the authored geometry."""
    xs, ys, zs = [], [], []
    def add(u, v, w):
        xs.append(u)
        ys.append(REST_Y + v)
        zs.append(w)
    for side, t, sid in SIDES:
        for u, v in web_poly + plate_profile(0.010) + cleat_profile(0.010):
            add(u, v, side * W_TIP)
    for u, v in tread_section(TREADS):
        add(u, v, 0)
    for _, pts, rr in handrail_strips():
        for a, b in pts:
            add(a + H_ORIGIN[0], b + H_ORIGIN[1], W_POST + rr)
    add(rail_u1 + POST_R, V_RAIL + POST_R, W_POST)
    for k, (s, u, w, a0) in legs_def.items():
        tip = u + (lens[k] if a0 == 0 else -lens[k])
        add(tip + (CHAIN_R if a0 == 0 else -CHAIN_R), 2 * CHAIN_R, w)
    add(U_SB + SHACKLE_R + lens["sling-lb"] + 2 * (ML_R + ML_r), CHAIN_R, W_SB)
    for xb in XB:
        xs += [xb - 0.06, xb + 0.06]
        ys += [0.0]
        zs += [-BEARER[2] / 2, BEARER[2] / 2]
    return min(xs), max(xs), min(ys), max(ys), min(zs), max(zs)


bx0, bx1, by0, by1, bz0, bz1 = bounds_stored()
dimensions = {"heightM": round(by1 - by0, 3), "widthM": round(bx1 - bx0, 3), "depthM": round(bz1 - bz0, 3), "massKg": round(MASS_TOTAL, 2)}

references = [
    {"id": "din-1026-1", "citation": "DIN 1026-1:2009-09 Warmgewalzter U-Profilstahl - Teil 1: U-Profilstahl mit geneigten Flanschflächen; Maße, Masse und statische Werte: UPN 200 h 200, b 75, s 8.5, t 11.5, r1 11.5, r2 6 mm, A 32.2 cm2, G 25.3 kg/m, Iy 1910 cm4, Wy 191 cm3, ey 2.01 cm, inner flange slope 8 %."},
    {"id": "din-17100", "citation": "DIN 17100:1980-01 Allgemeine Baustähle: St 37-2, upper yield strength 235 N/mm2 (the grade of stairs of the 1970s; today EN 10025-2 S235JR)."},
    {"id": "en-10025-2", "citation": "EN 10025-2:2019 Hot rolled products of structural steels, part 2: non-alloy structural steels (S235: yield strength 235 MPa, E 210 GPa)."},
    {"id": "din-59220", "citation": "DIN 59220:2000-04 Flachzeug aus Stahl - Warmgewalzte Bleche mit Mustern (Tränen- und Warzenbleche): teardrop plate 5/7 mm, about 42 kg/m2 (value used: 42.4 kg/m2, authored estimate within the table)."},
    {"id": "en-iso-14122-3", "citation": "EN ISO 14122-3:2016 Safety of machinery - Permanent means of access to machinery - Part 3: Stairs, stepladders and guard-rails: 600 mm <= g + 2h <= 660 mm, handrail 0.9 to 1.0 m above the pitch line, knee rail midway, clear width at least 600 mm, preferably 800 mm."},
    {"id": "din-en-1991-1-1-na", "citation": "DIN EN 1991-1-1/NA:2010-12 National Annex to Eurocode 1, Table 6.1DE: stairs category T2 (stairs and landings in offices, workshops, industrial access) qk 5.0 kN/m2, Qk 2.0 kN."},
    {"id": "en-10219-2", "citation": "EN 10219-2:2019 Cold formed welded structural hollow sections, tolerances, dimensions and sectional properties: CHS 42.4 x 2.6 mm 2.55 kg/m, CHS 33.7 x 2.6 mm 1.99 kg/m."},
    {"id": "en-10056-1", "citation": "EN 10056-1:2017 Structural steel equal and unequal leg angles, dimensions: L 100 x 100 x 10 mm, 15.0 kg/m."},
    {"id": "iso-4017", "citation": "ISO 4017:2022 Fasteners - Hexagon head screws, product grades A and B: M12 width across flats s = 18 mm, head height k = 7.5 mm; ISO 4032 nut M12 m = 10.8 mm (modelled 7.5 mm)."},
    {"id": "en-818-4", "citation": "EN 818-4:1996+A1:2008 Short link chain for lifting purposes, chain slings, grade 8: 8 mm chain about 1.4 kg/m, four-leg sling working load limit 4.25 t at 0 to 45 degrees."},
    {"id": "en-1677-4", "citation": "EN 1677-4:2000+A1:2008 Components for slings - Safety - Part 4: Links, grade 8 (master link for an 8 mm four-leg sling, about 16 mm material, 1.6 kg with intermediate links)."},
    {"id": "en-13889", "citation": "EN 13889:2003+A1:2008 Forged steel shackles for general lifting purposes, dee shackles and bow shackles, grade 6 (0.75 t bow shackle, pin 10 mm)."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C24 mean density 420 kg/m3, E0,mean 11 GPa; reclaimed, moist bearers taken at 450 kg/m3 (authored estimate)."},
    {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 33, physical properties of materials: emissivity and solar absorptance of painted, oxidised and rusted steel and of weathered wood (typical rounded values guided by this chapter, not table look-ups)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties: steel 50 W/mK, 450 J/kgK, 7800 kg/m3; softwood 0.13 W/mK, 1600 J/kgK."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); about half of it is taken as the storage depth of an exposed timber face."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, general principles of presentation, part 50: basic conventions for representing areas on cuts and sections (hatching)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: kinds, contents and basic principles of presentation (scales, line groups)."},
]

n_parts = {s: len(BODY[s][0]) for s in BODY}


def rep_decisions(scale):
    """📝 The eight drawing decisions of one scale."""
    if scale == 50:
        return {
            "silhouette": T(f"Side elevation from the right of the flight as it lies in the yard: the near UPN 200 stringer as its true outline (horizontal floor cut, 200 mm deep, plumb cut at the landing face; {U_TOP:.3f} m along the top edge), its flange toes as two detail lines, the welded 10 mm base plate under the floor cut with its 60 mm overhangs, the L 100 x 100 x 10 top cleat with the 25 mm stub of its flame-cut leg, the 28 M12 bolt heads of the tread cheeks, six empty 14 mm holes of the removed second handrail, the far handrail (three posts, handrail and knee rail of 42.4 and 33.7 mm) above the stringers, the four chain legs on the top flanges with their bow shackles and the master link, and the two 120 x 100 mm bearers end-on.", f"Seitenansicht von rechts des Laufs, wie er im Lager liegt: die vordere Wange UPN 200 mit ihrer wahren Kontur (waagrechter Bodenschnitt, 200 mm hoch, lotrechter Schnitt an der Podeststirn; {U_TOP:.3f} m entlang der Oberkante), ihre Flanschkanten als zwei Detaillinien, die angeschweißte 10-mm-Fußplatte unter dem Bodenschnitt mit ihren 60-mm-Überständen, die Lasche L 100 x 100 x 10 oben mit dem 25-mm-Stummel ihres abgebrannten Schenkels, die 28 Schraubenköpfe M12 der Stufenwangen, sechs leere 14-mm-Löcher des entfernten zweiten Handlaufs, der hintere Handlauf (drei Pfosten, Handlauf und Knieleiste aus 42,4 und 33,7 mm) über den Wangen, die vier Kettenstränge auf den Oberflanschen mit ihren Schäkeln und dem Aufhängering und die beiden Unterleger 120 x 100 mm im Querschnitt."),
            "retained": T("Every part at its true size: channel depth and flange toes, plate and cleat thickness of 10 mm (0.2 mm), bolt heads of 18 mm across flats, the 18 mm chain, the 20 mm shackle bows. Front and plan draw every part as the lateral sweep of its side profile, so the three views stay one body in the lifted and reinstalled poses.", "Jedes Teil in wahrer Größe: Profilhöhe und Flanschkanten, Platten- und Laschendicke 10 mm (0,2 mm), Schraubenköpfe mit 18 mm Schlüsselweite, die 18-mm-Kette, die 20-mm-Schäkelbügel. Vorderansicht und Draufsicht zeichnen jedes Teil als seitliche Auskragung seines Seitenprofils, so bleiben die drei Ansichten auch in den Posen angehoben und wieder eingebaut ein Körper."),
            "merged": T("Flanges, root fillets and web of a stringer are one outline in the side view; in front and plan the web and the two flange bands are drawn separately so the open channel reads.", "Flansche, Ausrundungen und Steg einer Wange sind in der Seitenansicht eine Kontur; in Vorderansicht und Draufsicht sind Steg und die beiden Flanschbänder getrennt gezeichnet, damit das offene U-Profil lesbar bleibt."),
            "omitted": T("The far stringer, its plates and the treads lie behind the near stringer in the side view and are covered by it; the post shoes and nuts lie inside the channels; the teardrops of the plate (2 mm) are a hatch.", "Die hintere Wange, ihre Platten und die Stufen liegen in der Seitenansicht hinter der vorderen Wange und werden von ihr verdeckt; die Pfostenlaschen und Muttern liegen in den U-Profilen; die Tränen des Blechs (2 mm) sind eine Schraffur."),
            "thinElements": T("The 5 mm tread plate and its 40 mm folded edges are filled tread outlines; the flame-cut edge of the cleat stub and the rust runs on the near web are texture strokes of 0.1 mm.", "Das 5-mm-Stufenblech und seine 40-mm-Abkantungen sind gefüllte Stufenkonturen; die Brennschnittkante des Laschenstummels und die Rostläufer auf dem vorderen Steg sind Texturstriche von 0,1 mm."),
            "hatching": T("Timber end grain of the bearers in 45-degree lines at 0.6 mm (DIN ISO 128-50); the chequer plate of the treads as a 45-degree cross hatch at 0.5 mm in plan.", "Hirnholz der Unterleger als 45-Grad-Linien im Abstand von 0,6 mm (DIN ISO 128-50); das Tränenblech der Stufen als 45-Grad-Kreuzschraffur im Abstand von 0,5 mm in der Draufsicht."),
            "styles": T("Mid-dark stringers, lighter treads, pale yellow handrail, dark chain and bolts, light timber.", "Mitteldunkle Wangen, hellere Stufen, blassgelber Handlauf, dunkle Kette und Schrauben, helles Holz."),
            "overlays": T("Thermal and structural overlays colour each part by its region (stringer steel, tread plate, handrail tube, bearer timber, sling chain) and by its member bone.", "Thermische und statische Überlagerungen färben jedes Teil nach seiner Region (Wangenstahl, Stufenblech, Handlaufrohr, Unterlegerholz, Kettengehänge) und nach seinem Stabknochen."),
        }
    if scale == 100:
        return {
            "silhouette": T("The near stringer outline with its flange toes as two lines (flanges and web generalised to the 15 mm feature), base plate and cleat at 15 mm, the far handrail with three posts, handrail and knee rail at their true diameters, the four chain legs on the top flanges with the master link, the bearers end-on.", "Die Kontur der vorderen Wange mit ihren Flanschkanten als zwei Linien (Flansche und Steg auf das 15-mm-Merkmal verallgemeinert), Fußplatte und Lasche mit 15 mm, der hintere Handlauf mit drei Pfosten, Handlauf und Knieleiste in wahren Durchmessern, die vier Kettenstränge auf den Oberflanschen mit dem Aufhängering, die Unterleger im Querschnitt."),
            "retained": T("Channel depth 200 mm (2 mm), the open channel in front and plan with 15 mm web and flanges, the 40 mm treads as pan envelopes, the 42.4 / 33.7 mm tubes, the 18 mm chain and the 16 mm master link.", "Profilhöhe 200 mm (2 mm), das offene U-Profil in Vorderansicht und Draufsicht mit 15 mm Steg und Flanschen, die 40-mm-Stufen als Wannenhüllen, die Rohre 42,4 / 33,7 mm, die 18-mm-Kette und der 16-mm-Aufhängering."),
            "merged": T("Tread pan, folded edges and side cheeks are one block of 260 x 40 mm reaching the webs.", "Stufenwanne, Abkantungen und Seitenwangen sind ein Block von 260 x 40 mm bis an die Stege."),
            "omitted": T("Bolt heads and nuts (18 mm, 7.5 mm high), the empty 14 mm holes, the shackles (12 mm material) and the post shoes are below the 15 mm feature or hidden and left out; the rust runs and the flame-cut texture are dropped.", "Schraubenköpfe und Muttern (18 mm, 7,5 mm hoch), die leeren 14-mm-Löcher, die Schäkel (12 mm Material) und die Pfostenlaschen liegen unter dem 15-mm-Merkmal oder verdeckt und entfallen; Rostläufer und Brennschnitttextur entfallen."),
            "thinElements": T("No part is drawn thinner than 15 mm; the chain legs are 18 mm strips.", "Kein Teil ist dünner als 15 mm gezeichnet; die Kettenstränge sind 18-mm-Streifen."),
            "hatching": T("End grain of the bearers at 0.5 mm, chequer cross hatch at 0.45 mm in plan.", "Hirnholz der Unterleger im Abstand von 0,5 mm, Tränenblech-Kreuzschraffur im Abstand von 0,45 mm in der Draufsicht."),
            "styles": T("As 1:50 with a 0.25 mm outline pen.", "Wie 1:50 mit einer Konturfeder von 0,25 mm."),
            "overlays": T("Overlays per region and member as at 1:50.", "Überlagerungen je Region und Stab wie bei 1:50."),
        }
    if scale == 200:
        return {
            "silhouette": T("Each stringer is one 200 x 75 mm bar from the floor to the landing face (base plate merged into its foot, the 10 mm cleat into its head), the fourteen treads as 251 x 36 mm blocks drawn with the 0.1 mm pen (the nosing kept; 1 mm overlap of consecutive treads when reinstalled, open risers of 89 mm in plan that stay 0.15 mm white at 1:350), the handrail tubes at their true diameters, the four chain legs as 18 mm strips with the master link merged into the end of the foot-end leg, the bearers end-on.", "Jede Wange ist ein Balken 200 x 75 mm vom Boden bis zur Podeststirn (Fußplatte in ihren Fuß, 10-mm-Lasche in ihren Kopf eingeschmolzen), die vierzehn Stufen als Blöcke 251 x 36 mm mit der 0,1-mm-Feder (Vorderkante beibehalten; eingebaut überlappen aufeinanderfolgende Stufen um 1 mm, offene Setzstufen von 89 mm im Grundriss bleiben bei 1:350 0,15 mm weiß), die Handlaufrohre in wahren Durchmessern, die vier Kettenstränge als 18-mm-Streifen, der Aufhängering in das Ende des fußseitigen Strangs eingeschmolzen, die Unterleger im Querschnitt."),
            "retained": T("Flight length, depth and width, the open risers between the tilted treads in plan (89 mm, 0.45 mm), handrail height, the bearers and the sling lying on the top flanges.", "Lauflänge, -höhe und -breite, die offenen Setzstufen zwischen den geneigten Stufen in der Draufsicht (89 mm, 0,45 mm), Handlaufhöhe, die Unterleger und das auf den Oberflanschen abgelegte Gehänge."),
            "merged": T("Channel web and flanges are one bar (the 60 mm channel opening is two features and reads in no view at 0.3 mm); plates and cleats are merged into the bars.", "Steg und Flansche des U-Profils sind ein Balken (die 60-mm-Profilöffnung ist zwei Merkmale und liest sich bei 0,3 mm in keiner Ansicht); Platten und Laschen sind in die Balken eingeschmolzen."),
            "omitted": T("Bolts, holes, shackles, post shoes, chequer pattern and all texture.", "Schrauben, Löcher, Schäkel, Pfostenlaschen, Tränenmuster und jede Textur."),
            "thinElements": T("Chain legs are filled 18 mm strips with the 0.18 mm body pen, readable at 1:350.", "Kettenstränge sind gefüllte 18-mm-Streifen mit der 0,18-mm-Konturfeder, bei 1:350 lesbar."),
            "hatching": T("No hatch: the bearers are 0.6 x 0.5 mm.", "Keine Schraffur: die Unterleger sind 0,6 x 0,5 mm groß."),
            "styles": T("Mid-dark bars, lighter treads, 0.18 mm outline.", "Mitteldunkle Balken, hellere Stufen, Kontur 0,18 mm."),
            "overlays": T("Overlays per region and member.", "Überlagerungen je Region und Stab."),
        }
    return {
        "silhouette": T("The flight as one 4.66 x 0.20 m bar, 0.95 m wide (9.3 x 0.4 mm on paper), on its two bearers; the handrail as the outlines of its true tubes above it, the four chain legs as outlined strips on the top.", "Der Lauf als ein Balken 4,66 x 0,20 m, 0,95 m breit (9,3 x 0,4 mm auf Papier), auf seinen beiden Unterlegern; der Handlauf als Kontur seiner wahren Rohre darüber, die vier Kettenstränge als umrissene Streifen obenauf."),
        "retained": T("Overall length, height and width, the bearers (120 x 100 mm, at least 0.23 mm with the pen at 1:1000), the handrail height of 1.0 m above the pitch line.", "Gesamtlänge, -höhe und -breite, die Unterleger (120 x 100 mm, mit der Feder bei 1:1000 mindestens 0,23 mm), die Handlaufhöhe von 1,0 m über der Lauflinie."),
        "merged": T("Stringers, treads, plates and cleats are one bar; the master link is merged into its leg.", "Wangen, Stufen, Platten und Laschen sind ein Balken; der Aufhängering ist in seinen Strang eingeschmolzen."),
        "omitted": T("Treads as separate parts (the 80 mm open risers are one feature), every detail below 75 mm.", "Stufen als eigene Teile (die 80-mm-Öffnungen sind ein Merkmal), jede Einzelheit unter 75 mm."),
        "thinElements": T("Tubes and chain legs are unfilled outlines of their true width with the 0.13 mm pen: a line that stays printable at 1:1000.", "Rohre und Kettenstränge sind ungefüllte Konturen ihrer wahren Breite mit der 0,13-mm-Feder: eine Linie, die bei 1:1000 druckbar bleibt."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Mid-grey bar, light bearers, 0.13 mm outline.", "Mittelgrauer Balken, helle Unterleger, Kontur 0,13 mm."),
        "overlays": T("The bar shows the area-weighted value of stringer steel and tread plate.", "Der Balken zeigt den flächengewichteten Wert von Wangenstahl und Stufenblech."),
    }


def vol_decisions(scale):
    """📝 The volume decisions of one scale."""
    if scale == 50:
        return [
            T(f"1:50 is the flight as dismantled: two UPN 200 stringers with the true DIN 1026-1 section (8.5 mm web, 8 % tapered flanges of 11.5 mm mean thickness, root radius 11.5 mm, toe radius 6 mm) as a web extrude with the six empty 14 mm holes of the removed second handrail in the right web and two flange polyhedra whose ends lie exactly in the horizontal floor cut and the plumb landing cut; 10 mm base plates with two 18 mm anchor holes each under the floor cut, the remaining legs of the flame-cut L 100 x 100 x 10 cleats at the head, 56 M12 bolt heads and nuts; fourteen folded 5 mm chequer-plate pans (260 x 40 mm) with 8 mm side cheeks as one solid each repeated along the pitch line; three side-mounted 42.4 mm posts with six post shoes, the 42.4 mm handrail and the 33.7 mm knee rail; four 8 mm chain legs as 18 mm slender rods, four bow shackles and the 16 mm master link; two 120 x 100 x 1100 mm bearers ({n_parts[50]} solids).", f"1:50 ist der Lauf wie rückgebaut: zwei Wangen UPN 200 mit dem wahren Querschnitt nach DIN 1026-1 (Steg 8,5 mm, um 8 % geneigte Flansche von 11,5 mm mittlerer Dicke, Ausrundung 11,5 mm, Kantenradius 6 mm) als Stegextrusion mit den sechs leeren 14-mm-Löchern des entfernten zweiten Handlaufs im rechten Steg und zwei Flansch-Polyedern, deren Enden genau im waagrechten Bodenschnitt und im lotrechten Podestschnitt liegen; 10-mm-Fußplatten mit je zwei 18-mm-Ankerlöchern unter dem Bodenschnitt, die verbliebenen Schenkel der abgebrannten Laschen L 100 x 100 x 10 am Kopf, 56 Schraubenköpfe und Muttern M12; vierzehn gekantete Tränenblechwannen 5 mm (260 x 40 mm) mit 8-mm-Seitenwangen als je ein Körper, entlang der Lauflinie wiederholt; drei seitlich angeschraubte Pfosten 42,4 mm mit sechs Pfostenlaschen, der Handlauf 42,4 mm und die Knieleiste 33,7 mm; vier Kettenstränge 8 mm als schlanke 18-mm-Stäbe, vier Bügelschäkel und der 16-mm-Aufhängering; zwei Unterleger 120 x 100 x 1100 mm ({n_parts[50]} Körper)."),
            T("Thermal: every body solid by reference in one of five regions; the bolts, plates and cleats belong to the stringer steel, the cheeks to the tread plate, the post shoes to the handrail, the shackles to the sling.", "Thermik: jeder Körper per Verweis in einer von fünf Regionen; Schrauben, Platten und Laschen gehören zum Wangenstahl, die Seitenwangen zum Stufenblech, die Pfostenlaschen zum Handlauf, die Schäkel zum Gehänge."),
            T("Structure: web and flanges are the member of each stringer bone, the treads the secondary members that carry the load into the cheeks and bolts (joints), base plates and cleats the joints where the supports of the reinstalled pose act; the bearers, chain legs and master link are members of their bones, the shackles the joints of the legs; the handrail tubes are members of the handrail bone with a mass marker at its centre of mass on the knee rail.", "Tragwerk: Steg und Flansche sind der Stab jedes Wangenknochens, die Stufen die Nebenträger, die die Last in die Seitenwangen und Schrauben (Gelenke) tragen, Fußplatten und Laschen die Gelenke, an denen die Auflager der Einbaupose wirken; Unterleger, Kettenstränge und Aufhängering sind Stäbe ihrer Knochen, die Schäkel die Gelenke der Stränge; die Handlaufrohre sind Stäbe des Handlaufknochens mit einer Massenmarke im Schwerpunkt auf der Knieleiste."),
        ]
    if scale == 100:
        return [
            T(f"1:100 keeps the open channel with web and flanges at the 15 mm feature (the 60 mm opening is four features), base plates with their 18 mm anchor holes and cleats at 15 mm reaching into the stringer ends, fourteen tread blocks of 260 x 40 mm spanning between the webs (pan, folded edges and cheeks merged), the three posts, handrail and knee rail at their true diameters, the chain legs and the 16 mm master link; bolts, nuts, empty holes, shackles and post shoes are below the feature and left out ({n_parts[100]} solids).", f"1:100 behält das offene U-Profil mit Steg und Flanschen auf dem 15-mm-Merkmal (die 60-mm-Öffnung sind vier Merkmale), Fußplatten mit ihren 18-mm-Ankerlöchern und Laschen mit 15 mm, die in die Wangenenden reichen, vierzehn Stufenblöcke 260 x 40 mm zwischen den Stegen (Wanne, Abkantungen und Seitenwangen verschmolzen), die drei Pfosten, Handlauf und Knieleiste in wahren Durchmessern, die Kettenstränge und den 16-mm-Aufhängering; Schrauben, Muttern, leere Löcher, Schäkel und Pfostenlaschen liegen unter dem Merkmal und entfallen ({n_parts[100]} Körper)."),
            T("Thermal: the body by reference in the five regions; per-region areas within 30 % of the 1:50 values.", "Thermik: der Körper per Verweis in den fünf Regionen; Flächen je Region innerhalb von 30 % der Werte bei 1:50."),
            T("Structure: web and flanges as stringer members, treads as secondary members, plates and cleats as joints, bearers, legs and master link as members, the handrail tubes with their mass marker.", "Tragwerk: Steg und Flansche als Wangenstäbe, Stufen als Nebenträger, Platten und Laschen als Gelenke, Unterleger, Stränge und Aufhängering als Stäbe, die Handlaufrohre mit ihrer Massenmarke."),
        ]
    if scale == 200:
        return [
            T(f"1:200 merges each channel into one 200 x 75 mm bar from the floor (base plate in its foot) to the landing face (cleat in its head), keeps the fourteen treads as 251 x 36 mm blocks (the 5 mm pan and its folds generalised to a 36 mm block that prints edge-on at 1:350, the depth shortened by 9 mm so the open risers stay printable at 1:350) with the open risers between them, the posts, handrail and knee rail at their true 42.4 and 33.7 mm (above the 30 mm feature; the rails slender), the four chain legs as slender rods with the master link merged into the end of the foot-end leg ({n_parts[200]} solids).", f"1:200 verschmilzt jedes U-Profil zu einem Balken 200 x 75 mm vom Boden (Fußplatte in seinem Fuß) bis zur Podeststirn (Lasche in seinem Kopf), behält die vierzehn Stufen als Blöcke 251 x 36 mm (die 5-mm-Wanne und ihre Abkantungen zu einem 36-mm-Block verallgemeinert, der hochkant bei 1:350 druckt, die Tiefe um 9 mm gekürzt, damit die offenen Setzstufen bei 1:350 druckbar bleiben) mit den offenen Setzstufen dazwischen, Pfosten, Handlauf und Knieleiste in wahren 42,4 und 33,7 mm (über dem 30-mm-Merkmal; die Holme schlank), die vier Kettenstränge als schlanke Stäbe, der Aufhängering in das Ende des fußseitigen Strangs eingeschmolzen ({n_parts[200]} Körper)."),
            T("Thermal: the body by reference; at this scale only the total of the regions is compared.", "Thermik: der Körper per Verweis; bei diesem Maßstab wird nur die Summe der Regionen verglichen."),
            T("Structure: the bars as stringer members, treads as secondary members, bearers, legs and handrail tubes as members, the handrail mass marker of the 30 mm feature.", "Tragwerk: die Balken als Wangenstäbe, Stufen als Nebenträger, Unterleger, Stränge und Handlaufrohre als Stäbe, die Handlauf-Massenmarke im 30-mm-Merkmal."),
        ]
    return [
        T(f"1:500 is the flight as one bar of the stringer outline, 0.95 m wide, from the floor to the landing face (treads, plates and cleats merged: the 80 mm open risers are about one feature), the posts, handrail and knee rail as slender tubes of their true diameters, the four chain legs as slender rods (master link merged into the foot-end leg), the two bearers ({n_parts[500]} solids).", f"1:500 ist der Lauf ein Balken der Wangenkontur, 0,95 m breit, vom Boden bis zur Podeststirn (Stufen, Platten und Laschen verschmolzen: die 80-mm-Öffnungen sind etwa ein Merkmal), Pfosten, Handlauf und Knieleiste als schlanke Rohre in wahren Durchmessern, die vier Kettenstränge als schlanke Stäbe (Aufhängering in den fußseitigen Strang eingeschmolzen), die beiden Unterleger ({n_parts[500]} Körper)."),
        T("Thermal: the bar is split into its two 75 mm stringer bands (stringer steel) and the band between them (tread plate); handrail, bearers and chain by reference; only the total is compared.", "Thermik: der Balken ist in seine beiden 75-mm-Wangenbänder (Wangenstahl) und das Band dazwischen (Stufenblech) geteilt; Handlauf, Unterleger und Kette per Verweis; nur die Summe wird verglichen."),
        T("Structure: the two stringer bands as the members of the stringer bones, the band between them as the treads, bearers, legs and tubes as members, the handrail mass marker of the 75 mm feature.", "Tragwerk: die beiden Wangenbänder als Stäbe der Wangenknochen, das Band dazwischen als die Stufen, Unterleger, Stränge und Rohre als Stäbe, die Handlauf-Massenmarke im 75-mm-Merkmal."),
    ]


representations = {}
volumes = {}
for scale in (50, 100, 200, 500):
    lo, hi = RANGES[scale]
    representations[str(scale)] = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES[scale], "geometry": {"elevation": elevation(scale), "front": front_plan(scale, "front"), "plan": front_plan(scale, "plan")}, "decisions": rep_decisions(scale)}
    body, thermal, structure, _ = BODY[scale]
    volumes[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": vol_decisions(scale)}

summary = T(
    f"A dismantled industrial steel stair flight for a storey of 2.80 m: two UPN 200 channel stringers ({U_TOP:.2f} m long, St 37-2), fourteen folded 5/7 mm chequer-plate treads of 800 x 260 mm bolted through welded cheeks, fifteen risers of 186.7 mm on goings of 250 mm (36.7 degrees, 2h + g = 623 mm), a side-mounted 42.4 mm handrail with knee rail on the left; the top cleats are flame-cut, the right handrail was removed (its empty holes remain), the treads are worn bare on the walking line and the grey paint has turned to rust patina. It lies on two bearers, is lifted on a four-leg chain sling and reinstalled with a walking person and the T2 stair load.",
    f"Ein rückgebauter Industrie-Stahltreppenlauf für ein Geschoss von 2,80 m: zwei Wangen aus U-Profil UPN 200 ({U_TOP:.2f} m lang, St 37-2), vierzehn gekantete Tränenblechstufen 5/7 mm von 800 x 260 mm, durch angeschweißte Seitenwangen verschraubt, fünfzehn Steigungen von 186,7 mm auf Auftritten von 250 mm (36,7 Grad, 2s + a = 623 mm), links ein seitlich angeschraubter Handlauf 42,4 mm mit Knieleiste; die Laschen oben sind abgebrannt, der rechte Handlauf ist entfernt (seine leeren Löcher bleiben), die Stufen sind auf der Lauflinie blank gelaufen und der graue Anstrich ist zu Rostpatina geworden. Er liegt auf zwei Unterlegern, wird an einem Vierstrang-Kettengehänge gehoben und mit einer gehenden Person und der Treppenlast T2 wieder eingebaut.",
)

thermal_model = {
    "environment": {"airTempC": 15, "meanRadiantTempC": 15, "skyTempC": 2, "groundTempC": 14, "relativeHumidity": 0.65, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 600, "diffuseFraction": 0.25, "pressurePa": 101325},
    "passengers": [],
    "operating": [],
    "regions": regions,
    "emitters": [],
    "assumptions": [
        T("All surfaces are passive: the flight has no device and no heat source, its total power is 0 W.", "Alle Flächen sind passiv: der Lauf hat kein Gerät und keine Wärmequelle, seine Gesamtleistung ist 0 W."),
        T("Weathered surfaces differ from new ones: the grey primer turned to rust patina absorbs about 0.75 of the sun (new grey paint about 0.6), the treads worn to bare rusty steel 0.82, the faded yellow handrail 0.50; emissivities 0.85 to 0.90 (authored estimates guided by ASHRAE Fundamentals).", "Verwitterte Flächen unterscheiden sich von neuen: die zu Rostpatina gewordene graue Grundierung absorbiert etwa 0,75 der Sonne (neuer grauer Anstrich etwa 0,6), die zu blankem Rost abgelaufenen Stufen 0,82, der verblasste gelbe Handlauf 0,50; Emissionsgrade 0,85 bis 0,90 (Schätzungen nach ASHRAE Fundamentals)."),
        T(f"Areal heat capacities follow the steel per surface area: UPN 200 25.3 kg/m on 0.66 m2/m, a {tread_mass:.1f} kg tread on 0.59 m2, a 42.4 mm tube 2.55 kg/m on 0.133 m2/m; the timber bearers store heat in half the 24 h penetration depth (EN ISO 13786).", f"Die flächenbezogenen Wärmekapazitäten folgen dem Stahl je Oberfläche: UPN 200 25,3 kg/m auf 0,66 m²/m, eine Stufe von {str(round(tread_mass, 1)).replace('.', ',')} kg auf 0,59 m², ein Rohr 42,4 mm 2,55 kg/m auf 0,133 m²/m; die Holz-Unterleger speichern Wärme in der halben 24-h-Eindringtiefe (EN ISO 13786)."),
        T("Stored and lifted in the open yard on a mild sunny day; reinstalled indoors with only a little sun through a window; the bearers lie on the yard floor with about a quarter of their surface in contact.", "Gelagert und angehoben im offenen Lager an einem milden sonnigen Tag; wieder eingebaut im Inneren mit nur wenig Sonne durch ein Fenster; die Unterleger liegen mit etwa einem Viertel ihrer Oberfläche auf dem Lagerplatz auf."),
    ],
}

structure_model = {
    "gravityMS2": 9.81,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        T("The load path changes with the pose, so every support is a pose contact: stored, each stringer rests on both bearers (four unilateral contacts at the bottom flanges); lifted, the four shackles in the top flanges hold the flight (pins: the sling legs carry the load to the master link); reinstalled, the base plates are pinned to the floor and the cut cleats bear on a new angle at the landing (rollers with a vertical reaction).", "Der Lastpfad wechselt mit der Pose, daher ist jedes Auflager ein Posenkontakt: gelagert liegt jede Wange auf beiden Unterlegern (vier einseitige Kontakte an den Unterflanschen); angehoben halten die vier Schäkel in den Oberflanschen den Lauf (Gelenke: die Kettenstränge tragen die Last zum Aufhängering); wieder eingebaut sind die Fußplatten am Boden festgelegt und die abgetrennten Laschen liegen auf einem neuen Winkel am Podest auf (Rollenlager mit senkrechter Auflagerkraft)."),
        T(f"Each stringer is an inclined simply supported UPN 200 (A 32.2 cm2, Iy 1910 cm4, E 210 GPa, St 37-2) along its centroid axis, {LS:.3f} m long, horizontal span {LS * CA:.3f} m; it carries its own 25.3 kg/m, half of the fourteen treads ({tread_mass:.1f} kg each with cheeks and bolts), its base plate, cleat and shackles as a uniform mass of {stringer_mass + 2 * shackle_mass:.1f} kg.", f"Jede Wange ist ein geneigter, frei aufliegender Träger UPN 200 (A 32,2 cm², Iy 1910 cm⁴, E 210 GPa, St 37-2) entlang ihrer Schwerachse, {LS:.3f} m lang, waagrechte Stützweite {LS * CA:.3f} m; sie trägt ihre eigenen 25,3 kg/m, die Hälfte der vierzehn Stufen (je {str(round(tread_mass, 1)).replace('.', ',')} kg mit Seitenwangen und Schrauben), Fußplatte, Lasche und Schäkel als gleichmäßige Masse von {stringer_mass + 2 * shackle_mass:.1f} kg."),
        T("Loads on the treads reach both stringers in equal halves (a person on the walking line, the T2 load 5.0 kN/m2 over the 0.80 m clear width); torsion from loads beside the centre line and the bending of the treads themselves are not evaluated.", "Lasten auf den Stufen erreichen beide Wangen je zur Hälfte (eine Person auf der Lauflinie, die Last T2 5,0 kN/m² über die lichte Breite von 0,80 m); Torsion aus Lasten neben der Mittellinie und die Biegung der Stufen selbst werden nicht ausgewertet."),
        T(f"The solver treats the asset as one rigid body: when the flight is lifted or reinstalled, the {2 * bearer_mass:.1f} kg of the two bearers left on the yard floor are carried by the supports of the flight in the model (about {100 * 2 * bearer_mass / MASS_TOTAL:.1f} % of the weight).", f"Der Löser behandelt das Objekt als einen starren Körper: ist der Lauf angehoben oder wieder eingebaut, tragen im Modell die Auflager des Laufs auch die {2 * bearer_mass:.1f} kg der beiden auf dem Lagerplatz liegenden Unterleger (etwa {100 * 2 * bearer_mass / MASS_TOTAL:.1f} % des Gewichts)."),
    ],
    "clinical": False,
}

asset = {
    "id": "reclaimed-steel-stair",
    "category": "components",
    "title": T("Reclaimed Steel Stair Flight", "Gebrauchter Stahltreppenlauf"),
    "summary": summary,
    "tags": ["component", "reclaimed", "structural", "stair", "stringer", "steel", "chequer-plate", "industrial"],
    "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": references, "notes": T(f"Storey 2.80 m in 15 risers of {RISE * 1000:.1f} mm on 14 goings of 250 mm (pitch {math.degrees(ALPHA):.2f} degrees), clear width 0.80 m between the stringer webs; stored length {dimensions['widthM']:.3f} m including the handrail overhang, mass {dimensions['massKg']:.1f} kg including bearers and sling.", f"Geschoss 2,80 m in 15 Steigungen von {RISE * 1000:.1f} mm auf 14 Auftritten von 250 mm (Neigung {math.degrees(ALPHA):.2f} Grad), lichte Breite 0,80 m zwischen den Wangenstegen; Lagerlänge {dimensions['widthM']:.3f} m einschließlich Handlaufüberstand, Masse {dimensions['massKg']:.1f} kg einschließlich Unterlegern und Gehänge.")},
    "dimensions": dimensions,
    "materials": materials,
    "rig": {"kind": "structural", "root": "yard", "bones": bones},
    "volumes": volumes,
    "anchors": anchors,
    "representations": representations,
    "symbols": [],
    "poses": poses,
    "clips": clips,
    "thermal": thermal_model,
    "structure": structure_model,
    "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
with open(OUT, "w") as fh:
    json.dump(doc, fh, ensure_ascii=False, indent=1)
print(f"[DEBUG] wrote {OUT}: dims {dimensions}, alpha {math.degrees(ALPHA):.3f}, U_TOP {U_TOP:.4f}, LS {LS:.4f}, legs {({k: round(v, 3) for k, v in lens.items()})}, apex {[round(a, 3) for a in apex]}, solids {n_parts}")


