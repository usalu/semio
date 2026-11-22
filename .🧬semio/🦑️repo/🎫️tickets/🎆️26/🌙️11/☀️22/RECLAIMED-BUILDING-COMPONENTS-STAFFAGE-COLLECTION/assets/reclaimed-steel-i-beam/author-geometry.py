"""🔩️ Hand-chosen numbers and exact profile geometry of the reclaimed INP 240 beam (DIN 1025-1), shared by the 2D drawings and the 3D solids.

The section is built from the DIN 1025-1 nominal values (h, b, s, t, r1, r2 and the 14 % flange slope); every other number is stated here once.
@see https://www.beuth.de/de/norm/din-1025-1/2366958 — DIN 1025-1 narrow flange I-sections
"""
import math

H = 0.240
B = 0.106
S = 0.0087
T = 0.0131
R1 = 0.0087
R2 = 0.0052
SLOPE = 0.14
L = 6.000
RHO_STEEL = 7850.0
C_STEEL = 450.0
RHO_TIMBER = 420.0
C_TIMBER = 1600.0

HALF = H / 2
WEB = S / 2
FLANGE = B / 2
W_MID = WEB + (B - S) / 4
Y_MID = HALF - T

CUT = 0.010
CLEAT_X = 5.930
CLEAT_LEG = 0.070
CLEAT_T = 0.008
CLEAT_OUT = 0.080
CLEAT_LEN = 0.160
CLEAT_ROOT = 0.010
CLEAT_END = CLEAT_LEG + CLEAT_T
CLEAT_HOLE_R = 0.0105
CLEAT_HOLE_W = WEB + 0.045
RIVET_X = 0.035
RIVET_V = 0.035
RIVET_HEAD_R = 0.014
RIVET_HEAD_K = 0.011
RIVET_SHANK_R = 0.008
WEB_HOLE_R = 0.011
WEB_HOLES = [(2.40, -0.035), (2.40, 0.035), (3.60, -0.035), (3.60, 0.035)]

BEARER = 0.100
BEARER_LEN = 0.600
BEARER_X = [1.20, 4.70]
BEAM_Y = BEARER + HALF
SLING_X = [1.25, 4.75]
LIFT = 0.60
INSTALL = 0.40
BEARING_WALL_X = 0.10
BEARING_LEN = 0.20
LOAD_X0 = 0.30


def inner(w):
    """📐️ Height of the sloped inner flange face of the upper flange at lateral position w."""
    return Y_MID + SLOPE * (w - W_MID)


NORM = math.sqrt(1 + SLOPE * SLOPE)


def foot(cw, cv):
    """🦶️ The foot of the perpendicular from (cw, cv) on the inner flange line."""
    c0 = Y_MID - SLOPE * W_MID
    t = (cw + SLOPE * (cv - c0)) / NORM ** 2
    return (t, SLOPE * t + c0)


def arc(c, r, a0, a1, n):
    """🌙️ n + 1 points of the circle about c with radius r from angle a0 to a1 (radians)."""
    return [(c[0] + r * math.cos(a0 + (a1 - a0) * k / n), c[1] + r * math.sin(a0 + (a1 - a0) * k / n)) for k in range(n + 1)]


ROOT_C = (WEB + R1, inner(WEB + R1) - R1 * NORM)
TOE_C = (FLANGE - R2, inner(FLANGE - R2) + R2 * NORM)
Y_T = ROOT_C[1]
ROOT_FOOT = foot(*ROOT_C)
TOE_FOOT = foot(*TOE_C)


def quadrant():
    """🍕️ The upper right quarter of the section outline from the web tangent of the root fillet to the top flange corner, (w, v)."""
    a_root_end = math.atan2(ROOT_FOOT[1] - ROOT_C[1], ROOT_FOOT[0] - ROOT_C[0])
    root = arc(ROOT_C, R1, math.pi, a_root_end, 6)
    a_toe_start = math.atan2(TOE_FOOT[1] - TOE_C[1], TOE_FOOT[0] - TOE_C[0])
    toe = arc(TOE_C, R2, a_toe_start, 0.0, 4)
    return root + toe + [(FLANGE, HALF)]


def flange_ring():
    """🔝️ Closed outline of the upper flange with both fillets down to the web tangent v = Y_T, (w, v)."""
    q = quadrant()
    left = [(-w, v) for (w, v) in reversed(q)]
    return q + left


def section_ring():
    """🅸️ Closed outline of the whole INP 240 section, (w, v), counter-clockwise."""
    q = quadrant()
    right_down = list(reversed(q)) + [(w, -v) for (w, v) in q]
    left_up = [(-w, v) for (w, v) in reversed(right_down)]
    return right_down + left_up


def shoelace(ring):
    """🧮️ Signed area of a closed ring."""
    return 0.5 * sum(ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1] for i in range(len(ring)))


def perimeter(ring):
    """📏️ Length of a closed ring."""
    return sum(math.dist(ring[i], ring[(i + 1) % len(ring)]) for i in range(len(ring)))


def second_moment(ring):
    """🌀️ Second moment of area about the v = 0 axis of a closed ring (w, v)."""
    s = 0.0
    for i in range(len(ring)):
        x0, y0 = ring[i]
        x1, y1 = ring[(i + 1) % len(ring)]
        a = x0 * y1 - x1 * y0
        s += a * (y0 * y0 + y0 * y1 + y1 * y1)
    return abs(s) / 12.0


def circle(cx, cy, r, n=16):
    """⭕️ A closed polygon of n points approximating a circle."""
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def rnd(points, nd=6):
    """🎯️ Points rounded for the document."""
    return [[round(a, nd), round(b, nd)] for (a, b) in points]


def angle_ring(t):
    """📐️ Outline of one cleat angle of leg thickness t in the (w, u) plane of the cleat bone, heel on the web face, with the root fillet."""
    w0, u1 = WEB, CLEAT_END
    wi, ui = WEB + t, CLEAT_END - t
    pts = [(w0, 0.0), (wi, 0.0), (wi, ui - CLEAT_ROOT)]
    pts += arc((wi + CLEAT_ROOT, ui - CLEAT_ROOT), CLEAT_ROOT, math.pi, math.pi / 2, 4)[1:-1]
    pts += [(wi + CLEAT_ROOT, ui), (WEB + CLEAT_OUT, ui), (WEB + CLEAT_OUT, u1), (w0, u1)]
    return pts


if __name__ == "__main__":
    ring = section_ring()
    print("[DEBUG] section area cm2", abs(shoelace(ring)) * 1e4, "perimeter m", perimeter(ring), "Iy cm4", second_moment(ring) * 1e8, "Y_T", Y_T)
    print("[DEBUG] flange area cm2", abs(shoelace(flange_ring())) * 1e4, "angle area cm2", abs(shoelace(angle_ring(CLEAT_T))) * 1e4)
