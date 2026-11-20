"""🧭 Round 3, machinery/sorption-wheel: the measures of the real machine (one place for the 3D solids and the 2D drawings) and the small geometry helpers both scripts use.

World frame: x ahead, y up, z lateral; the wheel axis is z through C = (0, CY). The elevation looks from -z at the wheel face.
"""
import math
import os

CY = 0.31
R_BAND = 0.31
T_SHEET = 0.001
R_FLANGE = 0.275
HALF_DEPTH = 0.11
R_ROTOR = 0.285
T_SHELL = 0.002
R_HUB = 0.045
R_SHAFT = 0.0125
SPOKE_T = 0.006
SPOKES = 8
BEAM_W = 0.04
BAR_W = 0.03
SECTOR_DEG = 45.0
FOOT_X0, FOOT_X1 = 0.08, 0.17
PLATE_X1 = 0.305
FOOT_HALF_Z = 0.09
MOTOR_X = 0.255
MOTOR_HALF = 0.035
GUARD_Z0, GUARD_Z1 = 0.0145, 0.0455
GUARD_A0, GUARD_A1 = -14.0, -75.0
BELT_Z0, BELT_Z1 = 0.025, 0.035
BELT_T = 0.005
PULLEY_R = 0.018
STEEL = "galvanised-steel"
GEL = "silica-gel"
DRAFT = os.environ.get("SW_DRAFT") == "1"
PAINT = STEEL if DRAFT else "painted-steel"
RUBBER = STEEL if DRAFT else "epdm"
MOTOR_REGION = "casing" if DRAFT else "motor"

SCALE = {
    50: {"rotorHalf": 0.1, "bar": 0.008, "plate": 0.008, "feature": 0.0075},
    100: {"rotorHalf": 0.095, "bar": 0.015, "plate": 0.015, "feature": 0.015},
    200: {"rotorHalf": 0.08, "bar": 0.03, "plate": 0.0, "feature": 0.03},
}


def r6(v):
    """🔢 Six decimals, no negative zero."""
    v = round(v, 6)
    return 0.0 if v == 0 else v


def pt(x, y):
    """📍 A rounded 2D point."""
    return [r6(x), r6(y)]


def polar(r, deg, cx=0.0, cy=0.0):
    """🧭 Point at radius r and angle deg about (cx, cy)."""
    a = math.radians(deg)
    return pt(cx + r * math.cos(a), cy + r * math.sin(a))


def arc(r, a0, a1, step=3.0, cx=0.0, cy=0.0):
    """🌙 Points of an arc from a0 to a1 degrees (either sense), chords of at most `step` degrees."""
    n = max(1, int(math.ceil(abs(a1 - a0) / step - 1e-9)))
    return [polar(r, a0 + (a1 - a0) * k / n, cx, cy) for k in range(n + 1)]


def sector(r0, r1, a0, a1, step=3.0, cx=0.0, cy=0.0):
    """🍰 Annular sector r0..r1 between a0 and a1 degrees, counter-clockwise."""
    return arc(r1, a0, a1, step, cx, cy) + arc(r0, a1, a0, step, cx, cy)


def matrix_sector(r0, r1, half, step=3.0):
    """🧇 One matrix field between two spokes at 0 and 360 / SPOKES degrees: the edges run parallel to the spoke flanks at the distance `half`."""
    span = 360.0 / SPOKES
    i = math.degrees(math.asin(half / r0))
    o = math.degrees(math.asin(half / r1))
    return arc(r1, o, span - o, step) + arc(r0, span - i, i, step)


def ring_y(x, r=R_BAND):
    """⭕ Height of the lower half of the circle of radius r about C at x."""
    return CY - math.sqrt(r * r - x * x)


def foot_profile(side, plate, x1=None, s=50):
    """🦶 Outline of one foot seen in the elevation (world): the plate on the ground and the gusset that follows the casing ring; side +1 is the motor side, x1 extends its plate as the motor plinth."""
    xs = [FOOT_X1, 0.155, 0.14, 0.125, 0.11, 0.095, FOOT_X0]
    top = [[x, ring_y(x, R_BAND - 0.0005)] for x in xs]
    if x1 is None or plate == 0:
        pts = [[FOOT_X0, 0.0], [FOOT_X1, 0.0]] + top
    else:
        pts = [[FOOT_X0, 0.0], [x1, 0.0], [x1, plate], [FOOT_X1, plate]] + top
    return [pt(side * x, y) for x, y in (pts if side > 0 else pts[::-1])]


def gusset_profile(side, plate):
    """📐 Outline of one gusset plate above the foot plate."""
    xs = [FOOT_X1, 0.155, 0.14, 0.125, 0.11, 0.095, FOOT_X0]
    pts = [[FOOT_X0, plate], [FOOT_X1, plate]] + [[x, ring_y(x, R_BAND - 0.0005)] for x in xs]
    return [pt(side * x, y) for x, y in (pts if side > 0 else pts[::-1])]


def motor_y(s):
    """⚙️ Height of the motor axis: the 70 mm frame stands on the plate (on the ground at 1:200)."""
    return SCALE[s]["plate"] + MOTOR_HALF


def guard_profile(s):
    """🛡️ Outline of the belt guard in the elevation (world): from the casing ring down to the plate and out to the motor side."""
    base = SCALE[s]["plate"]
    x_right = (R_BAND - 0.0005) * math.cos(math.radians(GUARD_A0))
    r = R_BAND - 0.0005
    reach = -math.degrees(math.asin(min(1.0, (CY - base) / r)))
    if reach > GUARD_A1:
        return arc(r, GUARD_A0, reach, 3.0, 0.0, CY) + [pt(x_right, base)]
    return arc(r, GUARD_A0, GUARD_A1, 3.0, 0.0, CY) + [pt(FOOT_X0, base), pt(x_right, base)]


def belt_loops(s):
    """➰ Outer and inner outline of the flat belt (world): wrapped round the rotor shell and the pulley, two straight runs between them."""
    px, py = MOTOR_X, motor_y(s)
    big, small = R_ROTOR + 0.0005 + BELT_T / 2, PULLEY_R + BELT_T / 2
    d = math.hypot(px, py - CY)
    phi = math.degrees(math.atan2(py - CY, px))
    beta = math.degrees(math.acos((big - small) / d))
    a1, a2 = phi + beta, phi - beta
    loops = []
    for off in (BELT_T / 2, -BELT_T / 2):
        loops.append(arc(big + off, a1, a2 + 360.0, 3.0, 0.0, CY) + arc(small + off, a2, a1, 10.0, px, py))
    return loops[0], loops[1]


def bar_profile(sign, r_end, mitre=True):
    """📏 Outline of one sector divider bar in the seal frame (origin C): centre line at sign * 45 degrees, BAR_W wide, cut parallel to the beam flank at the hub end when mitred."""
    c, s = math.cos(math.radians(SECTOR_DEG)), sign * math.sin(math.radians(SECTOR_DEG))
    h = BAR_W / 2
    n = (-s, c)
    out = []
    for k in (1, -1):
        t0 = (BEAM_W / 2 - k * h * n[0]) / c if mitre else (BEAM_W / 2 + h * abs(n[0])) / c
        t1 = math.sqrt(r_end * r_end - h * h)
        a, b = pt(t0 * c + k * h * n[0], t0 * s + k * h * n[1]), pt(t1 * c + k * h * n[0], t1 * s + k * h * n[1])
        out += [a, b] if k == 1 else [b, a]
    return out


def circle(r, cx, cy, n=16):
    """⚪ A regular polygon for a round hole."""
    return [polar(r, 360.0 * k / n, cx, cy) for k in range(n)]


def v_profile(r_end):
    """🔱 Outline of the sector frame of one face in the seal frame (origin C): the two divider bars at +45 and -45 degrees welded into one V whose root is cut along the beam flank."""
    up, down = bar_profile(1, r_end), bar_profile(-1, r_end)
    h = BAR_W / 2 / math.cos(math.radians(SECTOR_DEG))
    return [up[0], up[1], up[2], pt(h, 0.0), down[1], down[2], down[3]]
