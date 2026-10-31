# [scratch CVF-A] python re-implementation of the engine's hidden-line drawing of one pose at one scale (fills + visible outline runs), with markers at paper-mm positions
import json, math, sys, os, subprocess, re
from cva import *

def flat_path(d, n=12):
    toks = TOK.findall(d); out = []; i = 0; cmd = None; cur = None; start = None; closed = False
    rings = []; ring = []
    while i < len(toks):
        t = toks[i]
        if t.isalpha():
            cmd = t; i += 1
            if cmd in "Zz":
                closed = True; rings.append((ring, True)); ring = []; continue
            if cmd in "Mm" and ring: rings.append((ring, False)); ring = []
            continue
        if cmd in "M":
            cur = (float(toks[i]), float(toks[i+1])); i += 2; ring.append(cur); cmd = "L"
        elif cmd == "L":
            cur = (float(toks[i]), float(toks[i+1])); i += 2; ring.append(cur)
        elif cmd == "C":
            v = [float(x) for x in toks[i:i+6]]; i += 6
            for j in range(1, n+1):
                u = j/n; a = (1-u)**3; b = 3*u*(1-u)**2; c = 3*u*u*(1-u); e = u**3
                ring.append((a*cur[0]+b*v[0]+c*v[2]+e*v[4], a*cur[1]+b*v[1]+c*v[3]+e*v[5]))
            cur = (v[4], v[5])
        elif cmd == "Q":
            v = [float(x) for x in toks[i:i+4]]; i += 4
            for j in range(1, n+1):
                u = j/n; ring.append(((1-u)**2*cur[0]+2*u*(1-u)*v[0]+u*u*v[2], (1-u)**2*cur[1]+2*u*(1-u)*v[1]+u*u*v[3]))
            cur = (v[2], v[3])
        else: i += 1
    if ring: rings.append((ring, False))
    return rings

def shape_rings(s):
    k = s["kind"]
    if k == "path": return flat_path(s["d"])
    if k == "poly": return [([tuple(p) for p in s["points"]], bool(s["closed"]))]
    if k == "ellipse":
        rot = math.radians(s.get("rotationDeg", 0)); pts = []
        for j in range(48):
            t = 2*math.pi*j/48; x = s["rx"]*math.cos(t); y = s["ry"]*math.sin(t)
            pts.append((s["cx"]+x*math.cos(rot)-y*math.sin(rot), s["cy"]+x*math.sin(rot)+y*math.cos(rot)))
        return [(pts, True)]
    if k == "capsule":
        x0, y0, x1, y1, r0, r1 = (s[q] for q in ("x0", "y0", "x1", "y1", "r0", "r1")); a = math.atan2(y1-y0, x1-x0); pts = []
        for j in range(13):
            t = a-math.pi/2+math.pi*j/12; pts.append((x1+r1*math.cos(t), y1+r1*math.sin(t)))
        for j in range(13):
            t = a+math.pi/2+math.pi*j/12; pts.append((x0+r0*math.cos(t), y0+r0*math.sin(t)))
        return [(pts, True)]
    if k == "rect":
        x, y, w, h = s["x"], s["y"], s["w"], s["h"]; return [([(x, y), (x+w, y), (x+w, y+h), (x, y+h)], True)]
    return []

def inside(p, rings, eps=1e-4):
    c = False
    for ring in rings:
        n = len(ring); j = n-1
        for i in range(n):
            a = ring[i]; b = ring[j]
            if (a[1] > p[1]) != (b[1] > p[1]) and p[0] < (b[0]-a[0])*(p[1]-a[1])/(b[1]-a[1])+a[0]: c = not c
            j = i
    if c: return True
    for ring in rings:
        n = len(ring)
        for i in range(n):
            a = ring[i]; b = ring[(i+1) % n]
            dx, dy = b[0]-a[0], b[1]-a[1]; l2 = dx*dx+dy*dy
            t = 0 if l2 == 0 else max(0, min(1, ((p[0]-a[0])*dx+(p[1]-a[1])*dy)/l2))
            if math.hypot(p[0]-a[0]-t*dx, p[1]-a[1]-t*dy) <= eps: return True
    return False

def seg_cross(a, b, c, d):
    r = (b[0]-a[0], b[1]-a[1]); s = (d[0]-c[0], d[1]-c[1]); den = r[0]*s[1]-r[1]*s[0]
    if abs(den) < 1e-14: return None
    t = ((c[0]-a[0])*s[1]-(c[1]-a[1])*s[0])/den; u = ((c[0]-a[0])*r[1]-(c[1]-a[1])*r[0])/den
    return t if 0 <= t <= 1 and 0 <= u <= 1 else None

def visible_runs(pts, closed, hiders):
    n = len(pts); edges = n if closed else n-1; runs = []; cur = []
    for i in range(edges):
        a = pts[i]; b = pts[(i+1) % n]
        ts = [0.0, 1.0]
        for h in hiders:
            for ring in h:
                m = len(ring)
                for j in range(m):
                    t = seg_cross(a, b, ring[j], ring[(j+1) % m])
                    if t is not None: ts.append(t)
        ts.sort()
        for q in range(len(ts)-1):
            t0, t1 = ts[q], ts[q+1]
            if t1-t0 < 1e-9: continue
            mid = (a[0]+(b[0]-a[0])*(t0+t1)/2, a[1]+(b[1]-a[1])*(t0+t1)/2)
            hid = any(inside(mid, h) for h in hiders)
            p0 = (a[0]+(b[0]-a[0])*t0, a[1]+(b[1]-a[1])*t0); p1 = (a[0]+(b[0]-a[0])*t1, a[1]+(b[1]-a[1])*t1)
            if hid:
                if len(cur) > 1: runs.append(cur)
                cur = []
            else:
                if not cur: cur.append(p0)
                cur.append(p1)
    if len(cur) > 1: runs.append(cur)
    return runs

def prepare(asset, rep, pose_id, scale):
    w = pose_world(asset, pose_id); k = 1000/scale
    parents = {b["id"]: b["parent"] for b in asset["rig"]["bones"]}
    items = []
    for idx, s in enumerate(rep["geometry"]["elevation"]):
        pen = s.get("penMm", rep["pens"].get(s["role"]))
        if pen is None or s["role"] == "ground": continue
        rings = []
        closed = False
        for pts, c in shape_rings(s):
            P = [app(w[s["bone"]], x, y) for x, y in pts]; rings.append([(x*k, -y*k) for x, y in P]); closed = c
        items.append(dict(s=s, rings=rings, closed=closed, pen=pen, z=s["z"], idx=idx, parent=parents.get(s["bone"])))
    items.sort(key=lambda it: (it["z"], it["idx"]))
    for o, it in enumerate(items): it["order"] = o; it["occ"] = it["closed"] and it["s"]["filled"] and it["s"]["role"] in ("body", "detail", "texture")
    return items

def joined(a, b):
    sa, sb = a["s"], b["s"]
    return sa.get("region") is not None and sa.get("region") == sb.get("region") and (sa["bone"] == sb["bone"] or a["parent"] == sb["bone"] or b["parent"] == sa["bone"])

def runs_of(items):
    out = []
    for p in items:
        hiders = [q["rings"] for q in items if q["occ"] and q is not p and (q["order"] > p["order"] or joined(p, q))]
        for ring in p["rings"]:
            for run in visible_runs(ring, p["closed"], hiders): out.append((p, run))
    return out

def draw(asset_path, pose, scale, out, zoom=40, marks=(), only=None):
    asset = json.load(open(asset_path))["asset"]; rep = asset["representations"][str(scale)]
    items = prepare(asset, rep, pose, scale)
    xs = [p[0] for it in items for r in it["rings"] for p in r]; ys = [p[1] for it in items for r in it["rings"] for p in r]
    x0, y0 = min(xs)-1, min(ys)-1; W_, H_ = max(xs)-x0+1, max(ys)-y0+1
    g = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{int(W_*zoom)}" height="{int(H_*zoom)}" viewBox="0 0 {W_:.3f} {H_:.3f}"><rect width="100%" height="100%" fill="white"/>']
    d = lambda pts, c: "M" + "L".join(f"{p[0]-x0:.3f} {p[1]-y0:.3f}" for p in pts) + ("Z" if c else "")
    for it in items:
        if it["closed"] and it["s"]["filled"]:
            t = it["s"].get("tone", 0.5); v = int(255-t*200)
            for r in it["rings"]: g.append(f'<path d="{d(r, True)}" fill="rgb({v},{v},{v})"/>')
    for p, run in runs_of(items):
        g.append(f'<path d="{d(run, False)}" fill="none" stroke="black" stroke-width="{p["pen"]}" stroke-linecap="round" stroke-linejoin="round"/>')
    for (mx, my) in marks:
        g.append(f'<circle cx="{mx-x0:.3f}" cy="{my-y0:.3f}" r="{max(0.25, 8/zoom):.3f}" fill="none" stroke="red" stroke-width="{2/zoom:.4f}"/>')
    g.append("</svg>")
    svg = out[:-4]+".svg"; open(svg, "w").write("\n".join(g)); subprocess.run(["rsvg-convert", svg, "-o", out], check=True); os.remove(svg)

if __name__ == "__main__":
    f, pose, scale, out = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[4]
    zoom = float(sys.argv[5]) if len(sys.argv) > 5 else 40
    marks = [tuple(float(v) for v in m.split(",")) for m in sys.argv[6:]]
    draw(f, pose, scale, out, zoom, marks)
