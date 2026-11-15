"""Author 16 helper: compact constructors for handcrafted volume solids and the writer of T/volumes/<asset>.json."""
import json, math, os, copy

T = os.path.dirname(os.path.abspath(__file__))


def r5(x):
    return round(x + 0.0, 5)


def vec(v):
    return [r5(x) for x in v]


def _base(id, bone, mat, kind, tone, role, at=None, rot=None, rep=None, scale=None, **kw):
    s = {"id": id, "kind": kind, "bone": bone, "material": mat, "role": role, "tone": tone}
    s.update(kw)
    tr = {}
    if at is not None and any(abs(a) > 1e-9 for a in at):
        tr["translation"] = vec(at)
    if rot is not None:
        tr["rotation"] = [r5(x) for x in rot]
    if scale is not None:
        tr["scale"] = vec(scale)
    if tr:
        s["transform"] = tr
    if rep is not None:
        s["repeat"] = rep
    return s


def box(id, bone, mat, tone, size, at=(0, 0, 0), r=0, role="body", rot=None, rep=None):
    kw = {"size": vec(size)}
    if r:
        kw["cornerRadius"] = r5(r)
    return _base(id, bone, mat, "box", tone, role, at, rot, rep, **kw)


def ell(id, bone, mat, tone, radii, at=(0, 0, 0), role="body", rot=None, rep=None):
    return _base(id, bone, mat, "ellipsoid", tone, role, at, rot, rep, radii=vec(radii))


def cap(id, bone, mat, tone, p0, p1, r0, r1=None, role="body", rep=None, lateral=None):
    kw = {"p0": vec(p0), "p1": vec(p1), "r0": r5(r0), "r1": r5(r0 if r1 is None else r1)}
    if lateral:
        kw["lateralScale"] = lateral
    return _base(id, bone, mat, "capsule", tone, role, None, None, rep, **kw)


def rev(id, bone, mat, tone, profile, axis, origin=(0, 0, 0), role="body", rep=None, at=None, rot=None):
    return _base(id, bone, mat, "revolve", tone, role, at, rot, rep, profile=[vec(p) for p in profile], axis=vec(axis), origin=vec(origin))


def cyl(id, bone, mat, tone, p0, p1, r, role="body", rep=None, r1=None):
    d = [p1[i] - p0[i] for i in range(3)]
    n = math.sqrt(sum(x * x for x in d))
    return rev(id, bone, mat, tone, [[0, r], [n, r if r1 is None else r1]], [x / n for x in d], p0, role, rep)


def ext(id, bone, mat, tone, profile, plane, a, b, role="body", at=None, rep=None, holes=None, taper=None):
    kw = {"profile": [vec(p) for p in profile], "plane": plane, "from": r5(a), "to": r5(b)}
    if holes:
        kw["holes"] = [[vec(p) for p in h] for h in holes]
    if taper:
        kw["taperTo"] = vec(taper)
    return _base(id, bone, mat, "extrude", tone, role, at, None, rep, **kw)


def qz(deg):
    a = math.radians(deg) / 2
    return [0, 0, math.sin(a), math.cos(a)]


def qy(deg):
    a = math.radians(deg) / 2
    return [0, math.sin(a), 0, math.cos(a)]


def qx(deg):
    a = math.radians(deg) / 2
    return [math.sin(a), 0, 0, math.cos(a)]


def loft(id, bone, mat, tone, sections, role="body", rep=None, caps=(True, True)):
    secs = []
    for s in sections:
        d = {"at": vec(s[0]), "rx": r5(s[1]), "ry": r5(s[2])}
        if len(s) > 3 and s[3] is not None:
            d["exponent"] = s[3]
        if len(s) > 4 and s[4] is not None:
            d["rotation"] = [r5(x) for x in s[4]]
        secs.append(d)
    kw = {"sections": secs, "capStart": caps[0], "capEnd": caps[1]}
    return _base(id, bone, mat, "loft", tone, role, None, None, rep, **kw)


def rep(count, step, origin=None):
    d = {"count": count, "translation": vec(step)}
    if origin:
        d["origin"] = vec(origin)
    return d


def shrink(s, t, id=None):
    """Offsets a solid inwards by t (thermal inner layer): sizes, radii and sections shrink, ends of revolves move in."""
    c = copy.deepcopy(s)
    if id:
        c["id"] = id
    k = c["kind"]
    if k == "box":
        c["size"] = vec([max(1e-4, x - 2 * t) for x in c["size"]])
        if "cornerRadius" in c:
            c["cornerRadius"] = r5(max(0.0, c["cornerRadius"] - t))
            if c["cornerRadius"] == 0:
                del c["cornerRadius"]
    elif k == "ellipsoid":
        c["radii"] = vec([max(1e-4, x - t) for x in c["radii"]])
    elif k == "capsule":
        c["r0"] = r5(max(1e-4, c["r0"] - t))
        c["r1"] = r5(max(1e-4, c["r1"] - t))
    elif k == "revolve":
        p = c["profile"]
        q = [[x[0], max(1e-4, x[1] - t)] for x in p]
        if len(q) >= 2:
            q[0][0] += t if q[0][0] <= q[-1][0] else -t
            q[-1][0] += -t if q[0][0] <= q[-1][0] else t
        c["profile"] = [vec(x) for x in q]
    elif k == "loft":
        for sec in c["sections"]:
            sec["rx"] = r5(max(1e-4, sec["rx"] - t))
            sec["ry"] = r5(max(1e-4, sec["ry"] - t))
    elif k == "extrude":
        pr = c["profile"]
        cx = sum(x[0] for x in pr) / len(pr)
        cy = sum(x[1] for x in pr) / len(pr)
        xs = [x[0] for x in pr]
        ys = [x[1] for x in pr]
        hx = (max(xs) - min(xs)) / 2
        hy = (max(ys) - min(ys)) / 2
        fx = max(0.05, 1 - t / hx) if hx > 0 else 1
        fy = max(0.05, 1 - t / hy) if hy > 0 else 1
        c["profile"] = [vec([cx + (x[0] - cx) * fx, cy + (x[1] - cy) * fy]) for x in pr]
        lo, hi = min(c["from"], c["to"]), max(c["from"], c["to"])
        c["from"], c["to"] = r5(lo + t), r5(hi - t)
    return c


def tag(s, **kw):
    s = dict(s)
    s.update(kw)
    return s


def ref(id, of, region, layer=None, **kw):
    d = {"id": id, "of": of, "region": region}
    if layer:
        d["layer"] = layer
    d.update(kw)
    return d


def th(s, region, layer=None, id=None):
    c = copy.deepcopy(s)
    c["id"] = id or "th-" + s["id"]
    c["region"] = region
    if layer:
        c["layer"] = layer
    return c


def st(s, member, part, id=None):
    c = copy.deepcopy(s)
    if id:
        c["id"] = id
    c["member"] = member
    c["part"] = part
    return c


def stref(id, of, member, part):
    return {"id": id, "of": of, "member": member, "part": part}


def texts(en, de):
    return {"en": en, "de": de}


class Scale:
    def __init__(self, denominator):
        self.d = denominator
        self.body, self.thermal, self.structure, self.decisions = [], [], [], []

    def b(self, *s):
        self.body.extend(s)
        return self

    def t(self, *s):
        self.thermal.extend(s)
        return self

    def s(self, *s):
        self.structure.extend(s)
        return self

    def say(self, en, de):
        self.decisions.append(texts(en, de))

    def dump(self):
        return {"status": "authored", "denominator": self.d, "body": self.body, "thermal": self.thermal, "structure": self.structure, "decisions": self.decisions}


def unavailable(d):
    return {"status": "unavailable", "denominator": d}


def write(category, slug, scales):
    out = {"volumes": {}}
    for k in ("50", "100", "200", "500"):
        v = scales[k]
        out["volumes"][k] = v.dump() if isinstance(v, Scale) else v
    path = os.path.join(T, "volumes", category, slug + ".json")
    with open(path, "w") as f:
        json.dump(out, f, ensure_ascii=False, indent=1)
    return path


def unseeded(category, slug):
    return json.load(open(os.path.join(T, "volumes", category, slug + ".json")))["volumes"]


CATALOG = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(T))))))), "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog")


def asset_path(category_dir, slug):
    return os.path.join(CATALOG, category_dir, slug, "🔣️.json")


def mulq(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def qconj(q):
    return [-q[0], -q[1], -q[2], q[3]]


class World:
    """World-coordinate constructors: every position, axis and size is given in world metres (x forward/right, y up, z lateral) and mapped into the rest frame of the bone (planar rig frames)."""

    def __init__(self, category_dir, slug):
        self.asset = json.load(open(asset_path(category_dir, slug)))["asset"]
        self.frames = {}
        bones = {b["id"]: b for b in self.asset["rig"]["bones"]}
        for b in self.asset["rig"]["bones"]:
            self._frame(b["id"], bones)

    def _frame(self, id, bones):
        if id in self.frames:
            return self.frames[id]
        b = bones[id]
        if b["parent"] is None:
            o, a = (b["x"], b["y"]), b["angleDeg"]
        else:
            po, pa = self._frame(b["parent"], bones)
            c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            o = (po[0] + c * b["x"] - s * b["y"], po[1] + s * b["x"] + c * b["y"])
            a = pa + b["angleDeg"]
        self.frames[id] = (o, a)
        return o, a

    def P(self, bone, p):
        (ox, oy), a = self.frames[bone]
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        dx, dy = p[0] - ox, p[1] - oy
        return [c * dx + s * dy, -s * dx + c * dy, p[2]]

    def D(self, bone, d):
        (ox, oy), a = self.frames[bone]
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        return [c * d[0] + s * d[1], -s * d[0] + c * d[1], d[2]]

    def Q(self, bone, q):
        a = self.frames[bone][1]
        return mulq(qz(-a), q)

    def S(self, bone, size):
        d = self.D(bone, size)
        return [abs(x) for x in d]

    def box(self, bone, id, mat, tone, c, size, r=0, role="body", rotz=None, rep=None):
        return box(id, bone, mat, tone, self.S(bone, size), self.P(bone, c), r, role, qz(rotz) if rotz else None, rep)

    def ell(self, bone, id, mat, tone, c, radii, role="body", rotz=None, rep=None):
        return ell(id, bone, mat, tone, self.S(bone, radii), self.P(bone, c), role, qz(rotz) if rotz else None, rep)

    def cap(self, bone, id, mat, tone, p0, p1, r0, r1=None, role="body", rep=None, lateral=None):
        return cap(id, bone, mat, tone, self.P(bone, p0), self.P(bone, p1), r0, r1, role, rep, lateral)

    def cyl(self, bone, id, mat, tone, p0, p1, r, role="body", rep=None, r1=None):
        return cyl(id, bone, mat, tone, self.P(bone, p0), self.P(bone, p1), r, role, rep, r1)

    def rev(self, bone, id, mat, tone, profile, axis, origin, role="body", rep=None):
        return rev(id, bone, mat, tone, profile, self.D(bone, axis), self.P(bone, origin), role, rep)

    def ext(self, bone, id, mat, tone, profile, a, b, role="body", rep=None, holes=None, taper=None):
        """Profile in world (x, y), pushed along z between a and b."""
        pr = [self.P(bone, (p[0], p[1], 0))[:2] for p in profile]
        hs = [[self.P(bone, (p[0], p[1], 0))[:2] for p in h] for h in holes] if holes else None
        return ext(id, bone, mat, tone, pr, "xy", a, b, role, None, rep, hs, taper)

    def extx(self, bone, id, mat, tone, profile_yz, a, b, role="body", rep=None):
        """Profile in world (y, z), pushed along world x between a and b (frames rotated by multiples of 180 degrees only)."""
        ang = self.frames[bone][1] % 360
        assert ang in (0, 180), "extx needs an unrotated or 180 degree frame"
        sg = 1 if ang == 0 else -1
        ox, oy = self.frames[bone][0]
        pr = [[sg * (p[0] - oy), p[1]] for p in profile_yz]
        la, lb = sg * (a - ox), sg * (b - ox)
        return ext(id, bone, mat, tone, pr, "yz", min(la, lb), max(la, lb), role, None, rep)

    def loft(self, bone, id, mat, tone, sections, role="body", caps=(True, True), rep=None):
        """Sections (centre_world, rx, ry, exponent, rotation_world_quaternion)."""
        secs = []
        for s in sections:
            q = None if len(s) < 5 or s[4] is None else self.Q(bone, s[4])
            secs.append((self.P(bone, s[0]), s[1], s[2], s[3] if len(s) > 3 else None, q))
        return loft(id, bone, mat, tone, secs, role, rep, caps)

    def th_inner(self, s, t, id):
        return shrink(s, t, id)


def qaxes(x, y, z):
    """Quaternion of the rotation whose columns are the unit axes x, y, z."""
    m00, m10, m20 = x
    m01, m11, m21 = y
    m02, m12, m22 = z
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
        return [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s]
    if m00 > m11 and m00 > m22:
        s = math.sqrt(1 + m00 - m11 - m22) * 2
        return [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    if m11 > m22:
        s = math.sqrt(1 + m11 - m00 - m22) * 2
        return [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s]
    s = math.sqrt(1 + m22 - m00 - m11) * 2
    return [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s]


def pipe_loft_q(d):
    """Section rotation for a loft running along the horizontal direction d=(dx,dz): z along d, y up, x lateral."""
    n = math.hypot(d[0], d[1])
    dx, dz = d[0] / n, d[1] / n
    return qaxes((dz, 0, -dx), (0, 1, 0), (dx, 0, dz))


def _scalar(value):
    return not isinstance(value, (dict, list))


def _inline(value):
    if isinstance(value, list):
        return all(_scalar(item) for item in value) or (all(isinstance(item, list) and all(_scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth=0):
    """The catalog's own formatting (same as volumes-insert.py)."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if _inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


import re


def _member_end(text, start):
    depth, index, quoted = 0, start, False
    while index < len(text):
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    raise SystemExit("unbalanced")


def _js(text):
    def fix(m):
        v = float(m.group(0))
        if abs(v) >= 1e-6:
            return format(__import__("decimal").Decimal(m.group(0)), "f")
        mant, exp = m.group(0).split("e")
        return mant + "e-" + str(int(exp[1:]))
    return re.sub(r"-?\d+(?:\.\d+)?e-\d+", fix, text)


def edit_asset(category_dir, slug, fn):
    """Re-reads the asset, applies fn(asset dict) and writes it back; the member `volume`/`volumes` keeps its text and place, every other member is written with indent 2 (the file's own formatting)."""
    p = asset_path(category_dir, slug)
    raw = open(p, encoding="utf-8").read()
    doc = json.loads(raw)
    keep = None
    for key in ("volumes", "volume"):
        m = re.search(r'\n    "' + key + r'"\s*:\s*', raw)
        if m:
            keep = (key, raw[m.start() + 1 : _member_end(raw, m.end())])
            break
    asset = doc["asset"]
    if keep:
        asset.pop(keep[0])
    plain = _js(json.dumps(doc, indent=2, ensure_ascii=False))
    n = fn(asset)
    new = _js(json.dumps(doc, indent=2, ensure_ascii=False))
    if keep:
        place = re.search(r'\n    "presentation"\s*:', new)
        new = new[: place.start() + 1] + keep[1] + ",\n" + new[place.start() + 1 :]
        place0 = re.search(r'\n    "presentation"\s*:', plain)
        plain = plain[: place0.start() + 1] + keep[1] + ",\n" + plain[place0.start() + 1 :]
    if plain != raw and plain + "\n" != raw:
        raise SystemExit("asset formatting not reproducible, refusing: " + p)
    open(p, "w", encoding="utf-8").write(new + ("\n" if raw.endswith("\n") else ""))
    return n
