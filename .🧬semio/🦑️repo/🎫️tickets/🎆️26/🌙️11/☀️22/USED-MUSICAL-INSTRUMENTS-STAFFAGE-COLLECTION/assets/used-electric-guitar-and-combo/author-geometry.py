"""🧮️ Geometry kit of the used electric guitar and combo asset: rig frames, solid sampling, hulls, polygon offsets, ear clipping and polyhedra.

Pure Python (no third-party packages): `author-guitar-combo.py` imports it and states every hand-chosen number.

@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — section "Rest placement of a bone", "Volume"
"""
import math


def add(a, b):
    """➕️ Sum of two 3-vectors."""
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def sub(a, b):
    """➖️ Difference of two 3-vectors."""
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def mul(a, s):
    """✖️ A 3-vector scaled."""
    return (a[0] * s, a[1] * s, a[2] * s)


def dot(a, b):
    """🔸️ Dot product."""
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    """❎️ Cross product of the numeric coordinates."""
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def length(a):
    """📏️ Euclidean length."""
    return math.sqrt(dot(a, a))


def unit(a):
    """🧭️ The vector scaled to length one."""
    n = length(a)
    return mul(a, 1.0 / n)


def apply(R, p):
    """🔄️ A local vector expressed in the frame whose columns are `R` (u, v, w)."""
    return add(add(mul(R[0], p[0]), mul(R[1], p[1])), mul(R[2], p[2]))


def apply_t(R, p):
    """↩️ A world vector expressed in the local frame `R`."""
    return (dot(R[0], p), dot(R[1], p), dot(R[2], p))


def compose(A, B):
    """🔗️ The frame `B` (given in `A`) expressed in the world."""
    return [apply(A, B[0]), apply(A, B[1]), apply(A, B[2])]


def rot_w(deg):
    """🌀️ Rotation about w: u towards v."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [(c, s, 0.0), (-s, c, 0.0), (0.0, 0.0, 1.0)]


def rot_v(deg):
    """↪️ Rotation about v: u towards w (abduction)."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [(c, 0.0, s), (0.0, 1.0, 0.0), (-s, 0.0, c)]


def rot_u(deg):
    """🔃️ Rotation about u: v to -w, w to v (twist)."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [(1.0, 0.0, 0.0), (0.0, c, -s), (0.0, s, c)]


def placement(angle, abduction=0.0, twist=0.0):
    """🦴️ R(angle, abduction, twist) = Rw · Rv · Ru."""
    return compose(compose(rot_w(angle), rot_v(abduction)), rot_u(twist))


IDENTITY = [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)]


def frames(bones, offsets=None):
    """🗺️ World origin and frame of every bone for pose offsets, parents listed before children (README "Rest placement of a bone")."""
    offsets = offsets or {}
    out = {}
    for bone in bones:
        off = offsets.get(bone["id"], {})
        joint = bone["joint"]
        dx = dy = 0.0
        oa = ob = ot = 0.0
        if joint["type"] == "prismatic":
            ax = math.radians(joint.get("axisDeg", 0.0))
            s = off.get("dx", 0.0) * math.cos(ax) + off.get("dy", 0.0) * math.sin(ax)
            s = max(joint.get("minM", -1e9), min(joint.get("maxM", 1e9), s))
            dx, dy = s * math.cos(ax), s * math.sin(ax)
        elif joint["type"] == "revolute":
            oa, ob, ot = off.get("angleDeg", 0.0), off.get("abductionDeg", 0.0), off.get("twistDeg", 0.0)
        if bone["parent"] is None:
            op, rp = (0.0, 0.0, 0.0), IDENTITY
        else:
            op, rp = out[bone["parent"]]
        o = add(op, apply(rp, (bone["x"] + dx, bone["y"] + dy, bone.get("z", 0.0))))
        r = compose(compose(rp, placement(bone["angleDeg"], bone.get("abductionDeg", 0.0), bone.get("twistDeg", 0.0))), placement(oa, ob, ot))
        out[bone["id"]] = (o, r)
    return out


def to_world(frame, p):
    """🌍️ A bone-local point in world metres."""
    return add(frame[0], apply(frame[1], p))


def to_local(frame, p):
    """🏠️ A world point in bone-local metres."""
    return apply_t(frame[1], sub(p, frame[0]))


def qrot(q, p):
    """🌐️ A point rotated by the unit quaternion [x, y, z, w]."""
    qv = (q[0], q[1], q[2])
    t = mul(cross(qv, p), 2.0)
    return add(add(p, mul(t, q[3])), cross(qv, t))


def axis_rot(axis, angle, p):
    """🎡️ Rodrigues rotation of `p` about the unit `axis` through the origin."""
    k = unit(axis)
    c, s = math.cos(angle), math.sin(angle)
    return add(add(mul(p, c), mul(cross(k, p), s)), mul(k, dot(k, p) * (1 - c)))


def sphere_dirs(n_lat=6, n_lon=12):
    """🔮️ Unit directions on a sphere, poles included."""
    out = [(0.0, 0.0, 1.0), (0.0, 0.0, -1.0)]
    for i in range(1, n_lat):
        th = math.pi * i / n_lat
        for j in range(n_lon):
            ph = 2 * math.pi * j / n_lon
            out.append((math.sin(th) * math.cos(ph), math.sin(th) * math.sin(ph), math.cos(th)))
    return out


DIRS = sphere_dirs()


def perpendicular_basis(axis):
    """📐️ Two unit vectors perpendicular to `axis` and to each other."""
    a = unit(axis)
    helper = (1.0, 0.0, 0.0) if abs(a[0]) < 0.9 else (0.0, 1.0, 0.0)
    e1 = unit(cross(a, helper))
    e2 = cross(a, e1)
    return a, e1, e2


def raw_points(s):
    """☁️ Sample points of a solid in its own coordinates (before `transform` and `repeat`)."""
    kind = s["kind"]
    if kind == "box":
        sx, sy, sz = (c / 2 for c in s["size"])
        r = min(s.get("cornerRadius", 0.0), sx, sy, sz)
        if r <= 0:
            return [(a * sx, b * sy, c * sz) for a in (-1, 1) for b in (-1, 1) for c in (-1, 1)]
        pts = []
        for a in (-1, 1):
            for b in (-1, 1):
                for c in (-1, 1):
                    centre = (a * (sx - r), b * (sy - r), c * (sz - r))
                    for d in DIRS:
                        if d[0] * a >= -1e-9 and d[1] * b >= -1e-9 and d[2] * c >= -1e-9:
                            pts.append(add(centre, mul(d, r)))
        return pts
    if kind == "ellipsoid":
        rx, ry, rz = s["radii"]
        return [(d[0] * rx, d[1] * ry, d[2] * rz) for d in DIRS]
    if kind == "capsule":
        p0, p1 = s["p0"], s["p1"]
        return [add(p0, mul(d, s["r0"])) for d in DIRS] + [add(p1, mul(d, s["r1"])) for d in DIRS]
    if kind == "revolve":
        a, e1, e2 = perpendicular_basis(s["axis"])
        o = s["origin"]
        sweep = s.get("sweepRad", 2 * math.pi)
        n = 32
        pts = []
        for ax, rad in s["profile"]:
            for k in range(n + (0 if sweep >= 2 * math.pi - 1e-9 else 1)):
                th = sweep * k / n
                pts.append(add(add(o, mul(a, ax)), add(mul(e1, rad * math.cos(th)), mul(e2, rad * math.sin(th)))))
        return pts
    if kind == "extrude":
        pts = []
        for a, b in s["profile"]:
            for t in (s["from"], s["to"]):
                if s["plane"] == "xy":
                    pts.append((a, b, t))
                elif s["plane"] == "yz":
                    pts.append((t, a, b))
                else:
                    pts.append((b, t, a))
        return pts
    if kind == "polyhedron":
        return [tuple(v) for v in s["vertices"]]
    raise ValueError(kind)


def placed_points(s):
    """📍️ Sample points of a solid in its bone's local frame, `transform` and every `repeat` copy applied."""
    base = raw_points(s)
    tr = s.get("transform", {})
    out = []
    for p in base:
        if "scale" in tr:
            p = (p[0] * tr["scale"][0], p[1] * tr["scale"][1], p[2] * tr["scale"][2])
        if "rotation" in tr:
            p = qrot(tr["rotation"], p)
        if "translation" in tr:
            p = add(p, tr["translation"])
        out.append(p)
    rep = s.get("repeat")
    if not rep:
        return out
    copies = []
    for k in range(rep["count"]):
        for p in out:
            q = p
            for _ in range(k):
                if rep.get("rotationRad"):
                    origin = rep.get("origin", (0.0, 0.0, 0.0))
                    q = add(axis_rot(rep["rotationAxis"], rep["rotationRad"], sub(q, origin)), origin)
                if rep.get("translation"):
                    q = add(q, rep["translation"])
            copies.append(q)
    return copies


def hull(points):
    """🪢️ Convex hull of 2D points (monotone chain), counter-clockwise, no repeated end point."""
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) < 3:
        return pts

    def turn(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and turn(lower[-2], lower[-1], p) <= 1e-12:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and turn(upper[-2], upper[-1], p) <= 1e-12:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def signed_area(ring):
    """🧮️ Signed area of a 2D ring (counter-clockwise positive)."""
    a = 0.0
    for i in range(len(ring)):
        x0, y0 = ring[i]
        x1, y1 = ring[(i + 1) % len(ring)]
        a += x0 * y1 - x1 * y0
    return a / 2


def ccw(ring):
    """🔁️ The ring in counter-clockwise order."""
    return list(ring) if signed_area(ring) > 0 else list(reversed(ring))


def catmull(points, per=4):
    """🪶️ A closed Catmull-Rom spline through the control points, `per` samples per span."""
    n = len(points)
    out = []
    for i in range(n):
        p0, p1, p2, p3 = points[i - 1], points[i], points[(i + 1) % n], points[(i + 2) % n]
        for k in range(per):
            t = k / per
            t2, t3 = t * t, t * t * t
            out.append(tuple(0.5 * ((2 * p1[j]) + (-p0[j] + p2[j]) * t + (2 * p0[j] - 5 * p1[j] + 4 * p2[j] - p3[j]) * t2 + (-p0[j] + 3 * p1[j] - 3 * p2[j] + p3[j]) * t3) for j in range(2)))
    return out


def inset(ring, d):
    """↘️ A counter-clockwise ring moved inwards by `d` along its vertex bisectors (smooth outlines only)."""
    n = len(ring)
    out = []
    for i in range(n):
        p_prev, p, p_next = ring[i - 1], ring[i], ring[(i + 1) % n]
        e0 = (p[0] - p_prev[0], p[1] - p_prev[1])
        e1 = (p_next[0] - p[0], p_next[1] - p[1])
        l0, l1 = math.hypot(*e0), math.hypot(*e1)
        n0 = (-e0[1] / l0, e0[0] / l0)
        n1 = (-e1[1] / l1, e1[0] / l1)
        b = (n0[0] + n1[0], n0[1] + n1[1])
        lb = math.hypot(*b)
        b = (b[0] / lb, b[1] / lb)
        cos_half = max(0.3, b[0] * n0[0] + b[1] * n0[1])
        out.append((p[0] + b[0] * d / cos_half, p[1] + b[1] * d / cos_half))
    return out


def ear_clip(ring):
    """✂️ Triangles (index triples, counter-clockwise) of a simple counter-clockwise polygon."""
    idx = list(range(len(ring)))
    tris = []

    def inside(p, a, b, c):
        d1 = (p[0] - b[0]) * (a[1] - b[1]) - (a[0] - b[0]) * (p[1] - b[1])
        d2 = (p[0] - c[0]) * (b[1] - c[1]) - (b[0] - c[0]) * (p[1] - c[1])
        d3 = (p[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (p[1] - a[1])
        neg = d1 < 0 or d2 < 0 or d3 < 0
        pos = d1 > 0 or d2 > 0 or d3 > 0
        return not (neg and pos)

    guard = 0
    while len(idx) > 3 and guard < 100000:
        guard += 1
        found = False
        for k in range(len(idx)):
            i0, i1, i2 = idx[k - 1], idx[k], idx[(k + 1) % len(idx)]
            a, b, c = ring[i0], ring[i1], ring[i2]
            if (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]) <= 1e-14:
                continue
            if any(inside(ring[j], a, b, c) for j in idx if j not in (i0, i1, i2)):
                continue
            tris.append((i0, i1, i2))
            idx.pop(k)
            found = True
            break
        if not found:
            raise ValueError("ear clipping failed")
    tris.append((idx[0], idx[1], idx[2]))
    return tris


def stacked_polyhedron(rings_w):
    """🧱️ A closed polyhedron from rings of equal vertex count stacked along w: `[(ring_uv, w), ...]` from the lowest w up; caps are ear-clipped."""
    n = len(rings_w[0][0])
    verts = []
    for ring, w in rings_w:
        verts.extend((p[0], p[1], w) for p in ring)
    tris = []
    for level in range(len(rings_w) - 1):
        a0, b0 = level * n, (level + 1) * n
        for i in range(n):
            j = (i + 1) % n
            tris.append((a0 + i, a0 + j, b0 + j))
            tris.append((a0 + i, b0 + j, b0 + i))
    bottom = ear_clip(rings_w[0][0])
    top = ear_clip(rings_w[-1][0])
    last = (len(rings_w) - 1) * n
    tris.extend((t[0], t[2], t[1]) for t in bottom)
    tris.extend((last + t[0], last + t[1], last + t[2]) for t in top)
    vol = 0.0
    for t in tris:
        vol += dot(verts[t[0]], cross(verts[t[1]], verts[t[2]]))
    if vol < 0:
        tris = [(t[0], t[2], t[1]) for t in tris]
    return [list(map(lambda x: round(x, 6), v)) for v in verts], [list(t) for t in tris]


def lofted_polyhedron(sections):
    """🎚️ A closed polyhedron through cross-sections of equal vertex count, each `(u, ring_vw)`, ordered along u; ends are ear-clipped."""
    n = len(sections[0][1])
    verts = []
    for u, ring in sections:
        verts.extend((u, p[0], p[1]) for p in ring)
    tris = []
    for k in range(len(sections) - 1):
        a0, b0 = k * n, (k + 1) * n
        for i in range(n):
            j = (i + 1) % n
            tris.append((a0 + i, a0 + j, b0 + j))
            tris.append((a0 + i, b0 + j, b0 + i))
    first = ear_clip(sections[0][1])
    last_ring = ear_clip(sections[-1][1])
    last = (len(sections) - 1) * n
    tris.extend((t[0], t[2], t[1]) for t in first)
    tris.extend((last + t[0], last + t[1], last + t[2]) for t in last_ring)
    vol = 0.0
    for t in tris:
        vol += dot(verts[t[0]], cross(verts[t[1]], verts[t[2]]))
    if vol < 0:
        tris = [(t[0], t[2], t[1]) for t in tris]
    return [list(map(lambda x: round(x, 6), v)) for v in verts], [list(t) for t in tris]


def r5(x):
    """🎯️ A coordinate rounded to 0.01 mm."""
    return round(x + 0.0, 5)


def pts2(ring):
    """📝️ A 2D ring rounded for the JSON document."""
    return [[r5(p[0]), r5(p[1])] for p in ring]


def path_d(rings):
    """〰️ An SVG path of closed rings (even-odd holes)."""
    parts = []
    for ring in rings:
        parts.append("M " + " L ".join(f"{r5(p[0])} {r5(p[1])}" for p in ring) + " Z")
    return " ".join(parts)
