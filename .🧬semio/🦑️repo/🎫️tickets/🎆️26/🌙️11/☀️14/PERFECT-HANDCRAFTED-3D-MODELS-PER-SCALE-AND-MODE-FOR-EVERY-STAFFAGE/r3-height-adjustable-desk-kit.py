"""🗜️ Round 3, furniture/height-adjustable-desk: the measured construction of a dual-motor sit-stand desk, stated once by hand.

World metres at rest (= pose seated-height): x along the 1.6 m top (left end 0), y up, z towards the back (the user sits at -z, the
front edge is z = -0.4). Bone frames: foot u = x - foot joint, v = y; stage and column u = y - joint, v = column axis - x; top u = x,
v = y - 0.695. `r3-height-adjustable-desk-model.py` (solids) and `r3-height-adjustable-desk-draw.py` (2D) read these numbers.
"""
import math

COLUMN_X = (0.25, 1.35)
FOOT_JOINT_X = (0.21, 1.31)
FOOT_U = 0.04
STAGE_Y = 0.05
COLUMN_U0 = 0.08
COLUMN_Y = STAGE_Y + COLUMN_U0
STAGE_LEN = 0.43
COLUMN_LEN = 0.565
TOP_Y = COLUMN_Y + COLUMN_LEN
TRAVEL = 0.215

FOOT_W = 0.08
FOOT_L = 0.70
FOOT_UNDER = 0.01
FOOT_END_H = 0.028
FOOT_MID_H = 0.045
FOOT_MID_HALF = 0.07
GLIDE_R = 0.02
GLIDE_W = 0.31

SLEEVE = (0.064, 0.094, 0.005)
SLEEVE_V = (0.03, 0.39)
SLEEVE_COLLAR = (0.068, 0.098, 0.006)
SLEEVE_COLLAR_V = (0.39, 0.405)
STAGE = (0.056, 0.086, 0.004)
STAGE_U = (0.0, 0.425)
STAGE_COLLAR = (0.06, 0.09, 0.005)
STAGE_COLLAR_U = (0.425, 0.44)
COLUMN = (0.048, 0.078, 0.004)
COLUMN_U = (0.0, 0.54)
DRIVE = (0.08, 0.15, 0.008)
DRIVE_U = (0.44, 0.54)

BOARD = (1.6, 0.8, 0.025, 0.012)
LAMINATE_T = 0.001
BRACKET = (0.06, 0.025, 0.66)
CROSSBAR_V = (-0.115, -0.065)
CROSSBAR_W = 0.03
CROSSBAR_U = (COLUMN_X[0] + 0.024, COLUMN_X[1] - 0.024)
FLANGE_T = 0.008
FLANGE_V = (-0.122, -0.058)
FLANGE_W = 0.07
TRAY_U = (0.45, 1.15)
TRAY_W = (0.20, 0.34)
TRAY_DEPTH = 0.11
TRAY_T = 0.0015
TRAY_FLANGE = 0.015
CONTROL = ((0.80, 1.02), (-0.035, 0.0), (0.06, 0.145))
HANDSET = ((1.38, 1.51), (-0.024, 0.0), (-0.412, -0.332))

MON_X = 0.6
MON_BASE = ((0.475, 0.725), (0.025, 0.043), (0.06, 0.26))
MON_NECK = ((0.57, 0.63), (0.043, 0.36), (0.175, 0.20))
MON_HINGE = ((0.56, 0.64), (0.27, 0.34), (0.155, 0.18))
MON_PANEL = ((0.293, 0.907), (0.117, 0.485), (0.115, 0.13))
MON_SCREEN = ((0.301, 0.899), (0.141, 0.477), (0.1135, 0.1155))
MON_BULGE_V = 0.29
MON_BULGE = ((0.128, 0.26, 0.15), (0.15, 0.24, 0.14), (0.168, 0.18, 0.10))
LAP_BASE = ((0.89, 1.25), (0.025, 0.037), (-0.205, 0.045))
LAP_LID = ((0.891, 1.249), (0.037, 0.045), (-0.204, 0.044))
LAP_X = 1.07

STEEL = 7850.0


def text(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def r(value, digits=5):
    """🎯 Rounded number for the JSON."""
    return round(value + 0.0, digits)


def rounded_rect(a0, a1, b0, b1, radius, steps=3):
    """🔲 Rounded rectangle [a, b] points (clockwise in a-b), `steps` segments per corner; radius 0 gives the four corners."""
    if radius <= 0:
        return [[r(a0), r(b0)], [r(a0), r(b1)], [r(a1), r(b1)], [r(a1), r(b0)]]
    pts = []
    corners = [(a0 + radius, b0 + radius, 1.5 * math.pi), (a0 + radius, b1 - radius, math.pi), (a1 - radius, b1 - radius, math.pi / 2), (a1 - radius, b0 + radius, 0.0)]
    for ca, cb, start in corners:
        for k in range(steps + 1):
            t = start - (math.pi / 2) * k / steps
            pts.append([r(ca + radius * math.cos(t)), r(cb + radius * math.sin(t))])
    return pts


def centred(width, depth, radius, steps=3):
    """🔳 Rounded rectangle centred on the origin."""
    return rounded_rect(-width / 2, width / 2, -depth / 2, depth / 2, radius, steps)


def tube_kg(a, b, wall, length):
    """⚖️ Mass of a rectangular steel tube a x b with the given wall."""
    return 2 * (a + b - 2 * wall) * wall * length * STEEL


def masses():
    """⚖️ Bone masses (kg, com in bone-local u, v) from the construction: tubes x wall x 7850 kg/m3, pressed foot, motor, top parts."""
    foot = 3.2
    sleeve = tube_kg(SLEEVE[0], SLEEVE[1], 0.002, SLEEVE_V[1] - SLEEVE_V[0])
    foot_bone = (foot + sleeve, FOOT_U, (foot * 0.024 + sleeve * (SLEEVE_V[0] + SLEEVE_V[1]) / 2) / (foot + sleeve))
    stage = tube_kg(STAGE[0], STAGE[1], 0.0018, STAGE_U[1]) + 0.1
    stage_bone = (stage, (STAGE_U[1] + 0.015) / 2, 0.0)
    tube = tube_kg(COLUMN[0], COLUMN[1], 0.0018, COLUMN_U[1])
    drive = 2.5
    column_bone = (tube + drive, (tube * COLUMN_U[1] / 2 + drive * (DRIVE_U[0] + DRIVE_U[1]) / 2) / (tube + drive), 0.0)
    board = BOARD[0] * BOARD[1] * BOARD[2] * 650
    bracket = 2 * tube_kg(BRACKET[0], BRACKET[1], 0.002, BRACKET[2])
    crossbar = tube_kg(0.05, CROSSBAR_W, 0.0015, CROSSBAR_U[1] - CROSSBAR_U[0])
    tray = 2 * (TRAY_DEPTH + TRAY_FLANGE) * (TRAY_U[1] - TRAY_U[0]) * TRAY_T * STEEL + (TRAY_W[1] - TRAY_W[0]) * (TRAY_U[1] - TRAY_U[0]) * TRAY_T * STEEL
    control, handset = 0.5, 0.15
    parts = [(board, 0.8, BOARD[2] / 2), (bracket, 0.8, -BRACKET[1] / 2), (crossbar, 0.8, sum(CROSSBAR_V) / 2), (tray, 0.8, -TRAY_DEPTH / 2), (control, sum(CONTROL[0]) / 2, -0.0175), (handset, sum(HANDSET[0]) / 2, -0.012)]
    kg = sum(p[0] for p in parts)
    top_bone = (kg, sum(p[0] * p[1] for p in parts) / kg, sum(p[0] * p[2] for p in parts) / kg)
    return {"foot": foot_bone, "stage": stage_bone, "column": column_bone, "top": top_bone, "board": board, "frame": 2 * (foot_bone[0] + stage + tube + drive) + kg - board}


if __name__ == "__main__":
    for k, v in masses().items():
        print(k, v)
