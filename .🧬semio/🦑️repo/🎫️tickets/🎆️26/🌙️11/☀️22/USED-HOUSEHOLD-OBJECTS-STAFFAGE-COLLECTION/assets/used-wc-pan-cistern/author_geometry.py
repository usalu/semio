"""🚽️ Hand-chosen numbers and geometry helpers of the used close-coupled WC on its half pallet: every dimension of pan, cistern, seat, lid, cap, cardboard and pallet in world rest metres, and the volume, centre and outline functions the chisel uses for solids and drawings.

World frame of the staffage (x ahead = the front of the bowl, y up, z left), metres; every bone rests at angle 0, so a bone-local point is the world point minus the bone origin.

@see ./author-used-wc-pan-cistern.py — the chisel that writes the asset document
@see ../../brief.md — the brief of the wave
"""
import math


def T(en, de):
    """🗣️ One bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🎯️ Rounds a coordinate to the micrometre."""
    return round(v + 0.0, 6)


G = 9.80665
PALLET_H = 0.144
CARD_T = 0.005
PH = PALLET_H + CARD_T

PAL_X = 0.40
PAL_Z = 0.30
PAL_BOARD_W = 0.10
PAL_TOP_Z = (-0.25, -0.125, 0.0, 0.125, 0.25)
PAL_ROW_X = (-0.35, 0.0, 0.35)
PAL_ROW_Z = (-0.25, 0.0, 0.25)
Y_BOT1, Y_BLK1, Y_CROSS1 = 0.022, 0.100, 0.122
CARD = dict(x0=-0.27, x1=0.25, z=0.20)

XB = 0.09
RIM_R = 0.245
RIM_HALF_W = 0.18
LS = RIM_HALF_W / RIM_R
BOWL_OUTER = [(0.200, 0.048), (0.212, 0.090), (0.240, 0.135), (0.280, 0.180), (0.320, 0.215), (0.355, 0.238), (0.365, 0.245), (0.396, 0.245), (0.400, 0.241)]
BOWL_INNER = [(0.400, 0.226), (0.388, 0.225), (0.372, 0.218), (0.345, 0.210), (0.305, 0.185), (0.265, 0.150), (0.235, 0.110), (0.218, 0.068), (0.210, 0.032)]
BOWL_LOOP = BOWL_OUTER + BOWL_INNER + [BOWL_OUTER[0]]

FOOT = dict(cx=-0.01, rx=0.22, rz=0.105, n=2.6, t=0.008, y1=0.235, top=0.88)
FOOT_PLATE = (0.227, 0.235)
REAR = dict(x0=-0.31, x1=-0.125, z=0.115, r=0.03, t=0.008, y0=0.235, yw=0.243, y1=0.388)
DECK = dict(x0=-0.325, x1=-0.135, z=0.155, r=0.05, y0=0.388, y1=0.40)
NECK_Z = 0.10
NECK_TOP = 0.316
SPIGOT = dict(x0=-0.32, x1=-0.20, y=0.18, ro=0.051, ri=0.042)
CAP = dict(x0=-0.3245, x1=-0.2845, ro=0.058, ri=0.051, end=0.0045)
TRAP = [((XB, 0.205), (0.040, 0.100), 0.032), ((0.040, 0.100), (-0.060, 0.195), 0.032), ((-0.060, 0.195), (-0.205, 0.180), 0.042)]
TRAP_WALL = 0.009

CIS = dict(x0=-0.335, x1=-0.175, z=0.185, r=0.02, t=0.009, y0=0.40, yb=0.41, y1=0.772)
CLID = dict(x0=-0.34, x1=-0.17, z=0.19, r=0.022, y0=0.772, y1=0.79)
BUTTON = dict(x=-0.255, r=0.028, hole=0.029, y0=0.776, y1=0.795)
FLUSH_VALVE = dict(x=-0.255, z=0.0, r=0.032, y1=0.70)
FILL_VALVE = dict(x=-0.255, z=0.13, r=0.02, y1=0.72)
BOLTS_Z = 0.075

HINGE = (-0.145, 0.431)
SEAT = dict(cx=XB, rx=0.243, rz=0.178, xcut=-0.138, hx=0.105, hrx=0.15, hrz=0.118, y0=0.408, yb=0.421, y1=0.431, skirt=0.005)
LID = dict(y0=0.431, ys=0.446, y1=0.451, skirt=0.004)
BUFFERS = [(0.28, 0.10), (0.28, -0.10), (-0.10, 0.10), (-0.10, -0.10)]
BUFFER = dict(dx=0.018, dz=0.012, y0=0.400, y1=0.408)
LUG = dict(x0=-0.16, x1=-0.138, y0=0.418, y1=0.444, w=0.025)
SEAT_LUG_Z = 0.075
LID_LUG_Z = 0.05
BRACKET = dict(x0=-0.165, x1=-0.125, z=0.1025, w=0.015, y0=0.40, y1=0.446)
PIN = dict(z=0.105, r=0.006)
LID_OPEN_DEG = 90.5

ORIGIN = {"ground": (0.0, 0.0), "pallet": (-PAL_X, 0.0), "travel": (0.0, 0.0), "lift": (0.0, 0.0), "pan": (-0.23, PH), "cistern": (CIS["x0"], PH + CIS["y0"]), "cistern-lid": (CLID["x0"], PH + CLID["y0"]), "button": (BUTTON["x"], PH + CLID["y1"]), "seat": (HINGE[0], PH + HINGE[1]), "lid": (HINGE[0], PH + HINGE[1])}


def loc(bone, x, y):
    """📍️ World rest point (y measured from the floor) to the bone-local (u, v) of `bone`."""
    ox, oy = ORIGIN[bone]
    return (r6(x - ox), r6(y - oy))


def arc(cx, cy, r, a0, a1, n):
    """⭕️ Points of a circular arc from angle a0 to a1 (degrees), both ends included."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def superellipse(cx, cz, rx, rz, n, count=48):
    """🥚️ Plan points (x, z) of a superellipse |dx/rx|^n + |dz/rz|^n = 1, counter-clockwise from ahead."""
    pts = []
    for k in range(count):
        t = 2 * math.pi * k / count
        c, s = math.cos(t), math.sin(t)
        pts.append((cx + rx * math.copysign(abs(c) ** (2 / n), c), cz + rz * math.copysign(abs(s) ** (2 / n), s)))
    return pts


def ellipse_pts(cx, cz, rx, rz, count=48):
    """⚪️ Plan points (x, z) of an ellipse."""
    return [(cx + rx * math.cos(2 * math.pi * k / count), cz + rz * math.sin(2 * math.pi * k / count)) for k in range(count)]


def rounded_rect(x0, x1, z0, z1, r, n=4):
    """▢️ Plan points (x, z) of a rectangle with rounded corners."""
    pts = []
    for cx, cz, a0 in ((x1 - r, z1 - r, 0), (x0 + r, z1 - r, 90), (x0 + r, z0 + r, 180), (x1 - r, z0 + r, 270)):
        pts += arc(cx, cz, r, a0, a0 + 90, n)
    return pts


def clipped_ellipse(cx, rx, rz, xcut, count=56):
    """✂️ Plan points (x, z) of an ellipse cut straight at x = xcut behind (the seat and lid outline)."""
    a = math.acos((xcut - cx) / rx)
    pts = [(cx + rx * math.cos(-a + 2 * a * k / count), rz * math.sin(-a + 2 * a * k / count)) for k in range(count + 1)]
    return pts


def scale_about(pts, s, c):
    """🔍️ Points scaled by s about the centre c."""
    return [(c[0] + (x - c[0]) * s, c[1] + (z - c[1]) * s) for x, z in pts]


def poly_area(pts):
    """🧮️ Shoelace area of a ring."""
    return abs(sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1]))) / 2


def poly_centroid(pts):
    """🎯️ Area centroid of a ring."""
    a2 = sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1]))
    cx = sum((x0 + x1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1])) / (3 * a2)
    cy = sum((y0 + y1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1])) / (3 * a2)
    return cx, cy


def loop_pappus(loop):
    """🏺️ Area, radial moment ∫r dA and axial moment ∫a r dA of a closed [axial, radial] loop (Green's theorem)."""
    pts = loop[:-1] if loop[0] == loop[-1] else loop
    A = Mr = Mar = 0.0
    for (a0, r0), (a1, r1) in zip(pts, pts[1:] + pts[:1]):
        cr = a0 * r1 - a1 * r0
        A += cr / 2
        Mr += (r0 + r1) * cr / 6
        Mar += (a0 * r0 * 2 + a0 * r1 + a1 * r0 + a1 * r1 * 2) * cr / 24
    sgn = 1 if A > 0 else -1
    return abs(A), sgn * Mr, sgn * Mar


def revolve_loop_volume(loop, ls=1.0):
    """🍶️ Volume and axial centroid of a closed loop revolved a full turn and stretched laterally by ls."""
    A, Mr, Mar = loop_pappus(loop)
    return 2 * math.pi * Mr * ls, Mar / Mr


def revolve_open_volume(profile, ls=1.0):
    """🥛️ Volume and axial centroid of an open [axial, radial] profile closed through the axis (frusta)."""
    vol = num = 0.0
    for (a0, r0), (a1, r1) in zip(profile, profile[1:]):
        v = math.pi * (a1 - a0) / 3 * (r0 * r0 + r0 * r1 + r1 * r1)
        vol += v
        num += v * (a0 + a1) / 2
    return abs(vol) * ls, num / vol if vol else 0.0


def r_envelope(loop, a):
    """📈️ Largest radius of a closed profile at the axial position a (0 where it does not reach)."""
    best = None
    pts = loop[:-1] if loop[0] == loop[-1] else loop
    for (a0, r0), (a1, r1) in zip(pts, pts[1:] + pts[:1]):
        lo, hi = min(a0, a1), max(a0, a1)
        if lo - 1e-12 <= a <= hi + 1e-12:
            r = r0 if abs(a1 - a0) < 1e-12 else r0 + (r1 - r0) * (a - a0) / (a1 - a0)
            if abs(a1 - a0) < 1e-12:
                r = max(r0, r1)
            best = r if best is None else max(best, r)
    return best


def envelope_outline(loop, cx, ls=1.0):
    """🏺️ Silhouette (x or z, y) of a profile revolved about a vertical axis at cx: the largest radius at every profile height, mirrored."""
    pts = loop[:-1] if loop[0] == loop[-1] else loop
    ys = sorted({a for a, _ in pts})
    right = [(cx + ls * r_envelope(loop, a), a) for a in ys]
    left = [(cx - ls * r_envelope(loop, a), a) for a in reversed(ys)]
    return right + left
