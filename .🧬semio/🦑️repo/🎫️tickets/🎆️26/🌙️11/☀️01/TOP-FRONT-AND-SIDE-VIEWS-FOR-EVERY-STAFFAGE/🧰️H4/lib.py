import copy
import glob
import json
import math

S = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage"


def path(a):
    return glob.glob(S + "\\🗂️catalog\\*\\" + a + "\\🔣️.json")[0]


def load(a):
    with open(path(a), encoding="utf-8") as f:
        return json.load(f)


def save(a, doc):
    import sys
    sys.path.insert(0, "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW")
    from atomic_json import write_json_atomic
    write_json_atomic(path(a), doc)


class Rig:
    """Rest-pose forward kinematics of an asset (the same frame the front and plan shapes are authored in)."""

    def __init__(self, asset):
        self.asset = asset
        self.w = {}
        self.bones = {b["id"]: b for b in asset["rig"]["bones"]}
        for b in asset["rig"]["bones"]:
            if b["parent"] is None:
                px, py, pa = 0.0, 0.0, 0.0
            else:
                px, py, pa, _ = self.w[b["parent"]]
            c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            self.w[b["id"]] = (px + c * b["x"] - s * b["y"], py + s * b["x"] + c * b["y"], pa + b["angleDeg"], b["lengthM"])

    def o(self, b):
        return self.w[b][0], self.w[b][1]

    def t(self, b):
        x, y, a, l = self.w[b]
        return x + l * math.cos(math.radians(a)), y + l * math.sin(math.radians(a))

    def ox(self, b):
        return self.w[b][0]

    def oy(self, b):
        return self.w[b][1]

    def tx(self, b):
        return self.t(b)[0]

    def ty(self, b):
        return self.t(b)[1]

    def at(self, b, s):
        """Rest height at the fraction `s` of the bone from its origin (0) to its tip (1)."""
        return self.oy(b) + s * (self.ty(b) - self.oy(b))

    def anchor(self, a):
        x, y, ang, _ = self.w[a["bone"]]
        c, s = math.cos(math.radians(ang)), math.sin(math.radians(ang))
        return x + c * a["x"] - s * a["y"], y + s * a["x"] + c * a["y"]

    def ground(self):
        ys = [self.anchor(a)[1] for a in self.asset["anchors"] if a.get("kind") == "contact"]
        return min(ys)


def r4(v):
    return round(v + 0.0, 4)


def pts(points):
    return [[r4(x), r4(y)] for x, y in points]


def mir(points):
    return [(-x, y) for x, y in points]


def chaikin(points, it=2, closed=True):
    p = list(points)
    for _ in range(it):
        q = []
        n = len(p)
        rng = range(n) if closed else range(n - 1)
        if not closed:
            q.append(p[0])
        for i in rng:
            a, b = p[i], p[(i + 1) % n]
            q.append((0.75 * a[0] + 0.25 * b[0], 0.75 * a[1] + 0.25 * b[1]))
            q.append((0.25 * a[0] + 0.75 * b[0], 0.25 * a[1] + 0.75 * b[1]))
        if not closed:
            q.append(p[-1])
        p = q
    return p


def arc(cx, cy, rx, ry, a0, a1, n=8):
    return [(cx + rx * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + ry * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


class Sty:
    """Looks the tone, region, hatch and material of a part up in the elevation of the same representation."""

    def __init__(self, rep):
        self.elev = rep["geometry"]["elevation"]
        self.hatches = {h["id"] for h in rep["hatches"]}

    def _cands(self, region):
        out = []
        for s in self.elev:
            if not s.get("filled"):
                continue
            if s.get("region") == region or region in s.get("regions", []):
                out.append(s)
        return out

    def get(self, region, bone=None, hatch=True, tone=None):
        c = self._cands(region)
        if bone is not None:
            pref = [s for s in c if s["bone"] in (bone, bone[:-2] + "-r", bone[:-2] + "-l")]
            c = pref or c
        c = sorted(c, key=lambda s: (0 if s["bone"].endswith("-r") or not s["bone"].endswith("-l") else 1, -s["z"] if s["role"] == "body" else 0))
        if not c:
            return {"tone": 0.6 if tone is None else tone, "region": region}
        s = c[0]
        d = {"tone": s["tone"] if tone is None else tone, "region": region}
        if hatch and s.get("hatch") in self.hatches:
            d["hatch"] = s["hatch"]
        if s.get("material"):
            d["material"] = s["material"]
        return d

    def merged(self, regions, hatch=False):
        tones = [self.get(r, hatch=False)["tone"] for r in regions]
        return {"tone": round(sum(tones) / len(tones), 3), "regions": list(regions)}


def shape(geom, id, bone, role, z, st, filled=True, **extra):
    d = dict(geom)
    d.update({"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": st["tone"]})
    for k in ("region", "regions", "material"):
        if k in st:
            d[k] = st[k]
    if filled and "hatch" in st:
        d["hatch"] = st["hatch"]
    d.update(extra)
    return d


def poly(points, closed=True, follow=False):
    g = {"kind": "poly", "points": pts(points), "closed": closed}
    if follow:
        g["followTip"] = True
    return g


def ell(cx, cy, rx, ry, rot=0):
    g = {"kind": "ellipse", "cx": r4(cx), "cy": r4(cy), "rx": r4(rx), "ry": r4(ry)}
    if rot:
        g["rotationDeg"] = rot
    return g


def cap(x0, y0, x1, y1, r0, r1, follow=False):
    g = {"kind": "capsule", "x0": r4(x0), "y0": r4(y0), "x1": r4(x1), "y1": r4(y1), "r0": r4(r0), "r1": r4(r1)}
    if follow:
        g["followTip"] = True
    return g


def rect(x, y, w, h, r=0, follow=False):
    g = {"kind": "rect", "x": r4(x), "y": r4(y), "w": r4(w), "h": r4(h)}
    if r:
        g["r"] = r4(r)
    if follow:
        g["followTip"] = True
    return g


SCALES = ["50", "100", "200", "500"]
LV = {"500": 0, "200": 1, "100": 2, "50": 3}
