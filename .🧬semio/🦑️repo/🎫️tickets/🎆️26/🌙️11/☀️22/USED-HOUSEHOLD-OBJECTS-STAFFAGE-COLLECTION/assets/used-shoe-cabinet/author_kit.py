"""🧰 Hand-chosen numbers and helpers of the used hallway shoe cabinet: every dimension the build script draws and models from.

World frame of the staffage: x ahead (the flaps face +x), y up, z left. The cabinet stands with its back panel at x = 0 on a folded moving blanket.
See ../../brief.md and the report ../../reports/used-shoe-cabinet.md (reference table)."""
import math

T = 0.016
TB = 0.003
W = 0.80
HW = W / 2
D = 0.170
H = 1.450
CARCASS_FRONT = 0.154
INNER = HW - T
BLANKET_T = 0.015
BLANKET_X0 = -0.03
BLANKET_L = 0.60
BLANKET_HW = 0.45
PLINTH_H = 0.060
BOTTOM = (0.060, 0.076)
DIVIDERS = [(0.5105, 0.5265), (0.9685, 0.9845)]
TOP = (1.434, 1.450)
FLAP_H = 0.455
FLAP_GAP = 0.003
FLAP_BOTTOMS = [0.062, 0.520, 0.978]
FLAP_HW = 0.398
OPEN_DEG = 40.0
CHEEK_T = 0.012
CHEEK_W = (0.366, 0.378)
CHEEK = [[0.035, 0.016], [0.225, 0.130], [0.270, 0.130], [0.370, 0.016]]
TRAY_T = 0.010
HANDLE_U = (0.410, 0.420)
HANDLE_BAR_V = (-0.025, -0.015)
HANDLE_LEG_V = (-0.015, 0.0)
HANDLE_ORIGINAL = {"bar": 0.074, "leg": 0.064}
HANDLE_REPLACED = {"bar": 0.096, "leg": 0.080}
LEG_HALF = 0.005
BRACKET_W = (-0.3125, -0.2875)
BRACKET_L = 0.040
BRACKET_H = 0.025
BRACKET_S = 0.002
NOTCH_U = (0.003, 0.015)
NOTCH_W = (0.285, 0.315)
CRATER_DEPTH = 0.008
SHOE_PAIRS = 4
SHOE_PAIR_KG = 0.8
G = 9.80665

RHO = {"chipboard-foil": 650.0, "hdf-back": 850.0, "aluminium-handle": 2700.0, "brushed-steel": 7850.0, "zinc-plated-steel": 7850.0, "polyester-nonwoven": 148.0}


def rect(u0, u1, v0, v1):
    """⬜ The counter-clockwise rectangle [u0, u1] x [v0, v1] as a profile."""
    return [[r(u0), r(v0)], [r(u1), r(v0)], [r(u1), r(v1)], [r(u0), r(v1)]]


def r(value, digits=5):
    """🎯 Rounds a coordinate to 0.01 mm so the document stays readable."""
    return round(value + 0.0, digits)


def area(poly):
    """📐 Shoelace area of a simple polygon (absolute value)."""
    s = 0.0
    for (x0, y0), (x1, y1) in zip(poly, poly[1:] + poly[:1]):
        s += x0 * y1 - x1 * y0
    return abs(s) / 2


def centroid(poly):
    """⚖️ Area centroid of a simple polygon."""
    s = cx = cy = 0.0
    for (x0, y0), (x1, y1) in zip(poly, poly[1:] + poly[:1]):
        c = x0 * y1 - x1 * y0
        s += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    s /= 2
    return cx / (6 * s), cy / (6 * s)


def tray_profile():
    """🥿 The tray board under the cheeks: the cheek's lower edge offset 10 mm to the back-bottom side and run onto the inner face of the flap."""
    (u0, v0), (u1, v1) = CHEEK[0], CHEEK[1]
    du, dv = u1 - u0, v1 - v0
    n = math.hypot(du, dv)
    du, dv = du / n, dv / n
    nu, nv = -dv, du
    ou, ov = u0 + nu * TRAY_T, v0 + nv * TRAY_T
    k = (CHEEK[0][1] - ov) / dv
    foot = [ou + du * k, CHEEK[0][1]]
    end = [u1 + nu * TRAY_T, v1 + nv * TRAY_T]
    return [[r(u0), r(v0)], [r(u1), r(v1)], [r(end[0]), r(end[1])], [r(foot[0]), r(foot[1])]]


def blanket_profile(steps=8):
    """🧺 Side section of the folded blanket: a 15 mm slab with the two folds rounded to half circles."""
    rad = BLANKET_T / 2
    pts = []
    for i in range(steps + 1):
        a = -math.pi / 2 + math.pi * i / steps
        pts.append([r(BLANKET_L - rad + rad * math.cos(a)), r(rad + rad * math.sin(a))])
    for i in range(steps + 1):
        a = math.pi / 2 + math.pi * i / steps
        pts.append([r(rad + rad * math.cos(a)), r(rad + rad * math.sin(a))])
    return pts


def flap_world(i, u, v):
    """🔁 A point of flap i in its bone frame (u up, v backwards) at rest, in carcass coordinates (x ahead, y up from the cabinet floor)."""
    return D - v, FLAP_BOTTOMS[i] + u


def rotate_open(i, u, v, deg):
    """🚪 Where a flap point lies in carcass coordinates when the flap is tipped out by deg about its front-bottom edge."""
    a = math.radians(deg)
    x_rel, y_rel = -v, u
    return D + x_rel * math.cos(a) + y_rel * math.sin(a), FLAP_BOTTOMS[i] - x_rel * math.sin(a) + y_rel * math.cos(a)
