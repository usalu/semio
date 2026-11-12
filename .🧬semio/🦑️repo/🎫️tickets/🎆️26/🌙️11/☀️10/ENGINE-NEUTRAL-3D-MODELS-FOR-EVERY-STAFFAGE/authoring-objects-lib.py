"""Authoring helpers for the objects volumes: solids are given in staffage world coordinates at rest (x ahead, y up, z lateral left)
and written in the bone-local u, v, w frame of the bone they follow. Every bone frame of the objects is a rotation about z.
Usage: from authoring-objects-lib (importlib), `A = Asset("bicycle-stand", defaults)`, add solids, `A.save(decisions)`."""
import json, math, os, sys

T = os.path.dirname(os.path.abspath(__file__))
FRAMES = json.load(open(os.path.join(T, "🗑️generated", "authoring-objects", "frames.json")))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects"


def r5(x):
    v = round(x, 5)
    return 0.0 if v == 0 else v


def rv(v):
    return [r5(x) for x in v]


def rotz(v, a):
    c, s = math.cos(a), math.sin(a)
    return [c * v[0] - s * v[1], s * v[0] + c * v[1], v[2]]


def qmat(x, y, z):
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
    return [r5(c / n) for c in q]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(sum(c * c for c in a))
    return [c / n for c in a]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def qz(a):
    return [0, 0, r5(math.sin(a / 2)), r5(math.cos(a / 2))]


def snap(a):
    k = round(a / (math.pi / 2))
    return k if abs(a - k * math.pi / 2) < 1e-6 else None


class Asset:
    def __init__(self, slug, material, tone, region=None, role="body"):
        self.slug = slug
        self.fr = FRAMES[slug]
        self.solids = []
        self.d = dict(material=material, tone=tone, region=region, role=role)
        self.cat = json.load(open(os.path.join(CATALOG, slug, "🔣️.json")))["asset"]

    def phi(self, bone):
        u = self.fr[bone]["frame"]["u"]
        return math.atan2(u[1], u[0])

    def pt(self, bone, p):
        o = self.fr[bone]["frame"]["o"]
        return rotz(sub(p, o), -self.phi(bone))

    def dr(self, bone, d):
        return rotz(d, -self.phi(bone))

    def meta(self, sid, bone, kw):
        d = dict(self.d)
        for k in ("material", "tone", "region", "role", "maxDenominator", "regions"):
            if k in kw:
                d[k] = kw.pop(k)
        out = {"id": sid, "kind": None, "bone": bone, "material": d["material"], "role": d["role"], "tone": d["tone"]}
        if d.get("region"):
            out["region"] = d["region"]
        if d.get("regions"):
            out["regions"] = d["regions"]
        if d.get("maxDenominator"):
            out["maxDenominator"] = d["maxDenominator"]
        return out

    def finish(self, s, kind, body, translation=None, rotation=None, scale=None, repeat=None):
        s["kind"] = kind
        s.update(body)
        t = {}
        if scale is not None and any(abs(c - 1) > 1e-9 for c in scale):
            t["scale"] = rv(scale)
        if rotation is not None:
            t["rotation"] = rotation
        if translation is not None and any(abs(c) > 1e-9 for c in translation):
            t["translation"] = rv(translation)
        if t:
            s["transform"] = t
        if repeat:
            s["repeat"] = repeat
        self.solids.append(s)
        return s

    def place(self, bone, c, rot_z, sizes):
        """Local translation, rotation quaternion (or None) and axis-permuted sizes for a body oriented by world rotation rot_z (rad) about z."""
        a = rot_z - self.phi(bone)
        k = snap(a)
        sx, sy, sz = sizes
        if k is not None:
            if k % 2 != 0:
                sx, sy = sy, sx
            return self.pt(bone, c), None, [sx, sy, sz]
        return self.pt(bone, c), qz(a), [sx, sy, sz]

    def box(self, sid, bone, c, size, rot=0.0, corner=0.0, repeat=None, **kw):
        s = self.meta(sid, bone, kw)
        t, q, sz = self.place(bone, c, math.radians(rot), size)
        body = {"size": rv(sz)}
        if corner:
            body["cornerRadius"] = r5(corner)
        return self.finish(s, "box", body, t, q, repeat=repeat)

    def ell(self, sid, bone, c, radii, rot=0.0, repeat=None, **kw):
        s = self.meta(sid, bone, kw)
        t, q, sz = self.place(bone, c, math.radians(rot), radii)
        return self.finish(s, "ellipsoid", {"radii": rv(sz)}, t, q, repeat=repeat)

    def cap(self, sid, bone, p0, p1, r0, r1=None, repeat=None, **kw):
        s = self.meta(sid, bone, kw)
        body = {"p0": rv(self.pt(bone, p0)), "p1": rv(self.pt(bone, p1)), "r0": r5(r0), "r1": r5(r0 if r1 is None else r1)}
        return self.finish(s, "capsule", body, repeat=repeat)

    def rev(self, sid, bone, base, profile, axis="y", stretch=None, sweep=None, rot=0.0, local_axis=None, local_q=None, repeat=None, **kw):
        """Revolve about the world axis ("x", "y" or "z") through `base`; profile [axial, radial]; stretch = {world axis: factor} widens the cross-section along that axis."""
        s = self.meta(sid, bone, kw)
        unitv = {"x": [1, 0, 0], "y": [0, 1, 0], "z": [0, 0, 1]}
        la = [round(c) for c in self.dr(bone, unitv[axis])] if local_axis is None else local_axis
        scale = [1.0, 1.0, 1.0]
        for wa, f in (stretch or {}).items():
            if snap(self.phi(bone)) is None:
                raise ValueError("stretched revolve in a skew bone")
            lv = [abs(round(c)) for c in self.dr(bone, unitv[wa])]
            scale[lv.index(1)] = f
        body = {"profile": [rv(p) for p in profile], "axis": la, "origin": [0, 0, 0]}
        if sweep:
            body["sweepRad"] = r5(sweep)
        q = local_q
        if rot:
            a = math.radians(rot) - self.phi(bone)
            q = qz(a)
        return self.finish(s, "revolve", body, self.pt(bone, base), q, scale, repeat)

    def ext(self, sid, bone, pts, z0, z1, holes=None, repeat=None, **kw):
        """Extrusion of a world (x, y) polygon along world z from z0 to z1."""
        s = self.meta(sid, bone, kw)
        ph = self.phi(bone)

        def loc(p):
            q = self.pt(bone, [p[0], p[1], 0])
            return [r5(q[0]), r5(q[1])]

        body = {"profile": [loc(p) for p in pts], "plane": "xy", "from": r5(z0), "to": r5(z1)}
        if holes:
            body["holes"] = [[loc(p) for p in h] for h in holes]
        return self.finish(s, "extrude", body, repeat=repeat)

    def ext_yz(self, sid, bone, pts, x0, x1, holes=None, repeat=None, **kw):
        """Extrusion of a world (y, z) polygon along world x from x0 to x1; bone frame must be unrotated."""
        if abs(self.phi(bone)) > 1e-9:
            raise ValueError("yz extrusion needs an unrotated bone")
        s = self.meta(sid, bone, kw)
        o = self.fr[bone]["frame"]["o"]
        body = {"profile": [[r5(p[0] - o[1]), r5(p[1])] for p in pts], "plane": "yz", "from": r5(x0 - o[0]), "to": r5(x1 - o[0])}
        if holes:
            body["holes"] = [[[r5(p[0] - o[1]), r5(p[1])] for p in h] for h in holes]
        return self.finish(s, "extrude", body, repeat=repeat)

    def ring(self, sid, bone, c, y0, y1, ro0, ro1, t0, n=40, repeat=None, **kw):
        """Hollow conical ring about the vertical axis through world (x, z) = c: outer radius ro0 at y0 and ro1 at y1, wall t0 at y0 (the wall scales with the taper); bone frame must be unrotated."""
        if abs(self.phi(bone)) > 1e-9:
            raise ValueError("ring needs an unrotated bone")
        s = self.meta(sid, bone, kw)
        o = self.fr[bone]["frame"]["o"]

        def circ(rad):
            return [[r5(rad * math.sin(2 * math.pi * i / n)), r5(rad * math.cos(2 * math.pi * i / n))] for i in range(n)]

        body = {"profile": circ(ro0), "holes": [circ(ro0 - t0)], "plane": "zx", "from": r5(y0 - o[1]), "to": r5(y1 - o[1])}
        if abs(ro1 - ro0) > 1e-9:
            body["taperTo"] = [1, r5(ro1 / ro0)]
        return self.finish(s, "extrude", body, [c[0] - o[0], 0, c[1] - o[2]], None, None, repeat)

    def plan(self, sid, bone, pts, y0, y1, holes=None, repeat=None, **kw):
        """Extrusion of a world plan polygon (given as (x, z) points) along world y from y0 to y1; bone frame must be unrotated."""
        if abs(self.phi(bone)) > 1e-9:
            raise ValueError("plan extrusion needs an unrotated bone")
        s = self.meta(sid, bone, kw)
        o = self.fr[bone]["frame"]["o"]

        def loc(p):
            return [r5(p[1]), r5(p[0] - o[0])]

        body = {"profile": [loc(p) for p in pts], "plane": "zx", "from": r5(y0 - o[1]), "to": r5(y1 - o[1])}
        if holes:
            body["holes"] = [[loc(p) for p in h] for h in holes]
        return self.finish(s, "extrude", body, repeat=repeat)

    def loft(self, sid, bone, secs, first="z", cs=True, ce=True, smooth=False, repeat=None, **kw):
        """Loft through sections (world centre c, rx, ry, exponent); rx runs along world axis `first` ("x" or "z") projected perpendicular to the path, ry along the third axis."""
        s = self.meta(sid, bone, kw)
        ph = self.phi(bone)
        if smooth or len(secs) > 2:
            a0, a1 = secs[0]["c"], secs[1]["c"]
            tl = sub(secs[-1]["c"], secs[0]["c"])
            t0 = sub(a1, a0)
            dotp = sum(tl[k] * t0[k] for k in range(3))
            if dotp < 0:
                secs = secs[::-1]
            elif dotp < 0.2 * math.sqrt(sum(c * c for c in tl)) * math.sqrt(sum(c * c for c in t0)):
                raise ValueError(f"loft {sid}: path doubles back, split it")
        centres = [sc["c"] for sc in secs]
        out = []
        for i, sc in enumerate(secs):
            a = centres[min(i, len(secs) - 2)]
            b = centres[min(i + 1, len(secs) - 1)]
            if smooth and 0 < i < len(secs) - 1:
                a, b = centres[i - 1], centres[i + 1]
            z = unit(sub(b, a))
            ax = {"x": [1, 0, 0], "z": [0, 0, 1], "y": [0, 1, 0]}[first]
            d = sum(ax[k] * z[k] for k in range(3))
            x = unit([ax[k] - d * z[k] for k in range(3)])
            y = cross(z, x)
            lx, ly, lz = (self.dr(bone, v) for v in (x, y, z))
            item = {"at": rv(self.pt(bone, sc["c"])), "rx": r5(sc["rx"]), "ry": r5(sc["ry"])}
            if sc.get("e", 2) != 2:
                item["exponent"] = sc["e"]
            item["rotation"] = qmat(lx, ly, lz)
            out.append(item)
        return self.finish(s, "loft", {"sections": out, "capStart": cs, "capEnd": ce}, repeat=repeat)

    def save(self, decisions):
        ids = [s["id"] for s in self.solids]
        assert len(ids) == len(set(ids)), "duplicate ids"
        doc = {"volume": {"status": "authored", "solids": self.solids, "decisions": [{"en": e, "de": d} for e, d in decisions]}}
        out = os.path.join(T, "volumes", "objects")
        os.makedirs(out, exist_ok=True)
        path = os.path.join(out, self.slug + ".json")
        with open(path, "w") as f:
            json.dump(doc, f, ensure_ascii=False, indent=1)
        print(self.slug, len(self.solids), "solids ->", path)


def circle(ra, rr, n=24):
    """Closed circle profile [axial, radial] centred at (0, ra) with radius rr (a torus section when revolved)."""
    pts = [[rr * math.sin(2 * math.pi * i / n), ra + rr * math.cos(2 * math.pi * i / n)] for i in range(n)]
    return pts + [pts[0]]


def arc(c, rad, a0, a1, n):
    """World points on a circle in the xy plane (centre c, z kept) from angle a0 to a1 degrees, n steps."""
    return [[c[0] + rad * math.cos(math.radians(a0 + (a1 - a0) * i / n)), c[1] + rad * math.sin(math.radians(a0 + (a1 - a0) * i / n)), c[2]] for i in range(n + 1)]


def bez(p0, p1, p2, p3, n):
    """Points of a cubic Bezier (2D) in n steps."""
    out = []
    for i in range(n + 1):
        t = i / n
        a, b, c, d = (1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t * t, t ** 3
        out.append([a * p0[0] + b * p1[0] + c * p2[0] + d * p3[0], a * p0[1] + b * p1[1] + c * p2[1] + d * p3[1]])
    return out


def interp(pts, key, val, other):
    """Linear interpolation on a polyline: value of coordinate `other` where coordinate `key` equals val."""
    for a, b in zip(pts, pts[1:]):
        lo, hi = sorted((a[key], b[key]))
        if lo - 1e-12 <= val <= hi + 1e-12 and abs(b[key] - a[key]) > 1e-12:
            f = (val - a[key]) / (b[key] - a[key])
            return a[other] + f * (b[other] - a[other])
    raise ValueError("out of range")
