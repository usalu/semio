"""🧰 Round 3 excavator authoring kit: hand-stated outlines (lines, arcs, cubic curves, filleted polygons) that serve both the 2D drawing (as a path) and the 3D solid (as its flattened ring), the rig frames, and one writer that emits a part as its solid and as its three drawings so that drawing and model are one definition (ruling E1)."""
import json
import math


def r4(x):
    v = round(float(x), 4)
    return 0.0 if v == 0 else v


def rv(p):
    return [r4(c) for c in p]


def fmt(x):
    s = ("%.4f" % x).rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


class Path:
    """✒️ A closed or open outline in local metres, y up: M, L, A (circular arc, counter-clockwise when ccw) and C, the same commands the drawing stores."""

    def __init__(self):
        self.cmd = []
        self.closed = False

    def M(self, x, y):
        self.cmd.append(("M", x, y))
        return self

    def L(self, x, y):
        self.cmd.append(("L", x, y))
        return self

    def A(self, r, x, y, ccw=True, large=False):
        self.cmd.append(("A", r, large, ccw, x, y))
        return self

    def C(self, x1, y1, x2, y2, x, y):
        self.cmd.append(("C", x1, y1, x2, y2, x, y))
        return self

    def Z(self):
        self.closed = True
        return self

    def d(self):
        out = []
        for c in self.cmd:
            if c[0] == "Z":
                out.append("Z")
            elif c[0] in ("M", "L"):
                out.append("%s %s,%s" % (c[0], fmt(c[1]), fmt(c[2])))
            elif c[0] == "A":
                out.append("A %s %s 0 %d %d %s,%s" % (fmt(c[1]), fmt(c[1]), 1 if c[2] else 0, 1 if c[3] else 0, fmt(c[4]), fmt(c[5])))
            else:
                out.append("C %s,%s %s,%s %s,%s" % tuple(fmt(v) for v in c[1:]))
        return " ".join(out) + (" Z" if self.closed else "")

    def pts(self, chord=0.0015):
        out = []
        cur = None
        for c in self.cmd:
            if c[0] == "Z":
                continue
            if c[0] in ("M", "L"):
                cur = (c[1], c[2])
                out.append(cur)
            elif c[0] == "A":
                r, large, ccw, x, y = c[1:]
                out += arc_points(cur, (x, y), r, ccw, large, chord)
                cur = (x, y)
            else:
                p0, p1, p2, p3 = cur, (c[1], c[2]), (c[3], c[4]), (c[5], c[6])
                n = max(6, int(math.ceil((math.dist(p0, p1) + math.dist(p1, p2) + math.dist(p2, p3)) / 0.06)))
                for i in range(1, n + 1):
                    t = i / n
                    out.append(tuple((1 - t) ** 3 * p0[k] + 3 * (1 - t) ** 2 * t * p1[k] + 3 * (1 - t) * t * t * p2[k] + t ** 3 * p3[k] for k in (0, 1)))
                cur = p3
        clean = []
        for p in out:
            if not clean or math.dist(p, clean[-1]) > 1e-6:
                clean.append(p)
        if self.closed and len(clean) > 1 and math.dist(clean[0], clean[-1]) < 1e-6:
            clean.pop()
        return clean

    def moved(self, f):
        """The same outline with every point mapped by f (a similarity: radii are kept)."""
        q = Path()
        q.closed = self.closed
        for c in self.cmd:
            if c[0] in ("M", "L"):
                q.cmd.append((c[0],) + tuple(f((c[1], c[2]))))
            elif c[0] == "A":
                q.cmd.append(("A", c[1], c[2], c[3]) + tuple(f((c[4], c[5]))))
            else:
                q.cmd.append(("C",) + tuple(f((c[1], c[2]))) + tuple(f((c[3], c[4]))) + tuple(f((c[5], c[6]))))
        return q


def arc_points(a, b, r, ccw, large, chord):
    dx, dy = (a[0] - b[0]) / 2, (a[1] - b[1]) / 2
    lam = (dx * dx + dy * dy) / (r * r)
    if lam > 1:
        r *= math.sqrt(lam)
    num = r * r * r * r - r * r * dy * dy - r * r * dx * dx
    den = r * r * dy * dy + r * r * dx * dx
    f = (-1 if large == ccw else 1) * math.sqrt(max(0.0, num / den)) if den else 0.0
    cxp, cyp = f * dy, -f * dx
    cx, cy = cxp + (a[0] + b[0]) / 2, cyp + (a[1] + b[1]) / 2
    t1 = math.atan2(dy - cyp, dx - cxp)
    t2 = math.atan2(-dy - cyp, -dx - cxp)
    dt = t2 - t1
    if not ccw and dt > 0:
        dt -= 2 * math.pi
    elif ccw and dt < 0:
        dt += 2 * math.pi
    step = 2 * math.acos(max(-1.0, 1 - min(chord / r, 1.0)))
    n = max(2, int(math.ceil(abs(dt) / step)))
    return [(cx + r * math.cos(t1 + dt * i / n), cy + r * math.sin(t1 + dt * i / n)) for i in range(1, n)] + [b]


def poly(points):
    """A closed polygon; a third number on a corner is its fillet radius."""
    n = len(points)
    q = Path()
    first = True
    for i in range(n):
        p = points[i]
        r = p[2] if len(p) > 2 else 0
        if r <= 0:
            (q.M if first else q.L)(p[0], p[1])
            first = False
            continue
        a, b = points[i - 1], points[(i + 1) % n]
        ua = (a[0] - p[0], a[1] - p[1])
        ub = (b[0] - p[0], b[1] - p[1])
        la, lb = math.hypot(*ua), math.hypot(*ub)
        ua, ub = (ua[0] / la, ua[1] / la), (ub[0] / lb, ub[1] / lb)
        half = math.acos(max(-1.0, min(1.0, ua[0] * ub[0] + ua[1] * ub[1]))) / 2
        t = min(r / math.tan(half), la * 0.499, lb * 0.499)
        r = t * math.tan(half)
        s = (p[0] + ua[0] * t, p[1] + ua[1] * t)
        e = (p[0] + ub[0] * t, p[1] + ub[1] * t)
        ccw = (ua[0] * ub[1] - ua[1] * ub[0]) < 0
        (q.M if first else q.L)(*s)
        first = False
        q.A(r, e[0], e[1], ccw=ccw)
    return q.Z()


def rect(x0, y0, x1, y1, r=0):
    return poly([(x0, y0, r), (x1, y0, r), (x1, y1, r), (x0, y1, r)])


def circle(cx, cy, r):
    return Path().M(cx + r, cy).A(r, cx - r, cy).A(r, cx + r, cy).Z()


def line(*pts):
    """An open stroke; with more than two points it runs there and back, so that the closed outline a projected solid draws has no closing chord."""
    q = Path().M(*pts[0])
    for p in list(pts[1:]) + (list(pts[-2::-1]) if len(pts) > 2 else []):
        q.L(*p)
    return q


def strip(points, t):
    """A band of thickness t on the left of an open polyline (a plate following a curve), as a closed polygon."""
    n = len(points)
    off = []
    for i in range(n):
        a, b = points[max(i - 1, 0)], points[min(i + 1, n - 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        l = math.hypot(dx, dy) or 1
        off.append((points[i][0] - dy / l * t, points[i][1] + dx / l * t))
    return list(points) + off[::-1]


def band(points, width):
    """A drawn line of real width along an open polyline, as a closed outline: a stroke that is a prism in space (a seam, a rail, a pipe) is drawn in every view, which an open stroke swept along the viewing direction is not."""
    h = width / 2
    return Path_of(strip(strip(points, -h)[len(points):][::-1], width))


def bands(segments, width):
    """Several straight lines of real width in one outline (louvres, ribs)."""
    q = Path()
    for a, b in segments:
        ring = strip(strip([a, b], -width / 2)[2:][::-1], width)
        q.M(*ring[0])
        for pt in ring[1:]:
            q.L(*pt)
        q.cmd.append(("Z",))
    return q


def Path_of(ring):
    q = Path().M(*ring[0])
    for pt in ring[1:]:
        q.L(*pt)
    return q.Z()


def above(ring, v):
    """The part of a closed ring at or above the height v (a half-plane cut), as a ring."""
    out = []
    n = len(ring)
    for i in range(n):
        a, b = ring[i], ring[(i + 1) % n]
        if a[1] >= v:
            out.append(a)
        if (a[1] >= v) != (b[1] >= v):
            t = (v - a[1]) / (b[1] - a[1])
            out.append((a[0] + (b[0] - a[0]) * t, v))
    return out


def chain(ring, u0, u1, upper=True):
    """The upper (or lower) edge of a ring between two stations along u, as an open polyline in the direction of increasing u."""
    n = len(ring)
    area = sum(ring[i - 1][0] * ring[i][1] - ring[i][0] * ring[i - 1][1] for i in range(n))
    pts = ring if (area > 0) != upper else ring[::-1]
    start = min(range(n), key=lambda i: pts[i][0])
    seq = [pts[(start + i) % n] for i in range(n)]
    end = max(range(n), key=lambda i: seq[i][0])
    edge = seq[: end + 1]

    def at(u):
        for a, b in zip(edge, edge[1:]):
            if a[0] <= u <= b[0] and b[0] > a[0]:
                return (u, a[1] + (b[1] - a[1]) * (u - a[0]) / (b[0] - a[0]))
        return None

    return [at(u0)] + [q for q in edge if u0 < q[0] < u1] + [at(u1)]


def thin(points, tol):
    """The polyline without the points that lie within tol of the chord of their kept neighbours (Douglas-Peucker)."""
    if len(points) < 3:
        return list(points)
    a, b = points[0], points[-1]
    l = math.dist(a, b) or 1
    k, far = 0, 0.0
    for i in range(1, len(points) - 1):
        d = abs((b[0] - a[0]) * (a[1] - points[i][1]) - (a[0] - points[i][0]) * (b[1] - a[1])) / l
        if d > far:
            k, far = i, d
    if far <= tol:
        return [a, b]
    return thin(points[: k + 1], tol)[:-1] + thin(points[k:], tol)


def quat_z(deg):
    a = math.radians(deg) / 2
    return [0.0, 0.0, r4(math.sin(a)), r4(math.cos(a))]


def quat_y(deg):
    a = math.radians(deg) / 2
    return [0.0, r4(math.sin(a)), 0.0, r4(math.cos(a))]


def quat_axes(x, y, z):
    m00, m10, m20 = x
    m01, m11, m21 = y
    m02, m12, m22 = z
    t = m00 + m11 + m22
    if t > 0:
        s = math.sqrt(t + 1) * 2
        q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s]
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1 + m00 - m11 - m22) * 2
        q = [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    elif m11 > m22:
        s = math.sqrt(1 + m11 - m00 - m22) * 2
        q = [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s]
    else:
        s = math.sqrt(1 + m22 - m00 - m11) * 2
        q = [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s]
    n = math.sqrt(sum(c * c for c in q))
    return [round(c / n, 6) for c in q]


class Rig:
    """🦴 Planar frames of the bones at rest or for pose offsets: origin and angle of every bone in the world (x ahead, y up)."""

    def __init__(self, bones):
        self.bones = {b["id"]: b for b in bones}
        self.order = bones

    def frames(self, offsets=None):
        offsets = offsets or {}
        out = {}
        for b in self.order:
            off = offsets.get(b["id"], {})
            x, y, a = b["x"] + off.get("dx", 0), b["y"] + off.get("dy", 0), b["angleDeg"] + off.get("angleDeg", 0)
            if b["parent"] is None:
                out[b["id"]] = (x, y, a)
            else:
                px, py, pa = out[b["parent"]]
                c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
                out[b["id"]] = (px + x * c - y * s, py + x * s + y * c, pa + a)
        return out


def to_world(frame, p):
    c, s = math.cos(math.radians(frame[2])), math.sin(math.radians(frame[2]))
    return (frame[0] + p[0] * c - p[1] * s, frame[1] + p[0] * s + p[1] * c)


ROLE_RANK = {"body": 0, "detail": 1, "texture": 2, "glazing": 3, "ground": 4}


class Scale:
    """🧱 One scale under construction: the three solid lists and the three drawings; every writer below adds a part to the lists and views it is asked for."""

    def __init__(self, denominator, rig, rest):
        self.d = denominator
        self.feature = 0.00015 * denominator
        self.rig = rig
        self.rest = rest
        self.body, self.thermal, self.structure = [], [], []
        self.views = {"elevation": [], "front": [], "plan": []}
        self.decisions = []
        self.order = 0
        self.near = {}

    def _solid(self, sol, to, region=None, layer=None, member=None, part=None, anchor=None, thfaces=None):
        """Adds the solid to the lists named in `to`; a body solid with a region or a member is also referenced from the thermal or structure list."""
        base = {k: sol[k] for k in ("id", "kind", "bone", "material", "role", "tone")}
        rest = {k: v for k, v in sol.items() if k not in base}
        if "body" in to:
            self.body.append({**base, **rest})
            if region and "thermal" not in to:
                self.th(sol["id"], region, faces=thfaces)
            if member and "structure" not in to:
                self.st(sol["id"], member, part or "member")
        if "thermal" in to:
            own = {k: v for k, v in rest.items() if k not in ("texture",)}
            self.thermal.append({**base, "region": region, **({"layer": layer} if layer else {}), **own, **({"sectionFaces": thfaces} if thfaces else {})})
        if "structure" in to:
            own = {k: v for k, v in rest.items() if k not in ("texture",)}
            self.structure.append({**base, "member": member, "part": part or "member", **({"anchor": anchor} if anchor else {}), **own})

    def th(self, of, region, id=None, faces=None):
        e = {"id": id or "th-" + of, "of": of, "region": region}
        if faces:
            e["sectionFaces"] = faces
        self.thermal.append(e)

    def st(self, of, member, part="member", id=None):
        self.structure.append({"id": id or part + "-" + of, "of": of, "member": member, "part": part})

    def _world(self, bone, pts3):
        f = self.rest[bone]
        return [to_world(f, (p[0], p[1])) + (p[2],) for p in pts3]

    def _shape(self, view, shape, pts3, parent, sub):
        w = self._world(shape["bone"], pts3)
        near = {"elevation": -min(p[2] for p in w), "front": max(p[0] for p in w), "plan": max(p[1] for p in w)}[view]
        if parent is not None and (view, parent) in self.near:
            near = self.near[(view, parent)]
        else:
            self.near[(view, shape["id"])] = near
        self.order += 1
        self.views[view].append((round(near, 3), sub if parent is not None else 0, ROLE_RANK[shape["role"]] if parent is None else 0, self.order, shape))

    def draw(self, id, bone, path, sweep, a, b, *, role="body", tone=0.5, material=None, region=None, regions=None, filled=True, views="efp", parent=None, sub=1, pen=None, ids=None):
        """A drawn part: the outline in the plane across `sweep` (w: side outline (u, v); v: plan outline (u, w)) spanning a..b along it, placed in each named view as a projected solid."""
        geo = path if isinstance(path, dict) else {"kind": "path", "d": path.d()}
        pts = path.pts() if isinstance(path, Path) else [(geo["cx"] - geo["rx"], geo["cy"] - geo["ry"]), (geo["cx"] + geo["rx"], geo["cy"] + geo["ry"])]
        if sweep == "w":
            pts3 = [(p[0], p[1], z) for p in pts for z in (a, b)]
        else:
            pts3 = [(p[0], z, p[1]) for p in pts for z in (a, b)]
        for key, view in (("e", "elevation"), ("f", "front"), ("p", "plan")):
            if key not in views:
                continue
            s = {"id": (ids or {}).get(key, id), "bone": bone, "role": role, "z": 0, "filled": filled, "tone": tone, **geo}
            if region:
                s["region"] = region
            if regions:
                s["regions"] = regions
            if material:
                s["material"] = material
            if pen:
                s["penMm"] = pen
            s["solid"] = {"kind": "extruded", "sweep": sweep, "z0": r4(a), "z1": r4(b)}
            self._shape(view, s, pts3, parent, sub)

    def flat(self, view, id, bone, geo, pts3, *, role="body", tone=0.5, material=None, region=None, filled=True, parent=None, sub=1, pen=None):
        """A shape drawn directly in the coordinates of one view (root bone only)."""
        s = {"id": id, "bone": bone, "role": role, "z": 0, "filled": filled, "tone": tone, **(geo if isinstance(geo, dict) else {"kind": "path", "d": geo.d()})}
        if region:
            s["region"] = region
        if material:
            s["material"] = material
        if pen:
            s["penMm"] = pen
        self._shape(view, s, pts3, parent, sub)

    def bar(self, id, bone, p0, p1, dia, *, role="detail", tone=0.4, material=None, region=None, views="efp", parent=None, sub=1, ids=None):
        """A drawn round bar between two points (u, v, w) of one bone: the `member` shape of the drawing."""
        for key, view in (("e", "elevation"), ("f", "front"), ("p", "plan")):
            if key not in views:
                continue
            s = {"id": (ids or {}).get(key, id), "bone": bone, "role": role, "z": 0, "filled": True, "tone": tone, "kind": "member", "from": {"x": r4(p0[0]), "y": r4(p0[1]), "z": r4(p0[2])}, "to": {"x": r4(p1[0]), "y": r4(p1[1]), "z": r4(p1[2])}, "section": {"kind": "circle", "d": r4(dia)}}
            if region:
                s["region"] = region
            if material:
                s["material"] = material
            r = dia / 2
            self._shape(view, s, [(p[0], p[1] + dv, p[2] + dw) for p in (p0, p1) for dv in (-r, r) for dw in (-r, r)], parent, sub)

    def ext(self, id, bone, ring, plane, a, b, *, material, role="body", tone=0.5, holes=None, taper=None, repeat=None, to="body", **kw):
        """An extruded solid: ring in (u, v) for plane w, (u, w) for plane v, (v, w) for plane u; a..b along the named axis."""
        if plane == "w":
            prof, pl = [rv(p) for p in ring], "xy"
            hs = [[rv(p) for p in h] for h in holes] if holes else None
        elif plane == "v":
            prof, pl = [rv((p[1], p[0])) for p in ring], "zx"
            hs = [[rv((p[1], p[0])) for p in h] for h in holes] if holes else None
        else:
            prof, pl = [rv(p) for p in ring], "yz"
            hs = [[rv(p) for p in h] for h in holes] if holes else None
        sol = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": prof, "plane": pl, "from": r4(min(a, b)), "to": r4(max(a, b))}
        if hs:
            sol["holes"] = hs
        if taper:
            sol["taperTo"] = rv(taper)
        self._finish(sol, repeat, to, kw)

    def _finish(self, sol, repeat, to, kw):
        flags = {k: kw.pop(k) for k in ("slender", "texture", "sectionFaces", "tags", "envelope") if k in kw}
        if repeat:
            sol["repeat"] = repeat
        sol.update(flags)
        self._solid(sol, to, **kw)

    def box(self, id, bone, centre, size, *, material, role="body", tone=0.5, corner=0, rz=0, rot=None, repeat=None, to="body", **kw):
        sol = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": rv(size)}
        if corner:
            sol["cornerRadius"] = r4(corner)
        tr = {"translation": rv(centre)}
        if rot:
            tr["rotation"] = rot
        elif rz:
            tr["rotation"] = quat_z(rz)
        sol["transform"] = tr
        self._finish(sol, repeat, to, kw)

    def cyl(self, id, bone, p0, p1, radius, *, material, role="detail", tone=0.4, r1=None, bore=0, repeat=None, to="body", **kw):
        """A flat-ended cylinder, frustum or (with bore) tube between two points."""
        ax = [p1[k] - p0[k] for k in range(3)]
        l = math.sqrt(sum(c * c for c in ax))
        r1 = radius if r1 is None else r1
        prof = [[0, bore], [0, radius], [l, r1], [l, bore], [0, bore]] if bore else [[0, 0], [0, radius], [l, r1], [l, 0]]
        sol = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [rv(p) for p in prof], "axis": rv([c / l for c in ax]), "origin": rv(p0)}
        self._finish(sol, repeat, to, kw)

    def rev(self, id, bone, origin, axis, profile, *, material, role="detail", tone=0.4, sweep=None, repeat=None, to="body", **kw):
        sol = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [rv(p) for p in profile], "axis": rv(axis), "origin": rv(origin)}
        if sweep:
            sol["sweepRad"] = r4(sweep)
        self._finish(sol, repeat, to, kw)

    def cap(self, id, bone, p0, p1, r0, *, material, role="detail", tone=0.4, r1=None, repeat=None, to="body", **kw):
        sol = {"id": id, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": rv(p0), "p1": rv(p1), "r0": r4(r0), "r1": r4(r0 if r1 is None else r1)}
        self._finish(sol, repeat, to, kw)

    def ell(self, id, bone, centre, radii, *, material, role="detail", tone=0.4, repeat=None, to="body", **kw):
        sol = {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": role, "tone": tone, "radii": rv(radii), "transform": {"translation": rv(centre)}}
        self._finish(sol, repeat, to, kw)

    def loft(self, id, bone, sections, *, material, role="body", tone=0.5, caps=(True, True), repeat=None, to="body", **kw):
        """Sections (at, rx, ry[, exponent[, rotation]]) joined in order."""
        secs = []
        for s in sections:
            e = {"at": rv(s[0]), "rx": r4(s[1]), "ry": r4(s[2])}
            if len(s) > 3 and s[3]:
                e["exponent"] = s[3]
            if len(s) > 4 and s[4]:
                e["rotation"] = s[4]
            secs.append(e)
        sol = {"id": id, "kind": "loft", "bone": bone, "material": material, "role": role, "tone": tone, "sections": secs, "capStart": caps[0], "capEnd": caps[1]}
        self._finish(sol, repeat, to, kw)

    def tube(self, id, bone, points, radius, **kw):
        """A bent round tube through the points (a loft of circles)."""
        self.loft(id, bone, [(p, radius, radius) for p in points], **kw)

    def decide(self, en, de):
        self.decisions.append({"en": en, "de": de})

    def geometry(self):
        out = {}
        for view, items in self.views.items():
            items.sort(key=lambda t: (t[0], t[1], t[2], t[3]))
            shapes = []
            for i, t in enumerate(items):
                s = dict(t[4])
                s["z"] = i + 1
                shapes.append(s)
            ids = [s["id"] for s in shapes]
            assert len(ids) == len(set(ids)), (self.d, view, sorted(i for i in ids if ids.count(i) > 1))
            out[view] = shapes
        return out

    def volume(self):
        for name in ("body", "thermal", "structure"):
            ids = [s["id"] for s in getattr(self, name)]
            assert len(ids) == len(set(ids)), (self.d, name, sorted(set(i for i in ids if ids.count(i) > 1)))
        return {"status": "authored", "denominator": self.d, "body": self.body, "thermal": self.thermal, "structure": self.structure, "decisions": self.decisions}


def dump_asset(doc, path):
    with open(path, "w", encoding="utf8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
