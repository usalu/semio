"""🛴 Hand rework of `humans/child-on-kick-scooter`: shoulder and pelvic girdle on lateral rest placement, skeleton connected at its joints, coarse scales modelled for their feature.

Usage: python3 rework-child-on-kick-scooter-build.py [--publish]
Reads the pristine copies under 🗑️generated/rework-child-on-kick-scooter/orig (taken before the first edit), writes the candidate document to 🗑️generated/rework-child-on-kick-scooter/cand/🔣️.json and, with --publish, the source `volumes/humans/child-on-kick-scooter.json` and the changed members of the catalog file (re-read immediately before the write).
Rest placement of a bone: `rig-lateral-rest.md` section 1.
"""
import copy
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(T, "🗑️generated", "rework-child-on-kick-scooter")
SOURCE = os.path.join(T, "volumes", "humans", "child-on-kick-scooter.json")
CATALOG = os.path.normpath(os.path.join(T, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "child-on-kick-scooter", "🔣️.json"))


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth):
    """🧵 The stored formatting of the catalog and the volume sources (same as `volumes-insert.py`)."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def member_end(text, start):
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
    raise SystemExit("unbalanced asset file")


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def tr(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def app(a, p):
    return [sum(a[i][k] * p[k] for k in range(3)) for i in range(3)]


def local(bone):
    """🦴 Rest rotation of a bone in its parent frame, columns u, v, w: Rw(angle) · Rv(abduction) · Ru(twist)."""
    a, b, t = (math.radians(bone.get(key, 0) or 0) for key in ("angleDeg", "abductionDeg", "twistDeg"))
    ca, sa, cb, sb, ct, st = math.cos(a), math.sin(a), math.cos(b), math.sin(b), math.cos(t), math.sin(t)
    u, v1, w1 = [ca * cb, sa * cb, sb], [-sa, ca, 0], [-sb * ca, -sb * sa, cb]
    v = [ct * v1[i] - st * w1[i] for i in range(3)]
    w = [st * v1[i] + ct * w1[i] for i in range(3)]
    return [[u[i], v[i], w[i]] for i in range(3)]


def frames(bones):
    """🧭 World rest frame (origin, matrix with columns u, v, w) of every bone; world axes ahead, up, left."""
    out = {}
    for bone in bones:
        po, pf = out[bone["parent"]] if bone["parent"] else ([0, 0, 0], [[1, 0, 0], [0, 1, 0], [0, 0, 1]])
        offset = app(pf, [bone["x"], bone["y"], bone.get("z", 0) or 0])
        out[bone["id"]] = ([po[i] + offset[i] for i in range(3)], mul(pf, local(bone)))
    return out


def placed(parent, origin, frame):
    """📐 Rest fields that put a bone with the world frame `frame` at `origin` under `parent`."""
    po, pf = parent
    x, y, z = app(tr(pf), [origin[i] - po[i] for i in range(3)])
    m = mul(tr(pf), frame)
    u, v = [m[i][0] for i in range(3)], [m[i][1] for i in range(3)]
    b = math.asin(max(-1, min(1, u[2])))
    a = math.atan2(u[1], u[0])
    v1, w1 = [-math.sin(a), math.cos(a), 0], [-math.sin(b) * math.cos(a), -math.sin(b) * math.sin(a), math.cos(b)]
    t = math.atan2(-sum(v[i] * w1[i] for i in range(3)), sum(v[i] * v1[i] for i in range(3)))
    fields = {"x": round(x, 5), "y": round(y, 5), "z": round(z, 5), "angleDeg": round(math.degrees(a), 4), "abductionDeg": round(math.degrees(b), 4), "twistDeg": round(math.degrees(t), 4)}
    return {key: value for key, value in fields.items() if key in ("x", "y", "angleDeg") or abs(value) > 1e-6}


def unit(p):
    n = math.sqrt(sum(c * c for c in p))
    return [c / n for c in p]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def frame_of(u, w_hint):
    u = unit(u)
    d = sum(u[i] * w_hint[i] for i in range(3))
    w = unit([w_hint[i] - d * u[i] for i in range(3)])
    v = cross(w, u)
    return [[u[i], v[i], w[i]] for i in range(3)]


def quaternion(m):
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2
        return [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    if m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        return [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    if m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        return [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
    return [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]


def product(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def r5(p):
    return [round(c, 5) for c in p]


def r7(p):
    return [round(c, 7) for c in p]


class Move:
    """🔁 The same world solid written in the frame of another bone."""

    def __init__(self, old, new):
        (self.oo, self.of), (self.no, self.nf) = old, new
        self.turn = mul(tr(self.nf), self.of)
        self.q = quaternion(self.turn)

    def point(self, p):
        world = app(self.of, p)
        return r5(app(tr(self.nf), [world[i] + self.oo[i] - self.no[i] for i in range(3)]))

    def solid(self, solid, bone):
        kind = solid["kind"]
        if kind == "capsule":
            solid["p0"], solid["p1"] = self.point(solid["p0"]), self.point(solid["p1"])
        elif kind == "loft":
            for section in solid["sections"]:
                section["at"] = self.point(section["at"])
                section["rotation"] = r7(product(self.q, section.get("rotation", [0, 0, 0, 1])))
        elif kind == "ellipsoid":
            transform = solid.setdefault("transform", {})
            transform["translation"] = self.point(transform.get("translation", [0, 0, 0]))
            transform["rotation"] = r7(product(self.q, transform.get("rotation", [0, 0, 0, 1])))
        else:
            raise SystemExit(f"solid {solid['id']} of kind {kind} is not converted")
        solid["bone"] = bone
        if "member" in solid:
            solid["member"] = bone


GIRDLE = {
    "clavicle": {"parent": "torso", "from": [0.316, -0.048, 0.014], "to": [0.322, 0.004, 0.122], "child": "arm", "title": {"en": "{S} clavicle (shoulder girdle)", "de": "{S} Schlüsselbein (Schultergürtel)"}, "joint": {"type": "revolute", "minDeg": -8, "maxDeg": 30}, "solids": ["clavicle", "scapula"]},
    "ilium": {"parent": "pelvis", "from": [0.058, 0.024, 0.03], "to": [0, 0, 0.065], "child": "thigh", "title": {"en": "{S} hip bone (pelvic girdle)", "de": "{S} Hüftbein (Beckengürtel)"}, "joint": {"type": "revolute", "minDeg": -4, "maxDeg": 4}, "solids": ["ilium", "pubis", "ischium", "ischiopubic-ramus"]},
}
SIDES = {"l": (1, {"en": "Left", "de": "Linkes"}), "r": (-1, {"en": "Right", "de": "Rechtes"})}


def rig(asset):
    """🦴 Clavicles and hip bones run from the axial skeleton out to the shoulder and hip joints; arms and thighs hang from their tips without moving."""
    bones = asset["rig"]["bones"]
    old = frames(bones)
    for name, spec in GIRDLE.items():
        for side, (sign, word) in SIDES.items():
            po, pf = old[spec["parent"]]
            a, b = ([spec[key][0], spec[key][1], sign * spec[key][2]] for key in ("from", "to"))
            wa, wb = ([po[i] + app(pf, p)[i] for i in range(3)] for p in (a, b))
            u = [wb[i] - wa[i] for i in range(3)]
            frame = frame_of(u, [-1, 0, 0])
            bone = {"id": f"{name}-{side}", "parent": spec["parent"], "title": {lang: text.replace("{S}", word[lang]) for lang, text in spec["title"].items()}}
            bone.update(placed(old[spec["parent"]], wa, frame))
            bone["lengthM"] = round(math.sqrt(sum(c * c for c in u)), 5)
            bone["joint"] = dict(spec["joint"])
            child = next(item for item in bones if item["id"] == f"{spec['child']}-{side}")
            bones.insert(bones.index(child), bone)
            now = frames(bones)
            child["parent"] = bone["id"]
            for key in ("z", "abductionDeg", "twistDeg"):
                child.pop(key, None)
            fields = placed(now[bone["id"]], old[child["id"]][0], old[child["id"]][1])
            ordered = {}
            for key, value in child.items():
                ordered[key] = fields.get(key, value) if key in ("x", "y", "angleDeg") else value
                if key == "y" and "z" in fields:
                    ordered["z"] = fields["z"]
                if key == "angleDeg":
                    for extra in ("abductionDeg", "twistDeg"):
                        if extra in fields:
                            ordered[extra] = fields[extra]
            child.clear()
            child.update(ordered)
    new = frames(bones)
    for bone_id, (origin, frame) in old.items():
        drift = max(abs(new[bone_id][0][i] - origin[i]) for i in range(3))
        turned = max(abs(new[bone_id][1][i][j] - frame[i][j]) for i in range(3) for j in range(3))
        if drift > 2e-5 or turned > 2e-5:
            raise SystemExit(f"{bone_id} moved by {drift} m / {turned}")
    return old, new


def by_id(items):
    return {item["id"]: item for item in items}


def drop(volume, ids):
    """✂️ Removes body solids and every thermal or structure entry that depicts them."""
    ids = set(ids)
    volume["body"] = [s for s in volume["body"] if s["id"] not in ids]
    for name in ("thermal", "structure"):
        volume[name] = [s for s in volume[name] if s.get("of") not in ids and s["id"] not in ids]


def capsule(solid_id, bone, p0, p1, r, **extra):
    return {"id": solid_id, "kind": "capsule", "bone": bone, "material": "bone", "role": "body", "tone": 0.92, "p0": p0, "p1": p1, "r0": r, "r1": r, "member": bone, "part": "member", **extra}


def ball(solid_id, bone, at, radii, **extra):
    return {"id": solid_id, "kind": "ellipsoid", "bone": bone, "material": "bone", "role": "body", "tone": 0.92, "radii": radii, "transform": {"translation": at}, "member": bone, "part": "member", **extra}


def disc(half, radius, bevel):
    return [[-half, 0], [-half, radius - bevel], [-half + bevel, radius], [half - bevel, radius], [half, radius - bevel], [half, 0]]


FORK = {"200": [[0.526, 0.1745], [0.557, 0.18], [0.575, 0.0785], [0.575, 0.0665], [0.564, 0.0565], [0.55, 0.0585], [0.543, 0.0725]], "500": [[0.522, 0.174], [0.559, 0.1805], [0.578, 0.079], [0.578, 0.066], [0.565, 0.055], [0.549, 0.057], [0.54, 0.0725]]}
SUPPORT = {"200": (0.009, 0.015), "500": (0.015, 0.02)}
LONG = ("humerus", "radius", "ulna", "femur", "tibia", "fibula")


def scooter(volumes):
    """🛞 Wheels, fork and axles per scale: what is below the feature is merged into the part that carries it."""
    for key in ("spoke-front", "spoke-rear"):
        web = by_id(volumes["100"]["body"])[key]
        for field in ("size", "transform", "repeat", "cornerRadius"):
            web.pop(field, None)
        web.update({"kind": "revolve", "profile": [[-0.0075, 0.012], [-0.0075, 0.038], [0.0075, 0.038], [0.0075, 0.012], [-0.0075, 0.012]], "axis": [0, 0, 1], "origin": [0, 0, 0]})
    for scale in ("200", "500"):
        volume = volumes[scale]
        body = by_id(volume["body"])
        body["fork-l"].update({"id": "fork", "profile": FORK[scale], "from": -0.0275, "to": 0.0275})
        for name in ("thermal", "structure"):
            for entry in volume[name]:
                if entry.get("of") == "fork-l":
                    entry["of"] = "fork"
                    entry["id"] = entry["id"].replace("fork-l", "fork")
        gone = ["fork-r", "axle-rear", "axle-front", "spoke-front", "spoke-rear", "hub-front", "hub-rear"]
        if scale == "200":
            for key in ("rim-front", "rim-rear"):
                body[key]["profile"] = disc(0.015, 0.048, 0.002)
        else:
            gone += ["rim-front", "rim-rear", "fork-crown"]
            for key in ("tyre-rear", "tyre-front"):
                body[key]["profile"] = [[-0.036, 0], [-0.036, 0.06], [-0.027, 0.0705], [0, 0.0725], [0.027, 0.0705], [0.036, 0.06], [0.036, 0]]
        drop(volume, gone)
        radius, half = SUPPORT[scale]
        for entry in volume["structure"]:
            if entry.get("part") == "support":
                entry["profile"] = [[-half, 0], [-half, radius], [half, radius], [half, 0]]
                entry["origin"][1] = radius


def skeleton(volumes, old, new):
    """💀 Girdle solids on the girdle bones, members meeting at their joints, coarse lists at true shaft sections."""
    torso_to = {side: Move(old["torso"], new[f"clavicle-{side}"]) for side in SIDES}
    pelvis_to = {side: Move(old["pelvis"], new[f"ilium-{side}"]) for side in SIDES}
    for scale in ("50", "100", "200"):
        for solid in volumes[scale]["structure"]:
            for name, spec in GIRDLE.items():
                for side in SIDES:
                    if solid["id"] in [f"{stem}-{side}" for stem in spec["solids"]] and solid.get("bone") == spec["parent"]:
                        (torso_to if name == "clavicle" else pelvis_to)[side].solid(solid, f"{name}-{side}")
    for scale, radius in (("50", 0.005), ("100", 0.0075)):
        structure = volumes[scale]["structure"]
        ids = by_id(structure)
        for side, (sign, _) in SIDES.items():
            move = torso_to[side]
            at = structure.index(ids[f"clavicle-{side}"]) + 1
            structure[at:at] = [
                capsule(f"scapular-spine-{side}", f"clavicle-{side}", move.point([0.289, 0.074, sign * 0.046]), move.point([0.318, 0.03, sign * 0.118]), radius),
                capsule(f"acromion-{side}", f"clavicle-{side}", move.point([0.318, 0.03, sign * 0.118]), move.point([0.326, 0.004, sign * 0.134]), radius),
                ball(f"glenoid-{side}", f"clavicle-{side}", move.point([0.302, 0.014, sign * 0.103]), [0.011, 0.011, 0.011]),
            ]
            ids[f"patella-{side}"]["transform"]["translation"] = [0.292, 0.0295, 0]
    for solid in volumes["100"]["structure"]:
        if solid["id"].startswith("fibula"):
            solid["slender"] = True
    volume = volumes["200"]
    volume["structure"] = [s for s in volume["structure"] if not s["id"].startswith("patella") and s["id"] != "shoulder-girdle"]
    at = next(i for i, s in enumerate(volume["structure"]) if s["id"] == "humerus-l")
    volume["structure"][at:at] = [capsule(f"clavicle-{side}", f"clavicle-{side}", [0, 0, 0], [0.12, 0, 0], 0.015) for side in SIDES]
    for solid in volume["structure"]:
        if solid["id"].split("-")[0] in LONG:
            solid["slender"] = True
        if solid.get("part") == "joint":
            solid["radii"] = [0.016, 0.016, 0.016]
    volume = volumes["500"]
    fine = by_id(volumes["50"]["structure"])
    volume["structure"] = [s for s in volume["structure"] if not s["id"].startswith("patella") and s.get("part") != "joint" and s["id"] != "spine-cervical"]
    for solid in volume["structure"]:
        if solid["id"].split("-")[0] in LONG:
            solid["slender"] = True
        if solid["id"] == "pelvis-bone":
            solid["members"] = ["ilium-l", "ilium-r"]
        if solid["id"] == "spine":
            solid["sections"] = copy.deepcopy(fine["spine"]["sections"]) + [
                {"at": [0.385, 0.006, 0], "rx": 0.013, "ry": 0.012, "rotation": [0.6963642, -0.1227878, 0.6963642, 0.1227878]},
                {"at": [0.42, 0.001, 0], "rx": 0.012, "ry": 0.011, "rotation": [0.7059346, -0.0403608, 0.7059346, 0.0403608]},
                {"at": [0.447, 0, 0], "rx": 0.012, "ry": 0.011, "rotation": [0.7071068, 0, 0.7071068, 0]},
            ]
            solid["slender"] = True
            solid["members"] = ["neck"]
    at = next(i for i, s in enumerate(volume["structure"]) if s["id"] == "humerus-l")
    volume["structure"][at:at] = [capsule("shoulder-girdle", "torso", [0.319, 0.014, -0.122], [0.319, 0.014, 0.122], 0.008, slender=True, members=["clavicle-l", "clavicle-r"])]


def build():
    document = json.load(open(os.path.join(G, "orig", "catalog.json"), encoding="utf-8"))
    volumes = json.load(open(os.path.join(G, "orig", "volumes.json"), encoding="utf-8"))["volumes"]
    asset = document["asset"]
    old, new = rig(asset)
    scooter(volumes)
    skeleton(volumes, old, new)
    extra = os.path.join(T, "rework-child-on-kick-scooter-fix.py")
    if os.path.exists(extra):
        scope = {"__name__": "fix"}
        exec(compile(open(extra, encoding="utf-8").read(), extra, "exec"), scope)
        scope["fix"](asset, volumes, old, new, sys.modules[__name__])
    asset["volumes"] = volumes
    return document, volumes


def main():
    document, volumes = build()
    os.makedirs(os.path.join(G, "cand"), exist_ok=True)
    with open(os.path.join(G, "cand", "🔣️.json"), "w", encoding="utf-8") as file:
        file.write(dump(document, 0) + "\n")
    for bone in document["asset"]["rig"]["bones"]:
        if bone["id"].split("-")[0] in ("clavicle", "ilium", "arm", "thigh"):
            print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass", "joint")}))
    if "--publish" in sys.argv:
        with open(SOURCE, "w", encoding="utf-8") as file:
            file.write('{"volumes":' + dump(volumes, 0) + "}\n")
        text = open(CATALOG, encoding="utf-8").read()
        live = json.loads(text)["asset"]
        for member in ("rig", "poses", "clips", "anchors", "structure", "thermal", "dimensions", "representations"):
            if document["asset"][member] == live[member]:
                continue
            marker = f'\n    "{member}": '
            start = text.index(marker) + len(marker)
            text = text[:start] + dump(document["asset"][member], 2) + text[member_end(text, start):]
            print("catalog member written:", member)
        with open(CATALOG, "w", encoding="utf-8") as file:
            file.write(text)
        print("published source")


if __name__ == "__main__":
    main()
