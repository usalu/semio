import json, math, os
T = os.path.dirname(os.path.abspath(__file__))
FRAMES = json.load(open(os.path.join(T, "🗑️generated", "authoring-furniture", "frames.json")))
def r(x): return round(float(x), 5) + 0.0
def v3(a): return [r(x) for x in a]
def dot(a, b): return sum(x * y for x, y in zip(a, b))
def cross(a, b): return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def norm(a):
    l = math.sqrt(dot(a, a)); return [x / l for x in a]
X, Y, Z = [1, 0, 0], [0, 1, 0], [0, 0, 1]

def quat_from_cols(c0, c1, c2):
    m = [[c0[0], c1[0], c2[0]], [c0[1], c1[1], c2[1]], [c0[2], c1[2], c2[2]]]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2; q = [(m[2][1]-m[1][2])/s, (m[0][2]-m[2][0])/s, (m[1][0]-m[0][1])/s, s/4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2; q = [s/4, (m[0][1]+m[1][0])/s, (m[0][2]+m[2][0])/s, (m[2][1]-m[1][2])/s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2; q = [(m[0][1]+m[1][0])/s, s/4, (m[1][2]+m[2][1])/s, (m[0][2]-m[2][0])/s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2; q = [(m[0][2]+m[2][0])/s, (m[1][2]+m[2][1])/s, s/4, (m[1][0]-m[0][1])/s]
    n = math.sqrt(sum(x*x for x in q)); return [round(x / n, 6) + 0.0 for x in q]

def signed_axis(d, tol=1e-6):
    for k in range(3):
        if abs(abs(d[k]) - 1) < tol and all(abs(d[j]) < tol for j in range(3) if j != k): return k, (1 if d[k] > 0 else -1)
    return None

class Ctx:
    def __init__(self, slug):
        self.slug = slug; self.solids = []; self.decisions = []; self.fr = FRAMES[slug]
    def P(self, bone, p):
        f = self.fr[bone]["frame"]; d = [p[i] - f["o"][i] for i in range(3)]
        return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]
    def D(self, bone, d):
        f = self.fr[bone]["frame"]; return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]
    def add(self, s):
        assert all(s["id"] != o["id"] for o in self.solids), "duplicate " + s["id"]
        self.solids.append(s); return s
    def base(self, id, kind, bone, mat, role, tone, region, regions, maxD):
        s = {"id": id, "kind": kind, "bone": bone, "material": mat, "role": role, "tone": tone}
        if region: s["region"] = region
        if regions: s["regions"] = regions
        if maxD: s["maxDenominator"] = maxD
        return s
    def orient(self, bone, axes):
        d = [self.D(bone, a) for a in (axes or [X, Y, Z])]
        return d
    def sized(self, bone, size, axes):
        d = self.orient(bone, axes)
        sa = [signed_axis(x) for x in d]
        if all(sa) and len({k for k, _ in sa}) == 3:
            loc = [0, 0, 0]
            for i, (k, _) in enumerate(sa): loc[k] = size[i]
            return loc, None
        c0, c1 = norm(d[0]), norm(d[1]); c2 = cross(c0, c1)
        return list(size), quat_from_cols(c0, c1, c2)
    def box(self, id, bone, mat, role, tone, c, size, axes=None, corner=None, region=None, regions=None, maxD=None, repeat=None):
        s = self.base(id, "box", bone, mat, role, tone, region, regions, maxD)
        loc, q = self.sized(bone, size, axes)
        s["size"] = v3(loc)
        if corner: s["cornerRadius"] = r(corner)
        s["transform"] = {"translation": v3(self.P(bone, c))}
        if q: s["transform"]["rotation"] = q
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s)
    def ell(self, id, bone, mat, role, tone, c, radii, axes=None, region=None, regions=None, maxD=None, repeat=None):
        s = self.base(id, "ellipsoid", bone, mat, role, tone, region, regions, maxD)
        loc, q = self.sized(bone, radii, axes)
        s["radii"] = v3(loc)
        s["transform"] = {"translation": v3(self.P(bone, c))}
        if q: s["transform"]["rotation"] = q
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s)
    def cap(self, id, bone, mat, role, tone, p0, p1, r0, r1=None, region=None, regions=None, maxD=None, repeat=None):
        s = self.base(id, "capsule", bone, mat, role, tone, region, regions, maxD)
        s.update({"p0": v3(self.P(bone, p0)), "p1": v3(self.P(bone, p1)), "r0": r(r0), "r1": r(r0 if r1 is None else r1)})
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s)
    def rev(self, id, bone, mat, role, tone, profile, axis, origin, squash=None, sweep=None, region=None, regions=None, maxD=None, repeat=None):
        s = self.base(id, "revolve", bone, mat, role, tone, region, regions, maxD)
        s["profile"] = [[r(a), r(b)] for a, b in profile]
        ax = norm(self.D(bone, axis))
        s["axis"] = v3(ax)
        if squash:
            sc = [1, 1, 1]
            for k, f in enumerate([X, Y, Z]):
                pass
            f = self.fr[bone]["frame"]
            for k, name in enumerate(("u", "v", "w")):
                sa = signed_axis(f[name]); assert sa, "squash needs aligned frame"
                sc[k] = squash[sa[0]]
            s["origin"] = [0, 0, 0]
            s["transform"] = {"translation": v3(self.P(bone, origin)), "scale": v3(sc)}
        else:
            s["origin"] = v3(self.P(bone, origin))
        if sweep is not None: s["sweepRad"] = r(sweep)
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s)
    def ext(self, id, bone, mat, role, tone, pts, plane, lo, hi, holes=None, taper=None, region=None, regions=None, maxD=None, repeat=None):
        s = self.base(id, "extrude", bone, mat, role, tone, region, regions, maxD)
        ex = {"xy": 2, "xz": 1, "yz": 0}[plane]
        def lift(p, e): 
            q = [0, 0, 0]; idx = [i for i in range(3) if i != ex]
            q[idx[0]], q[idx[1]] = p[0], p[1]; q[ex] = e; return q
        f = self.fr[bone]["frame"]
        kdir = signed_axis(f["u"]) , signed_axis(f["v"]), signed_axis(f["w"])
        k = [i for i in range(3) if kdir[i] and kdir[i][0] == ex]; assert k, "extrusion axis not aligned with a local axis in " + id
        k = k[0]
        order = {2: (0, 1), 1: (2, 0), 0: (1, 2)}[k]; lplane = {2: "xy", 1: "zx", 0: "yz"}[k]
        def prof(ps): return [[r(self.P(bone, lift(p, 0))[order[0]]), r(self.P(bone, lift(p, 0))[order[1]])] for p in ps]
        a, b = self.P(bone, lift(pts[0], lo))[k], self.P(bone, lift(pts[0], hi))[k]
        s["profile"] = prof(pts)
        if holes: s["holes"] = [prof(h) for h in holes]
        s["plane"] = lplane; s["from"] = r(min(a, b)); s["to"] = r(max(a, b))
        if taper: s["taperTo"] = taper
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s)
    def loft(self, id, bone, mat, role, tone, secs, ax, ay, region=None, regions=None, maxD=None, repeat=None, caps=(True, True)):
        s = self.base(id, "loft", bone, mat, role, tone, region, regions, maxD)
        c0 = norm(self.D(bone, ax)); c1 = norm(self.D(bone, ay)); c2 = cross(c0, c1)
        q = quat_from_cols(c0, c1, c2)
        out = []
        for sec in secs:
            c, rx, ry = sec[0], sec[1], sec[2]
            d = {"at": v3(self.P(bone, c)), "rx": r(rx), "ry": r(ry)}
            if len(sec) > 3 and sec[3] is not None: d["exponent"] = sec[3]
            d["rotation"] = q
            out.append(d)
        s["sections"] = out; s["capStart"], s["capEnd"] = caps
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s)
    def rep(self, bone, rp):
        o = {"count": rp["count"]}
        if "translation" in rp: o["translation"] = v3(self.D(bone, rp["translation"]))
        if "rotationAxis" in rp: o["rotationAxis"] = v3(norm(self.D(bone, rp["rotationAxis"])))
        if "rotationRad" in rp: o["rotationRad"] = r(rp["rotationRad"])
        if "origin" in rp: o["origin"] = v3(self.P(bone, rp["origin"]))
        return o
    def ring(self, id, bone, mat, role, tone, centre, R, tube, axis=Y, n=24, region=None, regions=None, maxD=None, phase=0.0):
        h = math.pi / n; Rc = R / math.cos(h)
        ex, ey = ([1, 0, 0], [0, 0, 1]) if axis == Y else (([0, 1, 0], [0, 0, 1]) if axis == X else ([1, 0, 0], [0, 1, 0]))
        def at(a): return [centre[i] + Rc * (math.cos(a) * ex[i] + math.sin(a) * ey[i]) for i in range(3)]
        return self.cap(id, bone, mat, role, tone, at(phase - h), at(phase + h), tube, region=region, regions=regions, maxD=maxD, repeat={"count": n, "rotationAxis": axis, "rotationRad": 2 * h, "origin": centre})
    def lbox(self, id, bone, mat, role, tone, u, v, w, corner=None, region=None, regions=None, maxD=None, repeat=None):
        s = self.base(id, "box", bone, mat, role, tone, region, regions, maxD)
        s["size"] = v3([u[1] - u[0], v[1] - v[0], w[1] - w[0]])
        if corner: s["cornerRadius"] = r(corner)
        s["transform"] = {"translation": v3([(u[0] + u[1]) / 2, (v[0] + v[1]) / 2, (w[0] + w[1]) / 2])}
        if repeat: s["repeat"] = {k: v3(x) if isinstance(x, list) else x for k, x in repeat.items()}
        return self.add(s)
    def lcap(self, id, bone, mat, role, tone, p0, p1, r0, r1=None, region=None, regions=None, maxD=None, repeat=None):
        s = self.base(id, "capsule", bone, mat, role, tone, region, regions, maxD)
        s.update({"p0": v3(p0), "p1": v3(p1), "r0": r(r0), "r1": r(r0 if r1 is None else r1)})
        if repeat: s["repeat"] = {k: v3(x) if isinstance(x, list) else x for k, x in repeat.items()}
        return self.add(s)
    def lrev(self, id, bone, mat, role, tone, profile, axis, origin, region=None, regions=None, maxD=None, repeat=None):
        s = self.base(id, "revolve", bone, mat, role, tone, region, regions, maxD)
        s["profile"] = [[r(a), r(b)] for a, b in profile]; s["axis"] = v3(axis); s["origin"] = v3(origin)
        if repeat: s["repeat"] = {k: v3(x) if isinstance(x, list) else x for k, x in repeat.items()}
        return self.add(s)
    def bw(self, id, bone, mat, role, tone, x, y, z, **k):
        return self.box(id, bone, mat, role, tone, [(x[0]+x[1])/2, (y[0]+y[1])/2, (z[0]+z[1])/2], [x[1]-x[0], y[1]-y[0], z[1]-z[0]], **k)
    def dec(self, en, de): self.decisions.append({"en": en, "de": de})
    def write(self):
        out = {"volume": {"status": "authored", "solids": self.solids, "decisions": self.decisions}}
        p = os.path.join(T, "volumes", "furniture", self.slug + ".json")
        with open(p, "w") as f: json.dump(out, f, ensure_ascii=False, indent=1)
        print(self.slug, len(self.solids), "solids ->", p)

def circle(cx, cy, rad, n=12):
    pts = [[cx + rad * math.cos(2*math.pi*i/n), cy + rad * math.sin(2*math.pi*i/n)] for i in range(n)]
    return pts + [pts[0]]
def qbez(p0, c, p1, n=4):
    return [[(1-t)**2*p0[0]+2*(1-t)*t*c[0]+t*t*p1[0], (1-t)**2*p0[1]+2*(1-t)*t*c[1]+t*t*p1[1]] for t in [i/n for i in range(n+1)]]
def lerp(a, b, t): return [a[i] + (b[i]-a[i])*t for i in range(len(a))]
