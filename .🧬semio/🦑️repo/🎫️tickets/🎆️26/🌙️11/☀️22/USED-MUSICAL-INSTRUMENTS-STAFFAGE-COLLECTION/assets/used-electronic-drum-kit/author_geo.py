"""🥁 Geometry chisel of the used electronic drum kit: bones in their sagittal rest frames, parts authored in world rest metres, their solids in bone-local frames and their three orthographic 2D projections (side elevation in bone-local metres, front and plan in world rest metres).

@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — sections "Front and plan views", "Solid placements", "Volume"
"""
import math

TAU = 2 * math.pi


def r6(v):
    """🔢 Rounds a number or nested list to 0.1 mm precision for the document."""
    if isinstance(v, (list, tuple)):
        return [r6(x) for x in v]
    return round(float(v), 5) + 0.0


class Bone:
    """🦴 A sagittal bone: world rest origin and world angle follow from its parent; `z` is its lateral offset along the parent's w."""

    def __init__(self, bid, parent, x, y, angle, length, joint, title, z=0.0, mass=None, section=None):
        self.id, self.parent, self.x, self.y, self.z, self.angle, self.length, self.joint, self.title = bid, parent, x, y, z, angle, length, joint, title
        self.mass, self.section = mass, section
        self.o = (0.0, 0.0, 0.0)
        self.th = 0.0

    def place(self, bones):
        if self.parent is None:
            self.o, self.th = (0.0, 0.0, self.z), math.radians(self.angle)
            return
        p = bones[self.parent]
        c, s = math.cos(p.th), math.sin(p.th)
        self.o = (p.o[0] + c * self.x - s * self.y, p.o[1] + s * self.x + c * self.y, p.o[2] + self.z)
        self.th = p.th + math.radians(self.angle)

    def local(self, q):
        c, s = math.cos(self.th), math.sin(self.th)
        dx, dy = q[0] - self.o[0], q[1] - self.o[1]
        return (c * dx + s * dy, -s * dx + c * dy, q[2] - self.o[2])

    def ldir(self, d):
        c, s = math.cos(self.th), math.sin(self.th)
        return (c * d[0] + s * d[1], -s * d[0] + c * d[1], d[2])

    def world(self, l):
        c, s = math.cos(self.th), math.sin(self.th)
        return (self.o[0] + c * l[0] - s * l[1], self.o[1] + s * l[0] + c * l[1], self.o[2] + l[2])


def hull(points):
    """🧮 Convex hull (monotone chain), counter-clockwise, without collinear points."""
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) < 3:
        return pts
    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 1e-12:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 1e-12:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def ring_area(ring):
    """📐 Signed area of a ring."""
    return 0.5 * sum(ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1] for i in range(len(ring)))


def norm3(v):
    n = math.sqrt(sum(x * x for x in v))
    return tuple(x / n for x in v)


def cross3(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def add3(a, b, k=1.0):
    return (a[0] + k * b[0], a[1] + k * b[1], a[2] + k * b[2])


def frame_of(n):
    """🧭 Two unit vectors perpendicular to the axis `n`."""
    e1 = (0.0, 0.0, 1.0) if abs(n[2]) < 0.9 else (1.0, 0.0, 0.0)
    e1 = norm3(cross3(cross3(n, e1), n))
    e2 = norm3(cross3(n, e1))
    return e1, e2


class Part:
    """🧩 One solid of one scale: its document solid in bone-local metres, a cloud of world rest points for its hull projections, and how it is drawn."""

    def __init__(self, pid, bone, material, region, role, tone, solid, pts, draw=None, flags=None, mass=0.0, com=None):
        self.id, self.bone, self.material, self.region, self.role, self.tone = pid, bone, material, region, role, tone
        self.solid, self.pts, self.draw, self.flags, self.mass, self.com = solid, pts, draw or {}, flags or {}, mass, com


def capsule(B, pid, bone, p0, p1, r, material, region, role, tone, **kw):
    """💊 A tube or rod between two world points: capsule solid, exact capsule projections."""
    b = B[bone]
    l0, l1 = b.local(p0), b.local(p1)
    solid = {"kind": "capsule", "p0": r6(l0), "p1": r6(l1), "r0": r6(r), "r1": r6(r)}
    pts = []
    for p in (p0, p1):
        for ax in ((1, 0, 0), (0, 1, 0), (0, 0, 1)):
            pts.append(add3(p, ax, r))
            pts.append(add3(p, ax, -r))
    draw = {"capsule": (p0, p1, r)}
    draw.update(kw.pop("draw", {}))
    return Part(pid, bone, material, region, role, tone, solid, pts, draw, kw.pop("flags", None), kw.pop("mass", 0.0), kw.pop("com", None))


def revolve(B, pid, bone, base, n, profile, material, region, role, tone, closed=False, **kw):
    """🌀 A turned part: profile [axial, radial] about the world axis `n` through the world point `base`."""
    b = B[bone]
    n = norm3(n)
    solid = {"kind": "revolve", "profile": r6(profile), "axis": r6(b.ldir(n)), "origin": r6(b.local(base))}
    e1, e2 = frame_of(n)
    pts = []
    for a, rr in profile:
        c = add3(base, n, a)
        for i in range(48):
            t = TAU * i / 48
            radial = add3(tuple(x * math.cos(t) for x in e1), e2, math.sin(t))
            pts.append(add3(c, radial, rr))
    return Part(pid, bone, material, region, role, tone, solid, pts, kw.pop("draw", {}), kw.pop("flags", None), kw.pop("mass", 0.0), kw.pop("com", None))


def box(B, pid, bone, c, size, material, region, role, tone, angle=0.0, corner=0.0, **kw):
    """📦 A box centred on world `c`, its first edge turned by `angle` degrees about the lateral axis in the world."""
    b = B[bone]
    la = math.radians(angle) - b.th
    lc = b.local(c)
    solid = {"kind": "box", "size": r6(size)}
    if corner > 0:
        solid["cornerRadius"] = r6(corner)
    tr = {"translation": r6(lc)}
    if abs(math.sin(la / 2)) > 1e-9:
        tr["rotation"] = [0.0, 0.0, r6(math.sin(la / 2)), r6(math.cos(la / 2))]
    solid["transform"] = tr
    ca, sa = math.cos(math.radians(angle)), math.sin(math.radians(angle))
    pts = []
    for i in (-0.5, 0.5):
        for j in (-0.5, 0.5):
            for k in (-0.5, 0.5):
                du, dv, dw = i * size[0], j * size[1], k * size[2]
                pts.append((c[0] + ca * du - sa * dv, c[1] + sa * du + ca * dv, c[2] + dw))
    return Part(pid, bone, material, region, role, tone, solid, pts, kw.pop("draw", {}), kw.pop("flags", None), kw.pop("mass", 0.0), kw.pop("com", None))


def extrude(B, pid, bone, profile_local, w0, w1, material, region, role, tone, **kw):
    """🪚 A prism of a bone-local (u, v) profile from lateral w0 to w1 (bone-local w)."""
    b = B[bone]
    solid = {"kind": "extrude", "profile": r6(profile_local), "plane": "xy", "from": r6(w0), "to": r6(w1)}
    pts = [b.world((u, v, w)) for (u, v) in profile_local for w in (w0, w1)]
    return Part(pid, bone, material, region, role, tone, solid, pts, kw.pop("draw", {}), kw.pop("flags", None), kw.pop("mass", 0.0), kw.pop("com", None))


def ellipsoid(B, pid, bone, c, radii, material, region, role, tone, **kw):
    """🥚 An ellipsoid centred on world `c` with bone-axis radii."""
    b = B[bone]
    solid = {"kind": "ellipsoid", "radii": r6(radii), "transform": {"translation": r6(b.local(c))}}
    pts = []
    for i in range(24):
        t = TAU * i / 24
        pts.append(b.world(add3(b.local(c), (radii[0] * math.cos(t), radii[1] * math.sin(t), 0))))
        pts.append(b.world(add3(b.local(c), (radii[0] * math.cos(t), 0, radii[2] * math.sin(t)))))
        pts.append(b.world(add3(b.local(c), (0, radii[1] * math.cos(t), radii[2] * math.sin(t)))))
    return Part(pid, bone, material, region, role, tone, solid, pts, kw.pop("draw", {}), kw.pop("flags", None), kw.pop("mass", 0.0), kw.pop("com", None))


def project(view, q):
    """🔭 Paper coordinates of a world point: elevation (x, y), front (z, y), plan (x, z)."""
    if view == "elevation":
        return (q[0], q[1])
    if view == "front":
        return (q[2], q[1])
    return (q[0], q[2])


def depth(view, q):
    """🔭 How near a world point is to the viewer of a view (larger is nearer)."""
    return -q[2] if view == "elevation" else (q[0] if view == "front" else q[1])
