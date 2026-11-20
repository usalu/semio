"""🧰 Round 3, appliances/circulation-pump: the measures of the real pump (Grundfos UPS 25-60 180 class) and the small helpers both scripts share.

Frame: world x along the pipe, y up, z lateral (the motor shaft; the motor points to -z, towards the viewer of the elevation). All measures below are relative to the point where the pipe axis and the plane of the shaft meet (x 0, y 0), z as in the world. The housing bone starts at the left support and runs along the pipe (u = x + XH); the motor and rotor bones start on the shaft axis; the terminal bone stands up on the motor (u = y - TY, v = -x).
"""
import math

AXIS_Y = 0.051
XH = 0.104
WP = 0.035
TY = 0.040
TW = -0.028

CX, CY = 0.0015, -0.003
R_VOL = 0.048
W_BACK = 0.067
W_HOUSING = 0.008
W_FLANGE = 0.015
W_MOTOR = 0.0
W_FRONT = -0.067
FL = 0.042
FL_R = 0.010
R_STATOR = 0.042
W_STATOR = -0.060
SCREW = 0.034
R_BEZEL = 0.0305
R_PANE = 0.0265
R_THREAD = 0.0239
R_HEX = 0.03002
R_STUB = 0.014
R_BORE = 0.0125
X_NUT = (0.078, 0.097)
X_END = 0.110
BOX_TOP = 0.075


def r5(v):
    """🔢 Rounded to 0.01 mm."""
    return round(v + 0.0, 5)


def pt(p):
    """📍 A rounded point."""
    return [r5(c) for c in p]


def arc(cx, cy, r, a0, a1, n):
    """🌙 Points on an arc from a0 to a1 degrees, both ends included."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def rounded_square(half, r, n=5, cx=0.0, cy=0.0):
    """⬜ A square of half width `half` with corner radius r, counter-clockwise."""
    out = []
    for qx, qy, a in ((1, 1, 0), (-1, 1, 90), (-1, -1, 180), (1, -1, 270)):
        out += arc(cx + qx * (half - r), cy + qy * (half - r), r, a, a + 90, n)
    return [pt(p) for p in out]


def hexagon(r=R_HEX):
    """⬡ The union nut seen along the pipe as (y, z): corners up and down, flats to the sides."""
    return [pt((r * math.cos(math.radians(60 * k)), r * math.sin(math.radians(60 * k)))) for k in range(6)]


VOLUTE_50 = [(0.0120, 0.0462), (0.0140, 0.0480), (0.0570, 0.0480), (0.0610, 0.0468), (0.0640, 0.0440), (0.0660, 0.0395), (0.0670, 0.0330)]
VOLUTE_100 = [(0.0150, 0.0480), (0.0600, 0.0480), (0.0670, 0.0410)]
VOLUTE_200 = [(0.0080, 0.0480), (0.0610, 0.0480), (0.0670, 0.0420)]
NECK_50 = [(0.030, 0.0215), (0.064, 0.0195), (0.068, 0.0195), (0.0705, 0.0215), (0.072, R_THREAD), (0.0895, R_THREAD), (0.090, 0.0234)]
NECK_100 = [(0.030, 0.0205), (0.080, 0.0205)]
STATOR_50 = [(W_STATOR, 0.038)] + arc(-0.056, 0.038, 0.004, 180, 90, 4)[1:] + [(0.004, R_STATOR)]
STATOR_100 = [(-0.065, 0.038), (-0.061, R_STATOR), (0.004, R_STATOR)]
GLAND_50 = [(-0.002, 0.011), (0.005, 0.011), (0.005, 0.0095), (0.012, 0.0095), (0.0165, 0.0065)]
