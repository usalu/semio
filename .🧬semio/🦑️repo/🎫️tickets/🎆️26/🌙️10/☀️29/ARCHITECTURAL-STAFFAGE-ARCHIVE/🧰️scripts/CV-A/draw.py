# [scratch CV-A] hand-drawing helpers: every point is authored; these only turn points into bezier text and local coordinates
import math
from cva import *

def smooth(pts, closed=True, k=1.0):
    """Catmull-Rom through authored points; a point given as (x,y,'c') is a sharp corner."""
    P = [(p[0], p[1], len(p) > 2) for p in pts]
    n = len(P)
    def g(i): return P[i % n] if closed else P[min(max(i, 0), n-1)]
    d = f"M {P[0][0]:g} {P[0][1]:g}"
    for i in range(n if closed else n-1):
        p0, p1, p2, p3 = g(i-1), g(i), g(i+1), g(i+2)
        if p1[2]: c1 = (p1[0]+(p2[0]-p1[0])/3, p1[1]+(p2[1]-p1[1])/3)
        else: c1 = (p1[0]+(p2[0]-p0[0])/6*k, p1[1]+(p2[1]-p0[1])/6*k)
        if p2[2]: c2 = (p2[0]-(p2[0]-p1[0])/3, p2[1]-(p2[1]-p1[1])/3)
        else: c2 = (p2[0]-(p3[0]-p1[0])/6*k, p2[1]-(p3[1]-p1[1])/6*k)
        d += f" C {c1[0]:.4f} {c1[1]:.4f} {c2[0]:.4f} {c2[1]:.4f} {p2[0]:g} {p2[1]:g}"
    return d + (" Z" if closed else "")

def limb(A, B, wa, wb, bulge=0.0, front=0.5, mid=0.5):
    """tapered limb outline around the line A->B in world metres: widths wa at A, wb at B (full widths), `front` share of width on the leading (left-normal of A->B pointing +x side picks the side where normal.x>0)."""
    ax, ay = A; bx, by = B; L = math.hypot(bx-ax, by-ay); ux, uy = (bx-ax)/L, (by-ay)/L
    nx, ny = -uy, ux
    if nx < 0 or (abs(nx) < 1e-9 and ny < 0): nx, ny = -nx, -ny
    def pt(f, off, w): return (ax+ux*L*f+nx*off*w, ay+uy*L*f+ny*off*w)
    fw = front; bw = 1-front
    pts = [pt(0, fw, wa), pt(mid, fw, wa+(wb-wa)*mid + bulge*0.5), pt(1, fw, wb), pt(1, -bw, wb), pt(mid, -bw, wa+(wb-wa)*mid + bulge), pt(0, -bw, wa)]
    return smooth(pts)

def capsule_world(A, B, ra, rb, n=8):
    ax, ay = A; bx, by = B; a = math.atan2(by-ay, bx-ax)
    pts = []
    for i in range(n+1):
        th = a - math.pi/2 + math.pi*i/n; pts.append((bx+rb*math.cos(th), by+rb*math.sin(th)))
    for i in range(n+1):
        th = a + math.pi/2 + math.pi*i/n; pts.append((ax+ra*math.cos(th), ay+ra*math.sin(th)))
    return "M " + " L ".join(f"{x:.4f} {y:.4f}" for x, y in pts) + " Z"

class Rep:
    """collects authored parts of one scale in world coordinates of the default pose and emits local-coordinate shapes"""
    def __init__(self, world): self.world = world; self.shapes = []
    def add(self, id, bone, role, z, filled, tone, d=None, pts=None, closed=True, region=None, regions=None, material=None, hatch=None, penMm=None, poly=None):
        if pts is not None: d = smooth(pts, closed=closed)
        s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
        if poly is not None:
            m = inv(self.world[bone]); s.update(kind="poly", closed=closed and False, points=[[round(v, 4) for v in app(m, x, y)] for x, y in [(q[0], q[1]) for q in poly]])
        else:
            s.update(kind="path", d=world_to_local(self.world, bone, d))
        if region: s["region"] = region
        if regions: s["regions"] = regions
        if material: s["material"] = material
        if hatch: s["hatch"] = hatch
        if penMm is not None: s["penMm"] = penMm
        order = ["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"]
        self.shapes.append({k: s[k] for k in order if k in s} | {k: v for k, v in s.items() if k not in order})
        return s
    def ellipse(self, id, bone, role, z, filled, tone, c, rx, ry, rot=0, region=None, material=None, penMm=None):
        m = inv(self.world[bone]); cx, cy = app(m, *c)
        ang = math.degrees(math.atan2(self.world[bone][1], self.world[bone][0]))
        s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
        if region: s["region"] = region
        if material: s["material"] = material
        if penMm is not None: s["penMm"] = penMm
        s.update(kind="ellipse", cx=round(cx, 4), cy=round(cy, 4), rx=rx, ry=ry)
        if rot-ang: s["rotationDeg"] = round(rot-ang, 3)
        self.shapes.append(s)

def preview(asset, rep, pose, out, mm_per_m, zoom=1, dpi=600, pad_m=0.0, bbox=None, grid=None, skeleton_on=False):
    """print-size preview: paper mm = model m * mm_per_m; image px = mm * dpi/25.4 * zoom"""
    w = pose_world(asset, pose)
    shapes = rep["geometry"]["elevation"]
    bb = bbox or bbox_of(shapes, w)
    bb = (bb[0]-pad_m, min(bb[1], 0)-pad_m, bb[2]+pad_m, bb[3]+pad_m)
    ppm = mm_per_m * dpi/25.4 * zoom
    pens = {k: (v/mm_per_m if v else None) for k, v in rep["pens"].items()}
    pens["mmToM"] = 1000/mm_per_m
    render(shapes, w, pens, ppm, bb, out, grid=grid)
