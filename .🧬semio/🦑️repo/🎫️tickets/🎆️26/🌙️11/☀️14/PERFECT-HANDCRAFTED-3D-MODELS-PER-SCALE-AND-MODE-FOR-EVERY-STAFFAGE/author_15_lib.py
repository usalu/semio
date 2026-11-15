import json, math, os
T = os.path.dirname(os.path.abspath(__file__))
FRAMES = json.load(open(os.path.join(T, "🗑️generated", "author-15", "frames.json")))
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
    def __init__(self, slug, scale):
        self.slug = slug; self.scale = scale; self.fr = FRAMES[slug]
        self.lists = {"body": [], "thermal": [], "structure": []}; self.cur = "body"; self.decisions = []
    def to(self, name): self.cur = name; return self
    def P(self, bone, p):
        f = self.fr[bone]["frame"]; d = [p[i] - f["o"][i] for i in range(3)]
        return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]
    def D(self, bone, d):
        f = self.fr[bone]["frame"]; return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]
    def add(self, s, extra):
        s.update(extra)
        L = self.lists[self.cur]
        assert all(s["id"] != o["id"] for o in L), "duplicate " + s["id"]
        L.append(s); return s
    def ref(self, id, of, **extra):
        s = {"id": id, "of": of}; return self.add(s, extra)
    def base(self, id, kind, bone, mat, role, tone):
        return {"id": id, "kind": kind, "bone": bone, "material": mat, "role": role, "tone": tone}
    def sized(self, bone, size, axes):
        d = [self.D(bone, a) for a in (axes or [X, Y, Z])]
        sa = [signed_axis(x) for x in d]
        if all(sa) and len({k for k, _ in sa}) == 3:
            loc = [0, 0, 0]
            for i, (k, _) in enumerate(sa): loc[k] = size[i]
            return loc, None
        c0, c1 = norm(d[0]), norm(d[1]); c2 = cross(c0, c1)
        return list(size), quat_from_cols(c0, c1, c2)
    def rep(self, bone, rp):
        o = {"count": rp["count"]}
        if "translation" in rp: o["translation"] = v3(self.D(bone, rp["translation"]))
        if "rotationAxis" in rp: o["rotationAxis"] = v3(norm(self.D(bone, rp["rotationAxis"])))
        if "rotationRad" in rp: o["rotationRad"] = r(rp["rotationRad"])
        if "origin" in rp: o["origin"] = v3(self.P(bone, rp["origin"]))
        return o
    def box(self, id, bone, mat, role, tone, c, size, axes=None, corner=None, repeat=None, **ex):
        s = self.base(id, "box", bone, mat, role, tone)
        loc, q = self.sized(bone, size, axes)
        s["size"] = v3(loc)
        if corner: s["cornerRadius"] = r(corner)
        s["transform"] = {"translation": v3(self.P(bone, c))}
        if q: s["transform"]["rotation"] = q
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s, ex)
    def bw(self, id, bone, mat, role, tone, x, y, z, **k):
        return self.box(id, bone, mat, role, tone, [(x[0]+x[1])/2, (y[0]+y[1])/2, (z[0]+z[1])/2], [x[1]-x[0], y[1]-y[0], z[1]-z[0]], **k)
    def ell(self, id, bone, mat, role, tone, c, radii, axes=None, repeat=None, **ex):
        s = self.base(id, "ellipsoid", bone, mat, role, tone)
        loc, q = self.sized(bone, radii, axes)
        s["radii"] = v3(loc)
        s["transform"] = {"translation": v3(self.P(bone, c))}
        if q: s["transform"]["rotation"] = q
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s, ex)
    def cap(self, id, bone, mat, role, tone, p0, p1, r0, r1=None, repeat=None, **ex):
        s = self.base(id, "capsule", bone, mat, role, tone)
        s.update({"p0": v3(self.P(bone, p0)), "p1": v3(self.P(bone, p1)), "r0": r(r0), "r1": r(r0 if r1 is None else r1)})
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s, ex)
    def rev(self, id, bone, mat, role, tone, profile, axis, origin, sweep=None, repeat=None, **ex):
        s = self.base(id, "revolve", bone, mat, role, tone)
        s["profile"] = [[r(a), r(b)] for a, b in profile]
        s["axis"] = v3(norm(self.D(bone, axis))); s["origin"] = v3(self.P(bone, origin))
        if sweep is not None: s["sweepRad"] = r(sweep)
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s, ex)
    def ext(self, id, bone, mat, role, tone, pts, plane, lo, hi, holes=None, taper=None, repeat=None, **exx):
        s = self.base(id, "extrude", bone, mat, role, tone)
        ex = {"xy": 2, "xz": 1, "yz": 0}[plane]
        def lift(p, e):
            q = [0, 0, 0]; idx = [i for i in range(3) if i != ex]
            q[idx[0]], q[idx[1]] = p[0], p[1]; q[ex] = e; return q
        f = self.fr[bone]["frame"]
        kdir = signed_axis(f["u"]), signed_axis(f["v"]), signed_axis(f["w"])
        if plane == "xy": k = 2
        else:
            k = [i for i in range(3) if kdir[i] and kdir[i][0] == ex]; assert k, "extrusion axis not aligned in " + id
            k = k[0]
        order = {2: (0, 1), 1: (2, 0), 0: (1, 2)}[k]; lplane = {2: "xy", 1: "zx", 0: "yz"}[k]
        def prof(ps): return [[r(self.P(bone, lift(p, 0))[order[0]]), r(self.P(bone, lift(p, 0))[order[1]])] for p in ps]
        a, b = self.P(bone, lift(pts[0], lo))[k], self.P(bone, lift(pts[0], hi))[k]
        s["profile"] = prof(pts)
        if holes: s["holes"] = [prof(h) for h in holes]
        s["plane"] = lplane; s["from"] = r(min(a, b)); s["to"] = r(max(a, b))
        if taper: s["taperTo"] = taper
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s, exx)
    def loft(self, id, bone, mat, role, tone, secs, ax=None, ay=None, caps=(True, True), repeat=None, **ex):
        s = self.base(id, "loft", bone, mat, role, tone)
        q = None
        if ax is not None:
            c0 = norm(self.D(bone, ax)); c1 = norm(self.D(bone, ay)); c2 = cross(c0, c1)
            q = quat_from_cols(c0, c1, c2)
        out = []
        for sec in secs:
            c, rx, ry = sec[0], sec[1], sec[2]
            d = {"at": v3(self.P(bone, c)), "rx": r(rx), "ry": r(ry)}
            if len(sec) > 3 and sec[3] is not None: d["exponent"] = sec[3]
            if len(sec) > 4 and sec[4] is not None:
                c0 = norm(self.D(bone, sec[4][0])); c1 = norm(self.D(bone, sec[4][1])); d["rotation"] = quat_from_cols(c0, c1, cross(c0, c1))
            elif q is not None: d["rotation"] = q
            out.append(d)
        s["sections"] = out; s["capStart"], s["capEnd"] = caps
        if repeat: s["repeat"] = self.rep(bone, repeat)
        return self.add(s, ex)
    def torus(self, id, bone, mat, role, tone, centre, R, rt, n=12, axis=Y, repeat=None, **ex):
        pts = [[0.0, 0.0]] * 0
        rc = rt / math.cos(math.pi / n)
        pr = [[rc * math.sin(2 * math.pi * (i + 0.5) / n), R + rc * math.cos(2 * math.pi * (i + 0.5) / n)] for i in range(n)]
        pr.append(pr[0])
        return self.rev(id, bone, mat, role, tone, pr, axis, centre, repeat=repeat, **ex)
    def dec(self, en, de): self.decisions.append({"en": en, "de": de})
    def out(self):
        return {"status": "authored", "denominator": self.scale, "body": self.lists["body"], "thermal": self.lists["thermal"], "structure": self.lists["structure"], "decisions": self.decisions}

def unavailable(n): return {"status": "unavailable", "denominator": n}
def nudge(vol):
    f = 0.00015 * vol["denominator"]
    def up(v, k): return round(v * k, 5) if 0.97 * f <= v <= 1.0001 * f else v
    for lst in ("body", "thermal", "structure"):
        for s in vol[lst]:
            k = s.get("kind")
            if k == "box": s["size"] = [up(v, 1.004) for v in s["size"]]
            elif k == "capsule":
                if 0.97 * f <= 2 * s["r0"] <= 1.0001 * f: s["r0"] = round(s["r0"] * 1.03, 5)
                if 0.97 * f <= 2 * s["r1"] <= 1.0001 * f: s["r1"] = round(s["r1"] * 1.03, 5)
            elif k == "loft":
                for sec in s["sections"]:
                    sec["rx"] = round(sec["rx"] * 1.03, 5) if 0.97 * f <= 2 * sec["rx"] <= 1.0001 * f else sec["rx"]
                    sec["ry"] = round(sec["ry"] * 1.03, 5) if 0.97 * f <= 2 * sec["ry"] <= 1.0001 * f else sec["ry"]
    return vol

def save(slug, scales):
    scales = {k: (nudge(v) if v["status"] == "authored" else v) for k, v in scales.items()}
    p = os.path.join(T, "volumes", "furniture", slug + ".json")
    with open(p, "w") as f: json.dump({"volumes": {str(k): v for k, v in scales.items()}}, f, ensure_ascii=False, indent=1)
    print(slug, {k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in scales.items() if v["status"] == "authored"})
def circle(cx, cy, rad, n=12):
    pts = [[cx + rad * math.cos(2*math.pi*i/n), cy + rad * math.sin(2*math.pi*i/n)] for i in range(n)]
    return pts
def lerp(a, b, t): return [a[i] + (b[i]-a[i])*t for i in range(len(a))]
