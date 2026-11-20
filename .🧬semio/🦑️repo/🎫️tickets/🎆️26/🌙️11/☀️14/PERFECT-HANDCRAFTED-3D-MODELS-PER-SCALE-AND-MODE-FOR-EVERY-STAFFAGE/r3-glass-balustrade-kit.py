"""🪟 Round 3 glass balustrade kit: the dimensions of the clamped glass balustrade shared by the 2D root script and the volume script.

Construction (DIN 18008-4 category B, formerly TRAV): laminated safety glass 2 x 10 mm with 1.52 mm PVB, clamped over 100 mm
between two 15 mm steel clamp plates with 10 mm elastomer strips, through-bolts M12 at 300 mm in sleeved glass holes,
the fixed plate welded to a 300 x 15 mm base plate anchored with M12 studs at 400 mm, a slotted 42.4 x 2 mm tube handrail
with a U gasket over the glass edge and dished end caps.

Frame: u up along the bone `panel`, v across the panel (the fixed plate is on -v), w along the panel (left is +).

See: r3-components-glass-balustrade.md, round3-brief.md
"""

import math

HALF_LENGTH = 1.0
PLATE_T = 0.015
PLATE_HALF = 0.15
GLASS_HALF = 0.01075
PANE = 0.01
PVB_HALF = 0.00075
PAD = 0.01
CHEEK = 0.015
SLOT_HALF = GLASS_HALF + PAD
CLAMP_HALF = SLOT_HALF + CHEEK
SHOE_TOP = 0.125
GLASS_BOTTOM = 0.025
GLASS_TOP = 1.08
GLASS_HALF_LENGTH = 0.98
WELD = 0.0085
BOLT_U = 0.075
BOLT_W = [-0.9, -0.6, -0.3, 0.0, 0.3, 0.6, 0.9]
BOLT_R = 0.006
BOLT_HOLE_R = 0.007
SLEEVE_R = 0.013
HEX_AF = 0.019
HEAD_K = 0.0075
NUT_M = 0.0108
STUB = 0.005
ANCHOR_V = 0.11
ANCHOR_W = [-0.8, -0.4, 0.0, 0.4, 0.8]
ANCHOR_STUD = 0.019
BLOCK_W = [-0.65, 0.65]
BLOCK_LENGTH = 0.1
RAIL_U = 1.079
RAIL_R = 0.0212
RAIL_WALL = 0.002
RAIL_SLOT_HALF = 0.0135
RAIL_HALF_LENGTH = 0.992
CAP = 0.008
GASKET_BOTTOM = 1.0578
GASKET_TOP = 1.083


def r6(v):
    """🎯 Round to a micrometre."""
    return round(v + 0.0, 6)


def circle(cx, cy, r, n=24, phase=0.0):
    """⭕ A regular polygon of n corners on the circle of radius r about (cx, cy), counter-clockwise."""
    return [[r6(cx + r * math.cos(phase + 2 * math.pi * i / n)), r6(cy + r * math.sin(phase + 2 * math.pi * i / n))] for i in range(n)]


def hexagon(cx, cy, af=HEX_AF, phase=0.0):
    """⬡ A hexagon of the given width across flats about (cx, cy)."""
    return circle(cx, cy, af / math.sqrt(3), 6, phase)


def rect(a0, b0, a1, b1):
    """▭ A rectangle between two corners, counter-clockwise."""
    return [[r6(a0), r6(b0)], [r6(a1), r6(b0)], [r6(a1), r6(b1)], [r6(a0), r6(b1)]]


def arc(cu, r, half, n, inner=False):
    """🌙 Points (u, v) on the circle of radius r about (cu, 0) from the slot edge v = +half round the crown to v = -half; reversed for the inner wall."""
    t0 = math.asin(half / r)
    pts = [[r6(cu - r * math.cos(t0 + (2 * math.pi - 2 * t0) * i / n)), r6(r * math.sin(t0 + (2 * math.pi - 2 * t0) * i / n))] for i in range(n + 1)]
    return list(reversed(pts)) if inner else pts


def slotted_tube(half, n):
    """🧲 The section of the slotted handrail tube: a ring of RAIL_WALL with a slot of the given half width at its underside."""
    return arc(RAIL_U, RAIL_R, half, n) + arc(RAIL_U, RAIL_R - RAIL_WALL, half, n, inner=True)


def notched_disc(half, n):
    """🔑 The section of the handrail as a solid round with a notch of the given half width from below to its centre."""
    return arc(RAIL_U, RAIL_R, half, n) + [[r6(RAIL_U), r6(-half)], [r6(RAIL_U), r6(half)]]
