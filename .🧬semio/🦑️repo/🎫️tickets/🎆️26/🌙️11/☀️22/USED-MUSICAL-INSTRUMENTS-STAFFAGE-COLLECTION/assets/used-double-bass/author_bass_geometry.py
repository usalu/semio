"""📐 Geometry kit of the used double bass: outline splines, ruled polyhedra, the rest frames of the rig and the part record that writes one solid and its three 2D shapes from the same numbers."""
import math

YC = 0.365
Q = (0.0, 0.0)


def r6(v):
    """✂ A number rounded to the micrometre."""
    return round(v + 0.0, 6)


def catmull(points, per):
    """🪢 A centripetal-free uniform Catmull-Rom polyline through `points` with `per` steps per interval, ends kept."""
    if per <= 1 or len(points) < 3:
        return list(points)
    out = []
    ext = [points[0]] + list(points) + [points[-1]]
    for i in range(1, len(ext) - 2):
        p0, p1, p2, p3 = ext[i - 1], ext[i], ext[i + 1], ext[i + 2]
        for k in range(per):
            t = k / per
            t2, t3 = t * t, t * t * t
            out.append(tuple(0.5 * ((2 * p1[j]) + (-p0[j] + p2[j]) * t + (2 * p0[j] - 5 * p1[j] + 4 * p2[j] - p3[j]) * t2 + (-p0[j] + 3 * p1[j] - 3 * p2[j] + p3[j]) * t3) for j in range(2)))
    out.append(points[-1])
    return out


def chain(segments, per):
    """⛓ Several spline segments joined at their shared corner points (corners stay sharp)."""
    out = []
    for seg in segments:
        pts = catmull(seg, per)
        out.extend(pts if not out else pts[1:])
    return out


def half_at(poly, u):
    """📏 The half-width of a bottom-to-top half outline at the axial position `u` (linear between samples, the widest crossing)."""
    best = None
    for (u0, v0), (u1, v1) in zip(poly, poly[1:]):
        if (u0 - u) * (u1 - u) <= 0 and u0 != u1:
            v = v0 + (v1 - v0) * (u - u0) / (u1 - u0)
            best = v if best is None else max(best, v)
    return best


def cut_at_v(poly, vcut):
    """✂ The half outline from where it first reaches the half-width `vcut` (interpolated) to its end."""
    for i, ((u0, v0), (u1, v1)) in enumerate(zip(poly, poly[1:])):
        if v0 < vcut <= v1:
            t = (vcut - v0) / (v1 - v0)
            return [(u0 + (u1 - u0) * t, vcut)] + list(poly[i + 1:])
    return list(poly)


def seg_dist(p, a, b):
    """📐 Distance of the point `p` from the segment `a`-`b` in the plane."""
    ax, ay = b[0] - a[0], b[1] - a[1]
    L = ax * ax + ay * ay
    t = 0 if L == 0 else max(0, min(1, ((p[0] - a[0]) * ax + (p[1] - a[1]) * ay) / L))
    return math.hypot(p[0] - a[0] - t * ax, p[1] - a[1] - t * ay)


def clearance(inner, outer):
    """↔ The smallest distance from any point of `inner` to the polyline `outer`."""
    return min(min(seg_dist(p, a, b) for a, b in zip(outer, outer[1:])) for p in inner)


class Mesh:
    """🧊 A closed triangle mesh in a bone frame, built ring by ring, written as a `polyhedron` solid."""

    def __init__(self):
        self.v, self.t = [], []

    def add(self, p):
        self.v.append(tuple(p))
        return len(self.v) - 1

    def quad(self, a, b, c, d):
        self.t += [(a, b, c), (a, c, d)]

    def tri(self, a, b, c):
        self.t.append((a, b, c))

    def orient(self):
        """🔄 Flips every triangle when the signed volume is negative, so the normals point outwards."""
        vol = 0.0
        for a, b, c in self.t:
            (x1, y1, z1), (x2, y2, z2), (x3, y3, z3) = self.v[a], self.v[b], self.v[c]
            vol += x1 * (y2 * z3 - y3 * z2) - x2 * (y1 * z3 - y3 * z1) + x3 * (y1 * z2 - y2 * z1)
        if vol < 0:
            self.t = [(a, c, b) for a, b, c in self.t]
        return abs(vol) / 6

    def area(self):
        """📐 Surface area of the mesh."""
        s = 0.0
        for a, b, c in self.t:
            p, q, r = self.v[a], self.v[b], self.v[c]
            u = [q[i] - p[i] for i in range(3)]
            w = [r[i] - p[i] for i in range(3)]
            s += 0.5 * math.sqrt((u[1] * w[2] - u[2] * w[1]) ** 2 + (u[2] * w[0] - u[0] * w[2]) ** 2 + (u[0] * w[1] - u[1] * w[0]) ** 2)
        return s

    def solid(self):
        """🧱 The polyhedron members of a solid."""
        return {"kind": "polyhedron", "vertices": [[r6(a) for a in p] for p in self.v], "triangles": [list(t) for t in self.t]}


def symmetric_body(half, rings):
    """🎻 A closed mirror-symmetric body from a bottom-to-top half outline (first half-width 0, last positive) and rings (s, wfun): each ring is the outline with its half-widths scaled by `s` at the depth `wfun(u, s)`; the first and last rings are capped by strips between the mirrored sides."""
    m = Mesh()
    n = len(half) - 1
    idx = []
    for s, wf in rings:
        R = [m.add((u, s * v, wf(u, s))) for u, v in half]
        L = [None] + [m.add((u, -s * v, wf(u, s))) for u, v in half[1:]]
        L[0] = R[0]
        idx.append((R, L))
    loops = [R + [L[i] for i in range(n, 0, -1)] for R, L in idx]
    for a, b in zip(loops, loops[1:]):
        N = len(a)
        for k in range(N):
            m.quad(a[k], a[(k + 1) % N], b[(k + 1) % N], b[k])
    for flip, (R, L) in ((True, idx[0]), (False, idx[-1])):
        if flip:
            m.tri(R[0], L[1], R[1])
            for i in range(1, n):
                m.quad(R[i], L[i], L[i + 1], R[i + 1])
        else:
            m.tri(R[0], R[1], L[1])
            for i in range(1, n):
                m.quad(R[i], R[i + 1], L[i + 1], L[i])
    m.orient()
    return m


def hexahedron(c):
    """📦 A closed eight-corner solid from corners listed bottom face (0-3) then top face (4-7), both counter-clockwise seen from the same side."""
    m = Mesh()
    ids = [m.add(p) for p in c]
    b, t = ids[:4], ids[4:]
    m.quad(b[0], b[3], b[2], b[1])
    m.quad(t[0], t[1], t[2], t[3])
    for k in range(4):
        m.quad(b[k], b[(k + 1) % 4], t[(k + 1) % 4], t[k])
    m.orient()
    return m


def circle(cx, cy, r, n, start=0.0):
    """⭕ A polygon of `n` points on a circle."""
    return [(cx + r * math.cos(start + 2 * math.pi * k / n), cy + r * math.sin(start + 2 * math.pi * k / n)) for k in range(n)]


def ellipse(cx, cy, rx, ry, n):
    """🥚 A polygon of `n` points on an axis-parallel ellipse."""
    return [(cx + rx * math.cos(2 * math.pi * k / n), cy + ry * math.sin(2 * math.pi * k / n)) for k in range(n)]


def matmul(a, b):
    """✖ Product of two 3x3 matrices."""
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def mat_w(deg):
    """🔁 Rotation about the joint axis w: u towards v."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def mat_v(deg):
    """🔀 Rotation about the normal v: u towards w."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, 0, -s], [0, 1, 0], [s, 0, c]]


def mat_u(deg):
    """🌀 Rotation about the bone axis u: v towards -w."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[1, 0, 0], [0, c, s], [0, -s, c]]


def apply(m, p):
    """➡ A 3x3 matrix applied to a vector."""
    return tuple(sum(m[i][k] * p[k] for k in range(3)) for i in range(3))


def solve_rig(bones, offsets):
    """🦴 World frames {bone: (origin, matrix)} of the rig for pose offsets, by the chain rule of the staffage README."""
    out = {}
    for b in bones:
        off = offsets.get(b["id"], {})
        j = b["joint"]
        dx = dy = 0.0
        ang, abd, tw = 0.0, 0.0, 0.0
        if j["type"] == "prismatic":
            a = math.radians(j.get("axisDeg", 0))
            s = off.get("dx", 0) * math.cos(a) + off.get("dy", 0) * math.sin(a)
            s = max(j.get("minM", -1e9), min(j.get("maxM", 1e9), s))
            dx, dy = s * math.cos(a), s * math.sin(a)
        elif j["type"] == "revolute":
            ang, abd, tw = off.get("angleDeg", 0), off.get("abductionDeg", 0), off.get("twistDeg", 0)
        rest = matmul(matmul(mat_w(b["angleDeg"]), mat_v(b.get("abductionDeg", 0))), mat_u(b.get("twistDeg", 0)))
        rot = matmul(matmul(mat_w(ang), mat_v(abd)), mat_u(tw))
        if b["parent"] is None:
            po, pm = (0.0, 0.0, 0.0), [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
        else:
            po, pm = out[b["parent"]]
        d = apply(pm, (b["x"] + dx, b["y"] + dy, b.get("z", 0.0)))
        out[b["id"]] = (tuple(po[i] + d[i] for i in range(3)), matmul(matmul(pm, rest), rot))
    return out


def to_world(frames, bone, p):
    """🌍 A bone-local point in world coordinates."""
    o, m = frames[bone]
    q = apply(m, p)
    return tuple(o[i] + q[i] for i in range(3))
