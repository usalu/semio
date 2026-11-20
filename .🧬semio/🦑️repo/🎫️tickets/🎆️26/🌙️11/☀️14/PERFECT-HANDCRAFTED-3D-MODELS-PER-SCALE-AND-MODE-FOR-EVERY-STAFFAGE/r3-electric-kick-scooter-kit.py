"""🛴 Round 3 kit of vehicles/electric-kick-scooter: the rest frames of the corrected rig, the hand-stated key points of the real folding e-scooter in world metres (x ahead, y up, z left) and small writers that turn one hand-stated part into its kernel solid (bone-local u, v, w), its thermal entry and its 2D shape (extruded profile or flat outline) so that drawing and model are one body (ruling E1). The writers follow r3-city-bicycle-kit.py."""
import copy
import math

RAKE = 8.0
COSR, SINR = math.cos(math.radians(RAKE)), math.sin(math.radians(RAKE))
AF = (0.43, 0.108)
AR = (-0.43, 0.108)
TYRE_R = 0.108
H = (0.389, 0.38)
PIN_BACK = 0.024
P = (H[0] - PIN_BACK * COSR, H[1] - PIN_BACK * SINR)
STEM_LENGTH = 0.719
FOLD_DEG = 95.0
DOWN = (SINR, -COSR)
AHEAD = (COSR, SINR)
FORK_LENGTH = (AF[0] - H[0]) * DOWN[0] + (AF[1] - H[1]) * DOWN[1]
FORK_OFFSET = (AF[0] - H[0]) * AHEAD[0] + (AF[1] - H[1]) * AHEAD[1]

BONES = {
    "deck": ((0.0, 0.0), 0.0),
    "fork": (H, -90.0 + RAKE),
    "stem": (P, 90.0 + RAKE),
    "wheel-f": (AF, 0.0),
    "wheel-r": (AR, 0.0),
}

LAYERED = {}
SINGLE = {}


def r5(x):
    return round(x, 5) + 0.0


def r4(x):
    return round(x, 4) + 0.0


def vec(a):
    return [r5(a[0]), r5(a[1]), r5(a[2])]


def sub(a, b):
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def add(a, b, k=1.0):
    return [a[0] + k * b[0], a[1] + k * b[1], a[2] + k * b[2]]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def norm(a):
    return math.sqrt(dot(a, a))


def unit(a):
    n = norm(a)
    return [q / n for q in a]


def lerp(a, b, t):
    return [a[k] + (b[k] - a[k]) * t for k in range(len(a))]


def loc(bone, p):
    """📍 A world rest point (x, y[, z]) in the rest frame of the bone (u, v, w)."""
    (ox, oy), deg = BONES[bone]
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    dx, dy = p[0] - ox, p[1] - oy
    return [dx * c + dy * s, -dx * s + dy * c, p[2] if len(p) > 2 else 0.0]


def locd(bone, d):
    """🧭 A world direction in the rest frame of the bone."""
    deg = BONES[bone][1]
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [d[0] * c + d[1] * s, -d[0] * s + d[1] * c, d[2] if len(d) > 2 else 0.0]


def steer_pt(u, v=0.0, z=0.0):
    """🧷 A point given along the steering axis (u from the fork origin down the axis, v ahead of it) in world metres."""
    return [H[0] + u * DOWN[0] + v * AHEAD[0], H[1] + u * DOWN[1] + v * AHEAD[1], z]


def stem_pt(u, v=0.0, z=0.0):
    """🪜 A point given in the stem frame (u from the hinge pin up the stem, v behind it; the tube axis runs at v = -PIN_BACK) in world metres."""
    return [P[0] - u * DOWN[0] - v * AHEAD[0], P[1] - u * DOWN[1] - v * AHEAD[1], z]


def polar(c, r, deg, z=0.0):
    return [c[0] + r * math.cos(math.radians(deg)), c[1] + r * math.sin(math.radians(deg)), z]


def quat(X, Y):
    """🌀 The unit quaternion [x, y, z, w] of the rotation that carries local x to X and local y to Y (both unit, orthogonal)."""
    Z = cross(X, Y)
    m = [[X[0], Y[0], Z[0]], [X[1], Y[1], Z[1]], [X[2], Y[2], Z[2]]]
    tr = m[0][0] + m[1][1] + m[2][2]
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    n = math.sqrt(sum(c * c for c in q))
    return [round(c / n, 7) + 0.0 for c in q]


def rot_w(deg):
    h = math.radians(deg) / 2
    return [0, 0, round(math.sin(h), 7), round(math.cos(h), 7)]


def bezier(a, c, b, n):
    """〰️ n + 1 points of the quadratic curve from a to b with the corner c (a bend of a tube)."""
    out = []
    for k in range(n + 1):
        t = k / n
        out.append([(1 - t) ** 2 * a[q] + 2 * t * (1 - t) * c[q] + t * t * b[q] for q in range(len(a))])
    return out


def bent(points, radii, n=5):
    """🪈 The centre line of a bent tube: the corners of the polyline are rounded over the given tangent lengths."""
    out = [list(points[0])]
    for k in range(1, len(points) - 1):
        a, c, b = points[k - 1], points[k], points[k + 1]
        d = radii[k - 1]
        ta = add(c, unit(sub(a, c)), d)
        tb = add(c, unit(sub(b, c)), d)
        out.extend(bezier(ta, c, tb, n))
    out.append(list(points[-1]))
    return out


def arc(c, r, a0, a1, n):
    return [[c[0] + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), c[1] + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))] for k in range(n + 1)]


def ribbon(path, half):
    """🎗️ The closed outline of a band of half width `half` (a number or one per point) about a planar centre line."""
    left, right = [], []
    n = len(path)
    for k, p in enumerate(path):
        a, b = path[max(0, k - 1)], path[min(n - 1, k + 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy)
        h = half[k] if isinstance(half, (list, tuple)) else half
        left.append([p[0] - dy / L * h, p[1] + dx / L * h])
        right.append([p[0] + dy / L * h, p[1] - dx / L * h])
    return left + right[::-1]


def thin(points, tol):
    """✂️ Douglas-Peucker on an open polyline: a drawn outline carries no tessellation noise."""
    if len(points) < 3:
        return points
    a, b = points[0], points[-1]
    L = math.hypot(b[0] - a[0], b[1] - a[1]) or 1e-12
    worst, at = 0.0, 0
    for k in range(1, len(points) - 1):
        p = points[k]
        d = abs((b[0] - a[0]) * (a[1] - p[1]) - (a[0] - p[0]) * (b[1] - a[1])) / L
        if d > worst:
            worst, at = d, k
    if worst <= tol:
        return [a, b]
    return thin(points[: at + 1], tol)[:-1] + thin(points[at:], tol)


def thin_ring(points, tol):
    k = max(range(len(points)), key=lambda i: math.hypot(points[i][0] - points[0][0], points[i][1] - points[0][1]))
    a = thin(points[: k + 1], tol)
    b = thin(points[k:] + [points[0]], tol)
    return a[:-1] + b[:-1]


class Scale:
    """🗂️ One authored scale: the three solid lists and the three drawings, filled part by part."""

    def __init__(self, denominator, pens):
        self.d = denominator
        self.body, self.thermal, self.structure = [], [], []
        self.shapes = {"elevation": [], "front": [], "plan": []}
        self.pens = pens
        self.layers = denominator == 50
        self.areas = {}

    def solid(self, i, kind, bone, material, tone, role="body", **extra):
        s = {"id": i, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
        s.update({k: v for k, v in extra.items() if v is not None})
        return s

    def put(self, solid, region=None, inner=None):
        """➕ A body solid with its thermal entry: the outer layer is the body surface itself, a region with a steel wall under a paint film gets the wall as a nested solid drawn in by the film (1:50)."""
        self.body.append(solid)
        if region is not None:
            self.therm(solid, region, inner)
        return solid

    def therm(self, solid, region, inner=None, own=False):
        i = solid["id"]
        if own:
            t = dict(solid)
            t["id"] = "th-" + i
            t["region"] = region
            self.thermal.append(t)
            return
        if region in LAYERED and self.layers:
            self.thermal.append({"id": "th-" + i, "of": i, "region": region, "layer": "paint"})
            w = inner if inner is not None else shrink(solid, LAYERED[region])
            w = dict(w)
            w["id"] = "th-" + i + "-wall"
            w["region"] = region
            w["layer"] = "wall"
            self.thermal.append(w)
        elif region in SINGLE and self.layers:
            self.thermal.append({"id": "th-" + i, "of": i, "region": region, "layer": SINGLE[region]})
        else:
            self.thermal.append({"id": "th-" + i, "of": i, "region": region})

    def struct(self, solid, member, part="member", anchor=None):
        s = dict(solid)
        s["member"] = member
        s["part"] = part
        if anchor is not None:
            s["anchor"] = anchor
        self.structure.append(s)

    def struct_of(self, i, member, part="member"):
        self.structure.append({"id": i, "of": i, "member": member, "part": part})

    def cyl(self, i, bone, a, b, r0, material, tone, r1=None, role="body", **extra):
        """🥫 A flat-ended round bar between two world points (a cone frustum when r1 differs)."""
        la, lb = loc(bone, a), loc(bone, b)
        L = norm(sub(lb, la))
        return self.solid(i, "revolve", bone, material, tone, role, profile=[[0.0, r5(r0)], [r5(L), r5(r0 if r1 is None else r1)]], axis=[round(q, 6) for q in unit(sub(lb, la))], origin=vec(la), **extra)

    def pipe(self, i, bone, a, b, r, wall, material, tone, role="body", **extra):
        """🧻 A hollow round tube between two world points with its real wall."""
        la, lb = loc(bone, a), loc(bone, b)
        L = norm(sub(lb, la))
        return self.solid(i, "revolve", bone, material, tone, role, profile=[[0.0, r5(r - wall)], [r5(L), r5(r - wall)], [r5(L), r5(r)], [0.0, r5(r)], [0.0, r5(r - wall)]], axis=[round(q, 6) for q in unit(sub(lb, la))], origin=vec(la), **extra)

    def cap(self, i, bone, a, b, r0, material, tone, r1=None, role="body", **extra):
        return self.solid(i, "capsule", bone, material, tone, role, p0=vec(loc(bone, a)), p1=vec(loc(bone, b)), r0=r5(r0), r1=r5(r0 if r1 is None else r1), **extra)

    def tube(self, i, bone, points, material, tone, hint=(0.0, 0.0, 1.0), role="body", caps=(True, True), **extra):
        """🧵 A loft along a free world path: points (position, radius along the hint side, radius across[, exponent]); every section is perpendicular to the path."""
        secs = []
        n = len(points)
        h = locd(bone, hint)
        pts = [loc(bone, p[0]) for p in points]
        for k, p in enumerate(points):
            a, b = pts[max(0, k - 1)], pts[min(n - 1, k + 1)]
            t = unit(sub(b, a))
            d = dot(h, t)
            X = unit([h[q] - d * t[q] for q in range(3)])
            Y = cross(t, X)
            s = {"at": vec(pts[k]), "rx": r5(p[1]), "ry": r5(p[2])}
            if len(p) > 3 and abs(p[3] - 2.0) > 1e-9:
                s["exponent"] = p[3]
            s["rotation"] = quat(X, Y)
            secs.append(s)
        return self.solid(i, "loft", bone, material, tone, role, sections=secs, capStart=caps[0], capEnd=caps[1], **extra)

    def rev(self, i, bone, profile, origin, axis, material, tone, role="body", **extra):
        """🏺 A profile of [axial, radial] revolved about a world axis through a world point."""
        return self.solid(i, "revolve", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], axis=[round(q, 6) for q in unit(locd(bone, axis))], origin=vec(loc(bone, origin)), **extra)

    def prism(self, i, bone, outline, z0, z1, material, tone, role="body", holes=None, **extra):
        """🧇 A world outline in the sagittal plane pushed from z0 to z1."""
        pts = [loc(bone, p)[:2] for p in outline]
        hs = None if holes is None else [[[r5(q[0]), r5(q[1])] for q in (loc(bone, p)[:2] for p in h)] for h in holes]
        return self.solid(i, "extrude", bone, material, tone, role, profile=[[r5(p[0]), r5(p[1])] for p in pts], holes=hs, plane="xy", **{"from": r5(z0), "to": r5(z1)}, **extra)

    def box(self, i, bone, size, at, material, tone, role="body", corner=None, rotation=None, **extra):
        tr = {"translation": vec(loc(bone, at))}
        if rotation is not None:
            tr["rotation"] = rotation
        return self.solid(i, "box", bone, material, tone, role, size=vec(size), cornerRadius=corner, transform=tr, **extra)

    def ball(self, i, bone, radii, at, material, tone, role="body", rotation=None, **extra):
        tr = {"translation": vec(loc(bone, at))}
        if rotation is not None:
            tr["rotation"] = rotation
        return self.solid(i, "ellipsoid", bone, material, tone, role, radii=vec(radii), transform=tr, **extra)

    def shape(self, views, s, depth):
        """✏️ Files a 2D shape in the named views; `depth` is its world (x, y, z) nearness key, turned into the paint order of each view when the drawing is closed."""
        for v in views:
            q = copy.deepcopy(s)
            q["_depth"] = depth
            self.shapes[v].append(q)

    def member(self, i, bone, a, b, d, role, tone, region, material, pen=None, filled=True, views=("elevation", "front", "plan"), rect=None, depth=None):
        """📏 A bar between two world points drawn in its true outline in every view."""
        la, lb = loc(bone, a), loc(bone, b)
        s = {"kind": "member", "id": i, "bone": bone, "role": role, "z": 0, "filled": filled, "tone": tone, "region": region, "material": material, "from": {"x": r4(la[0]), "y": r4(la[1]), "z": r4(la[2])}, "to": {"x": r4(lb[0]), "y": r4(lb[1]), "z": r4(lb[2])}, "section": {"kind": "circle", "d": r4(d)} if rect is None else {"kind": "rect", "w": r4(rect[0]), "h": r4(rect[1])}}
        if pen is not None:
            s["penMm"] = pen
        m = lerp(a, b, 0.5)
        e = d / 2 if rect is None else max(rect) / 2
        self.shape(views, s, depth or [max(a[0], b[0]) + e, max(a[1], b[1]) + e, min(a[2] if len(a) > 2 else 0, b[2] if len(b) > 2 else 0) - e])

    def swept(self, i, bone, outline, z0, z1, role, tone, region, material, pen=None, filled=True, views=("elevation", "front", "plan"), holes=None, tol=0.0008, regions=None, tip=None):
        """🧱 A world outline in the sagittal plane drawn as a prism from z0 to z1: the true view in elevation, front and plan."""
        ring = [loc(bone, p)[:2] for p in outline]
        ring = thin_ring(ring, tol) if tol > 0 else ring
        if holes is None:
            s = {"kind": "poly", "id": i, "bone": bone, "role": role, "z": 0, "filled": filled, "tone": tone, "points": [[r4(p[0]), r4(p[1])] for p in ring], "closed": True}
        else:
            d = ""
            for g in [ring] + [thin_ring([loc(bone, p)[:2] for p in h], tol) for h in holes]:
                d += "M" + " L".join(f"{r4(p[0])} {r4(p[1])}" for p in g) + " Z "
            s = {"kind": "path", "id": i, "bone": bone, "role": role, "z": 0, "filled": filled, "tone": tone, "d": d.strip()}
        if regions is not None:
            s["regions"] = regions
        else:
            s["region"] = region
        s["material"] = material
        if pen is not None:
            s["penMm"] = pen
        s["solid"] = {"kind": "extruded", "sweep": "w", "z0": r4(z0), "z1": r4(z1)}
        if tip is not None:
            s["solid"]["zTip"] = [r4(tip[0]), r4(tip[1])]
        xs, ys = [p[0] for p in outline], [p[1] for p in outline]
        self.shape(views, s, [max(xs), max(ys), z0])

    def disc(self, i, bone, c, d, z0, z1, role, tone, region, material, pen=None, filled=True, views=("elevation", "front", "plan")):
        """🥏 A round part about a lateral axis (hub, shell, pivot) from z0 to z1: a circle in the side view, a rectangle from ahead and from above."""
        o = loc(bone, c)
        s = {"kind": "ellipse", "id": i, "bone": bone, "role": role, "z": 0, "filled": filled, "tone": tone, "region": region, "material": material, "cx": r4(o[0]), "cy": r4(o[1]), "rx": r4(d / 2), "ry": r4(d / 2)}
        if pen is not None:
            s["penMm"] = pen
        s["solid"] = {"kind": "extruded", "sweep": "w", "z0": r4(min(z0, z1)), "z1": r4(max(z0, z1))}
        self.shape(views, s, [c[0] + d / 2, c[1] + d / 2, min(z0, z1)])

    def plank(self, i, bone, a, b, d, zc, role, tone, region, material, pen=None, filled=True, views=("elevation", "front", "plan"), wide=None, d1=None):
        """🪵 A bar that lies in the sagittal plane (a quill, a bracket rod, a spring): its side outline pushed across by its own thickness."""
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy)
        nx, ny = -dy / L, dx / L
        h0, h1 = d / 2, (d if d1 is None else d1) / 2
        quad = [[a[0] + nx * h0, a[1] + ny * h0], [a[0] - nx * h0, a[1] - ny * h0], [b[0] - nx * h1, b[1] - ny * h1], [b[0] + nx * h1, b[1] + ny * h1]]
        w = (d if wide is None else wide) / 2
        self.swept(i, bone, quad, zc - w, zc + w, role, tone, region, material, pen=pen, filled=filled, views=views, tol=0.0)

    def wire(self, i, a, b, role, tone, region, material, pen, views=("elevation", "front", "plan")):
        """🧶 A thin rod of the deck bone (which never leaves the paper plane) as an open stroke in each view."""
        lines = {"elevation": [[a[0], a[1]], [b[0], b[1]]], "front": [[a[2], a[1]], [b[2], b[1]]], "plan": [[a[0], a[2]], [b[0], b[2]]]}
        for v in views:
            p, q = lines[v]
            if math.hypot(q[0] - p[0], q[1] - p[1]) < 0.004:
                continue
            self.flat(v, i, "deck", role, tone, region, material, [p, q], [max(a[0], b[0]), max(a[1], b[1]), min(a[2], b[2])], pen=pen, filled=False, closed=False)

    def sliver(self, i, bone, a, b, role, tone, region, material, pen, views=("elevation",)):
        """🪡 A spoke in the side view: a 2 mm sliver between flange and rim, carried with the wheel."""
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy)
        nx, ny = -dy / L * 0.001, dx / L * 0.001
        quad = [[a[0] + nx, a[1] + ny], [a[0] - nx, a[1] - ny], [b[0] - nx, b[1] - ny], [b[0] + nx, b[1] + ny]]
        self.swept(i, bone, quad, -0.001, 0.001, role, tone, region, material, pen=pen, filled=False, views=views, tol=0.0)

    def flat(self, view, i, bone, role, tone, region, material, points, depth, pen=None, filled=True, closed=True, regions=None, tol=0.0008):
        """🖊️ A flat outline of one view in that view's own frame (elevation x ahead / y up, front x left / y up, plan x ahead / y left) on a bone that never leaves the paper plane."""
        ring = thin_ring(points, tol) if closed else thin(points, tol)
        s = {"kind": "poly", "id": i, "bone": bone, "role": role, "z": 0, "filled": filled, "tone": tone, "points": [[r4(p[0]), r4(p[1])] for p in ring], "closed": closed}
        if regions is not None:
            s["regions"] = regions
        else:
            s["region"] = region
        s["material"] = material
        if pen is not None:
            s["penMm"] = pen
        self.shape((view,), s, depth)

    def drawing(self):
        """🖼️ The three shape lists in paint order: farthest first by the nearness of each view (elevation: the right side faces the viewer, front: ahead, plan: above); texture strokes and details of the same depth come after the bodies they lie on."""
        out = {}
        for view, key in (("elevation", lambda p: -p[2]), ("front", lambda p: p[0]), ("plan", lambda p: p[1])):
            rank = {"body": 0, "detail": 1, "glazing": 2, "texture": 3}
            order = sorted(range(len(self.shapes[view])), key=lambda k: (round(key(self.shapes[view][k]["_depth"]), 3), rank[self.shapes[view][k]["role"]], k))
            shapes = []
            for z, k in enumerate(order):
                s = {q: v for q, v in self.shapes[view][k].items() if q != "_depth"}
                s["z"] = z + 1
                shapes.append(s)
            ids = [s["id"] for s in shapes]
            assert len(ids) == len(set(ids)), (self.d, view, sorted(set(i for i in ids if ids.count(i) > 1)))
            out[view] = shapes
        return out

    def volume(self, decisions):
        for name in ("body", "thermal", "structure"):
            ids = [s["id"] for s in getattr(self, name)]
            assert len(ids) == len(set(ids)), (self.d, name, sorted(set(i for i in ids if ids.count(i) > 1)))
        return {"status": "authored", "denominator": self.d, "body": self.body, "thermal": self.thermal, "structure": self.structure, "decisions": decisions}


def shrink(solid, d):
    """🧅 The same solid drawn inwards by d on every side: the steel wall under the paint film."""
    s = copy.deepcopy(solid)
    k = s["kind"]
    if k == "loft":
        secs = s["sections"]
        for q in secs:
            q["rx"] = round(max(q["rx"] - d, 0.0003), 6)
            q["ry"] = round(max(q["ry"] - d, 0.0003), 6)
        for a, b in ((0, 1), (-1, -2)):
            pa, pb = secs[a]["at"], secs[b]["at"]
            L = norm(sub(pa, pb))
            f = min(0.5, d / L) if L > 0 else 0
            secs[a]["at"] = [round(pa[q] + (pb[q] - pa[q]) * f, 6) for q in range(3)]
    elif k == "ellipsoid":
        s["radii"] = [round(max(q - d, 0.0003), 6) for q in s["radii"]]
    elif k == "capsule":
        s["r0"] = round(max(s["r0"] - d, 0.0003), 6)
        s["r1"] = round(max(s["r1"] - d, 0.0003), 6)
    elif k == "box":
        s["size"] = [round(max(q - 2 * d, 0.0006), 6) for q in s["size"]]
        if "cornerRadius" in s:
            s["cornerRadius"] = round(max(0.0, min(s["cornerRadius"] - d, min(s["size"]) / 2)), 6)
    elif k == "extrude":
        s["profile"] = inset(s["profile"], d)
        if "holes" in s:
            s["holes"] = [inset(h, -d) for h in s["holes"]]
        lo, hi = min(s["from"], s["to"]), max(s["from"], s["to"])
        s["from"], s["to"] = round(lo + d, 6), round(hi - d, 6)
    elif k == "revolve":
        p = s["profile"]
        if len(p) == 2:
            L = p[1][0] - p[0][0]
            s["profile"] = [[round(p[0][0] + d, 6), round(p[0][1] - d, 6)], [round(p[1][0] - d, 6), round(p[1][1] - d, 6)]] if L > 4 * d else p
        elif p[0] == p[-1]:
            s["profile"] = inset(p[:-1], d)
            s["profile"].append(list(s["profile"][0]))
        else:
            q = inset([[p[0][0], 0.0]] + p + [[p[-1][0], 0.0]], d)
            s["profile"] = q[1:-1]
    else:
        raise ValueError(k)
    return s


def inset(ring, d):
    """📐 A simple polygon moved inwards by d (outwards for negative d): every edge is shifted along its inner normal and neighbouring edges are intersected."""
    n = len(ring)
    area = sum(ring[k][0] * ring[(k + 1) % n][1] - ring[(k + 1) % n][0] * ring[k][1] for k in range(n)) / 2
    sign = 1.0 if area > 0 else -1.0
    lines = []
    for k in range(n):
        a, b = ring[k], ring[(k + 1) % n]
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy) or 1e-12
        nx, ny = -dy / L * sign, dx / L * sign
        lines.append((a[0] + nx * d, a[1] + ny * d, dx / L, dy / L))
    out = []
    for k in range(n):
        x1, y1, ux, uy = lines[k - 1]
        x2, y2, vx, vy = lines[k]
        den = ux * vy - uy * vx
        if abs(den) < 1e-9:
            out.append([round(x2, 6), round(y2, 6)])
        else:
            t = ((x2 - x1) * vy - (y2 - y1) * vx) / den
            out.append([round(x1 + ux * t, 6), round(y1 + uy * t, 6)])
    return out
