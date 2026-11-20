"""🧭 Round 3, machinery/plate-heat-exchanger: the measures of the real object and the frames of its bones, shared by the volumes and the 2D drawings.

World frame (rest): x ahead (towards the supply-side end wall), y up, z lateral (left is +); origin at the centre of the base on the floor.
The casing is a 0.5 x 0.57 x 0.5 m galvanised box on two U-channel base rails 30 mm high; a cross-flow plate core of 0.318 m square plates
stands inside on one corner (diamond), its four corners sealed against the casing by a cradle, a top rail and two side seal rails, so the
casing is divided into four chambers; each chamber has a DN 200 spigot on its end wall (outdoor air upper left, exhaust lower left, extract
upper right, supply lower right); a bypass opening in the top panel is closed by a hinged flap.
"""
import math

RIG = {"frame": (-0.25, 0.0), "plates": (-0.225, 0.315), "ports": (-0.31, 0.315), "damper": (-0.2, 0.61)}
BASE = 0.03
TOP = 0.6
HALF = 0.25
CORE_Y = 0.315
CORE_HALF = 0.225
SPIGOT_Y = (0.16, 0.47)
SPIGOT_R = 0.1
SPIGOT_WALL = 0.001
SPIGOT_OUT = 0.06
FLANGE_R = 0.115
OPENING = 0.18
FLAP = 0.4
FLAP_HALF_Z = 0.2
BARREL_R = 0.01
RAIL_X = 0.19
RAIL_W = 0.04
RAIL_H = 0.03
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}


def loc(bone, x, y):
    """📍 World rest point (x, y) in the local frame of a bone (all bones have angle 0)."""
    ox, oy = RIG[bone]
    return [round(x - ox, 6), round(y - oy, 6)]


def circle(cx, cy, r, n=48, start=0.0):
    """⭕ A closed polygon approximating a circle, counter-clockwise."""
    return [[round(cx + r * math.cos(start + 2 * math.pi * k / n), 6), round(cy + r * math.sin(start + 2 * math.pi * k / n), 6)] for k in range(n)]


def rect(x0, y0, x1, y1):
    """▭ A counter-clockwise rectangle polygon."""
    return [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]


def walls(scale):
    """🧱 Panel thickness of the casing at a scale (the real 1 mm sheet drawn at the scale's smallest feature)."""
    return FEATURE[scale]


def core(scale):
    """💠 Half diagonals (horizontal a, vertical b) of the plate core at a scale: square plates at 1:50, the side seal rails merged at 1:100, every seal rail merged at 1:200."""
    t = walls(scale)
    if scale == 50:
        return CORE_HALF, CORE_HALF
    if scale == 100:
        return HALF - t + 0.001, CORE_HALF
    return HALF - t + 0.001, CORE_Y - (BASE + t) + 0.001


def cradle(scale):
    """🔻 Cradle under the bottom corner of the core (world x, y), V notch parallel to the core faces."""
    t = walls(scale)
    a, b = core(scale)
    slope = b / a
    if scale == 50:
        apex = CORE_Y - b - 0.014
    else:
        apex = CORE_Y - b + 0.002
    w = 0.05
    return [[-w, BASE + t], [w, BASE + t], [w, round(apex + slope * w, 6)], [0.0, round(apex, 6)], [-w, round(apex + slope * w, 6)]]


def top_rail(scale):
    """🔺 Top seal rail over the upper corner of the core (world x, y)."""
    t = walls(scale)
    a, b = core(scale)
    slope = b / a
    if scale == 50:
        apex = CORE_Y + b + 0.014
    else:
        apex = CORE_Y + b - 0.002
    w = 0.05
    return [[-w, TOP - t], [-w, round(apex - slope * w, 6)], [0.0, round(apex, 6)], [w, round(apex - slope * w, 6)], [w, TOP - t]]


def side_rail(sign):
    """◀ Side seal rail on an end wall over the side corner of the core at 1:50 (world x, y); sign +1 ahead, -1 behind."""
    inner = HALF - walls(50) + 0.001
    tip = CORE_HALF + 0.014
    d = 0.05
    pts = [[tip - d, CORE_Y - d], [inner, CORE_Y - d], [inner, CORE_Y + d], [tip - d, CORE_Y + d], [tip, CORE_Y]]
    out = [[round(sign * x, 6), y] for x, y in pts]
    return out if sign > 0 else out[::-1]


def diamond(a, b):
    """💎 The core outline (world x, y) counter-clockwise from its bottom corner."""
    return [[0.0, round(CORE_Y - b, 6)], [round(a, 6), CORE_Y], [0.0, round(CORE_Y + b, 6)], [round(-a, 6), CORE_Y]]


def damper_profile(scale):
    """🚪 The flap profile in its bone frame (hinge axis at the origin): blade lying on the lid; at 1:100 and 1:200 blade and hinge barrel are one profile."""
    r = BARREL_R
    low = round(TOP - RIG["damper"][1], 6)
    if scale == 50:
        return [[0.0, low], [FLAP, low], [FLAP, low + 0.0075], [0.0, low + 0.0075]]
    if scale == 100:
        top = low + 0.015
        start = math.degrees(math.asin(top / r))
        arc = [[round(r * math.cos(math.radians(d)), 6), round(r * math.sin(math.radians(d)), 6)] for d in [start + (180 - start) * k / 6 for k in range(7)]]
        return [[-r, low], [FLAP, low], [FLAP, top]] + arc
    return [[-r, low], [FLAP, low], [FLAP, low + 0.02], [-r, low + 0.02]]
