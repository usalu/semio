"""🪝 Geometry of the used bentwood coat stand: hand-chosen centrelines, the turned pole profile and the folded blanket, plus the helpers that sample, project, offset and measure them.

All numbers are metres. Stand coordinates are (r, phi, h): radius from the pole axis, azimuth from ahead (+x) towards the left (+z) and height above the stand origin (the underside of the feet), which rests 3 mm deep in the folded moving blanket.

@see ../../brief.md — the author brief
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — frames of shapes and solids
"""
import math

BLANKET_TOP = 0.045
STAND_BASE = 0.042
BLANKET_X = 0.70
BLANKET_Z = 0.56
G = 9.80665

POLE_PROFILE = [
    (0.075, 0.000), (0.075, 0.010), (0.078, 0.016), (0.085, 0.020), (0.095, 0.024), (0.105, 0.025), (0.115, 0.024), (0.125, 0.022),
    (0.140, 0.023), (0.300, 0.0235), (0.800, 0.0235), (1.300, 0.0225), (1.495, 0.022), (1.500, 0.025), (1.507, 0.027), (1.543, 0.027),
    (1.550, 0.025), (1.556, 0.022), (1.650, 0.022), (1.656, 0.025), (1.663, 0.028), (1.717, 0.028), (1.724, 0.025), (1.730, 0.020),
    (1.742, 0.016), (1.750, 0.018), (1.758, 0.026), (1.770, 0.034), (1.786, 0.040), (1.802, 0.041), (1.816, 0.038), (1.828, 0.031),
    (1.838, 0.022), (1.845, 0.013), (1.849, 0.006), (1.850, 0.000),
]

FOOT = [(0.034, 0.680), (0.034, 0.640), (0.040, 0.600), (0.058, 0.565), (0.088, 0.532), (0.120, 0.500), (0.146, 0.465), (0.165, 0.420),
        (0.178, 0.360), (0.189, 0.290), (0.200, 0.210), (0.212, 0.140), (0.226, 0.080), (0.243, 0.036), (0.265, 0.016), (0.290, 0.013),
        (0.312, 0.018), (0.327, 0.030)]
FOOT_R = 0.013
FOOT_PHI = [45.0, 135.0, 225.0, 315.0]

UPPER = [(0.012, 1.700), (0.040, 1.706), (0.082, 1.708), (0.130, 1.702), (0.180, 1.684), (0.226, 1.652), (0.262, 1.610), (0.283, 1.562),
         (0.289, 1.520), (0.279, 1.486), (0.258, 1.466), (0.233, 1.461), (0.211, 1.470), (0.199, 1.488), (0.197, 1.507)]
UPPER_R = 0.011
UPPER_PHI = [0.0, 90.0, 180.0, 270.0]
UPPER_HANG = (0.233, 1.472)

LOWER = [(0.008, 1.522), (0.040, 1.527), (0.075, 1.526), (0.110, 1.514), (0.140, 1.492), (0.160, 1.462), (0.168, 1.428), (0.163, 1.398),
         (0.147, 1.377), (0.125, 1.370), (0.104, 1.376), (0.091, 1.392), (0.088, 1.411), (0.091, 1.430)]
LOWER_R = 0.009
LOWER_PHI = [45.0, 135.0, 315.0]
LOWER_HANG = (0.125, 1.379)
BROKEN_PHI = 225.0
STUMP = [(0.008, 1.522), (0.040, 1.527), (0.068, 1.524)]

STEEL_PLATE = {"h": 1.405, "len": 0.050, "thick": 0.006, "wide": 0.020, "r_in": 0.0205}
STEEL_ARM = [(0.024, 1.418), (0.050, 1.414), (0.074, 1.404), (0.090, 1.410), (0.097, 1.426), (0.095, 1.442)]
STEEL_ARM_R = 0.0045
STEEL_BALL = {"r": 0.006}
STEEL_HANG = (0.082, 1.414)

RING_H = 0.400
RING_R = 0.189
RING_T = 0.009

GRIP_LOWER_U = 0.75
GRIP_UPPER_U = 1.20
LABEL = {"h": 1.05, "w": 0.025, "hgt": 0.040}

BLANKET_SIDE = [(0.000, 0.016), (0.004, 0.006), (0.014, 0.001), (0.030, 0.000), (0.350, 0.000), (0.670, 0.000), (0.686, 0.001),
                (0.696, 0.006), (0.700, 0.016), (0.700, 0.030), (0.696, 0.039), (0.686, 0.044), (0.670, 0.045), (0.350, 0.0452),
                (0.030, 0.045), (0.014, 0.044), (0.004, 0.039), (0.000, 0.030)]


def rnd(value, digits=5):
    """🔢 Rounds a number or a nested list of numbers for the document."""
    if isinstance(value, (list, tuple)):
        return [rnd(v, digits) for v in value]
    out = round(value, digits)
    return 0.0 if out == 0 else out


def catmull(points, per_segment):
    """🧵 Samples a centripetal-free uniform Catmull-Rom spline through the control points with `per_segment` samples per segment (1 returns the control points)."""
    if per_segment <= 1:
        return [tuple(p) for p in points]
    out = []
    n = len(points)
    for i in range(n - 1):
        p0 = points[max(i - 1, 0)]
        p1 = points[i]
        p2 = points[i + 1]
        p3 = points[min(i + 2, n - 1)]
        for k in range(per_segment):
            t = k / per_segment
            t2, t3 = t * t, t * t * t
            out.append(tuple(0.5 * ((2 * p1[j]) + (-p0[j] + p2[j]) * t + (2 * p0[j] - 5 * p1[j] + 4 * p2[j] - p3[j]) * t2 + (-p0[j] + 3 * p1[j] - 3 * p2[j] + p3[j]) * t3) for j in range(2)))
    out.append(tuple(points[-1]))
    return out


def pick(points, indices):
    """🎯 The control points at the given indices: a hand-chosen coarser polyline of the same curve."""
    return [points[i] for i in indices]


def path_length(points):
    """📏 Length of a polyline in any dimension."""
    return sum(math.dist(points[i], points[i + 1]) for i in range(len(points) - 1))


def to_local(r, phi, h):
    """🧭 Stand-bone coordinates (u up, v backwards, w left) of a point at radius r, azimuth phi (degrees) and height h."""
    a = math.radians(phi)
    return (h, -r * math.cos(a), r * math.sin(a))


def world_x(r, phi):
    """➡️ Forward world coordinate of a radial point."""
    return r * math.cos(math.radians(phi))


def world_z(r, phi):
    """⬅️ Lateral world coordinate (left +) of a radial point."""
    return r * math.sin(math.radians(phi))


def quat_from_columns(a, b, c):
    """🌀 Unit quaternion [x, y, z, w] of the rotation whose matrix has the columns a, b, c."""
    m00, m10, m20 = a
    m01, m11, m21 = b
    m02, m12, m22 = c
    trace = m00 + m11 + m22
    if trace > 0:
        s = math.sqrt(trace + 1.0) * 2
        w, x, y, z = 0.25 * s, (m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1.0 + m00 - m11 - m22) * 2
        w, x, y, z = (m21 - m12) / s, 0.25 * s, (m01 + m10) / s, (m02 + m20) / s
    elif m11 > m22:
        s = math.sqrt(1.0 + m11 - m00 - m22) * 2
        w, x, y, z = (m02 - m20) / s, (m01 + m10) / s, 0.25 * s, (m12 + m21) / s
    else:
        s = math.sqrt(1.0 + m22 - m00 - m11) * 2
        w, x, y, z = (m10 - m01) / s, (m02 + m20) / s, (m12 + m21) / s, 0.25 * s
    n = math.sqrt(x * x + y * y + z * z + w * w)
    return [x / n, y / n, z / n, w / n]


def cross(a, b):
    """✖️ Cross product of two 3-vectors."""
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def unit(a):
    """🎯 The vector scaled to unit length."""
    n = math.sqrt(sum(x * x for x in a))
    return tuple(x / n for x in a)


def tube_loft(points, radius, phi, taper_tip=True, taper_start=False):
    """🥢 Loft sections of a round rod of `radius` along a radial-plane centreline (r, h) at azimuth `phi`, each section perpendicular to the path; a tapered tip closes the rod end in a rounded knob."""
    pts = [to_local(r, phi, h) for r, h in points]
    a = math.radians(phi)
    lateral = (0.0, math.sin(a), math.cos(a))
    rows = [(p, radius) for p in pts]
    if taper_tip:
        last, prev = pts[-1], pts[-2]
        t = unit(tuple(last[i] - prev[i] for i in range(3)))
        rows += [(tuple(last[i] + t[i] * radius * f for i in range(3)), radius * s) for f, s in ((0.45, 0.89), (0.8, 0.6), (0.97, 0.24))]
    if taper_start:
        first, nxt = pts[0], pts[1]
        t = unit(tuple(first[i] - nxt[i] for i in range(3)))
        rows = [(tuple(first[i] + t[i] * radius * f for i in range(3)), radius * s) for f, s in ((0.97, 0.24), (0.8, 0.6), (0.45, 0.89))] + rows
    sections = []
    for i, (p, rad) in enumerate(rows):
        q0 = rows[max(i - 1, 0)][0]
        q1 = rows[min(i + 1, len(rows) - 1)][0]
        t = unit(tuple(q1[k] - q0[k] for k in range(3)))
        b = unit(tuple(lateral[k] - t[k] * sum(lateral[j] * t[j] for j in range(3)) for k in range(3)))
        c = cross(b, t)
        sections.append({"at": rnd(list(p)), "rx": rnd(rad), "ry": rnd(rad), "rotation": rnd(quat_from_columns(c, b, t), 6)})
    return sections


def offset_band(points, radius, cap_end=True, cap_start=True, tip_taper=False, segs=6):
    """🖊️ Closed outline of a constant-width band of half-width `radius` around a 2D centreline, with round caps; the 2D picture of a round rod in any projection."""
    n = len(points)
    normals = []
    for i in range(n):
        p0 = points[max(i - 1, 0)]
        p1 = points[min(i + 1, n - 1)]
        tx, ty = p1[0] - p0[0], p1[1] - p0[1]
        ln = math.hypot(tx, ty)
        normals.append((-ty / ln, tx / ln))
    left = [(points[i][0] + radius * normals[i][0], points[i][1] + radius * normals[i][1]) for i in range(n)]
    right = [(points[i][0] - radius * normals[i][0], points[i][1] - radius * normals[i][1]) for i in range(n)]
    out = list(left)
    if cap_end:
        nx, ny = normals[-1]
        cx, cy = points[-1]
        base = math.atan2(ny, nx)
        out += [(cx + radius * math.cos(base - math.pi * k / segs), cy + radius * math.sin(base - math.pi * k / segs)) for k in range(1, segs)]
    out += list(reversed(right))
    if cap_start:
        nx, ny = normals[0]
        cx, cy = points[0]
        base = math.atan2(-ny, -nx)
        out += [(cx + radius * math.cos(base - math.pi * k / segs), cy + radius * math.sin(base - math.pi * k / segs)) for k in range(1, segs)]
    return [rnd(list(p)) for p in out]


def segments_intersect(p1, p2, p3, p4):
    """✂️ Whether two closed segments properly cross."""
    def orient(a, b, c):
        return (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
    d1, d2, d3, d4 = orient(p3, p4, p1), orient(p3, p4, p2), orient(p1, p2, p3), orient(p1, p2, p4)
    return ((d1 > 0) != (d2 > 0)) and ((d3 > 0) != (d4 > 0)) and d1 * d2 < 0 and d3 * d4 < 0


def self_intersects(ring):
    """🔍 Whether a closed polygon crosses itself (brute force)."""
    n = len(ring)
    for i in range(n):
        a, b = ring[i], ring[(i + 1) % n]
        for j in range(i + 2, n):
            if i == 0 and j == n - 1:
                continue
            c, d = ring[j], ring[(j + 1) % n]
            if segments_intersect(a, b, c, d):
                return True
    return False


def revolve_area_volume(profile):
    """🏺 Surface area and volume of a solid of revolution of a (h, r) profile closed through the axis."""
    area = 0.0
    vol = 0.0
    for (h0, r0), (h1, r1) in zip(profile, profile[1:]):
        slant = math.hypot(h1 - h0, r1 - r0)
        area += math.pi * (r0 + r1) * slant
        vol += math.pi * (h1 - h0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3
    return area, abs(vol)


def rod_area_volume(points, radius):
    """🥢 Lateral area plus two end discs and volume of a round rod along a centreline."""
    length = path_length(points)
    return 2 * math.pi * radius * length + 2 * math.pi * radius * radius, math.pi * radius * radius * length, length


def polygon_area(points):
    """📐 Signed shoelace area of a polygon."""
    return 0.5 * sum(points[i][0] * points[(i + 1) % len(points)][1] - points[(i + 1) % len(points)][0] * points[i][1] for i in range(len(points)))


def polygon_perimeter(points):
    """⭕ Perimeter of a closed polygon."""
    return sum(math.dist(points[i], points[(i + 1) % len(points)]) for i in range(len(points)))


def circle_path(cx, cy, r):
    """🔵 An SVG path of a full circle."""
    return f"M {rnd(cx + r)} {rnd(cy)} A {rnd(r)} {rnd(r)} 0 1 0 {rnd(cx - r)} {rnd(cy)} A {rnd(r)} {rnd(r)} 0 1 0 {rnd(cx + r)} {rnd(cy)} Z"
