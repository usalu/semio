"""🪜️ Hand-chosen numbers and the planar kinematics of the used folding ladder shelf: world rest frames (the shelf open at its home wall), bone-local conversion and pose frames.

Everything here is one source of truth for `author-build.py`; world x ahead (away from the wall), y up, z left; the wall is the plane x = 0 at rest.

@see ../../brief.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md"""
import math

G = 9.80665
RAIL_HALF = 0.022
RAIL_W = (0.272, 0.300)
REAR_X = (0.0, 0.034)
REAR_TOP = 1.80
PIVOT_Y = 1.76
FOOT_X = 0.330
PIVOT_X = REAR_X[1] + RAIL_HALF + 0.0003
TAN = (FOOT_X - PIVOT_X) / PIVOT_Y
THETA = math.degrees(math.atan(TAN))
COS = math.cos(math.atan(TAN))
LADDER_LEN = math.hypot(FOOT_X - PIVOT_X, PIVOT_Y)
SHELF_TOPS = [0.26, 0.62, 0.92, 1.22, 1.52]
SHELF_T = 0.018
SHELF_FRONT = 0.012
SHELF_REAR_X = 0.005
SHELF_Z = 0.268
PIN_R = 0.004
CLEAT_H = 0.020
CLEAT_Z = (0.240, 0.272)
BLANKET_X = 0.55
BLANKET = (0.50, 0.020, 0.45)
ANCHOR_REST = (0.0, 1.70)
ANCHOR = (0.060, 0.022, (0.244, 0.272))
DENSITY = {"pine-aged": 500.0, "pine-board": 480.0, "beech": 720.0, "steel-zinc": 7850.0, "felt-blanket": 110.0, "pine-raw": 500.0}

STORED_LIFT = BLANKET[1]
STORED_SLIDE = 0.39
ANCHOR_STORED = (0.90, BLANKET[1] + ANCHOR[1])
CARRY_ANGLE = 70.0
CARRY_FOOT = (0.45, 0.30)
HOME_SLIDE = 0.0


def xc(y):
    """📐️ The x of the front rail centre line at height y (rest)."""
    return FOOT_X - y * TAN


def shelf_pin_y(i):
    """📌️ Height of the hinge pin of shelf i (0-based) at rest."""
    return SHELF_TOPS[i] - SHELF_T / 2


def shelf_pin_u(i):
    """📏️ Position of the hinge pin of shelf i along the ladder bone."""
    return shelf_pin_y(i) / COS


def shelf_end_u(i):
    """🔚️ Bone-local u of the rear edge of shelf i (the rear edge stands 5 mm in front of the wall)."""
    return xc(shelf_pin_y(i)) - SHELF_REAR_X


def shelf_depth(i):
    """↔️ Real depth of shelf i from the front edge to the rear edge."""
    return shelf_end_u(i) + SHELF_FRONT


BONES = [
    {"id": "ground", "parent": None, "x": 0.0, "y": 0.0, "angle": 0.0, "length": 0.0, "joint": {"type": "fixed"}},
    {"id": "blanket", "parent": "ground", "x": BLANKET_X, "y": 0.0, "angle": 0.0, "length": 0.0, "joint": {"type": "fixed"}},
    {"id": "lift", "parent": "ground", "x": 0.0, "y": 0.0, "angle": 0.0, "length": 0.0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.05, "maxM": 1.2}},
    {"id": "slide", "parent": "lift", "x": 0.0, "y": 0.0, "angle": 0.0, "length": 0.0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -1.2, "maxM": 1.2}},
    {"id": "ladder", "parent": "slide", "x": FOOT_X, "y": 0.0, "angle": 90.0 + THETA, "length": LADDER_LEN, "joint": {"type": "revolute", "minDeg": -60.0, "maxDeg": 15.0}},
    {"id": "rear", "parent": "ladder", "x": LADDER_LEN, "y": 0.0, "angle": 180.0 - THETA, "length": PIVOT_Y, "joint": {"type": "revolute", "minDeg": 0.0, "maxDeg": round(THETA, 4)}},
] + [
    {"id": f"shelf-{i + 1}", "parent": "ladder", "x": shelf_pin_u(i), "y": 0.0, "angle": 90.0 - THETA, "length": shelf_end_u(i), "joint": {"type": "revolute", "minDeg": round(-(90.0 - THETA), 4), "maxDeg": 0.0}}
    for i in range(5)
] + [
    {"id": "anchor-lift", "parent": "ground", "x": 0.0, "y": 0.0, "angle": 0.0, "length": 0.0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -1.8, "maxM": 0.2}},
    {"id": "anchor-slide", "parent": "anchor-lift", "x": 0.0, "y": 0.0, "angle": 0.0, "length": 0.0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.2, "maxM": 1.2}},
    {"id": "wall-anchor", "parent": "anchor-slide", "x": ANCHOR_REST[0], "y": ANCHOR_REST[1], "angle": 90.0, "length": ANCHOR[0], "joint": {"type": "revolute", "minDeg": -95.0, "maxDeg": 5.0}},
]
BY_ID = {b["id"]: b for b in BONES}


def frames(offsets=None):
    """🦴️ World frames {bone: (ox, oy, angleDeg)} for bone offsets (angleDeg, dx, dy), parents first, with prismatic offsets projected on their axis."""
    offsets = offsets or {}
    out = {}
    for bone in BONES:
        off = offsets.get(bone["id"], {})
        joint = bone["joint"]
        dx = dy = da = 0.0
        if joint["type"] == "prismatic":
            ax = math.radians(joint["axisDeg"])
            s = off.get("dx", 0.0) * math.cos(ax) + off.get("dy", 0.0) * math.sin(ax)
            s = max(joint["minM"], min(joint["maxM"], s))
            dx, dy = s * math.cos(ax), s * math.sin(ax)
        elif joint["type"] == "revolute":
            da = max(joint["minDeg"], min(joint["maxDeg"], off.get("angleDeg", 0.0)))
        if bone["parent"] is None:
            out[bone["id"]] = (bone["x"], bone["y"], bone["angle"] + da)
            continue
        px, py, pa = out[bone["parent"]]
        c, s_ = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        lx, ly = bone["x"] + dx, bone["y"] + dy
        out[bone["id"]] = (px + c * lx - s_ * ly, py + s_ * lx + c * ly, pa + bone["angle"] + da)
    return out


def to_world(frame, u, v):
    """🌍️ A bone-local point in world coordinates."""
    ox, oy, a = frame
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return (ox + c * u - s * v, oy + s * u + c * v)


def to_local(frame, x, y):
    """📍️ A world point in bone-local coordinates."""
    ox, oy, a = frame
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    dx, dy = x - ox, y - oy
    return (c * dx + s * dy, -s * dx + c * dy)


REST = frames()


def local_poly(bone, world_points):
    """🔄️ A world rest polygon as bone-local points, rounded to 0.1 mm."""
    return [[round(p, 5) for p in to_local(REST[bone], x, y)] for x, y in world_points]


def arc(cx, cy, r, a0, a1, n):
    """⌒️ Points of a circular arc from a0 to a1 degrees (both ends included)."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def poly_area_centroid(points):
    """⚖️ Signed-area-free area and centroid of a simple polygon."""
    a = cx = cy = 0.0
    n = len(points)
    for k in range(n):
        x0, y0 = points[k]
        x1, y1 = points[(k + 1) % n]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a *= 0.5
    return abs(a), (cx / (6 * a), cy / (6 * a))


def perimeter(points):
    """📏️ Perimeter of a closed polygon."""
    return sum(math.dist(points[k], points[(k + 1) % len(points)]) for k in range(len(points)))


STORED = {"lift": {"dy": STORED_LIFT}, "slide": {"dx": STORED_SLIDE}, "rear": {"angleDeg": round(THETA, 4)}, **{f"shelf-{i + 1}": {"angleDeg": round(-(90.0 - THETA), 4)} for i in range(5)}, "anchor-lift": {"dy": round(ANCHOR_STORED[1] - ANCHOR_REST[1], 4)}, "anchor-slide": {"dx": ANCHOR_STORED[0]}, "wall-anchor": {"angleDeg": -90.0}}


def carried_offsets():
    """🧳️ Offsets of the carried pose: the folded package lifted with its feet 0.30 m up and tilted to 70 degrees, the storage aids where they lay."""
    out = {k: dict(v) for k, v in STORED.items()}
    out["lift"] = {"dy": CARRY_FOOT[1]}
    out["slide"] = {"dx": round(CARRY_FOOT[0] - FOOT_X, 4)}
    out["ladder"] = {"angleDeg": round(CARRY_ANGLE - (90.0 + THETA), 4)}
    return out


def folded_home_offsets():
    """🏠️ Offsets of the folded package leaning on the home wall before it is opened, the wall anchor already screwed on."""
    out = {"rear": {"angleDeg": round(THETA, 4)}, **{f"shelf-{i + 1}": {"angleDeg": round(-(90.0 - THETA), 4)} for i in range(5)}}
    f = frames(out)
    x, _ = to_world(f["rear"], -0.04, -(PIVOT_X - REAR_X[0]))
    out["slide"] = {"dx": round(-x, 4)}
    return out
