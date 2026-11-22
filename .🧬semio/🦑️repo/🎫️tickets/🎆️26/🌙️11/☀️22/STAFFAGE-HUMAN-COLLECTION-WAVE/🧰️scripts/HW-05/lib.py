"""🧬️ HW-05 derivation library: rest frames of a staffage rig and the per-bone similarity that rescales a whole human asset.

A bone b gets one factor f_b. Everything authored in the bone's local frame (elevation shapes, solid profiles, volume solids, anchors,
centres of mass, bone length) is multiplied by f_b; everything authored in world rest coordinates (plain front and plan shapes, the
lateral spans of `extruded` sweep `w` solids) is moved with the bone origin and scaled about it by f_b; a child bone's placement in its
parent frame is multiplied by the parent's factor. Every bone that carries geometry rests sagittal (w = world z), so one isotropic
factor per bone keeps the side elevation, the front elevation, the plan and the volume one body.

@see ../../📋️brief.md
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import copy
import json
import math
from pathlib import Path


REPO = Path("/Users/ueli/Documents/semio")
STAFFAGE = REPO / "🎓️teaching/🏛️architecture/🧍️staffage"
HUMANS = STAFFAGE / "🗂️catalog/👤️humans"
TICKET = REPO / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
SCRIPTS = TICKET / "🧰️scripts/HW-05"
GENERATED = TICKET / "🗑️generated/HW-05"
BUN = "/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
SCALES = ["500", "200", "100", "50"]
VIEWS = ["elevation", "front", "plan"]


def T(en, de):
    return {"en": en, "de": de}


def load(slug):
    return json.loads((HUMANS / slug / "🔣️.json").read_text())


def _compact(o, level=0):
    pad = "  " * level
    if isinstance(o, dict):
        if not o:
            return "{}"
        items = [f'{pad}  {json.dumps(k, ensure_ascii=False)}: {_compact(v, level + 1)}' for k, v in o.items()]
        return "{\n" + ",\n".join(items) + "\n" + pad + "}"
    if isinstance(o, list):
        if not o:
            return "[]"
        if all(not isinstance(v, (dict, list)) for v in o) or all(isinstance(v, list) and all(not isinstance(x, (dict, list)) for x in v) for v in o):
            return json.dumps(o, ensure_ascii=False, separators=(", ", ": "))
        items = [f"{pad}  {_compact(v, level + 1)}" for v in o]
        return "[\n" + ",\n".join(items) + "\n" + pad + "]"
    return json.dumps(o, ensure_ascii=False)


def save(doc, slug):
    folder = HUMANS / slug
    folder.mkdir(parents=True, exist_ok=True)
    (folder / "🔣️.json").write_text(_compact(doc) + "\n")


def r(x, n=5):
    return float(round(x, n))


def mm(A, B):
    return [[sum(A[i][k] * B[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def mv(A, x):
    return [sum(A[i][k] * x[k] for k in range(3)) for i in range(3)]


def add(a, b):
    return [a[i] + b[i] for i in range(3)]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def col(R, j):
    return [R[0][j], R[1][j], R[2][j]]


def _rw(a):
    c, s = math.cos(a), math.sin(a)
    return [[c, -s, 0], [s, c, 0], [0, 0, 1.0]]


def _rv(a):
    c, s = math.cos(a), math.sin(a)
    return [[c, 0, -s], [0, 1.0, 0], [s, 0, c]]


def _ru(a):
    c, s = math.cos(a), math.sin(a)
    return [[1.0, 0, 0], [0, c, s], [0, -s, c]]


def rest_rotation(bone, offset=None):
    o = offset or {}
    rad = math.radians
    rest = mm(mm(_rw(rad(bone.get("angleDeg", 0))), _rv(rad(bone.get("abductionDeg", 0)))), _ru(rad(bone.get("twistDeg", 0))))
    off = mm(mm(_rw(rad(o.get("angleDeg", 0))), _rv(rad(o.get("abductionDeg", 0)))), _ru(rad(o.get("twistDeg", 0))))
    return mm(rest, off)


def frames(bones, offsets=None, root=(0.0, 0.0)):
    """Rest (or posed, with `offsets`) frames: id -> (origin, R with the columns u, v, w) in world metres."""
    offsets = offsets or {}
    by = {b["id"]: b for b in bones}
    out = {}

    def solve(bid):
        if bid in out:
            return out[bid]
        b = by[bid]
        off = offsets.get(bid, {})
        local = [b.get("x", 0) + off.get("dx", 0), b.get("y", 0) + off.get("dy", 0), b.get("z", 0)]
        if b["parent"] is None:
            o = add(local, [root[0], root[1], 0])
            R = rest_rotation(b, off)
        else:
            po, pR = solve(b["parent"])
            o = add(po, mv(pR, local))
            R = mm(pR, rest_rotation(b, off))
        out[bid] = (o, R)
        return out[bid]

    for b in bones:
        solve(b["id"])
    return out


def scale_list(v, f):
    return [r(x * f) for x in v]


def _world(p, o_old, o_new, f, axes):
    return [r(o_new[a] + f * (p[i] - o_old[a])) for i, a in enumerate(axes)]


def _scale_solid(s, f):
    """A volume solid in its bone-local frame, scaled by f about the bone origin."""
    s = copy.deepcopy(s)
    k = s["kind"]
    if k == "box":
        s["size"] = scale_list(s["size"], f)
        if "cornerRadius" in s:
            s["cornerRadius"] = r(s["cornerRadius"] * f)
    elif k == "ellipsoid":
        s["radii"] = scale_list(s["radii"], f)
    elif k == "capsule":
        s["p0"], s["p1"] = scale_list(s["p0"], f), scale_list(s["p1"], f)
        s["r0"], s["r1"] = r(s["r0"] * f), r(s["r1"] * f)
    elif k == "revolve":
        s["profile"] = [scale_list(p, f) for p in s["profile"]]
        s["origin"] = scale_list(s["origin"], f)
    elif k == "extrude":
        s["profile"] = [scale_list(p, f) for p in s["profile"]]
        if "holes" in s:
            s["holes"] = [[scale_list(p, f) for p in h] for h in s["holes"]]
        s["from"], s["to"] = r(s["from"] * f), r(s["to"] * f)
    elif k == "loft":
        for sec in s["sections"]:
            sec["at"] = scale_list(sec["at"], f)
            sec["rx"], sec["ry"] = r(sec["rx"] * f), r(sec["ry"] * f)
    elif k == "polyhedron":
        s["vertices"] = [scale_list(p, f) for p in s["vertices"]]
    else:
        raise ValueError(k)
    if "transform" in s and "translation" in s["transform"]:
        s["transform"]["translation"] = scale_list(s["transform"]["translation"], f)
    if "repeat" in s:
        for key in ("translation", "origin"):
            if key in s["repeat"]:
                s["repeat"][key] = scale_list(s["repeat"][key], f)
    return s


def scale_shape(shape, view, f, o_old, o_new, fa=None):
    """A drawn shape of one viewpoint on a bone with factor f (rest origins o_old, o_new); `fa` the factor of a revolve's axis bone."""
    s = copy.deepcopy(shape)
    if s["kind"] != "poly":
        raise ValueError(f"{s['id']}: kind {s['kind']}")
    solid = s.get("solid")
    if solid is None:
        if view == "elevation":
            s["points"] = [scale_list(p, f) for p in s["points"]]
        elif view == "front":
            s["points"] = [_world(p, o_old, o_new, f, (2, 1)) for p in s["points"]]
        else:
            s["points"] = [_world(p, o_old, o_new, f, (0, 2)) for p in s["points"]]
        return s
    s["points"] = [scale_list(p, f) for p in s["points"]]
    if solid["kind"] == "revolved":
        g = fa if fa is not None else f
        solid["axisX"] = r(solid["axisX"] * g)
        if "axisZ" in solid:
            solid["axisZ"] = r(solid["axisZ"] * g)
    elif solid["sweep"] == "w":
        solid["z0"] = r(o_new[2] + f * (solid["z0"] - o_old[2]))
        solid["z1"] = r(o_new[2] + f * (solid["z1"] - o_old[2]))
        if "zTip" in solid:
            solid["zTip"] = [r(o_new[2] + f * (z - o_old[2])) for z in solid["zTip"]]
    else:
        solid["z0"], solid["z1"] = r(solid["z0"] * f), r(solid["z1"] * f)
        if "zTip" in solid:
            solid["zTip"] = scale_list(solid["zTip"], f)
    return s


def rescale(doc, factor, ground_bone="toe-l", motion_factor=None):
    """The asset of `doc` with every bone b scaled by factor(b) (a dict or a function); returns a new document."""
    doc = copy.deepcopy(doc)
    a = doc["asset"]
    bones = a["rig"]["bones"]
    f = factor if callable(factor) else (lambda b: factor.get(b, 1.0))
    parent = {b["id"]: b["parent"] for b in bones}
    old = frames(bones)
    ground_old = old[ground_bone][0][1]
    for b in bones:
        fp = f(b["parent"]) if b["parent"] else f(b["id"])
        b["x"], b["y"] = r(b.get("x", 0) * fp), r(b.get("y", 0) * fp)
        if "z" in b:
            b["z"] = r(b["z"] * fp)
        b["lengthM"] = r(b["lengthM"] * f(b["id"]))
        if "mass" in b:
            b["mass"]["comX"] = r(b["mass"]["comX"] * f(b["id"]))
            b["mass"]["comY"] = r(b["mass"]["comY"] * f(b["id"]))
    root = next(b for b in bones if b["parent"] is None)
    trial = frames(bones)
    root["y"] = r(root["y"] + f(ground_bone) * ground_old - trial[ground_bone][0][1], 6)
    new = frames(bones)
    for an in a["anchors"]:
        g = f(an["bone"])
        for key in ("x", "y", "z", "footprintLengthM", "footprintWidthM"):
            if key in an:
                an[key] = r(an[key] * g)
    for rep in a["representations"].values():
        if rep.get("status") != "authored":
            continue
        for view in VIEWS:
            rep["geometry"][view] = [
                scale_shape(s, view, f(s["bone"]), old[s["bone"]][0], new[s["bone"]][0], f(s["solid"]["axisBone"]) if s.get("solid", {}).get("kind") == "revolved" else None)
                for s in rep["geometry"].get(view, [])
            ]
    for vol in a["volumes"].values():
        if vol.get("status") != "authored":
            continue
        for lst in ("body", "thermal", "structure"):
            vol[lst] = [s if "of" in s else _scale_solid(s, f(s["bone"])) for s in vol[lst]]
    mf = motion_factor if motion_factor is not None else f(ground_bone)

    def offsets(bmap):
        for bid, off in bmap.items():
            g = f(parent[bid]) if parent.get(bid) else f(bid)
            for key in ("dx", "dy"):
                if key in off:
                    off[key] = r(off[key] * g)

    for p in a["poses"]:
        offsets(p.get("bones", {}))
        if "root" in p:
            for key in ("x", "y"):
                if key in p["root"]:
                    p["root"][key] = r(p["root"][key] * mf)
    for c in a["clips"]:
        for k in c.get("keys", []):
            offsets(k.get("bones", {}))
            if "root" in k:
                for key in ("x", "y"):
                    if key in k["root"]:
                        k["root"][key] = r(k["root"][key] * mf)
        if "advanceM" in c:
            c["advanceM"] = r(c["advanceM"] * mf, 6)
    return doc


def save_path(doc, path):
    Path(path).write_text(_compact(doc) + "\n")


def qrot(q, v):
    x, y, z, w = q
    ux, uy, uz = v
    tx, ty, tz = 2 * (y * uz - z * uy), 2 * (z * ux - x * uz), 2 * (x * uy - y * ux)
    return [ux + w * tx + (y * tz - z * ty), uy + w * ty + (z * tx - x * tz), uz + w * tz + (x * ty - y * tx)]


def _axis_map(R):
    """For an axis-aligned rest frame: world axis index -> (local axis index, sign)."""
    out = {}
    for j in range(3):
        c = col(R, j)
        i = max(range(3), key=lambda k: abs(c[k]))
        if abs(abs(c[i]) - 1) > 1e-6:
            raise ValueError("bone frame is not axis-aligned")
        out[i] = (j, 1 if c[i] > 0 else -1)
    return out


def affine_bone(doc, bone, c, D, select=lambda item: True, scales=None):
    """Scales the geometry of one axis-aligned bone by the diagonal D = (du, dv, dw) about the bone-local point c, in every scale, view and list."""
    a = doc["asset"]
    o, R = frames(a["rig"]["bones"])[bone]
    amap = _axis_map(R)
    m = lambda p: [c[i] + D[i] * (p[i] - c[i]) for i in range(3)]

    def world(p, axes):
        out = []
        for value, wa in zip(p, axes):
            j, sgn = amap[wa]
            loc = sgn * (value - o[wa])
            out.append(r(o[wa] + sgn * (c[j] + D[j] * (loc - c[j]))))
        return out

    for key, rep in a["representations"].items():
        if rep.get("status") != "authored" or (scales and key not in scales):
            continue
        for view in VIEWS:
            for s in rep["geometry"][view]:
                if s["bone"] != bone or not select(s):
                    continue
                sol = s.get("solid")
                if sol is None:
                    if view == "elevation":
                        s["points"] = [[r(c[0] + D[0] * (u - c[0])), r(c[1] + D[1] * (v - c[1]))] for u, v in s["points"]]
                    else:
                        s["points"] = [world(p, (2, 1) if view == "front" else (0, 2)) for p in s["points"]]
                elif sol["kind"] == "extruded" and sol["sweep"] == "v":
                    s["points"] = [[r(c[0] + D[0] * (u - c[0])), r(c[2] + D[2] * (w - c[2]))] for u, w in s["points"]]
                    sol["z0"], sol["z1"] = r(c[1] + D[1] * (sol["z0"] - c[1])), r(c[1] + D[1] * (sol["z1"] - c[1]))
                    if "zTip" in sol:
                        sol["zTip"] = [r(c[1] + D[1] * (z - c[1])) for z in sol["zTip"]]
                elif sol["kind"] == "extruded" and sol["sweep"] == "w":
                    s["points"] = [[r(c[0] + D[0] * (u - c[0])), r(c[1] + D[1] * (v - c[1]))] for u, v in s["points"]]
                    for key in ("z0", "z1"):
                        sol[key] = r(o[2] + c[2] + D[2] * (sol[key] - o[2] - c[2]))
                    if "zTip" in sol:
                        sol["zTip"] = [r(o[2] + c[2] + D[2] * (z - o[2] - c[2])) for z in sol["zTip"]]
                else:
                    raise ValueError(f"{s['id']}: {sol}")
    for key, vol in a["volumes"].items():
        if vol.get("status") != "authored" or (scales and key not in scales):
            continue
        for lst in ("body", "thermal", "structure"):
            for s in vol[lst]:
                if "of" in s or s["bone"] != bone or not select(s):
                    continue
                tr = s.get("transform", {})
                if "rotation" in tr and any(abs(abs(x) - 1) > 1e-6 and abs(x) > 1e-6 for x in qrot(tr["rotation"], [1, 0, 0]) + qrot(tr["rotation"], [0, 1, 0])):
                    raise ValueError(f"{s['id']}: rotated solid")
                if "translation" in tr:
                    t0 = tr["translation"]
                    tr["translation"] = [r(x) for x in m(t0)]
                    pre = [0.0, 0.0, 0.0]
                else:
                    pre = None
                Dl = D if "rotation" not in tr else [abs(sum(D[i] * qrot(tr["rotation"], [1 if k == j else 0 for k in range(3)])[i] ** 2 for i in range(3))) for j in range(3)]
                lm = (lambda p: [Dl[i] * p[i] for i in range(3)]) if pre is not None else m
                k = s["kind"]
                if k == "loft":
                    for sec in s["sections"]:
                        sec["at"] = [r(x) for x in lm(sec["at"])]
                        q = sec.get("rotation", [0, 0, 0, 1])
                        for key, ax in (("rx", [1, 0, 0]), ("ry", [0, 1, 0])):
                            v = qrot(q, ax)
                            sec[key] = r(sec[key] * math.sqrt(sum((Dl[i] * v[i]) ** 2 for i in range(3))))
                elif k == "box":
                    s["size"] = [r(s["size"][i] * Dl[i]) for i in range(3)]
                elif k == "ellipsoid":
                    s["radii"] = [r(s["radii"][i] * Dl[i]) for i in range(3)]
                elif k == "capsule":
                    s["p0"], s["p1"] = [r(x) for x in lm(s["p0"])], [r(x) for x in lm(s["p1"])]
                    ax = [abs(s["p1"][i] - s["p0"][i]) for i in range(3)]
                    main = max(range(3), key=lambda i: ax[i])
                    g = math.sqrt(math.prod(Dl[i] for i in range(3) if i != main))
                    s["r0"], s["r1"] = r(s["r0"] * g), r(s["r1"] * g)
                else:
                    raise ValueError(f"{s['id']}: {k}")
    return doc
