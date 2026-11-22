"""🦯 Chisel of the author of `humans/person-with-white-cane`: rest frames of the girdle bones, hand-stated stations in rest world coordinates [ahead, up, left] carried into the frame of their bone, mirrored pairs, candidate in and out.

See ./rework-brief.md, ./rig-lateral-rest.md (1.1 order of the rest placement) and ./rework/assets/humans--person-with-white-cane.md.
"""
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
SLUG = "person-with-white-cane"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-with-white-cane/🔣️.json"
SOURCE = os.path.join(T, "volumes", "humans", SLUG + ".json")
OUT = os.path.join(T, "🗑️generated", "rework-" + SLUG)
CANDIDATE = os.path.join(OUT, "wcane", "🔣️.json")


def r5(x):
    return round(x, 5) + 0.0


def add(a, b, k=1.0):
    return [a[i] + b[i] * k for i in range(3)]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def mul(a, k):
    return [c * k for c in a]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(dot(a, a)) or 1.0
    return [c / n for c in a]


def mz(p):
    return [p[0], p[1], -p[2]]


def quat(ex, ey):
    x = unit(ex)
    y = unit(sub(ey, mul(x, dot(ey, x))))
    z = cross(x, y)
    m00, m10, m20, m01, m11, m21, m02, m12, m22 = x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
        q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4]
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1 + m00 - m11 - m22) * 2
        q = [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    elif m11 > m22:
        s = math.sqrt(1 + m11 - m00 - m22) * 2
        q = [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s]
    else:
        s = math.sqrt(1 + m22 - m00 - m11) * 2
        q = [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s]
    n = math.sqrt(sum(c * c for c in q))
    return [r5(c / n) for c in q]


class Frame:
    """🧭 Rest frame of a bone: origin and the axes u (along), v (normal), w (joint axis) in world coordinates."""

    def __init__(self, o, u, v, w):
        self.o, self.u, self.v, self.w = o, u, v, w

    def L(self, p):
        d = sub(p, self.o)
        return [dot(d, self.u), dot(d, self.v), dot(d, self.w)]

    def D(self, d):
        return [dot(d, self.u), dot(d, self.v), dot(d, self.w)]

    def mirrored(self):
        return Frame(mz(self.o), mz(self.u), mz(self.v), mul(mz(self.w), -1))


def lateral(origin, tip, w0=(-1.0, 0.0, 0.0)):
    """🦴 Frame of a bone from `origin` to `tip` whose joint axis is `w0` made perpendicular to it (as `rig-rest.ts --solve`)."""
    u = unit(sub(tip, origin))
    w = unit(sub(w0, mul(u, dot(w0, u))))
    return Frame(list(origin), u, cross(w, u), w)


def placement(parent, child):
    """📐 Rest placement of `child` under `parent`: R = Rw(angle) · Rv(abduction) · Ru(twist) with the senses of rig-lateral-rest.md 1.1."""
    o = parent.L(child.o)
    m = [[dot(a, b) for b in (child.u, child.v, child.w)] for a in (parent.u, parent.v, parent.w)]
    b = math.asin(max(-1.0, min(1.0, m[2][0])))
    a = math.atan2(m[1][0], m[0][0])
    ca, sa, cb, sb = math.cos(a), math.sin(a), math.cos(b), math.sin(b)
    rz = [[ca, -sa, 0], [sa, ca, 0], [0, 0, 1]]
    ry = [[cb, 0, -sb], [0, 1, 0], [sb, 0, cb]]
    r = [[sum(rz[i][k] * ry[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    p = [[sum(r[k][i] * m[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    t = math.atan2(p[1][2], p[1][1])
    return {"x": round(o[0], 5), "y": round(o[1], 5), "z": round(o[2], 5), "angleDeg": round(math.degrees(a), 4), "abductionDeg": round(math.degrees(b), 4), "twistDeg": round(math.degrees(t), 4)}


BONE = {"material": "bone", "role": "body", "tone": 0.62}


def st(c, ex, ey, rx, ry, e=2.0):
    return {"c": list(c), "ex": unit(ex), "ey": unit(ey), "rx": rx, "ry": ry, "e": e}


def flip(s):
    return {**s, "c": mz(s["c"]), "ex": mul(mz(s["ex"]), -1), "ey": mz(s["ey"])}


def tube(path, radii, up=(0.0, 1.0, 0.0)):
    """🪈 Stations along a hand-stated path: the section follows the path, `ry` lies as near to `up` as the path allows; radii `(rx, ry)` per point."""
    out = []
    for i, at in enumerate(path):
        t = unit(sub(path[min(i + 1, len(path) - 1)], path[max(i - 1, 0)]))
        ey = sub(up, mul(t, dot(up, t)))
        if dot(ey, ey) < 1e-9:
            ey = sub([0, 0, 1], mul(t, t[2]))
        ey = unit(ey)
        r = radii[min(i, len(radii) - 1)]
        rx, ry = (r, r) if isinstance(r, (int, float)) else r
        out.append(st(at, cross(t, ey), ey, rx, ry))
    return out


def plate(rows, e=2.6):
    """🥞 Stations of a plate stacked upwards: per row `(y, (x, z) of one edge, (x, z) of the other edge, half thickness)`."""
    out = []
    for y, a, b, h in rows:
        pa, pb = [a[0], y, a[1]], [b[0], y, b[1]]
        ex = unit(sub(pb, pa))
        out.append(st(mul(add(pa, pb), 0.5), ex, [ex[2], 0, -ex[0]], math.dist(pa, pb) / 2, h, e))
    return out


def loft(f, sid, bone, stations, extra=None, base=BONE):
    secs = [{"at": [r5(c) for c in f.L(s["c"])], "rx": r5(s["rx"]), "ry": r5(s["ry"]), "exponent": r5(s["e"]), "rotation": quat(f.D(s["ex"]), f.D(s["ey"]))} for s in stations]
    return {"id": sid, "kind": "loft", "bone": bone, **base, "sections": secs, "capStart": True, "capEnd": True, **(extra or {})}


def capsule(f, sid, bone, p0, p1, r0, r1, extra=None, base=BONE):
    return {"id": sid, "kind": "capsule", "bone": bone, **base, "p0": [r5(c) for c in f.L(p0)], "p1": [r5(c) for c in f.L(p1)], "r0": r5(r0), "r1": r5(r1), **(extra or {})}


def ellipsoid(f, sid, bone, radii, at, extra=None, base=BONE, ax=((1, 0, 0), (0, 1, 0))):
    return {"id": sid, "kind": "ellipsoid", "bone": bone, **base, "radii": [r5(c) for c in radii], "transform": {"rotation": quat(f.D(ax[0]), f.D(ax[1])), "translation": [r5(c) for c in f.L(at)]}, **(extra or {})}


def member(bone, kind="member", **more):
    return {"member": bone, "part": kind, **more}


def load():
    with open(CATALOG, encoding="utf8") as h:
        return json.load(h)


def save(path, doc, like=None):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf8") as h:
        h.write((json.dumps(doc, ensure_ascii=False, separators=(",", ":")) if like is None else json.dumps(doc, ensure_ascii=False, indent=2)) + "\n")
