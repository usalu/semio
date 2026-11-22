"""🤰 pregnant-person: the rig gets its anatomical chain. `lumbar` (L5/S1 to T12/L1) is inserted below the thoracic `torso`, the foot bone runs from the ankle to the ball instead of level above the sole, `clavicle-l/-r` run laterally from the sternoclavicular to the acromioclavicular joint and carry the arms, `ilium-l/-r` run from the sacroiliac joint to the hip joint and carry the thighs (see `rig-lateral-rest.md`).

Every solid, 2D elevation shape, anchor and mass bound to a bone whose frame changed is re-expressed in the new frame (same world geometry); arms, thighs and toes keep their world rest frames.

Usage: python3 rework-05-pregnant-person-rig.py            writes the candidate `🗑️generated/rework-pregnant-person/cand/🔣️.json`
       python3 rework-05-pregnant-person-rig.py --apply    re-reads the catalog file, converts it and writes it and the volumes source
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "pregnant-person", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "pregnant-person.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-pregnant-person", "cand", "🔣️.json")


def load(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(T, file))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def dumper():
    text = open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read()
    scope = {"json": json}
    exec(text[text.index("def scalar"):text.index("def member_end")], scope)
    return scope["dump"]


K = load("rig_kit", "rig-lateral-standing-adult.py")
INLINE = dumper()


def DUMP(document, depth):
    if "asset" not in document:
        return INLINE(document, depth)
    volumes = document["asset"]["volumes"]
    mark = "\u0000volumes\u0000"
    text = json.dumps({**document, "asset": {**document["asset"], "volumes": mark}}, indent=2, ensure_ascii=False)
    return text.replace(json.dumps(mark), INLINE(volumes, 2))

LUMBAR_LENGTH = 0.17
LUMBAR_MASS, THORAX_MASS = 9.728, 10.258
LUMBAR_COM, THORAX_COM = (0.09, -0.0257), (0.1331, -0.0257)
STERNOCLAVICULAR = (0.41, -0.083, 0.02)
ACROMIOCLAVICULAR = (0.43, -0.025, 0.155)
SACROILIAC = (0.07, 0.045, 0.05)
GIRDLE = ("clavicle-", "scapula-", "scapular-spine-", "glenoid-", "shoulder-girdle-")
HIP = ("hip-bone-", "pubic-ramus-", "ischial-ramus-", "acetabulum-")
LUMBAR_SOLIDS = ("vertebra-l", "spinous-l", "joint-lumbosacral")
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}
IDENTITY = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]


def r5(values):
    return [round(x, 5) + 0.0 for x in values]


def moved(point, matrix, shift):
    return r5([K.apply(matrix, point)[i] + shift[i] for i in range(3)])


def rebind(solid, matrix, shift):
    q = K.quaternion(matrix)
    turned = matrix != IDENTITY
    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = moved(solid["p0"], matrix, shift), moved(solid["p1"], matrix, shift)
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = moved(section["at"], matrix, shift)
            if turned and "rotation" in section:
                section["rotation"] = r5(K.product(q, section["rotation"]))
            elif turned and abs(section["rx"] - section["ry"]) > 1e-9:
                raise SystemExit(f"loft {solid['id']} has a flat section without rotation")
    elif solid["kind"] == "ellipsoid":
        transform = solid["transform"]
        transform["translation"] = moved(transform["translation"], matrix, shift)
        if turned and (len(set(solid["radii"])) > 1 or "rotation" in transform):
            transform["rotation"] = r5(K.product(q, transform.get("rotation", [0, 0, 0, 1])))
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} is not converted")


def change(old, new):
    matrix = K.mul(K.transpose(new[1]), old[1])
    shift = K.apply(K.transpose(new[1]), [old[0][i] - new[0][i] for i in range(3)])
    matrix = [[round(x, 12) + 0.0 for x in row] for row in matrix]
    return (IDENTITY if all(abs(matrix[i][j] - IDENTITY[i][j]) < 1e-9 for i in range(3) for j in range(3)) else matrix), shift


def solids(volumes):
    for scale, lists in volumes.items():
        for name in ("body", "thermal", "structure"):
            for solid in lists.get(name, []):
                if "bone" in solid:
                    yield scale, name, solid


def heel_as_loft(solid):
    xs = [p[1] for p in solid["profile"]]
    zs = [p[0] for p in solid["profile"]]
    centre, half_length, half_width = (max(xs) + min(xs)) / 2, (max(xs) - min(xs)) / 2, (max(zs) - min(zs)) / 2
    turn = [-0.70711, 0, 0, 0.70711]
    solid["sections"] = [
        {"at": r5([centre, solid["from"], 0]), "rx": round(half_length, 5), "ry": round(half_width, 5), "exponent": 3.2, "rotation": turn},
        {"at": r5([centre, solid["to"], 0]), "rx": round(half_length + 0.001, 5), "ry": round(half_width + 0.0005, 5), "exponent": 3.2, "rotation": turn},
    ]
    solid["capStart"], solid["capEnd"] = True, True
    for key in ("profile", "plane", "from", "to"):
        del solid[key]
    solid["kind"] = "loft"


def bone(asset, bone_id):
    return next(b for b in asset["rig"]["bones"] if b["id"] == bone_id)


def put_before(asset, new, before):
    bones = asset["rig"]["bones"]
    bones.insert(next(i for i, b in enumerate(bones) if b["id"] == before), new)


def shapes(asset, bone_id):
    for representation in asset["representations"].values():
        for shape in representation["geometry"].get("elevation", []):
            if shape.get("bone") == bone_id:
                if "solid" in shape:
                    raise SystemExit(f"shape {shape['id']} on {bone_id} has a solid profile")
                yield shape


def foot(asset, volumes_sets, side):
    foot_bone, toe_bone = bone(asset, f"foot-{side}"), bone(asset, f"toe-{side}")
    before = K.frames(asset["rig"]["bones"])
    drop = math.atan2(toe_bone["y"], toe_bone["x"]) * K.DEG
    length = math.hypot(toe_bone["x"], toe_bone["y"])
    foot_bone["angleDeg"] = round(foot_bone["angleDeg"] + drop, 4)
    foot_bone["lengthM"] = round(length, 5)
    toe_bone.update({"x": round(length, 5), "y": 0, "angleDeg": round(toe_bone["angleDeg"] - drop, 4)})
    after = K.frames(asset["rig"]["bones"])
    matrix, shift = change(before[foot_bone["id"]], after[foot_bone["id"]])
    com = K.apply(matrix, [foot_bone["mass"]["comX"], foot_bone["mass"]["comY"], 0])
    foot_bone["mass"]["comX"], foot_bone["mass"]["comY"] = round(com[0], 4), round(com[1], 4)
    for anchor in asset["anchors"]:
        if anchor["bone"] == foot_bone["id"]:
            at = K.apply(matrix, [anchor["x"], anchor["y"], 0])
            anchor["x"], anchor["y"] = round(at[0], 5) + 0.0, round(at[1], 5) + 0.0
            if "normalDeg" in anchor:
                anchor["normalDeg"] = round(anchor["normalDeg"] - drop, 4)
    for shape in shapes(asset, foot_bone["id"]):
        shape["points"] = [[round(matrix[0][0] * x + matrix[0][1] * y, 4), round(matrix[1][0] * x + matrix[1][1] * y, 4)] for x, y in shape["points"]]
    for volumes in volumes_sets:
        for _, _, solid in solids(volumes):
            if solid["bone"] == foot_bone["id"]:
                if solid["kind"] == "extrude":
                    heel_as_loft(solid)
                rebind(solid, matrix, shift)


def lumbar(asset, volumes_sets):
    torso = bone(asset, "torso")
    new = {"id": "lumbar", "parent": "pelvis", "title": {"en": "Lumbar spine and abdomen", "de": "Lendenwirbelsäule und Bauchraum"}, "x": torso["x"], "y": torso["y"], "angleDeg": torso["angleDeg"], "lengthM": LUMBAR_LENGTH, "joint": {"type": "revolute", "minDeg": -30, "maxDeg": 20}, "mass": dict(torso["mass"], kg=LUMBAR_MASS, comX=LUMBAR_COM[0], comY=LUMBAR_COM[1])}
    put_before(asset, new, "torso")
    torso.update({"parent": "lumbar", "title": {"en": "Thorax with sweater", "de": "Brustkorb mit Pullover"}, "x": LUMBAR_LENGTH, "y": 0, "angleDeg": 0, "lengthM": round(torso["lengthM"] - LUMBAR_LENGTH, 5), "joint": {"type": "revolute", "minDeg": -12, "maxDeg": 8}})
    torso["mass"].update({"kg": THORAX_MASS, "comX": THORAX_COM[0], "comY": THORAX_COM[1]})
    for child in asset["rig"]["bones"]:
        if child["parent"] == "torso" and child["id"] != "belly":
            child["x"] = round(child["x"] - LUMBAR_LENGTH, 5)
    bone(asset, "belly")["parent"] = "lumbar"
    shift = [-LUMBAR_LENGTH, 0, 0]
    for shape in shapes(asset, "torso"):
        shape["points"] = [[round(x - LUMBAR_LENGTH, 5), y] for x, y in shape["points"]]
    for entry in asset["poses"] + [key for clip in asset["clips"] for key in clip["keys"]]:
        if "torso" in entry.get("bones", {}):
            entry["bones"] = {("lumbar" if k == "torso" else k): v for k, v in entry["bones"].items()}
    for volumes in volumes_sets:
        for scale, lists in volumes.items():
            structure = lists.get("structure", [])
            for index, solid in enumerate(list(structure)):
                if solid["id"] == "spine":
                    sections = solid["sections"]
                    cut = next(i for i, s in enumerate(sections) if s["at"][0] > LUMBAR_LENGTH)
                    a, b = sections[cut - 1], sections[cut]
                    t = (LUMBAR_LENGTH - a["at"][0]) / (b["at"][0] - a["at"][0])
                    joint = dict(a, at=r5([LUMBAR_LENGTH, a["at"][1] + t * (b["at"][1] - a["at"][1]), 0]))
                    low = copy.deepcopy(solid)
                    low.update({"id": "lumbar-spine", "bone": "lumbar", "member": "lumbar", "sections": copy.deepcopy(sections[:cut]) + [copy.deepcopy(joint)]})
                    solid["sections"] = [copy.deepcopy(joint)] + sections[cut:]
                    structure.insert(structure.index(solid), low)
            for name in ("body", "thermal", "structure"):
                for solid in lists.get(name, []):
                    if solid.get("bone") != "torso":
                        continue
                    if solid["id"].startswith(LUMBAR_SOLIDS):
                        solid["bone"] = "lumbar"
                        if "member" in solid:
                            solid["member"] = "lumbar"
                    else:
                        rebind(solid, IDENTITY, shift)
            sacral = next((s for s in structure if s["id"] == "joint-lumbosacral"), None)
            mass = next((s for s in structure if s["id"] == "mass-torso"), None)
            if sacral:
                top = copy.deepcopy(sacral)
                top.update({"id": "joint-thoracolumbar", "bone": "torso", "member": "torso"})
                structure.insert(structure.index(sacral) + 1, top)
            if mass:
                low = copy.deepcopy(mass)
                low.update({"id": "mass-lumbar", "bone": "lumbar", "member": "lumbar"})
                low["transform"]["translation"] = [LUMBAR_COM[0], LUMBAR_COM[1], 0]
                mass["transform"]["translation"] = [THORAX_COM[0], THORAX_COM[1], 0]
                if scale == "50":
                    low["radii"] = [0.02092] * 3
                    mass["radii"] = [0.02149] * 3
                structure.insert(structure.index(mass), low)


def girdle(asset, volumes_sets, side, new_id, parent_id, start, end, limb_id, prefixes, title, joint, joint_id):
    sign = 1 if side == "l" else -1
    rest = K.frames(asset["rig"]["bones"])
    parent = rest[parent_id]
    world = lambda p: [parent[0][i] + K.apply(parent[1], [p[0], p[1], sign * p[2]])[i] for i in range(3)]
    origin = world(start)
    tip = world(end) if end else rest[limb_id][0]
    u = [tip[i] - origin[i] for i in range(3)]
    placed = K.placed(parent, origin, u, (-1.0, 0.0, 0.0) if side == "l" else (1.0, 0.0, 0.0))
    length = round(math.sqrt(sum(c * c for c in u)), 5)
    new = {"id": new_id, "parent": parent_id, "title": title, **placed, "lengthM": length, "joint": joint}
    put_before(asset, new, limb_id)
    frame = K.frames(asset["rig"]["bones"])[new_id]
    limb = bone(asset, limb_id)
    limb.update({"parent": new_id, **K.fields(frame, rest[limb_id][0], rest[limb_id][1])})
    bones = asset["rig"]["bones"]
    after = K.frames(bones)
    drift = max(abs(after[limb_id][0][i] - rest[limb_id][0][i]) for i in range(3))
    turn_drift = max(abs(after[limb_id][1][i][j] - rest[limb_id][1][i][j]) for i in range(3) for j in range(3))
    print(f"{new_id}: length {length}, {limb_id} moved {drift:.6f} m, turned {turn_drift:.6f}")
    matrix, shift = change(parent, frame)
    for volumes in volumes_sets:
        for scale, lists in volumes.items():
            structure = lists.get("structure", [])
            bound = [s for s in structure if s.get("bone") == parent_id and s["id"].startswith(prefixes) and s["id"].endswith("-" + side)]
            for solid in bound:
                rebind(solid, matrix, shift)
                solid["bone"], solid["member"] = new_id, new_id
            radius = max(0.011, FEATURE[scale] / 2 + 0.0003)
            if not bound:
                bar = {"id": f"{prefixes[-1]}{side}", "kind": "loft", "bone": new_id, "material": "bone", "role": "body", "tone": 0.92, "sections": [{"at": [0, 0, 0], "rx": radius, "ry": radius}, {"at": [length, 0, 0], "rx": radius, "ry": radius}], "capStart": True, "capEnd": True, "member": new_id, "part": "member"}
                structure.append(bar)
            structure.append({"id": f"{joint_id}-{side}", "kind": "ellipsoid", "bone": new_id, "material": "bone", "role": "body", "tone": 0.6, "radii": [radius] * 3, "transform": {"translation": [0, 0, 0]}, "member": new_id, "part": "joint"})


def convert(asset, volumes_sets):
    if any(b["id"] == "lumbar" for b in asset["rig"]["bones"]):
        raise SystemExit("already converted")
    for side in ("l", "r"):
        foot(asset, volumes_sets, side)
    lumbar(asset, volumes_sets)
    for side, name, de in (("l", "Left", "Linkes"), ("r", "Right", "Rechtes")):
        girdle(asset, volumes_sets, side, f"clavicle-{side}", "torso", [STERNOCLAVICULAR[0] - LUMBAR_LENGTH, STERNOCLAVICULAR[1], STERNOCLAVICULAR[2]], [ACROMIOCLAVICULAR[0] - LUMBAR_LENGTH, ACROMIOCLAVICULAR[1], ACROMIOCLAVICULAR[2]], f"arm-{side}", GIRDLE, {"en": f"{name} clavicle with shoulder blade", "de": f"{de} Schlüsselbein mit Schulterblatt"}, {"type": "revolute", "minDeg": -8, "maxDeg": 35}, "joint-sternoclavicular")
        girdle(asset, volumes_sets, side, f"ilium-{side}", "pelvis", SACROILIAC, None, f"thigh-{side}", HIP, {"en": f"{name} hip bone", "de": f"{de} Hüftbein"}, {"type": "revolute", "minDeg": -4, "maxDeg": 4}, "joint-sacroiliac")


def main():
    document = json.load(open(CATALOG, encoding="utf-8"))
    text = open(CATALOG, encoding="utf-8").read()
    print("round trip identical:", DUMP(document, 0) + "\n" == text)
    asset = document["asset"]
    source = json.load(open(SOURCE, encoding="utf-8"))
    if source["volumes"] != asset["volumes"]:
        raise SystemExit("catalog volumes and source differ")
    convert(asset, [asset["volumes"]])
    out = DUMP(document, 0) + "\n"
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    open(CANDIDATE, "w", encoding="utf-8").write(out)
    print("candidate", CANDIDATE)
    if "--apply" in sys.argv:
        open(CATALOG, "w", encoding="utf-8").write(out)
        open(SOURCE, "w", encoding="utf-8").write(DUMP({"volumes": asset["volumes"]}, 0) + "\n")
        print("applied")


if __name__ == "__main__":
    main()
