"""🫖 Round 3 electric jug kettle: the real measures of a 1.7 L stainless jug kettle on its 360-degree power base and the solid writers every scale states its parts with.

Frames: base bone = world (x ahead to the handle, y up, z left); body bone origin at world (0, 0.03), v = world y - 0.03; handle bone origin at body (HANDLE_X, HANDLE_Y); water bone at body (0, WATER_Y); lid bone at body (0, RIM_V). Every number is metres.
"""
import math

BASE_R = 0.09
BASE_H = 0.03
RECESS_R = 0.077
RECESS_FLOOR = 0.026

R_BOT = 0.075
R_TOP = 0.07
BODY_H = 0.175
WALL = 0.0007
SKIRT_TOP = 0.016
SKIRT_R = 0.0752
SKIRT_IN = 0.068
SKIRT_LOW = RECESS_FLOOR - BASE_H
FLOOR_LOW = 0.0085
FLOOR_TOP = 0.0105
RIM_V = 0.175
COLLAR_IN = 0.0625

LID_FLANGE = 0.066
LID_SKIRT = 0.063
LID_SKIRT_LOW = -0.010

WATER_Y = 0.01
WATER_KG = 1.0
HANDLE_X = 0.072
HANDLE_Y = 0.09

SPOUT = [(-0.066, 0.1543, 0.0165, 0.0285), (-0.07, 0.156, 0.0155, 0.0275), (-0.085, 0.1665, 0.0115, 0.0235), (-0.1, 0.178, 0.0078, 0.0195), (-0.111, 0.1865, 0.006, 0.0165), (-0.1155, 0.1895, 0.0042, 0.0135)]
SPOUT_TIP = (-0.116, 0.19)
SWITCH = (0.0535, -0.03, 0.005, 0.009, 0.008)


def r5(x):
    return round(x, 5) + 0.0


def v3(p):
    return [r5(p[0]), r5(p[1]), r5(p[2])]


def r_out(v):
    """📐 Outer radius of the tapered stainless wall at body height v (straight cone 0.15 m to 0.14 m)."""
    return R_BOT - (R_BOT - R_TOP) * v / BODY_H


def r_in(v):
    return r_out(v) - WALL


def water_height(kg=WATER_KG):
    """💧 Fill height above the floor that holds kg of water in the tapered jug."""
    h = 0.0
    for _ in range(40):
        a = math.pi * r_in(FLOOR_TOP + h / 2) ** 2
        h = kg / 1000.0 / a
    return h


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


def planar_frame(t):
    """🧭 Section rotation for a path in the u-v plane with tangent t: section x = in-plane normal (carries rx), y = w (carries ry), z = t."""
    n = math.hypot(t[0], t[1])
    tx, ty = t[0] / n, t[1] / n
    return quat_from_axes((-ty, tx, 0.0), (0.0, 0.0, 1.0), (tx, ty, 0.0))


def solid(sid, kind, bone, material, role, tone, **extra):
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update(extra)
    return s


def revolve(sid, bone, material, role, tone, profile, origin=(0.0, 0.0, 0.0), **extra):
    return solid(sid, "revolve", bone, material, role, tone, profile=[[r5(a), r5(r)] for a, r in profile], axis=[0, 1, 0], origin=v3(origin), **extra)


def ellipsoid(sid, bone, material, role, tone, radii, at, **extra):
    return solid(sid, "ellipsoid", bone, material, role, tone, radii=[r5(c) for c in radii], transform={"translation": v3(at)}, **extra)


def box(sid, bone, material, role, tone, size, at, corner=0.0, rotation=None, **extra):
    t = {"translation": v3(at)}
    if rotation is not None:
        t["rotation"] = rotation
    s = solid(sid, "box", bone, material, role, tone, size=[r5(c) for c in size], transform=t, **extra)
    if corner:
        s["cornerRadius"] = r5(corner)
    return s


def planar_loft(sid, bone, material, role, tone, path, n=2.0, cap=(True, True), **extra):
    """🪢 A loft along a centre line in the u-v plane: path = [(u, v, rx in-plane, ry lateral)], every section square to the path (central difference), so the skin never twists."""
    secs = []
    for k, (u, v, rx, ry) in enumerate(path):
        a = path[max(k - 1, 0)]
        b = path[min(k + 1, len(path) - 1)]
        secs.append({"at": v3((u, v, 0.0)), "rx": r5(rx), "ry": r5(ry), "exponent": n, "rotation": planar_frame((b[0] - a[0], b[1] - a[1]))})
    return solid(sid, "loft", bone, material, role, tone, sections=secs, capStart=cap[0], capEnd=cap[1], **extra)


def arc(cx, cy, r, a0, a1, steps):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / steps)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / steps))) for k in range(steps + 1)]


def handle_path(grip_x=0.0445, corner=0.0175, top_y=0.068, bottom_y=-0.0575, arm=(0.010, 0.0095), grip=(0.0115, 0.012), width=(0.0145, 0.016), steps=3, roots=(-0.004, -0.001)):
    """✊ Centre line of the C handle in the handle frame (origin on the wall at mid height): top arm from inside the wall, rounded corner, slightly bellied grip, rounded corner, bottom arm back into the wall; (u, v, rx, ry)."""
    cx = grip_x - corner
    top = [(roots[0], top_y, arm[0], width[0]), (cx * 0.45, top_y, arm[0] * 0.97, width[0] + 0.0005)]
    c1 = [(x, y, arm[0] + (grip[0] - arm[0]) * k / steps, width[0] + (width[1] - width[0]) * k / steps) for k, (x, y) in enumerate(arc(cx, top_y - corner, corner, 90, 0, steps))]
    mid = [(grip_x + 0.001, 0.03, grip[1], width[1]), (grip_x + 0.0015, 0.0, grip[1], width[1]), (grip_x + 0.001, -0.025, grip[0], width[1])]
    c2 = [(x, y, grip[0] + (arm[1] - grip[0]) * k / steps, width[1] + (width[0] - width[1]) * k / steps) for k, (x, y) in enumerate(arc(cx, bottom_y + corner, corner, 0, -90, steps))]
    bot = [(cx * 0.5, bottom_y - 0.002, arm[1], width[0]), (roots[1], bottom_y - 0.0035, arm[1], width[0])]
    return top + c1 + mid + c2 + bot


def section_points(path):
    """✏️ The in-plane outline of a planar loft: the two offset edges of the centre line, for the elevation drawing."""
    left, right = [], []
    for k, (u, v, rx, _) in enumerate(path):
        a = path[max(k - 1, 0)]
        b = path[min(k + 1, len(path) - 1)]
        tx, ty = b[0] - a[0], b[1] - a[1]
        n = math.hypot(tx, ty)
        nx, ny = -ty / n, tx / n
        left.append((u + nx * rx, v + ny * rx))
        right.append((u - nx * rx, v - ny * rx))
    return left, right


HANDLE = {
    "50": handle_path(),
    "100": handle_path(steps=1),
    "200": handle_path(grip_x=0.043, corner=0.017, top_y=0.066, bottom_y=-0.058, arm=(0.015, 0.015), grip=(0.015, 0.0155), width=(0.017, 0.018), steps=1, roots=(-0.006, -0.004)),
}
SPOUTS = {
    "50": SPOUT,
    "100": [SPOUT[0], SPOUT[2], (-0.113, 0.188, 0.0058, 0.016)],
    "200": [(-0.056, 0.146, 0.021, 0.029), (-0.086, 0.169, 0.013, 0.021), (-0.114, 0.189, 0.0075, 0.015)],
}
BASE = {
    "50": [(0, 0), (0, 0.0855), (0.0015, 0.0885), (0.004, 0.09), (0.021, 0.09), (0.0255, 0.0889), (0.0285, 0.0858), (0.03, 0.0815), (0.03, 0.0785), (0.0272, 0.0775), (RECESS_FLOOR, 0.0762), (RECESS_FLOOR, 0)],
    "100": [(0, 0), (0, 0.086), (0.004, 0.09), (0.025, 0.09), (0.03, 0.085), (0.03, 0)],
    "200": [(0, 0), (0, 0.091), (0.004, 0.095), (0.036, 0.095), (0.04, 0.091), (0.04, 0)],
}
LID = {
    "50": [(LID_SKIRT_LOW, 0), (LID_SKIRT_LOW, 0.0605), (LID_SKIRT_LOW + 0.0025, LID_SKIRT), (0.0, LID_SKIRT), (0.0, 0.0652), (0.0012, LID_FLANGE), (0.0035, LID_FLANGE), (0.0052, 0.0642), (0.0075, 0.057), (0.0102, 0.045), (0.0124, 0.031), (0.0136, 0.018), (0.0141, 0.008), (0.0142, 0)],
    "100": [(-0.008, 0), (-0.008, LID_SKIRT), (0.0, LID_SKIRT), (0.0, LID_FLANGE), (0.004, LID_FLANGE), (0.008, 0.058), (0.0125, 0.04), (0.0145, 0.022), (0.016, 0.0125), (0.0275, 0.0125), (0.03, 0.009), (0.03, 0)],
    "200": [(-0.006, 0), (-0.006, 0.064), (0.0, 0.064), (0.0, 0.07), (0.006, 0.07), (0.014, 0.054), (0.021, 0.032), (0.024, 0.016), (0.04, 0.016), (0.04, 0)],
}
KNOB = [(0.01, 0), (0.01, 0.0085), (0.0185, 0.0085), (0.0205, 0.012), (0.0265, 0.0125), (0.0295, 0.0115), (0.0315, 0.0085), (0.032, 0.004), (0.032, 0)]
SHELL_OUTLINE = {
    "50": [(0.0, SKIRT_R), (SKIRT_TOP - 0.0015, SKIRT_R), (SKIRT_TOP, 0.0746), (0.167, r_out(0.167)), (0.1705, 0.0699), (0.1732, 0.069), (RIM_V, 0.0672)],
    "100": [(0.0, R_BOT), (0.17, r_out(0.17)), (RIM_V, 0.0685)],
    "200": [(0.0, 0.078), (0.17, 0.0722), (RIM_V, 0.068)],
}
