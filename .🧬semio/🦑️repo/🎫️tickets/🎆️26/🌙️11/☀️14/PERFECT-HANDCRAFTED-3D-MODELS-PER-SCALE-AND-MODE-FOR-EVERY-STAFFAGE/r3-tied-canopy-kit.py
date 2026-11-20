"""🪢 Round 3 tied canopy: the hand-stated construction of the canopy (system line, pins, gussets, fork heads, glass layout, clamps, anchor bolts) shared by the volume script and the 2D root script.

All values are bone-local metres of the bone `canopy` (joint at the wall face, beam axis, +x along the beam, +y up, z lateral).

See: r3-components-tied-canopy.md (reference table), round3-brief.md
"""

import math

ANGLE = math.radians(144.246113)
DIR = (math.cos(ANGLE), math.sin(ANGLE))
NODE = (4.0, 0.0)


def on_line(t):
    """📍 Point of the strut system line at parameter t (metres from the beam node towards the wall)."""
    return (NODE[0] + t * DIR[0], NODE[1] + t * DIR[1])


PB = on_line(0.17 / DIR[1])
PW = on_line((0.09 - NODE[0]) / DIR[0])
WALL_NODE = on_line(-NODE[0] / DIR[0])
L = math.hypot(PW[0] - PB[0], PW[1] - PB[1])
AXIS_B = DIR
AXIS_W = (-DIR[0], -DIR[1])

FLANGE_TOP = 0.08
BEAM_END = 5.5
END_PLATE = 0.008
BEARING = 0.008
GLASS_T = 0.0168
GLASS_Y0 = FLANGE_TOP + BEARING
GLASS_Y1 = GLASS_Y0 + GLASS_T
CLAMP_T = 0.008
GLASS_X0 = 0.03
JOINT = 0.02
PANE = (BEAM_END - GLASS_X0 - 3 * JOINT) / 4
PANES = [(GLASS_X0 + i * (PANE + JOINT), GLASS_X0 + i * (PANE + JOINT) + PANE) for i in range(4)]
JOINTS = [PANES[i][1] for i in range(3)]
CLAMPS = [GLASS_X0 + 0.03] + [PANES[i][1] + JOINT / 2 for i in range(3)] + [BEAM_END - 0.03]
CLAMP_L = 0.06
CLAMP_W = 0.08

LUG_R = 0.04
GUSSET_HALF = 0.0075
CHEEK_IN = 0.0085
CHEEK_OUT = 0.0185
CHEEK_R = 0.03
PIN_R = 0.012
PIN_HEAD_R = 0.018
PIN_Z0 = -0.0265
PIN_LEN = 0.053
TUBE_R = 0.03015
TUBE_WALL = 0.0032
CHEEK_END = 0.105
BRIDGE_FROM = 0.065
SHANK_R = 0.018
FORK_BODY = [[0.095, 0.0], [0.095, SHANK_R], [0.115, SHANK_R], [0.145, TUBE_R], [0.16, TUBE_R], [0.16, 0.0]]
TUBE_FROM = 0.15

BOLT_Y = 0.073
BOLT_Z = 0.058
NUT_R = 0.01386
NUT_H = 0.013
PLATE = (0.02, 0.2, 0.16)
STRUT_PLATE_Y = WALL_NODE[1]


def r6(v):
    """✂️ Rounds to micrometres for stable JSON."""
    return round(v + 0.0, 6)


def pt(p):
    """🔢 A rounded 2D point."""
    return [r6(p[0]), r6(p[1])]


def along(origin, axis, s, n):
    """🧭 The point s along the axis and n across it (to the left of the axis in the xy plane)."""
    return (origin[0] + s * axis[0] - n * axis[1], origin[1] + s * axis[1] + n * axis[0])


def circle(c, r, n=32, a0=0.0, a1=2 * math.pi):
    """⭕ Points of a circle arc."""
    return [(c[0] + r * math.cos(a0 + (a1 - a0) * i / n), c[1] + r * math.sin(a0 + (a1 - a0) * i / n)) for i in range(n + 1)]


def hull(points):
    """🪢 Convex hull, counter-clockwise (monotone chain)."""
    p = sorted(set((round(x, 7), round(y, 7)) for x, y in points))
    cross = lambda o, a, b: (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lower, upper = [], []
    for q in p:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], q) <= 0:
            lower.pop()
        lower.append(q)
    for q in reversed(p):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], q) <= 0:
            upper.pop()
        upper.append(q)
    return lower[:-1] + upper[:-1]


def gusset_beam():
    """🔺 Gusset welded on the top flange: base 3.735-3.995 m (overlapping the flange by 2.5 mm), lug of radius 40 mm round the beam pin."""
    return hull([(3.735, FLANGE_TOP - 0.0025), (3.995, FLANGE_TOP - 0.0025)] + circle(PB, LUG_R, 48))


def gusset_wall():
    """🔻 Gusset welded on the strut anchor plate: base along the plate face, lug of radius 40 mm round the wall pin."""
    return hull([(-0.002, STRUT_PLATE_Y - 0.095), (-0.002, STRUT_PLATE_Y + 0.095)] + circle(PW, LUG_R, 48))


def cheek(pin, axis):
    """🍴 One cheek of a fork head: rounded round the pin (radius 30 mm), 60 mm wide, square-ended 105 mm from the pin where the bridge and the shank join it."""
    back = [along(pin, axis, CHEEK_R * math.cos(a), CHEEK_R * math.sin(a)) for a in [math.pi / 2 + math.pi * i / 16 for i in range(17)]]
    return back + [along(pin, axis, CHEEK_END, -CHEEK_R), along(pin, axis, CHEEK_END, CHEEK_R)]


def bridge(pin, axis):
    """🧩 The bridge that closes the fork slot behind the gusset lug (65-105 mm from the pin)."""
    return [along(pin, axis, s, n) for s, n in ((BRIDGE_FROM, -CHEEK_R), (CHEEK_END, -CHEEK_R), (CHEEK_END, CHEEK_R), (BRIDGE_FROM, CHEEK_R))]


def fork_outline(pin, axis, upto=0.16):
    """🔱 Elevation outline of a whole fork head (cheeks, shank and end cone) up to `upto` along the axis."""
    back = [along(pin, axis, CHEEK_R * math.cos(a), CHEEK_R * math.sin(a)) for a in [math.pi / 2 + math.pi * i / 16 for i in range(17)]]
    lower = [(CHEEK_END, -CHEEK_R), (CHEEK_END, -SHANK_R), (0.115, -SHANK_R), (0.145, -TUBE_R), (upto, -TUBE_R)]
    upper = [(upto, TUBE_R), (0.145, TUBE_R), (0.115, SHANK_R), (CHEEK_END, SHANK_R), (CHEEK_END, CHEEK_R)]
    return back + [along(pin, axis, s, n) for s, n in lower] + [along(pin, axis, s, n) for s, n in upper]


def node_outline(pin, axis, base):
    """🔩 The node of 1:200: gusset and fork head merged into one lug outline."""
    return hull(base + circle(pin, LUG_R, 32) + [along(pin, axis, 0.10, CHEEK_R), along(pin, axis, 0.10, -CHEEK_R)])


def x_range_at(poly, y0, y1):
    """↔️ The x extent of a convex polygon between two heights."""
    xs = []
    n = len(poly)
    for i in range(n):
        a, b = poly[i], poly[(i + 1) % n]
        for y in (y0, y1):
            if (a[1] - y) * (b[1] - y) <= 0 and a[1] != b[1]:
                xs.append(a[0] + (y - a[1]) / (b[1] - a[1]) * (b[0] - a[0]))
        for p in (a,):
            if y0 <= p[1] <= y1:
                xs.append(p[0])
    return min(xs), max(xs)


NOTCH = (round(x_range_at(gusset_beam(), GLASS_Y0, GLASS_Y1)[0] - 0.01, 3), PANES[2][1])
NOTCH_HALF = 0.0125
GUSSET_AT_GLASS = x_range_at(gusset_beam(), GLASS_Y0, GLASS_Y1)


def hexagon(cy, cz, r=NUT_R):
    """⬡ A hexagon (nut seen along its bolt) in the yz plane, corners up and down."""
    return [[r6(cy + r * math.cos(math.pi / 2 + i * math.pi / 3)), r6(cz + r * math.sin(math.pi / 2 + i * math.pi / 3))] for i in range(6)]


def node_beam():
    """🔩 The 1:200 node lug on the beam."""
    return node_outline(PB, AXIS_B, [(3.735, FLANGE_TOP - 0.0025), (3.995, FLANGE_TOP - 0.0025)])


def node_wall():
    """🔩 The 1:200 node lug at the wall."""
    return node_outline(PW, AXIS_W, [(-0.002, STRUT_PLATE_Y - 0.095), (-0.002, STRUT_PLATE_Y + 0.095)])
