import json, math, re, sys, os
sys.stdout.reconfigure(encoding="utf8")
ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/"
SCALES = ["50", "100", "200", "500"]


def path_of(aid):
    return ROOT + aid + "/🔣️.json"


def read_doc(aid):
    raw = open(path_of(aid), "rb").read().decode("utf8")
    crlf = "\r\n" in raw
    return json.loads(raw), crlf


def write_doc(aid, doc, crlf):
    text = json.dumps(doc, ensure_ascii=False, indent=1)
    if crlf:
        text = text.replace("\n", "\r\n")
    open(path_of(aid), "wb").write((text + ("\r\n" if crlf else "\n")).encode("utf8"))


def rest_world(asset):
    w = {}
    for b in asset["rig"]["bones"]:
        p = b["parent"]
        if p is None:
            w[b["id"]] = (b["x"], b["y"], b["angleDeg"])
        else:
            px, py, pa = w[p]
            r = math.radians(pa)
            w[b["id"]] = (px + b["x"] * math.cos(r) - b["y"] * math.sin(r), py + b["x"] * math.sin(r) + b["y"] * math.cos(r), pa + b["angleDeg"])
    return w


TOK = re.compile(r"[MLCQZmlcqz]|-?\d*\.?\d+(?:e-?\d+)?")


def sample_path(d, per=8):
    toks = TOK.findall(d)
    pts, i, cur, cmd = [], 0, (0.0, 0.0), None
    while i < len(toks):
        t = toks[i]
        if t.isalpha():
            cmd = t
            i += 1
            if cmd in "Zz":
                continue
        if cmd in "ML":
            cur = (float(toks[i]), float(toks[i + 1]))
            pts.append(cur)
            i += 2
        elif cmd == "C":
            p1, p2, p3 = [(float(toks[i + 2 * k]), float(toks[i + 2 * k + 1])) for k in range(3)]
            for s in range(1, per + 1):
                u = s / per
                a, b, c, e = (1 - u) ** 3, 3 * u * (1 - u) ** 2, 3 * u * u * (1 - u), u ** 3
                pts.append((a * cur[0] + b * p1[0] + c * p2[0] + e * p3[0], a * cur[1] + b * p1[1] + c * p2[1] + e * p3[1]))
            cur = p3
            i += 6
        elif cmd == "Q":
            p1, p2 = [(float(toks[i + 2 * k]), float(toks[i + 2 * k + 1])) for k in range(2)]
            for s in range(1, per + 1):
                u = s / per
                pts.append(((1 - u) ** 2 * cur[0] + 2 * u * (1 - u) * p1[0] + u * u * p2[0], (1 - u) ** 2 * cur[1] + 2 * u * (1 - u) * p1[1] + u * u * p2[1]))
            cur = p2
            i += 4
        else:
            i += 1
    return pts


def shape_pts(shape, per=8):
    k = shape["kind"]
    if k == "path":
        return sample_path(shape["d"], per)
    if k == "poly":
        return [tuple(p) for p in shape["points"]]
    if k == "rect":
        x, y, w, h = shape["x"], shape["y"], shape["w"], shape["h"]
        return [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
    if k == "ellipse":
        cr, sr = math.cos(math.radians(shape.get("rotationDeg", 0))), math.sin(math.radians(shape.get("rotationDeg", 0)))
        pp = [(shape["rx"] * math.cos(t / 16 * 2 * math.pi), shape["ry"] * math.sin(t / 16 * 2 * math.pi)) for t in range(16)]
        return [(shape["cx"] + x * cr - y * sr, shape["cy"] + x * sr + y * cr) for x, y in pp]
    if k == "capsule":
        return [(shape["x0"] - shape["r0"], shape["y0"] - shape["r0"]), (shape["x0"] + shape["r0"], shape["y0"] + shape["r0"]), (shape["x1"] - shape["r1"], shape["y1"] - shape["r1"]), (shape["x1"] + shape["r1"], shape["y1"] + shape["r1"])]
    raise ValueError(k)


def elev_bounds(asset, scale):
    w = rest_world(asset)
    xs, ys = [], []
    for s in asset["representations"][scale]["geometry"]["elevation"]:
        if s["role"] == "ground":
            continue
        ox, oy, ang = w[s["bone"]]
        c, sn = math.cos(math.radians(ang)), math.sin(math.radians(ang))
        for px, py in shape_pts(s):
            xs.append(ox + px * c - py * sn)
            ys.append(oy + px * sn + py * c)
    return min(xs), max(xs), min(ys), max(ys)


def view_bounds(shapes, plan=False):
    xs, ys = [], []
    for s in shapes:
        if s["role"] == "ground":
            continue
        for px, py in shape_pts(s):
            xs.append(px)
            ys.append(py)
    return min(xs), max(xs), min(ys), max(ys)


def fmt(v):
    r = round(v, 4)
    if r == 0:
        r = 0.0
    s = ("%.4f" % r).rstrip("0").rstrip(".")
    return s if s not in ("-0", "") else "0"


def closed_path(pts):
    n = len(pts)
    out = ["M %s %s" % (fmt(pts[0][0]), fmt(pts[0][1]))]
    for i in range(n):
        p0, p1, p2, p3 = pts[(i - 1) % n], pts[i], pts[(i + 1) % n], pts[(i + 2) % n]
        c1 = (p1[0] + (p2[0] - p0[0]) / 6, p1[1] + (p2[1] - p0[1]) / 6)
        c2 = (p2[0] - (p3[0] - p1[0]) / 6, p2[1] - (p3[1] - p1[1]) / 6)
        out.append("C %s %s %s %s %s %s" % (fmt(c1[0]), fmt(c1[1]), fmt(c2[0]), fmt(c2[1]), fmt(p2[0]), fmt(p2[1])))
    return " ".join(out) + " Z"


def open_path(pts):
    n = len(pts)
    out = ["M %s %s" % (fmt(pts[0][0]), fmt(pts[0][1]))]
    for i in range(n - 1):
        p0, p1, p2, p3 = pts[max(i - 1, 0)], pts[i], pts[i + 1], pts[min(i + 2, n - 1)]
        c1 = (p1[0] + (p2[0] - p0[0]) / 6, p1[1] + (p2[1] - p0[1]) / 6)
        c2 = (p2[0] - (p3[0] - p1[0]) / 6, p2[1] - (p3[1] - p1[1]) / 6)
        out.append("C %s %s %s %s %s %s" % (fmt(c1[0]), fmt(c1[1]), fmt(c2[0]), fmt(c2[1]), fmt(p2[0]), fmt(p2[1])))
    return " ".join(out)


def outline(cx, cy, rx, ry, n=16, k=5, amp=0.08, ph=0.0, k2=0, amp2=0.0, ph2=0.0, rot=0.0, top=1.0, bottom=1.0, skew=0.0):
    pts = []
    cr, sr = math.cos(math.radians(rot)), math.sin(math.radians(rot))
    for i in range(n):
        t = 2 * math.pi * i / n
        r = 1 + amp * math.sin(k * t + ph) + amp2 * math.sin(k2 * t + ph2)
        sy = top if math.sin(t) > 0 else bottom
        x = r * rx * math.cos(t) + skew * ry * math.sin(t)
        y = r * ry * math.sin(t) * sy
        pts.append((cx + x * cr - y * sr, cy + x * sr + y * cr))
    return pts


def blob(*a, **k):
    return closed_path(outline(*a, **k))


class Builder:
    def __init__(self, aid):
        self.aid = aid
        self.doc, self.crlf = read_doc(aid)
        self.asset = self.doc["asset"]
        self.w = rest_world(self.asset)
        self.elev = {sc: {s["id"]: s for s in self.asset["representations"][sc]["geometry"]["elevation"]} for sc in SCALES}
        self.views = {sc: {"front": [], "plan": []} for sc in SCALES}
        self.bones = {b["id"] for b in self.asset["rig"]["bones"]}

    def like(self, sc, eid):
        s = None
        for k in (sc, "50", "100", "200", "500"):
            if eid in self.elev[k]:
                s = self.elev[k][eid]
                break
        assert s is not None, eid
        out = {}
        for k in ("region", "regions", "material", "hatch", "penMm"):
            if k in s:
                out[k] = s[k]
        return out, s

    def add(self, sc, view, sid, bone, z, d=None, like=None, role="body", tone=None, filled=True, kind="path", region=None, material=None, regions=None, **geo):
        assert bone in self.bones, bone
        ids = {s["id"] for s in self.views[sc][view]}
        assert sid not in ids, (sc, view, sid)
        shape = {"id": sid, "bone": bone, "role": role, "z": round(z, 4), "filled": filled}
        meta, src = ({}, None)
        if like is not None:
            meta, src = self.like(sc, like)
        if tone is None and src is not None:
            tone = src.get("tone")
        if tone is not None:
            shape["tone"] = tone
        shape["kind"] = kind
        if kind == "path":
            shape["d"] = d
        else:
            shape.update(geo)
        if region is not None:
            meta["region"] = region
            meta.pop("regions", None)
        if regions is not None:
            meta["regions"] = regions
            meta.pop("region", None)
        if material is not None:
            meta["material"] = material
        if role in ("texture", "detail") and like is None and region is None and regions is None:
            meta = {}
        shape.update(meta)
        self.views[sc][view].append(shape)
        return shape

    def finish(self, decisions):
        asset = self.asset
        for sc in SCALES:
            g = asset["representations"][sc]["geometry"]
            g["front"] = self.views[sc]["front"]
            g["plan"] = self.views[sc]["plan"]
            dec = asset["representations"][sc]["decisions"]
            for key, (en, de) in decisions[sc].items():
                if en not in dec[key]["en"]:
                    dec[key]["en"] = dec[key]["en"].rstrip() + " " + en
                if de not in dec[key]["de"]:
                    dec[key]["de"] = dec[key]["de"].rstrip() + " " + de
        pres = asset["presentation"]
        pres.pop("viewpoints", None)
        write_doc(self.aid, self.doc, self.crlf)

    def report(self):
        a = self.asset
        dm = a["dimensions"]
        print("== %s dims w %.2f d %.2f h %.2f" % (self.aid, dm["widthM"], dm["depthM"], dm["heightM"]))
        for sc in SCALES:
            ex0, ex1, ey0, ey1 = elev_bounds(a, sc)
            fx0, fx1, fy0, fy1 = view_bounds(self.views[sc]["front"])
            px0, px1, py0, py1 = view_bounds(self.views[sc]["plan"])
            tol = max(0.02, 0.03 * max(dm["widthM"], dm["depthM"], dm["heightM"]))
            checks = [
                ("elevX~planX", ex0, ex1, px0, px1),
                ("elevY~frontY", ey0, ey1, fy0, fy1),
                ("frontX~planY", fx0, fx1, py0, py1),
            ]
            bad = []
            for name, a0, a1, b0, b1 in checks:
                if abs(a0 - b0) > tol or abs(a1 - b1) > tol:
                    bad.append("%s [%.2f,%.2f] vs [%.2f,%.2f]" % (name, a0, a1, b0, b1))
            print(" %s tol %.2f elev x[%.2f,%.2f] y[%.2f,%.2f] front x[%.2f,%.2f] y[%.2f,%.2f] plan x[%.2f,%.2f] y[%.2f,%.2f] n(front %d plan %d) %s" % (sc, tol, ex0, ex1, ey0, ey1, fx0, fx1, fy0, fy1, px0, px1, py0, py1, len(self.views[sc]["front"]), len(self.views[sc]["plan"]), "OK" if not bad else "BAD " + "; ".join(bad)))
        # note: dims check vs elevation extents
