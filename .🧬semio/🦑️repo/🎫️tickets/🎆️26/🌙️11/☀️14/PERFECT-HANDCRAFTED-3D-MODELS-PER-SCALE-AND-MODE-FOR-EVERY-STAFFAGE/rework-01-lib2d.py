"""✏️ Author 01 ruler: the orthographic outline of one authored body solid in the frame of the 2D shapes (elevation: bone-local u, v; front: lateral, height of the rest pose; plan: ahead, lateral of the rest pose), so that every drawing shape is the view of its own solid. A loft seen from the side is the ribbon between its two flanks, a loft seen along its path the hull of its sections, an ellipsoid an ellipse, a capsule a capsule. Points closer than the stated tolerance to their neighbours' chord are left out.
"""
import math


def qrot(q, v):
    x, y, z, w = q
    tx, ty, tz = 2 * (y * v[2] - z * v[1]), 2 * (z * v[0] - x * v[2]), 2 * (x * v[1] - y * v[0])
    return [v[0] + w * tx + (y * tz - z * ty), v[1] + w * ty + (z * tx - x * tz), v[2] + w * tz + (x * ty - y * tx)]


class Views:
    """🎥 Projection of bone-local points and directions into a view."""

    def __init__(self, frames):
        self.f = frames

    def point(self, bone, local, view):
        if view == "elevation":
            return (local[0], local[1])
        if view == "lateral":
            return (local[0], local[2])
        world = self.f.world(bone, local)
        return (world[2], world[1]) if view == "front" else (world[0], world[2])

    def vector(self, bone, local, view):
        if view == "elevation":
            return (local[0], local[1])
        if view == "lateral":
            return (local[0], local[2])
        angle = math.radians(self.f.rest[bone][3])
        c, s = math.cos(angle), math.sin(angle)
        world = (local[0] * c - local[1] * s, local[0] * s + local[1] * c, local[2])
        return (world[2], world[1]) if view == "front" else (world[0], world[2])

    def lateral(self, bone, local):
        return self.f.world(bone, local)[2]


def simplify(points, tolerance):
    """🪒 Douglas-Peucker on a closed ring, anchored at its two farthest points."""
    if len(points) < 6:
        return points
    def dp(seq):
        if len(seq) < 3:
            return seq
        (ax, ay), (bx, by) = seq[0], seq[-1]
        length = math.hypot(bx - ax, by - ay) or 1e-12
        worst, index = 0, 0
        for i in range(1, len(seq) - 1):
            d = abs((bx - ax) * (ay - seq[i][1]) - (ax - seq[i][0]) * (by - ay)) / length
            if d > worst:
                worst, index = d, i
        if worst <= tolerance:
            return [seq[0], seq[-1]]
        return dp(seq[: index + 1])[:-1] + dp(seq[index:])
    far = max(range(len(points)), key=lambda i: math.hypot(points[i][0] - points[0][0], points[i][1] - points[0][1]))
    return dp(points[: far + 1])[:-1] + dp(points[far:] + [points[0]])[:-1]


def hull(points):
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
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


def loft_outline(views, solid, view, tolerance):
    bone = solid["bone"]
    rows = []
    for s in solid["sections"]:
        ex, ey = qrot(s["rotation"], [1, 0, 0]), qrot(s["rotation"], [0, 1, 0])
        rows.append((views.point(bone, s["at"], view), views.vector(bone, ex, view), views.vector(bone, ey, view), s["rx"], s["ry"], s.get("exponent", 2)))
    travel = sum(math.hypot(rows[i + 1][0][0] - rows[i][0][0], rows[i + 1][0][1] - rows[i][0][1]) for i in range(len(rows) - 1))
    reach = max(max(r[3], r[4]) for r in rows)
    if travel < 1.2 * reach:
        ring = []
        for p, ex, ey, rx, ry, e in rows:
            for k in range(32):
                a = 2 * math.pi * k / 32
                c, s = math.cos(a), math.sin(a)
                x = rx * math.copysign(abs(c) ** (2 / e), c)
                y = ry * math.copysign(abs(s) ** (2 / e), s)
                ring.append((p[0] + ex[0] * x + ey[0] * y, p[1] + ex[1] * x + ey[1] * y))
        return simplify(hull(ring), tolerance)
    left, right = [], []
    count = len(rows)
    for i, (p, ex, ey, rx, ry, e) in enumerate(rows):
        a, b = rows[max(0, i - 1)][0], rows[min(count - 1, i + 1)][0]
        n = math.hypot(b[0] - a[0], b[1] - a[1]) or 1e-12
        t = ((b[0] - a[0]) / n, (b[1] - a[1]) / n)
        q = (-t[1], t[0])
        h = math.hypot((ex[0] * q[0] + ex[1] * q[1]) * rx, (ey[0] * q[0] + ey[1] * q[1]) * ry)
        along = math.hypot((ex[0] * t[0] + ex[1] * t[1]) * rx, (ey[0] * t[0] + ey[1] * t[1]) * ry) if i in (0, count - 1) else 0
        if i == 0 and along > tolerance:
            right.append((p[0] - t[0] * along, p[1] - t[1] * along))
        left.append((p[0] + q[0] * h, p[1] + q[1] * h))
        right.append((p[0] - q[0] * h, p[1] - q[1] * h))
        if i == count - 1 and along > tolerance:
            left.append((p[0] + t[0] * along, p[1] + t[1] * along))
    return simplify(left + right[::-1], tolerance)


def loft_span(views, solid):
    """↔️ The lateral reach (rest-world z) of a loft."""
    lo, hi = math.inf, -math.inf
    for s in solid["sections"]:
        ex, ey = qrot(s["rotation"], [1, 0, 0]), qrot(s["rotation"], [0, 1, 0])
        z = views.lateral(solid["bone"], s["at"])
        h = math.hypot(ex[2] * s["rx"], ey[2] * s["ry"])
        lo, hi = min(lo, z - h), max(hi, z + h)
    return lo, hi


def reach(solid, axis):
    """↔️ The extent of a solid along a bone-local axis (1 = v, 2 = w)."""
    kind = solid["kind"]
    if kind == "loft":
        lo, hi = math.inf, -math.inf
        for s in solid["sections"]:
            ex, ey = qrot(s["rotation"], [1, 0, 0]), qrot(s["rotation"], [0, 1, 0])
            h = math.hypot(ex[axis] * s["rx"], ey[axis] * s["ry"])
            lo, hi = min(lo, s["at"][axis] - h), max(hi, s["at"][axis] + h)
        return lo, hi
    if kind == "capsule":
        return min(solid["p0"][axis] - solid["r0"], solid["p1"][axis] - solid["r1"]), max(solid["p0"][axis] + solid["r0"], solid["p1"][axis] + solid["r1"])
    centre = solid["transform"]["translation"][axis]
    half = solid["radii"][axis] if kind == "ellipsoid" else solid["size"][axis] / 2
    if axis == 1 and kind == "ellipsoid":
        q = solid["transform"].get("rotation", [0, 0, 0, 1])
        a = 2 * math.atan2(q[2], q[3])
        half = math.hypot(solid["radii"][0] * math.sin(a), solid["radii"][1] * math.cos(a))
    return centre - half, centre + half


def geometry(views, solid, view, tolerance):
    """📐 The 2D geometry members of a body solid in a view, and its lateral span."""
    bone, kind = solid["bone"], solid["kind"]
    r = lambda x: round(x, 4) + 0.0
    if kind == "loft":
        return {"kind": "poly", "points": [[r(p[0]), r(p[1])] for p in loft_outline(views, solid, view, tolerance)], "closed": True}, loft_span(views, solid)
    if kind == "ellipsoid":
        centre = solid["transform"]["translation"]
        q = solid["transform"].get("rotation", [0, 0, 0, 1])
        local = 2 * math.atan2(q[2], q[3])
        a, b, c = solid["radii"]
        p = views.point(bone, centre, view)
        z = views.lateral(bone, centre)
        if view == "lateral":
            return {"kind": "ellipse", "cx": r(p[0]), "cy": r(p[1]), "rx": r(math.hypot(a * math.cos(local), b * math.sin(local))), "ry": r(c)}, (z - c, z + c)
        if view == "elevation":
            shape = {"kind": "ellipse", "cx": r(p[0]), "cy": r(p[1]), "rx": r(a), "ry": r(b)}
            if abs(math.degrees(local)) > 0.5 and abs(a - b) > 1e-6:
                shape["rotationDeg"] = round(math.degrees(local), 2)
            return shape, (z - c, z + c)
        world = local + math.radians(views.f.rest[bone][3])
        up, ahead = math.hypot(a * math.sin(world), b * math.cos(world)), math.hypot(a * math.cos(world), b * math.sin(world))
        if view == "front":
            return {"kind": "ellipse", "cx": r(p[0]), "cy": r(p[1]), "rx": r(c), "ry": r(up)}, (z - c, z + c)
        return {"kind": "ellipse", "cx": r(p[0]), "cy": r(p[1]), "rx": r(ahead), "ry": r(c)}, (z - c, z + c)
    if kind == "capsule":
        p0, p1 = views.point(bone, solid["p0"], view), views.point(bone, solid["p1"], view)
        z0, z1 = views.lateral(bone, solid["p0"]), views.lateral(bone, solid["p1"])
        span = (min(z0 - solid["r0"], z1 - solid["r1"]), max(z0 + solid["r0"], z1 + solid["r1"]))
        if math.hypot(p1[0] - p0[0], p1[1] - p0[1]) < 1e-4:
            radius = max(solid["r0"], solid["r1"])
            return {"kind": "ellipse", "cx": r(p0[0]), "cy": r(p0[1]), "rx": r(radius), "ry": r(radius)}, span
        return {"kind": "capsule", "x0": r(p0[0]), "y0": r(p0[1]), "x1": r(p1[0]), "y1": r(p1[1]), "r0": r(solid["r0"]), "r1": r(solid["r1"])}, span
    if kind == "box":
        centre = solid["transform"]["translation"]
        q = solid["transform"].get("rotation", [0, 0, 0, 1])
        half = [x / 2 for x in solid["size"]]
        axis = max(range(3), key=lambda i: math.hypot(*views.vector(bone, qrot(q, [1 if j == i else 0 for j in range(3)]), view)) * half[i])
        d = qrot(q, [half[axis] if j == axis else 0 for j in range(3)])
        a = views.point(bone, [centre[j] - d[j] for j in range(3)], view)
        b = views.point(bone, [centre[j] + d[j] for j in range(3)], view)
        z = views.lateral(bone, centre)
        return {"kind": "poly", "points": [[r(a[0]), r(a[1])], [r(b[0]), r(b[1])]], "closed": False}, (z - half[2], z + half[2])
    raise ValueError(f"no outline for {kind}")
