"""💧 Hand-chosen geometry of `used-water-play-table` (world rest metres: x ahead along the channel towards the screw, y up, z left; origin on the channel axis at sand level) and the frame helpers that turn it into bone-local 2D shapes and 3D solids.

@see ../../brief.md — the brief of the wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Front and plan views", "Volume"
"""
import math

FOOT_R = 0.15
FOOT_Y0 = -0.60
POST_R = 0.06
POST_Y0 = -0.50
POST_Y1 = 0.40
POST_X = 0.66
POST_Z = 0.17
BEARER = (0.40, 0.50)
BEARER_HALF_X = 0.06
BEARER_Z = 0.25
DECK = (-0.78, 0.78, 0.50, 0.54, -0.25, 0.25)
APRON_Y = (0.44, 0.68)
APRON_X = 0.82
APRON_Z = (0.25, 0.29)
END_X = (0.78, 0.82)
WEAR = 0.008
TR_X = 0.78
TR_Z = 0.25
TR_T = 0.0015
TR_Y0 = 0.54
TR_RIM = 0.68
TR_FL = 0.28
END_FL_X = 0.81
TR_IN_X = TR_X - TR_T
TR_IN_Z = TR_Z - TR_T
TR_FLOOR = TR_Y0 + TR_T
TR_TOP = TR_RIM + TR_T
GATE_X = {"gate-1": 0.26, "gate-2": -0.26}
GATE_T = 0.003
GATE_Z = 0.2435
GATE_H = 0.175
GATE_HOLE = (0.142, 0.165, 0.05)
GATE_OPEN = 0.10
GUIDE = (0.005, 0.2385)
WATER_D = 0.035
DRAIN = 0.037
POOLS = {"pool-1": (GATE_X["gate-1"] + GATE_T / 2, TR_IN_X), "pool-2": (GATE_X["gate-2"] + GATE_T / 2, GATE_X["gate-1"] - GATE_T / 2), "pool-3": (-TR_IN_X, GATE_X["gate-2"] - GATE_T / 2)}
SPOUT = (-0.90, -TR_IN_X, 0.60, 0.0212, 0.0192)
SAND_DRIFT = (-TR_IN_X, -0.62, 0.028)
MUD = (-0.97, 0.08, 0.22, 0.025)
PLINTH = (1.23, 1.75, -0.10, 0.15, 0.26)
BS_X = (1.28, 1.70)
BS_T = 0.0015
BS_Y0 = 0.15
BS_RIM = 0.34
BS_FL = 0.035
BS_Z = 0.21
BS_CX = 1.49
BS_FLOOR = BS_Y0 + BS_T
BS_WATER = 0.11
P = (0.92, 0.84)
AXIS_DEG = -45.0
TUBE = (-0.02, 0.86, 0.075, 0.08)
CLAMP = (-0.02, 0.02, 0.084)
CAP = (-0.028, -0.02, 0.0215, 0.0845)
SHAFT = (-0.10, 0.90, 0.0212)
HELIX = (0.02, 0.84, 0.0212, 0.073, 0.16)
CRANK = (-0.09, -0.07, -0.025, 0.15, 0.012)
GRIP_UV = (-0.09, 0.135)
GRIP = (0.0, 0.11, 0.016, 0.125, 0.02)
FOOT = (0.06, 0.06, 0.025)
PIN = (0.008, 0.0835, 0.100)
PLATE_Z = (0.088, 0.096)
PLATE = [(0.82, 0.56), (0.86, 0.56), (0.955, 0.83), (0.945, 0.87), (0.905, 0.87), (0.82, 0.66)]
CHUTE_R = 0.05
CHUTE_T = 0.0015
CHUTE_X = (0.70, 0.875)
CHUTE_SLOPE = 0.2
SWING = 40.0

BONES = {
    "frame": {"origin": (0.0, 0.0), "theta": 0.0, "z": 0.0},
    "post-rl": {"origin": (-POST_X, FOOT_Y0), "theta": 90.0, "z": POST_Z},
    "post-rr": {"origin": (-POST_X, FOOT_Y0), "theta": 90.0, "z": -POST_Z},
    "post-fl": {"origin": (POST_X, FOOT_Y0), "theta": 90.0, "z": POST_Z},
    "post-fr": {"origin": (POST_X, FOOT_Y0), "theta": 90.0, "z": -POST_Z},
    "gate-1": {"origin": (GATE_X["gate-1"], TR_FLOOR), "theta": 90.0, "z": 0.0},
    "gate-2": {"origin": (GATE_X["gate-2"], TR_FLOOR), "theta": 90.0, "z": 0.0},
    "pool-1": {"origin": (sum(POOLS["pool-1"]) / 2, TR_FLOOR), "theta": 0.0, "z": 0.0},
    "pool-2": {"origin": (sum(POOLS["pool-2"]) / 2, TR_FLOOR), "theta": 0.0, "z": 0.0},
    "pool-3": {"origin": (sum(POOLS["pool-3"]) / 2, TR_FLOOR), "theta": 0.0, "z": 0.0},
    "basin": {"origin": (BS_CX, 0.0), "theta": 0.0, "z": 0.0},
    "basin-water": {"origin": (BS_CX, BS_FLOOR), "theta": 0.0, "z": 0.0},
    "screw-mount": {"origin": P, "theta": AXIS_DEG, "z": 0.0},
    "screw": {"origin": P, "theta": AXIS_DEG, "z": 0.0},
}
POSTS = ("post-rl", "post-rr", "post-fl", "post-fr")
POOL_IDS = ("pool-1", "pool-2", "pool-3")


def r5(value):
    """🔢 Rounds a coordinate to 0.01 mm and removes negative zero."""
    out = round(value, 5)
    return 0.0 if out == 0 else out


def local(bone, x, y):
    """📐 World rest point (x, y) to the bone-local (u, v) of `bone`."""
    ox, oy = BONES[bone]["origin"]
    t = math.radians(BONES[bone]["theta"])
    dx, dy = x - ox, y - oy
    return (r5(dx * math.cos(t) + dy * math.sin(t)), r5(-dx * math.sin(t) + dy * math.cos(t)))


def world(bone, u, v):
    """🌍 Bone-local rest point (u, v) of `bone` to world (x, y)."""
    ox, oy = BONES[bone]["origin"]
    t = math.radians(BONES[bone]["theta"])
    return (ox + u * math.cos(t) - v * math.sin(t), oy + u * math.sin(t) + v * math.cos(t))


def lpoly(bone, points):
    """🔷 World polygon to bone-local points."""
    return [local(bone, x, y) for x, y in points]


def arc(cx, cy, r, a0, a1, n):
    """🌙 Points on a circle from angle a0 to a1 (degrees, inclusive)."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def quat_w(deg):
    """🔄 Unit quaternion [x, y, z, w] of a turn by `deg` about the bone's w axis."""
    h = math.radians(deg) / 2
    return [0.0, 0.0, r5(math.sin(h)), r5(math.cos(h))]


def quat_axis(axis, deg):
    """🧭 Unit quaternion [x, y, z, w] of a turn by `deg` about the unit vector `axis` of the bone frame."""
    h = math.radians(deg) / 2
    n = math.sqrt(sum(a * a for a in axis))
    return [r5(a / n * math.sin(h)) for a in axis] + [r5(math.cos(h))]


BONES["grip"] = {"origin": world("screw", *GRIP_UV), "theta": AXIS_DEG + 180.0, "z": 0.0}


def screw_point(u, v):
    """🌀 World (x, y) of a point (u, v) of the screw frame at rest."""
    return world("screw", u, v)


def chute_line():
    """🛝 Rim line of the outlet chute: upper end A under the tube outlet, lower end B over pool 1, the gutter bottom resting on the inner edge of the end flange of the trough."""
    xa, xb = CHUTE_X[1], CHUTE_X[0]
    c = 1 / math.hypot(1, CHUTE_SLOPE)
    drop = CHUTE_R * c
    shift = CHUTE_R * CHUTE_SLOPE * c
    y_at_edge = TR_TOP - 0.0005 + drop
    ya = y_at_edge + CHUTE_SLOPE * (xa - (TR_X - shift))
    yb = ya - CHUTE_SLOPE * (xa - xb)
    return (xa, ya), (xb, yb)


def chute_outline_world():
    """🛝 Side outline of the chute (world x, y): the rim line and the gutter bottom one radius below it, normal to the slope."""
    (xa, ya), (xb, yb) = chute_line()
    c = 1 / math.hypot(1, CHUTE_SLOPE)
    nx, ny = CHUTE_SLOPE * c * CHUTE_R, -c * CHUTE_R
    return [(xa, ya), (xb, yb), (xb + nx, yb + ny), (xa + nx, ya + ny)]


def grip_top():
    """⬆️ Highest point of the crank grip at rest (world y)."""
    gx, gy = screw_point(*GRIP_UV)
    ux, uy = -math.cos(math.radians(AXIS_DEG)), -math.sin(math.radians(AXIS_DEG))
    tip = (gx + ux * GRIP[3], gy + uy * GRIP[3])
    return tip[1] + GRIP[4] * abs(ux)


def foot_centre():
    """🦶 World (x, y0, y1) of the foot bearing block under the lower shaft end, standing on the basin floor and holding the shaft end 25 mm deep."""
    x, y = screw_point(SHAFT[1], 0.0)
    return x, BS_FLOOR, y + FOOT[2]
