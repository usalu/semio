"""🧰 Hand-modelling kit of the dairy cow: lofts and ellipsoids stated station by station in the rest frame of a bone or in the world (x ahead, h above the ground of the default pose, z to the left), written as schema solids in bone-local coordinates."""
import math

GROUND = -1.104


def norm(v):
    n = math.sqrt(sum(c * c for c in v))
    return [c / n for c in v]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def quat(x, y, z):
    """🧭 Unit quaternion [x, y, z, w] of the rotation whose matrix has the columns x, y, z."""
    m = [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2
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
    return [round(c / n, 6) for c in q]


def r5(v):
    return round(v, 5)


DOME = ((1.0, 0.34), (0.74, 0.66), (0.4, 0.9))


class Kit:
    """🐄 One list of solids of one scale; `meta` keeps, per solid id, the 2D shape group and the thermal region."""

    def __init__(self, bones):
        self.bones = {b["id"]: b for b in bones}
        self.rest = {}
        for b in bones:
            self.frame(b["id"])
        self.solids = []
        self.meta = {}
        self.thermal = None

    def frame(self, i):
        if i not in self.rest:
            b = self.bones[i]
            if b["parent"] is None:
                self.rest[i] = (b["x"], b["y"], b["angleDeg"])
            else:
                px, py, pa = self.frame(b["parent"])
                c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
                self.rest[i] = (px + b["x"] * c - b["y"] * s, py + b["x"] * s + b["y"] * c, pa + b["angleDeg"])
        return self.rest[i]

    def local(self, bone, x, h, z):
        """🌍 World point (x ahead, h above the default ground, z left) in the rest frame of `bone`."""
        ox, oy, a = self.rest[bone]
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        dx, dy = x - ox, h + GROUND - oy
        return [dx * c + dy * s, -dx * s + dy * c, z]

    def direction(self, bone, d):
        a = math.radians(self.rest[bone][2])
        c, s = math.cos(a), math.sin(a)
        return [d[0] * c + d[1] * s, -d[0] * s + d[1] * c, d[2]]

    def add(self, solid, group, region, **more):
        if solid["id"] in self.meta:
            raise SystemExit("duplicate solid " + solid["id"])
        self.solids.append(solid)
        self.meta[solid["id"]] = {"group": group, "region": region, **more}
        return solid

    def loft(self, i, bone, stations, look, group, region, world=False, axis=None, dome=(0, 0), caps=(True, True), exp=2.0, side=1, **more):
        """🧵 A loft: stations (a, b, c, r_sag, r_lat[, exponent[, tilt]]) (tilt: the section normal turned by that many degrees in the sagittal plane from the first axis) are (u, v, w) of the bone, or (x, h, z) of the world with `world`; `axis` is the fixed section normal (default: the bone axis u, or world x), "auto" the path tangent; r_sag lies in the sagittal plane, r_lat is lateral; `dome` rounds an end over that length; `side` -1 mirrors to the near (right) side."""
        pts = []
        for s in stations:
            p = self.local(bone, s[0], s[1], s[2] * side) if world else [s[0], s[1], s[2] * side]
            tilt = None
            if len(s) > 6:
                a = math.radians(s[6])
                tilt = norm(self.direction(bone, [math.cos(a), math.sin(a), 0]) if world else [math.cos(a), math.sin(a), 0])
            pts.append((p, s[3], s[4], s[5] if len(s) > 5 else exp, tilt))
        if axis is None:
            fixed = self.direction(bone, [1, 0, 0]) if world else [1, 0, 0]
        elif axis == "auto":
            fixed = None
        else:
            fixed = norm(self.direction(bone, list(axis)) if world else list(axis))
            fixed = [fixed[0], fixed[1], fixed[2] * side]

        def tangent(k):
            if pts[k][4] is not None:
                return pts[k][4]
            if fixed is not None:
                return fixed
            a, b = pts[max(0, k - 1)][0], pts[min(len(pts) - 1, k + 1)][0]
            return norm([b[j] - a[j] for j in range(3)])

        full = []
        if dome[0] > 0:
            p, rs, rl, e, _ = pts[0]
            t = tangent(0)
            full += [([p[j] - t[j] * dome[0] * d for j in range(3)], rs * f, rl * f, e, t) for d, f in DOME]
        full += [(p, rs, rl, e, tangent(k)) for k, (p, rs, rl, e, _) in enumerate(pts)]
        if dome[1] > 0:
            p, rs, rl, e, _ = pts[-1]
            t = tangent(len(pts) - 1)
            full += [([p[j] + t[j] * dome[1] * d for j in range(3)], rs * f, rl * f, e, t) for d, f in reversed(DOME)]
        sections = []
        for p, rs, rl, e, t in full:
            w = [0, 0, 1]
            lat = [w[j] - dot(w, t) * t[j] for j in range(3)]
            if math.sqrt(dot(lat, lat)) < 1e-6:
                lat = [0, 1, 0]
            lat = norm(lat)
            sag = cross(lat, t)
            section = {"at": [r5(c) for c in p], "rx": r5(rs), "ry": r5(rl), "rotation": quat(sag, lat, t)}
            if abs(e - 2) > 1e-9:
                section["exponent"] = e
            sections.append(section)
        return self.add({"id": i, "kind": "loft", "bone": bone, **look, "sections": sections, "capStart": caps[0], "capEnd": caps[1]}, group, region, **more)

    def ell(self, i, bone, at, radii, look, group, region, world=False, along=(1, 0, 0), side=1, **more):
        """🥚 An ellipsoid at `at` with the semi-axes radii = (along, sagittal, lateral); `along` is the direction of the first semi-axis."""
        p = self.local(bone, at[0], at[1], at[2] * side) if world else [at[0], at[1], at[2] * side]
        a = norm(self.direction(bone, list(along)) if world else list(along))
        a = [a[0], a[1], a[2] * side]
        w = [0, 0, 1]
        lat = [w[j] - dot(w, a) * a[j] for j in range(3)]
        lat = norm(lat) if math.sqrt(dot(lat, lat)) > 1e-6 else [0, 1, 0]
        sag = cross(lat, a)
        q = quat(a, sag, lat)
        transform = {"translation": [r5(c) for c in p]}
        if abs(q[3]) < 0.999999:
            transform["rotation"] = q
        return self.add({"id": i, "kind": "ellipsoid", "bone": bone, **look, "radii": [r5(r) for r in radii], "transform": transform}, group, region, **more)

    def cap(self, i, bone, p0, p1, r0, r1, look, group, region, world=False, side=1, **more):
        """💊 A capsule from p0 to p1."""
        a = self.local(bone, p0[0], p0[1], p0[2] * side) if world else [p0[0], p0[1], p0[2] * side]
        b = self.local(bone, p1[0], p1[1], p1[2] * side) if world else [p1[0], p1[1], p1[2] * side]
        return self.add({"id": i, "kind": "capsule", "bone": bone, **look, "p0": [r5(c) for c in a], "p1": [r5(c) for c in b], "r0": r5(r0), "r1": r5(r1)}, group, region, **more)


    def strap(self, i, bone, rows, look, group, region, thick=0.014, **more):
        """🩹 A coat patch as a strap laid on the skin: rows are (point, normal, across, half width, proud) in the world; the section is `half width` along `across` and `thick` along the normal, its centre sunk so that it stands `proud` of the skin."""
        sections = []
        for p, n, d, half, proud in rows:
            n, d = norm(n), norm(d)
            t = cross(d, n)
            c = [p[j] - n[j] * (thick - proud) for j in range(3)]
            at = self.local(bone, c[0], c[1], c[2])
            sections.append({"at": [r5(v) for v in at], "rx": r5(half), "ry": r5(thick), "rotation": quat(self.direction(bone, d), self.direction(bone, n), self.direction(bone, t)), "exponent": 4.0})
        return self.add({"id": i, "kind": "loft", "bone": bone, **look, "sections": sections, "capStart": True, "capEnd": True}, group, region, **more)

    def plate(self, i, bone, outline, z0, z1, look, group, region, side=1, **more):
        """🪵 A flat plate: a world outline [(x, h)] in the sagittal plane pushed laterally from z0 to z1."""
        pts = [[r5(c) for c in self.local(bone, x, h, 0)[:2]] for x, h in outline]
        lo, hi = sorted((z0 * side, z1 * side))
        return self.add({"id": i, "kind": "extrude", "bone": bone, **look, "profile": pts, "plane": "xy", "from": r5(lo), "to": r5(hi)}, group, region, **more)

    def hoop(self, i, bone, top, bottom, half_width, bar, tilt, look, group, region, sweep=None, **more):
        """⭕ A pair of ribs as an elliptic ring about the trunk axis: `top` = (x, h) of the rib heads at the spine, `bottom` the height of the lower end, `half_width` the lateral half width, `bar` = (along the axis, radial) half sizes of the rib, `tilt` degrees the lower end is swept back; `sweep` degrees leaves the ring open below."""
        r = (top[1] - bottom) / 2
        ti = math.radians(tilt)
        cx, ch = top[0] - r * math.sin(ti), top[1] - r * math.cos(ti)
        a, t = bar
        profile = [[-a, r - t], [a, r - t], [a, r + t], [-a, r + t], [-a, r - t]]
        ang = math.radians(-tilt - self.rest[bone][2])
        q = (0.0, 0.0, math.sin(ang / 2), math.cos(ang / 2))
        solid = {"id": i, "kind": "revolve", "bone": bone, **look, "profile": [[r5(p[0]), r5(p[1])] for p in profile], "axis": [1, 0, 0], "origin": [0, 0, 0], "lateralScale": r5(half_width / r)}
        if sweep is not None:
            solid["sweepRad"] = r5(math.radians(sweep))
            h = math.radians(sweep) / 4
            b = (math.sin(h), 0.0, 0.0, math.cos(h))
            q = (q[3] * b[0] + q[0] * b[3] + q[1] * b[2] - q[2] * b[1], q[3] * b[1] - q[0] * b[2] + q[1] * b[3] + q[2] * b[0], q[3] * b[2] + q[0] * b[1] - q[1] * b[0] + q[2] * b[3], q[3] * b[3] - q[0] * b[0] - q[1] * b[1] - q[2] * b[2])
        solid["transform"] = {"translation": [r5(c) for c in self.local(bone, cx, ch, 0)], "rotation": [round(c, 6) for c in q]}
        return self.add(solid, group, region, **more)

    def world(self, bone, u, v, w=0.0):
        """🗺️ The world point (x, h, z) of the bone-local point (u, v, w) at rest."""
        ox, oy, a = self.rest[bone]
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        return (ox + u * c - v * s, oy + u * s + v * c - GROUND, w)


def at_x(stations, x):
    """📏 The station of a world loft at x, linear between the stated stations (as the kernel joins them)."""
    for a, b in zip(stations, stations[1:]):
        lo, hi = (a, b) if a[0] <= b[0] else (b, a)
        if lo[0] - 1e-9 <= x <= hi[0] + 1e-9:
            f = 0 if hi[0] == lo[0] else (x - lo[0]) / (hi[0] - lo[0])
            e0 = lo[5] if len(lo) > 5 else 2.0
            e1 = hi[5] if len(hi) > 5 else 2.0
            return [x] + [lo[k] + (hi[k] - lo[k]) * f for k in range(1, 5)] + [e0 + (e1 - e0) * f]
    raise SystemExit("x %s outside the loft" % x)


def cut(stations, x, tilt, h_ref=1.0):
    """🔪 The section of a world loft in the plane through (x, h_ref) whose normal is turned by `tilt` degrees from the x axis: the joint at which two coloured pieces of one mass meet."""
    t = math.tan(math.radians(tilt))
    xc = x
    for _ in range(4):
        xc = x - (at_x(stations, xc)[1] - h_ref) * t
    s = at_x(stations, xc)
    return (xc, s[1], s[2], s[3] / math.cos(math.radians(tilt)), s[4], s[5], tilt)


def pieces(stations, cuts):
    """🍰 A world loft cut into consecutive pieces at `cuts` = [(x, tilt)]; each piece keeps the stated stations that lie clear of its joints, so neighbours share their joint section exactly."""
    lo, hi = stations[0][0], stations[-1][0]
    inner = [c for c in cuts if lo < c[0] < hi]
    bounds = [None] + inner + [None]
    out = []
    for a, b in zip(bounds, bounds[1:]):
        x0 = lo if a is None else a[0]
        x1 = hi if b is None else b[0]
        rows = [] if a is None else [cut(stations, *a)]
        for s in stations:
            clear0 = True if a is None else s[0] > a[0] + (0.085 if a[1] else 0.02)
            clear1 = True if b is None else s[0] < b[0] - (0.085 if b[1] else 0.02)
            if (a is None and s[0] <= x0 + 1e-9) or (b is None and s[0] >= x1 - 1e-9) or (clear0 and clear1 and x0 < s[0] < x1):
                rows.append(tuple(s) + ((2.0,) if len(s) < 6 else ()))
        if b is not None:
            rows.append(cut(stations, *b))
        out.append(((x0 + x1) / 2, a is None, b is None, rows))
    return out
