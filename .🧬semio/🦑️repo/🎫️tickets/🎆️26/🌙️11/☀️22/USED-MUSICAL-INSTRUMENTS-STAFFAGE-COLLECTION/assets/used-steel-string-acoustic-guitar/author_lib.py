"""🧰 Geometry, mesh and rig helpers for the used steel-string guitar author script: frames as the product composes them, polygons, ear clipping and closed polyhedra."""
import math

R6 = lambda x: round(x, 6)


def rw(a):
    """🔄 Rotation about the joint axis w: u towards v."""
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def rv(b):
    """↪️ Rotation about the normal v: u towards w."""
    c, s = math.cos(math.radians(b)), math.sin(math.radians(b))
    return [[c, 0, -s], [0, 1, 0], [s, 0, c]]


def ru(t):
    """🌀 Rotation about the axis u: v to -w, w to v."""
    c, s = math.cos(math.radians(t)), math.sin(math.radians(t))
    return [[1, 0, 0], [0, c, s], [0, -s, c]]


def mm(a, b):
    """✖️ Matrix product."""
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def mv(a, v):
    """➡️ Matrix times vector."""
    return [sum(a[i][k] * v[k] for k in range(3)) for i in range(3)]


def tr(a):
    """🔁 Transpose."""
    return [[a[j][i] for j in range(3)] for i in range(3)]


def rot(a=0, b=0, t=0):
    """🧭 Rw(a)·Rv(b)·Ru(t)."""
    return mm(mm(rw(a), rv(b)), ru(t))


def decompose(m):
    """🧩 Angles (a, b, t) with rot(a, b, t) == m."""
    b = math.degrees(math.asin(max(-1, min(1, m[2][0]))))
    a = math.degrees(math.atan2(m[1][0], m[0][0]))
    r = mm(tr(mm(rw(a), rv(b))), m)
    t = math.degrees(math.atan2(r[1][2], r[1][1]))
    back = rot(a, b, t)
    assert max(abs(back[i][j] - m[i][j]) for i in range(3) for j in range(3)) < 1e-6, (m, back)
    return a, b, t


def add(a, b):
    return [a[i] + b[i] for i in range(3)]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def scl(a, k):
    return [x * k for x in a]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def norm(a):
    n = math.sqrt(dot(a, a))
    return [x / n for x in a]


def frame_cols(u, v, w):
    """🧱 Matrix whose columns are u, v, w."""
    return [[u[i], v[i], w[i]] for i in range(3)]


def solve_rig(bones, offsets=None, root=None):
    """🦴 World frames {id: (R, o)} of a rig as the product composes them (root placement adds x, y and turns about z)."""
    offsets = offsets or {}
    root = root or {}
    out = {}
    by = {b["id"]: b for b in bones}

    def go(b):
        if b["id"] in out:
            return out[b["id"]]
        off = offsets.get(b["id"], {})
        rest = rot(b.get("angleDeg", 0), b.get("abductionDeg", 0), b.get("twistDeg", 0))
        o_rot = rot(off.get("angleDeg", 0), off.get("abductionDeg", 0), off.get("twistDeg", 0))
        if b["parent"] is None:
            r = mm(mm(rw(root.get("angleDeg", 0)), rest), o_rot)
            o = [b["x"] + root.get("x", 0), b["y"] + root.get("y", 0), b.get("z", 0)]
        else:
            pr, po = go(by[b["parent"]])
            r = mm(mm(pr, rest), o_rot)
            o = add(po, mv(pr, [b["x"] + off.get("dx", 0), b["y"] + off.get("dy", 0), b.get("z", 0)]))
        out[b["id"]] = (r, o)
        return out[b["id"]]

    for b in bones:
        go(b)
    return out


def world(frames, bone, p):
    """🌍 World position of a bone-local point."""
    r, o = frames[bone]
    return add(o, mv(r, p))


def rounded_rect(cx, cy, hx, hy, r, n=5):
    """⬜ Counter-clockwise rounded rectangle."""
    pts = []
    for (sx, sy, a0) in [(1, 1, 0), (-1, 1, 90), (-1, -1, 180), (1, -1, 270)]:
        ccx, ccy = cx + sx * (hx - r), cy + sy * (hy - r)
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            pts.append((R6(ccx + r * math.cos(a)), R6(ccy + r * math.sin(a))))
    return pts


def rect(x0, y0, x1, y1):
    """▭ Counter-clockwise rectangle."""
    return [(R6(x0), R6(y0)), (R6(x1), R6(y0)), (R6(x1), R6(y1)), (R6(x0), R6(y1))]


def circle(cx, cy, r, n=24, rx=None):
    """⭕ Counter-clockwise circle or ellipse."""
    rx = r if rx is None else rx
    return [(R6(cx + rx * math.cos(2 * math.pi * k / n)), R6(cy + r * math.sin(2 * math.pi * k / n))) for k in range(n)]


def area2(p):
    return 0.5 * sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))


def ccw(p):
    """🔃 The polygon in counter-clockwise order."""
    return list(p) if area2(p) > 0 else list(reversed(p))


def catmull(points, closed=True, per=6):
    """〰️ Catmull-Rom resampling through control points."""
    n = len(points)
    out = []
    rng = range(n) if closed else range(n - 1)
    for i in rng:
        p0 = points[(i - 1) % n] if closed or i > 0 else points[0]
        p1 = points[i]
        p2 = points[(i + 1) % n]
        p3 = points[(i + 2) % n] if closed or i + 2 < n else points[-1]
        for k in range(per):
            t = k / per
            t2, t3 = t * t, t * t * t
            out.append(tuple(0.5 * (2 * p1[j] + (-p0[j] + p2[j]) * t + (2 * p0[j] - 5 * p1[j] + 4 * p2[j] - p3[j]) * t2 + (-p0[j] + 3 * p1[j] - 3 * p2[j] + p3[j]) * t3) for j in range(2)))
    if not closed:
        out.append(points[-1])
    return out


def inset(poly, d):
    """↘️ Offset a counter-clockwise polygon inwards by d along averaged vertex normals."""
    n = len(poly)
    out = []
    for i in range(n):
        a, b, c = poly[i - 1], poly[i], poly[(i + 1) % n]
        e1 = (b[0] - a[0], b[1] - a[1])
        e2 = (c[0] - b[0], c[1] - b[1])
        l1, l2 = math.hypot(*e1) or 1, math.hypot(*e2) or 1
        n1 = (-e1[1] / l1, e1[0] / l1)
        n2 = (-e2[1] / l2, e2[0] / l2)
        m = (n1[0] + n2[0], n1[1] + n2[1])
        lm = math.hypot(*m) or 1
        cosh = max(0.3, (m[0] * n1[0] + m[1] * n1[1]) / lm)
        out.append((R6(b[0] + m[0] / lm * d / cosh), R6(b[1] + m[1] / lm * d / cosh)))
    return out


def earclip(poly):
    """✂️ Triangles (index triples, counter-clockwise) of a simple counter-clockwise polygon."""
    idx = list(range(len(poly)))
    tris = []

    def inside(p, a, b, c):
        def s(p1, p2, p3):
            return (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])
        d1, d2, d3 = s(p, a, b), s(p, b, c), s(p, c, a)
        return not ((d1 < 0 or d2 < 0 or d3 < 0) and (d1 > 0 or d2 > 0 or d3 > 0))

    guard = 0
    while len(idx) > 3 and guard < 100000:
        guard += 1
        found = False
        for k in range(len(idx)):
            i0, i1, i2 = idx[k - 1], idx[k], idx[(k + 1) % len(idx)]
            a, b, c = poly[i0], poly[i1], poly[i2]
            if (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]) <= 1e-14:
                continue
            if any(inside(poly[j], a, b, c) for j in idx if j not in (i0, i1, i2)):
                continue
            tris.append((i0, i1, i2))
            idx.pop(k)
            found = True
            break
        if not found:
            raise ValueError("earclip failed")
    tris.append(tuple(idx))
    return tris


def loft_mesh(rings, cap2d=None):
    """🧊 Closed polyhedron through rings of equal length (3D points): side quads, ear-clipped end caps (cap2d gives the 2D polygons of the two caps for clipping); oriented outward."""
    n = len(rings[0])
    verts = [list(map(R6, p)) for ring in rings for p in ring]
    tris = []
    for i in range(len(rings) - 1):
        for j in range(n):
            a, b = i * n + j, i * n + (j + 1) % n
            c, d = (i + 1) * n + (j + 1) % n, (i + 1) * n + j
            tris += [(a, b, c), (a, c, d)]
    caps = cap2d or [None, None]
    cap_tris = []
    for end, ring_i in [(0, 0), (1, len(rings) - 1)]:
        base = ring_i * n
        poly = caps[end]
        own = []
        if poly is None:
            cen = [sum(p[k] for p in rings[ring_i]) / n for k in range(3)]
            ci = len(verts)
            verts.append(list(map(R6, cen)))
            for j in range(n):
                own.append((ci, base + j, base + (j + 1) % n))
        else:
            flip = area2(poly) < 0
            p2 = ccw(poly)
            order = list(range(n)) if not flip else list(reversed(range(n)))
            for (x, y, z) in earclip(p2):
                own.append((base + order[x], base + order[y], base + order[z]))
        cap_tris.append(own)
    best = None
    for f0 in (False, True):
        for f1 in (False, True):
            cand = tris + [(a, c, b) if f0 else (a, b, c) for a, b, c in cap_tris[0]] + [(a, c, b) if f1 else (a, b, c) for a, b, c in cap_tris[1]]
            try:
                check_manifold(cand)
                best = cand
            except AssertionError:
                pass
    assert best is not None, "no consistent cap orientation"
    tris = best
    vol = signed_volume(verts, tris)
    if vol < 0:
        tris = [(a, c, b) for (a, b, c) in tris]
    check_manifold(tris)
    return verts, tris


def signed_volume(verts, tris):
    return sum(dot(verts[a], cross(verts[b], verts[c])) for a, b, c in tris) / 6


def check_manifold(tris):
    edges = {}
    for a, b, c in tris:
        for e in [(a, b), (b, c), (c, a)]:
            edges[e] = edges.get(e, 0) + 1
    for (a, b), k in edges.items():
        assert k == 1 and edges.get((b, a)) == 1, ("non-manifold", a, b)


def mesh_area(verts, tris):
    return sum(math.sqrt(dot(c, c)) / 2 for c in (cross(sub(verts[b], verts[a]), sub(verts[c], verts[a])) for a, b, c in tris))
