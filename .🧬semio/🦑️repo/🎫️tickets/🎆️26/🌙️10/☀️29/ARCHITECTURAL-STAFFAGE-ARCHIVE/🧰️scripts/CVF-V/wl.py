import math
from lib2 import *

def _cr(pts, n):
    """Catmull-Rom sample of the open polyline pts, n samples per segment; returns list of (x,y) and index of each vertex in the list"""
    out = []; idx = []
    P = [pts[0]] + list(pts) + [pts[-1]]
    for i in range(1, len(P) - 2):
        p0, p1, p2, p3 = P[i-1], P[i], P[i+1], P[i+2]
        idx.append(len(out))
        for k in range(n):
            t = k / n
            out.append(tuple(0.5 * ((2*p1[j]) + (-p0[j] + p2[j])*t + (2*p0[j] - 5*p1[j] + 4*p2[j] - p3[j])*t*t + (-p0[j] + 3*p1[j] - 3*p2[j] + p3[j])*t**3) for j in (0, 1)))
    idx.append(len(out)); out.append(tuple(pts[-1]))
    return out, idx

def pieces(R, pts, radii, bones, ids, region, z=2, tone=0.4, ext=0.5, n=10, zones=None, role="body", back=None, tipcap=True, regions=None):
    """a woody member through world vertices pts (bone k runs pts[k]->pts[k+1]); radius at each vertex; one shape per bone, overlapping its neighbours by ext*radius so joints leave no seam"""
    C, idx = _cr(pts, n)
    s = [0.0]
    for i in range(1, len(C)): s.append(s[-1] + math.hypot(C[i][0]-C[i-1][0], C[i][1]-C[i-1][1]))
    sv = [s[i] for i in idx]
    def rad(a):
        for k in range(len(sv) - 1):
            if a <= sv[k+1] or k == len(sv) - 2:
                t = (a - sv[k]) / ((sv[k+1] - sv[k]) or 1)
                return radii[k] + (radii[k+1] - radii[k]) * t
    def norm(i):
        a = C[max(i-1, 0)]; b = C[min(i+1, len(C)-1)]
        dx, dy = b[0]-a[0], b[1]-a[1]; l = math.hypot(dx, dy) or 1
        return -dy/l, dx/l
    def at(a):
        for i in range(len(C) - 1):
            if a <= s[i+1] + 1e-12:
                t = (a - s[i]) / ((s[i+1] - s[i]) or 1)
                return (C[i][0] + (C[i+1][0]-C[i][0])*t, C[i][1] + (C[i+1][1]-C[i][1])*t), i
        return C[-1], len(C) - 2
    out = []
    for k in range(len(pts) - 1):
        F = zones or [0.0] * len(pts)
        a0 = sv[k] + (F[k] - ext * radii[k] if (k > 0 or back) else 0.0)
        a1 = sv[k+1] + (F[k+1] + ext * radii[k+1] if k < len(pts) - 2 else 0.0)
        a0 = max(a0, 0.0)
        P0, i0 = at(a0); P1, i1 = at(a1)
        mids = [i for i in range(len(C)) if a0 + 1e-9 < s[i] < a1 - 1e-9]
        samp = [(a0, P0)] + [(s[i], C[i]) for i in mids] + [(a1, P1)]
        def nrm(i, P):
            a = C[max(i-1, 0)]; b = C[min(i+1, len(C)-1)]
            dx, dy = b[0]-a[0], b[1]-a[1]; l = math.hypot(dx, dy) or 1
            return -dy/l, dx/l
        L = []; Rr = []
        for j, (aa, P) in enumerate(samp):
            ci = min(range(len(C)), key=lambda i: abs(s[i] - aa)) if j in (0, len(samp)-1) else mids[j-1]
            nx, ny = nrm(ci, P); r = rad(aa)
            L.append((P[0] + nx*r, P[1] + ny*r)); Rr.append((P[0] - nx*r, P[1] - ny*r))
        d = "M" + "L".join(f"{f(x)} {f(y)}" for x, y in L + Rr[::-1]) + "Z"
        sh = shape(R, ids[k], bones[k], role, d, region=region if not regions else None, regions=regions, tone=tone, z=z)
        out.append(sh)
    return out

def circle(cx, cy, r, n=14):
    return "M" + "L".join(f"{f(cx + r*math.cos(2*math.pi*i/n))} {f(cy + r*math.sin(2*math.pi*i/n))}" for i in range(n)) + "Z"

def wpts(R, s):
    return flatten(shape_world_d(R, s), 4)

def centroid(R, s):
    P = wpts(R, s); X = [p[0] for p in P]; Y = [p[1] for p in P]
    return ((min(X) + max(X)) / 2, (min(Y) + max(Y)) / 2)

def xf(R, shapes, sid, sx=1.0, sy=None, dx=0.0, dy=0.0, rotd=0.0, about=None):
    """transform shape sid (world: scale about `about`/bbox centre, rotate, translate) and rebind it to its bone"""
    sy = sx if sy is None else sy
    for i, s in enumerate(shapes):
        if s["id"] != sid: continue
        c = about or centroid(R, s)
        d = shape_world_d(R, s)
        def fn(x, y):
            x, y = c[0] + (x - c[0]) * sx, c[1] + (y - c[1]) * sy
            if rotd: x, y = rot((x, y), rotd, c)
            return (x + dx, y + dy)
        nd = affine_path(d, fn)
        t = dict(s); t["kind"] = "path"; t["d"] = bind(R, s["bone"], nd)
        for k in ("points", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "x", "y", "w", "h", "r", "rotationDeg"):
            if s["kind"] != "path": t.pop(k, None)
        shapes[i] = t
        return t
    raise KeyError(sid)

def drop(shapes, *ids):
    shapes[:] = [s for s in shapes if s["id"] not in ids]

def sec_r(A, bid, default):
    for b in A["rig"]["bones"]:
        if b["id"] == bid: return b.get("section", {}).get("extremeFibreM", default)

def member(R, A, bones, region, tip=0.4, z=2.0, tone=0.4, ext=0.4, zones=None, rs=1.0, back=0.0, r0=None, ids=None, rmin=0.0, radii=None, via=None):
    """a woody member along `bones` (parent to child), drawn at its true section radius (rs scales it); back > 0 starts it that many metres inside the parent"""
    B = {b["id"]: b for b in A["rig"]["bones"]}
    pts = []; rad = []
    for i, bid in enumerate(bones):
        s = R.to_world(bid, 0, 0); r = sec_r(A, bid, 0.03) * rs
        if i == 0 and back > 0:
            e = R.to_world(bid, B[bid]["lengthM"], 0); d = math.hypot(e[0]-s[0], e[1]-s[1]) or 1
            s = (s[0] - (e[0]-s[0])/d*back, s[1] - (e[1]-s[1])/d*back)
        pts.append(s); rad.append(max(r0 if (i == 0 and r0) else r, rmin))
    e = R.to_world(bones[-1], B[bones[-1]]["lengthM"], 0); pts.append(e); rad.append(max(rad[-1] * tip, rmin * 0.5))
    if radii: rad = list(radii)
    return pieces(R, pts, rad, list(bones), ids or list(bones), region, z=z, tone=tone, ext=ext, back=back > 0, zones=zones)

def prune(shapes, leaf_regions, bone_prefixes):
    """keep foliage shapes and foliage marks; drop wood and bark detail"""
    out = []
    for s in shapes:
        regs = set([s.get("region")] + list(s.get("regions") or []))
        if regs & set(leaf_regions) or (s["bone"].startswith(tuple(bone_prefixes)) and s["role"] != "body" ) or (s["bone"].startswith(tuple(bone_prefixes)) and s["role"] == "body" and not s.get("region")):
            out.append(s)
    return out
