"""🧮 Geometry kit of the used marching drums: rig frames, world-to-bone conversion, the solid primitives of the asset and their exact projections into the three authored views.

Every primitive is stated once in world rest coordinates (x ahead, y up, z left, metres) and yields both its kernel solid (bone-local u, v, w) and its outline in the side elevation (x, y), the front elevation (z, y) and the plan (x, z), so 2D and 3D come from one set of numbers.

@see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — sections "Volume" and "Front and plan views"
"""
import math

SEGMENTS = 64


def q_axis(axis, deg):
    """🌀 A unit quaternion [x, y, z, w] turning `deg` degrees about `axis`."""
    n = math.sqrt(sum(a * a for a in axis))
    s = math.sin(math.radians(deg) / 2) / n
    return [axis[0] * s, axis[1] * s, axis[2] * s, math.cos(math.radians(deg) / 2)]


def q_mul(a, b):
    """✖️ The Hamilton product a·b of two quaternions [x, y, z, w]."""
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def q_rot(q, v):
    """🔄 The vector `v` turned by the quaternion `q`."""
    x, y, z, w = q
    p = [v[0], v[1], v[2], 0.0]
    r = q_mul(q_mul(q, p), [-x, -y, -z, w])
    return [r[0], r[1], r[2]]


def q_from_columns(ex, ey, ez):
    """🧭 The quaternion of the rotation matrix whose columns are the orthonormal axes ex, ey, ez."""
    m00, m01, m02 = ex[0], ey[0], ez[0]
    m10, m11, m12 = ex[1], ey[1], ez[1]
    m20, m21, m22 = ex[2], ey[2], ez[2]
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        return [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s]
    if m00 > m11 and m00 > m22:
        s = math.sqrt(1.0 + m00 - m11 - m22) * 2
        return [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    if m11 > m22:
        s = math.sqrt(1.0 + m11 - m00 - m22) * 2
        return [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s]
    s = math.sqrt(1.0 + m22 - m00 - m11) * 2
    return [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s]


def r6(v):
    """✂️ A number rounded to a tenth of a millimetre-micrometre grid (6 decimals)."""
    return round(v + 0.0, 6)


class Frame:
    """🦴 The world rest frame of a bone: origin (x, y, z) and the angle of its u axis in the x-y plane."""

    def __init__(self, x, y, z, deg):
        self.o = (x, y, z)
        self.deg = deg
        c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
        self.u = (c, s, 0.0)
        self.v = (-s, c, 0.0)

    def local(self, p):
        """📍 The bone-local (u, v, w) of the world point p."""
        d = (p[0] - self.o[0], p[1] - self.o[1])
        return [r6(d[0] * self.u[0] + d[1] * self.u[1]), r6(d[0] * self.v[0] + d[1] * self.v[1]), r6(p[2] - self.o[2])]

    def local_dir(self, d):
        """➡️ The bone-local direction of the world direction d."""
        return [r6(d[0] * self.u[0] + d[1] * self.u[1]), r6(d[0] * self.v[0] + d[1] * self.v[1]), r6(d[2])]

    def local_q(self, q_world):
        """🔁 The bone-local rotation of a world rotation."""
        return [r6(c) for c in q_mul(q_axis((0, 0, 1), -self.deg), q_world)]

    def elev(self, x, y):
        """📐 The bone-local (u, v) of a side-elevation point (world x, y)."""
        d = (x - self.o[0], y - self.o[1])
        return [r6(d[0] * self.u[0] + d[1] * self.u[1]), r6(d[0] * self.v[0] + d[1] * self.v[1])]


AXES = {"x": (1.0, 0.0, 0.0), "y": (0.0, 1.0, 0.0), "z": (0.0, 0.0, 1.0)}


def circle_pts(cx, cy, r, n=SEGMENTS, a0=0.0, a1=360.0, closed=True):
    """⭕ Points of a circle or arc about (cx, cy)."""
    k = n if closed else n + 1
    return [[r6(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n))), r6(cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n)))] for i in range(k)]


def hull(points):
    """🫧 The convex hull of 2D points, counter-clockwise (monotone chain)."""
    pts = sorted(set((round(p[0], 7), round(p[1], 7)) for p in points))
    if len(pts) <= 2:
        return [list(p) for p in pts]

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
    return [[r6(a), r6(b)] for a, b in lower[:-1] + upper[:-1]]


def view_of(p, view):
    """👁 The paper coordinates of a world point in a view: elevation (x, y), front (z, y), plan (x, z)."""
    if view == "elevation":
        return (p[0], p[1])
    if view == "front":
        return (p[2], p[1])
    return (p[0], p[2])


def nearness(points3, view):
    """🔭 How near the nearest point is to the viewer: elevation from the right (-z), front from ahead (+x), plan from above (+y)."""
    if view == "elevation":
        return max(-p[2] for p in points3)
    if view == "front":
        return max(p[0] for p in points3)
    return max(p[1] for p in points3)


def offset_path(path, r):
    """🧵 The outline of a planar polyline swept by a disc of radius r (round caps, mitred joints for gentle bends)."""
    left, right = [], []
    n = len(path)
    for i in range(n):
        if i == 0:
            d = (path[1][0] - path[0][0], path[1][1] - path[0][1])
        elif i == n - 1:
            d = (path[-1][0] - path[-2][0], path[-1][1] - path[-2][1])
        else:
            a = (path[i][0] - path[i - 1][0], path[i][1] - path[i - 1][1])
            b = (path[i + 1][0] - path[i][0], path[i + 1][1] - path[i][1])
            la, lb = math.hypot(*a), math.hypot(*b)
            d = (a[0] / la + b[0] / lb, a[1] / la + b[1] / lb)
        ld = math.hypot(*d)
        nx, ny = -d[1] / ld, d[0] / ld
        scale = 1.0
        if 0 < i < n - 1:
            a = (path[i][0] - path[i - 1][0], path[i][1] - path[i - 1][1])
            la = math.hypot(*a)
            nax, nay = -a[1] / la, a[0] / la
            scale = 1.0 / max(0.5, nx * nax + ny * nay)
        left.append([path[i][0] + nx * r * scale, path[i][1] + ny * r * scale])
        right.append([path[i][0] - nx * r * scale, path[i][1] - ny * r * scale])
    d0 = (path[1][0] - path[0][0], path[1][1] - path[0][1])
    d1 = (path[-1][0] - path[-2][0], path[-1][1] - path[-2][1])
    a_end = math.degrees(math.atan2(d1[1], d1[0]))
    a_start = math.degrees(math.atan2(d0[1], d0[0]))
    end_cap = circle_pts(path[-1][0], path[-1][1], r, 12, a_end + 90, a_end - 90, closed=False)[1:-1]
    start_cap = circle_pts(path[0][0], path[0][1], r, 12, a_start - 90, a_start - 270, closed=False)[1:-1]
    ring = left + end_cap + list(reversed(right)) + start_cap
    return [[r6(a), r6(b)] for a, b in ring]


def arc_path(cx, cy, r, a0, a1, n):
    """🌈 Points of an arc from a0 to a1 degrees about (cx, cy)."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


class Prim:
    """🧱 A primitive of the asset in world rest coordinates; subclasses give its solid, its volume, its centroid and its outline per view."""

    texture = False

    def bounds3(self):
        return self.points3()


class Ring(Prim):
    """🥁 A hollow (r_in > 0) or solid cylinder about the world axis `axis` ('y' or 'z') through `c`, from axial a0 to a1; `outline` optionally replaces the side rectangle by a stepped profile [(axial, radius)] closed loop."""

    def __init__(self, axis, c, a0, a1, r_in, r_out, profile=None):
        self.axis, self.c, self.a0, self.a1, self.r_in, self.r_out, self.profile = axis, c, a0, a1, r_in, r_out, profile

    def at(self, a, r, ang):
        cx, cy, cz = self.c
        if self.axis == "y":
            return (cx + r * math.cos(ang), cy + a, cz + r * math.sin(ang))
        return (cx + r * math.cos(ang), cy + r * math.sin(ang), cz + a)

    def points3(self):
        return [self.at(a, self.r_out, math.radians(t)) for a in (self.a0, self.a1) for t in range(0, 360, 15)]

    def solid(self, frame):
        loop = self.loop()
        if self.profile and self.r_in == 0:
            prof = [[r6(a), r6(r)] for a, r in [(self.profile[0][0], 0.0)] + list(self.profile) + [(self.profile[-1][0], 0.0)]]
        elif self.r_in > 0 or self.profile:
            prof = [[r6(a), r6(r)] for a, r in loop] + [[r6(loop[0][0]), r6(loop[0][1])]]
        else:
            prof = [[r6(self.a0), 0.0], [r6(self.a0), r6(self.r_out)], [r6(self.a1), r6(self.r_out)], [r6(self.a1), 0.0]]
        return {"kind": "revolve", "profile": prof, "axis": frame.local_dir(AXES[self.axis]), "origin": frame.local(self.c)}

    def loop(self):
        """🔁 The closed profile loop [(axial, radius)] of the ring (with the axis for a stepped solid)."""
        if self.profile and self.r_in == 0:
            return [(self.profile[0][0], 0.0)] + list(self.profile) + [(self.profile[-1][0], 0.0)]
        return list(self.profile or [(self.a0, self.r_in), (self.a1, self.r_in), (self.a1, self.r_out), (self.a0, self.r_out)])

    def volume(self):
        loop = self.loop()
        area, moment = 0.0, 0.0
        for i in range(len(loop)):
            a1, r1 = loop[i]
            a2, r2 = loop[(i + 1) % len(loop)]
            cr = a1 * r2 - a2 * r1
            area += cr
            moment += (r1 + r2) * cr
        return abs(math.pi * moment / 3.0)

    def centroid(self):
        a = (self.a0 + self.a1) / 2
        cx, cy, cz = self.c
        return (cx, cy + a, cz) if self.axis == "y" else (cx, cy, cz + a)

    def side_rings(self, view):
        cx, cy, cz = self.c
        if self.profile:
            right = self.side_profile()
            outline = [(r, a) for a, r in right] + [(-r, a) for a, r in reversed(right)]
        else:
            outline = [(self.r_out, self.a0), (self.r_out, self.a1), (-self.r_out, self.a1), (-self.r_out, self.a0)]
        pts = []
        for s, a in outline:
            if self.axis == "y":
                p = (cx + s, cy + a, cz) if view == "elevation" else (cx, cy + a, cz + s)
            else:
                p = (cx + s, cy, cz + a) if view == "plan" else (cx, cy + s, cz + a)
            pts.append([r6(c) for c in view_of(p, view)])
        return [pts]

    def side_profile(self):
        """🪜 The outer radius along the axis as a staircase [(axial, radius)] from a0 to a1 (the silhouette of a stepped ring seen from the side)."""
        cuts = sorted(set(a for a, _ in self.loop()))
        spans = []
        for i in range(len(cuts) - 1):
            m = (cuts[i] + cuts[i + 1]) / 2
            rmax = 0.0
            loop = self.loop()
            for j in range(len(loop)):
                a1, r1 = loop[j]
                a2, r2 = loop[(j + 1) % len(loop)]
                if (a1 - m) * (a2 - m) < 0:
                    rr = r1 + (r2 - r1) * (m - a1) / (a2 - a1)
                    rmax = max(rmax, rr)
            spans.append((cuts[i], cuts[i + 1], rmax))
        out = []
        for a0, a1, r in spans:
            out.append((a0, r))
            out.append((a1, r))
        return out

    def outline(self, view):
        """✏️ The outline rings in paper coordinates; the second element tells whether rings after the first are holes."""
        along = (self.axis == "y" and view == "plan") or (self.axis == "z" and view == "elevation")
        if along:
            cx, cy, cz = self.c
            c2 = (cx, cz) if view == "plan" else (cx, cy)
            outer = circle_pts(c2[0], c2[1], self.r_out)
            rings = [outer]
            if self.r_in > 0:
                rings.append(list(reversed(circle_pts(c2[0], c2[1], self.r_in))))
            return rings
        return self.side_rings(view)


class Box(Prim):
    """📦 A box of `size` (along its own axes) centred at `c`, turned by the world quaternion `q`."""

    def __init__(self, c, size, q=(0.0, 0.0, 0.0, 1.0)):
        self.c, self.size, self.q = c, size, list(q)

    def points3(self):
        out = []
        for sx in (-0.5, 0.5):
            for sy in (-0.5, 0.5):
                for sz in (-0.5, 0.5):
                    d = q_rot(self.q, (sx * self.size[0], sy * self.size[1], sz * self.size[2]))
                    out.append((self.c[0] + d[0], self.c[1] + d[1], self.c[2] + d[2]))
        return out

    def solid(self, frame):
        s = {"kind": "box", "size": [r6(v) for v in self.size]}
        t = {"translation": frame.local(self.c)}
        ql = frame.local_q(self.q)
        if max(abs(ql[0]), abs(ql[1]), abs(ql[2])) > 1e-9:
            t["rotation"] = ql
        s["transform"] = t
        return s

    def volume(self):
        return self.size[0] * self.size[1] * self.size[2]

    def centroid(self):
        return tuple(self.c)

    def outline(self, view):
        return [hull([view_of(p, view) for p in self.points3()])]


class Capsule(Prim):
    """💊 A sphere-swept segment from p0 (radius r0) to p1 (radius r1)."""

    def __init__(self, p0, p1, r0, r1=None):
        self.p0, self.p1, self.r0, self.r1 = p0, p1, r0, (r0 if r1 is None else r1)

    def points3(self):
        out = []
        for p, r in ((self.p0, self.r0), (self.p1, self.r1)):
            for d in ((r, 0, 0), (-r, 0, 0), (0, r, 0), (0, -r, 0), (0, 0, r), (0, 0, -r)):
                out.append((p[0] + d[0], p[1] + d[1], p[2] + d[2]))
        return out

    def solid(self, frame):
        return {"kind": "capsule", "p0": frame.local(self.p0), "p1": frame.local(self.p1), "r0": r6(self.r0), "r1": r6(self.r1)}

    def length(self):
        return math.dist(self.p0, self.p1)

    def volume(self):
        h = self.length()
        return math.pi * h * (self.r0 ** 2 + self.r0 * self.r1 + self.r1 ** 2) / 3 + 2 * math.pi * (self.r0 ** 3 + self.r1 ** 3) / 3

    def centroid(self):
        return tuple((a + b) / 2 for a, b in zip(self.p0, self.p1))

    def outline(self, view):
        a, b = view_of(self.p0, view), view_of(self.p1, view)
        if math.dist(a, b) < 1e-6:
            return [circle_pts(a[0], a[1], max(self.r0, self.r1), 32)]
        return [("capsule", [r6(a[0]), r6(a[1]), r6(b[0]), r6(b[1]), r6(self.r0), r6(self.r1)])]


class Tube(Prim):
    """🪈 A round bar of radius r along a polyline at constant world z (the carrier bars, the stand arms at coarse scales): capsule chain or loft."""

    def __init__(self, path, z, r, loft=False):
        self.path, self.z, self.r, self.loft = [tuple(p) for p in path], z, r, loft

    def points3(self):
        return [(p[0] + dx, p[1] + dy, self.z + dz) for p in self.path for dx, dy, dz in ((self.r, 0, 0), (-self.r, 0, 0), (0, self.r, 0), (0, -self.r, 0), (0, 0, self.r), (0, 0, -self.r))]

    def length(self):
        return sum(math.dist(self.path[i], self.path[i + 1]) for i in range(len(self.path) - 1))

    def segments(self):
        return [Capsule((self.path[i][0], self.path[i][1], self.z), (self.path[i + 1][0], self.path[i + 1][1], self.z), self.r) for i in range(len(self.path) - 1)]

    def loft_solid(self, frame):
        sections = []
        for i, p in enumerate(self.path):
            if i == 0:
                d = (self.path[1][0] - p[0], self.path[1][1] - p[1])
            elif i == len(self.path) - 1:
                d = (p[0] - self.path[i - 1][0], p[1] - self.path[i - 1][1])
            else:
                d = (self.path[i + 1][0] - self.path[i - 1][0], self.path[i + 1][1] - self.path[i - 1][1])
            ld = math.hypot(*d)
            t = frame.local_dir((d[0] / ld, d[1] / ld, 0.0))
            ex = [0.0, 0.0, 1.0]
            ey = [t[1] * ex[2] - t[2] * ex[1], t[2] * ex[0] - t[0] * ex[2], t[0] * ex[1] - t[1] * ex[0]]
            q = q_from_columns(ex, ey, t)
            sections.append({"at": frame.local((p[0], p[1], self.z)), "rx": r6(self.r), "ry": r6(self.r), "rotation": [r6(c) for c in q]})
        return {"kind": "loft", "sections": sections, "capStart": True, "capEnd": True}

    def volume(self):
        return math.pi * self.r ** 2 * self.length()

    def centroid(self):
        tot, acc = 0.0, [0.0, 0.0]
        for i in range(len(self.path) - 1):
            l = math.dist(self.path[i], self.path[i + 1])
            tot += l
            acc[0] += l * (self.path[i][0] + self.path[i + 1][0]) / 2
            acc[1] += l * (self.path[i][1] + self.path[i + 1][1]) / 2
        return (acc[0] / tot, acc[1] / tot, self.z)

    def outline(self, view):
        if view == "elevation":
            return [offset_path(self.path, self.r)]
        xs = [p[0] for p in self.path]
        ys = [p[1] for p in self.path]
        if view == "front":
            return [("capsule", [r6(self.z), r6(min(ys)), r6(self.z), r6(max(ys)), r6(self.r), r6(self.r)])]
        return [("capsule", [r6(min(xs)), r6(self.z), r6(max(xs)), r6(self.z), r6(self.r), r6(self.r)])]


class Prism(Prim):
    """🧊 A profile in the world x-y plane pushed from z0 to z1 (an extrude along w)."""

    def __init__(self, profile, z0, z1):
        self.profile, self.z0, self.z1 = [tuple(p) for p in profile], z0, z1

    def points3(self):
        return [(p[0], p[1], z) for p in self.profile for z in (self.z0, self.z1)]

    def solid(self, frame):
        prof = [frame.elev(x, y) for x, y in self.profile]
        a = sum(prof[i][0] * prof[(i + 1) % len(prof)][1] - prof[(i + 1) % len(prof)][0] * prof[i][1] for i in range(len(prof)))
        if a < 0:
            prof = list(reversed(prof))
        return {"kind": "extrude", "profile": prof, "plane": "xy", "from": r6(self.z0 - frame.o[2]), "to": r6(self.z1 - frame.o[2])}

    def area(self):
        p = self.profile
        return abs(sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))) / 2

    def volume(self):
        return self.area() * (self.z1 - self.z0)

    def centroid(self):
        p = self.profile
        a = cx = cy = 0.0
        for i in range(len(p)):
            x0, y0 = p[i]
            x1, y1 = p[(i + 1) % len(p)]
            c = x0 * y1 - x1 * y0
            a += c
            cx += (x0 + x1) * c
            cy += (y0 + y1) * c
        return (cx / (3 * a), cy / (3 * a), (self.z0 + self.z1) / 2)

    def outline(self, view):
        if view == "elevation":
            return [[[r6(x), r6(y)] for x, y in self.profile]]
        xs = [p[0] for p in self.profile]
        ys = [p[1] for p in self.profile]
        if view == "front":
            return [[[r6(self.z0), r6(min(ys))], [r6(self.z1), r6(min(ys))], [r6(self.z1), r6(max(ys))], [r6(self.z0), r6(max(ys))]]]
        return [[[r6(min(xs)), r6(self.z0)], [r6(max(xs)), r6(self.z0)], [r6(max(xs)), r6(self.z1)], [r6(min(xs)), r6(self.z1)]]]


class Ellipsoid(Prim):
    """🥚 An ellipsoid with semi-axes along the world axes, centred at c."""

    def __init__(self, c, radii):
        self.c, self.radii = c, radii

    def points3(self):
        return [(self.c[0] + sx * self.radii[0], self.c[1] + sy * self.radii[1], self.c[2] + sz * self.radii[2]) for sx, sy, sz in ((1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1))]

    def solid(self, frame):
        s = {"kind": "ellipsoid", "radii": [r6(v) for v in self.radii], "transform": {"translation": frame.local(self.c)}}
        ql = frame.local_q((0, 0, 0, 1))
        if max(abs(ql[0]), abs(ql[1]), abs(ql[2])) > 1e-9:
            s["transform"]["rotation"] = ql
        return s

    def volume(self):
        return 4 / 3 * math.pi * self.radii[0] * self.radii[1] * self.radii[2]

    def centroid(self):
        return tuple(self.c)

    def outline(self, view):
        a = view_of(self.c, view)
        r = {"elevation": (0, 1), "front": (2, 1), "plan": (0, 2)}[view]
        return [("ellipse", [r6(a[0]), r6(a[1]), r6(self.radii[r[0]]), r6(self.radii[r[1]])])]
