"""🔩 Authoring helpers for the machinery volumes: solids are written in world rest coordinates (x ahead, y up, z left, metres) and converted to the bone-local u, v, w frame of the rest pose.

Usage in a per-asset script: `from lib import Volume` (run with the ticket folder as working directory and `authoring-machinery-lib.py` imported through importlib, see any `volumes/machinery/<slug>.py`).
The frames come from `🗑️generated/authoring-machinery/<slug>.frames.json` written by `authoring-machinery-inspect.ts`.
"""
import json
import math
import os

TICKET = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(TICKET, "🗑️generated", "authoring-machinery")


def r5(x):
    v = round(x, 5)
    return 0.0 if v == 0 else v


def rv(v):
    return [r5(c) for c in v]


def sub(a, b):
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(dot(a, a)) or 1.0
    return [a[0] / n, a[1] / n, a[2] / n]


def quat(x, y, z):
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
    return [c / n for c in q]


def is_identity(x, y, z, eps=1e-9):
    return all(abs(a - b) < eps for a, b in zip(x + y + z, [1, 0, 0, 0, 1, 0, 0, 0, 1]))


def rot_z(deg):
    a = math.radians(deg)
    return [math.cos(a), math.sin(a), 0], [-math.sin(a), math.cos(a), 0], [0, 0, 1]


class Volume:
    def __init__(self, slug):
        self.slug = slug
        self.frames = json.load(open(os.path.join(GEN, f"{slug}.frames.json")))
        self.solids = []
        self.decisions = []

    def L(self, bone, p):
        f = self.frames[bone]
        d = sub(p, f["o"])
        return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]

    def D(self, bone, d):
        f = self.frames[bone]
        return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]

    def W(self, bone, p):
        f = self.frames[bone]
        return [f["o"][k] + p[0] * f["u"][k] + p[1] * f["v"][k] + p[2] * f["w"][k] for k in range(3)]

    def angle(self, bone):
        f = self.frames[bone]
        return math.degrees(math.atan2(f["u"][1], f["u"][0]))

    def basis(self, bone, rz=0.0, axes=None):
        ex, ey, ez = axes if axes is not None else rot_z(rz)
        return self.D(bone, ex), self.D(bone, ey), self.D(bone, ez)

    def put(self, sol, meta):
        order = ["id", "kind", "bone", "material", "role", "tone", "region", "regions", "maxDenominator"]
        meta = dict(meta)
        out = {}
        for k in order:
            if k in sol:
                out[k] = sol[k]
            elif k in meta:
                out[k] = meta.pop(k)
        for k, v in sol.items():
            if k not in out:
                out[k] = v
        for k, v in meta.items():
            if k not in out:
                out[k] = v
        self.solids.append(out)
        return out

    def _meta(self, id, bone, kind, m):
        d = {"id": id, "kind": kind, "bone": bone}
        d.update(m)
        return d

    def _repeat(self, bone, rep):
        if rep is None:
            return {}
        out = {"count": rep["count"]}
        if "step" in rep:
            out["translation"] = rv(self.D(bone, rep["step"]))
        if "angle" in rep:
            out["rotationRad"] = r5(rep["angle"])
            out["rotationAxis"] = rv(self.D(bone, rep.get("axis", [0, 0, 1])))
            if "origin" in rep:
                out["origin"] = rv(self.L(bone, rep["origin"]))
        return {"repeat": out}

    def box(self, id, bone, c, size, rz=0.0, axes=None, cr=None, rep=None, **m):
        bx, by, bz = self.basis(bone, rz, axes)
        d = self._meta(id, bone, "box", m)
        d["size"] = rv(size)
        if cr:
            d["cornerRadius"] = r5(cr)
        tr = {"translation": rv(self.L(bone, c))}
        if not is_identity(bx, by, bz):
            tr["rotation"] = rv(quat(bx, by, bz))
        d["transform"] = tr
        d.update(self._repeat(bone, rep))
        return self.put(d, {})

    def ell(self, id, bone, c, radii, rz=0.0, axes=None, rep=None, **m):
        bx, by, bz = self.basis(bone, rz, axes)
        d = self._meta(id, bone, "ellipsoid", m)
        d["radii"] = rv(radii)
        tr = {"translation": rv(self.L(bone, c))}
        if not is_identity(bx, by, bz):
            tr["rotation"] = rv(quat(bx, by, bz))
        d["transform"] = tr
        d.update(self._repeat(bone, rep))
        return self.put(d, {})

    def cap(self, id, bone, a, b, r0, r1=None, rep=None, **m):
        d = self._meta(id, bone, "capsule", m)
        d["p0"] = rv(self.L(bone, a))
        d["p1"] = rv(self.L(bone, b))
        d["r0"] = r5(r0)
        d["r1"] = r5(r0 if r1 is None else r1)
        d.update(self._repeat(bone, rep))
        return self.put(d, {})

    def rev(self, id, bone, o, axis, profile, sweep=None, rep=None, **m):
        d = self._meta(id, bone, "revolve", m)
        d["profile"] = [rv(p) for p in profile]
        d["axis"] = rv(unit(self.D(bone, axis)))
        d["origin"] = rv(self.L(bone, o))
        if sweep is not None:
            d["sweepRad"] = r5(sweep)
        d.update(self._repeat(bone, rep))
        return self.put(d, {})

    def cyl(self, id, bone, a, b, r0, r1=None, rep=None, **m):
        """Flat-ended cylinder or frustum from world point a (radius r0) to b (radius r1) as a revolve about the segment."""
        length = math.sqrt(dot(sub(b, a), sub(b, a)))
        r1 = r0 if r1 is None else r1
        return self.rev(id, bone, a, sub(b, a), [[0, r0], [length, r1]], rep=rep, **m)

    def loft(self, id, bone, secs, hint=(0, 0, 1), caps=(True, True), rep=None, **m):
        """secs: list of (centre world, rx, ry[, exponent]); rx along `hint` (world, projected perpendicular to the travel), ry along travel x hint-axis."""
        d = self._meta(id, bone, "loft", m)
        cs = [self.L(bone, s[0]) for s in secs]
        hintl = self.D(bone, list(hint))
        sections = []
        for i, s in enumerate(secs):
            a = cs[min(i, len(cs) - 2)]
            b = cs[min(i + 1, len(cs) - 1)]
            if i > 0 and i < len(cs) - 1:
                a = cs[i - 1]
            z = unit(sub(b, a))
            x = sub(hintl, [c * dot(hintl, z) for c in z])
            if dot(x, x) < 1e-8:
                alt = self.D(bone, [0, 1, 0])
                x = sub(alt, [c * dot(alt, z) for c in z])
            x = unit(x)
            y = cross(z, x)
            sec = {"at": rv(cs[i]), "rx": r5(s[1]), "ry": r5(s[2])}
            if len(s) > 3:
                sec["exponent"] = r5(s[3])
            sec["rotation"] = rv(quat(x, y, z))
            sections.append(sec)
        d["sections"] = sections
        d["capStart"] = caps[0]
        d["capEnd"] = caps[1]
        d.update(self._repeat(bone, rep))
        return self.put(d, {})

    def ext(self, id, bone, view, pts, t0, t1, holes=None, taper=None, rep=None, **m):
        """view side: pts (x, y) extruded along z t0..t1; plan: pts (x, z) along y; front: pts (y, z) along x (plane yz)."""
        d = self._meta(id, bone, "extrude", m)
        plane = {"side": "xy", "plan": "zx", "front": "yz"}[view]
        f = self.frames[bone]
        bx, by, bz = self.basis(bone)
        lo, hi = min(t0, t1), max(t0, t1)
        o = f["o"]
        if is_identity(bx, by, bz):
            if view == "side":
                conv = lambda p: [p[0] - o[0], p[1] - o[1]]
                a, b = lo - o[2], hi - o[2]
            elif view == "plan":
                conv = lambda p: [p[1] - o[2], p[0] - o[0]]
                a, b = lo - o[1], hi - o[1]
            else:
                conv = lambda p: [p[0] - o[1], p[1] - o[2]]
                a, b = lo - o[0], hi - o[0]
            d["profile"] = [rv(conv(p)) for p in pts]
            if holes:
                d["holes"] = [[rv(conv(p)) for p in h] for h in holes]
            d["plane"] = plane
            d["from"], d["to"] = r5(a), r5(b)
        else:
            assert view == "side" and abs(f["w"][2] - 1) < 1e-9, f"extrude on rotated bone {bone} supports side view only"
            conv = lambda p: self.L(bone, [p[0], p[1], 0])[:2]
            d["profile"] = [rv(conv(p)) for p in pts]
            if holes:
                d["holes"] = [[rv(conv(p)) for p in h] for h in holes]
            d["plane"] = "xy"
            d["from"], d["to"] = r5(lo - o[2]), r5(hi - o[2])
        if taper:
            d["taperTo"] = rv(taper)
        d.update(self._repeat(bone, rep))
        return self.put(d, {})

    def write(self, path):
        data = {"volume": {"status": "authored", "solids": self.solids, "decisions": self.decisions}}
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf8") as fh:
            json.dump(data, fh, ensure_ascii=False, indent=1)
        print(self.slug, len(self.solids), "solids ->", path)

    def decide(self, en, de):
        self.decisions.append({"en": en, "de": de})
