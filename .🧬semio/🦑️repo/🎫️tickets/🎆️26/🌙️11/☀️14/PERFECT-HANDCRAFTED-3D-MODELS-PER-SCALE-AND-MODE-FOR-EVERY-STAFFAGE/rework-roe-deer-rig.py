"""🦌 roe-deer: the limbs hang from the girdles at their true breadths, on the full 3D rest placement of the rig (see `rig-lateral-rest.md`).

What changes: two hip bones (`hip-hn`, `hip-hf`, sacroiliac joint to hip joint) carry the thighs; the stifle sits 18 mm outside the hip joint; the shoulder blades lie on the rib cage, their lower end further out, and carry the upper arms; the ears leave the skull at their bases. Every solid keeps its place in the world and is re-expressed in the new frame of its bone; the hip-bone solids (ilium, ischium) move from `pelvis` to the hip bones; the flat 2D shapes of the tilted shoulder blades become extruded solid shapes.

Usage: python3 rework-roe-deer-rig.py            writes the candidate `🗑️generated/rework-roe-deer/roe-deer/🔣️.json`
       python3 rework-roe-deer-rig.py --apply    re-reads catalog and source and writes `rig`, `representations`, `volumes`
"""
import copy
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🐕️animals", "roe-deer", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "animals", "roe-deer.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-roe-deer", "roe-deer", "🔣️.json")

SIDES = {"n": -1.0, "f": 1.0}
SIDE_NAME = {"n": "near", "f": "far"}
SIDE_TITLE = {"n": ({"en": "Near", "de": "Nahes"}), "f": ({"en": "Far", "de": "Fernes"})}
HIP_HALF_BREADTH = 0.05
STIFLE_HALF_BREADTH = 0.068
SACROILIAC_IN_PELVIS = (0.085, 0.012, 0.024)
SCAPULA_TOP_HALF_BREADTH = 0.045
SHOULDER_HALF_BREADTH = 0.068
EAR_BASE_HALF_BREADTH = 0.03
SCAPULA_SPAN = (0.033, 0.061)
LIMB = [0.5, 0.5, 0.5, 0.5]


def kit():
    spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


K = kit()


def add(a, b):
    return [a[i] + b[i] for i in range(3)]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def bone_of(bones, bone_id):
    return next(bone for bone in bones if bone["id"] == bone_id)


def place(bones, bone_id, parent_id, origin, world):
    bone = bone_of(bones, bone_id)
    solved = K.fields(K.frames(bones)[parent_id], origin, world)
    bone["parent"] = parent_id
    for key in ("x", "y", "z", "angleDeg", "abductionDeg", "twistDeg"):
        if key in ("z", "abductionDeg", "twistDeg") and abs(solved[key]) < 1e-4:
            bone.pop(key, None)
        else:
            bone[key] = solved[key]


def lateral(point, z):
    return [point[0], point[1], z]


def new_rig(bones, old):
    bones = copy.deepcopy(bones)
    pelvis_origin, pelvis_basis = old["pelvis"]
    for side, sign in SIDES.items():
        joint = add(pelvis_origin, K.apply(pelvis_basis, [SACROILIAC_IN_PELVIS[0], SACROILIAC_IN_PELVIS[1], sign * SACROILIAC_IN_PELVIS[2]]))
        socket = lateral(old[f"thigh-h{side}"][0], sign * HIP_HALF_BREADTH)
        along = sub(socket, joint)
        length = sum(c * c for c in along) ** 0.5
        hip = {
            "id": f"hip-h{side}",
            "parent": "pelvis",
            "title": {"en": f"{SIDE_NAME[side].capitalize()} hip bone (sacroiliac joint to hip joint)", "de": f"{'Nahes' if side == 'n' else 'Fernes'} Hüftbein (Kreuzdarmbeingelenk bis Hüftgelenk)"},
            **K.placed(old["pelvis"], joint, along, (0.0, 0.0, 1.0)),
            "lengthM": round(length, 5),
            "joint": {"type": "revolute", "minDeg": -3, "maxDeg": 3},
        }
        bones.insert(next(i for i, bone in enumerate(bones) if bone["id"] == f"thigh-h{side}"), hip)
        place(bones, f"thigh-h{side}", f"hip-h{side}", socket, old[f"thigh-h{side}"][1])
        bone_of(bones, f"shank-h{side}")["z"] = round(sign * (STIFLE_HALF_BREADTH - HIP_HALF_BREADTH), 5)
        top = lateral(old[f"scapula-f{side}"][0], sign * SCAPULA_TOP_HALF_BREADTH)
        shoulder = lateral(old[f"humerus-f{side}"][0], sign * SHOULDER_HALF_BREADTH)
        blade = sub(shoulder, top)
        scapula = bone_of(bones, f"scapula-f{side}")
        scapula.update(K.placed(old["thorax"], top, blade, (0.0, 0.0, 1.0)))
        scapula["lengthM"] = round(sum(c * c for c in blade) ** 0.5, 5)
        if abs(scapula["twistDeg"]) < 1e-4:
            scapula.pop("twistDeg")
        place(bones, f"humerus-f{side}", f"scapula-f{side}", shoulder, old[f"humerus-f{side}"][1])
        bone_of(bones, f"ear-{side}")["z"] = sign * EAR_BASE_HALF_BREADTH
    return bones


def moved(solid, turn, shift, turned):
    if solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = K.r5(add(K.apply(turn, section["at"]), shift))
            if turned:
                section["rotation"] = K.r5(K.product(K.quaternion(turn), section.get("rotation", [0, 0, 0, 1])))
    elif solid["kind"] == "ellipsoid":
        transform = solid.setdefault("transform", {})
        transform["translation"] = K.r5(add(K.apply(turn, transform.get("translation", [0, 0, 0])), shift))
        if turned:
            transform["rotation"] = K.r5(K.product(K.quaternion(turn), transform.get("rotation", [0, 0, 0, 1])))
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} on {solid['bone']} is not converted")


def between(old_frame, new_frame):
    turn = K.mul(K.transpose(new_frame[1]), old_frame[1])
    shift = K.apply(K.transpose(new_frame[1]), sub(old_frame[0], new_frame[0]))
    turned = any(abs(turn[i][j] - (1.0 if i == j else 0.0)) > 1e-6 for i in range(3) for j in range(3))
    return turn, shift, turned


def hip_bar(side, length, radius, slender):
    solid = {
        "id": f"{SIDE_NAME[side]}-hip-bone",
        "kind": "loft",
        "bone": f"hip-h{side}",
        "material": "bone",
        "role": "body",
        "tone": 0.18,
        "sections": [{"at": [round(x, 5), 0, 0], "rx": radius, "ry": radius, "exponent": 2, "rotation": LIMB} for x in (0.0, length / 2, length)],
        "capStart": True,
        "capEnd": True,
        "member": f"hip-h{side}",
        "part": "member",
    }
    if slender:
        solid["slender"] = True
    return solid


def convert(asset, volumes_sets):
    bones = asset["rig"]["bones"]
    if any(bone["id"] == "hip-hn" for bone in bones):
        return False
    old = K.frames(bones)
    asset["rig"]["bones"] = new_rig(bones, old)
    new = K.frames(asset["rig"]["bones"])
    for bone_id in old:
        for i in range(3):
            if abs(old[bone_id][0][i] - new[bone_id][0][i]) > 1e-4 and i != 2:
                raise SystemExit(f"{bone_id} moved in the sagittal plane")
    hips = {f"{SIDE_NAME[side]}-{part}": f"hip-h{side}" for side in SIDES for part in ("ilium", "ischium")}
    for volumes in volumes_sets:
        for scale, volume in volumes.items():
            for name in ("body", "thermal", "structure"):
                for solid in volume.get(name, []):
                    if "kind" not in solid or "bone" not in solid:
                        continue
                    was = solid["bone"]
                    if name == "structure" and solid["id"] in hips:
                        solid["bone"] = solid["member"] = hips[solid["id"]]
                    turn, shift, turned = between(old[was], new[solid["bone"]])
                    if turned or any(abs(c) > 1e-9 for c in shift):
                        moved(solid, turn, shift, turned)
            structure = volume["structure"]
            ids = {solid["id"] for solid in structure}
            pubis = next((solid for solid in structure if solid["id"] == "pubis"), None)
            if pubis is not None and "sacral-wings" not in ids:
                across = pubis["sections"][1]["rotation"]
                wings = {**copy.deepcopy(pubis), "id": "sacral-wings"}
                wings["sections"] = [{"at": [0.085, 0.012, z], "rx": rx, "ry": ry, "exponent": 2, "rotation": across} for z, rx, ry in ((-0.047, 0.008, 0.014), (-0.024, 0.009, 0.013), (0, 0.011, 0.012), (0.024, 0.009, 0.013), (0.047, 0.008, 0.014))]
                structure.insert(structure.index(pubis), wings)
            for side in SIDES:
                hip = bone_of(asset["rig"]["bones"], f"hip-h{side}")
                if f"{SIDE_NAME[side]}-ilium" not in ids:
                    structure.append(hip_bar(side, hip["lengthM"], 0.009, True))
                if f"joint-thigh-h{side}" in ids and int(scale) <= 100:
                    like = next(solid for solid in structure if solid["id"] == f"joint-thigh-h{side}")
                    joint = copy.deepcopy(like)
                    joint.update({"id": f"joint-hip-h{side}", "bone": f"hip-h{side}", "member": f"hip-h{side}", "radii": [0.008, 0.008, 0.008]})
                    joint["transform"] = {"translation": [0, 0, 0], "rotation": [0, 0, 0, 1]}
                    structure.append(joint)
    for scale, representation in asset["representations"].items():
        geometry = representation["geometry"]
        for side, sign in SIDES.items():
            bone_id = f"scapula-f{side}"
            flat = [shape for shape in geometry["elevation"] if shape.get("bone") == bone_id]
            for shape in flat:
                if "solid" in shape:
                    continue
                span = sorted([sign * SCAPULA_SPAN[0], sign * SCAPULA_SPAN[1]])
                shape["solid"] = {"kind": "extruded", "sweep": "w", "z0": span[0], "z1": span[1]}
                for view in ("front", "plan"):
                    for index, twin in enumerate(geometry[view]):
                        if twin["id"] == shape["id"] and twin.get("bone") == bone_id:
                            geometry[view][index] = {**copy.deepcopy(shape), "z": twin["z"]}
    return True


def splice(text, key, after):
    """Replaces one member of the asset in the catalog text in the catalog layout; every other member stays byte-identical."""
    import re

    insert, lib = K.insert_module(), K.lib_module()
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    start, end = insert.member_range(text, indent, key)
    return text[:start] + indent + json.dumps(key) + ": " + lib.catalog_text(after, len(indent) // 2) + text[end:]


def main():
    if "--apply" not in sys.argv:
        document = json.load(open(CATALOG, encoding="utf8"))
        if not convert(document["asset"], [document["asset"]["volumes"]]):
            raise SystemExit("already converted")
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        for bone in document["asset"]["rig"]["bones"]:
            if bone["id"][:3] in ("hip", "thi", "sha", "sca", "hum", "ear"):
                print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass", "joint")}))
        print(CANDIDATE)
        return
    original = text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    source_text = open(SOURCE, encoding="utf8").read()
    source = json.loads(source_text)
    if source["volumes"] != document["asset"]["volumes"]:
        raise SystemExit("source and catalog volumes differ: make them agree first")
    if not convert(document["asset"], [document["asset"]["volumes"]]):
        raise SystemExit("already converted")
    source["volumes"] = document["asset"]["volumes"]
    for key in ("rig", "representations", "volumes"):
        text = splice(text, key, document["asset"][key])
    if json.loads(text) != document:
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf8").read() != original:
        raise SystemExit("the catalog file changed while converting: run again")
    open(CATALOG, "w", encoding="utf8").write(text)
    open(SOURCE, "w", encoding="utf8").write(json.dumps(source, ensure_ascii=False, separators=(",", ":")) + "\n")
    print("applied to", CATALOG, "and", SOURCE)


if __name__ == "__main__":
    main()
