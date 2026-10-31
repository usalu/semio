"""CV-H2 scratch helpers: forward kinematics, world<->bone-local conversion of hand-authored coordinates, SVG/PNG preview.
No content is generated here; all outlines and numbers are authored in the per-asset data files."""
import json, math, re, subprocess, os, copy

S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
H = S + "/🗂️catalog/👤️humans/"
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
GEN = T + "/🗑️generated/CV-H2/"


def mul(p, l):
    return [p[0]*l[0]+p[2]*l[1], p[1]*l[0]+p[3]*l[1], p[0]*l[2]+p[2]*l[3], p[1]*l[2]+p[3]*l[3],
            p[0]*l[4]+p[2]*l[5]+p[4], p[1]*l[4]+p[3]*l[5]+p[5]]


def place(x, y, deg):
    r = math.radians(deg); c, s = math.cos(r), math.sin(r)
    return [c, s, -s, c, x, y]


def inv(m):
    det = m[0]*m[3]-m[1]*m[2]
    return [m[3]/det, -m[1]/det, -m[2]/det, m[0]/det, (m[2]*m[5]-m[3]*m[4])/det, (m[1]*m[4]-m[0]*m[5])/det]


def ap(m, x, y):
    return (m[0]*x+m[2]*y+m[4], m[1]*x+m[3]*y+m[5])


def load(asset_id):
    return json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/v1/" + asset_id + ".json"))


def order(rig):
    by = {b["id"]: b for b in rig["bones"]}; out = []; seen = set()
    def v(b):
        if b["id"] in seen: return
        if b["parent"]: v(by[b["parent"]])
        seen.add(b["id"]); out.append(b)
    for b in rig["bones"]: v(b)
    return out


def clampj(j, a):
    if a is None: a = 0
    if j.get("minDeg") is not None: a = max(a, j["minDeg"])
    if j.get("maxDeg") is not None: a = min(a, j["maxDeg"])
    return a


def solve(asset, offs, root):
    w = {}
    o = place(root[0], root[1], root[2])
    for b in order(asset["rig"]):
        off = offs.get(b["id"], {}); j = b["joint"]; dx = dy = 0; ang = 0
        if j["type"] == "revolute": ang = clampj(j, off.get("angleDeg", 0))
        elif j["type"] == "prismatic":
            ax = math.radians(j.get("axisDeg", 0)); s = off.get("dx", 0)*math.cos(ax)+off.get("dy", 0)*math.sin(ax)
            if j.get("minM") is not None: s = max(s, j["minM"])
            if j.get("maxM") is not None: s = min(s, j["maxM"])
            dx, dy = s*math.cos(ax), s*math.sin(ax)
        loc = place(b["x"]+dx, b["y"]+dy, b["angleDeg"]+ang)
        w[b["id"]] = mul(o if b["parent"] is None else w[b["parent"]], loc)
    return w


def pose_world(asset, pose_id=None, overrides=None):
    pid = pose_id or asset["presentation"]["defaultPose"]
    p = next(q for q in asset["poses"] if q["id"] == pid)
    offs = dict(p["bones"])
    if overrides:
        for k, v in overrides.items(): offs[k] = {**offs.get(k, {}), **v}
    r = p.get("root", {})
    rx, ry, ra = r.get("x", 0), r.get("y", 0), r.get("angleDeg", 0)
    if p["ground"] == "auto" and p["contacts"]:
        w0 = solve(asset, offs, (0, 0, ra)); anc = {a["id"]: a for a in asset["anchors"]}
        low = min(ap(w0[anc[c]["bone"]], anc[c]["x"], anc[c]["y"])[1] for c in p["contacts"])
        ry = -low
    return solve(asset, offs, (rx, ry, ra))


# ---- path handling -------------------------------------------------------------------------------------------------
TOK = re.compile(r"[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?")


def path_pts(d):
    t = TOK.findall(d); out = []; i = 0
    while i < len(t):
        c = t[i]; i += 1
        if c == "Z": out.append(("Z", [])); continue
        n = {"M": 1, "L": 1, "C": 3, "Q": 2}[c]
        out.append((c, [(float(t[i+2*k]), float(t[i+2*k+1])) for k in range(n)])); i += 2*n
    return out


def fmt(v):
    s = f"{v:.4f}".rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


def path_str(segs, m=None):
    o = []
    for c, ps in segs:
        if c == "Z": o.append("Z"); continue
        q = [ap(m, x, y) for x, y in ps] if m else ps
        o.append(c + " " + " ".join(f"{fmt(x)} {fmt(y)}" for x, y in q))
    return " ".join(o)


def to_local(world, d):
    return path_str(path_pts(d), inv(world))


def kappa_circle(cx, cy, rx, ry=None):
    ry = rx if ry is None else ry; k = 0.5522847498
    return (f"M {cx+rx} {cy} C {cx+rx} {cy+k*ry} {cx+k*rx} {cy+ry} {cx} {cy+ry} C {cx-k*rx} {cy+ry} {cx-rx} {cy+k*ry} {cx-rx} {cy} "
            f"C {cx-rx} {cy-k*ry} {cx-k*rx} {cy-ry} {cx} {cy-ry} C {cx+k*rx} {cy-ry} {cx+rx} {cy-k*ry} {cx+rx} {cy} Z")


def stadium(x0, y0, x1, y1, r0, r1=None):
    """world-space path of a tapered stadium (cheap cubic ends)"""
    r1 = r0 if r1 is None else r1
    dx, dy = x1-x0, y1-y0; L = math.hypot(dx, dy); ux, uy = dx/L, dy/L; nx, ny = -uy, ux; k = 0.5522847498
    a = (x0+nx*r0, y0+ny*r0); b = (x1+nx*r1, y1+ny*r1); c = (x1-nx*r1, y1-ny*r1); e = (x0-nx*r0, y0-ny*r0)
    def p(q): return f"{fmt(q[0])} {fmt(q[1])}"
    return (f"M {p(a)} L {p(b)} C {p((b[0]+ux*k*r1, b[1]+uy*k*r1))} {p((x1+ux*r1+nx*k*r1, y1+uy*r1+ny*k*r1))} {p((x1+ux*r1, y1+uy*r1))} "
            f"C {p((x1+ux*r1-nx*k*r1, y1+uy*r1-ny*k*r1))} {p((c[0]+ux*k*r1, c[1]+uy*k*r1))} {p(c)} L {p(e)} "
            f"C {p((e[0]-ux*k*r0, e[1]-uy*k*r0))} {p((x0-ux*r0-nx*k*r0, y0-uy*r0-ny*k*r0))} {p((x0-ux*r0, y0-uy*r0))} "
            f"C {p((x0-ux*r0+nx*k*r0, y0-uy*r0+ny*k*r0))} {p((a[0]-ux*k*r0, a[1]-uy*k*r0))} {p(a)} Z")


def poly_world(pts, closed=False):
    return ("M " + " L ".join(f"{fmt(x)} {fmt(y)}" for x, y in pts)) + (" Z" if closed else "")


def shape_path_local(s):
    k = s["kind"]
    if k == "path": return path_pts(s["d"])
    if k == "poly":
        segs = [("M" if i == 0 else "L", [tuple(p)]) for i, p in enumerate(s["points"])]
        if s["closed"]: segs.append(("Z", []))
        return segs
    if k == "rect":
        x, y, w, h = s["x"], s["y"], s["w"], s["h"]
        return [("M", [(x, y)]), ("L", [(x+w, y)]), ("L", [(x+w, y+h)]), ("L", [(x, y+h)]), ("Z", [])]
    if k == "ellipse":
        cx, cy, rx, ry = s["cx"], s["cy"], s["rx"], s["ry"]; rot = s.get("rotationDeg", 0)
        segs = path_pts(kappa_circle(cx, cy, rx, ry))
        if rot:
            m = mul(place(cx, cy, rot), place(-cx, -cy, 0)); m = mul(place(cx, cy, rot), [1, 0, 0, 1, -cx, -cy])
            segs = [(c, [ap(m, x, y) for x, y in ps]) for c, ps in segs]
        return segs
    if k == "capsule":
        return path_pts(stadium(s["x0"], s["y0"], s["x1"], s["y1"], s["r0"], s["r1"]))
    raise ValueError(k)


# ---- SVG preview ---------------------------------------------------------------------------------------------------
def tone_col(t):
    t = max(0, min(1, t)); a = (0x24, 0x32, 0x35); b = (0xd3, 0xd2, 0xc5)
    return "#%02x%02x%02x" % tuple(round(a[i]+(b[i]-a[i])*t) for i in range(3))


def render_svg(shapes, world, denom, pens, hatches=None, bounds=None, mode="line", zoom=1.0, pad=0.12, ground=True, wmax=None, extra=""):
    """paper-true SVG: 1 unit = 1 mm paper; shapes drawn in z order; line mode strokes with the pens, solid mode fills in tone."""
    k = 1000.0/denom
    allp = []
    items = []
    for s in sorted(shapes, key=lambda s: s["z"]):
        segs = shape_path_local(s)
        w = world[s["bone"]]
        ws = [(c, [ap(w, x, y) for x, y in ps]) for c, ps in segs]
        for c, ps in ws: allp += ps
        items.append((s, ws))
    if bounds is None:
        xs = [p[0] for p in allp]; ys = [p[1] for p in allp]
        bounds = (min(xs)-pad, 0-0.02, max(xs)+pad, max(ys)+pad)
    x0, y0, x1, y1 = bounds
    W = (x1-x0)*k; Hh = (y1-y0)*k
    def tr(p): return ((p[0]-x0)*k, (y1-p[1])*k)
    o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W*zoom:.3f}mm" height="{Hh*zoom:.3f}mm" viewBox="0 0 {W:.4f} {Hh:.4f}"><rect width="100%" height="100%" fill="#ffffff"/>']
    if ground:
        gy = (y1-0)*k
        o.append(f'<line x1="0" y1="{gy:.4f}" x2="{W:.4f}" y2="{gy:.4f}" stroke="#888" stroke-width="{pens["ground"]}"/>')
    pen = lambda s: s.get("penMm") or {"body": pens["body"], "detail": pens["detail"], "texture": pens["texture"], "glazing": pens["glazing"], "ground": pens["ground"]}[s["role"]]
    for s, ws in items:
        pw = pen(s)
        if pw is None: continue
        d = " ".join(("Z" if c == "Z" else c + " " + " ".join(f"{tr(p)[0]:.4f} {tr(p)[1]:.4f}" for p in ps)) for c, ps in ws)
        if mode == "solid":
            fill = tone_col(s["tone"]) if s["filled"] else "none"
            o.append(f'<path d="{d}" fill="{fill}" stroke="#001117" stroke-width="{pw*0.5:.3f}" stroke-linejoin="round" stroke-linecap="round"/>')
        else:
            fill = "#ffffff" if s["filled"] else "none"
            o.append(f'<path d="{d}" fill="{fill}" stroke="#001117" stroke-width="{pw:.3f}" stroke-linejoin="round" stroke-linecap="round"/>')
    o.append(extra); o.append("</svg>")
    return "\n".join(o)


def to_png(svg, name, dpi=600):
    os.makedirs(GEN, exist_ok=True)
    f = GEN + name + ".svg"; open(f, "w").write(svg)
    subprocess.run(["rsvg-convert", "-d", str(dpi), "-p", str(dpi), "-o", GEN + name + ".png", f], check=True)
    return GEN + name + ".png"


def overlay(asset, world, bounds, denom, grid=0.1, bones=True, anchors=True):
    """grid in metres (labels in cm) plus bone axes and anchors for design work"""
    k = 1000.0/denom; x0, y0, x1, y1 = bounds
    tr = lambda x, y: ((x-x0)*k, (y1-y)*k)
    o = []
    gx = math.ceil(x0/grid)*grid
    while gx <= x1:
        a, b = tr(gx, y0); c, d = tr(gx, y1); strong = abs(round(gx/0.5)*0.5-gx) < 1e-6
        o.append(f'<line x1="{a:.3f}" y1="{b:.3f}" x2="{c:.3f}" y2="{d:.3f}" stroke="{"#e07070" if strong else "#e6c3c3"}" stroke-width="0.12"/>')
        o.append(f'<text x="{a+0.3:.2f}" y="{d+3:.2f}" font-size="2.6" fill="#b04040">{int(round(gx*100))}</text>'); gx += grid
    gy = math.ceil(y0/grid)*grid
    while gy <= y1:
        a, b = tr(x0, gy); c, d = tr(x1, gy); strong = abs(round(gy/0.5)*0.5-gy) < 1e-6
        o.append(f'<line x1="{a:.3f}" y1="{b:.3f}" x2="{c:.3f}" y2="{d:.3f}" stroke="{"#e07070" if strong else "#e6c3c3"}" stroke-width="0.12"/>')
        o.append(f'<text x="{a+0.3:.2f}" y="{b-0.4:.2f}" font-size="2.6" fill="#b04040">{int(round(gy*100))}</text>'); gy += grid
    if bones:
        for b in asset["rig"]["bones"]:
            w = world[b["id"]]; p0 = ap(w, 0, 0); p1 = ap(w, b["lengthM"], 0)
            a, bb = tr(*p0); c, d = tr(*p1)
            o.append(f'<line x1="{a:.3f}" y1="{bb:.3f}" x2="{c:.3f}" y2="{d:.3f}" stroke="#2060d0" stroke-width="0.3"/><circle cx="{a:.3f}" cy="{bb:.3f}" r="0.8" fill="#2060d0"/><text x="{c+0.5:.2f}" y="{d:.2f}" font-size="2.2" fill="#2060d0">{b["id"]}</text>')
    if anchors:
        for an in asset["anchors"]:
            p = ap(world[an["bone"]], an["x"], an["y"]); a, bb = tr(*p)
            o.append(f'<circle cx="{a:.3f}" cy="{bb:.3f}" r="0.9" fill="none" stroke="#10a050" stroke-width="0.3"/><text x="{a+1:.2f}" y="{bb:.2f}" font-size="2" fill="#10a050">{an["id"]}</text>')
    return "\n".join(o)
