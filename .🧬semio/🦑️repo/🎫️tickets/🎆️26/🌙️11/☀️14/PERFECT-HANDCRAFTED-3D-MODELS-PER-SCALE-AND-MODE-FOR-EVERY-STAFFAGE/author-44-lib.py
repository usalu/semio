import os, sys, math, json
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-19-lib.py")).read())

CATALOG_V = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/"
GEN = os.path.join(T, "🗑️generated", "author-44")


def asset_v(slug):
    return json.load(open(CATALOG_V + slug + "/🔣️.json"))["asset"]


def dump_of(slug):
    return json.load(open(os.path.join(GEN, slug + ".dump.json")))


def shape(slug, scale, view, id):
    for s in dump_of(slug)["scales"][str(scale)][view]:
        if s["id"] == id:
            return s["rings"][0]
    raise KeyError(id)


def slc(ring, x, axis=0):
    k, o = (0, 1) if axis == 0 else (1, 0)
    ys = []
    n = len(ring)
    for i in range(n):
        a, b = ring[i], ring[(i + 1) % n]
        if (a[k] - x) * (b[k] - x) < 0 or (a[k] == x) != (b[k] == x) and min(a[k], b[k]) <= x <= max(a[k], b[k]):
            if a[k] == b[k]:
                ys += [a[o], b[o]]
            else:
                t = (x - a[k]) / (b[k] - a[k])
                ys.append(a[o] + t * (b[o] - a[o]))
    return sorted(ys)


def span(ring, x, axis=0):
    ys = slc(ring, x, axis)
    return (ys[0], ys[-1]) if ys else None


def save_v(slug, scales, category="vehicles"):
    path = os.path.join(T, "volumes", category, slug + ".json")
    cur = json.load(open(path))
    v = cur["volumes"]
    sup = asset_v(slug)["anchors"]
    for s in scales:
        v[str(s.den)] = s.out()
    json.dump(cur, open(path, "w"), ensure_ascii=False, indent=1)
    print("saved", path, {k: (len(x.get("body", [])), len(x.get("thermal", [])), len(x.get("structure", []))) for k, x in v.items() if x["status"] == "authored"})


def xloft(id, bone, mat, tone, st, e=8, role="body", caps=True, rot=None, tags=None):
    secs = []
    for row in st:
        x, y0, y1, z0, z1 = row[:5]
        ee = row[5] if len(row) > 5 else e
        secs.append(sec([x, (y0 + y1) / 2, (z0 + z1) / 2], max((z1 - z0) / 2, 1e-4), max((y1 - y0) / 2, 1e-4), ee, rotation=Qy(math.pi / 2)))
    return loft(id, bone, mat, tone, secs, role=role, capStart=caps, capEnd=caps, rot=rot, tags=tags)


def Qz(a):
    return [0, 0, r6(math.sin(a / 2)), r6(math.cos(a / 2))]


def Qy(a):
    return [0, r6(math.sin(a / 2)), 0, r6(math.cos(a / 2))]


def Qx(a):
    return [r6(math.sin(a / 2)), 0, 0, r6(math.cos(a / 2))]


def rbox(id, bone, mat, tone, size, at, r=0.0, rot=None, role="body", tags=None):
    return box(id, bone, mat, tone, size, at, r if r else None, role, rot, None, tags)


def ref(id, of, region, layer=None, faces=None, tags=None):
    d = {"id": id, "of": of, "region": region}
    if layer:
        d["layer"] = layer
    if faces:
        d["sectionFaces"] = faces
    if tags:
        d["tags"] = tags
    return d


def sref(id, of, member, part):
    return {"id": id, "of": of, "member": member, "part": part}


def tag(d, *tags):
    d["tags"] = list(tags)
    return d


def faces(d, *f):
    d["sectionFaces"] = list(f)
    return d


def slender(d):
    d["slender"] = True
    return d


def texture(d):
    d["texture"] = True
    return d


def inset(d, t, id=None):
    c = json.loads(json.dumps(d))
    if id:
        c["id"] = id
    k = c["kind"]
    if k == "box":
        c["size"] = [r6(max(s - 2 * t, 1e-5)) for s in c["size"]]
        if "cornerRadius" in c:
            c["cornerRadius"] = r6(max(c["cornerRadius"] - t, 0)) or None
            if c["cornerRadius"] is None:
                del c["cornerRadius"]
    elif k == "ellipsoid":
        c["radii"] = [r6(max(r - t, 1e-5)) for r in c["radii"]]
    elif k == "capsule":
        c["r0"] = r6(max(c["r0"] - t, 1e-5))
        c["r1"] = r6(max(c["r1"] - t, 1e-5))
    elif k == "loft":
        ss = c["sections"]
        for s in ss:
            s["rx"] = r6(max(s["rx"] - t, 1e-5))
            s["ry"] = r6(max(s["ry"] - t, 1e-5))
        a, b = ss[0]["at"], ss[1]["at"]
        z, y = ss[-1]["at"], ss[-2]["at"]
        for s_, o_, sg in ((ss[0], ss[1], 1), (ss[-1], ss[-2], 1)):
            dv = [o_["at"][i] - s_["at"][i] for i in range(3)]
            L = math.sqrt(sum(q * q for q in dv)) or 1
            s_["at"] = [r6(s_["at"][i] + dv[i] / L * t) for i in range(3)]
    elif k == "revolve":
        pass
    elif k == "extrude":
        c["from"] = r6(c["from"] + t)
        c["to"] = r6(c["to"] - t)
    return c


def stack(*lists):
    out = []
    for l in lists:
        out.extend(l if isinstance(l, list) else [l])
    return out


def ring_poly(cx, cy, rx, ry, n=24, a0=0.0):
    return [[cx + rx * math.cos(a0 + 2 * math.pi * k / n), cy + ry * math.sin(a0 + 2 * math.pi * k / n)] for k in range(n)]


def hsl(*a):
    return a


def clip_box(ring, x0, x1, y0, y1):
    def clip(poly, inside, cut):
        out = []
        n = len(poly)
        for i in range(n):
            a, b = poly[i], poly[(i + 1) % n]
            ia, ib = inside(a), inside(b)
            if ia:
                out.append(a)
            if ia != ib:
                out.append(cut(a, b))
        return out

    def vx(v, keep_ge):
        return (lambda p: p[0] >= v if keep_ge else p[0] <= v), (lambda a, b: [v, a[1] + (v - a[0]) / (b[0] - a[0]) * (b[1] - a[1])])

    def vy(v, keep_ge):
        return (lambda p: p[1] >= v if keep_ge else p[1] <= v), (lambda a, b: [a[0] + (v - a[1]) / (b[1] - a[1]) * (b[0] - a[0]), v])

    poly = [list(p) for p in ring]
    for f in (vx(x0, True), vx(x1, False), vy(y0, True), vy(y1, False)):
        poly = clip(poly, f[0], f[1])
        if not poly:
            return []
    return [[r6(p[0]), r6(p[1])] for p in poly]


def dedup(poly, eps=1e-6):
    out = []
    for p in poly:
        if not out or abs(p[0] - out[-1][0]) > eps or abs(p[1] - out[-1][1]) > eps:
            out.append(p)
    if len(out) > 1 and abs(out[0][0] - out[-1][0]) < eps and abs(out[0][1] - out[-1][1]) < eps:
        out.pop()
    return out


def cap_prof(pts):
    p = [list(q) for q in pts]
    if p[0] != p[-1]:
        p.append(list(p[0]))
    return p


def tyre_profile(rin, rout, half, bevel=0.012):
    h = half
    return cap_prof([[-h + bevel, rin], [-h, rin + 0.03], [-h, rout - 0.03], [-h + 0.012, rout - 0.008], [-0.5 * h, rout], [0.5 * h, rout], [h - 0.012, rout - 0.008], [h, rout - 0.03], [h, rin + 0.03], [h - bevel, rin]])


def set_area(slug, region, value):
    import re
    path = CATALOG_V + slug + "/🔣️.json"
    text = open(path, encoding="utf-8").read()
    t0 = text.index('\n  "thermal"')
    t1 = text.index('\n  "structure"')
    seg = text[t0:t1]
    m = re.search(r'("id": "%s",.*?"areaM2": )([0-9.eE+-]+)' % re.escape(region), seg, re.S)
    assert m, region
    old = m.group(2)
    seg = seg[: m.start(2)] + repr(round(value, 3)).rstrip("0").rstrip(".") + seg[m.end(2) :]
    open(path, "w", encoding="utf-8").write(text[:t0] + seg + text[t1:])
    json.loads(open(path, encoding="utf-8").read())
    print("areaM2", slug, region, old, "->", round(value, 3))


class Fr:
    def __init__(self, dump, name):
        f = dump["frames"][name]
        self.name, self.o, self.u, self.v = name, f["o"], f["u"], f["v"]
        self.phi = math.atan2(self.u[1], self.u[0])

    def P(self, x, y, z=0.0):
        dx, dy = x - self.o[0], y - self.o[1]
        return [r6(dx * self.u[0] + dy * self.u[1]), r6(dx * self.v[0] + dy * self.v[1]), z]

    def D(self, dx, dy, dz=0.0):
        return [r6(dx * self.u[0] + dy * self.u[1]), r6(dx * self.v[0] + dy * self.v[1]), dz]

    def xl(self, x, y=0.0):
        return self.P(x, y)[0]

    def yl(self, y, x=0.0):
        return self.P(x, y)[1]

    def pts(self, ring):
        return [self.P(x, y)[:2] for x, y in ring]


def Bx(id, fr, mat, tone, size, at, r=None, role="body", rotz=0.0, repeat=None):
    p = fr.P(*at[:2], at[2])
    rot = Qz(rotz - fr.phi) if abs(rotz - fr.phi) > 1e-9 else None
    d = box(id, fr.name, mat, tone, size, p, r, role, rot)
    if repeat:
        d["repeat"] = repeat
    return d


def Ex(id, fr, mat, tone, plane, profile, a, b, holes=None, role="body"):
    if plane == "xy":
        pr = fr.pts(profile)
        hs = [fr.pts(h) for h in holes] if holes else None
        return ext(id, fr.name, mat, tone, "xy", pr, a, b, holes=hs, role=role)
    if plane == "zx":
        pr = [[z, fr.xl(x)] for z, x in profile]
        hs = [[[z, fr.xl(x)] for z, x in h] for h in holes] if holes else None
        lo, hi = sorted((fr.yl(a), fr.yl(b)))
        return ext(id, fr.name, mat, tone, "zx", pr, lo, hi, holes=hs, role=role)
    if plane == "yz":
        pr = [[fr.yl(y), z] for y, z in profile]
        hs = [[[fr.yl(y), z] for y, z in h] for h in holes] if holes else None
        lo, hi = sorted((fr.xl(a), fr.xl(b)))
        return ext(id, fr.name, mat, tone, "yz", pr, lo, hi, holes=hs, role=role)


def Lf(id, fr, mat, tone, rows, e=8, role="body"):
    out = []
    for row in rows:
        x, y0, y1, z0, z1 = row[:5]
        ya, yb = sorted((fr.yl(y0), fr.yl(y1)))
        out.append((fr.xl(x), ya, yb, z0, z1) + tuple(row[5:]))
    d = xloft(id, fr.name, mat, tone, out, e=e, role=role)
    return d


def Rv(id, fr, mat, tone, axis_w, origin_w, profile, role="body", repeat=None):
    d = revolve(id, fr.name, mat, tone, fr.D(*axis_w), fr.P(*origin_w), profile, role=role)
    if repeat:
        d["repeat"] = repeat
    return d


def Cp(id, fr, mat, tone, p0, p1, r0, r1=None, role="body"):
    return capsule(id, fr.name, mat, tone, fr.P(*p0), fr.P(*p1), r0, r1, role=role)
