"""🧮 Round 3, vegetation/miscanthus-grass: vectors, the planar rest rig, bone-local lofts from world paths and the projections of the three drawing views. It converts only; every form is stated in `r3-miscanthus-grass-plant.py`.

See: ./r3-vegetation-miscanthus-grass.md
"""
import math

UP = (0.0, 1.0, 0.0)


def add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def mul(a, k):
    return (a[0] * k, a[1] * k, a[2] * k)


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def norm(a):
    return math.sqrt(dot(a, a))


def unit(a):
    n = norm(a)
    return (a[0] / n, a[1] / n, a[2] / n)


def lerp(a, b, t):
    return (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t)


def direction(inclination_deg, azimuth_deg):
    """🧭 The unit vector leaning `inclination_deg` from the vertical towards the azimuth (0 = +x ahead, 90 = +z left)."""
    i, a = math.radians(inclination_deg), math.radians(azimuth_deg)
    return (math.sin(i) * math.cos(a), math.cos(i), math.sin(i) * math.sin(a))


def r5(x):
    return round(x, 5)


def quaternion(c0, c1, c2):
    """🌀 The unit quaternion [x, y, z, w] of the rotation matrix with the columns c0, c1, c2."""
    m00, m10, m20 = c0
    m01, m11, m21 = c1
    m02, m12, m22 = c2
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
        q = ((m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4)
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1 + m00 - m11 - m22) * 2
        q = (s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s)
    elif m11 > m22:
        s = math.sqrt(1 + m11 - m00 - m22) * 2
        q = ((m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s)
    else:
        s = math.sqrt(1 + m22 - m00 - m11) * 2
        q = ((m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s)
    n = math.sqrt(sum(v * v for v in q))
    return [r5(v / n) for v in q]


class Rig:
    """🦴 The rest frames of the planar rig: origin and cumulative angle of every bone in world space (x ahead, y up, z left)."""

    def __init__(self, bones):
        self.frames = {}
        by_id = {b["id"]: b for b in bones}

        def frame(bone_id):
            if bone_id in self.frames:
                return self.frames[bone_id]
            b = by_id[bone_id]
            if b["parent"] is None:
                origin, angle = (b["x"], b["y"], b.get("z", 0.0)), b["angleDeg"]
            else:
                po, pa = frame(b["parent"])
                c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
                origin = (po[0] + b["x"] * c - b["y"] * s, po[1] + b["x"] * s + b["y"] * c, po[2] + b.get("z", 0.0))
                angle = pa + b["angleDeg"]
            self.frames[bone_id] = (origin, angle)
            return self.frames[bone_id]

        for b in bones:
            frame(b["id"])
        self.length = {b["id"]: b["lengthM"] for b in bones}

    def axes(self, bone):
        a = math.radians(self.frames[bone][1])
        return (math.cos(a), math.sin(a), 0.0), (-math.sin(a), math.cos(a), 0.0), (0.0, 0.0, 1.0)

    def origin(self, bone):
        return self.frames[bone][0]

    def tip(self, bone):
        return add(self.origin(bone), mul(self.axes(bone)[0], self.length[bone]))

    def point(self, bone, p):
        u, v, w = self.axes(bone)
        d = sub(p, self.origin(bone))
        return [r5(dot(d, u)), r5(dot(d, v)), r5(dot(d, w))]

    def vector(self, bone, d):
        u, v, w = self.axes(bone)
        return (dot(d, u), dot(d, v), dot(d, w))


def tangents(points):
    out = []
    for i in range(len(points)):
        a, b = points[max(i - 1, 0)], points[min(i + 1, len(points) - 1)]
        out.append(unit(sub(b, a)))
    return out


def width_axis(tangent, fallback):
    """↔️ The horizontal axis across a blade whose path has the tangent; `fallback` where the path is nearly vertical and fixes the sign."""
    c = cross(UP, tangent)
    x = unit(c) if norm(c) > 0.25 else unit(fallback)
    return x if dot(x, fallback) >= 0 else mul(x, -1)


def loft(rig, solid_id, bone, points, widths, rx, ry, fields, exponent=2.0, caps=True):
    """🪢 A bone-local loft through the world `points`; `widths[i]` is the unit axis of `rx[i]`, the section normal is the path tangent."""
    sections = []
    for p, t, x, a, b in zip(points, tangents(points), widths, rx, ry):
        x = unit(sub(x, mul(t, dot(x, t))))
        y = cross(t, x)
        section = {"at": rig.point(bone, p), "rx": r5(max(a, 2e-4)), "ry": r5(max(b, 2e-4))}
        if exponent != 2.0:
            section["exponent"] = exponent
        section["rotation"] = quaternion(rig.vector(bone, x), rig.vector(bone, y), rig.vector(bone, t))
        sections.append(section)
    return {"id": solid_id, "kind": "loft", "bone": bone, **fields, "sections": sections, "capStart": caps, "capEnd": caps}


def rod(rig, solid_id, bone, points, radii, fields):
    """🥢 A round bone-local loft through the world `points`."""
    ts = tangents(points)
    widths = []
    for t in ts:
        c = cross(t, (0.0, 0.0, 1.0))
        widths.append(unit(c) if norm(c) > 0.2 else (1.0, 0.0, 0.0))
    return loft(rig, solid_id, bone, points, widths, radii, radii, fields)


def bezier(p0, c, p1, n):
    return [add(add(mul(p0, (1 - t) ** 2), mul(c, 2 * t * (1 - t))), mul(p1, t * t)) for t in (i / (n - 1) for i in range(n))]


def arch(start, d0, d1, length, power, n, steps=48, floor=None):
    """🌾 A path of `length` from `start` whose direction turns from d0 to d1 with the share s^power of its length and which lies down on the `floor` where it reaches it; `n` points at equal arc length."""
    pts, p = [start], start
    for k in range(steps):
        s = (k + 0.5) / steps
        d = unit(lerp(d0, d1, s ** power))
        p = add(p, mul(d, length / steps))
        if floor is not None and p[1] < floor:
            p = (p[0], floor, p[2])
        pts.append(p)
    return [pts[round(i * steps / (n - 1))] for i in range(n)]


def along(points, t):
    """📍 The point at the share t of the polyline's length."""
    lengths = [norm(sub(b, a)) for a, b in zip(points, points[1:])]
    target, run = t * sum(lengths), 0.0
    for (a, b), l in zip(zip(points, points[1:]), lengths):
        if run + l >= target - 1e-12:
            return lerp(a, b, 0 if l == 0 else (target - run) / l)
        run += l
    return points[-1]


def view_xy(view, rig, bone, p):
    """📐 The coordinates of the world point in the frame of a 2D shape of `bone`: elevation bone-local (u, v), front (lateral, height), plan (ahead, lateral) of the rest pose."""
    if view == "elevation":
        q = rig.point(bone, p)
        return (q[0], q[1])
    return (p[2], p[1]) if view == "front" else (p[0], p[2])


def simplify(points, tolerance):
    """✂️ Douglas-Peucker on an open polyline."""
    if len(points) < 3:
        return list(points)
    a, b = points[0], points[-1]
    dx, dy = b[0] - a[0], b[1] - a[1]
    l = math.hypot(dx, dy)
    worst, index = 0.0, 0
    for i in range(1, len(points) - 1):
        p = points[i]
        d = math.hypot(p[0] - a[0], p[1] - a[1]) if l == 0 else abs((p[0] - a[0]) * dy - (p[1] - a[1]) * dx) / l
        if d > worst:
            worst, index = d, i
    if worst <= tolerance:
        return [a, b]
    return simplify(points[: index + 1], tolerance)[:-1] + simplify(points[index:], tolerance)


def hull(points):
    pts = sorted(set(points))
    if len(pts) < 3:
        return pts

    def half(seq):
        out = []
        for p in seq:
            while len(out) >= 2 and (out[-1][0] - out[-2][0]) * (p[1] - out[-2][1]) - (out[-1][1] - out[-2][1]) * (p[0] - out[-2][0]) <= 0:
                out.pop()
            out.append(p)
        return out[:-1]

    return half(pts) + half(pts[::-1])
