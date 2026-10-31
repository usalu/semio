"""NW-VE scratch library: shape constructors, rest/pose forward kinematics, bbox self-check and an SVG preview (rendered with rsvg-convert).
Scratch only: every shape in the asset scripts is designed by hand; nothing here simplifies geometry."""
import json, math, re, subprocess, os

S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/"
OUT = T + "🗑️generated/NW-VE/"


def tx(en, de):
    return {"en": en, "de": de}


def _common(id, bone, role, z, filled, tone, region, regions, material, hatch, pen):
    d = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        d["region"] = region
    if regions:
        d["regions"] = regions
    if material:
        d["material"] = material
    if hatch:
        d["hatch"] = hatch
    if pen:
        d["penMm"] = pen
    return d


def path(id, bone, role, z, d, tone=0.5, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
    return {"kind": "path", "d": d, **_common(id, bone, role, z, filled, tone, region, regions, material, hatch, pen)}


def cap(id, bone, role, z, x0, y0, x1, y1, r0, r1=None, tone=0.5, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
    return {"kind": "capsule", "x0": x0, "y0": y0, "x1": x1, "y1": y1, "r0": r0, "r1": r0 if r1 is None else r1, **_common(id, bone, role, z, filled, tone, region, regions, material, hatch, pen)}


def ell(id, bone, role, z, cx, cy, rx, ry=None, tone=0.5, filled=True, rot=None, region=None, regions=None, material=None, hatch=None, pen=None):
    d = {"kind": "ellipse", "cx": cx, "cy": cy, "rx": rx, "ry": rx if ry is None else ry, **_common(id, bone, role, z, filled, tone, region, regions, material, hatch, pen)}
    if rot:
        d["rotationDeg"] = rot
    return d


def rect(id, bone, role, z, x, y, w, h, r=None, tone=0.5, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
    d = {"kind": "rect", "x": x, "y": y, "w": w, "h": h, **_common(id, bone, role, z, filled, tone, region, regions, material, hatch, pen)}
    if r:
        d["r"] = r
    return d


def poly(id, bone, role, z, pts, closed=True, tone=0.5, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
    return {"kind": "poly", "points": [list(p) for p in pts], "closed": closed, **_common(id, bone, role, z, filled, tone, region, regions, material, hatch, pen)}


def pens(body, detail, texture, glazing, ground):
    return {"body": body, "detail": detail, "texture": texture, "glazing": glazing, "ground": ground}


def hatch(id, en, de, pattern, angle, spacing, pen):
    return {"id": id, "title": tx(en, de), "pattern": pattern, "angleDeg": angle, "spacingMm": spacing, "penMm": pen}


def rep(den, lo, hi, pn, hatches, elevation, decisions, plan=None):
    g = {"elevation": elevation}
    if plan:
        g["plan"] = plan
    return {"status": "authored", "denominator": den, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": pn, "hatches": hatches, "geometry": g, "decisions": decisions}


def decisions(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: v for k, v in zip(keys, [silhouette, retained, merged, omitted, thin, hatching, styles, overlays])}


def convert_v1_shapes(shapes, drop=(), tone_fix=None):
    out = []
    for s in shapes:
        if s["id"] in drop:
            continue
        n = {k: v for k, v in s.items() if k != "lod"}
        out.append(n)
    return out


# ---------- kinematics and geometry sampling ----------

def mat(a, b, c, d, e, f):
    return (a, b, c, d, e, f)


def mul(m, n):
    a, b, c, d, e, f = m
    A, B, C, D, E, F = n
    return (a * A + c * B, b * A + d * B, a * C + c * D, b * C + d * D, a * E + c * F + e, b * E + d * F + f)


def trs(x, y, deg):
    r = math.radians(deg)
    return (math.cos(r), math.sin(r), -math.sin(r), math.cos(r), x, y)


def world(asset, pose=None):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    off = (pose or {}).get("bones", {})
    root = (pose or {}).get("root", {}) or {}
    cache = {}

    def w(bid):
        if bid in cache:
            return cache[bid]
        b = bones[bid]
        o = off.get(bid, {})
        local = trs(b["x"] + o.get("dx", 0), b["y"] + o.get("dy", 0), b["angleDeg"] + o.get("angleDeg", 0))
        if b["parent"] is None:
            base = trs(root.get("x", 0), root.get("y", 0), root.get("angleDeg", 0))
            m = mul(base, local)
        else:
            m = mul(w(b["parent"]), local)
        cache[bid] = m
        return m

    return {k: w(k) for k in bones}


def ap(m, p):
    return (m[0] * p[0] + m[2] * p[1] + m[4], m[1] * p[0] + m[3] * p[1] + m[5])



def arc_pts(p0, rx, ry, rot, la, sw, p1, n=24):
    x1, y1 = p0
    x2, y2 = p1
    if rx == 0 or ry == 0:
        return [p1]
    phi = math.radians(rot)
    dx, dy = (x1 - x2) / 2, (y1 - y2) / 2
    x1p = math.cos(phi) * dx + math.sin(phi) * dy
    y1p = -math.sin(phi) * dx + math.cos(phi) * dy
    lam = x1p ** 2 / rx ** 2 + y1p ** 2 / ry ** 2
    if lam > 1:
        rx *= math.sqrt(lam)
        ry *= math.sqrt(lam)
    num_ = rx ** 2 * ry ** 2 - rx ** 2 * y1p ** 2 - ry ** 2 * x1p ** 2
    den = rx ** 2 * y1p ** 2 + ry ** 2 * x1p ** 2
    co = math.sqrt(max(0, num_ / den)) * (-1 if la == sw else 1)
    cxp = co * rx * y1p / ry
    cyp = -co * ry * x1p / rx
    cx = math.cos(phi) * cxp - math.sin(phi) * cyp + (x1 + x2) / 2
    cy = math.sin(phi) * cxp + math.cos(phi) * cyp + (y1 + y2) / 2
    ang = lambda ux, uy, vx, vy: math.atan2(ux * vy - uy * vx, ux * vx + uy * vy)
    th1 = ang(1, 0, (x1p - cxp) / rx, (y1p - cyp) / ry)
    dth = ang((x1p - cxp) / rx, (y1p - cyp) / ry, (-x1p - cxp) / rx, (-y1p - cyp) / ry)
    if not sw and dth > 0:
        dth -= 2 * math.pi
    elif sw and dth < 0:
        dth += 2 * math.pi
    out = []
    for k in range(1, n + 1):
        t = th1 + dth * k / n
        out.append((cx + rx * math.cos(t) * math.cos(phi) - ry * math.sin(t) * math.sin(phi), cy + rx * math.cos(t) * math.sin(phi) + ry * math.sin(t) * math.cos(phi)))
    return out


def path_pts(d, n=14):
    toks = re.findall(r"[MLHVCQSAZmlhvcqsaz]|-?\d*\.?\d+(?:e-?\d+)?", d)
    i = 0
    pts = []
    cur = (0, 0)
    start = (0, 0)
    cmd = None
    while i < len(toks):
        t = toks[i]
        if re.match(r"[A-Za-z]", t):
            cmd = t
            i += 1
            if cmd in "Zz":
                pts.append(start)
                cur = start
            continue
        rel = cmd.islower()
        c = cmd.upper()
        def num(k):
            return float(toks[i + k])
        if c == "M":
            x, y = num(0), num(1)
            if rel:
                x, y = cur[0] + x, cur[1] + y
            cur = start = (x, y)
            pts.append(cur)
            i += 2
            cmd = "l" if rel else "L"
        elif c == "L":
            x, y = num(0), num(1)
            if rel:
                x, y = cur[0] + x, cur[1] + y
            cur = (x, y)
            pts.append(cur)
            i += 2
        elif c == "H":
            x = num(0)
            cur = (cur[0] + x if rel else x, cur[1])
            pts.append(cur)
            i += 1
        elif c == "V":
            y = num(0)
            cur = (cur[0], cur[1] + y if rel else y)
            pts.append(cur)
            i += 1
        elif c == "C":
            v = [num(k) for k in range(6)]
            if rel:
                v = [v[0] + cur[0], v[1] + cur[1], v[2] + cur[0], v[3] + cur[1], v[4] + cur[0], v[5] + cur[1]]
            p0 = cur
            for k in range(1, n + 1):
                u = k / n
                a = (1 - u) ** 3
                b = 3 * (1 - u) ** 2 * u
                c2 = 3 * (1 - u) * u * u
                d2 = u ** 3
                pts.append((a * p0[0] + b * v[0] + c2 * v[2] + d2 * v[4], a * p0[1] + b * v[1] + c2 * v[3] + d2 * v[5]))
            cur = (v[4], v[5])
            i += 6
        elif c == "Q":
            v = [num(k) for k in range(4)]
            if rel:
                v = [v[0] + cur[0], v[1] + cur[1], v[2] + cur[0], v[3] + cur[1]]
            p0 = cur
            for k in range(1, n + 1):
                u = k / n
                pts.append(((1 - u) ** 2 * p0[0] + 2 * (1 - u) * u * v[0] + u * u * v[2], (1 - u) ** 2 * p0[1] + 2 * (1 - u) * u * v[1] + u * u * v[3]))
            cur = (v[2], v[3])
            i += 4
        elif c == "A":
            rx, ry, rot, la, sw, x, y = [num(k) for k in range(7)]
            if rel:
                x, y = cur[0] + x, cur[1] + y
            pts.extend(arc_pts(cur, rx, ry, rot, int(la), int(sw), (x, y)))
            cur = (x, y)
            i += 7
        else:
            raise ValueError("path command " + c)
    return pts


def shape_pts(s):
    k = s["kind"]
    if k == "path":
        return path_pts(s["d"])
    if k == "poly":
        return [tuple(p) for p in s["points"]]
    if k == "rect":
        x, y, w, h = s["x"], s["y"], s["w"], s["h"]
        return [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
    if k == "ellipse":
        r = math.radians(s.get("rotationDeg", 0))
        out = []
        for i in range(32):
            a = 2 * math.pi * i / 32
            px, py = s["rx"] * math.cos(a), s["ry"] * math.sin(a)
            out.append((s["cx"] + px * math.cos(r) - py * math.sin(r), s["cy"] + px * math.sin(r) + py * math.cos(r)))
        return out
    if k == "capsule":
        out = []
        for (cx, cy, r) in ((s["x0"], s["y0"], s["r0"]), (s["x1"], s["y1"], s["r1"])):
            for i in range(16):
                a = 2 * math.pi * i / 16
                out.append((cx + r * math.cos(a), cy + r * math.sin(a)))
        return out
    raise ValueError(k)


def bbox(asset, shapes, pose=None):
    W = world(asset, pose)
    xs, ys = [], []
    for s in shapes:
        if s["role"] == "ground":
            continue
        for p in shape_pts(s):
            x, y = ap(W[s["bone"]], p)
            xs.append(x)
            ys.append(y)
    return min(xs), min(ys), max(xs), max(ys)


# ---------- SVG preview ----------

def tone_col(t):
    lo, hi = (0x24, 0x32, 0x35), (0xd3, 0xd2, 0xc5)
    return "#%02x%02x%02x" % tuple(round(lo[i] + (hi[i] - lo[i]) * t) for i in range(3))


def svg_shape(s, W, pen_set, mm_per_m, show_fill=True):
    m = W[s["bone"]]
    k = s["kind"]
    pen = s.get("penMm") or pen_set.get(s["role"])
    if pen is None:
        return ""
    sw = pen / mm_per_m
    fill = tone_col(s["tone"]) if (s["filled"] and show_fill) else "none"
    tr = "matrix(%f %f %f %f %f %f)" % m
    sty = 'fill-rule="evenodd" fill="%s" stroke="#001117" stroke-width="%f" stroke-linejoin="round" stroke-linecap="round"' % (fill, sw)
    if k == "path":
        el = '<path d="%s" %s/>' % (s["d"], sty)
    elif k == "poly":
        pts = " ".join("%f,%f" % tuple(p) for p in s["points"])
        el = '<%s points="%s" %s/>' % ("polygon" if s["closed"] else "polyline", pts, sty)
        if not s["closed"]:
            el = el.replace('fill="%s"' % fill, 'fill="none"')
    elif k == "rect":
        el = '<rect x="%f" y="%f" width="%f" height="%f" rx="%f" %s/>' % (s["x"], s["y"], s["w"], s["h"], s.get("r", 0), sty)
    elif k == "ellipse":
        rot = s.get("rotationDeg", 0)
        el = '<ellipse cx="%f" cy="%f" rx="%f" ry="%f" transform="rotate(%f %f %f)" %s/>' % (s["cx"], s["cy"], s["rx"], s["ry"], rot, s["cx"], s["cy"], sty)
    else:
        x0, y0, x1, y1, r0, r1 = s["x0"], s["y0"], s["x1"], s["y1"], s["r0"], s["r1"]
        dx, dy = x1 - x0, y1 - y0
        L = math.hypot(dx, dy)
        if L < 1e-9:
            el = '<circle cx="%f" cy="%f" r="%f" %s/>' % (x0, y0, r0, sty)
        else:
            ang = math.atan2(dy, dx)
            a = math.asin(max(-1, min(1, (r0 - r1) / L)))
            p = []
            for (cx, cy, r, sgn) in ((x0, y0, r0, 1), (x1, y1, r1, -1)):
                pass
            n1 = ang + math.pi / 2 + a
            n2 = ang - math.pi / 2 - a
            P0a = (x0 + r0 * math.cos(n1), y0 + r0 * math.sin(n1))
            P1a = (x1 + r1 * math.cos(n1), y1 + r1 * math.sin(n1))
            P1b = (x1 + r1 * math.cos(n2), y1 + r1 * math.sin(n2))
            P0b = (x0 + r0 * math.cos(n2), y0 + r0 * math.sin(n2))
            d = "M %f,%f L %f,%f A %f,%f 0 1 0 %f,%f L %f,%f A %f,%f 0 1 0 %f,%f Z" % (P0a + P1a + (r1, r1) + P1b + P0b + (r0, r0) + P0a)
            el = '<path d="%s" %s/>' % (d, sty)
    return '<g transform="%s">%s</g>' % (tr, el)


def render(asset, rep, pose, den, outfile, ppm=24, view=None, show_fill=True, pad_m=0.5):
    """ppm = pixels per paper mm. view=(x0,y0,x1,y1) in metres to zoom."""
    W = world(asset, pose)
    mm_per_m = 1000.0 / den
    shapes = sorted(rep["geometry"]["elevation"], key=lambda s: s["z"])
    pn = rep["pens"]
    x0, y0, x1, y1 = view or (lambda b: (b[0] - pad_m, b[1] - pad_m, b[2] + pad_m, b[3] + pad_m))(bbox(asset, shapes, pose))
    sc = mm_per_m * ppm
    wpx, hpx = (x1 - x0) * sc, (y1 - y0) * sc
    body = "".join(svg_shape(s, W, pn, mm_per_m, show_fill) for s in shapes)
    gy = pn.get("ground") or 0.2
    gl = '<line x1="%f" y1="0" x2="%f" y2="0" stroke="#001117" stroke-width="%f"/>' % (x0, x1, gy / mm_per_m)
    svg = '<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="0 0 %d %d"><rect width="100%%" height="100%%" fill="#f7f3e3"/><g transform="matrix(%f 0 0 %f %f %f)">%s%s</g></svg>' % (
        wpx, hpx, wpx, hpx, sc, -sc, -x0 * sc, y1 * sc, gl, body)
    os.makedirs(os.path.dirname(outfile), exist_ok=True)
    sf = outfile.replace(".png", ".svg")
    open(sf, "w").write(svg)
    subprocess.run(["rsvg-convert", "-o", outfile, sf], check=True)
    os.remove(sf)
    return outfile


def load(rel):
    return json.load(open(S + rel + "/🔣️.json"))


def save(rel, doc):
    with open(S + rel + "/🔣️.json", "w") as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
        f.write("\n")
