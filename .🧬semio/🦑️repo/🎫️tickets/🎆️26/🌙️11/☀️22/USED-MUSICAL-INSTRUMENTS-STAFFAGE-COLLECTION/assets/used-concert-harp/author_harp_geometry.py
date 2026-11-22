"""🪕 Geometry of the used concert harp: every hand-chosen dimension, the rig at rest and the solids of every part, in metres.

World frame of the staffage archive: x ahead (from the soundbox towards the column), y up, z to the left of the harp
(the harpist's left). The harp frame `h` has its origin at the contact of the rear feet; in the stored pose it stands on the
trolley deck, so world = harp frame + (0, DECK_TOP, 0).

@see ../../brief.md
@see ../../reports/used-concert-harp.md
"""

import math

T_DECK = 0.091
BASE_X0, BASE_X1, BASE_Z, BASE_Y0, BASE_Y1 = -0.03, 0.56, 0.215, 0.025, 0.13
FEET_X, FEET_Z, FOOT_R = (0.02, 0.50), 0.17, 0.02
COL_X, COL_Y0, COL_TOP = 0.48, 0.13, 1.85
SHAFT_Y0, SHAFT_Y1, SHAFT_R0, SHAFT_R1 = 0.25, 1.60, 0.036, 0.034
SB_ORIGIN, SB_ANGLE, SB_LEN = (0.34, 0.264), 122.4, 1.46
SB_HW0, SB_HW1, SB_DP0, SB_DP1, SB_EXP = 0.225, 0.065, 0.25, 0.10, 2.3
BOARD_T = 0.008
STRIP_T, STRIP_W = 0.008, 0.018
NECK_HALF, PLATE_T, PLATE_BAND = 0.024, 0.003, 0.08
N_STRINGS, STRING_U0, STRING_U1 = 47, 0.10, 1.40
PEDAL_PIVOT, PEDAL_LEN, PEDAL_NOTCH_DEG = (0.20, 0.085), 0.30, 3.44
PEDALS = [("d", 0.14), ("c", 0.10), ("b", 0.06), ("e", -0.06), ("f", -0.10), ("g", -0.14), ("a", -0.18)]
TROLLEY_X0, TROLLEY_LEN, TROLLEY_HW, DECK_Y0, DECK_T, CARPET_T, RIM = -0.08, 0.68, 0.25, 0.068, 0.018, 0.005, 0.020
CASTOR_X, CASTOR_Z, WHEEL_R, WHEEL_W = (-0.01, 0.53), 0.19, 0.025, 0.020
COVER_HEM, COVER_PAD, COVER_MARGIN = 0.20, 0.015, 0.012
COVER_TURN_Z = 0.42
TILT_DEG = 18.0


def r6(x):
    """🎯️ Rounds to micrometres."""
    return round(x + 0.0, 6)


def dirv(angle):
    """🧭️ The unit vector of an angle in degrees."""
    a = math.radians(angle)
    return (math.cos(a), math.sin(a))


def hpt(hx, hy):
    """🪜️ A point of the harp frame in world rest coordinates of the stored pose."""
    return (hx, hy + T_DECK)


SB_U, SB_V = dirv(SB_ANGLE), dirv(SB_ANGLE + 90)


def sb_point(u, v):
    """🪵️ A point of the soundbox frame (u along the soundboard, v into the box) in the harp frame."""
    return (SB_ORIGIN[0] + u * SB_U[0] + v * SB_V[0], SB_ORIGIN[1] + u * SB_U[1] + v * SB_V[1])


def sb_hw(u):
    """↔️ Half width of the soundbox at u."""
    return SB_HW0 + (SB_HW1 - SB_HW0) * u / SB_LEN


def sb_dp(u):
    """↕️ Depth of the soundbox back at u."""
    return SB_DP0 + (SB_DP1 - SB_DP0) * u / SB_LEN


NECK_UNDER = [(0.47, 1.716), (0.36, 1.712), (0.25, 1.70), (0.12, 1.678), (0.0, 1.652), (-0.12, 1.62), (-0.24, 1.585), (-0.34, 1.553), (-0.40, 1.532)]
NECK_TOP = [(-0.47, 1.625), (-0.40, 1.664), (-0.28, 1.71), (-0.15, 1.754), (0.0, 1.789), (0.15, 1.807), (0.28, 1.812), (0.38, 1.806), (0.47, 1.80)]


def neck_profile():
    """🎻️ The side outline of the neck in the harp frame, from the crown along the top, down the scroll onto the soundbox top face and back along the string line."""
    s_board = sb_point(SB_LEN - 0.002, -BOARD_T)
    s_back = sb_point(SB_LEN - 0.002, sb_dp(SB_LEN) - 0.004)
    scroll = [(-0.515, 1.592), (-0.538, 1.548), (-0.536, 1.50)]
    face = [s_back, s_board]
    return list(reversed(NECK_TOP)) + scroll + face + list(reversed(NECK_UNDER))


def neck_under_y(x):
    """📏️ Height of the neck string line (underside) above x in the harp frame."""
    pts = sorted(NECK_UNDER)
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if x0 <= x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return pts[0][1] if x < pts[0][0] else pts[-1][1]


def plate_profile():
    """🟨️ The brass action plate band along the string line of the neck, in the harp frame."""
    under = [p for p in NECK_UNDER if -0.38 <= p[0] <= 0.44]
    return [(x, y + 0.002) for x, y in reversed(under)] + [(x, y + PLATE_BAND) for x, y in under]


CROWN = [(1.60, 0.036), (1.612, 0.044), (1.632, 0.05), (1.662, 0.062), (1.70, 0.072), (1.74, 0.075), (1.772, 0.073), (1.80, 0.066), (1.82, 0.06), (1.836, 0.058), (1.846, 0.05), (1.85, 0.03)]
COLUMN_FOOT = [(0.13, 0.058), (0.146, 0.058), (0.158, 0.051), (0.172, 0.055), (0.198, 0.05), (0.222, 0.044), (0.242, 0.04), (0.25, 0.036)]


def strings():
    """🎼️ The 47 strings C1 to G7: (index, note, harp-frame foot point, top height, diameter, material)."""
    names = "CDEFGAB"
    out = []
    for i in range(N_STRINGS):
        u = STRING_U0 + (STRING_U1 - STRING_U0) * i / (N_STRINGS - 1)
        foot = sb_point(u, -BOARD_T - STRIP_T)
        top = neck_under_y(foot[0])
        octave = 1 + i // 7
        note = names[i % 7]
        if i < 12:
            d, mat = 0.0021 - 0.00006 * i, "wound-string"
        elif i < 41:
            d, mat = 0.0018 - 0.000028 * (i - 12), "gut-string"
        else:
            d, mat = 0.00095 - 0.00003 * (i - 41), "nylon-string"
        out.append((i, f"{note}{octave}", foot, top, d, mat))
    return out
