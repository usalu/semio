"""📐 Round 3, appliances/mvhr-unit: the measures of the wall unit and the frame helpers shared by the 2D and 3D scripts.

World frame of the asset (metres): X lateral (the 0.6 m width, + to the right in the side elevation), Y up from the lower edge of the casing,
Z depth (front cover at -0.15, wall at +0.15). Bones after the round-3 rig fix: `casing` stands on the vertical centreline (angle 90,
local u = Y, v = -X), `spigots` continues it from the top (origin Y 0.6, local u = Y - 0.6, v = -X), `impeller` keeps the world
axes at the window centre (origin X 0, Y 0.3, Z -0.13).

References: Vent-Axia Kinetic (550 x 550 x 285 mm, 15 kg, four 125 mm spigots on top), EnviroVent energiSava 260 (560 x 635 x 260 mm,
14 kg), ebm-papst RadiCal R3G190 (backward-curved shrouded impeller of 190 mm with an external-rotor motor in the hub).
"""
import math

R5 = lambda v: round(v + 0.0, 6)

W, H = 0.6, 0.6
EDGE_R = 0.025
Z_FRONT, Z_COVER, Z_BACK, Z_WALL = -0.150, -0.120, 0.135, 0.150
FILLET = 0.008
WIN_C = (0.0, 0.3)
GLASS_R, BEZEL_R = 0.112, 0.128
IMP_Z = -0.130
SPIGOTS = [("exhaust", -0.225, 0.07), ("extract", -0.075, -0.07), ("supply", 0.075, 0.07), ("outdoor", 0.225, -0.07)]
TUBE_RO, TUBE_RI, TUBE_H = 0.049, 0.0483, 0.062
SEAL_RO, SEAL_Y0 = 0.057, 0.048
LED = (0.215, 0.1, 0.014)
GRIP = (-0.08, 0.08, 0.030, 0.054)
CORE_HEX = [(-0.265, 0.30), (-0.135, 0.17), (0.135, 0.17), (0.265, 0.30), (0.135, 0.43), (-0.135, 0.43)]
CORE_Z = (-0.085, 0.110)
VOLUTE = (-0.165, 0.50, 0.045, 0.072, 0.572)
VOLUTE_Z = (0.0, 0.105)
FILTERS = [("extract-filter", -0.17, 0.02), ("outdoor-filter", 0.13, 0.275)]
FILTER_Y, FILTER_Z = (0.455, 0.49), (-0.125, -0.005)
LINER = 0.019
SHEET = 0.001
RAIL = [(0.525, 0.150), (0.586, 0.150), (0.586, 0.1435), (0.595, 0.1435), (0.595, 0.1405), (0.583, 0.1405), (0.583, 0.1475), (0.525, 0.1475)]
HOOK = [(0.520, 0.135), (0.600, 0.135), (0.600, 0.1465), (0.586, 0.1465), (0.586, 0.1440), (0.597, 0.1440), (0.597, 0.1375), (0.520, 0.1375)]
REST = (0.015, 0.045, 0.135, 0.150)
SPAN = 0.15


def casing_uv(x, y):
    """🧭 World (X, Y) to the bone-local (u, v) of `casing`."""
    return [R5(y), R5(-x)]


def spigot_uv(x, y):
    """🧭 World (X, Y) to the bone-local (u, v) of `spigots`."""
    return [R5(y - 0.6), R5(-x)]


def imp_xy(x, y):
    """🧭 World (X, Y) to the bone-local (x, y) of `impeller`."""
    return [R5(x - WIN_C[0]), R5(y - WIN_C[1])]


def rr(hx, hy, r, cx=0.0, cy=0.0, n=10):
    """⬜ Counter-clockwise rounded rectangle of half sizes hx, hy and corner radius r about (cx, cy)."""
    pts = []
    for k, (sx, sy, a0) in enumerate([(1, 1, 0), (-1, 1, 90), (-1, -1, 180), (1, -1, 270)]):
        ox, oy = cx + sx * (hx - r), cy + sy * (hy - r)
        for i in range(n + 1):
            a = math.radians(a0 + 90 * i / n)
            pts.append((ox + r * math.cos(a), oy + r * math.sin(a)))
    return pts


def circle(r, cx=0.0, cy=0.0, n=64):
    """⭕ Counter-clockwise circle polygon."""
    return [(cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)) for i in range(n)]


def side_outline(z0, z1, y0, y1, rf):
    """📐 Counter-clockwise outline (Z, Y) of the casing in the side view: front corners rounded by rf, back corners square."""
    pts = [(z1, y0), (z1, y1)]
    if rf > 0:
        for i in range(7):
            a = math.radians(90 + 90 * i / 6)
            pts.append((z0 + rf + rf * math.cos(a), y1 - rf + rf * math.sin(a)))
        for i in range(7):
            a = math.radians(180 + 90 * i / 6)
            pts.append((z0 + rf + rf * math.cos(a), y0 + rf + rf * math.sin(a)))
    else:
        pts += [(z0, y1), (z0, y0)]
    return pts


def volute():
    """🌀 Counter-clockwise outline (X, Y) of the extract-fan scroll: a spiral from the cutwater round one turn to a tangential outlet rising to the exhaust spigot."""
    cx, cy, r0, r1, top = VOLUTE
    spiral = []
    for i in range(49):
        t = 165 - 360 * i / 48
        r = r0 + (r1 - r0) * i / 48
        spiral.append((cx + r * math.cos(math.radians(t)), cy + r * math.sin(math.radians(t))))
    pts = spiral + [(spiral[-1][0], top), (spiral[0][0], top)]
    area = sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))
    return pts if area > 0 else pts[::-1]


def blade_line(r0=0.046, r1=0.094, sweep=0.75, theta0=0.32, n=8):
    """🪶 Centre line (impeller-local x, y) of one backward-curved blade: the angle falls with the radius, so the tip trails a counter-clockwise turn."""
    return [((r0 + (r1 - r0) * i / (n - 1)) * math.cos(theta0 - sweep * i / (n - 1)), (r0 + (r1 - r0) * i / (n - 1)) * math.sin(theta0 - sweep * i / (n - 1))) for i in range(n)]


def blade_profile(t=0.0025):
    """🪶 Closed outline of one blade of thickness t around its centre line."""
    line = blade_line()
    left, right = [], []
    for i, (x, y) in enumerate(line):
        a = line[max(i - 1, 0)]
        b = line[min(i + 1, len(line) - 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy)
        nx, ny = -dy / L, dx / L
        left.append((x + nx * t / 2, y + ny * t / 2))
        right.append((x - nx * t / 2, y - ny * t / 2))
    pts = left + right[::-1]
    area = sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))
    return pts if area > 0 else pts[::-1]


def rot(points, a):
    """🔄 Points turned by a radians about the origin."""
    c, s = math.cos(a), math.sin(a)
    return [(x * c - y * s, x * s + y * c) for x, y in points]


def poly_area(pts):
    """📏 Signed area of a polygon."""
    return 0.5 * sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))
