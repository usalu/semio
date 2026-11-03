import json, math, os, sys, subprocess
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import jsonfmt

T = os.path.dirname(HERE)
GEN = os.path.join(T, "🗑️generated", "A2")
CAT = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🐕️animals"
REPO = r"C:\git\semio"


def r5(v):
    return round(v + 0.0, 5)


class Asset:
    def __init__(s, aid):
        s.aid = aid
        s.path = os.path.join(CAT, aid, "🔣️.json")
        s.doc = jsonfmt.read(s.path)
        s.a = s.doc["asset"]
        bones = {b["id"]: b for b in s.a["rig"]["bones"]}
        w = {}

        def solve(i):
            if i in w:
                return w[i]
            b = bones[i]
            px, py, pa = (0.0, 0.0, 0.0) if b.get("parent") is None else solve(b["parent"])
            c, sn = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            w[i] = (px + c * b["x"] - sn * b["y"], py + sn * b["x"] + c * b["y"], pa + b["angleDeg"])
            return w[i]

        s.rest = {}
        for i, b in bones.items():
            x, y, a = solve(i)
            s.rest[i] = (x, y, x + b["lengthM"] * math.cos(math.radians(a)), y + b["lengthM"] * math.sin(math.radians(a)))
        s.evs = {}
        own = set()
        for src in [q["bones"] for q in s.a["poses"]] + [k.get("bones", {}) for c in s.a["clips"] for k in c["keys"]]:
            for b, v in src.items():
                if v.get("abductionDeg") or v.get("twistDeg"):
                    own.add(b)
        s.spread = set()
        for b in bones:
            c = b
            while c is not None:
                if c in own:
                    s.spread.add(b)
                    break
                c = bones[c].get("parent")
        s.dp = s.a["presentation"]["defaultPose"]
        s.d = s.bones_at(s.dp)

    def evalfile(s, scale=50):
        return os.path.join(GEN, "%s-%d.json" % (s.aid, scale))

    def bones_at(s, label, scale=50):
        ev = json.load(open(s.evalfile(scale), encoding="utf-8"))
        for q in ev:
            if q["label"] == label:
                return {k: tuple(v) for k, v in q["bones"].items()}
        raise KeyError(label)

    def like(s, scale, sid):
        for sc in (str(scale), "50", "100"):
            rep = s.a["representations"].get(sc, {})
            for sh in rep.get("geometry", {}).get("elevation", []):
                if sh["id"] == sid:
                    return sh
        raise KeyError("no elevation shape %s" % sid)


def smooth(pts, closed=True, n=5):
    """Centripetal Catmull-Rom through the control points I choose (Barry-Goldman evaluation)."""
    P = [tuple(p) for p in pts]
    m = len(P)
    out = []

    def seg(p0, p1, p2, p3):
        def tj(ti, a, b):
            return ti + max(math.hypot(b[0] - a[0], b[1] - a[1]), 1e-9) ** 0.5
        t0 = 0.0
        t1 = tj(t0, p0, p1)
        t2 = tj(t1, p1, p2)
        t3 = tj(t2, p2, p3)
        for k in range(n):
            t = t1 + (t2 - t1) * k / n
            def lerp(a, b, ta, tb):
                w = (t - ta) / (tb - ta)
                return (a[0] * (1 - w) + b[0] * w, a[1] * (1 - w) + b[1] * w)
            a1 = lerp(p0, p1, t0, t1)
            a2 = lerp(p1, p2, t1, t2)
            a3 = lerp(p2, p3, t2, t3)
            b1 = lerp(a1, a2, t0, t2)
            b2 = lerp(a2, a3, t1, t3)
            c = lerp(b1, b2, t1, t2)
            out.append([c[0], c[1]])

    rng = range(m) if closed else range(m - 1)
    for i in rng:
        p1 = P[i]
        p2 = P[(i + 1) % m]
        p0 = P[(i - 1) % m] if closed or i > 0 else (2 * p1[0] - p2[0], 2 * p1[1] - p2[1])
        p3 = P[(i + 2) % m] if closed or i + 2 < m else (2 * p2[0] - p1[0], 2 * p2[1] - p1[1])
        seg(p0, p1, p2, p3)
    if not closed:
        out.append(list(P[-1]))
    return out


def mirror_x(pts):
    return [[-x, y] for x, y in pts][::-1]


class View:
    """Collects shapes of one view (front or plan) at one scale, authored in default-pose world metres."""

    def __init__(s, asset, scale, vp):
        s.A, s.scale, s.vp, s.shapes = asset, scale, vp, []

    def _meta(s, id, bone, z, like, role, filled, tone, region, regions, material, hatch, penMm):
        m = {}
        if like:
            lk = s.A.like(s.scale, like)
            for k in ("tone", "region", "regions", "material", "hatch", "penMm"):
                if k in lk:
                    m[k] = lk[k]
        if penMm is None and "penMm" in m:
            penMm = round(m["penMm"], 3)
        if tone is not None:
            m["tone"] = tone
        if region:
            m["region"] = region
            m.pop("regions", None)
        if regions:
            m["regions"] = regions
            m.pop("region", None)
        if material:
            m["material"] = material
        if hatch:
            m["hatch"] = hatch
        out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": m.get("tone", 0.5)}
        for k in ("region", "regions", "material", "hatch"):
            if k in m:
                out[k] = m[k]
        if penMm:
            out["penMm"] = penMm
        return out

    def _dy(s, bone):
        return s.A.d[bone][1] - s.A.rest[bone][1]

    def _dx(s, bone):
        return s.A.d[bone][0] - s.A.rest[bone][0]

    def _inv_y(s, bone, Y):
        oxd, oyd, txd, tyd = s.A.d[bone]
        _, y0r, _, ty_r = s.A.rest[bone]
        span = ty_r - y0r
        dy = oyd - y0r
        dtip = tyd - ty_r
        dspan = tyd - oyd
        if abs(dspan) < 1e-6:
            return Y - dy
        t = (Y - oyd) / dspan
        if t < 0:
            return Y - dy
        if t > 1:
            return Y - dtip
        return y0r + t * span

    def add(s, id, bone, geom, z, like=None, role="body", filled=True, tone=None, region=None, regions=None, material=None, hatch=None, penMm=None):
        sh = s._meta(id, bone, z, like, role, filled, tone, region, regions, material, hatch, penMm)
        sh.update(geom)
        s.shapes.append(sh)
        return sh

    def _map(s, bone, x, y, follow=False):
        if s.vp == "front":
            return (x, s._inv_y(bone, y) if follow else y - s._dy(bone))
        return (x - s._dx(bone), y)

    def ell(s, id, bone, cx, cy, rx, ry, rot=0, **kw):
        x, y = s._map(bone, cx, cy)
        g = {"kind": "ellipse", "cx": r5(x), "cy": r5(y), "rx": r5(rx), "ry": r5(ry)}
        if rot:
            g["rotationDeg"] = rot
        return s.add(id, bone, g, **kw)

    def poly(s, id, bone, pts, closed=True, follow=False, **kw):
        f = follow and s.vp == "front"
        q = [list(map(r5, s._map(bone, x, y, f))) for x, y in pts]
        g = {"kind": "poly", "points": q, "closed": closed}
        if f:
            g["followTip"] = True
        return s.add(id, bone, g, **kw)

    def blob(s, id, bone, ctrl, follow=False, n=5, **kw):
        return s.poly(id, bone, smooth(ctrl, True, n), True, follow, **kw)

    def rect(s, id, bone, x, y, w, h, r=0, follow=False, **kw):
        f = follow and s.vp == "front"
        x0, y0 = s._map(bone, x, y, f)
        _, y1 = s._map(bone, x, y + h, f)
        g = {"kind": "rect", "x": r5(x0), "y": r5(y0), "w": r5(w), "h": r5(y1 - y0 if s.vp == "front" else h)}
        if r:
            g["r"] = r
        if f:
            g["followTip"] = True
        return s.add(id, bone, g, **kw)

    def cap(s, id, bone, x0, y0, x1, y1, r0, r1, follow=False, **kw):
        if s.vp == "plan":
            xa = x0 - s._dx(bone)
            xb = x1 - (s.A.d[bone][2] - s.A.rest[bone][2] if follow else s._dx(bone))
            ya, yb = y0, y1
        else:
            xa, xb = x0, x1
            ya, yb = y0 - s._dy(bone), y1 - s._dy(bone)
            follow = False
        g = {"kind": "capsule", "x0": r5(xa), "y0": r5(ya), "x1": r5(xb), "y1": r5(yb), "r0": r5(r0), "r1": r5(r1)}
        if follow:
            g["followTip"] = True
        return s.add(id, bone, g, **kw)


def commit(A, scale, front, plan, note_en, note_de, fields=("silhouette",)):
    A.doc = jsonfmt.read(A.path)
    A.a = A.doc["asset"]
    rep = A.a["representations"][str(scale)]
    for name, lst in (("front", front), ("plan", plan)):
        ids = [x["id"] for x in lst]
        assert len(ids) == len(set(ids)), "dup %s ids" % name
    for sh in rep["geometry"]["elevation"]:
        if sh.get("solid", {}).get("kind") == "extruded" and sh["bone"] not in A.spread and (scale, sh["id"]) not in A.evs:
            del sh["solid"]
        hit = A.evs.get((scale, sh["id"]))
        if hit is not None and hit[0] == "rev" and hit[1] == sh["bone"]:
            for k in [k for k in sh if k in ("x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "rotationDeg", "points", "closed", "x0", "y0", "x1", "y1", "r0", "r1", "followTip", "d", "solid")]:
                del sh[k]
            sh.update({"kind": "poly", "points": hit[2], "closed": True, "solid": hit[3]})
        elif hit is not None and len(hit) == 3 and hit[0] == sh["bone"]:
            sh["solid"] = {"kind": "extruded", "z0": hit[1], "z1": hit[2]}
    rep["geometry"] = {"elevation": rep["geometry"]["elevation"], "front": front, "plan": plan}
    import extra
    note_en, note_de = extra.texts(A.aid, scale, note_en, note_de)
    for f in fields:
        d = rep["decisions"][f]
        for lang, note in (("en", note_en), ("de", note_de)):
            base = d[lang].split(" Front view")[0].split(" Vorderansicht")[0]
            d[lang] = base + " " + note
    A.a.get("presentation", {}).pop("viewpoints", None)
    jsonfmt.write(A.path, A.doc)


def run_eval(aid, scale, samples=5):
    out = os.path.join(GEN, "%s-%d.json" % (aid, scale))
    path = os.path.join(CAT, aid, "🔣️.json")
    r = subprocess.run(["bun", os.path.join(HERE, "eval.ts"), path, str(scale), out, str(samples)], capture_output=True, text=True, encoding="utf-8", errors="replace", cwd=REPO)
    print((r.stdout or "").strip()[-200:], (r.stderr or "").strip()[-800:])
    return out


def profile(pts):
    """Closed symmetric outline (about lateral 0) from (along, half-width) pairs; zero half-width at an end gives a single point."""
    up = [(x, hw) for x, hw in pts]
    lo = [(x, -hw) for x, hw in reversed(pts) if hw != 0]
    return up + lo


def _limb_plan(self, id, bone, lat, r0, r1, pext=0.0, lat1=None, **kw):
    ox, _, tx, _ = self.A.d[bone]
    return self.cap(id, bone, ox, lat, tx + pext, lat if lat1 is None else lat1, r0, r1, follow=True, **kw)


def _limb_front(self, id, bone, lat, w0, w1, over=0.0, under=0.0, **kw):
    _, oy, _, ty = self.A.d[bone]
    sgn = 1 if oy >= ty else -1
    pts = [(lat - w0 / 2, oy + sgn * over), (lat + w0 / 2, oy + sgn * over), (lat + w1 / 2, ty - sgn * under), (lat - w1 / 2, ty - sgn * under)]
    return self.poly(id, bone, pts, True, True, **kw)


View.limb_plan = _limb_plan
View.limb_front = _limb_front


def _local(self, bone, x, y):
    ox, oy, tx, ty = self.A.d[bone]
    a = math.atan2(ty - oy, tx - ox)
    c, s = math.cos(-a), math.sin(-a)
    dx, dy = x - ox, y - oy
    return (c * dx - s * dy, s * dx + c * dy)


def _bar(self, bone, a0, a1, half):
    """Default-pose world quad along the bone axis from a0 to a1 (m from origin), half thickness `half`."""
    ox, oy, tx, ty = self.A.d[bone]
    L = math.hypot(tx - ox, ty - oy)
    ux, uy = (tx - ox) / L, (ty - oy) / L
    nx, ny = -uy, ux
    pts = []
    for a, h in ((a0, half), (a1, half), (a1, -half), (a0, -half)):
        pts.append((ox + ux * a + nx * h, oy + uy * a + ny * h))
    q = BarPts(pts)
    q.spec = (bone, a0, a1, half)
    return q


class BarPts(list):
    spec = None


def _shaped(self, bone, like, world_pts):
    """The profile of an extruded solid in the bone frame: the elevation shape `like` of the same bone when it is a polygon or an ellipse (one shaped 3D definition for all three views), else the bar given in world metres."""
    if like:
        try:
            lk = self.A.like(self.scale, like)
        except KeyError:
            lk = None
        if lk is not None and lk["bone"] == bone:
            if lk["kind"] == "poly" and lk.get("closed", True):
                return [list(pt) for pt in lk["points"]]
            if lk["kind"] == "path":
                return [[r5(x), r5(y)] for x, y in path_points(lk["d"])]
            if lk["kind"] == "ellipse":
                a = math.radians(lk.get("rotationDeg", 0))
                pts = []
                for k in range(24):
                    t = 2 * math.pi * k / 24
                    ex, ey = lk["rx"] * math.cos(t), lk["ry"] * math.sin(t)
                    pts.append([r5(lk["cx"] + ex * math.cos(a) - ey * math.sin(a)), r5(lk["cy"] + ex * math.sin(a) + ey * math.cos(a))])
                return pts
    return [[r5(v) for v in self._local(bone, x, y)] for x, y in world_pts]


def _ext(self, id, bone, world_pts, z0, z1, **kw):
    """Extruded solid with a shaped profile (front: lateral z0..z1 by the height of the profile; plan: forward range by z0..z1). A part whose bone is tilted at rest (neck) stays a tapered followTip trapezoid until revolved solids about the bone axis exist."""
    kw.pop("keep_box", None)
    kw.pop("taper", None)
    kw.pop("tippad", None)
    if kw.pop("spread", False) and self.vp == "front":
        pad = world_pts.spec[3] if getattr(world_pts, "spec", None) else 0.0
        _, oy, _, ty = self.A.d[bone]
        lo, hi = min(oy, ty) - pad, max(oy, ty) + pad
        return self.poly(id, bone, [(z0, lo), (z1, lo), (z1, hi), (z0, hi)], True, True, **kw)
    kw.pop("spread", None)
    rx0, ry0, rx1, ry1 = self.A.rest[bone]
    blen = math.hypot(rx1 - rx0, ry1 - ry0)
    spec = getattr(world_pts, "spec", None)
    if spec is not None and self.vp == "front" and abs(ry1 - ry0) >= 0.45 * blen:
        _, a0, a1, half = spec
        ox, oy, tx, ty = self.A.d[bone]
        L = math.hypot(tx - ox, ty - oy)
        uy = (ty - oy) / L
        wmax, zc = (z1 - z0) / 2, (z0 + z1) / 2
        ya, yb = oy + uy * a0, oy + uy * a1
        return self.poly(id, bone, [(zc - wmax, ya), (zc + wmax, ya), (zc + wmax * 0.7, yb), (zc - wmax * 0.7, yb)], True, True, **kw)
    q = self._shaped(bone, kw.get("like"), world_pts)
    if kw.get("like") and bone in self.A.spread:
        self.A.evs[(self.scale, kw["like"])] = (bone, r5(z0), r5(z1))
    return self.add(id, bone, {"kind": "poly", "points": q, "closed": True, "solid": {"kind": "extruded", "z0": r5(z0), "z1": r5(z1)}}, **kw)



View._local = _local
View.bar = _bar
View.ext = _ext
View._shaped = _shaped


def path_points(d, steps=10):
    """Flatten an SVG path of M, L, C and Z commands (absolute) to a polygon."""
    t = d.replace(",", " ").split()
    pts, i, cur, cmd = [], 0, (0.0, 0.0), None
    while i < len(t):
        if t[i] in "MLCQZ":
            cmd = t[i]
            i += 1
            if cmd == "Z":
                continue
        if cmd in "ML":
            cur = (float(t[i]), float(t[i + 1]))
            pts.append(cur)
            i += 2
        elif cmd == "C":
            c1, c2, e = [(float(t[i + 2 * k]), float(t[i + 2 * k + 1])) for k in range(3)]
            for k in range(1, steps + 1):
                u = k / steps
                a, b, c, dd = (1 - u) ** 3, 3 * (1 - u) ** 2 * u, 3 * (1 - u) * u * u, u ** 3
                pts.append((a * cur[0] + b * c1[0] + c * c2[0] + dd * e[0], a * cur[1] + b * c1[1] + c * c2[1] + dd * e[1]))
            cur = e
            i += 6
        elif cmd == "Q":
            c1, e = [(float(t[i + 2 * k]), float(t[i + 2 * k + 1])) for k in range(2)]
            for k in range(1, steps + 1):
                u = k / steps
                pts.append(((1 - u) ** 2 * cur[0] + 2 * (1 - u) * u * c1[0] + u * u * e[0], (1 - u) ** 2 * cur[1] + 2 * (1 - u) * u * c1[1] + u * u * e[1]))
            cur = e
            i += 4
        else:
            raise ValueError(cmd)
    return pts


def _radial(self, bone, like, stations=14):
    """Half profile (along the bone axis, radius) of a part: the largest distance from the bone axis of the side-view outline `like` at each station, in the bone frame."""
    lk = self.A.like(self.scale, like)
    assert lk["bone"] == bone, (like, bone)
    if lk["kind"] == "poly":
        pts = [tuple(q) for q in lk["points"]]
    elif lk["kind"] == "path":
        pts = path_points(lk["d"])
    elif lk["kind"] == "ellipse":
        a = math.radians(lk.get("rotationDeg", 0))
        pts = []
        for k in range(48):
            t = 2 * math.pi * k / 48
            ex, ey = lk["rx"] * math.cos(t), lk["ry"] * math.sin(t)
            pts.append((lk["cx"] + ex * math.cos(a) - ey * math.sin(a), lk["cy"] + ex * math.sin(a) + ey * math.cos(a)))
    else:
        raise ValueError(lk["kind"])
    x0, x1 = min(x for x, _ in pts), max(x for x, _ in pts)
    prof = []
    self._rows = []
    for i in range(stations):
        x = x0 + (x1 - x0) * i / (stations - 1)
        ys = []
        for (ax, ay), (bx, by) in zip(pts, pts[1:] + pts[:1]):
            if (ax - x) * (bx - x) <= 0 and ax != bx:
                ys.append(ay + (by - ay) * (x - ax) / (bx - ax))
        r = max((abs(y) for y in ys), default=0.0)
        if ys:
            self._rows.append((x, min(ys), max(ys)))
        prof.append((r5(x), r5(max(r, 0.00011 * self.scale))))
    return [[prof[0][0], 0.0]] + [list(q) for q in prof] + [[prof[-1][0], 0.0]]


def _rev(self, id, bone, like, lat=None, **kw):
    """Revolved solid about the bone's own axis with the symmetric envelope of the side-view part `like`: one 3D definition (circular cross-sections) for elevation, front and plan; the elevation shape is rewritten by commit."""
    prof = self._radial(bone, like)
    rows = self._rows[2:-2] or self._rows
    off = max(abs(a + b) / 2 / max((b - a) / 2, 1e-3) for _, a, b in rows)
    rmax = max(r for _, r in prof[1:-1])
    if off > 0.6 or (lat is not None and not 0.8 <= lat / rmax <= 1.25):
        half = r5(lat if lat is not None else max((b - a) / 2 for _, a, b in self._rows))
        if bone in self.A.spread:
            self.A.evs[(self.scale, like)] = (bone, -half, half)
        kw.setdefault("like", like)
        return self.add(id, bone, {"kind": "poly", "points": self._shaped(bone, like, None), "closed": True, "solid": {"kind": "extruded", "z0": -half, "z1": half}}, **kw)
    solid = {"kind": "revolved", "axisBone": bone, "axisX": r5(self.A.rest[bone][0]), "axisZ": 0.0, "mirror": True}
    self.A.evs[(self.scale, like)] = ("rev", bone, prof, solid)
    kw.setdefault("like", like)
    return self.add(id, bone, {"kind": "poly", "points": prof, "closed": True, "solid": solid}, **kw)


View._radial = _radial
View.rev = _rev


def limbs(F, P, rows):
    """Both sides of every limb row: followTip trapezoids in front, followTip capsules in plan (near = right = -lateral)."""
    for r in rows:
        for s, side, sd in (("f", "far", 1), ("n", "near", -1)):
            fmt = {"s": s, "side": side}
            id = r["id"].format(**fmt)
            bone = r["bone"].format(**fmt)
            like = r.get("like", r["id"]).format(**fmt)
            if F is not None and "fw" in r:
                F.limb_front(id, bone, sd * r["flat"], r["fw"][0], r["fw"][1], over=r.get("over", 0.0), under=r.get("under", 0.0), like=like, z=r["fz"])
            if P is not None and "pr" in r:
                P.limb_plan(id, bone, sd * r["plat"], r["pr"][0], r["pr"][1], pext=r.get("pext", 0.0), like=like, z=r["pz"])


def limbs_line(F, P, rows):
    """Stroke legs for the smallest scales: an open followTip polyline in front, a hairline followTip capsule in plan."""
    for r in rows:
        for s, side, sd in (("f", "far", 1), ("n", "near", -1)):
            fmt = {"s": s, "side": side}
            id = r["id"].format(**fmt)
            bone = r["bone"].format(**fmt)
            like = r.get("like", r["id"]).format(**fmt)
            if F is not None:
                _, oy, _, ty = F.A.d[bone]
                sgn = 1 if oy >= ty else -1
                F.poly(id, bone, [(sd * r["flat"], oy + sgn * r.get("over", 0.0)), (sd * r["flat"], ty - sgn * r.get("under", 0.0))], closed=False, follow=True, filled=False, like=like, z=r["fz"])
            if P is not None:
                ox, _, tx, _ = P.A.d[bone]
                P.cap(id, bone, ox, sd * r["plat"], tx, sd * r["plat"], 0.001, 0.001, follow=True, filled=False, like=like, z=r["pz"])
