from author_37_lib import *
import math

PS = Ctx("rear-frame", "paint-steel", None, "body", 0.45)
WS = Ctx("bucket", "wear-steel", "bucket", "body", 0.3)


def area2(p):
    return sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p))) / 2


def ccw(p):
    p = [list(x) for x in p]
    return p if area2(p) > 0 else p[::-1]


def inset(poly, d):
    """Mitre inset of a simple polygon by d (inward)."""
    p = ccw(poly)
    n = len(p)
    out = []
    for i in range(n):
        a, b, c = p[i - 1], p[i], p[(i + 1) % n]
        e1 = (b[0] - a[0], b[1] - a[1])
        e2 = (c[0] - b[0], c[1] - b[1])
        l1, l2 = math.hypot(*e1), math.hypot(*e2)
        n1 = (-e1[1] / l1, e1[0] / l1)
        n2 = (-e2[1] / l2, e2[0] / l2)
        bx, by = n1[0] + n2[0], n1[1] + n2[1]
        k = 1 + n1[0] * n2[0] + n1[1] * n2[1]
        out.append([b[0] + d * bx / k, b[1] + d * by / k])
    return out


def strip(path, d, side=1):
    """Polygon of a polyline thickened by d on one side (side=+1 left of travel, -1 right), mitre joins."""
    n = len(path)
    off = []
    for i in range(n):
        a = path[i - 1] if i > 0 else None
        b = path[i]
        c = path[i + 1] if i + 1 < n else None
        ns = []
        for (p, q) in ((a, b), (b, c)):
            if p is None or q is None:
                continue
            e = (q[0] - p[0], q[1] - p[1])
            l = math.hypot(*e)
            ns.append((-e[1] / l * side, e[0] / l * side))
        if len(ns) == 1:
            off.append([b[0] + d * ns[0][0], b[1] + d * ns[0][1]])
        else:
            bx, by = ns[0][0] + ns[1][0], ns[0][1] + ns[1][1]
            k = 1 + ns[0][0] * ns[1][0] + ns[0][1] * ns[1][1]
            off.append([b[0] + d * bx / k, b[1] + d * by / k])
    return [list(p) for p in path] + off[::-1]


def beam(prefix, ctx, poly, w0, w1, wall, mirror=True, plate=None, **kw):
    """Closed box beam from a (u, v) outline: hollow tunnel plus two side plates of `wall`, mirrored about w = 0 when `mirror`."""
    plate = wall if plate is None else plate
    inner = inset(poly, wall)
    rep = (lambda a, b: {"count": 2, "translation": [0, 0, -(a + b)]}) if mirror else (lambda a, b: None)
    out = []
    t = ext(prefix + "-tube", ctx, ccw(poly), "xy", w0 + plate - 0.001, w1 - plate + 0.001, holes=[inner])
    if mirror:
        t["repeat"] = rep(w0 + plate - 0.001, w1 - plate + 0.001)
    out.append(t)
    for nm, a, b in ((prefix + "-plate-o", w1 - plate, w1), (prefix + "-plate-i", w0, w0 + plate)):
        s = ext(nm, ctx, ccw(poly), "xy", a, b)
        if mirror:
            s["repeat"] = rep(a, b)
        out.append(s)
    return out


def sk(sid, ctx, poly, w0, w1, layer, **kw):
    return ext(sid, ctx(layer=layer), ccw(poly), "xy", w0, w1, **kw)
