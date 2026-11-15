import json, math, os

T = os.path.dirname(os.path.abspath(__file__))


def r6(x):
    return round(x + 0.0, 6)


def vec(v):
    return [r6(a) for a in v]


def qaxis(axis, deg):
    a = math.radians(deg) / 2
    n = math.sqrt(sum(c * c for c in axis))
    s = math.sin(a) / n
    return [r6(axis[0] * s), r6(axis[1] * s), r6(axis[2] * s), r6(math.cos(a))]


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def base(kind, id, bone, mat, tone, role, at, rot, repeat, tags):
    d = {"id": id, "kind": kind, "bone": bone, "material": mat, "role": role, "tone": tone}
    return d


def place(d, at=None, rot=None, repeat=None, tags=None):
    tr = {}
    if at is not None and any(abs(c) > 1e-12 for c in at):
        tr["translation"] = vec(at)
    if rot is not None:
        tr["rotation"] = [r6(c) for c in rot]
    if tr:
        d["transform"] = tr
    if repeat is not None:
        d["repeat"] = repeat
    if tags:
        d["tags"] = tags
    return d


def box(id, bone, mat, tone, size, at=None, r=None, role="body", rot=None, repeat=None, tags=None):
    d = base("box", id, bone, mat, tone, role, at, rot, repeat, tags)
    d["size"] = vec(size)
    if r:
        d["cornerRadius"] = r6(r)
    return place(d, at, rot, repeat, tags)


def ellipsoid(id, bone, mat, tone, radii, at=None, role="body", rot=None, repeat=None, tags=None):
    d = base("ellipsoid", id, bone, mat, tone, role, at, rot, repeat, tags)
    d["radii"] = vec(radii)
    return place(d, at, rot, repeat, tags)


def capsule(id, bone, mat, tone, p0, p1, r0, r1=None, role="body", at=None, rot=None, repeat=None, tags=None):
    d = base("capsule", id, bone, mat, tone, role, at, rot, repeat, tags)
    d.update({"p0": vec(p0), "p1": vec(p1), "r0": r6(r0), "r1": r6(r0 if r1 is None else r1)})
    return place(d, at, rot, repeat, tags)


AX = {"x": [1, 0, 0], "y": [0, 1, 0], "z": [0, 0, 1]}


def cyl(id, bone, mat, tone, axis, center, length, radius, role="body", rot=None, repeat=None, tags=None):
    a = AX[axis]
    d = base("revolve", id, bone, mat, tone, role, None, rot, repeat, tags)
    d["profile"] = [[0, r6(radius)], [r6(length), r6(radius)]]
    d["axis"] = a
    d["origin"] = vec([center[i] - a[i] * length / 2 for i in range(3)])
    return place(d, None, rot, repeat, tags)


def revolve(id, bone, mat, tone, axis, origin, profile, role="body", sweep=None, rot=None, repeat=None, tags=None):
    d = base("revolve", id, bone, mat, tone, role, None, rot, repeat, tags)
    d["profile"] = [vec(p) for p in profile]
    d["axis"] = AX[axis] if isinstance(axis, str) else vec(axis)
    d["origin"] = vec(origin)
    if sweep is not None:
        d["sweepRad"] = r6(sweep)
    return place(d, None, rot, repeat, tags)


def cw(ring):
    a = sum(ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1] for i in range(len(ring)))
    return list(ring) if a < 0 else list(reversed(ring))


def ext(id, bone, mat, tone, plane, profile, a, b, holes=None, role="body", at=None, rot=None, repeat=None, tags=None):
    d = base("extrude", id, bone, mat, tone, role, at, rot, repeat, tags)
    d["profile"] = [vec(p) for p in cw(profile)]
    if holes:
        d["holes"] = [[vec(p) for p in cw(h)] for h in holes]
    d["plane"] = plane
    d["from"] = r6(a)
    d["to"] = r6(b)
    return place(d, at, rot, repeat, tags)


def loft(id, bone, mat, tone, sections, role="body", capStart=True, capEnd=True, at=None, rot=None, repeat=None, tags=None):
    d = base("loft", id, bone, mat, tone, role, at, rot, repeat, tags)
    d["sections"] = sections
    d["capStart"] = capStart
    d["capEnd"] = capEnd
    return place(d, at, rot, repeat, tags)


def sec(at, rx, ry, exponent=None, rotation=None):
    s = {"at": vec(at), "rx": r6(rx), "ry": r6(ry)}
    if exponent:
        s["exponent"] = exponent
    if rotation:
        s["rotation"] = rotation
    return s


def rect(cx, cy, w, h):
    return [[cx - w / 2, cy - h / 2], [cx + w / 2, cy - h / 2], [cx + w / 2, cy + h / 2], [cx - w / 2, cy + h / 2]]


def rrect(cx, cy, w, h, c, n=1):
    x0, x1, y0, y1 = cx - w / 2, cx + w / 2, cy - h / 2, cy + h / 2
    return [[x0 + c, y0], [x1 - c, y0], [x1, y0 + c], [x1, y1 - c], [x1 - c, y1], [x0 + c, y1], [x0, y1 - c], [x0, y0 + c]]


class Scale:
    def __init__(self, den):
        self.den = den
        self.body = []
        self.thermal = []
        self.structure = []
        self.decisions = []
        self.feature = 0.00015 * den

    def b(self, s):
        self.body.append(s)
        return s

    def th(self, s, region, layer=None):
        s = dict(s)
        s["region"] = region
        if layer:
            s["layer"] = layer
        self.thermal.append(s)
        return s

    def thof(self, id, of, region, layer=None):
        d = {"id": id, "of": of, "region": region}
        if layer:
            d["layer"] = layer
        self.thermal.append(d)

    def st(self, s, member, part):
        s = dict(s)
        s["member"] = member
        s["part"] = part
        self.structure.append(s)
        return s

    def stof(self, id, of, member, part):
        self.structure.append({"id": id, "of": of, "member": member, "part": part})

    def dec(self, en, de):
        self.decisions.append({"en": en, "de": de})

    def out(self):
        return {"status": "authored", "denominator": self.den, "body": self.body, "thermal": self.thermal, "structure": self.structure, "decisions": self.decisions}


def mark(solids, faces):
    for d in solids:
        if "of" not in d:
            d["sectionFaces"] = faces
    return solids


CAT = {"objects": "🪑️objects"}


def anchors_of(slug, category="objects"):
    a = json.load(open("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/%s/%s/🔣️.json" % (CAT[category], slug)))["asset"]
    return a["structure"]["supports"]


def save(category, slug, scales):
    sup = anchors_of(slug, category)
    for sc_ in scales:
        for d in sc_.structure:
            if d.get("part") == "support":
                m = [x for x in sup if x in d["id"]]
                d["anchor"] = m[0] if m else sup[0]
    path = os.path.join(T, "volumes", category, slug + ".json")
    cur = json.load(open(path))
    v = cur["volumes"]
    for s in scales:
        v[str(s.den)] = s.out()
    json.dump(cur, open(path, "w"), ensure_ascii=False, indent=1)
    print("saved", path, {k: (len(x.get("body", [])), len(x.get("thermal", [])), len(x.get("structure", []))) for k, x in v.items() if x["status"] == "authored"})


def rebase(sc_, offsets):
    for lst in (sc_.body, sc_.thermal, sc_.structure):
        for d in lst:
            if "of" in d or d.get("bone") not in offsets:
                continue
            dx, dy = offsets[d["bone"]]
            if d["kind"] in ("box", "ellipsoid", "extrude", "loft", "capsule") and d["kind"] != "extrude" and d["kind"] != "loft":
                t = d.setdefault("transform", {}).setdefault("translation", [0, 0, 0])
                t[0] = r6(t[0] - dx)
                t[1] = r6(t[1] - dy)
            elif d["kind"] == "revolve":
                d["origin"] = [r6(d["origin"][0] - dx), r6(d["origin"][1] - dy), d["origin"][2]]
            elif d["kind"] == "extrude":
                sh = lambda ring: [[r6(p[0] - dx), r6(p[1] - dy)] for p in ring]
                d["profile"] = sh(d["profile"])
                if d.get("holes"):
                    d["holes"] = [sh(h) for h in d["holes"]]
            elif d["kind"] == "loft":
                for sct in d["sections"]:
                    sct["at"] = [r6(sct["at"][0] - dx), r6(sct["at"][1] - dy), sct["at"][2]]
