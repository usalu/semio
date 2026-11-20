"""🏮 Shared geometry of `objects/street-lamp` for round 3: one set of hand-stated dimensions that both the 3D solids (`r3-street-lamp-model.py`) and the 2D drawings (`r3-street-lamp-roots.py`) are built from, so every drawing is the true view of the modelled body.

World: x ahead (along the bracket arm), y up, z lateral (left +). Bones: `foundation` at (0, 0), u = x; `pole` at (0, 0.12), u = y, v = -x; `arm` at (0, 5.80), u = x; `head` at (0.72, 5.80), u = x.
@see ./r3-objects-street-lamp.md
@see ./round3-brief.md
"""
import math

PLATE_TOP = 0.12
POLE_L = 5.88
POLE_R0, POLE_R1 = 0.085, 0.040
POLE_WALL = 0.004
TUBE_TOP = 5.866
ARM_Y = 5.80
ARM_L = 0.82
HEAD_U = 0.72
ARM_R = 0.0302
ARM_WALL = 0.003
CLAMP_R, CLAMP_H = 0.050, 0.100
BRACE_R, BRACE_WALL = 0.0135, 0.0026
BRACE_FOOT = (0.050, -0.455)
BRACE_TOP = (0.600, -0.028)
BAND_U0, BAND_U1, BAND_R = 5.195, 5.255, 0.053
DOOR_U0, DOOR_U1, DOOR_HALF = 0.55, 1.05, 0.045
BOLT_C = 0.12
NUT_AF, NUT_Y0, NUT_Y1 = 0.030, 0.120, 0.139
STUB_R, STUB_Y1 = 0.010, 0.150
RIM_V = -0.012
TRAY_GROW = 0.002

HEAD = [
    (0.000, 0.048, 0.034, -0.030),
    (0.012, 0.068, 0.041, -0.039),
    (0.035, 0.094, 0.046, -0.046),
    (0.080, 0.118, 0.049, -0.049),
    (0.150, 0.130, 0.050, -0.050),
    (0.250, 0.135, 0.048, -0.050),
    (0.340, 0.134, 0.044, -0.050),
    (0.410, 0.130, 0.038, -0.050),
    (0.450, 0.124, 0.031, -0.049),
    (0.478, 0.112, 0.022, -0.046),
    (0.495, 0.090, 0.012, -0.041),
    (0.502, 0.062, 0.004, -0.035),
    (0.505, 0.030, -0.002, -0.029),
]
HEAD_L = HEAD[-1][0]
COVER_N, TRAY_N = 2.4, 6.0
FIN_U = [0.200, 0.235, 0.270, 0.305, 0.340, 0.375]
FIN_T, FIN_RISE, FIN_Z, FIN_FOOT = 0.004, 0.014, 0.095, 0.015
LENS_U0, LENS_U1, LENS_Z, LENS_R = 0.150, 0.430, 0.0875, 0.020
LENS_Y0, LENS_Y1 = -0.0545, -0.0500
PLUME_U, PLUME_RX, PLUME_RY = 0.29, 0.15, 0.12
FOOT_LUG = [(0.048, -0.482), (0.085, -0.452), (0.085, -0.428), (0.048, -0.428)]
LUG_HALF = 0.006
EAR_X, EAR_BOLT_V, EAR_BOLT_AF, EAR_BOLT_Z = -0.0565, 0.022, 0.017, 0.021
JOINT_U, JOINT_T, JOINT_RISE = 0.17, 0.004, 0.0015
NOSE_INSET = 0.0015
CAP = [(5.846, 0.0), (5.846, 0.044), (5.866, 0.044), (5.8715, 0.0395), (5.876, 0.032), (5.879, 0.020), (5.880, 0.0)]


def r6(v):
    """🔢 Rounded to 0.1 mm / 6 decimals."""
    return round(v, 6)


def pole_r(u):
    """📐 Outer radius of the straight conical column at pole-local height u."""
    return POLE_R0 + (POLE_R1 - POLE_R0) * u / POLE_L


def head_at(u, rows=HEAD):
    """📏 (W, T, B) of the luminaire body at head-local u, linear between stations (the loft joins its rings straight)."""
    for (u0, w0, t0, b0), (u1, w1, t1, b1) in zip(rows, rows[1:]):
        if u0 <= u <= u1:
            k = 0.0 if u1 == u0 else (u - u0) / (u1 - u0)
            return (w0 + k * (w1 - w0), t0 + k * (t1 - t0), b0 + k * (b1 - b0))
    return rows[-1][1:] if u > rows[-1][0] else rows[0][1:]


def cover_sec(w, t, b):
    """🫧 The domed upper cover section: centre, half-width, half-height (its lower half hides inside the tray)."""
    bb = b + 0.004
    return ((t + bb) / 2, w, (t - bb) / 2)


def tray_sec(w, t, b):
    """🧺 The flat-bottomed lower tray section between the bottom and the parting line."""
    top = min(RIM_V, t - 0.002)
    return ((top + b) / 2, w + TRAY_GROW, (top - b) / 2)


def superellipse(cy, rx, ry, n, count=96):
    """⭕ Points (z, y) of a superellipse |z/rx|^n + |(y-cy)/ry|^n = 1, the kernel's parameterisation."""
    e = 2.0 / n
    sp = lambda c: math.copysign(abs(c) ** e, c)
    return [(rx * sp(math.cos(2 * math.pi * j / count)), cy + ry * sp(math.sin(2 * math.pi * j / count))) for j in range(count)]


def sp_top(cy, rx, ry, n, z):
    """⛰ Height of the upper rim of a superellipse section at lateral z."""
    q = abs(z) / rx
    return cy + ry * max(0.0, 1 - q ** n) ** (1 / n) if q < 1 else cy


def unit(v):
    """➡ Unit vector."""
    m = math.sqrt(sum(c * c for c in v))
    return tuple(c / m for c in v)


def quat_normal(n):
    """🔄 The unit quaternion [x, y, z, w] that turns the local z axis onto n (section plane normal), section x kept towards w."""
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


ALONG_U = quat_normal((1.0, 0.0, 0.0))


def loft(sections_u, n):
    """🧵 Loft sections along u from (u, centre v, rx, ry) tuples, every ring held upright (normal u)."""
    return [{"at": [r6(u), r6(cy), 0.0], "rx": r6(rx), "ry": r6(ry), "exponent": n, "rotation": ALONG_U} for u, cy, rx, ry in sections_u]


def cover_rows(rows=HEAD):
    """🫧 Cover loft rows (u, centre, rx, ry)."""
    last = len(rows) - 1
    return [(u - (NOSE_INSET if i == last else 0.0), *cover_sec(w, t, b)) for i, (u, w, t, b) in enumerate(rows)]


def tray_rows(rows=HEAD):
    """🧺 Tray loft rows (u, centre, rx, ry)."""
    return [(u, *tray_sec(w, t, b)) for u, w, t, b in rows]


def single_rows(rows, grow=TRAY_GROW):
    """🥚 Rows of a single merged housing loft (coarser scales): the full height between bottom and top."""
    return [(u, (t + b) / 2, w + grow, (t - b) / 2) for u, w, t, b in rows]


def front_outline(loft_rows_n, count=96):
    """🔭 Front view (z, v) outline of lofts: the convex hull of all their rings; the rings join straight, so the hull of the rings is the hull of the body."""
    pts = []
    for rows, n in loft_rows_n:
        for u, cy, rx, ry in rows:
            pts += superellipse(cy, rx, ry, n, count)
    return hull(pts)


def hull(points):
    """🪢 Convex hull (counter-clockwise) of 2D points."""
    p = sorted(set((round(x, 6), round(y, 6)) for x, y in points))
    if len(p) < 3:
        return p
    cross = lambda o, a, b: (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lo, up = [], []
    for q in p:
        while len(lo) >= 2 and cross(lo[-2], lo[-1], q) <= 0:
            lo.pop()
        lo.append(q)
    for q in reversed(p):
        while len(up) >= 2 and cross(up[-2], up[-1], q) <= 0:
            up.pop()
        up.append(q)
    return lo[:-1] + up[:-1]


def simplify(points, tol):
    """✂ Douglas-Peucker on a closed ring."""
    def dp(seg):
        if len(seg) < 3:
            return seg
        a, b = seg[0], seg[-1]
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy) or 1e-12
        far, idx = 0.0, 0
        for i in range(1, len(seg) - 1):
            d = abs(dy * (seg[i][0] - a[0]) - dx * (seg[i][1] - a[1])) / L
            if d > far:
                far, idx = d, i
        if far <= tol:
            return [a, b]
        return dp(seg[: idx + 1])[:-1] + dp(seg[idx:])
    i = max(range(len(points)), key=lambda k: points[k][0])
    ring = points[i:] + points[:i] + [points[i]]
    j = min(range(len(ring)), key=lambda k: ring[k][0])
    out = dp(ring[: j + 1])[:-1] + dp(ring[j:])[:-1]
    return out


def side_outline(rows, u0=0.0):
    """📐 Side elevation outline (u, v) of a housing: top line T and bottom line B between the end caps."""
    return [(r6(u0 + u), r6(t)) for u, w, t, b in rows] + [(r6(u0 + u), r6(b)) for u, w, t, b in reversed(rows)]


def plan_outline(rows, u0=0.0, grow=TRAY_GROW):
    """🗺 Plan outline (x, lateral) of a housing: the widest section half-width at every u."""
    return [(r6(u0 + u), r6(-(w + grow))) for u, w, t, b in rows] + [(r6(u0 + u), r6(w + grow)) for u, w, t, b in reversed(rows)]


def rounded_rect(x0, x1, y0, y1, r, steps=4):
    """▢ Closed rounded rectangle points."""
    pts = []
    for cx, cy, a0 in ((x1 - r, y0 + r, -90), (x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180)):
        for k in range(steps + 1):
            a = math.radians(a0 + 90 * k / steps)
            pts.append((r6(cx + r * math.cos(a)), r6(cy + r * math.sin(a))))
    return pts


def hexagon(af):
    """⬡ Hexagon (a, b) points with across-flats af along b and corners on a."""
    R = af / math.sqrt(3)
    return [(r6(R * math.cos(math.radians(60 * k))), r6(R * math.sin(math.radians(60 * k)))) for k in range(6)]


def fin_profile(u, steps=8):
    """🪭 Profile (v, w) of one transverse cooling fin at head-local u: foot buried in the cover, top edge 14 mm above the domed cover."""
    w, t, b = head_at(u)
    cy, rx, ry = cover_sec(w, t, b)
    zs = [-FIN_Z + 2 * FIN_Z * k / steps for k in range(steps + 1)]
    top = [(r6(sp_top(cy, rx, ry, COVER_N, z) + FIN_RISE), r6(z)) for z in zs]
    return [(FIN_FOOT, r6(FIN_Z))] + list(reversed(top)) + [(FIN_FOOT, r6(-FIN_Z))]


def joint_profile(count=48):
    """🪡 Profile (v, w) of the gear-hatch joint lip: the cover section at the joint grown by 1.5 mm, above the parting line."""
    w, t, b = head_at(JOINT_U)
    cy, rx, ry = cover_sec(w, t, b)
    ring = [(y, z) for z, y in superellipse(cy, rx + JOINT_RISE, ry + JOINT_RISE, COVER_N, count)]
    keep = [p[0] >= RIM_V for p in ring]
    start = next(i for i in range(count) if keep[i] and not keep[i - 1])
    return [ring[(start + i) % count] for i in range(count) if keep[(start + i) % count]]


def fin_top(u, z):
    """🪭 Top edge height of the fin at u and lateral z."""
    w, t, b = head_at(u)
    cy, rx, ry = cover_sec(w, t, b)
    return sp_top(cy, rx, ry, COVER_N, z) + FIN_RISE
