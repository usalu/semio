"""🪨️ Hand-chosen numbers and pure geometry of `used-bubbler-stone-fountain`: rig frames, the boulder sections, outlines and masses.

Every length is metres in the world frame of the rest pose (x ahead, y up, z left); `loc` maps a world point into the frame of a bone.
See ../../brief.md and the staffage README (🎓️teaching/🏛️architecture/🧍️staffage/README.md).
"""
import math

PI = math.pi


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def r6(x):
    """✂️ Rounds to micrometres."""
    return round(x + 0.0, 6)


def pts(seq):
    """📍️ Rounds a point list."""
    return [[r6(a), r6(b)] for a, b in seq]


Y_SHAFT = -0.98
Y_RIM = -0.20
Y_FLOOR = -0.86
Y_WATER = -0.50
Y_SLAB_TOP = -0.05
Y_GRAVEL = -0.02
R_IN, R_OUT = 0.60, 0.72
SLAB_T = 0.15
X_LID = 0.40
R_HOLE = 0.215
R_PLATE = 0.186
HINGE = (0.595, -0.024)
X_STONE = -0.25
H_STONE = 0.80
R_SETT_IN, R_SETT_OUT = 0.90, 1.00
R_HAUNCH_IN = 0.89
Y_HAUNCH = -0.22
LIFT = 0.96
SINK = 0.14
LID_OPEN = -90.0
HOSE_R = 0.0125

STONE = [
    (0.0, 0.0, 0.3453, 0.2906, 2.6),
    (0.04, 0.0015, 0.3828, 0.3222, 2.6),
    (0.1, 0.005, 0.411, 0.3457, 2.59),
    (0.18, 0.0108, 0.425, 0.3571, 2.57),
    (0.28, 0.0192, 0.43, 0.36, 2.54),
    (0.4, 0.0305, 0.4251, 0.3524, 2.48),
    (0.52, 0.0428, 0.4, 0.3255, 2.39),
    (0.62, 0.0538, 0.3562, 0.283, 2.3),
    (0.7, 0.063, 0.2958, 0.2289, 2.22),
    (0.755, 0.0696, 0.2275, 0.1722, 2.15),
    (0.785, 0.0732, 0.1672, 0.1248, 2.12),
    (0.8, 0.075, 0.1185, 0.0877, 2.1),
]
STONE_100 = [STONE[i] for i in (0, 1, 3, 4, 5, 6, 7, 9, 11)]
STONE_200 = [STONE[i] for i in (0, 2, 4, 6, 8, 11)]
STONE_500 = [STONE[0], STONE[3], STONE[6], STONE[8], (0.80, STONE[-1][1], 0.17, 0.13, 2.1)]
X_BORE = X_STONE + STONE[-1][1]
Y_TOP = Y_SLAB_TOP + H_STONE

BUBBLE = [(0.0, 0.0), (0.0, 0.042), (0.016, 0.040), (0.034, 0.031), (0.048, 0.017), (0.056, 0.0)]
BUBBLE_500 = [(0.0, 0.0), (0.0, 0.045), (0.03, 0.04), (0.06, 0.025), (0.075, 0.0)]
FRAME = [(0.200, -0.130), (0.215, -0.130), (0.215, -0.050), (0.245, -0.050), (0.245, -0.042), (0.215, -0.042), (0.215, -0.020), (0.200, -0.020), (0.200, -0.028), (0.188, -0.028), (0.188, -0.036), (0.200, -0.036)]
PUMP_L, PUMP_W, PUMP_BASE, PUMP_TOP = 0.24, 0.16, 0.04, 0.15
HANDLE = (0.15, 0.21, 0.07, 0.016, 0.01)
OUTLET_U, OUTLET_R, OUTLET_V = 0.10, 0.016, 0.135
LEAVES = [(0.70, 0.35, 30.0), (-0.45, -0.62, -40.0), (0.20, -0.60, 75.0), (-0.62, 0.50, 120.0)]

WORLD = {
    "shaft": (0.0, Y_SHAFT, 90.0),
    "slab": (-R_OUT, Y_RIM + SLAB_T / 2, 0.0),
    "basin": (0.0, Y_GRAVEL, 0.0),
    "stone": (X_STONE, Y_SLAB_TOP, 90.0),
    "bubble": (X_BORE, Y_TOP, 90.0),
    "lid": (HINGE[0], HINGE[1], 180.0),
    "pump": (X_LID, Y_FLOOR, 90.0),
}
PARENT = {"shaft": None, "slab": "shaft", "basin": "shaft", "stone": "slab", "bubble": "stone", "lid": "slab", "pump": "shaft"}


def loc(bone, x, y):
    """🧭️ A world point (x, y) of the rest pose in the (u, v) frame of a bone."""
    ox, oy, a = WORLD[bone]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    dx, dy = x - ox, y - oy
    return (dx * c + dy * s, -dx * s + dy * c)


def world(bone, u, v):
    """🌍️ A (u, v) point of a bone frame in world coordinates of the rest pose."""
    ox, oy, a = WORLD[bone]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return (ox + u * c - v * s, oy + u * s + v * c)


def placement(bone):
    """📌️ The rest placement of a bone in its parent frame: x, y, angleDeg."""
    p = PARENT[bone]
    ox, oy, a = WORLD[bone]
    if p is None:
        return ox, oy, a
    u, v = loc(p, ox, oy)
    return r6(u), r6(v), r6(a - WORLD[p][2])


def circle(cx, cy, r, n=72, phase=0.0):
    """⭕️ A polygon approximating a circle."""
    return [(cx + r * math.cos(phase + 2 * PI * k / n), cy + r * math.sin(phase + 2 * PI * k / n)) for k in range(n)]


def superellipse(cx, cy, a, b, n, count=72):
    """🥚️ A polygon on |x/a|^n + |y/b|^n = 1 about (cx, cy)."""
    out = []
    for k in range(count):
        t = 2 * PI * k / count
        c, s = math.cos(t), math.sin(t)
        out.append((cx + a * math.copysign(abs(c) ** (2 / n), c), cy + b * math.copysign(abs(s) ** (2 / n), s)))
    return out


def ellipse(cx, cy, a, b, count=48):
    """🥚️ A polygon on an ellipse."""
    return superellipse(cx, cy, a, b, 2.0, count)


def stone_side(sections):
    """🪨️ The side elevation outline of the boulder in world (x, y): right flank up, left flank down."""
    right = [(X_STONE + dx + rx, Y_SLAB_TOP + h) for h, dx, rx, rz, n in sections]
    left = [(X_STONE + dx - rx, Y_SLAB_TOP + h) for h, dx, rx, rz, n in reversed(sections)]
    return right + left


def stone_front(sections):
    """🪨️ The front elevation outline of the boulder in world (z, y)."""
    right = [(rz, Y_SLAB_TOP + h) for h, dx, rx, rz, n in sections]
    left = [(-rz, Y_SLAB_TOP + h) for h, dx, rx, rz, n in reversed(sections)]
    return right + left


def ray_superellipse(px, pz, dx, dz, cx, a, b, n):
    """🎯️ Distance along a ray from (px, pz) to the boundary of a superellipse centred (cx, 0)."""
    f = lambda t: abs((px + t * dx - cx) / a) ** n + abs((pz + t * dz) / b) ** n - 1
    lo, hi = 0.0, 2.0
    if f(lo) > 0:
        return 0.0
    for _ in range(60):
        mid = (lo + hi) / 2
        if f(mid) > 0:
            hi = mid
        else:
            lo = mid
    return lo


def stone_plan(sections, count=96):
    """🪨️ The plan outline of the boulder in world (x, z): the union of its sections, star-shaped about the bore axis."""
    px = X_STONE + 0.02
    out = []
    for k in range(count):
        t = 2 * PI * k / count
        dx, dz = math.cos(t), math.sin(t)
        d = max(ray_superellipse(px, 0.0, dx, dz, X_STONE + sdx, rx, rz, n) for h, sdx, rx, rz, n in sections)
        out.append((px + d * dx, d * dz))
    return out


def super_area(a, b, n):
    """📐️ Area of a superellipse."""
    return 4 * a * b * math.gamma(1 + 1 / n) ** 2 / math.gamma(1 + 2 / n)


def stone_volume(sections, steps=40):
    """⚖️ Volume, centre height and centre ahead of the lofted boulder (sections interpolated linearly)."""
    vol = mh = mx = 0.0
    for (h0, d0, a0, b0, n0), (h1, d1, a1, b1, n1) in zip(sections, sections[1:]):
        for k in range(steps):
            t = (k + 0.5) / steps
            h = h0 + t * (h1 - h0)
            area = super_area(a0 + t * (a1 - a0), b0 + t * (b1 - b0), n0 + t * (n1 - n0))
            dv = area * (h1 - h0) / steps
            vol += dv
            mh += dv * h
            mx += dv * (d0 + t * (d1 - d0))
    return vol, mh / vol, mx / vol


def polygon_area(poly):
    """📐️ Signed area and centroid of a polygon."""
    a = cx = cy = 0.0
    for i in range(len(poly)):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % len(poly)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return a, cx / (6 * a), cy / (6 * a)


def revolve_volume(profile):
    """🌀️ Volume and axial centroid of a profile revolved about its axis (Pappus), profile (axial, radial) closed through the axis."""
    p = [tuple(q) for q in profile]
    if p[0] != p[-1]:
        if p[-1][1] != 0:
            p.append((p[-1][0], 0.0))
        if p[0][1] != 0:
            p.append((p[0][0], 0.0))
    else:
        p = p[:-1]
    a, ca, cr = polygon_area(p)
    return abs(a) * 2 * PI * cr, ca


def hose_path():
    """🧵️ The centre line of the PE riser hose in world (x, y): from the outlet at the boulder top down the bore, round the bend and across to the pump outlet."""
    path = [(X_BORE, Y_TOP - 0.005), (X_BORE, 0.30), (X_BORE, -0.20), (X_BORE, Y_FLOOR + OUTLET_U + 0.06)]
    cx, cy, rb = X_BORE + 0.06, Y_FLOOR + OUTLET_U + 0.06, 0.06
    for k in range(1, 7):
        t = PI + (PI / 2) * k / 6
        path.append((cx + rb * math.cos(t), cy + rb * math.sin(t)))
    path += [(0.10, Y_FLOOR + OUTLET_U), (X_LID - OUTLET_V, Y_FLOOR + OUTLET_U)]
    return path


def hose_length():
    """📏️ Length of the hose centre line."""
    p = hose_path()
    return sum(math.dist(a, b) for a, b in zip(p, p[1:]))


def band(path, half):
    """🎗️ A closed polygon of constant half width around an open polyline (mitred joints)."""
    left, right = [], []
    for i, (x, y) in enumerate(path):
        a = path[max(i - 1, 0)]
        b = path[min(i + 1, len(path) - 1)]
        tx, ty = b[0] - a[0], b[1] - a[1]
        ln = math.hypot(tx, ty)
        nx, ny = -ty / ln, tx / ln
        left.append((x + half * nx, y + half * ny))
        right.append((x - half * nx, y - half * ny))
    return left + list(reversed(right))


def leaf_outline(cx, cz, deg, length=0.07, width=0.036, count=14):
    """🍂️ A lime leaf lying on the gravel in plan (x, z): heart-shaped base, pointed tip."""
    out = []
    a = math.radians(deg)
    for k in range(count):
        t = 2 * PI * k / count
        u = 0.5 * length * math.cos(t)
        w = 0.5 * width * math.sin(t) * (1.0 - 0.35 * (u / (0.5 * length)) - 0.0)
        out.append((cx + u * math.cos(a) - w * math.sin(a), cz + u * math.sin(a) + w * math.cos(a)))
    return out


STONE_VOL, STONE_COM_H, STONE_COM_DX = stone_volume(STONE)
DENS_GRANITE = 2650
HOSE_AREA = PI * (HOSE_R ** 2 - (HOSE_R - 0.0023) ** 2)
HOSE_KG = HOSE_AREA * hose_length() * 960 + PI * (HOSE_R - 0.0023) ** 2 * hose_length() * 1000
STONE_KG = STONE_VOL * DENS_GRANITE
LID_KG = PI * R_PLATE ** 2 * 0.008 * 7850 + PI * (0.16 ** 2 - 0.15 ** 2) * 0.030 * 7850 + PI * 0.009 ** 2 * 0.08 * 7850
PUMP_SHELL_KG = 2 * (PUMP_L * PUMP_W + (PUMP_L + PUMP_W) * PUMP_TOP) * 0.002 * 7900
PUMP_CORE_KG = PI * 0.045 ** 2 * 0.10 * 7800
PUMP_BASE_KG = 0.55
PUMP_KG = PUMP_SHELL_KG + PUMP_CORE_KG + PUMP_BASE_KG
SHAFT_KG = (revolve_volume([(0, 0), (0, R_OUT), (0.78, R_OUT), (0.78, R_IN), (0.12, R_IN), (0.12, 0)])[0]) * 2400
SLAB_KG = (PI * R_OUT ** 2 - PI * R_HOLE ** 2) * SLAB_T * 2500
WATER_KG = PI * R_IN ** 2 * (Y_WATER - Y_FLOOR) * 1000
