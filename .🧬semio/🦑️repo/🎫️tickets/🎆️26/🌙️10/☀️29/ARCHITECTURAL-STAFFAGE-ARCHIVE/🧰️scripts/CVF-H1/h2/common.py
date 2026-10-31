"""CV-H2 authoring helpers (coordinate conversion, splines through hand-entered points, shape records). No content is generated here."""
import sys, json, math, copy
sys.path.insert(0, '.')
from lib import *


def cr(pts, closed=True, k=1.0):
    """Catmull-Rom spline through hand-entered points as cubic Beziers; a third element 'c' marks a corner."""
    n = len(pts); P = [(p[0], p[1]) for p in pts]; C = [len(p) > 2 and p[2] == 'c' for p in pts]
    def g(i):
        return P[i % n] if closed else P[max(0, min(n-1, i))]
    segs = [("M", [P[0]])]
    rng = range(n) if closed else range(n-1)
    for i in rng:
        p0, p1, p2, p3 = g(i-1), g(i), g(i+1), g(i+2)
        a = (p1[0]+(p2[0]-p0[0])/6*k, p1[1]+(p2[1]-p0[1])/6*k) if not C[i % n] else (p1[0]+(p2[0]-p1[0])/3, p1[1]+(p2[1]-p1[1])/3)
        b = (p2[0]-(p3[0]-p1[0])/6*k, p2[1]-(p3[1]-p1[1])/6*k) if not C[(i+1) % n] else (p2[0]-(p2[0]-p1[0])/3, p2[1]-(p2[1]-p1[1])/3)
        if not closed and i == 0 and not C[0]: a = (p1[0]+(p2[0]-p1[0])/3, p1[1]+(p2[1]-p1[1])/3)
        if not closed and i == n-2 and not C[n-1]: b = (p2[0]-(p2[0]-p1[0])/3, p2[1]-(p2[1]-p1[1])/3)
        segs.append(("C", [a, b, p2]))
    if closed: segs.append(("Z", []))
    return segs


def D(segs):
    return path_str(segs)


def tube(L, prof, front=1, cap0=0.0, cap1=0.0):
    """closed outline along a bone in LOCAL coordinates from a hand-entered width table [(s, front_dist, back_dist)];
    front=+1 when local +y is anterior (bones pointing down), -1 when local -y is anterior (bones pointing up).
    cap0/cap1 add a rounded end beyond the first/last station."""
    fr = [(s, front*f) for s, f, b in prof]; bk = [(s, -front*b) for s, f, b in prof]
    pts = []
    if cap0: pts.append((prof[0][0]-cap0, (fr[0][1]+bk[0][1])/2))
    pts += fr
    if cap1: pts.append((prof[-1][0]+cap1, (fr[-1][1]+bk[-1][1])/2))
    pts += bk[::-1]
    return D(cr(pts))


class Ctx:
    def __init__(self, asset_id):
        self.doc = load(asset_id); self.a = self.doc['asset']; self.id = asset_id
        self.w = pose_world(self.a)
        self.shapes = []

    def loc(self, bone, d):
        return to_local(self.w[bone], d)

    def add(self, sid, bone, role, z, d=None, *, world=None, pts=None, closed=False, filled=True, tone=0.5, region=None, regions=None,
            material=None, hatch=None, pen=None, cap=None, ell=None):
        s = {"id": sid, "bone": bone}
        if world is not None: s.update(kind="path", d=self.loc(bone, world))
        elif d is not None: s.update(kind="path", d=d)
        elif pts is not None: s.update(kind="poly", points=[[round(x, 4), round(y, 4)] for x, y in pts], closed=closed)
        elif cap is not None: s.update(kind="capsule", x0=cap[0], y0=cap[1], x1=cap[2], y1=cap[3], r0=cap[4], r1=cap[5])
        elif ell is not None: s.update(kind="ellipse", cx=ell[0], cy=ell[1], rx=ell[2], ry=ell[3])
        s.update(role=role, z=z, filled=filled, tone=tone)
        if region: s["region"] = region
        if regions: s["regions"] = regions
        if material: s["material"] = material
        if hatch: s["hatch"] = hatch
        if pen: s["penMm"] = pen
        self.shapes.append(s); return s

    def wpoly(self, sid, bone, role, z, wpts, **kw):
        m = inv(self.w[bone])
        return self.add(sid, bone, role, z, pts=[ap(m, x, y) for x, y in wpts], **kw)


def hatch(i, title_en, title_de, pattern, angle, spacing, pen):
    return {"id": i, "title": {"en": title_en, "de": title_de}, "pattern": pattern, "angleDeg": angle, "spacingMm": spacing, "penMm": pen}


def preview(ctx, shapes, hatches, pens, denom, name, dpi=1200, pose=None, mode='line', bounds=None, zoom=1):
    w = pose_world(ctx.a, pose)
    svg = render_svg(shapes, w, denom, pens, bounds=bounds, mode=mode, ground=True)
    return to_png(svg, name, dpi)


def vap(r):
    return round(r/(16.5e-3*0.38), 1)
