"""approximate re-implementation (python) of the ink-gap rule of the print profile, per named pose, to locate near-miss outlines"""
import math, re, sys, json
from nw import *


def flat(s):
    k = s["kind"]
    if k == "poly": return [[tuple(p) for p in s["points"]]], s["closed"]
    if k == "ellipse":
        return [ell_pts(s["cx"], s["cy"], s["rx"], s["ry"], s.get("rotationDeg", 0), 24)], True
    toks = re.findall(r"[MCZ]|-?\d+\.?\d*(?:e-?\d+)?", s["d"])
    rings, cur, i, closed = [], [], 0, False
    pos = None
    while i < len(toks):
        t = toks[i]
        if t == "M":
            cur = [(float(toks[i+1]), float(toks[i+2]))]; rings.append(cur); pos = cur[0]; i += 3
        elif t == "C":
            p1 = (float(toks[i+1]), float(toks[i+2])); p2 = (float(toks[i+3]), float(toks[i+4])); p3 = (float(toks[i+5]), float(toks[i+6]))
            for j in range(1, 9):
                u = j/8
                x = (1-u)**3*pos[0] + 3*(1-u)**2*u*p1[0] + 3*(1-u)*u*u*p2[0] + u**3*p3[0]
                y = (1-u)**3*pos[1] + 3*(1-u)**2*u*p1[1] + 3*(1-u)*u*u*p2[1] + u**3*p3[1]
                cur.append((x, y))
            pos = p3; i += 7
        elif t == "Z":
            closed = True; i += 1
        else:
            i += 1
    return rings, closed


def seg_inter(a, b, c, d):
    def o(p, q, r): return (q[0]-p[0])*(r[1]-p[1]) - (q[1]-p[1])*(r[0]-p[0])
    return o(a, b, c)*o(a, b, d) <= 0 and o(c, d, a)*o(c, d, b) <= 0


def seg_dist(p, q, r, s):
    if seg_inter(p, q, r, s): return 0.0
    def ps(x, a, b):
        dx, dy = b[0]-a[0], b[1]-a[1]
        l2 = dx*dx + dy*dy
        t = 0 if l2 == 0 else max(0, min(1, ((x[0]-a[0])*dx + (x[1]-a[1])*dy)/l2))
        return math.hypot(x[0]-a[0]-t*dx, x[1]-a[1]-t*dy)
    return min(ps(p, r, s), ps(q, r, s), ps(r, p, q), ps(s, p, q))


def inside(pt, ring):
    c = False
    for i in range(len(ring)):
        a, b = ring[i], ring[i-1]
        if (a[1] > pt[1]) != (b[1] > pt[1]) and pt[0] < (b[0]-a[0])*(pt[1]-a[1])/(b[1]-a[1]) + a[0]: c = not c
    return c


def edges(rings, closed):
    out = []
    for r in rings:
        n = len(r)
        for i in range(n-1): out.append((r[i], r[i+1]))
        if closed and n > 2: out.append((r[-1], r[0]))
    return out


def gaps(asset, key, min_gap=0.15, only=None):
    rep = asset["representations"][key]
    den = int(key)
    k = 1000/den
    res = []
    for pose in asset["poses"]:
        if pose.get("key"): continue
        if only and pose["id"] not in only: continue
        W = pose_world(asset, pose)
        items = []
        for s in rep["geometry"]["elevation"]:
            pen = s.get("penMm") or rep["pens"].get(s["role"])
            if pen is None or s["role"] == "ground": continue
            rings, closed = flat(s)
            rr = [[(apply(W[s["bone"]], p)[0]*k, apply(W[s["bone"]], p)[1]*k) for p in r] for r in rings]
            items.append((s["id"], rr, closed, pen))
        for i in range(len(items)):
            for j in range(i+1, len(items)):
                a, b = items[i], items[j]
                ea, eb = edges(a[1], a[2]), edges(b[1], b[2])
                d = min((seg_dist(p, q, r, s_) for p, q in ea for r, s_ in eb if True), default=9)
                if d <= 1e-4: continue
                if a[2] and any(inside(r[0], rr) for rr in a[1] for r in b[1]): continue
                if b[2] and any(inside(r[0], rr) for rr in b[1] for r in a[1]): continue
                g = max(0.0, d - (a[3] + b[3])/2)
                if g < min_gap: res.append((pose["id"], a[0], b[0], round(g, 3)))
    return res


if __name__ == "__main__":
    import importlib
    mod, fn = sys.argv[1], sys.argv[2]
    m = importlib.import_module(mod)
    A = getattr(m, fn)()
    for key in sys.argv[3:]:
        print("scale", key)
        for r in gaps(A, key): print("  ", r)
