"""🪚 Chisel of rework author 05: constructors for hand-stated solids, frames and document io. It holds no number of any asset.

Solids are authored in the rest frame of their bone: u along the bone, v its normal in the sagittal plane, w lateral (left +).
A loft is stated as station rows `(u, v, w, rw, rv[, exponent])`: centre of the cross-section and its half extents along w and v.
@see ./rework-brief.md
@see ./r3-tools.md
"""
import json
import math
import os
import subprocess

T = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(T, "..", "..", "..", "..", "..", "..", ".."))
CATALOG = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog")
ALONG_U = [0, -0.70711, 0, 0.70711]


def r5(x):
    """🔢 Rounds to a tenth of a millimetre (lists recursively)."""
    return [r5(c) for c in x] if isinstance(x, (list, tuple)) else round(float(x), 5)


def spline(rows, step):
    """〰️ Catmull-Rom curve through the stated station rows, sampled about every `step` along u; every stated station is kept."""
    rows = [tuple(r) + ((2.0,) if len(r) == 5 else ()) for r in rows]
    out = []
    n = len(rows)
    for i in range(n - 1):
        p0, p1, p2, p3 = rows[max(i - 1, 0)], rows[i], rows[i + 1], rows[min(i + 2, n - 1)]
        k = max(1, int(math.ceil(abs(p2[0] - p1[0]) / step)))
        for j in range(k):
            t = j / k
            row = []
            for c in range(6):
                a, b, d, e = p0[c], p1[c], p2[c], p3[c]
                m1 = (d - a) / 2 if i > 0 else d - b
                m2 = (e - b) / 2 if i < n - 2 else d - b
                h = (2 * t**3 - 3 * t**2 + 1) * b + (t**3 - 2 * t**2 + t) * m1 + (-2 * t**3 + 3 * t**2) * d + (t**3 - t**2) * m2
                row.append(h)
            lo, hi = min(p1[0], p2[0]), max(p1[0], p2[0])
            row[0] = min(max(row[0], lo), hi)
            for c in (3, 4):
                row[c] = max(row[c], min(p1[c], p2[c]) * 0.9)
            out.append(tuple(row))
    out.append(rows[-1])
    return out


def dome(row, length, toward, n=4):
    """🥚 Rounded end for a loft: `n` shrinking sections over `length` beyond the station `row`, in direction `toward` (+1 or -1) along u, ordered away from the body."""
    u, v, w, rw, rv, e = tuple(row) + ((2.0,) if len(row) == 5 else ())
    out = []
    for k in range(1, n + 1):
        a = k / n * math.pi / 2 * 0.94
        out.append((u + toward * length * math.sin(a), v, w, rw * math.cos(a), rv * math.cos(a), e))
    return out


def loft(id, bone, material, tone, rows, role="body", step=0.02, round0=0.0, round1=0.0, smooth=True, rotation=None, **more):
    """🏺 Loft through station rows `(u, v, w, rw, rv[, exponent])`; `round0` / `round1` add a dome of that length before the first / after the last station; `smooth` splines between the stations."""
    rows = spline(rows, step) if smooth and len(rows) > 2 else [tuple(r) + ((2.0,) if len(r) == 5 else ()) for r in rows]
    if round0 > 0:
        rows = list(reversed(dome(rows[0], round0, -1 if rows[-1][0] > rows[0][0] else 1))) + rows
    if round1 > 0:
        rows = rows + dome(rows[-1], round1, 1 if rows[-1][0] > rows[0][0] else -1)
    sections = [{"at": r5([u, v, w]), "rx": r5(max(rw, 0.0004)), "ry": r5(max(rv, 0.0004)), "exponent": r5(e), "rotation": rotation or ALONG_U} for u, v, w, rw, rv, e in rows]
    return {"id": id, "kind": "loft", "bone": bone, "material": material, "role": role, "tone": tone, "sections": sections, "capStart": True, "capEnd": True, **more}


def capsule(id, bone, material, tone, p0, p1, r0, r1=None, role="body", **more):
    """💊 Capsule from `p0` to `p1` with end radii `r0`, `r1`."""
    return {"id": id, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": r5(p0), "p1": r5(p1), "r0": r5(r0), "r1": r5(r0 if r1 is None else r1), **more}


def ellipsoid(id, bone, material, tone, centre, radii, role="body", rotation=None, **more):
    """🥚 Ellipsoid with `radii` (u, v, w) at `centre`, optionally turned by a unit quaternion."""
    transform = {"translation": r5(centre)}
    if rotation is not None:
        transform["rotation"] = r5(rotation)
    return {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": role, "tone": tone, "radii": r5(radii), "transform": transform, **more}


def box(id, bone, material, tone, centre, size, corner=0.0, role="body", rotation=None, **more):
    """📦 Box of `size` (u, v, w) at `centre`."""
    transform = {"translation": r5(centre)}
    if rotation is not None:
        transform["rotation"] = r5(rotation)
    return {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": r5(size), "cornerRadius": r5(corner), "transform": transform, **more}


def extrude(id, bone, material, tone, profile, plane, start, end, role="body", **more):
    """🧇 Extrusion of a simple polygon: plane `xy` (u, v; along w), `yz` (v, w; along u) or `zx` (w, u; along v), from `start` to `end`."""
    return {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": r5(profile), "plane": plane, "from": r5(start), "to": r5(end), **more}


def revolve(id, bone, material, tone, profile, axis, origin, role="body", **more):
    """🏺 Revolution of an `[axial, radial]` profile about `axis` through `origin`."""
    return {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": r5(profile), "axis": r5(axis), "origin": r5(origin), **more}


def quat(axis, deg):
    """🧭 Unit quaternion `[x, y, z, w]` of a turn by `deg` about `axis` (u, v, w)."""
    n = math.sqrt(sum(c * c for c in axis))
    s = math.sin(math.radians(deg) / 2)
    return [axis[0] / n * s, axis[1] / n * s, axis[2] / n * s, math.cos(math.radians(deg) / 2)]


def qmul(a, b):
    """✖️ Product of two quaternions (apply `b` first, then `a`)."""
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def turned(solid, axis, deg, about=(0, 0, 0)):
    """🔄 The solid turned by `deg` about `axis` through the point `about` of its bone frame (through `transform`)."""
    q = quat(axis, deg)
    t = solid.get("transform", {})
    old_q = t.get("rotation", [0, 0, 0, 1])
    old_t = t.get("translation", [0, 0, 0])
    rel = [old_t[i] - about[i] for i in range(3)]
    rot = qrot(q, rel)
    out = dict(solid)
    out["transform"] = {**t, "rotation": r5(qmul(q, old_q)), "translation": r5([rot[i] + about[i] for i in range(3)])}
    return out


def qrot(q, p):
    """🌀 The point `p` turned by the quaternion `q`."""
    x, y, z, w = q
    px, py, pz = p
    tx, ty, tz = 2 * (y * pz - z * py), 2 * (z * px - x * pz), 2 * (x * py - y * px)
    return [px + w * tx + (y * tz - z * ty), py + w * ty + (z * tx - x * tz), pz + w * tz + (x * ty - y * tx)]


def mirrored(solid, old="-l", new="-r"):
    """🪞 The solid reflected in w for the other body side: ids and bones `…-l…` become `…-r…`."""
    def name(s):
        return s.replace(old + "-", new + "-") if (old + "-") in s else (s[: -len(old)] + new if s.endswith(old) else s)

    out = json.loads(json.dumps(solid))
    out["id"] = name(out["id"])
    out["bone"] = name(out["bone"])
    for key in ("member", "of", "anchor"):
        if key in out:
            out[key] = name(out[key])
    if "weights" in out:
        out["weights"] = [{**x, "bone": name(x["bone"])} for x in out["weights"]]
    kind = out.get("kind")
    if kind == "loft":
        for s in out["sections"]:
            s["at"][2] = -s["at"][2]
            q = s["rotation"]
            if q != ALONG_U:
                s["rotation"] = [-q[0], -q[1], q[2], q[3]]
    if kind == "capsule":
        out["p0"][2] = -out["p0"][2]
        out["p1"][2] = -out["p1"][2]
    if kind == "extrude":
        if out["plane"] == "xy":
            out["from"], out["to"] = -out["to"], -out["from"]
        elif out["plane"] == "yz":
            out["profile"] = [[p[0], -p[1]] for p in reversed(out["profile"])]
        else:
            out["profile"] = [[-p[0], p[1]] for p in reversed(out["profile"])]
    if kind == "revolve":
        out["origin"][2] = -out["origin"][2]
        out["axis"][2] = -out["axis"][2]
    t = out.get("transform")
    if t:
        if "translation" in t:
            t["translation"][2] = -t["translation"][2]
        if "rotation" in t:
            q = t["rotation"]
            t["rotation"] = [-q[0], -q[1], q[2], q[3]]
    return out


def both(solid):
    """👐 A left solid and its mirrored right twin."""
    return [solid, mirrored(solid)]


def shell(solid, id, inset, material=None, tone=None, **tags):
    """🧅 An inner shell of a loft, capsule or ellipsoid: the same form with every radius reduced by `inset` (never below 1 mm), for nested thermal layers."""
    out = json.loads(json.dumps({k: v for k, v in solid.items() if not k.startswith("_")}))
    out["id"] = id
    if material:
        out["material"] = material
    if tone is not None:
        out["tone"] = tone
    if out["kind"] == "loft":
        for s in out["sections"]:
            s["rx"] = r5(max(s["rx"] - inset, 0.001))
            s["ry"] = r5(max(s["ry"] - inset, 0.001))
        if len(out["sections"]) > 2:
            a, b = out["sections"][0], out["sections"][-1]
            d0 = [out["sections"][1]["at"][i] - a["at"][i] for i in range(3)]
            d1 = [out["sections"][-2]["at"][i] - b["at"][i] for i in range(3)]
            for s, d in ((a, d0), (b, d1)):
                n = math.sqrt(sum(c * c for c in d)) or 1
                s["at"] = r5([s["at"][i] + d[i] / n * min(inset, n * 0.9) for i in range(3)])
    elif out["kind"] == "capsule":
        out["r0"] = r5(max(out["r0"] - inset, 0.001))
        out["r1"] = r5(max(out["r1"] - inset, 0.001))
    elif out["kind"] == "ellipsoid":
        out["radii"] = r5([max(r - inset, 0.001) for r in out["radii"]])
    out.update(tags)
    return out


def ref(id, of, **tags):
    """🔗 A thermal or structural reference to a body solid."""
    return {"id": id, "of": of, **tags}


def loft_area(solid):
    """📏 Lateral area of a loft from its sections (Ramanujan perimeter times slant length), in square metres."""
    area = 0.0
    s = solid["sections"]

    def per(x):
        a, b = x["rx"], x["ry"]
        return math.pi * (3 * (a + b) - math.sqrt((3 * a + b) * (a + 3 * b)))

    for a, b in zip(s, s[1:]):
        length = math.dist(a["at"], b["at"])
        area += (per(a) + per(b)) / 2 * math.hypot(length, ((a["rx"] + a["ry"]) - (b["rx"] + b["ry"])) / 2)
    return area


def area_of(solid):
    """📐 Outer area of a loft, capsule, ellipsoid or box (analytic approximations), in square metres."""
    k = solid.get("kind")
    if k == "loft":
        return loft_area(solid)
    if k == "capsule":
        length = math.dist(solid["p0"], solid["p1"])
        r = (solid["r0"] + solid["r1"]) / 2
        return 2 * math.pi * r * length + 4 * math.pi * r * r
    if k == "ellipsoid":
        a, b, c = solid["radii"]
        p = 1.6075
        return 4 * math.pi * (((a * b) ** p + (a * c) ** p + (b * c) ** p) / 3) ** (1 / p)
    if k == "box":
        a, b, c = solid["size"]
        return 2 * (a * b + a * c + b * c)
    return 0.0


def to_world(frame, p):
    """🌍 A bone-local point (u, v, w) in the world of the frame `{o, u, v, w}`."""
    return [frame["o"][i] + p[0] * frame["u"][i] + p[1] * frame["v"][i] + p[2] * frame["w"][i] for i in range(3)]


def to_local(frame, p):
    """🦴 A world point in the bone-local frame `{o, u, v, w}`."""
    d = [p[i] - frame["o"][i] for i in range(3)]
    return [sum(d[i] * frame[a][i] for i in range(3)) for a in ("u", "v", "w")]


def frames_of(document_path, queries=()):
    """🦴 World frames `{query: {bone: {o, u, v, w, tip}}}` of a candidate document for poses (`id`) or clip instants (`clip@seconds`), solved by the product's own `evaluate`."""
    bun = sorted(__import__("glob").glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
    out = subprocess.run([bun, os.path.join(T, "rework-05-load.ts"), document_path, *queries], capture_output=True, text=True, cwd=T)
    if out.returncode != 0:
        raise RuntimeError(out.stderr[-2000:])
    return json.loads(out.stdout)


def catalog_path(category_folder, slug):
    """📁 Path of a catalog asset document, e.g. `catalog_path("👤️humans", "pregnant-person")`."""
    return os.path.join(CATALOG, category_folder, slug, "🔣️.json")


def read_json(path):
    """📥 Reads a JSON file."""
    with open(path, encoding="utf8") as f:
        return json.load(f)


def write_json(path, value):
    """📤 Writes a JSON file compactly (the catalog convention) after creating its folder."""
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf8") as f:
        f.write(json.dumps(value, ensure_ascii=False) + "\n")


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def outline_uv(solid):
    """✂️ The outline of a loft (sections along u), capsule or ellipsoid in the (u, v) plane of its bone and its span along w: `(points, w_min, w_max)`."""
    k = solid["kind"]
    t = solid.get("transform", {}).get("translation", [0, 0, 0])
    if k == "loft":
        s = solid["sections"]
        top = [[x["at"][0] + t[0], x["at"][1] + x["ry"] + t[1]] for x in s]
        bottom = [[x["at"][0] + t[0], x["at"][1] - x["ry"] + t[1]] for x in reversed(s)]
        return r5(top + bottom), min(x["at"][2] - x["rx"] for x in s) + t[2], max(x["at"][2] + x["rx"] for x in s) + t[2]
    if k == "capsule":
        pts = []
        for p, r in ((solid["p0"], solid["r0"]), (solid["p1"], solid["r1"])):
            pts += [(p[0] + t[0] + r * math.cos(a * math.pi / 8), p[1] + t[1] + r * math.sin(a * math.pi / 8)) for a in range(16)]
        return r5(hull(pts)), min(solid["p0"][2] - solid["r0"], solid["p1"][2] - solid["r1"]) + t[2], max(solid["p0"][2] + solid["r0"], solid["p1"][2] + solid["r1"]) + t[2]
    if k == "ellipsoid":
        a, b, c = solid["radii"]
        return r5([[t[0] + a * math.cos(i * math.pi / 8), t[1] + b * math.sin(i * math.pi / 8)] for i in range(16)]), t[2] - c, t[2] + c
    if k == "box":
        a, b, c = [x / 2 for x in solid["size"]]
        return r5([[t[0] - a, t[1] - b], [t[0] + a, t[1] - b], [t[0] + a, t[1] + b], [t[0] - a, t[1] + b]]), t[2] - c, t[2] + c
    raise ValueError("no outline for kind %s" % k)


def hull(points):
    """🪢 Convex hull of 2D points (Andrew's monotone chain), counter-clockwise."""
    pts = sorted(set((round(x, 6), round(y, 6)) for x, y in points))
    def half(seq):
        h = []
        for p in seq:
            while len(h) >= 2 and (h[-1][0] - h[-2][0]) * (p[1] - h[-2][1]) - (h[-1][1] - h[-2][1]) * (p[0] - h[-2][0]) <= 0:
                h.pop()
            h.append(p)
        return h[:-1]
    return [list(p) for p in half(pts) + half(list(reversed(pts)))]


def solid_shape(solid, bone_z, fields):
    """🧊 The 2D shape of a body solid on a bone that twists or spreads: a half profile revolved about the bone axis when the solid is round and on the axis, else its (u, v) outline extruded over its lateral span; `bone_z` is the rest lateral position of the bone, `fields` the shape members (id, bone, role, z, tone, region, material)."""
    if solid["kind"] == "loft" and "transform" not in solid:
        s = solid["sections"]
        if all(abs(x["at"][1]) <= 0.006 and abs(x["at"][2]) <= 0.006 and abs(x["rx"] - x["ry"]) <= 0.16 * max(x["rx"], x["ry"]) for x in s):
            pts = [[s[0]["at"][0], 0]] + [[x["at"][0], (x["rx"] + x["ry"]) / 2] for x in s] + [[s[-1]["at"][0], 0]]
            return {**fields, "filled": True, "kind": "poly", "points": r5(pts), "closed": True, "solid": {"kind": "revolved", "axisBone": solid["bone"], "axis": "u", "axisX": 0, "mirror": True}}
    pts, w0, w1 = outline_uv(solid)
    return {**fields, "filled": True, "kind": "poly", "points": pts, "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": r5(bone_z + w0), "z1": r5(bone_z + w1)}}


def reach(solid, frame, d):
    """🔭 How far a solid reaches along the world direction `d` when its bone is at `frame` (support of its sections, ends or radii; a turned ellipsoid or box is taken by its largest radius)."""
    def at(p, ru=0.0, rv=0.0, rw=0.0):
        c = to_world(frame, p)
        du, dv, dw = (sum(frame[a][i] * d[i] for i in range(3)) for a in ("u", "v", "w"))
        return sum(c[i] * d[i] for i in range(3)) + math.sqrt((ru * du) ** 2 + (rv * dv) ** 2 + (rw * dw) ** 2)
    t = solid.get("transform", {}).get("translation", [0, 0, 0])
    k = solid["kind"]
    if k == "loft":
        return max(at([x["at"][i] + t[i] for i in range(3)], 0, x["ry"], x["rx"]) for x in solid["sections"])
    if k == "capsule":
        return max(at([solid["p0"][i] + t[i] for i in range(3)], solid["r0"], solid["r0"], solid["r0"]), at([solid["p1"][i] + t[i] for i in range(3)], solid["r1"], solid["r1"], solid["r1"]))
    if k == "ellipsoid":
        r = solid["radii"] if "rotation" not in solid.get("transform", {}) else [max(solid["radii"])] * 3
        return at(t, *r)
    if k == "box":
        r = [x / 2 for x in solid["size"]]
        return at(t, *r)
    if k == "extrude":
        ax = {"xy": (0, 1, 2), "yz": (1, 2, 0), "zx": (2, 0, 1)}[solid["plane"]]
        best = -1e9
        for a, b in solid["profile"]:
            for c in (solid["from"], solid["to"]):
                p = [0, 0, 0]
                p[ax[0]], p[ax[1]], p[ax[2]] = a, b, c
                best = max(best, at([p[i] + t[i] for i in range(3)]))
        return best
    return at(t)


def publish(candidate_path, catalog_path_, source_path):
    """🚀 Writes a validated candidate document to the catalog file in the catalog's own formatting (the `dump` of `volumes-insert.py`) and its volumes to the volumes source; the catalog file is re-read immediately before the write and must hold the same asset id."""
    with open(os.path.join(T, "volumes-insert.py"), encoding="utf8") as f:
        source = f.read()
    scope = {}
    exec(source[: source.index("\ndef main():")], scope)
    doc = read_json(candidate_path)
    now = read_json(catalog_path_)
    if now["asset"]["id"] != doc["asset"]["id"]:
        raise SystemExit("candidate and catalog hold different assets")
    write_json(source_path, {"volumes": doc["asset"]["volumes"]})
    with open(catalog_path_, "w", encoding="utf8") as f:
        f.write(scope["dump"](doc, 0) + "\n")
