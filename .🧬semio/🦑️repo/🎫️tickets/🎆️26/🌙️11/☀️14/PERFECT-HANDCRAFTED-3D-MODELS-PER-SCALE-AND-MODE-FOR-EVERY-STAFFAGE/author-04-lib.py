import json
import math
import os

TICKET = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(TICKET, "🗑️generated", "author-04")
LISTS = ("body", "thermal", "structure")


def r5(x):
    v = round(x, 5)
    return 0.0 if v == 0 else v


def rv(v):
    return [r5(c) for c in v]


def sub(a, b):
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def add(a, b):
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]]


def mul(a, s):
    return [a[0] * s, a[1] * s, a[2] * s]


def lerp(a, b, t):
    return [a[k] + (b[k] - a[k]) * t for k in range(3)]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def length(a):
    return math.sqrt(dot(a, a))


def unit(a):
    n = length(a) or 1.0
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


def frame_along(a, b, up=(0, 1, 0)):
    x = unit(sub(b, a))
    u = list(up)
    y = sub(u, mul(x, dot(u, x)))
    if dot(y, y) < 1e-8:
        u = [0, 0, 1]
        y = sub(u, mul(x, dot(u, x)))
    y = unit(y)
    z = cross(x, y)
    return [x, y, z]


class Volume:
    def __init__(self, slug, category="machinery"):
        self.slug = slug
        self.category = category
        self.frames = json.load(open(os.environ.get("FRAMES") or os.path.join(GEN, f"{slug}.frames.json")))
        self.data = {50: {"body": [], "thermal": [], "structure": [], "decisions": []},
                     100: {"body": [], "thermal": [], "structure": [], "decisions": []},
                     200: {"body": [], "thermal": [], "structure": [], "decisions": []},
                     500: {"body": [], "thermal": [], "structure": [], "decisions": []}}
        self.scale = 50
        self.list = "body"

    def at(self, scale, lst=None):
        self.scale = scale
        if lst:
            self.list = lst
        return self

    def to(self, lst):
        self.list = lst
        return self

    def L(self, bone, p):
        f = self.frames[bone]
        d = sub(p, f["o"])
        return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]

    def D(self, bone, d):
        f = self.frames[bone]
        return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]

    def basis(self, bone, rz=0.0, axes=None):
        ex, ey, ez = axes if axes is not None else rot_z(rz)
        return self.D(bone, ex), self.D(bone, ey), self.D(bone, ez)

    def put(self, d):
        order = ["id", "of", "kind", "bone", "material", "role", "tone", "region", "layer", "member", "part"]
        out = {}
        for k in order:
            if k in d:
                out[k] = d[k]
        for k, v in d.items():
            if k not in out:
                out[k] = v
        lst = self.data[self.scale][self.list]
        assert all(s["id"] != out["id"] for s in lst), f"duplicate {out['id']} s{self.scale} {self.list}"
        lst.append(out)
        return out

    def ref(self, id, of, **m):
        d = {"id": id, "of": of}
        d.update(m)
        return self.put(d)

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
        return self.put(d)

    def bar(self, id, bone, a, b, w, h, up=(0, 1, 0), cr=None, grow=0.0, **m):
        """Box from world a to b: length along a-b, w across `up`-normal (horizontal), h along `up`."""
        ax = frame_along(a, b, up)
        ln = length(sub(b, a)) + grow
        c = lerp(a, b, 0.5)
        return self.box(id, bone, c, [ln, h, w], axes=ax, cr=cr, **m)

    def ell(self, id, bone, c, radii, rz=0.0, axes=None, rep=None, **m):
        bx, by, bz = self.basis(bone, rz, axes)
        d = self._meta(id, bone, "ellipsoid", m)
        d["radii"] = rv(radii)
        tr = {"translation": rv(self.L(bone, c))}
        if not is_identity(bx, by, bz):
            tr["rotation"] = rv(quat(bx, by, bz))
        d["transform"] = tr
        d.update(self._repeat(bone, rep))
        return self.put(d)

    def cap(self, id, bone, a, b, r0, r1=None, rep=None, **m):
        d = self._meta(id, bone, "capsule", m)
        d["p0"] = rv(self.L(bone, a))
        d["p1"] = rv(self.L(bone, b))
        d["r0"] = r5(r0)
        d["r1"] = r5(r0 if r1 is None else r1)
        d.update(self._repeat(bone, rep))
        return self.put(d)

    def rev(self, id, bone, o, axis, profile, sweep=None, rep=None, **m):
        d = self._meta(id, bone, "revolve", m)
        d["profile"] = [rv(p) for p in profile]
        d["axis"] = rv(unit(self.D(bone, axis)))
        d["origin"] = rv(self.L(bone, o))
        if sweep is not None:
            d["sweepRad"] = r5(sweep)
        d.update(self._repeat(bone, rep))
        return self.put(d)

    def cyl(self, id, bone, a, b, r0, r1=None, rep=None, **m):
        ln = length(sub(b, a))
        r1 = r0 if r1 is None else r1
        return self.rev(id, bone, a, sub(b, a), [[0, r0], [ln, r1]], rep=rep, **m)

    def tube(self, id, bone, a, b, ro, ri, **m):
        ln = length(sub(b, a))
        return self.rev(id, bone, a, sub(b, a), [[0, ro], [ln, ro], [ln, ri], [0, ri], [0, ro]], **m)

    def loft(self, id, bone, secs, hint=(0, 0, 1), caps=(True, True), rep=None, **m):
        d = self._meta(id, bone, "loft", m)
        cs = [self.L(bone, s[0]) for s in secs]
        hintl = self.D(bone, list(hint))
        sections = []
        for i, s in enumerate(secs):
            a = cs[min(i, len(cs) - 2)]
            b = cs[min(i + 1, len(cs) - 1)]
            if 0 < i < len(cs) - 1:
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
        return self.put(d)

    def ext(self, id, bone, view, pts, t0, t1, holes=None, taper=None, rep=None, **m):
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
        return self.put(d)

    def decide(self, scale, en, de):
        self.data[scale]["decisions"].append({"en": en, "de": de})

    def write(self, unavailable=()):
        out = {}
        for s in (50, 100, 200, 500):
            if s in unavailable:
                out[str(s)] = {"status": "unavailable", "denominator": s}
            else:
                dd = self.data[s]
                out[str(s)] = {"status": "authored", "denominator": s, "body": dd["body"], "thermal": dd["thermal"], "structure": dd["structure"], "decisions": dd["decisions"]}
        path = os.environ.get("OUT") or os.path.join(TICKET, "volumes", self.category, f"{self.slug}.json")
        os.makedirs(os.path.dirname(path), exist_ok=True)
        json.dump({"volumes": out}, open(path, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        print(self.slug, {s: [len(self.data[s][l]) for l in LISTS] for s in (50, 100, 200, 500) if s not in unavailable}, "->", path)
