import sys, os, math
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

def rot(p, ang, c=(0, 0)):
    a = math.radians(ang); ca, sa = math.cos(a), math.sin(a)
    x, y = p[0]-c[0], p[1]-c[1]
    return (c[0] + x*ca - y*sa, c[1] + x*sa + y*ca)

def pca_frame(pts):
    n = len(pts); mx = sum(p[0] for p in pts)/n; my = sum(p[1] for p in pts)/n
    sxx = sum((p[0]-mx)**2 for p in pts); syy = sum((p[1]-my)**2 for p in pts); sxy = sum((p[0]-mx)*(p[1]-my) for p in pts)
    ang = 0.5*math.atan2(2*sxy, sxx-syy); ca, sa = math.cos(ang), math.sin(ang)
    u = [(p[0]-mx)*ca + (p[1]-my)*sa for p in pts]; v = [-(p[0]-mx)*sa + (p[1]-my)*ca for p in pts]
    cu = (max(u)+min(u))/2; cv = (max(v)+min(v))/2
    return dict(cx=mx + cu*ca - cv*sa, cy=my + cu*sa + cv*ca, ang=math.degrees(ang), a=(max(u)-min(u))/2, b=(max(v)-min(v))/2)

def shape_pts(R, s):
    return flatten(shape_world_d(R, s))

def blob(cx, cy, a, b, ang, n, depth, bulge, jit=(0,), phase=0.0, squash=1.0):
    def build(sa, sb):
        pts = []
        for i in range(n):
            t = phase + 2*math.pi*i/n
            r = 1 - depth*(1 + jit[i % len(jit)])
            pts.append((sa*r*math.cos(t), sb*r*math.sin(t)*squash))
        return pts
    sa, sb = a, b
    for _ in range(3):
        pts = build(sa, sb); d = lobes(pts, bulge)
        fp = flatten(d); X = [p[0] for p in fp]; Y = [p[1] for p in fp]
        ea = (max(X)-min(X))/2; eb = (max(Y)-min(Y))/2
        sa *= a/ea; sb *= b/eb
    pts = build(sa, sb)
    fp = flatten(lobes(pts, bulge)); X = [p[0] for p in fp]; Y = [p[1] for p in fp]
    ox = (max(X)+min(X))/2; oy = (max(Y)+min(Y))/2
    pts = [rot((p[0]-ox, p[1]-oy), ang) for p in pts]
    pts = [(p[0]+cx, p[1]+cy) for p in pts]
    return lobes(pts, bulge)

def chain(points, radii, round_end=False):
    n = len(points); L = []; Rr = []
    for i, p in enumerate(points):
        if i == 0: dx, dy = points[1][0]-p[0], points[1][1]-p[1]
        elif i == n-1: dx, dy = p[0]-points[i-1][0], p[1]-points[i-1][1]
        else: dx, dy = points[i+1][0]-points[i-1][0], points[i+1][1]-points[i-1][1]
        l = math.hypot(dx, dy) or 1; nx, ny = -dy/l, dx/l
        L.append((p[0]+nx*radii[i], p[1]+ny*radii[i])); Rr.append((p[0]-nx*radii[i], p[1]-ny*radii[i]))
    left = smooth(L, closed=False)
    right = smooth(Rr[::-1], closed=False)
    right = "L" + right[1:]
    return left + right + "Z"

def poly(pts):
    return "M" + "L".join(f"{f(x)} {f(y)}" for x, y in pts) + "Z"

def stroke(pts, smooth_=True):
    return smooth(pts, closed=False) if smooth_ and len(pts) > 2 else "M" + "L".join(f"{f(x)} {f(y)}" for x, y in pts)

def px(pts, x0, y0, s, top):
    return [(x0 + p[0]/s, top - p[1]/s) for p in pts]

def fit_bbox(path_world, target):
    fp = flatten(path_world); X = [p[0] for p in fp]; Y = [p[1] for p in fp]
    return (min(X), max(X), min(Y), max(Y))

def woody(R, O, bid, child=None, ratio=0.55, minr=0.01):
    b = O["rig"]["bones"]; B = {x["id"]: x for x in b}
    s = R.to_world(bid, 0, 0); e = R.to_world(bid, B[bid]["lengthM"], 0)
    r0 = B[bid].get("section", {}).get("extremeFibreM", minr)
    if child and "section" in B[child]: r1 = B[child]["section"]["extremeFibreM"]
    else: r1 = max(r0*ratio, minr)
    return s, e, r0, r1

def mid(p, q, t=0.5):
    return (p[0]+(q[0]-p[0])*t, p[1]+(q[1]-p[1])*t)

def limb_path(R, O, bones, tip=0.45, ratio=0.5, bow=0.0):
    pts = []; rad = []
    for i, bid in enumerate(bones):
        s, e, r0, r1 = woody(R, O, bid, None, ratio)
        if i == 0: pts.append(s); rad.append(r0)
        pts.append(e); rad.append(r1 if i == len(bones)-1 else (woody(R, O, bones[i+1])[2]))
    rad[-1] = max(rad[-1]*tip/0.45*0.5, 0.012) if False else rad[-1]
    return chain(pts, rad)

PENS = {
    "500": {"body": 0.15, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
    "200": {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25},
    "100": {"body": 0.2, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.3},
    "50": {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": 0.1, "ground": 0.35},
}
VALID = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}

def affine_path(d, fn):
    out = []
    for c in parse_path(d):
        if c[0] == "Z": out.append("Z"); continue
        pts = [fn(c[i], c[i+1]) for i in range(1, len(c), 2)]
        out.append(c[0] + " ".join(f"{f(p[0])} {f(p[1])}" for p in pts))
    return "".join(out)

def fit_to(d, tgt):
    fp = flatten(d); X = [p[0] for p in fp]; Y = [p[1] for p in fp]
    x0, x1, y0, y1 = min(X), max(X), min(Y), max(Y)
    sx = (tgt[1]-tgt[0])/(x1-x0); sy = (tgt[3]-tgt[2])/(y1-y0)
    return affine_path(d, lambda x, y: (tgt[0] + (x-x0)*sx, tgt[2] + (y-y0)*sy))

def fit_x(d, x0, x1):
    fp = flatten(d); X = [p[0] for p in fp]
    a, b = min(X), max(X)
    return affine_path(d, lambda x, y: (x0 + (x-a)*(x1-x0)/(b-a), y))

def fit_y(d, y0, y1):
    fp = flatten(d); Y = [p[1] for p in fp]
    a, b = min(Y), max(Y)
    return affine_path(d, lambda x, y: (x, y0 + (y-a)*(y1-y0)/(b-a)))
