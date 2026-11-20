"""🔥 Round 3 electric fan heater: the real measures of a 2 kW box fan heater (U-shell, top cover, front and rear frame, steel louvre cassette, intake panel, axial fan, coil box) and the solid writers every scale states its parts with.

Frame: body bone = world (x ahead to the outlet, y up, z left); knob bone origin at body (0, KNOB_Y), its axis is z. Every number is metres.
"""
import math

HALF_W = 0.114
Y_BOTTOM = 0.016
Y_TOP = 0.272
Y_SEAM = 0.232
EDGE_R = 0.022
WALL = 0.0025

TUBE_X0 = -0.084
TUBE_X1 = 0.082
FRONT_LIP_X1 = 0.090
FRONT_LIP_SCALE = 0.9
REAR_LIP_X0 = -0.092
REAR_LIP_SCALE = 0.9
FRONT_X = 0.098
REAR_X = -0.098

OUTLET = (0.060, 0.230, 0.094)
OUTLET_R = 0.006
CASSETTE_X0 = 0.082
CASSETTE_BAR = 0.005
CASSETTE_X1 = 0.094
SLAT_COUNT = 8
SLAT_PITCH = 0.020
SLAT_Y0 = 0.075
SLAT_IN = (0.0835, 0.0045)
SLAT_OUT = (0.098, -0.0055)
SLAT_T = 0.0015
AXIS_Y = 0.145

INTAKE_HOLE = (0.070, 0.220, 0.090)
INTAKE_PANEL = (0.063, 0.227, 0.097)
INTAKE_PANEL_R = 0.012
INTAKE_SLOT = (0.0215, 0.011, 0.080)
INTAKE_SLOTS = 7

KNOB_Y = 0.185
KNOB_R = 0.022
KNOB_OUT = 0.126
SWITCH_Y = 0.112

FOOT_X = 0.058
FOOT_Z = 0.075
PAD_H = 0.010

HANDLE_Z = 0.058
HANDLE_BEND = 0.012
HANDLE_BAR_Y = 0.304
HANDLE_DEPTH = 0.013

FAN_X = (-0.052, -0.028)
FAN_R = 0.065
HEATER_X = (0.004, 0.058)
HEATER_BOX = (0.074, 0.216, 0.080)


def r5(x):
    return round(x, 5) + 0.0


def v3(p):
    return [r5(p[0]), r5(p[1]), r5(p[2])]


def pts(points):
    return [[r5(a), r5(b)] for a, b in points]


def arc(cx, cy, r, a0, a1, steps):
    """🌙 Points of a circular arc from a0 to a1 degrees, both ends included."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / steps)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / steps))) for k in range(steps + 1)]


def rounded_rect(y0, y1, z0, z1, r, steps=5, scale=1.0, about=(0.0, 0.0)):
    """⬜ A rounded rectangle as [y, z] points, counter-clockwise from the low corner, optionally scaled about a point."""
    out = []
    if r <= 0:
        out = [(y0, z0), (y1, z0), (y1, z1), (y0, z1)]
    else:
        out += arc(y0 + r, z0 + r, r, 180, 270, steps)
        out += arc(y1 - r, z0 + r, r, 270, 360, steps)
        out += arc(y1 - r, z1 - r, r, 0, 90, steps)
        out += arc(y0 + r, z1 - r, r, 90, 180, steps)
    return [(about[0] + (y - about[0]) * scale, about[1] + (z - about[1]) * scale) for y, z in out]


def u_profile(y_low, y_high, half_w, r, wall, steps=5, cap=False):
    """🧲 The section of a channel as [y, z] points: a wall of constant thickness round the low side (cap False, the U-shell) or the high side (cap True, the top cover), open at the other."""
    if not cap:
        left = [(y_low + r - r * math.sin(math.radians(90 * k / steps)), -half_w + r - r * math.cos(math.radians(90 * k / steps))) for k in range(steps + 1)]
        right = [(y, -z) for y, z in left[::-1]]
        outer = [(y_high, -half_w)] + left + right + [(y_high, half_w)]
        ri = r - wall
        left_in = [(y_low + r - ri * math.sin(math.radians(90 * k / steps)), -half_w + r - ri * math.cos(math.radians(90 * k / steps))) for k in range(steps + 1)]
        right_in = [(y, -z) for y, z in left_in[::-1]]
        inner = [(y_high, -half_w + wall)] + left_in + right_in + [(y_high, half_w - wall)]
        return outer + inner[::-1]
    mirrored = u_profile(-y_high, -y_low, half_w, r, wall, steps, False)
    return [(-y, z) for y, z in mirrored]


def u_half(y_low, y_high, half_w, r, wall, steps=5, side=-1, cap=False):
    """🌗 One lateral half of the channel section (side -1 = right, z below 0; +1 = left), cut on the centre plane, so that its inward faces look along two bone axes only."""
    if cap:
        return [(-y, z) for y, z in u_half(-y_high, -y_low, half_w, r, wall, steps, side, False)]
    outer = [(y_high, -half_w)] + [(y_low + r - r * math.sin(math.radians(90 * k / steps)), -half_w + r - r * math.cos(math.radians(90 * k / steps))) for k in range(steps + 1)] + [(y_low, 0.0)]
    ri = r - wall
    inner = [(y_high, -half_w + wall)] + [(y_low + r - ri * math.sin(math.radians(90 * k / steps)), -half_w + r - ri * math.cos(math.radians(90 * k / steps))) for k in range(steps + 1)] + [(y_low + wall, 0.0)]
    return [(y, -side * z) for y, z in outer + inner[::-1]]


def quat_from_axes(x, y, z):
    """🧭 Unit quaternion [x, y, z, w] of the rotation whose columns are the given orthonormal axes."""
    m00, m01, m02 = x[0], y[0], z[0]
    m10, m11, m12 = x[1], y[1], z[1]
    m20, m21, m22 = x[2], y[2], z[2]
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        q = ((m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s)
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1.0 + m00 - m11 - m22) * 2
        q = (0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s)
    elif m11 > m22:
        s = math.sqrt(1.0 + m11 - m00 - m22) * 2
        q = ((m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s)
    else:
        s = math.sqrt(1.0 + m22 - m00 - m11) * 2
        q = ((m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s)
    n = math.sqrt(sum(c * c for c in q))
    return [round(c / n, 6) + 0.0 for c in q]


def quat_axis(axis, deg):
    """🌀 Unit quaternion of a turn by deg degrees about a coordinate axis (0 = x, 1 = y, 2 = z)."""
    h = math.radians(deg) / 2
    q = [0.0, 0.0, 0.0, round(math.cos(h), 6)]
    q[axis] = round(math.sin(h), 6)
    return q


def solid(sid, kind, bone, material, role, tone, **extra):
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update(extra)
    return s


def extrude(sid, material, role, tone, profile, plane, lo, hi, holes=None, taper=None, bone="body", **extra):
    s = solid(sid, "extrude", bone, material, role, tone, profile=pts(profile), plane=plane)
    s["from"] = r5(lo)
    s["to"] = r5(hi)
    if holes:
        s["holes"] = [pts(h) for h in holes]
    if taper:
        s["taperTo"] = list(taper)
    s.update(extra)
    return s


def revolve(sid, material, role, tone, profile, axis, origin, bone="body", **extra):
    return solid(sid, "revolve", bone, material, role, tone, profile=pts(profile), axis=list(axis), origin=v3(origin), **extra)


def box(sid, material, role, tone, size, at, corner=0.0, rotation=None, bone="body", **extra):
    t = {"translation": v3(at)}
    if rotation is not None:
        t["rotation"] = rotation
    s = solid(sid, "box", bone, material, role, tone, size=[r5(c) for c in size], transform=t)
    if corner:
        s["cornerRadius"] = r5(corner)
    s.update(extra)
    return s


def ellipsoid(sid, material, role, tone, radii, at, bone="body", **extra):
    return solid(sid, "ellipsoid", bone, material, role, tone, radii=[r5(c) for c in radii], transform={"translation": v3(at)}, **extra)


def lateral_loft(sid, material, role, tone, path, x=0.0, n=2.6, cap=(True, True), **extra):
    """🪢 A loft along a centre line in the lateral plane at depth x: path = [(z, y, half thickness in the plane, half depth along x)], every section square to the path."""
    secs = []
    for k, (z, y, rn, rd) in enumerate(path):
        a = path[max(k - 1, 0)]
        b = path[min(k + 1, len(path) - 1)]
        tz, ty = b[0] - a[0], b[1] - a[1]
        m = math.hypot(tz, ty)
        t = (0.0, ty / m, tz / m)
        sy = (1.0, 0.0, 0.0)
        sx = (sy[1] * t[2] - sy[2] * t[1], sy[2] * t[0] - sy[0] * t[2], sy[0] * t[1] - sy[1] * t[0])
        secs.append({"at": v3((x, y, z)), "rx": r5(rn), "ry": r5(rd), "exponent": n, "rotation": quat_from_axes(sx, sy, t)})
    return solid(sid, "loft", "body", material, role, tone, sections=secs, capStart=cap[0], capEnd=cap[1], **extra)


def handle_path(bar=0.006, post=0.008, bend_steps=4, foot=True, depth=HANDLE_DEPTH, root_y=0.2700):
    """✊ Centre line of the carry handle across the top cover: post, bend, grip bar, bend, post; (z, y, half thickness, half depth)."""
    zc = HANDLE_Z - HANDLE_BEND
    yc = HANDLE_BAR_Y - HANDLE_BEND
    up = []
    if foot:
        up += [(-HANDLE_Z, root_y, post + 0.004, depth + 0.004), (-HANDLE_Z, 0.2745, post + 0.0012, depth + 0.0014), (-HANDLE_Z, 0.279, post, depth)]
    else:
        up += [(-HANDLE_Z, root_y, post, depth)]
    bend = []
    for k in range(bend_steps + 1):
        a = math.radians(180 - 90 * k / bend_steps)
        bend.append((-zc + HANDLE_BEND * math.cos(a), yc + HANDLE_BEND * math.sin(a), post + (bar - post) * k / bend_steps, depth))
    left = up + bend
    mid = [(0.0, HANDLE_BAR_Y, bar, depth)]
    right = [(-z, y, rn, rd) for z, y, rn, rd in left[::-1]]
    return left + mid + right


def slat_centres(count=SLAT_COUNT, pitch=SLAT_PITCH, y0=SLAT_Y0):
    return [y0 + pitch * k for k in range(count)]


def slat_angle():
    return math.degrees(math.atan2(SLAT_IN[1] - SLAT_OUT[1], SLAT_OUT[0] - SLAT_IN[0]))


def slat_chord():
    return math.hypot(SLAT_OUT[0] - SLAT_IN[0], SLAT_IN[1] - SLAT_OUT[1])


def intake_slots(count=INTAKE_SLOTS, pitch=INTAKE_SLOT[0], height=INTAKE_SLOT[1], half=INTAKE_SLOT[2], r=0.003, steps=2):
    """🕳️ The slot holes of the intake panel as [y, z] rings, centred on the fan axis."""
    out = []
    for k in range(count):
        c = AXIS_Y + (k - (count - 1) / 2) * pitch
        out.append(rounded_rect(c - height / 2, c + height / 2, -half, half, r, steps))
    return out


BLADE = [(0.012, -0.008), (0.020, -0.013), (0.034, -0.021), (0.048, -0.027), (0.058, -0.028), (0.0635, -0.022), (0.065, -0.010), (0.0645, 0.004), (0.061, 0.016), (0.054, 0.022), (0.044, 0.022), (0.032, 0.017), (0.021, 0.011), (0.012, 0.007)]
