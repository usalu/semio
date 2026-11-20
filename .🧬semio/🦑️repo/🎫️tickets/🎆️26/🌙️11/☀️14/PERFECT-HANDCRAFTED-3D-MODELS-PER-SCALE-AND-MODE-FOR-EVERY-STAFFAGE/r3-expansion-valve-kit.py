"""🧭 Round 3, machinery/expansion-valve: the measures of the real valve and the frames of its bones, shared by the volumes and the 2D drawings.

World frame (rest): x ahead (the outlet runs towards +x), y up, z lateral (left is +); the cut end of the liquid line lies on the valve axis at y = CUT, which is the origin of the asset (the rig root and the front drawings are lowered by CUT).
The valve is the Danfoss T 2 pattern (flare x solder): a forged brass angle body on a vertical axis, the liquid line entering from below through a
3/8 in flare nut, the 12 mm solder outlet 54 mm to the right of the axis, the superheat setting spindle under a hexagon cap pointing down to the left,
the thermostatic element 30 mm above the outlet axis, the 1.5 m capillary coiled above the element and led to the 14.5 x 50 mm bulb, which is strapped
on top of a piece of the suction line beside the valve. All lengths below are millimetres and are converted by the helpers.
"""
import math

MM = 0.001
OUTLET_Y = 93.0
CUT = 22.0
BULB_Z = 24.0
SUCTION_Y = 78.0
BULB_Y = SUCTION_Y + 8.0 + 7.25
BONES = {
    "body": (0.0, OUTLET_Y, 0.0, 0.0),
    "head": (0.0, 108.0, 0.0, 90.0),
    "stem": (-12.0, 94.0, 0.0, 210.0),
    "bulb": (-57.0, BULB_Y, BULB_Z, 180.0),
}
LENGTHS = {"body": 54.0, "head": 15.0, "stem": 22.0, "bulb": 50.0}
FEATURE = {50: 7.5, 100: 15.0}

LIQUID_R = 4.76
NUT_AF = 19.0
OUTLET_R = 6.0
STUB_R = 6.35
BELL_R = 7.35
SUCTION_R = 8.0
BULB_R = 7.25
CAP_R = 0.8
COIL_R = 29.0
COIL_TURNS = 7
COIL_PITCH = 2.0
COIL_TOP = 142.0
COIL_BOTTOM = COIL_TOP - COIL_TURNS * COIL_PITCH
STEM_AF = 17.0
OUTLET_END = 82.0
SUCTION_X = (-114.0, -50.0)

BARREL = [(70.0, 8.6, 2.0), (72.0, 9.4, 2.0), (78.0, 9.6, 2.2), (81.0, 11.0, 3.0), (84.0, 11.5, 4.0), (102.0, 11.5, 4.0), (105.0, 11.0, 3.0), (106.5, 9.5, 2.0), (108.2, 8.5, 2.0)]
HEAD = [(-1.5, 7.0), (2.2, 7.0), (2.8, 8.0), (6.0, 19.6), (6.6, 21.0), (9.6, 21.0), (10.2, 20.2), (10.5, 18.4), (11.3, 17.6), (12.7, 9.0), (13.2, 4.2), (13.6, 3.4), (15.0, 3.2)]
BULB = [(0.0, 1.3), (0.8, 3.6), (2.4, 5.8), (4.6, 7.0), (6.0, 7.25), (48.4, 7.25), (49.6, 6.6), (50.0, 5.6), (50.0, 1.7), (51.2, 1.7)]
BULB_TIP = 52.2


def m(v):
    """📏 Millimetres to metres, rounded to the micrometre."""
    return round(v * MM, 6)


def local(bone, p):
    """📍 A world rest point (mm) in the local frame (u, v, w) of a bone, in metres."""
    ox, oy, oz, deg = BONES[bone]
    a = math.radians(deg)
    dx, dy = p[0] - ox, p[1] - oy
    return [m(dx * math.cos(a) + dy * math.sin(a)), m(-dx * math.sin(a) + dy * math.cos(a)), m((p[2] if len(p) > 2 else 0.0) - oz)]


def local2(bone, p):
    """📐 A world rest point (mm) as an elevation point (u, v) of a bone, in metres."""
    return local(bone, p)[:2]


def direction(bone, d):
    """🧭 A world direction in the local frame of a bone."""
    deg = BONES[bone][3]
    a = math.radians(deg)
    return [round(d[0] * math.cos(a) + d[1] * math.sin(a), 6), round(-d[0] * math.sin(a) + d[1] * math.cos(a), 6), round(d[2], 6)]


def world(bone, u, v, w=0.0):
    """🌍 A bone-local point (mm) in the world rest frame (mm)."""
    ox, oy, oz, deg = BONES[bone]
    a = math.radians(deg)
    return (ox + u * math.cos(a) - v * math.sin(a), oy + u * math.sin(a) + v * math.cos(a), oz + w)


def hexagon(af, start_deg=0.0):
    """⬡ The six corners (mm) of a hexagon of the given width across flats, the first corner at start_deg."""
    r = af / 2 / math.cos(math.radians(30))
    return [(r * math.cos(math.radians(start_deg + 60 * k)), r * math.sin(math.radians(start_deg + 60 * k))) for k in range(6)]


def ring(r, n=24):
    """⭕ A circle of radius r (mm) as n points, counter-clockwise."""
    return [(r * math.cos(2 * math.pi * k / n), r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def strap_outline(grow, n=10):
    """➰ The outline (v, w in mm, bulb frame) of a band drawn tight around the bulb (centre 0, 0) and the suction line (centre 15.25, 0), grown by `grow`."""
    d = BULB_Y - SUCTION_Y
    r1, r2 = BULB_R + grow, SUCTION_R + grow
    t = math.acos((r2 - r1) / d)
    pts = []
    for k in range(n + 1):
        a = -t + 2 * t * k / n
        pts.append((d + r2 * math.cos(a), r2 * math.sin(a)))
    for k in range(n + 1):
        a = t + (2 * math.pi - 2 * t) * k / n
        pts.append((r1 * math.cos(a), r1 * math.sin(a)))
    return pts


def riser_path():
    """🌀 Centre line (world mm) of the capillary from the nipple on the element up through the coil and out along a flat spiral to the first turn."""
    pts = [(0.0, 122.6, 0.0), (0.0, 130.0, 0.0), (0.0, 135.0, 0.0), (0.0, COIL_TOP - 4.0, 0.0)]
    for k in range(1, 5):
        a = math.radians(22.5 * k)
        pts.append((0.0, COIL_TOP - 4.0 + 4.0 * math.sin(a), 4.0 * (1 - math.cos(a))))
    for k in range(1, 25):
        s = k / 24
        r = 4.0 + (COIL_R - 4.0) * (2 * s - s * s)
        phi = math.radians(90.0) + 2 * math.pi * s * s
        pts.append((r * math.cos(phi), COIL_TOP, r * math.sin(phi)))
    return pts


def coil_path(per_turn=24):
    """🧵 Centre line (world mm) of the coiled surplus capillary: seven turns of 58 mm at a pitch of 2 mm, from the top turn down."""
    pts = []
    for k in range(COIL_TURNS * per_turn + 1):
        phi = math.radians(90.0) + 2 * math.pi * k / per_turn
        pts.append((COIL_R * math.cos(phi), COIL_TOP - COIL_PITCH * k / per_turn, COIL_R * math.sin(phi)))
    return pts


def lead_path(n=20):
    """〰️ Centre line (world mm) of the capillary from the lowest turn of the coil to the bulb: a cubic curve leaving the coil along -x and entering the bulb along its axis."""
    b = [(0.0, COIL_BOTTOM, COIL_R), (-40.0, COIL_BOTTOM, COIL_R), (-25.0, BULB_Y, BULB_Z), (-57.0, BULB_Y, BULB_Z)]
    pts = []
    for k in range(n + 1):
        t = k / n
        c = ((1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t * t, t ** 3)
        pts.append(tuple(sum(c[i] * b[i][j] for i in range(4)) for j in range(3)))
    return pts


def path_length(pts):
    """📏 Length (mm) of a polyline."""
    return sum(math.dist(pts[i], pts[i + 1]) for i in range(len(pts) - 1))


def hull(points):
    """🪢 The convex hull of 2D points, counter-clockwise."""
    pts = sorted(set((round(x, 4), round(y, 4)) for x, y in points))

    def half(seq):
        out = []
        for p in seq:
            while len(out) >= 2 and (out[-1][0] - out[-2][0]) * (p[1] - out[-2][1]) - (out[-1][1] - out[-2][1]) * (p[0] - out[-2][0]) <= 1e-9:
                out.pop()
            out.append(p)
        return out[:-1]

    return half(pts) + half(pts[::-1])


def mirrored(profile):
    """🪞 The closed outline (u, v) of a turned profile [u, radial]: one flank out, the other back."""
    return [(u, r) for u, r in profile] + [(u, -r) for u, r in reversed(profile)]
