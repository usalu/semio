import json, math, re, subprocess, os
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
ORIG = f"{T}/🗑️generated/cv-v/orig"
OUT = f"{T}/🗑️generated/cv-v"
CAT = f"{S}/🗂️catalog/🌳️vegetation"

def load_orig(a):
    return json.load(open(f"{ORIG}/{a}.json"))

def f(n):
    s = f"{n:.3f}".rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s

class Rig:
    def __init__(self, asset):
        self.b = {b["id"]: b for b in asset["rig"]["bones"]}
        self.cache = {}
    def world(self, bid):
        if bid in self.cache: return self.cache[bid]
        b = self.b[bid]
        a = math.radians(b["angleDeg"])
        ca, sa = math.cos(a), math.sin(a)
        if b["parent"] is None:
            m = (ca, -sa, b["x"], sa, ca, b["y"])
        else:
            p = self.world(b["parent"])
            lx, ly = b["x"], b["y"]
            tx = p[0]*lx + p[1]*ly + p[2]
            ty = p[3]*lx + p[4]*ly + p[5]
            m = (p[0]*ca + p[1]*sa, -p[0]*sa + p[1]*ca, tx, p[3]*ca + p[4]*sa, -p[3]*sa + p[4]*ca, ty)
        self.cache[bid] = m
        return m
    def to_world(self, bid, x, y):
        m = self.world(bid)
        return (m[0]*x + m[1]*y + m[2], m[3]*x + m[4]*y + m[5])
    def to_local(self, bid, X, Y):
        m = self.world(bid)
        det = m[0]*m[4] - m[1]*m[3]
        dx, dy = X - m[2], Y - m[5]
        return ((m[4]*dx - m[1]*dy)/det, (-m[3]*dx + m[0]*dy)/det)
    def angle(self, bid):
        m = self.world(bid)
        return math.degrees(math.atan2(m[3], m[0]))

TOK = re.compile(r"([MLHVCQZmlhvcqz])|(-?\d*\.?\d+(?:e-?\d+)?)")
def parse_path(d):
    toks = [(m.group(1), m.group(2)) for m in TOK.finditer(d)]
    cmds = []; cur = None; nums = []
    for c, n in toks:
        if c:
            if cur: cmds.append((cur, nums))
            cur, nums = c, []
        else:
            nums.append(float(n))
    if cur: cmds.append((cur, nums))
    out = []; x = y = 0.0; sx = sy = 0.0
    for c, n in cmds:
        rel = c.islower(); C = c.upper()
        if C == "Z": out.append(("Z",)); x, y = sx, sy; continue
        step = {"M": 2, "L": 2, "H": 1, "V": 1, "C": 6, "Q": 4}[C]
        for i in range(0, len(n), step):
            a = n[i:i+step]
            if C in "ML":
                px, py = (x+a[0], y+a[1]) if rel else (a[0], a[1])
                out.append(("M" if (C == "M" and i == 0) else "L", px, py)); x, y = px, py
                if C == "M" and i == 0: sx, sy = x, y
            elif C == "H":
                x = x+a[0] if rel else a[0]; out.append(("L", x, y))
            elif C == "V":
                y = y+a[0] if rel else a[0]; out.append(("L", x, y))
            elif C == "C":
                p = [(x+a[0], y+a[1], x+a[2], y+a[3], x+a[4], y+a[5])] if rel else [tuple(a)]
                q = p[0]; out.append(("C",)+q); x, y = q[4], q[5]
            elif C == "Q":
                q = (x+a[0], y+a[1], x+a[2], y+a[3]) if rel else tuple(a)
                out.append(("Q",)+q); x, y = q[2], q[3]
    return out

def path_to_world(rig, bone, d):
    out = []
    for c in parse_path(d):
        if c[0] == "Z": out.append("Z"); continue
        pts = []
        for i in range(1, len(c), 2):
            pts.append(rig.to_world(bone, c[i], c[i+1]))
        out.append(c[0] + " ".join(f"{f(p[0])} {f(p[1])}" for p in pts))
    return "".join(out)

def shape_world_d(rig, s):
    k = s["kind"]; b = s["bone"]
    if k == "path": return path_to_world(rig, b, s["d"])
    if k == "poly":
        pts = [rig.to_world(b, x, y) for x, y in s["points"]]
        return "M" + "L".join(f"{f(x)} {f(y)}" for x, y in pts) + ("Z" if s["closed"] else "")
    if k == "ellipse":
        n = 28; pts = []; r = math.radians(s.get("rotationDeg", 0))
        for i in range(n):
            t = 2*math.pi*i/n; x = s["rx"]*math.cos(t); y = s["ry"]*math.sin(t)
            pts.append(rig.to_world(b, s["cx"] + x*math.cos(r) - y*math.sin(r), s["cy"] + x*math.sin(r) + y*math.cos(r)))
        return "M" + "L".join(f"{f(x)} {f(y)}" for x, y in pts) + "Z"
    if k == "capsule":
        x0, y0, x1, y1, r0, r1 = (s[q] for q in ("x0", "y0", "x1", "y1", "r0", "r1"))
        a = math.atan2(y1-y0, x1-x0); nx, ny = -math.sin(a), math.cos(a)
        pts = [(x0+nx*r0, y0+ny*r0), (x1+nx*r1, y1+ny*r1), (x1-nx*r1, y1-ny*r1), (x0-nx*r0, y0-ny*r0)]
        pts = [rig.to_world(b, x, y) for x, y in pts]
        return "M" + "L".join(f"{f(x)} {f(y)}" for x, y in pts) + "Z"
    if k == "rect":
        x, y, w, h = s["x"], s["y"], s["w"], s["h"]
        pts = [rig.to_world(b, px, py) for px, py in ((x, y), (x+w, y), (x+w, y+h), (x, y+h))]
        return "M" + "L".join(f"{f(x)} {f(y)}" for x, y in pts) + "Z"
    raise ValueError(k)

def smooth(pts, closed=True, k=1.0):
    n = len(pts); d = f"M{f(pts[0][0])} {f(pts[0][1])}"
    def tan(i):
        a = pts[(i-1) % n] if (closed or i > 0) else pts[i]
        b = pts[(i+1) % n] if (closed or i < n-1) else pts[i]
        dx, dy = b[0]-a[0], b[1]-a[1]; l = math.hypot(dx, dy) or 1
        return dx/l, dy/l
    for i in (range(n) if closed else range(n-1)):
        p1 = pts[i]; p2 = pts[(i+1) % n]
        L = math.hypot(p2[0]-p1[0], p2[1]-p1[1])/3*k
        t1 = tan(i); t2 = tan((i+1) % n)
        d += f"C{f(p1[0]+t1[0]*L)} {f(p1[1]+t1[1]*L)} {f(p2[0]-t2[0]*L)} {f(p2[1]-t2[1]*L)} {f(p2[0])} {f(p2[1])}"
    return d + ("Z" if closed else "")

def lobes(notches, bulge, closed=True, ccw=None):
    """scalloped outline: notches (world points, counter-clockwise), bulge per lobe as fraction of chord pushed outward"""
    n = len(notches); d = f"M{f(notches[0][0])} {f(notches[0][1])}"
    if ccw is None:
        ccw = sum(notches[i][0]*notches[(i+1)%n][1]-notches[(i+1)%n][0]*notches[i][1] for i in range(n)) > 0
    for i in range(n if closed else n-1):
        a = notches[i]; b = notches[(i+1) % n]
        bu = bulge[i] if isinstance(bulge, (list, tuple)) else bulge
        dx, dy = b[0]-a[0], b[1]-a[1]
        nx, ny = dy, -dx
        if not ccw: nx, ny = -nx, -ny
        h = bu*1.333
        c1 = (a[0] + dx*0.2 + nx*h*0.7, a[1] + dy*0.2 + ny*h*0.7)
        c2 = (b[0] - dx*0.2 + nx*h*0.7, b[1] - dy*0.2 + ny*h*0.7)
        d += f"C{f(c1[0])} {f(c1[1])} {f(c2[0])} {f(c2[1])} {f(b[0])} {f(b[1])}"
    return d + ("Z" if closed else "")

def bind(rig, bone, d):
    """convert absolute world path d (M L C Q Z) into bone-local path"""
    out = []
    for c in parse_path(d):
        if c[0] == "Z": out.append("Z"); continue
        pts = [rig.to_local(bone, c[i], c[i+1]) for i in range(1, len(c), 2)]
        out.append(c[0] + " ".join(f"{f(p[0])} {f(p[1])}" for p in pts))
    return "".join(out)

def shape(rig, id, bone, role, d, region=None, regions=None, tone=0.3, filled=True, z=0, material=None, hatch=None, pen=None):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "path", "d": bind(rig, bone, d)}
    if region: s["region"] = region
    if regions: s["regions"] = regions
    if material: s["material"] = material
    if hatch: s["hatch"] = hatch
    if pen: s["penMm"] = pen
    return s

def ellipse_pts(cx, cy, rx, ry, n=16, rot=0):
    r = math.radians(rot)
    return [(cx + rx*math.cos(2*math.pi*i/n)*math.cos(r) - ry*math.sin(2*math.pi*i/n)*math.sin(r), cy + rx*math.cos(2*math.pi*i/n)*math.sin(r) + ry*math.sin(2*math.pi*i/n)*math.cos(r)) for i in range(n)]

def render_svg(shapes_world, W, H, x0, y0, scale_px, out_svg, extra=""):
    """shapes_world: list of (d, fill, stroke, strokew_mm, fillopacity) in model metres; x0,y0 = model lower-left; scale_px = px per metre"""
    parts = []
    for d, fill, stroke, sw, fo in shapes_world:
        parts.append(f'<path d="{d}" fill="{fill}" fill-opacity="{fo}" stroke="{stroke}" stroke-width="{sw}" stroke-linejoin="round" stroke-linecap="round" vector-effect="non-scaling-stroke"/>')
    svg = f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}"><rect width="{W}" height="{H}" fill="white"/><g transform="translate({-x0*scale_px} {H + y0*scale_px}) scale({scale_px} {-scale_px})">{"".join(parts)}</g>{extra}</svg>'
    open(out_svg, "w").write(svg)

def png(svg, out_png, dpi=None, zoom=None):
    cmd = ["rsvg-convert", svg, "-o", out_png]
    if dpi: cmd += ["-d", str(dpi), "-p", str(dpi)]
    if zoom: cmd += ["-z", str(zoom)]
    subprocess.run(cmd, check=True)

def flatten(d, n=12):
    pts = []; cx = cy = 0
    for c in parse_path(d):
        if c[0] in "ML": cx, cy = c[1], c[2]; pts.append((cx, cy))
        elif c[0] == "C":
            for i in range(1, n+1):
                t = i/n; u = 1-t
                pts.append((u**3*cx + 3*u*u*t*c[1] + 3*u*t*t*c[3] + t**3*c[5], u**3*cy + 3*u*u*t*c[2] + 3*u*t*t*c[4] + t**3*c[6]))
            cx, cy = c[5], c[6]
        elif c[0] == "Q":
            for i in range(1, n+1):
                t = i/n; u = 1-t
                pts.append((u*u*cx + 2*u*t*c[1] + t*t*c[3], u*u*cy + 2*u*t*c[2] + t*t*c[4]))
            cx, cy = c[3], c[4]
    return pts

def bbox_shapes(rig, shapes, roles=("body",)):
    X = []; Y = []
    for s in shapes:
        d = shape_world_d(rig, s)
        for x, y in flatten(d): X.append(x); Y.append(y)
    return (min(X), max(X), min(Y), max(Y))
