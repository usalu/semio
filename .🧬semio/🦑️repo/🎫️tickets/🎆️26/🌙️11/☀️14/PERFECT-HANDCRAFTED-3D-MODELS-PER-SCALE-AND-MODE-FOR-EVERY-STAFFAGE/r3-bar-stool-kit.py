"""🪑 Round 3, furniture/bar-stool: the measured construction of the stool and the bookkeeping from world metres (x ahead, y up, z left, rest pose) to bone-local solids and drawing coordinates.

Every number of the construction is stated here once by hand; `r3-bar-stool-model.py` (solids) and `r3-bar-stool-draw.py` (2D) read them.
"""
import math

SEAT_JOINT_Y = 0.698
PAD_JOINT_Y = 0.72
BACK_JOINT = (-0.14, 0.694)

FOOT_R = 0.1975
GLIDE_H = 0.022
GLIDE_R = 0.015
TUBE_R = 0.0125
LEG_TOP = (0.144, 0.6705)
LEG_FOOT = (FOOT_R, 0.014)
RING_Y = 0.27
RING_R = 0.2
PAN_R = 0.16
PAN_TOP = 0.686
PAN_RIM = 0.020
PAN_T = 0.002
SWIVEL_R = 0.085
SWIVEL_RACE_R = 0.072
SEAT_R = 0.18
BOARD_R = 0.177
BOARD_T = 0.012
SEAT_TOP = 0.752
COVER_T = 0.0015
BAR_W = 0.04
BAR_T = 0.008
BAR_START_X = -0.10
BAR_X = -0.194
BAR_TOP_Y = 0.84
BACK_BEND_R = 0.30
BACK_CENTRE_X = -0.199 + BACK_BEND_R
BACK_HALF_T = 0.013
BACK_R_IN = BACK_BEND_R - BACK_HALF_T
BACK_R_OUT = BACK_BEND_R + BACK_HALF_T
BACK_Y0 = 0.78
BACK_Y1 = 0.87
BACK_HALF_RAD = 0.5

FRAMES = {
    "ground": ((0.0, 0.0, 0.0), 0.0),
    "frame": ((0.0, 0.0, 0.0), 0.0),
    "seat": ((0.0, SEAT_JOINT_Y, 0.0), 90.0),
    "pad": ((0.0, PAD_JOINT_Y, 0.0), 0.0),
    "backrest": ((BACK_JOINT[0], BACK_JOINT[1], 0.0), 90.0),
}


def rnd(value, digits=5):
    out = round(value, digits)
    return 0.0 if out == 0 else out


def text(en, de):
    return {"en": en, "de": de}


def to_local(bone, p):
    """🧭 World point to the rest frame of a bone (u along the bone, v its sagittal normal, w lateral)."""
    (ox, oy, oz), deg = FRAMES[bone]
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    dx, dy = p[0] - ox, p[1] - oy
    return [rnd(c * dx + s * dy), rnd(-s * dx + c * dy), rnd((p[2] if len(p) > 2 else 0.0) - oz)]


def leg_radius_at(y):
    """📏 Distance of the leg axis from the stool axis at height y."""
    (x0, y0), (x1, y1) = LEG_FOOT, LEG_TOP
    return x0 + (x1 - x0) * (y - y0) / (y1 - y0)


def arc(ca, cr, r, deg0, deg1, n):
    """⭕ Points (axial, radial) of a circular arc about (ca, cr); 0 degrees points radially out, 90 degrees along +axial."""
    out = []
    for i in range(n + 1):
        a = math.radians(deg0 + (deg1 - deg0) * i / n)
        out.append((ca + r * math.sin(a), cr + r * math.cos(a)))
    return out


def loop(ca, cr, r, n):
    """🔵 Closed circle (axial, radial) for a torus or bead profile."""
    pts = [(ca + r * math.sin(2 * math.pi * i / n), cr + r * math.cos(2 * math.pi * i / n)) for i in range(n)]
    return pts + [pts[0]]


def prof(points):
    return [[rnd(a), rnd(r)] for a, r in points]


def head(solid_id, kind, bone, material, role, tone):
    return {"id": solid_id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def revolve_y(solid_id, bone, material, role, tone, origin, points):
    """🌀 Revolve about a vertical world axis through `origin`; profile (world height, radius)."""
    o = to_local(bone, origin)
    axis = [1.0, 0.0, 0.0] if FRAMES[bone][1] == 90.0 else [0.0, 1.0, 0.0]
    oy = origin[1]
    return {**head(solid_id, "revolve", bone, material, role, tone), "profile": prof([(a - oy, r) for a, r in points]), "axis": axis, "origin": o}


def capsule(solid_id, bone, material, role, tone, p0, p1, r0, r1=None):
    return {**head(solid_id, "capsule", bone, material, role, tone), "p0": to_local(bone, p0), "p1": to_local(bone, p1), "r0": rnd(r0), "r1": rnd(r1 if r1 is not None else r0)}


def extrude_xy(solid_id, bone, material, role, tone, points, z0, z1):
    """🏗 A side profile stated in world (x, y) pushed laterally from z0 to z1."""
    return {**head(solid_id, "extrude", bone, material, role, tone), "profile": [to_local(bone, (x, y))[:2] for x, y in points], "plane": "xy", "from": rnd(z0), "to": rnd(z1)}


def ellipsoid(solid_id, bone, material, role, tone, centre, radii):
    return {**head(solid_id, "ellipsoid", bone, material, role, tone), "radii": [rnd(t) for t in radii], "transform": {"translation": to_local(bone, centre)}}


def arc_loft(solid_id, bone, material, role, tone, radius, y_mid, half_h, half_t, half_rad, n, end_round, exponent, centre_x=0.0):
    """🌙 A pad bent round a vertical axis at world x `centre_x` (the stool axis by default): superelliptic sections (height x thickness) on the radius, from -half_rad to +half_rad, its ends rounded in plan by `end_round`."""
    rows = []
    ends = []
    for beta in (80.0, 55.0, 25.0):
        b = math.radians(beta)
        dist = end_round * (1 - math.sin(b))
        shrink = end_round * (1 - math.cos(b))
        ends.append((dist / radius, shrink))
    first = [(-half_rad + d, s) for d, s in ends]
    last = [(half_rad - d, s) for d, s in reversed(ends)]
    body = [(-half_rad + end_round / radius + i * (2 * half_rad - 2 * end_round / radius) / (n - 1), 0.0) for i in range(n)]
    for theta, shrink in first + body + last:
        p = (centre_x - radius * math.cos(theta), y_mid, radius * math.sin(theta))
        rows.append({"at": to_local(bone, p), "rx": rnd(half_h - shrink), "ry": rnd(half_t - shrink), "exponent": exponent})
    return {**head(solid_id, "loft", bone, material, role, tone), "sections": rows, "capStart": True, "capEnd": True}


def bar_outline(thick, start_x=BAR_START_X, top_y=BAR_TOP_Y, centre_y=None, bend=0.012):
    """🔩 Side outline (world x, y) of the bent flat back bar: horizontal under the seat board, bent up behind the cushion into the back pad."""
    h = thick / 2
    cy = centre_y if centre_y is not None else SEAT_JOINT_Y - h
    cx = BAR_X + bend
    bc = (cx, cy + bend)
    outer = [(start_x, cy - h), (cx, cy - h)]
    for i in range(1, 6):
        a = math.radians(-90 - 90 * i / 6)
        outer.append((bc[0] + (bend + h) * math.cos(a), bc[1] + (bend + h) * math.sin(a)))
    outer += [(BAR_X - h, bc[1]), (BAR_X - h, top_y), (BAR_X + h, top_y), (BAR_X + h, bc[1])]
    inner = []
    for i in range(1, 6):
        a = math.radians(-180 + 90 * i / 6)
        inner.append((bc[0] + (bend - h) * math.cos(a), bc[1] + (bend - h) * math.sin(a)))
    return outer + inner + [(cx, cy + h), (start_x, cy + h)]


def back_side_outline(r_in, r_out, y0, y1, half_rad, corner, centre_x=0.0):
    """🪧 Side view (world x, y) of a pad bent round a vertical axis at world x `centre_x`: from its rear face at the centre to the inner face at its ends, corners rounded."""
    x_back = centre_x - r_out
    x_front = centre_x - r_in * math.cos(half_rad)
    pts = []
    for (cx, cy, a0) in ((x_front - corner, y1 - corner, 0), (x_back + corner, y1 - corner, 90), (x_back + corner, y0 + corner, 180), (x_front - corner, y0 + corner, 270)):
        for i in range(4):
            a = math.radians(a0 + 90 * i / 3)
            pts.append((cx + corner * math.cos(a), cy + corner * math.sin(a)))
    return pts


def frame_mass_centre():
    """⚖️ Centre of mass of the steel frame from tube lengths (0.86 kg/m for 25 x 1.5 mm), the pressed pan and the glides."""
    leg_len = math.hypot(LEG_TOP[0] - LEG_FOOT[0], LEG_TOP[1] - LEG_FOOT[1])
    kg_m = 7850 * math.pi * (0.025 ** 2 - 0.022 ** 2) / 4
    legs = 4 * leg_len * kg_m
    ring = 2 * math.pi * RING_R * kg_m
    pan = 7850 * PAN_T * (math.pi * PAN_R ** 2 + 2 * math.pi * PAN_R * PAN_RIM)
    parts = [(legs, (LEG_TOP[1] + LEG_FOOT[1]) / 2), (ring, RING_Y), (pan, PAN_TOP - 0.004)]
    total = sum(m for m, _ in parts)
    return total, sum(m * y for m, y in parts) / total
