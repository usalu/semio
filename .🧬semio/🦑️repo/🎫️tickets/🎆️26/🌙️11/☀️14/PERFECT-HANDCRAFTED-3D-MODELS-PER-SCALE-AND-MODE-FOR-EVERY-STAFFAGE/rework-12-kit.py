"""🌳 Chisel of author 12 for the eight trees: turns hand-stated botanical axes (stations with radii, branch angles) and hand-stated foliage masses into closed polyhedra bound to a 3D rig.

World frame of the statements: x ahead, y up, z left, metres. Azimuth `az` in plan from +x towards +z, elevation `el` above the horizon, degrees.
Nothing here chooses a number of a tree: every station, angle, radius, relief count and merge is stated in `rework-12-<slug>-model.py`.
"""
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🌳️vegetation")
OUT = os.path.join(T, "🗑️generated", "rework-12")
SCALES = (50, 100, 200, 500)
MIN_DIA = {50: 0.019, 100: 0.037, 200: 0.085, 500: 0.24}
RING_STEP = {50: 0.22, 100: 0.38, 200: 0.7, 500: 1.3}
ROUND = {50: 4, 100: 4, 200: 3, 500: 3}


def add(a, b): return (a[0] + b[0], a[1] + b[1], a[2] + b[2])
def sub(a, b): return (a[0] - b[0], a[1] - b[1], a[2] - b[2])
def mul(a, s): return (a[0] * s, a[1] * s, a[2] * s)
def dot(a, b): return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
def cross(a, b): return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])
def norm(a): return math.sqrt(dot(a, a))


def unit(a):
    n = norm(a)
    return (a[0] / n, a[1] / n, a[2] / n) if n > 1e-12 else (0.0, 1.0, 0.0)


def direction(az, el):
    a, e = math.radians(az), math.radians(el)
    return (math.cos(e) * math.cos(a), math.sin(e), math.cos(e) * math.sin(a))


def rotate(v, axis, deg):
    a, k = math.radians(deg), unit(axis)
    c, s = math.cos(a), math.sin(a)
    return add(add(mul(v, c), mul(cross(k, v), s)), mul(k, dot(k, v) * (1 - c)))


def catmull(points, per):
    """🧵 Catmull-Rom samples through tuples of any length; `per[i]` samples on segment i, the last point included."""
    out = []
    n = len(points)
    for i in range(n - 1):
        p0, p1, p2, p3 = points[max(i - 1, 0)], points[i], points[i + 1], points[min(i + 2, n - 1)]
        for k in range(per[i]):
            t = k / per[i]
            out.append(tuple(0.5 * (2 * b + (c - a) * t + (2 * a - 5 * b + 4 * c - d) * t * t + (3 * b - a - 3 * c + d) * t ** 3) for a, b, c, d in zip(p0, p1, p2, p3)))
    out.append(tuple(points[-1]))
    return out


class Path:
    """🪵 A sampled axis: positions, a value tuple per sample (radii), tangents and the arc length."""

    def __init__(self, stations, step):
        pos = [tuple(s[:3]) for s in stations]
        per = [max(2, int(math.ceil(norm(sub(pos[i + 1], pos[i])) / step))) for i in range(len(pos) - 1)]
        self.p = catmull(pos, per)
        vals = []
        for i in range(len(pos) - 1):
            for k in range(per[i]):
                t = k / per[i]
                vals.append(tuple(a + (b - a) * t for a, b in zip(stations[i][3:], stations[i + 1][3:])))
        vals.append(tuple(stations[-1][3:]))
        self.v = vals
        self.s = [0.0]
        for i in range(1, len(self.p)):
            self.s.append(self.s[-1] + norm(sub(self.p[i], self.p[i - 1])))
        self.length = self.s[-1]
        n = len(self.p)
        self.t = [unit(sub(self.p[min(i + 1, n - 1)], self.p[max(i - 1, 0)])) for i in range(n)]

    def at(self, f):
        """📍 Position, values and tangent at the fraction `f` of the length."""
        target = max(0.0, min(1.0, f)) * self.length
        for i in range(1, len(self.s)):
            if self.s[i] >= target - 1e-12:
                span = self.s[i] - self.s[i - 1]
                k = 0 if span < 1e-12 else (target - self.s[i - 1]) / span
                pos = add(self.p[i - 1], mul(sub(self.p[i], self.p[i - 1]), k))
                val = tuple(a + (b - a) * k for a, b in zip(self.v[i - 1], self.v[i]))
                return pos, val, unit(add(mul(self.t[i - 1], 1 - k), mul(self.t[i], k)))
        return self.p[-1], self.v[-1], self.t[-1]

    def cut(self, f0, f1):
        """✂️ The samples between two fractions with exact end points: lists of (position, values, tangent)."""
        a, b = f0 * self.length, f1 * self.length
        gap = 0.35 * self.length / max(1, len(self.s) - 1)
        out = [self.at(f0)]
        for i in range(len(self.s)):
            if a + gap < self.s[i] < b - gap:
                out.append((self.p[i], self.v[i], self.t[i]))
        out.append(self.at(f1))
        return out


def heights(stations):
    """📏 For an upright axis: the function from a height y to the fraction of its length."""
    path = Path([tuple(map(float, s)) for s in stations], 0.12)

    def fraction(y):
        for i in range(1, len(path.p)):
            if path.p[i][1] >= y:
                a, b = path.p[i - 1][1], path.p[i][1]
                k = 0 if b == a else (y - a) / (b - a)
                return (path.s[i - 1] + k * (path.s[i] - path.s[i - 1])) / path.length
        return 1.0

    return fraction


class Mesh:
    """🔺 One closed triangular shell in world metres."""

    def __init__(self):
        self.v = []
        self.f = []

    def ring_loft(self, rings, start_pole, end_pole):
        n = len(rings[0])
        base = len(self.v)
        for ring in rings:
            self.v.extend(ring)
        for i in range(len(rings) - 1):
            for j in range(n):
                a, b = base + i * n + j, base + i * n + (j + 1) % n
                c, d = a + n, b + n
                self.f.append((a, b, d))
                self.f.append((a, d, c))
        s = len(self.v)
        self.v.append(start_pole)
        e = len(self.v)
        self.v.append(end_pole)
        last = base + (len(rings) - 1) * n
        for j in range(n):
            self.f.append((s, base + (j + 1) % n, base + j))
            self.f.append((e, last + j, last + (j + 1) % n))
        if self.volume() < 0:
            self.f = [(a, c, b) for a, b, c in self.f]
        return self

    def volume(self):
        return sum(dot(self.v[a], cross(self.v[b], self.v[c])) for a, b, c in self.f) / 6

    def area(self):
        return sum(norm(cross(sub(self.v[b], self.v[a]), sub(self.v[c], self.v[a]))) for a, b, c in self.f) / 2

    def centroid(self):
        n = len(self.v)
        return (sum(p[0] for p in self.v) / n, sum(p[1] for p in self.v) / n, sum(p[2] for p in self.v) / n)

    def ground(self):
        self.v = [(p[0], max(0.0, p[1]), p[2]) for p in self.v]
        return self


def transported(samples):
    """🧭 Rotation-minimising normals along samples of (position, values, tangent), starting towards the horizon."""
    t0 = samples[0][2]
    seed = cross(t0, (0.0, 1.0, 0.0))
    n = unit(seed) if norm(seed) > 0.2 else unit(cross(t0, (0.0, 0.0, 1.0)))
    out = []
    for _, _, t in samples:
        n = unit(sub(n, mul(t, dot(n, t))))
        out.append(n)
    return out


def tube(samples, sides, aspect=1.0, ridges=None, lobes=None):
    """🪵 A round, tapering limb through samples of (position, (radius,), tangent) with a flat buried start and a rounded tip; `aspect` stretches the section along the world vertical, `ridges` (count, depth m) flutes the bark, `lobes` [(count, share, phase deg, fade m)] swell a root collar."""
    normals = transported(samples)
    rings = []
    cap = []
    last_p, last_v, last_t = samples[-1]
    r_end = last_v[0]
    for q in (35, 65):
        a = math.radians(q)
        cap.append((add(last_p, mul(last_t, r_end * math.sin(a))), (r_end * math.cos(a),), last_t))
    capn = [normals[-1], normals[-1]]
    for (p, v, t), n in zip(samples + cap, normals + capn):
        b = cross(t, n)
        r = v[0]
        ring = []
        for j in range(sides):
            th = 2 * math.pi * j / sides
            k = 1.0
            if ridges:
                k += ridges[1] / r * (abs(math.sin(ridges[0] * th / 2 + 0.6 * math.sin(p[1] * 1.3))) - 0.6)
            for count, share, phase, fade in lobes or ():
                k += share * math.exp(-max(p[1], 0) / fade) * math.cos(count * th + math.radians(phase))
            off = add(mul(n, r * k * math.cos(th)), mul(b, r * k * math.sin(th)))
            if aspect != 1.0:
                off = (off[0], off[1] * aspect, off[2])
            ring.append(add(p, off))
        rings.append(ring)
    return Mesh().ring_loft(rings, samples[0][0], add(last_p, mul(last_t, r_end)))


def envelope(u, start=0.1, peak=0.6, base=0.35):
    """🍃 The swelling of a foliage mass along its branch: rounded where it begins, widest at `peak`, an elliptic nose beyond."""
    if u < peak:
        w = base + (1 - base) * math.sin(math.pi / 2 * u / peak)
        return w * (math.sqrt(max(0.0, 1 - (1 - u / start) ** 2)) if u < start else 1.0)
    return math.sqrt(max(0.0, 1 - ((u - peak) / (1 - peak)) ** 2))


def blob(samples, around, relief, flat=0.62, shape=None, phase=0.0, fine=0.0):
    """🍃 A foliage mass lofted along samples of (position, (rx, ry), tangent): horizontal half width rx, vertical half height ry, a flatter underside and a hand-stated relief (lobes around, lobes along, depth)."""
    k1, m1, depth = relief
    total = sum(norm(sub(samples[i][0], samples[i - 1][0])) for i in range(1, len(samples)))
    rings = []
    run = 0.0
    side = None
    for i, (p, v, t) in enumerate(samples):
        if i:
            run += norm(sub(p, samples[i - 1][0]))
        u = run / total
        w = shape(u) if shape else envelope(u)
        if w < 0.04:
            continue
        if side is None:
            h = cross(t, (0.0, 1.0, 0.0))
            side = unit(h) if norm(h) > 0.25 else unit(cross(t, (0.0, 0.0, 1.0)))
        side = unit(sub(side, mul(t, dot(side, t))))
        up = cross(side, t)
        ring = []
        for j in range(around):
            th = 2 * math.pi * j / around
            rho = 1 + depth * (math.cos(k1 * th + phase + 1.7 * u * m1) * math.cos(math.pi * m1 * u + phase * 0.7) + 0.55 * math.cos((k1 + 3) * th - 1.6 * phase + 2.9 * u) * math.cos(math.pi * (m1 + 2) * u + 2 * phase) + fine * math.cos((2 * k1 + 1) * th + 3.1 * phase) * math.cos(math.pi * (2 * m1 + 3) * u - phase))
            cs, sn = math.cos(th), math.sin(th)
            ring.append(add(p, add(mul(side, v[0] * w * rho * cs), mul(up, v[1] * w * rho * (sn if sn > 0 else sn * flat)))))
        rings.append(ring)
    return Mesh().ring_loft(rings, samples[0][0], samples[-1][0])


class Tree:
    """🌳 The hand statements of one tree and what follows from them: rig, solids per scale, anchors, masses."""

    def __init__(self, slug, bark, wood, wood_density, regions, leaf_materials, tones=None):
        self.slug = slug
        self.bark, self.wood, self.density = bark, wood, wood_density
        self.regions, self.leaf_materials = regions, leaf_materials
        self.tones = tones or {"trunk": 0.27, "limb": 0.3, "sun": 0.56, "shade": 0.42}
        self.axes = {}
        self.order = []
        self.bones = {}
        self.masses = []
        self.frames = {}

    def axis(self, id, stations, bones=None, bone=None, parent=None, kind="limb", sides=None, level=1, aspect=1.0, ridges=None, lobes=None, ground=False, scales=SCALES, parent_axis=None, at=None):
        """🪵 A woody axis through hand stations (x, y, z, radius). `bones` [(id, f0, f1, level, title en, title de, section noun en, section noun de)] cuts rig bones out of it; otherwise it is rigid to `bone`."""
        a = {"id": id, "stations": [tuple(map(float, s)) for s in stations], "bones": bones or [], "bone": bone, "kind": kind, "sides": sides, "level": level, "aspect": aspect, "ridges": ridges, "lobes": lobes, "ground": ground, "scales": scales, "parent_axis": parent_axis, "at": at}
        a["path"] = Path(a["stations"], 0.12)
        self.axes[id] = a
        self.order.append(id)
        prev = parent
        for spec in a["bones"]:
            bid, f0, f1, lvl, en, de, sen, sde = spec
            p0, _, _ = a["path"].at(f0)
            p1, _, _ = a["path"].at(f1)
            _, v0, _ = a["path"].at(f0 + 0.3 * (f1 - f0))
            self.bones[bid] = {"id": bid, "parent": prev, "a": p0, "b": p1, "level": lvl, "title": {"en": en, "de": de}, "r": v0[0], "noun": (sen, sde), "axis": id, "f": (f0, f1)}
            prev = bid
        return a

    def fy(self, axis_id, y):
        """📏 The fraction of the axis length at which it first reaches the height y."""
        path = self.axes[axis_id]["path"]
        for i in range(1, len(path.p)):
            if path.p[i][1] >= y:
                a, b = path.p[i - 1][1], path.p[i][1]
                k = 0 if b == a else (y - a) / (b - a)
                return (path.s[i - 1] + k * (path.s[i] - path.s[i - 1])) / path.length
        return 1.0

    def bone_at(self, axis_id, f):
        a = self.axes[axis_id]
        for bid, f0, f1, *_ in a["bones"]:
            if f0 - 1e-9 <= f <= f1 + 1e-9:
                return bid
        return a["bone"]

    def branch(self, id, parent, at, az, el, length, r0, r1=None, up=0.0, turn=0.0, bones=None, bone=None, kind="limb", level=2, sides=None, scales=SCALES, wave=0.0, collar=1.25, bend=1.0, socket=False):
        """🌿 A branch leaving the axis `parent` at the fraction `at` towards (az, el), `length` long, base radius r0, tip radius r1; with `socket` a fixed collar bone from the parent axis to its bark turns the hinge of the first bone into the horizontal across the branch, so the branch bends in its own vertical plane; over its length it turns up by `up` (late when `bend` > 1: it holds its angle first and arches over near the end) and sideways by `turn` degrees and swings `wave` metres out of line (the elbow of a gnarled limb)."""
        origin, pv, _ = self.axes[parent]["path"].at(at)
        r1 = r1 if r1 is not None else max(0.004, r0 * 0.12)
        stations = [(origin[0], origin[1], origin[2], r0 * collar)]
        p = origin
        steps = 6
        for i in range(steps):
            f = (i + 0.5) / steps
            d = direction(az + turn * f, el + up * f ** bend)
            p = add(p, mul(d, length / steps))
            g = (i + 1) / steps
            q = p
            if wave:
                lateral = unit(cross(d, (0.0, 1.0, 0.0)))
                q = add(p, mul(lateral, wave * math.sin(math.pi * 2 * g) * (1 - g * 0.5)))
            r = r0 + (r1 - r0) * g ** 0.85
            stations.append((q[0], q[1], q[2], r))
        host = self.bone_at(parent, at)
        collar_id = None
        if socket and bones:
            exit_f = min(0.3, pv[0] / max(0.25, math.cos(math.radians(el))) / length)
            first = bones[0]
            collar_id = first[0] + "-collar"
            bones = [(collar_id, 0.0, exit_f, 0, "Collar of the " + first[4][0].lower() + first[4][1:], "Astansatz: " + first[5], "Collar", "Astansatz"), (first[0], exit_f) + tuple(first[2:])] + list(bones[1:])
        made = self.axis(id, stations, bones=bones, bone=bone or (None if bones else host), parent=host if bones else None, kind=kind, sides=sides, level=level, scales=scales, parent_axis=parent, at=at)
        if collar_id:
            out = direction(az, 0)
            self.bones[collar_id].update(a=origin, b=add(origin, mul(out, pv[0])), fixed=True)
        return made

    def spray(self, id, axis, f0, rx, ry, relief, f1=1.0, over=0.3, lift=0.0, shade=False, scales=(50, 100), bone=None, flat=0.62, peak=0.6, base=0.35, drop=0.0, cls=None, group=None):
        """🍃 A foliage mass that follows the axis from fraction f0 to f1 and `over` metres beyond its tip: half width rx, half height ry, raised by `lift` (sagging by `drop` towards the tip), relief (lobes around, lobes along, depth)."""
        self.masses.append({"id": id, "axis": axis, "f0": f0, "f1": f1, "over": over, "rx": rx, "ry": ry, "lift": lift, "drop": drop, "relief": relief, "shade": shade, "scales": scales, "bone": bone, "flat": flat, "peak": peak, "base": base, "stations": None, "cls": cls, "group": group})

    def mass(self, id, bone, stations, relief, shade=False, scales=(50, 100), flat=0.62, peak=0.5, base=0.6, cls=None, group=None):
        """🍃 A foliage mass through hand stations (x, y, z, rx, ry), rigid to `bone`."""
        self.masses.append({"id": id, "axis": None, "stations": [tuple(map(float, s)) for s in stations], "relief": relief, "shade": shade, "scales": scales, "bone": bone, "flat": flat, "peak": peak, "base": base, "cls": cls, "group": group})

    def solve(self):
        """🦴 The rest frames of the bones from their world joints (the product's own chain: sagittal angle, then abduction)."""
        base = {"o": (0.0, 0.0, 0.0), "u": (1.0, 0.0, 0.0), "v": (0.0, 1.0, 0.0), "w": (0.0, 0.0, 1.0)}
        rig = []
        for bid, b in self.bones.items():
            parent = self.frames[b["parent"]] if b["parent"] else base
            rel = sub(b["a"], parent["o"])
            x, y, z = dot(rel, parent["u"]), dot(rel, parent["v"]), dot(rel, parent["w"])
            d = unit(sub(b["b"], b["a"]))
            du, dv, dw = dot(d, parent["u"]), dot(d, parent["v"]), dot(d, parent["w"])
            phi = math.degrees(math.asin(max(-1.0, min(1.0, dw))))
            theta = math.degrees(math.atan2(dv, du))
            theta, phi, x, y, z = round(theta, 3), round(phi, 3), round(x, 4), round(y, 4), round(z, 4)
            ts, tc, ps, pc = math.sin(math.radians(theta)), math.cos(math.radians(theta)), math.sin(math.radians(phi)), math.cos(math.radians(phi))
            u1 = add(mul(parent["u"], tc), mul(parent["v"], ts))
            v1 = add(mul(parent["u"], -ts), mul(parent["v"], tc))
            o = add(parent["o"], add(add(mul(parent["u"], x), mul(parent["v"], y)), mul(parent["w"], z)))
            self.frames[bid] = {"o": o, "u": add(mul(u1, pc), mul(parent["w"], ps)), "v": v1, "w": add(mul(u1, -ps), mul(parent["w"], pc))}
            b.update(x=x, y=y, z=z, angleDeg=theta, abductionDeg=phi, lengthM=round(norm(sub(b["b"], b["a"])), 4))
            rig.append(b)
        return rig

    def local(self, bone, p):
        f = self.frames[bone]
        d = sub(p, f["o"])
        return (dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"]))

    def offsets(self, bone, kind, deg):
        """🌬️ The joint offset that realises a turn of `deg` degrees as far as the hinge of the bone allows: `lee` about the lateral world axis (negative leans ahead, as in a sagittal rig), `droop` towards the ground. Since 2026-11-22 the product turns an offset about the own rest joint axis of the bone (`T/rig-lateral-rest.md`), so the offset is the share of the turn about that axis; no spread and no twist is used, so every flat drawing stays in its plane."""
        b = self.bones[bone]
        d = unit(sub(b["b"], b["a"]))
        if kind == "lee":
            omega = (0.0, 0.0, deg)
        else:
            h = cross(d, (0.0, -1.0, 0.0))
            omega = (0.0, 0.0, 0.0) if norm(h) < 1e-6 else mul(unit(h), abs(deg))
        return {"angleDeg": round(dot(omega, self.frames[bone]["w"]), 2)}

    def mass_samples(self, m, scale):
        step = RING_STEP[scale] * 1.7
        rings = {50: 8, 100: 7, 200: 6, 500: 6}[scale]
        if m["stations"] is not None:
            span = sum(norm(sub(m["stations"][i][:3], m["stations"][i - 1][:3])) for i in range(1, len(m["stations"])))
            path = Path(m["stations"], min(step, span / rings))
            return [(p, v, t) for p, v, t in zip(path.p, path.v, path.t)]
        path = self.axes[m["axis"]]["path"]
        cut = path.cut(m["f0"], m["f1"])
        pts = [c[0] for c in cut]
        end, tan = pts[-1], cut[-1][2]
        if m["over"] > 0:
            flat_tan = unit((tan[0], tan[1] * 0.4, tan[2]))
            pts.append(add(end, mul(flat_tan, m["over"])))
        reach = sum(norm(sub(pts[i], pts[i - 1])) for i in range(1, len(pts)))
        thin = [pts[0]]
        for p in pts[1:-1]:
            if norm(sub(p, thin[-1])) >= min(step, reach / 4):
                thin.append(p)
        thin.append(pts[-1])
        if len(thin) < 3:
            thin = [pts[0], mul(add(pts[0], pts[-1]), 0.5), pts[-1]]
        total = sum(norm(sub(thin[i], thin[i - 1])) for i in range(1, len(thin)))
        run, stations = 0.0, []
        for i, p in enumerate(thin):
            if i:
                run += norm(sub(p, thin[i - 1]))
            u = run / total
            stations.append((p[0], p[1] + m["lift"] * math.sin(math.pi * min(1.0, u * 1.2) / 2) - m["drop"] * u * u, p[2], m["rx"], m["ry"]))
        path2 = Path(stations, min(step, total / rings))
        return [(p, v, t) for p, v, t in zip(path2.p, path2.v, path2.t)]

    def build(self, scale, around_leaf=None, relief_gain=None, box=None):
        """🏗️ The world meshes of one scale: wood solids per axis and bone, foliage masses, with their bindings. With `box` (the extent of the finest model) a merged mass of a coarser scale that would reach beyond it is narrowed in steps of 5 % until it lies inside: a coarser crown never grows."""
        min_r = MIN_DIA[scale] / 2
        step = RING_STEP[scale]
        digits = ROUND[scale]
        wood, leaves = [], []
        default_sides = {50: {0: 40, 1: 16, 2: 10, 3: 6, 4: 6}, 100: {0: 28, 1: 12, 2: 8, 3: 6, 4: 6}, 200: {0: 18, 1: 8, 2: 6, 3: 6, 4: 6}, 500: {0: 12, 1: 6, 2: 6, 3: 6, 4: 6}}[scale]
        for aid in self.order:
            a = self.axes[aid]
            if scale not in a["scales"] or (a["stations"][0][3] < min_r * 1.05 and not a.get("keep") and not a["bones"]):
                continue
            stations = [(s[0], s[1], s[2], max(s[3], min_r)) for s in a["stations"]]
            fine_step = step * (1.0 if a["level"] < 2 else 1.6 if a["level"] == 2 else 2.2)
            path = Path(stations, fine_step)
            true = Path(a["stations"], fine_step)
            sides = a["sides"][SCALES.index(scale)] if a["sides"] else default_sides[min(a["level"], 4)]
            ridges = a["ridges"] if a["ridges"] and scale == 50 else None
            if ridges:
                sides = max(sides, ridges[0] * 4)
            pieces = a["bones"] or [(a["bone"], 0.0, 1.0)]
            for k, piece in enumerate(pieces):
                bid, f0, f1 = piece[0], piece[1], piece[2]
                thick = true.at(f0)[1][0]
                slender = thick < min_r * 1.05 and not a.get("keep") and (f1 - f0) * true.length / (2 * thick) >= 24
                source = true if slender else path
                back = 0.0
                if k > 0:
                    back = min(f0, 1.6 * source.at(f0)[1][0] / source.length)
                samples = source.cut(f0 - back, f1)
                fluted = None if slender or ridges is None or min(q[1][0] for q in samples) < 8 * ridges[1] else ridges
                mesh = tube(samples, 6 if slender else sides if fluted or not ridges else default_sides[min(a["level"], 4)], a["aspect"], fluted, a["lobes"])
                if a["ground"]:
                    mesh.ground()
                sid = aid if len(pieces) == 1 else (bid if bid not in self.axes or bid == aid else f"{aid}-{bid}")
                thin = samples if len(samples) < 5 else [q for i, q in enumerate(samples) if i % 2 == 0 or i == len(samples) - 1]
                core = [(p, (v[0] * 0.55 if slender else max(v[0] * 0.55, MIN_DIA[scale] * 0.26),), t) for p, v, t in thin]
                wood.append({"id": sid, "bone": bid, "axis": aid, "kind": a["kind"], "mesh": mesh, "core": tube(core, 6 if slender else max(6, sides // 3)).ground() if a["ground"] else tube(core, 6 if slender else max(6, sides // 3)), "samples": samples, "level": a["level"], "slender": slender})
        gain = relief_gain if relief_gain is not None else {50: 1.0, 100: 0.8, 200: 0.75, 500: 0.6}[scale]
        spacing = {50: 0.3, 100: 0.5, 200: 0.8, 500: 1.3}[scale]
        for index, m in enumerate(self.masses):
            if scale not in m["scales"]:
                continue
            k1, m1, depth = m["relief"]
            shape = (lambda u, mm=m: envelope(u, 0.1, mm["peak"], mm["base"]))
            spec = m
            for attempt in range(12):
                samples = self.mass_samples(spec, scale)
                wide = spec["rx"] if spec["stations"] is None else max(q[3] for q in spec["stations"])
                around = around_leaf or max({50: 16, 100: 14, 200: 12, 500: 12}[scale], min(36, int(round(6.2832 * wide / spacing))))
                mesh = blob(samples, around, (k1, m1, depth * gain), m["flat"], shape, phase=index * 2.399963, fine=0.4 if scale == 50 else 0.0)
                if box is None or all(box[0][k] - 1e-3 <= p[k] <= box[1][k] + 1e-3 for p in mesh.v for k in range(3)):
                    break
                spec = dict(spec)
                if spec["stations"] is None:
                    spec.update(rx=spec["rx"] * 0.95, ry=spec["ry"] * 0.95, over=spec["over"] * 0.8, lift=spec["lift"] * 0.9)
                else:
                    spec["stations"] = [(q[0], q[1], q[2], q[3] * 0.95, q[4] * 0.95) for q in spec["stations"]]
            bone = m["bone"] or self.bone_at(m["axis"], (m["f0"] + m["f1"]) / 2)
            leaves.append({"id": m["id"], "bone": bone, "mesh": mesh, "shade": m["shade"], "spec": m, "samples": samples})
        self.digits = digits
        return wood, leaves

    def solid(self, id, bone, mesh, material, tone, role="body", extra=None, digits=4):
        out = {"id": id, "kind": "polyhedron", "bone": bone, "material": material, "role": role, "tone": tone}
        out.update(extra or {})
        out["vertices"] = [[round(c, digits) for c in self.local(bone, p)] for p in mesh.v]
        out["triangles"] = [list(f) for f in mesh.f]
        return out


def sphere(center, radius, n=8):
    rings = []
    for i in range(1, n // 2):
        lat = math.pi * i / (n // 2) - math.pi / 2
        rings.append([add(center, (radius * math.cos(lat) * math.cos(2 * math.pi * j / n), radius * math.sin(lat), radius * math.cos(lat) * math.sin(2 * math.pi * j / n))) for j in range(n)])
    return Mesh().ring_loft(rings, add(center, (0, -radius, 0)), add(center, (0, radius, 0)))


def lens(center, radius, height, n=12):
    """💧 A low dome seated on a foliage top: the outlet patch of a transpiration emitter."""
    rings = [[add(center, (radius * k * math.cos(2 * math.pi * j / n), height * h, radius * k * math.sin(2 * math.pi * j / n))) for j in range(n)] for k, h in ((1.0, 0.0), (0.7, 0.6))]
    return Mesh().ring_loft(rings, add(center, (0, -height * 0.4, 0)), add(center, (0, height, 0)))


def disc(center, radius, height, n=24):
    rings = [[add(center, (radius * math.cos(2 * math.pi * j / n), h, radius * math.sin(2 * math.pi * j / n))) for j in range(n)] for h in (0.0, height)]
    return Mesh().ring_loft(rings, center, add(center, (0, height, 0)))


def closed(mesh):
    """🔒 True when every edge is shared by exactly two triangles in opposite senses."""
    edges = {}
    for a, b, c in mesh.f:
        for e in ((a, b), (b, c), (c, a)):
            edges[e] = edges.get(e, 0) + 1
    return all(n == 1 and edges.get((b, a)) == 1 for (a, b), n in edges.items())


def write(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(value, f, ensure_ascii=False, separators=(",", ":"))
