"""🪑 Hand-chosen geometry of the used movable park armchair (world metres: x ahead, y up, z left) and the pure-python helpers that turn it into bone-local solids and view outlines.

Imported by `author-armchair.py`, the one script that writes `🔣️.json`.
@see ../../brief.md
"""
import math

T_R = 0.0125
T_SEC = (0.025, 0.002)
RAIL_R = 0.010
STRETCH_R = 0.008
HEM_R = 0.008
SHEET_T = 0.002
CARRIER_H = 0.030
CARRIER_BACK_TOP_H = 0.012
CARRIER_W = 0.008
BEND_R = 0.06
FOOT_H = 0.065
FLANGE_R = 0.015
FLANGE_T = 0.004
Z_FOOT = 0.290
Z_ARM = 0.275
SEAT_HALF = 0.265
CARRIER_Z1 = 0.2685

FOOT_REAR = (0.030, 0.0)
FOOT_FRONT = (0.600, 0.0)
CORNER_REAR = (0.060, 0.600)
CORNER_FRONT = (0.540, 0.600)
SEAT_REAR = (0.110, 0.368)
SEAT_FRONT = (0.560, 0.400)
BACK_BOTTOM = (0.095, 0.420)
BACK_TILT_DEG = 13.0
BACK_LEN = 0.384
STRETCH_Y = 0.15
CARRIER_SEAT_U = (-0.0525, 0.455)
CARRIER_BACK_U = (-0.075, 0.381)

RHO_STEEL = 7850.0
OPEN_AREA = 0.40


def zleg(y):
    """📐 Lateral position of a splayed leg centre line at height y (feet 15 mm wider than the armrests, so the chairs nest when stacked)."""
    return Z_FOOT - (Z_FOOT - Z_ARM) * min(max(y / CORNER_REAR[1], 0.0), 1.0)


def sub(a, b):
    """➖ Component difference."""
    return tuple(x - y for x, y in zip(a, b))


def add(a, b):
    """➕ Component sum."""
    return tuple(x + y for x, y in zip(a, b))


def mul(a, s):
    """✖️ Scalar product."""
    return tuple(x * s for x in a)


def dot(a, b):
    """🔘 Dot product."""
    return sum(x * y for x, y in zip(a, b))


def norm(a):
    """📏 Euclidean length."""
    return math.sqrt(dot(a, a))


def unit(a):
    """🎯 Unit vector."""
    n = norm(a)
    return tuple(x / n for x in a)


def cross(a, b):
    """❎ Cross product."""
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def bend(corner, d_in, d_out, radius, steps):
    """🌀 Points of a circular bend of centre-line radius `radius` that turns a straight run arriving along `d_in` into one leaving along `d_out` at `corner` (2D)."""
    a = unit(d_in)
    b = unit(d_out)
    phi = math.acos(max(-1.0, min(1.0, dot(a, b))))
    lt = radius * math.tan(phi / 2)
    t1 = sub(corner, mul(a, lt))
    turn = a[0] * b[1] - a[1] * b[0]
    n = (-a[1], a[0]) if turn > 0 else (a[1], -a[0])
    c = add(t1, mul(n, radius))
    start = math.atan2(t1[1] - c[1], t1[0] - c[0])
    sign = 1 if turn > 0 else -1
    return [add(c, (radius * math.cos(start + sign * phi * k / steps), radius * math.sin(start + sign * phi * k / steps))) for k in range(steps + 1)], lt


def side_path(side, steps):
    """🦿 The 3D centre line of one bent side frame (side +1 left, -1 right): rear foot, rear bend, armrest, front bend, front foot, and the indices where the bends begin and end."""
    rear, lt_r = bend(CORNER_REAR, sub(CORNER_REAR, FOOT_REAR), (1.0, 0.0), BEND_R, steps)
    front, lt_f = bend(CORNER_FRONT, (1.0, 0.0), sub(FOOT_FRONT, CORNER_FRONT), BEND_R, steps)
    pts = [FOOT_REAR] + rear + front + [FOOT_FRONT]
    return [(x, y, side * zleg(y)) for x, y in pts], (1, 1 + steps, 2 + steps, 2 + 2 * steps)


def bone(id_, origin, tip):
    """🦴 A ground-parented fixed bone from `origin` to `tip` (2D world)."""
    d = sub(tip, origin)
    return {"id": id_, "x": origin[0], "y": origin[1], "angleDeg": math.degrees(math.atan2(d[1], d[0])), "lengthM": norm(d)}


BACK_DIR = (-math.sin(math.radians(BACK_TILT_DEG)), math.cos(math.radians(BACK_TILT_DEG)))
BACK_TOP = add(BACK_BOTTOM, mul(BACK_DIR, BACK_LEN))

BONES = {
    "ground": {"id": "ground", "x": 0.0, "y": 0.0, "angleDeg": 0.0, "lengthM": 0.0},
    "leg-rear": bone("leg-rear", FOOT_REAR, CORNER_REAR),
    "leg-front": bone("leg-front", FOOT_FRONT, CORNER_FRONT),
    "armrest": bone("armrest", CORNER_REAR, CORNER_FRONT),
    "seat": bone("seat", SEAT_REAR, SEAT_FRONT),
    "back": bone("back", BACK_BOTTOM, BACK_TOP),
}


def to_local(bone_id, p):
    """🧭 World point (2D or 3D) into the rest frame (u, v[, w]) of a ground-parented bone."""
    b = BONES[bone_id]
    a = math.radians(b["angleDeg"])
    dx, dy = p[0] - b["x"], p[1] - b["y"]
    uv = (dx * math.cos(a) + dy * math.sin(a), -dx * math.sin(a) + dy * math.cos(a))
    return uv if len(p) == 2 else (uv[0], uv[1], p[2])


def to_world(bone_id, q):
    """🌍 Bone-local point (u, v[, w]) into world coordinates."""
    b = BONES[bone_id]
    a = math.radians(b["angleDeg"])
    xy = (b["x"] + q[0] * math.cos(a) - q[1] * math.sin(a), b["y"] + q[0] * math.sin(a) + q[1] * math.cos(a))
    return xy if len(q) == 2 else (xy[0], xy[1], q[2])


def dir_local(bone_id, d):
    """➡️ World direction into the bone-local frame."""
    b = BONES[bone_id]
    a = math.radians(b["angleDeg"])
    return (d[0] * math.cos(a) + d[1] * math.sin(a), -d[0] * math.sin(a) + d[1] * math.cos(a), d[2])


def quat_from_columns(x, y, z):
    """🔄 Unit quaternion [x, y, z, w] of the rotation whose matrix columns are the axes x, y, z."""
    m00, m01, m02 = x[0], y[0], z[0]
    m10, m11, m12 = x[1], y[1], z[1]
    m20, m21, m22 = x[2], y[2], z[2]
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        q = ((m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s)
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1.0 + m00 - m11 - m22) * 2
        q = (0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s)
    elif m11 > m22:
        s = math.sqrt(1.0 + m11 - m00 - m22) * 2
        q = ((m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s)
    else:
        s = math.sqrt(1.0 + m22 - m00 - m11) * 2
        q = ((m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s)
    if q[3] < 0:
        q = tuple(-c for c in q)
    return q


def loft_sections(local_pts, r):
    """🧵 Circular loft sections along a bone-local 3D path with parallel-transport frames, so the skin never twists."""
    out = []
    prev_x = None
    for i, p in enumerate(local_pts):
        a = local_pts[max(i - 1, 0)]
        b = local_pts[min(i + 1, len(local_pts) - 1)]
        t = unit(sub(b, a))
        if prev_x is None:
            ref = (0.0, 0.0, 1.0) if abs(t[2]) < 0.9 else (1.0, 0.0, 0.0)
            x = unit(cross(ref, t))
        else:
            x = unit(sub(prev_x, mul(t, dot(prev_x, t))))
        y = cross(t, x)
        prev_x = x
        out.append({"at": [round(c, 6) for c in p], "rx": r, "ry": r, "rotation": [round(c, 6) for c in quat_from_columns(x, y, t)]})
    return out


def offset_path(pts, r, cap0="flat", cap1="flat", arc=8):
    """🖍️ Closed outline of a 2D polyline widened by `r` on both sides (mitred joints), with flat or round end caps."""
    n = len(pts)
    normals = []
    for i in range(n - 1):
        d = unit(sub(pts[i + 1], pts[i]))
        normals.append((-d[1], d[0]))
    left, right = [], []
    for i in range(n):
        if i == 0:
            m = normals[0]
            s = 1.0
        elif i == n - 1:
            m = normals[-1]
            s = 1.0
        else:
            a, b = normals[i - 1], normals[i]
            m = unit(add(a, b))
            s = 1.0 / max(dot(m, a), 0.3)
        left.append(add(pts[i], mul(m, r * s)))
        right.append(sub(pts[i], mul(m, r * s)))
    ring = list(left)
    if cap1 == "round":
        d = unit(sub(pts[-1], pts[-2]))
        a0 = math.atan2(normals[-1][1], normals[-1][0])
        ring += [add(pts[-1], (r * math.cos(a0 - math.pi * k / arc), r * math.sin(a0 - math.pi * k / arc))) for k in range(1, arc)]
    ring += list(reversed(right))
    if cap0 == "round":
        a0 = math.atan2(-normals[0][1], -normals[0][0])
        ring += [add(pts[0], (r * math.cos(a0 - math.pi * k / arc), r * math.sin(a0 - math.pi * k / arc))) for k in range(1, arc)]
    return ring


def hull(points):
    """🫧 Convex hull (monotone chain) of 2D points, counter-clockwise."""
    pts = sorted(set((round(p[0], 7), round(p[1], 7)) for p in points))
    if len(pts) < 3:
        return pts

    def turn(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and turn(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and turn(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def circle(c, r, n=20):
    """⭕ Polygon of a circle."""
    return [(c[0] + r * math.cos(2 * math.pi * k / n), c[1] + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def view_of(p, view):
    """👁️ World 3D point to the paper coordinates of a view in world metres: elevation (x, y), front (z, y), plan (x, z)."""
    return {"elevation": (p[0], p[1]), "front": (p[2], p[1]), "plan": (p[0], p[2])}[view]


def tube_mass(length, d=T_SEC[0], t=T_SEC[1]):
    """⚖️ Mass of a steel tube of outer diameter d and wall t."""
    return math.pi / 4 * (d * d - (d - 2 * t) ** 2) * length * RHO_STEEL


def path_length(pts):
    """📐 Length of a polyline."""
    return sum(norm(sub(pts[i + 1], pts[i])) for i in range(len(pts) - 1))
