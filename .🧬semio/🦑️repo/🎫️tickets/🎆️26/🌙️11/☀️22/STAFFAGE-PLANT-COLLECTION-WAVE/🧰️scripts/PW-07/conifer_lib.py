"""🌲️ PW-07 conifer kit: one parametric 3D design per scale feeds the rig, the volumes and the three 2D views, so every drawn shape is the true
orthographic projection of the solid it draws (revolve about a vertical axis: its radial envelope; capsule: the hull of its two end circles;
ellipsoid: its ellipse). Species files (`norway_spruce.py`, `european_larch.py`, `atlas_cedar.py`) choose every number; this module only places,
projects, measures and serialises.

Reuses `T` and `wind_force` of the NW-V1 vegetation toolkit and its pens and valid ranges, imported from their place.
@see ../../../../🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-V1/lib.py
"""
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *([".."] * 9)))
NWV1 = os.path.join(REPO, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️10", "☀️29", "ARCHITECTURAL-STAFFAGE-ARCHIVE", "🧰️scripts", "NW-V1")
sys.path.insert(0, NWV1)
from lib import T, wind_force  # noqa: E402
from common import PENS, RANGES  # noqa: E402

CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🌳️vegetation")
TICKET = os.path.abspath(os.path.join(HERE, "..", ".."))
SCALES = ["500", "200", "100", "50"]
D2R = math.pi / 180.0
G = 9.80665
RHO = 1.25


def r4(v):
    v = round(v, 4)
    return 0.0 if v == 0 else v


def r3(v):
    v = round(v, 3)
    return 0.0 if v == 0 else v


class Bone:
    def __init__(self, id, parent, origin, theta, length, title, limit=None, section=None, mass=None):
        self.id, self.parent, self.o, self.theta, self.L, self.title = id, parent, tuple(origin), theta, length, title
        self.limit, self.section, self.mass = limit, section, mass
        c, s = math.cos(theta * D2R), math.sin(theta * D2R)
        self.u, self.v = (c, s), (-s, c)

    def local(self, p):
        dx, dy = p[0] - self.o[0], p[1] - self.o[1]
        return (dx * self.u[0] + dy * self.u[1], dx * self.v[0] + dy * self.v[1], p[2] - self.o[2])

    def world(self, q):
        return (self.o[0] + q[0] * self.u[0] + q[1] * self.v[0], self.o[1] + q[0] * self.u[1] + q[1] * self.v[1], self.o[2] + q[2])

    def tip(self):
        return self.world((self.L, 0, 0))

    @property
    def vertical(self):
        return abs(self.theta - 90) < 1e-9


class Rig:
    def __init__(self):
        self.bones, self.order = {}, []

    def add(self, *a, **k):
        b = Bone(*a, **k)
        assert b.id not in self.bones, b.id
        self.bones[b.id] = b
        self.order.append(b.id)
        return b

    def json(self):
        out = []
        for id in self.order:
            b = self.bones[id]
            if b.parent is None:
                d = {"id": id, "parent": None, "title": b.title, "x": r4(b.o[0]), "y": r4(b.o[1]), "angleDeg": r3(b.theta), "lengthM": r3(b.L), "joint": {"type": "fixed"}}
            else:
                p = self.bones[b.parent]
                lx, ly, lz = p.local(b.o)
                d = {"id": id, "parent": b.parent, "title": b.title, "x": r4(lx), "y": r4(ly)}
                if abs(lz) > 1e-9:
                    d["z"] = r4(lz)
                d.update({"angleDeg": r3(b.theta - p.theta), "lengthM": r3(b.L), "joint": {"type": "revolute", "minDeg": -b.limit, "maxDeg": b.limit}})
            if b.mass:
                d["mass"] = b.mass
            if b.section:
                d["section"] = b.section
            out.append(d)
        return out


def section(d, material, title_en, title_de):
    return {"areaM2": float("%.6g" % (math.pi * d * d / 4)), "secondMomentM4": float("%.4g" % (math.pi * d ** 4 / 64)), "extremeFibreM": r4(d / 2), "material": material,
            "title": T("%s, solid section d = %.0f cm" % (title_en, d * 100), "%s, Vollquerschnitt d = %.0f cm" % (title_de, d * 100))}


# ------------------------------------------------------------------ profiles of revolution ----------------------------------------------

def rmax_at(profile, a):
    best = None
    n = len(profile)
    for i in range(n):
        a0, r0 = profile[i]
        a1, r1 = profile[(i + 1) % n]
        lo, hi = min(a0, a1), max(a0, a1)
        if a < lo - 1e-12 or a > hi + 1e-12:
            continue
        if abs(a1 - a0) < 1e-12:
            r = max(r0, r1)
        else:
            t = (a - a0) / (a1 - a0)
            r = r0 + (r1 - r0) * t
        best = r if best is None else max(best, r)
    return best


def envelope(profile, step=0.02):
    """The radial envelope R(a) of a closed profile, top to bottom, as (a, R) points: the half outline of the solid seen from the side."""
    ys = sorted(set([p[0] for p in profile]))
    lo, hi = ys[0], ys[-1]
    n = max(2, int((hi - lo) / step))
    hs = sorted(set(ys + [lo + (hi - lo) * i / n for i in range(n + 1)]), reverse=True)
    pts = [(a, rmax_at(profile, a)) for a in hs]
    return simplify(pts, 0.002)


def simplify(pts, tol):
    if len(pts) < 3:
        return pts
    a, b = pts[0], pts[-1]
    dx, dy = b[0] - a[0], b[1] - a[1]
    L = math.hypot(dx, dy) or 1e-12
    worst, k = -1, 0
    for i in range(1, len(pts) - 1):
        d = abs((pts[i][0] - a[0]) * dy - (pts[i][1] - a[1]) * dx) / L
        if d > worst:
            worst, k = d, i
    if worst <= tol:
        return [a, b]
    return simplify(pts[: k + 1], tol)[:-1] + simplify(pts[k:], tol)


def profile_volume(profile):
    m = 0.0
    n = len(profile)
    for i in range(n):
        a0, r0 = profile[i]
        a1, r1 = profile[(i + 1) % n]
        m += (r0 + r1) * (a0 * r1 - a1 * r0)
    return abs(m) / 6.0 * 2 * math.pi


def profile_lateral_area(profile):
    s = 0.0
    n = len(profile)
    for i in range(n):
        a0, r0 = profile[i]
        a1, r1 = profile[(i + 1) % n]
        if r0 < 1e-9 and r1 < 1e-9:
            continue
        s += math.pi * (r0 + r1) * math.hypot(a1 - a0, r1 - r0)
    return s


# ------------------------------------------------------------------ solids with exact projections -----------------------------------------

CHORD = 0.02


def segments(r):
    """Segments of a full circle of radius r in the reference tessellation of the silhouette validators (chord 20 mm for assets of 16 m and more, 24 to 64 segments)."""
    ratio = min(1.0, CHORD / max(r, 1e-9))
    n = 24 if ratio >= 1 else math.ceil(math.pi / math.acos(1 - ratio))
    return max(24, min(64, n))


def ring_factors(n):
    """Reach of a tessellated ring of n segments towards the bone normal (both sides) and towards -w, as fractions of its radius (+w is always reached)."""
    return max(math.sin(2 * math.pi * j / n) for j in range(n)), max(-math.cos(2 * math.pi * j / n) for j in range(n))


def fit_ring(x0, x1, z0, z1, R_unit):
    """Centre, radius factor and lateral scale of a revolve of unit maximal radius R_unit whose tessellated plan box is [x0, x1] x [z0, z1]."""
    N = 32
    for _ in range(8):
        s, c = ring_factors(N)
        R = (x1 - x0) / (2 * s)
        k = (z1 - z0) / ((1 + c) * R)
        N2 = segments(R * max(1.0, k))
        if N2 == N:
            break
        N = N2
    s, c = ring_factors(N)
    R = (x1 - x0) / (2 * s)
    k = (z1 - z0) / ((1 + c) * R)
    return (x0 + x1) / 2, z1 - k * R, R / R_unit, k


def fit_lateral(z0, z1, R):
    """Centre and lateral scale of a revolve of maximal radius R whose tessellated lateral extent is [z0, z1]."""
    N = segments(R)
    for _ in range(8):
        s, c = ring_factors(N)
        k = (z1 - z0) / ((1 + c) * R)
        N2 = segments(R * max(1.0, k))
        if N2 == N:
            break
        N = N2
    s, c = ring_factors(N)
    k = (z1 - z0) / ((1 + c) * R)
    return z1 - k * R, k


class Solid:
    """One body solid in world rest coordinates with its bone, its volume JSON and its three projections."""
    region = None
    regions = None

    def common(self):
        d = {"id": self.id, "bone": self.bone.id, "material": self.material, "role": "body", "tone": self.tone}
        if getattr(self, "slender", False):
            d["slender"] = True
        return d


class Revolve(Solid):
    def __init__(self, id, bone, cx, cz, profile, material, tone, region=None, ls=1.0):
        assert bone.vertical, bone.id
        self.id, self.bone, self.cx, self.cz, self.material, self.tone, self.region, self.ls = id, bone, cx, cz, material, tone, region, ls
        self.profile = [(a - bone.o[1], r) for a, r in profile]
        self.env = envelope(self.profile)
        self.R = max(r for _, r in self.profile)
        self.N = segments(self.R * max(1.0, ls))
        self.s, self.c = ring_factors(self.N)

    def json(self):
        d = self.common()
        lv = -(self.cx - self.bone.o[0])
        lw = self.cz - self.bone.o[2]
        d.update({"kind": "revolve", "profile": [[r4(a), r4(r)] for a, r in self.profile], "axis": [1, 0, 0], "origin": [0, r4(lv), r4(lw)]})
        if abs(self.ls - 1) > 1e-9:
            d["lateralScale"] = r4(self.ls)
        return d

    def outline_world(self, view):
        oy = self.bone.o[1]
        if view == "plan":
            return ("poly", [(self.cx + self.R * math.sin(2 * math.pi * j / self.N), self.cz + self.R * self.ls * math.cos(2 * math.pi * j / self.N)) for j in range(self.N)])
        if view == "elevation":
            c, kp, km = self.cx, self.s, self.s
        else:
            c, kp, km = self.cz, self.ls, self.ls * self.c
        right = [(c + kp * r, oy + a) for a, r in self.env]
        left = [(c - km * r, oy + a) for a, r in reversed(self.env)]
        ring = right + left
        out = []
        for p in ring:
            if not out or math.hypot(p[0] - out[-1][0], p[1] - out[-1][1]) > 1e-6:
                out.append(p)
        if math.hypot(out[0][0] - out[-1][0], out[0][1] - out[-1][1]) < 1e-6:
            out.pop()
        return ("poly", out)

    def volume(self):
        return profile_volume(self.profile) * self.ls

    def lateral_area(self):
        return profile_lateral_area(self.profile) * (1 + self.ls) / 2

    def bbox(self):
        ys = [self.bone.o[1] + a for a, _ in self.profile]
        return (self.cx - self.s * self.R, min(ys), self.cz - self.c * self.R * self.ls, self.cx + self.s * self.R, max(ys), self.cz + self.R * self.ls)


class Capsule(Solid):
    def __init__(self, id, bone, p0, p1, r0, r1, material, tone, region=None, slender=False):
        self.id, self.bone, self.p0, self.p1, self.r0, self.r1, self.material, self.tone, self.region, self.slender = id, bone, tuple(p0), tuple(p1), r0, r1, material, tone, region, slender

    def json(self):
        d = self.common()
        a, b = self.bone.local(self.p0), self.bone.local(self.p1)
        d.update({"kind": "capsule", "p0": [r4(x) for x in a], "p1": [r4(x) for x in b], "r0": r4(self.r0), "r1": r4(self.r1)})
        return d

    def outline_world(self, view):
        if view == "elevation":
            return ("capsule", (self.p0[0], self.p0[1], self.p1[0], self.p1[1], self.r0, self.r1))
        if view == "front":
            return ("capsule", (self.p0[2], self.p0[1], self.p1[2], self.p1[1], self.r0, self.r1))
        return ("capsule", (self.p0[0], self.p0[2], self.p1[0], self.p1[2], self.r0, self.r1))

    def volume(self):
        L = math.dist(self.p0, self.p1)
        return math.pi / 3 * L * (self.r0 ** 2 + self.r0 * self.r1 + self.r1 ** 2) + 2 / 3 * math.pi * (self.r0 ** 3 + self.r1 ** 3)

    def lateral_area(self):
        L = math.dist(self.p0, self.p1)
        return math.pi * (self.r0 + self.r1) * math.hypot(L, self.r0 - self.r1) + 2 * math.pi * (self.r0 ** 2 + self.r1 ** 2)

    def bbox(self):
        lo = [min(self.p0[k] - self.r0, self.p1[k] - self.r1) for k in range(3)]
        hi = [max(self.p0[k] + self.r0, self.p1[k] + self.r1) for k in range(3)]
        return (lo[0], lo[1], lo[2], hi[0], hi[1], hi[2])


class Ellipsoid(Solid):
    def __init__(self, id, bone, c, radii, material, tone, region=None):
        assert bone.vertical, bone.id
        self.id, self.bone, self.c, self.radii, self.material, self.tone, self.region = id, bone, tuple(c), tuple(radii), material, tone, region

    def json(self):
        d = self.common()
        q = self.bone.local(self.c)
        rx, ry, rz = self.radii
        d.update({"kind": "ellipsoid", "radii": [r4(ry), r4(rx), r4(rz)], "transform": {"translation": [r4(x) for x in q]}})
        return d

    def outline_world(self, view):
        cx, cy, cz = self.c
        rx, ry, rz = self.radii
        if view == "elevation":
            return ("ellipse", (cx, cy, rx, ry))
        if view == "front":
            return ("ellipse", (cz, cy, rz, ry))
        return ("ellipse", (cx, cz, rx, rz))

    def volume(self):
        return 4 / 3 * math.pi * self.radii[0] * self.radii[1] * self.radii[2]

    def bbox(self):
        return tuple(self.c[k] - self.radii[k] for k in range(3)) + tuple(self.c[k] + self.radii[k] for k in range(3))


# ------------------------------------------------------------------ 2D shapes ----------------------------------------------------------

def path_d(rings, closed=True):
    parts = []
    for ring in rings:
        s = "M %s %s" % (r4(ring[0][0]), r4(ring[0][1])) + "".join(" L %s %s" % (r4(p[0]), r4(p[1])) for p in ring[1:])
        parts.append(s + (" Z" if closed else ""))
    return " ".join(parts)


def to_elev_local(bone, pts):
    out = []
    for x, y in pts:
        q = bone.local((x, y, bone.o[2]))
        out.append((q[0], q[1]))
    return out


def shape_of(solid, view, z, pen=None, tone=None):
    kind, g = solid.outline_world(view)
    d = {"id": solid.id, "bone": solid.bone.id, "role": "body", "z": z, "filled": True, "tone": solid.tone if tone is None else tone}
    if solid.regions:
        d["regions"] = solid.regions
    elif solid.region:
        d["region"] = solid.region
    d["material"] = solid.material
    if pen:
        d["penMm"] = pen
    b = solid.bone
    if kind == "poly":
        pts = to_elev_local(b, g) if view == "elevation" else g
        d.update({"kind": "path", "d": path_d([pts])})
    elif kind == "ellipse":
        cx, cy, rx, ry = g
        if view == "elevation":
            q = b.local((cx, cy, 0))
            if b.vertical:
                cx, cy, rx, ry = q[0], q[1], ry, rx
            else:
                raise ValueError("ellipse on tilted bone")
        d.update({"kind": "ellipse", "cx": r4(cx), "cy": r4(cy), "rx": r4(rx), "ry": r4(ry)})
    else:
        x0, y0, x1, y1, ra, rb = g
        if view == "elevation":
            q0, q1 = b.local((x0, y0, 0)), b.local((x1, y1, 0))
            x0, y0, x1, y1 = q0[0], q0[1], q1[0], q1[1]
        d.update({"kind": "capsule", "x0": r4(x0), "y0": r4(y0), "x1": r4(x1), "y1": r4(y1), "r0": r4(ra), "r1": r4(rb)})
    return d


def outline_polyline(solid, view, n=48):
    """The outline of a solid's projection in world paper axes (elevation x-y, front z-y, plan x-z) as a closed polyline for distance checks."""
    kind, g = solid.outline_world(view)
    if kind == "poly":
        return g
    if kind == "ellipse":
        cx, cy, rx, ry = g
        return [(cx + rx * math.cos(2 * math.pi * i / n), cy + ry * math.sin(2 * math.pi * i / n)) for i in range(n)]
    return capsule_ring(*g, n=n)


def capsule_ring(x0, y0, x1, y1, r0, r1, n=24):
    dx, dy = x1 - x0, y1 - y0
    L = math.hypot(dx, dy)
    if L < 1e-9 or L <= abs(r0 - r1):
        r = max(r0, r1)
        cx, cy = (x0, y0) if r0 >= r1 else (x1, y1)
        return [(cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)) for i in range(n)]
    ux, uy = dx / L, dy / L
    ang = math.atan2(uy, ux)
    beta = math.acos(max(-1, min(1, (r0 - r1) / L)))
    pts = []
    for i in range(n + 1):
        t = ang + beta + (2 * math.pi - 2 * beta) * i / n
        pts.append((x0 + r0 * math.cos(t), y0 + r0 * math.sin(t)))
    for i in range(n + 1):
        t = ang - beta + 2 * beta * i / n
        pts.append((x1 + r1 * math.cos(t), y1 + r1 * math.sin(t)))
    return pts


def inside(pt, ring):
    x, y = pt
    c = False
    n = len(ring)
    for i in range(n):
        ax, ay = ring[i]
        bx, by = ring[(i + 1) % n]
        if (ay > y) != (by > y) and x < (bx - ax) * (y - ay) / (by - ay + 1e-300) + ax:
            c = not c
    return c


def seg_dist(p, q, a, b):
    def pt_seg(p, a, b):
        dx, dy = b[0] - a[0], b[1] - a[1]
        l2 = dx * dx + dy * dy
        t = 0 if l2 == 0 else max(0, min(1, ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / l2))
        return math.hypot(p[0] - a[0] - t * dx, p[1] - a[1] - t * dy)

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    d1, d2, d3, d4 = cross(a, b, p), cross(a, b, q), cross(p, q, a), cross(p, q, b)
    if ((d1 > 0) != (d2 > 0)) and ((d3 > 0) != (d4 > 0)):
        return 0.0
    return min(pt_seg(p, a, b), pt_seg(q, a, b), pt_seg(a, p, q), pt_seg(b, p, q))


def poly_dist(A, B, closedA=True, closedB=True):
    sa = list(zip(A, A[1:] + ([A[0]] if closedA else [])))
    sb = list(zip(B, B[1:] + ([B[0]] if closedB else [])))
    best = float("inf")
    for p, q in sa:
        pminx, pmaxx, pminy, pmaxy = min(p[0], q[0]), max(p[0], q[0]), min(p[1], q[1]), max(p[1], q[1])
        for a, b in sb:
            if max(a[0], b[0]) < pminx - best or min(a[0], b[0]) > pmaxx + best or max(a[1], b[1]) < pminy - best or min(a[1], b[1]) > pmaxy + best:
                continue
            d = seg_dist(p, q, a, b)
            if d < best:
                best = d
                if best == 0:
                    return 0.0
    return best


# ------------------------------------------------------------------ texture marks -------------------------------------------------------

class Marks:
    """Texture strokes of one view, kept inside their host outline and at least `margin` (metres) from every other outline and from the marks of other shapes."""

    def __init__(self, view, outlines, margin):
        self.view, self.outlines, self.margin = view, outlines, margin
        self.boxes = {k: box(r) for k, r in outlines.items()}
        self.accepted = {}

    def offer(self, key, host, stroke):
        hostring = self.outlines[host]
        for p in stroke:
            if not inside(p, hostring):
                return False
        sb = box(stroke, self.margin)
        for oid, ring in self.outlines.items():
            if not overlaps(sb, self.boxes[oid]):
                continue
            if poly_dist(stroke, ring, False, True) < self.margin:
                return False
        for k, strokes in self.accepted.items():
            if k == key:
                continue
            for s in strokes:
                if overlaps(sb, box(s)) and poly_dist(stroke, s, False, False) < self.margin:
                    return False
        self.accepted.setdefault(key, []).append(stroke)
        return True


def box(ring, grow=0.0):
    xs = [p[0] for p in ring]
    ys = [p[1] for p in ring]
    return (min(xs) - grow, min(ys) - grow, max(xs) + grow, max(ys) + grow)


def overlaps(a, b):
    return a[0] <= b[2] and b[0] <= a[2] and a[1] <= b[3] and b[1] <= a[3]


def marks_shape(id, bone, view, strokes, z, tone, region=None, material=None):
    pts = [to_elev_local(bone, s) if view == "elevation" else s for s in strokes]
    d = {"id": id, "bone": bone.id, "role": "texture", "z": z, "filled": False, "tone": tone, "kind": "path", "d": path_d(pts, closed=False)}
    if region:
        d["region"] = region
    if material:
        d["material"] = material
    return d


# ------------------------------------------------------------------ physics helpers ----------------------------------------------------

def v_at(v10, z):
    return v10 * (max(z, 1.0) / 10.0) ** 0.22


def union_width(intervals):
    iv = sorted(intervals)
    tot, cur = 0.0, None
    for a, b in iv:
        if cur is None or a > cur[1]:
            if cur:
                tot += cur[1] - cur[0]
            cur = [a, b]
        else:
            cur[1] = max(cur[1], b)
    if cur:
        tot += cur[1] - cur[0]
    return tot


def slice_intervals(solids, y, view="elevation"):
    out = []
    for s in solids:
        ring = outline_polyline(s, view, 36)
        xs = []
        n = len(ring)
        for i in range(n):
            (ax, ay), (bx, by) = ring[i], ring[(i + 1) % n]
            if (ay - y) * (by - y) <= 0 and ay != by:
                xs.append(ax + (bx - ax) * (y - ay) / (by - ay))
        if len(xs) >= 2:
            out.append((min(xs), max(xs)))
    return out


def cantilever_slopes(heights, forces, EI_of, k_root, y_top):
    """Slope (rad) of a cantilever stem fixed in a rotational root spring k_root under horizontal point forces F_i at heights h_i."""
    M0 = sum(f * h for f, h in zip(forces, heights))
    slope, y, dy = M0 / k_root, 0.0, 0.05
    out = [(0.0, slope)]
    while y < y_top:
        ym = y + dy / 2
        M = sum(f * (h - ym) for f, h in zip(forces, heights) if h > ym)
        slope += M / EI_of(ym) * dy
        y += dy
        out.append((y, slope))
    return out


def slope_at(table, y):
    for (y0, s0), (y1, s1) in zip(table, table[1:]):
        if y0 <= y <= y1:
            return s0 + (s1 - s0) * (y - y0) / (y1 - y0 + 1e-12)
    return table[-1][1]


def dump(path, obj):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(obj, f, ensure_ascii=False, separators=(",", ":"))
        f.write("\n")


def decisions(d):
    return {k: T(*d[k]) for k in ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]}


def representation(denominator, shapes, dec):
    lo, hi = RANGES[denominator]
    return {"status": "authored", "denominator": int(denominator), "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[denominator], "hatches": [],
            "geometry": shapes, "decisions": decisions(dec)}


def bbox_union(solids):
    bs = [s.bbox() for s in solids]
    return (min(b[0] for b in bs), min(b[1] for b in bs), min(b[2] for b in bs), max(b[3] for b in bs), max(b[4] for b in bs), max(b[5] for b in bs))


def ring_clearance(ringA, ringB):
    """0 when two closed outlines cross or one holds the other, else their smallest distance."""
    d = poly_dist(ringA, ringB)
    return d


def gap_report(solids, view, denominator, pens_mm, min_gap_mm=0.15):
    """Pairs of filled outlines of different shapes closer than the printable gap without touching (conservative: hidden lines are not removed)."""
    rings = [(s.id, outline_polyline(s, view, 40)) for s in solids]
    need = (min_gap_mm + pens_mm) * int(denominator) / 1000.0
    boxes = [box(r, need) for _, r in rings]
    bad = []
    for i in range(len(rings)):
        for j in range(i + 1, len(rings)):
            if not overlaps(boxes[i], boxes[j]):
                continue
            d = poly_dist(rings[i][1], rings[j][1])
            if 0 < d < need:
                bad.append((rings[i][0], rings[j][0], d * 1000 / int(denominator)))
    return bad
