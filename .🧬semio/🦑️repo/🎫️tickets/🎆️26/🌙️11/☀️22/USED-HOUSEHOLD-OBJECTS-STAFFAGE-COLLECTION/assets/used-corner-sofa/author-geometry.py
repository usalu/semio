"""🧮️ Geometry kit of the staffage asset `used-corner-sofa`: rotation senses of the rig, quaternions, superellipse sections, convex hulls and the outline helpers that the authoritative script `author-used-corner-sofa.py` draws its 2D shapes and solids with.

@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Rest placement of a bone" (senses of `angleDeg`, `abductionDeg`, `twistDeg`) and "Volume" (loft sections)
"""
import math


def rw(deg):
    """🔄️ Rotation about the joint axis w: u towards v."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]


def rv(deg):
    """↪️ Rotation about the normal v: u towards w."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, 0.0, -s], [0.0, 1.0, 0.0], [s, 0.0, c]]


def ru(deg):
    """🌀️ Rotation about the bone axis u: v towards -w."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[1.0, 0.0, 0.0], [0.0, c, s], [0.0, -s, c]]


def mul(a, b):
    """✖️ Product of two 3x3 matrices."""
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def apply(m, p):
    """➡️ A 3x3 matrix applied to a vector."""
    return [sum(m[i][k] * p[k] for k in range(3)) for i in range(3)]


def column(m, j):
    """📐️ Column j of a 3x3 matrix."""
    return [m[0][j], m[1][j], m[2][j]]


def qmul(a, b):
    """🔗️ Hamilton product of two quaternions [x, y, z, w] (b applied first)."""
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def qmat(q):
    """🧭️ The rotation matrix of a unit quaternion [x, y, z, w]."""
    x, y, z, w = q
    return [[1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)], [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)], [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)]]


LIMB = [0.5, 0.5, 0.5, 0.5]


def lean_quat(deg):
    """🪑️ The section rotation of a loft along u whose section x axis leans from v towards w by `deg` (rx along the leaning height, ry along the thickness)."""
    h = math.radians(deg) / 2
    return qmul([math.sin(h), 0.0, 0.0, math.cos(h)], LIMB)


def superellipse(rx, ry, n, count=56):
    """🥚️ Points of |x/rx|^n + |y/ry|^n = 1, counter-clockwise from +x."""
    pts = []
    for k in range(count):
        t = 2 * math.pi * k / count
        c, s = math.cos(t), math.sin(t)
        pts.append((rx * math.copysign(abs(c) ** (2 / n), c), ry * math.copysign(abs(s) ** (2 / n), s)))
    return pts


def section_points(section, count=56):
    """🧩️ The bone-local 3D points of one loft section."""
    m = qmat(section.get("rotation", LIMB))
    at = section["at"]
    out = []
    for x, y in superellipse(section["rx"], section["ry"], section.get("exponent", 2), count):
        d = apply(m, [x, y, 0.0])
        out.append((at[0] + d[0], at[1] + d[1], at[2] + d[2]))
    return out


def hull(points):
    """🫧️ Convex hull (monotone chain), counter-clockwise."""
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) < 3:
        return pts

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def simplify(points, tol):
    """✂️ Drops vertices closer than `tol` to the chord of their neighbours (closed ring)."""
    pts = list(points)
    changed = True
    while changed and len(pts) > 4:
        changed = False
        for i in range(len(pts)):
            a, b, c = pts[i - 1], pts[i], pts[(i + 1) % len(pts)]
            dx, dy = c[0] - a[0], c[1] - a[1]
            length = math.hypot(dx, dy)
            dist = abs((b[0] - a[0]) * dy - (b[1] - a[1]) * dx) / length if length > 0 else math.hypot(b[0] - a[0], b[1] - a[1])
            if dist < tol:
                pts.pop(i)
                changed = True
                break
    return pts


def rrect(x0, y0, x1, y1, r, seg=4):
    """⬜️ A rounded rectangle from (x0, y0) to (x1, y1) with corner radius r, counter-clockwise."""
    r = min(r, (x1 - x0) / 2, (y1 - y0) / 2)
    if r <= 1e-9:
        return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
    pts = []
    for cx, cy, a0 in ((x1 - r, y0 + r, -90), (x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180)):
        for k in range(seg + 1):
            a = math.radians(a0 + 90 * k / seg)
            pts.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    return pts


def arc(cx, cy, r, a0, a1, seg):
    """🌙️ Points of a circular arc from angle a0 to a1 (degrees), both ends included."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / seg)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / seg))) for k in range(seg + 1)]


def inside(point, poly):
    """📍️ Even-odd point-in-polygon test."""
    x, y = point
    hit = False
    for i in range(len(poly)):
        x0, y0 = poly[i - 1]
        x1, y1 = poly[i]
        if (y0 > y) != (y1 > y) and x < x0 + (y - y0) * (x1 - x0) / (y1 - y0):
            hit = not hit
    return hit


def area(poly):
    """📏️ Signed shoelace area."""
    return sum(poly[i - 1][0] * poly[i][1] - poly[i][0] * poly[i - 1][1] for i in range(len(poly))) / 2


def r5(v):
    """🔢️ Rounds a coordinate to 0.01 mm."""
    return round(v + 0.0, 5)


def pts5(points):
    """🔢️ Rounds a point list to 0.01 mm."""
    return [[r5(p[0]), r5(p[1])] for p in points]
