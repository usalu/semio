"""🪑 Round 3, furniture/cafe-chair: the measures of the real chair (bistro café chair: Ø22 x 2 mm steel tube frame whose rear legs bend into the back posts, two front legs, two Ø16 side stretchers, a 20 mm beech seat board on an 18 x 45 mm beech apron frame, a curved 18 mm beech backrest board and a lower rail screwed to the post fronts, nylon glides, plastic post caps) and the helpers that turn world coordinates into bone-local solids.

World frame: x ahead (the front of the chair is +x), y up, z lateral (left of the chair is +). Metres, degrees. The chair is symmetric about z = 0.
"""
import math

R = 0.011
RS = 0.008
BEND_R = 0.08
GLIDE_R, GLIDE_H = 0.0125, 0.010
CAP_H = 0.008
SLAB500_V0, SLAB500_V1 = -0.060, 0.015

REAR_FOOT = (0.0, 0.0)
REAR_FOOT_Z = 0.200
CORNER = (0.060, 0.445)
POST_Z = 0.185
POST_DEG, POST_L = 98.0, 0.433

FRONT_FOOT = (0.455, 0.0)
FRONT_DEG = 92.5
FRONT_FOOT_Z, FRONT_TOP_Z = 0.210, 0.190

SEAT_O = (0.045, 0.452)
SEAT_DEG, SEAT_L = 2.0, 0.43
BOARD_T = 0.020
APRON_V0, APRON_V1 = -0.065, -0.020
APRON_T = 0.018
APRON_W1 = 0.184
APRON_W0 = APRON_W1 - APRON_T
SEAT_FRONT_W, SEAT_REAR_W = 0.215, 0.199
NOTCH_W, NOTCH_U = 0.172, 0.030
SEAT_CORNER = 0.03

STRETCH_Y = 0.15

BOARD_U0, BOARD_U1 = 0.245, 0.415
RAIL_U0, RAIL_U1 = 0.115, 0.165
BACK_T = 0.018
BACK_W = 0.205
BACK_VC_END, BACK_VC_MID = -0.0195, 0.006

BEECH, STEEL, NYLON = "beech-timber", "steel-tube", "polyamide-glide"
TONE = {BEECH: 0.6, STEEL: 0.3, NYLON: 0.15}
SIDES = (("l", 1), ("r", -1))

TUBE_KG_M = math.pi / 4 * (0.022 ** 2 - 0.018 ** 2) * 7850
STRETCH_KG_M = math.pi / 4 * (0.016 ** 2 - 0.012 ** 2) * 7850


def r6(value):
    """🔢 Rounds to a hundredth of a millimetre."""
    return round(value + 0.0, 5)


def unit(v):
    """📏 The unit vector of v."""
    n = math.sqrt(sum(c * c for c in v))
    return tuple(c / n for c in v)


def deg(a):
    """🧭 The unit direction of an angle in degrees."""
    return (math.cos(math.radians(a)), math.sin(math.radians(a)))


def add(p, d, t):
    """➕ p + t d."""
    return tuple(a + t * b for a, b in zip(p, d))


REAR_DIR = unit((CORNER[0] - REAR_FOOT[0], CORNER[1] - REAR_FOOT[1]))
REAR_DEG = math.degrees(math.atan2(REAR_DIR[1], REAR_DIR[0]))
REAR_L = math.hypot(CORNER[0] - REAR_FOOT[0], CORNER[1] - REAR_FOOT[1])
POST_DIR = deg(POST_DEG)
TURN = math.radians(POST_DEG - REAR_DEG)
TANGENT = BEND_R * math.tan(TURN / 2)
ARC_A = add(CORNER, REAR_DIR, -TANGENT)
ARC_B = add(CORNER, POST_DIR, TANGENT)
ARC_O = add(ARC_A, (-REAR_DIR[1], REAR_DIR[0]), BEND_R)
POST_TOP = add(CORNER, POST_DIR, POST_L)
FRONT_DIR = deg(FRONT_DEG)
SEAT_DIR = deg(SEAT_DEG)


def seat_uv(p):
    """🪑 World (x, y) in the seat frame (u along the seat towards the front edge, v up)."""
    dx, dy = p[0] - SEAT_O[0], p[1] - SEAT_O[1]
    return (dx * SEAT_DIR[0] + dy * SEAT_DIR[1], -dx * SEAT_DIR[1] + dy * SEAT_DIR[0])


def seat_xy(u, v):
    """🪑 Seat frame (u, v) in world (x, y)."""
    return (SEAT_O[0] + u * SEAT_DIR[0] - v * SEAT_DIR[1], SEAT_O[1] + u * SEAT_DIR[1] + v * SEAT_DIR[0])


def _front_len():
    """🦵 The length of the front leg axis from the floor to 2 mm inside the seat board."""
    lo, hi = 0.3, 0.6
    for _ in range(60):
        mid = (lo + hi) / 2
        if seat_uv(add(FRONT_FOOT, FRONT_DIR, mid))[1] < -BOARD_T + 0.002:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


FRONT_L = _front_len()
FRONT_TOP = add(FRONT_FOOT, FRONT_DIR, FRONT_L)


def front_axis(y):
    """🦵 The front leg axis point (x, y, |z|) at height y."""
    t = y / FRONT_DIR[1]
    return (FRONT_FOOT[0] + t * FRONT_DIR[0], y, FRONT_FOOT_Z + (FRONT_TOP_Z - FRONT_FOOT_Z) * t / FRONT_L)


def rear_axis(y):
    """🦿 The rear leg axis point (x, y, |z|) at a height y on the straight leg (below the bend)."""
    t = y / REAR_DIR[1]
    la = math.hypot(ARC_A[0], ARC_A[1])
    return (REAR_FOOT[0] + t * REAR_DIR[0], y, REAR_FOOT_Z + (POST_Z - REAR_FOOT_Z) * min(t, la) / la)


def arc(alpha):
    """↪ The point and tangent of the bend at the turned angle alpha (0 at the leg end, TURN at the post end)."""
    a0 = math.atan2(ARC_A[1] - ARC_O[1], ARC_A[0] - ARC_O[0])
    a = a0 + alpha
    return (ARC_O[0] + BEND_R * math.cos(a), ARC_O[1] + BEND_R * math.sin(a)), (-math.sin(a), math.cos(a))


STRETCH_REAR = rear_axis(STRETCH_Y)
STRETCH_FRONT = front_axis(STRETCH_Y)
STRETCH_L = STRETCH_FRONT[0] - STRETCH_REAR[0]
U_FRONT = seat_uv(front_axis(0.41))[0]
U_POST = seat_uv(rear_axis(0.41))[0]

ORIGIN = {
    "ground": (0.0, 0.0, 0.0),
    "leg-rear": (REAR_FOOT[0], REAR_FOOT[1], REAR_DEG),
    "leg-front": (FRONT_FOOT[0], FRONT_FOOT[1], FRONT_DEG),
    "stretcher": (STRETCH_REAR[0], STRETCH_Y, 0.0),
    "seat": (SEAT_O[0], SEAT_O[1], SEAT_DEG),
    "back": (CORNER[0], CORNER[1], POST_DEG),
}


def local(bone, p):
    """📐 A world point (x, y, z) in the rest frame of a bone (u, v, w)."""
    ox, oy, a = ORIGIN[bone]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    dx, dy = p[0] - ox, p[1] - oy
    return (dx * c + dy * s, -dx * s + dy * c, p[2])


def local_dir(bone, d):
    """🧭 A world direction (x, y, z) in the rest frame of a bone."""
    a = ORIGIN[bone][2]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return (d[0] * c + d[1] * s, -d[0] * s + d[1] * c, d[2])


def world(bone, q):
    """🌍 A bone-local point (u, v, w) in world (x, y, z)."""
    ox, oy, a = ORIGIN[bone]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return (ox + q[0] * c - q[1] * s, oy + q[0] * s + q[1] * c, q[2])


def quat_normal(n):
    """🔄 The unit quaternion that turns the bone-local z axis onto the direction n (a section plane normal), x kept as close to w as possible."""
    n = unit(n)
    ref = (0.0, 0.0, 1.0) if abs(n[2]) < 0.9 else (1.0, 0.0, 0.0)
    d = sum(a * b for a, b in zip(ref, n))
    x = unit(tuple(a - d * b for a, b in zip(ref, n)))
    y = (n[1] * x[2] - n[2] * x[1], n[2] * x[0] - n[0] * x[2], n[0] * x[1] - n[1] * x[0])
    m = ((x[0], y[0], n[0]), (x[1], y[1], n[1]), (x[2], y[2], n[2]))
    tr = m[0][0] + m[1][1] + m[2][2]
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        q = ((m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, 0.25 * s)
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1.0 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = (0.25 * s, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s)
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1.0 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = ((m[0][1] + m[1][0]) / s, 0.25 * s, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s)
    else:
        s = math.sqrt(1.0 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = ((m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, 0.25 * s, (m[1][0] - m[0][1]) / s)
    return [r6(c) for c in q]


def base(sid, kind, bone, material, tone=None, role="body"):
    """🏷 The common members of a solid."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone}


def tube(sid, bone, stations, radius, material=STEEL, tone=None, role="body", **more):
    """🧪 A round tube as a loft through world stations ((x, y, z), tangent (x, y, z)); every section is turned explicitly perpendicular to its tangent so that tubes of two bones meet seamlessly."""
    out = base(sid, "loft", bone, material, tone, role)
    out["sections"] = [{"at": [r6(c) for c in local(bone, p)], "rx": r6(radius), "ry": r6(radius), "rotation": quat_normal(local_dir(bone, t))} for p, t in stations]
    out["capStart"] = True
    out["capEnd"] = True
    out.update(more)
    return out


def capsule(sid, bone, p0, p1, r0, r1=None, material=STEEL, tone=None, role="body", **more):
    """💊 A capsule between two world points."""
    out = base(sid, "capsule", bone, material, tone, role)
    out.update({"p0": [r6(c) for c in local(bone, p0)], "p1": [r6(c) for c in local(bone, p1)], "r0": r6(r0), "r1": r6(r0 if r1 is None else r1)})
    out.update(more)
    return out


def box(sid, bone, material, centre_local, size, radius=0.0, tone=None, role="body", **more):
    """📦 A box at a bone-local centre."""
    out = base(sid, "box", bone, material, tone, role)
    out["size"] = [r6(s) for s in size]
    if radius:
        out["cornerRadius"] = r6(radius)
    out["transform"] = {"translation": [r6(c) for c in centre_local]}
    out.update(more)
    return out


def ellipsoid(sid, bone, material, centre_world, radii, tone=None, role="body", **more):
    """🥚 An ellipsoid at a world centre with semi-axes along the bone axes."""
    out = base(sid, "ellipsoid", bone, material, tone, role)
    out["radii"] = [r6(r) for r in radii]
    out["transform"] = {"translation": [r6(c) for c in local(bone, centre_world)]}
    out.update(more)
    return out


def revolve(sid, bone, material, profile, axis_world, origin_world, tone=None, role="body", **more):
    """⭕ A revolve of an (axial, radial) profile about a world axis through a world origin."""
    out = base(sid, "revolve", bone, material, tone, role)
    out.update({"profile": [[r6(a), r6(r)] for a, r in profile], "axis": [r6(c) for c in local_dir(bone, axis_world)], "origin": [r6(c) for c in local(bone, origin_world)]})
    out.update(more)
    return out


def plate(sid, bone, material, outline_wu, v0, v1, tone=None, role="body", holes=None, **more):
    """⬛ A plate of the seat: an outline of (w, u) points pushed along v from v0 to v1."""
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(w), r6(u)] for w, u in outline_wu], "plane": "zx", "from": r6(v0), "to": r6(v1)})
    if holes:
        out["holes"] = [[[r6(w), r6(u)] for w, u in h] for h in holes]
    out.update(more)
    return out


def back_vc(w):
    """🌙 The centre line of the curved back board in the back frame: v (backwards positive) at lateral w; concave towards the sitter, touching the post fronts."""
    b = (BACK_VC_END - BACK_VC_MID) / POST_Z ** 2
    return BACK_VC_MID + b * w * w


def curved(sid, u0, u1, thick, half_w, n, exponent=6, vc=back_vc, tone=None, **more):
    """🌙 A curved beech board of the back between heights u0 and u1 (along the posts), as a loft along w through n sections."""
    out = base(sid, "loft", "back", BEECH, tone)
    um = (u0 + u1) / 2
    out["sections"] = [{"at": [r6(um), r6(vc(w)), r6(w)], "rx": r6((u1 - u0) / 2), "ry": r6(thick / 2), "exponent": exponent} for w in [-half_w + 2 * half_w * k / (n - 1) for k in range(n)]]
    out["capStart"] = True
    out["capEnd"] = True
    out.update(more)
    return out


def rear_stations(fine):
    """🦿 The stations of one rear tube from the glide up to the middle of the bend (leg-rear part) and from there to the top (back part), each ((x, y), tangent); z is added by the caller."""
    foot = add(REAR_FOOT, REAR_DIR, 0.005 / REAR_DIR[1])
    leg = [(foot, REAR_DIR), (ARC_A, REAR_DIR)]
    steps = 4 if fine else 2
    for k in range(1, steps + 1):
        leg.append(arc(TURN / 2 * k / steps))
    post = [arc(TURN / 2)]
    for k in range(1, steps + 1):
        post.append(arc(TURN / 2 + TURN / 2 * k / steps))
    post.append((POST_TOP, POST_DIR))
    return leg, post


def with_z(stations, side, z_of):
    """↔ Stations ((x, y), t) with the lateral z of the side; tangents get the lateral slope."""
    out = []
    for i, (p, t) in enumerate(stations):
        z = z_of(p)
        dz = 0.0
        if i + 1 < len(stations):
            q = stations[i + 1][0]
            ds = math.hypot(q[0] - p[0], q[1] - p[1])
            dz = (z_of(q) - z) / ds if ds else 0.0
        elif i:
            q = stations[i - 1][0]
            ds = math.hypot(q[0] - p[0], q[1] - p[1])
            dz = (z - z_of(q)) / ds if ds else 0.0
        out.append(((p[0], p[1], side * z), (t[0], t[1], side * dz)))
    return out


def rear_z(p):
    """🦿 The lateral |z| of the rear tube centre at a station: splayed to the foot below the bend, constant along the post."""
    la = math.hypot(ARC_A[0] - REAR_FOOT[0], ARC_A[1] - REAR_FOOT[1])
    s = math.hypot(p[0] - REAR_FOOT[0], p[1] - REAR_FOOT[1])
    if p[1] >= ARC_A[1]:
        return POST_Z
    return REAR_FOOT_Z + (POST_Z - REAR_FOOT_Z) * s / la


def front_z(p):
    """🦵 The lateral |z| of the front tube centre at a station."""
    return front_axis(p[1])[2]


def seat_outline(corner_steps=4, notch=True, corner=SEAT_CORNER):
    """⬜ The plan of the seat board as (w, u) points: trapezoid, rounded front corners, notched round the posts at the rear."""
    if corner_steps:
        arc_pts = [(SEAT_FRONT_W - corner + corner * math.sin(math.pi / 2 * k / corner_steps), SEAT_L - corner + corner * math.cos(math.pi / 2 * k / corner_steps)) for k in range(corner_steps + 1)]
    else:
        arc_pts = [(SEAT_FRONT_W, SEAT_L)]
    rear = [(SEAT_REAR_W, NOTCH_U), (NOTCH_W, NOTCH_U), (NOTCH_W, 0.0)] if notch else [(SEAT_REAR_W, 0.0)]
    half = arc_pts + rear
    return half + [(-w, u) for w, u in reversed(half)]


def plan_area(pts):
    """📐 Shoelace area and centroid of a polygon."""
    a = cx = cy = 0.0
    for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1]):
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return abs(a), cx / (6 * a), cy / (6 * a)


def masses():
    """⚖ Mass (kg) and centre (bone-local u, v) of every bone from the modelled parts: steel 7850, beech 720, polyamide 1150 kg/m3."""
    beech, nylon = 720.0, 1150.0
    glide = math.pi * GLIDE_R ** 2 * GLIDE_H * nylon
    cap = math.pi * R ** 2 * CAP_H * 0.6 * nylon
    leg, post = rear_stations(True)
    rear_len = math.hypot(ARC_A[0], ARC_A[1]) + BEND_R * TURN / 2
    post_len = BEND_R * TURN / 2 + POST_L - TANGENT
    rear_kg = 2 * (rear_len * TUBE_KG_M + glide)
    rear_c = (local("leg-rear", (ARC_A[0] / 2, ARC_A[1] / 2, 0))[0], 0.0)
    front_kg = 2 * (FRONT_L * TUBE_KG_M + glide)
    stretch_kg = 2 * STRETCH_L * STRETCH_KG_M
    area, cw, cu = plan_area(seat_outline())
    board = area * BOARD_T * beech
    side = 2 * (U_FRONT - U_POST - 0.008) * (APRON_V1 - APRON_V0) * APRON_T * beech
    cross = 2 * (2 * APRON_W0) * (APRON_V1 - APRON_V0) * APRON_T * beech
    seat_kg = board + side + cross
    seat_u = (board * cu + side * (U_FRONT + U_POST) / 2 + cross * (U_FRONT + U_POST) / 2) / seat_kg
    seat_v = (board * -BOARD_T / 2 + (side + cross) * (APRON_V0 + APRON_V1) / 2) / seat_kg
    posts = 2 * (post_len * TUBE_KG_M + cap)
    width = 2 * BACK_W * 1.01
    board_b = width * (BOARD_U1 - BOARD_U0) * BACK_T * beech
    rail_b = width * (RAIL_U1 - RAIL_U0) * BACK_T * beech
    vbar = sum(back_vc(-BACK_W + 2 * BACK_W * k / 40) for k in range(41)) / 41
    back_kg = posts + board_b + rail_b
    back_u = (posts * POST_L / 2 + board_b * (BOARD_U0 + BOARD_U1) / 2 + rail_b * (RAIL_U0 + RAIL_U1) / 2) / back_kg
    back_v = ((board_b + rail_b) * vbar) / back_kg
    return {
        "leg-rear": (rear_kg, rear_c[0], 0.0),
        "leg-front": (front_kg, FRONT_L / 2, 0.0),
        "stretcher": (stretch_kg, STRETCH_L / 2, 0.0),
        "seat": (seat_kg, seat_u, seat_v),
        "back": (back_kg, back_u, back_v),
        "parts": {"board": board, "rails": side + cross, "posts": posts, "back-board": board_b, "back-rail": rail_b, "glide": glide},
    }


def rig(old):
    """🦴 The corrected rig: rear legs raked back to a bend at the seat, back posts reclined 8 degrees from the bend, front legs raked 2.5 degrees, the seat raked 2 degrees from its rear edge, the stretcher at 0.15 m between the leg axes, the floor reference without length; masses and centres from the modelled parts."""
    m = masses()
    geometry = {
        "ground": dict(x=0, y=0, angleDeg=0, lengthM=0),
        "leg-rear": dict(x=r6(REAR_FOOT[0]), y=r6(REAR_FOOT[1]), angleDeg=round(REAR_DEG, 3), lengthM=round(REAR_L, 4)),
        "leg-front": dict(x=r6(FRONT_FOOT[0]), y=r6(FRONT_FOOT[1]), angleDeg=FRONT_DEG, lengthM=round(FRONT_L, 4)),
        "stretcher": dict(x=round(STRETCH_REAR[0], 4), y=STRETCH_Y, angleDeg=0, lengthM=round(STRETCH_L, 4)),
        "seat": dict(x=SEAT_O[0], y=SEAT_O[1], angleDeg=SEAT_DEG, lengthM=SEAT_L),
        "back": dict(x=CORNER[0], y=CORNER[1], angleDeg=POST_DEG, lengthM=POST_L),
    }
    bones = []
    for bone in old["bones"]:
        bone = dict(bone)
        bone.update(geometry[bone["id"]])
        if bone["id"] in m and "mass" in bone:
            kg, cu, cv = m[bone["id"]]
            bone["mass"] = dict(bone["mass"], kg=round(kg, 2), comX=round(cu, 3), comY=round(cv, 3))
        bones.append(bone)
    return dict(old, bones=bones)


if __name__ == "__main__":
    m = masses()
    print({k: (round(v[0], 3), round(v[1], 4), round(v[2], 4)) for k, v in m.items() if k != "parts"})
    print({k: round(v, 3) for k, v in m["parts"].items()}, "total", round(sum(v[0] for k, v in m.items() if k != "parts"), 2))
    print("rear", round(REAR_DEG, 3), round(REAR_L, 4), "turn", round(math.degrees(TURN), 2), "front L", round(FRONT_L, 4), "top", [round(c, 4) for c in FRONT_TOP], "post top", [round(c, 4) for c in POST_TOP])
    print("stretch", [round(c, 4) for c in STRETCH_REAR], [round(c, 4) for c in STRETCH_FRONT], round(STRETCH_L, 4), "u front", round(U_FRONT, 4), "u post", round(U_POST, 4))
    print("seat front top", [round(c, 4) for c in seat_xy(SEAT_L, 0)], "seat rear bottom", [round(c, 4) for c in seat_xy(0, -BOARD_T)])
