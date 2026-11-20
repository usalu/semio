"""🍞 Round 3 two-slice toaster: the real measures of a chromed pop-up toaster with its controls on the long face and the solid writers every scale states its parts with.

Frames: base bone = world (x along the 0.30 m length, y up, z across the depth; the control face looks to -z, the cord leaves at +z); slices bone origin at world (0, SEAT), the seat of the bread on the carriage; lever bone at world (LEVER_X, LEVER_Y) = the knob with the carriage up; dial bone at world (DIAL_X, DIAL_Y) = the spindle. Every number is metres.
"""
import math

HX = 0.15
HZ = 0.076
FOOT_H = 0.012
TRAY_TOP = 0.03
TRAY_HX = 0.144
TRAY_HZ = 0.07
HEM = 0.028
WALL_X = 0.15
WALL_Z = 0.166
TOP = 0.188
TOP_HX = 0.114
TOP_HZ = 0.054
SEAM = 0.174
DECK_LOW = 0.18
N = 9

FOOT_X = 0.1
FOOT_Z = 0.048
FOOT_R0 = 0.011
FOOT_R1 = 0.0135

SLOT_Z = 0.03
SLOT_W = 0.03
SLOT_HX = 0.068
SLOT_R = 0.006
WELL_W = 0.034
WELL_HX = 0.071
CHAMBER_HX = 0.078
CHAMBER_HZ = 0.056

SEAT = 0.085
BREAD_T = 0.013
BREAD_HX = 0.052
BREAD_H = 0.115
TRAVEL = 0.04

LEVER_X = 0.1
LEVER_Y = 0.145
DIAL_X = -0.1
DIAL_Y = 0.13
DIAL_R = 0.015
BUTTON = (-0.028, 0.127, 0.012, 0.007)
BUSH = (-0.11, 0.021, 0.007)
YOKE_X = 0.1
ROD_X = 0.112

UP = [-0.5, -0.5, -0.5, 0.5]

BREAD = [(0.046, 0.085), (0.05, 0.0865), (0.052, 0.09), (0.052, 0.166), (0.0555, 0.169), (0.058, 0.174), (0.0585, 0.18), (0.057, 0.186), (0.053, 0.191), (0.046, 0.195), (0.034, 0.198), (0.018, 0.1995), (0.0, 0.2)]
CRUMB = [(0.043, 0.0885), (0.0472, 0.0898), (0.0485, 0.093), (0.0485, 0.1675), (0.052, 0.1705), (0.0545, 0.175), (0.055, 0.18), (0.0535, 0.185), (0.05, 0.189), (0.044, 0.192), (0.033, 0.1947), (0.0175, 0.1961), (0.0, 0.1965)]
BREAD_100 = [(0.048, 0.085), (0.052, 0.089), (0.052, 0.166), (0.057, 0.171), (0.0585, 0.18), (0.055, 0.189), (0.044, 0.196), (0.022, 0.1995), (0.0, 0.2)]
BREAD_200 = [(0.052, 0.1), (0.052, 0.166), (0.058, 0.174), (0.057, 0.186), (0.048, 0.194), (0.026, 0.199), (0.0, 0.2)]
BREAD_500 = [(0.052, 0.13), (0.057, 0.174), (0.05, 0.192), (0.028, 0.199), (0.0, 0.2)]

SHELL_Y = {
    "50": [0.15, 0.155, 0.16, 0.164, 0.166, 0.168, 0.17, 0.172, SEAM],
    "100": [0.15, 0.158, 0.166, 0.171, SEAM],
}
RIM_Y = {
    "50": [SEAM, 0.176, 0.178, 0.18, 0.182, 0.184, 0.1855, 0.1868, 0.1876, TOP],
    "100": [SEAM, 0.18, 0.185, TOP],
}
HOUSING_Y = {
    "200": [0.0, 0.15, 0.166, 0.176, 0.184, TOP],
    "500": [0.0, 0.15, 0.176, TOP],
}


def r5(x):
    return round(x, 5) + 0.0


def v3(p):
    return [r5(p[0]), r5(p[1]), r5(p[2])]


def hx(y):
    """📐 Half length of the housing at height y: 2 mm hem, straight wall, then the end shoulder, a quarter ellipse 36 mm wide and 38 mm high."""
    if y <= HEM + 0.004:
        return HX - 0.002 * max(0.0, (HEM + 0.004 - y) / 0.004)
    if y <= WALL_X:
        return HX
    t = min(1.0, (y - WALL_X) / (TOP - WALL_X))
    return TOP_HX + (HX - TOP_HX) * math.sqrt(1.0 - t * t)


def hz(y):
    """📐 Half depth of the housing at height y: hem, straight wall, then the long shoulder, a quarter circle of 22 mm."""
    if y <= HEM + 0.004:
        return HZ - 0.002 * max(0.0, (HEM + 0.004 - y) / 0.004)
    if y <= WALL_Z:
        return HZ
    t = min(1.0, (y - WALL_Z) / (TOP - WALL_Z))
    return TOP_HZ + (HZ - TOP_HZ) * math.sqrt(1.0 - t * t)


def superellipse(a, b, n=N, per_quarter=8):
    """⬭ The outline |x/a|^n + |y/b|^n = 1 as points (x, y), counter-clockwise from (a, 0), denser at the corners."""
    pts = []
    for q in range(4):
        for k in range(per_quarter):
            t = (q + k / per_quarter) * math.pi / 2
            c, s = math.cos(t), math.sin(t)
            pts.append((a * math.copysign(abs(c) ** (2.0 / n), c), b * math.copysign(abs(s) ** (2.0 / n), s)))
    return pts


def rounded_rect(x0, y0, x1, y1, r, steps=3):
    """▭ A rectangle with round corners as points, counter-clockwise."""
    pts = []
    for cx, cy, a0 in ((x1 - r, y0 + r, -90), (x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180)):
        for k in range(steps + 1):
            a = math.radians(a0 + 90.0 * k / steps)
            q = (cx + r * math.cos(a), cy + r * math.sin(a))
            if not pts or math.hypot(q[0] - pts[-1][0], q[1] - pts[-1][1]) > 1e-7:
                pts.append(q)
    if math.hypot(pts[0][0] - pts[-1][0], pts[0][1] - pts[-1][1]) <= 1e-7:
        pts.pop()
    return pts


def mirror(half, dy=0.0):
    """🪞 A closed symmetric outline from its right half given bottom to top; the last point may lie on the axis."""
    right = [(x, y + dy) for x, y in half]
    left = [(-x, y + dy) for x, y in reversed(half) if abs(x) > 1e-9]
    return right + left


def solid(sid, kind, bone, material, role, tone, **extra):
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update(extra)
    return s


def box(sid, bone, material, role, tone, size, at, corner=0.0, **extra):
    s = solid(sid, "box", bone, material, role, tone, size=[r5(c) for c in size], transform={"translation": v3(at)}, **extra)
    if corner:
        s["cornerRadius"] = r5(corner)
    return s


def ellipsoid(sid, bone, material, role, tone, radii, at, **extra):
    return solid(sid, "ellipsoid", bone, material, role, tone, radii=[r5(c) for c in radii], transform={"translation": v3(at)}, **extra)


def capsule(sid, bone, material, role, tone, p0, p1, r, **extra):
    return solid(sid, "capsule", bone, material, role, tone, p0=v3(p0), p1=v3(p1), r0=r5(r), r1=r5(r), **extra)


def revolve(sid, bone, material, role, tone, profile, axis=(0, 1, 0), origin=(0.0, 0.0, 0.0), **extra):
    return solid(sid, "revolve", bone, material, role, tone, profile=[[r5(a), r5(r)] for a, r in profile], axis=list(axis), origin=v3(origin), **extra)


def extrude(sid, bone, material, role, tone, plane, profile, lo, hi, holes=None, **extra):
    s = solid(sid, "extrude", bone, material, role, tone, plane=plane, profile=[[r5(a), r5(b)] for a, b in profile], **extra)
    s["from"], s["to"] = r5(lo), r5(hi)
    if holes:
        s["holes"] = [[[r5(a), r5(b)] for a, b in h] for h in holes]
    return s


def stack(sid, bone, material, role, tone, rows, caps=(True, True), n=N, **extra):
    """🥞 A loft of horizontal superelliptic sections: rows = [(y, half length along x, half depth along z)], bottom to top."""
    secs = [{"at": v3((0.0, y, 0.0)), "rx": r5(b), "ry": r5(a), "exponent": n, "rotation": UP} for y, a, b in rows]
    return solid(sid, "loft", bone, material, role, tone, sections=secs, capStart=caps[0], capEnd=caps[1], **extra)


def along_z(sid, bone, material, role, tone, rows, n=4, **extra):
    """🧈 A loft of upright superelliptic sections along the depth: rows = [(z, half width along x, half height along y)], centre (cx, cy) optional as 4th and 5th item."""
    secs = []
    for row in rows:
        z, a, b = row[:3]
        cx, cy = (row[3], row[4]) if len(row) > 3 else (0.0, 0.0)
        secs.append({"at": v3((cx, cy, z)), "rx": r5(a), "ry": r5(b), "exponent": n})
    return solid(sid, "loft", bone, material, role, tone, sections=secs, capStart=True, capEnd=True, **extra)


def shell_rows(den):
    """🧥 Sections of the chromed wrap from the hem on the plinth to the seam of the dark cap."""
    low = [(TRAY_TOP, TRAY_HX, TRAY_HZ), (TRAY_TOP + 0.0005, HX - 0.0015, HZ - 0.0015), (TRAY_TOP + 0.003, HX, HZ)] if den == "50" else [(TRAY_TOP, TRAY_HX, TRAY_HZ), (TRAY_TOP + 0.0005, HX, HZ)]
    return low + [(y, hx(y), hz(y)) for y in SHELL_Y[den]]


def rim_rows(den):
    return [(y, hx(y), hz(y)) for y in RIM_Y[den]]


def housing_rows(den):
    return [(y, hx(max(y, 0.04)), hz(max(y, 0.04))) for y in HOUSING_Y[den]]


def slot(sign, grow=0.0, steps=3):
    """🕳️ One bread slot as points (z, x) for a zx profile; sign -1 = the slot at the control face."""
    z0, z1 = sign * SLOT_Z - SLOT_W / 2 - grow, sign * SLOT_Z + SLOT_W / 2 + grow
    return rounded_rect(z0, -SLOT_HX - grow, z1, SLOT_HX + grow, SLOT_R + grow, steps)


def well(sign):
    z0, z1 = sign * SLOT_Z - WELL_W / 2, sign * SLOT_Z + WELL_W / 2
    return [(z0, -WELL_HX), (z1, -WELL_HX), (z1, WELL_HX), (z0, WELL_HX)]


def deck_outline(per_quarter=12):
    """⬭ Flat top of the cap as (z, x) points."""
    return superellipse(TOP_HZ, TOP_HX, N, per_quarter)
