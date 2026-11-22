"""🦴 runner-with-blade-prosthesis: shoulder and pelvic girdle on the lateral rest placement (`rig-lateral-rest.md`).

New bones `clavicle-l/-r` (sternoclavicular joint to acromioclavicular joint, child of `torso`) and `ilium-l/-r` (sacroiliac joint to hip joint centre, child of `pelvis`); the upper arms and thighs are re-parented without moving. The skeleton solids of the shoulder blade and collarbone and of the hip bones are re-expressed in the new bone frames, the collarbone is re-modelled as the S-curved bone along its rig bone, the manubrium is lowered to the jugular notch, joints are added, `hands-on-hips` and the inhaling keys of `recover-breathing` lift the shoulder girdle.

Usage: python3 rework-runner-with-blade-prosthesis-girdle.py            candidate `🗑️generated/rework-runner-with-blade-prosthesis/cand/🔣️.json`
       python3 rework-runner-with-blade-prosthesis-girdle.py --apply    writes `rig`, `poses`, `clips` of the catalog file and the volumes source
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
SLUG = "runner-with-blade-prosthesis"
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", SLUG, "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", SLUG + ".json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-" + SLUG, "cand", "🔣️.json")

spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
rig = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rig)

STERNOCLAVICULAR = (0.058, 1.4887, 0.024)
ACROMIOCLAVICULAR = (-0.01, 1.50028, 0.156)
SACROILIAC = (-0.036, 1.01173, 0.04)
BACK = (-1.0, 0.0, 0.0)
CLAVICLE_JOINT = {"l": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40, "twistMaxDeg": 5}, "r": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -5, "twistMaxDeg": 40}}
ILIUM_JOINT = {"type": "revolute", "minDeg": -4, "maxDeg": 4}
TITLES = {
    "clavicle-l": {"en": "Left collarbone and shoulder blade", "de": "Linkes Schlüsselbein mit Schulterblatt"},
    "clavicle-r": {"en": "Right collarbone and shoulder blade", "de": "Rechtes Schlüsselbein mit Schulterblatt"},
    "ilium-l": {"en": "Left hip bone", "de": "Linkes Hüftbein"},
    "ilium-r": {"en": "Right hip bone", "de": "Rechtes Hüftbein"},
}
SHOULDER = ("sk-clavicle", "sk-scapula", "sk-scapular-spine", "sk-scapula-neck", "sk-glenoid", "sk-acromion", "sk-coracoid")
HIP = ("sk-acetabulum", "sk-iliac-crest", "sk-ilium", "sk-sacroiliac", "sk-pubis", "sk-ischium", "sk-ischiopubic-ramus", "sk-hip")
CLAVICLE_STATIONS = [(0.0, 0.0, 0.0, 0.0105, 0.0115), (0.13, 0.0005, -0.0045, 0.0078, 0.0082), (0.33, 0.001, -0.009, 0.0063, 0.0066), (0.58, 0.0005, -0.004, 0.006, 0.0062), (0.79, 0.0, 0.005, 0.0055, 0.008), (0.92, 0.0, 0.004, 0.005, 0.0095), (1.0, 0.0, 0.0, 0.005, 0.009)]
LIFT = {"hands-on-hips": 5}
INHALE_LIFT = 8
MANUBRIUM = {0.45: (0.436, -0.0705), 0.48: (0.458, -0.066)}


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def mirrored(fields):
    return {key: (-value if key in ("z", "abductionDeg", "twistDeg") else value) for key, value in fields.items()}


def ordered(bone):
    keys = ("id", "parent", "title", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg", "lengthM", "joint")
    out = {key: bone[key] for key in keys if key in bone and not (key in ("z", "abductionDeg", "twistDeg") and abs(bone[key]) < 1e-9)}
    out.update({key: value for key, value in bone.items() if key not in keys})
    return out


def rebind(solid, old, new):
    """The same world solid written in the frame `new` instead of `old` (origin, basis)."""
    m = rig.mul(rig.transpose(new[1]), old[1])
    q = rig.quaternion(m)
    shift = rig.apply(rig.transpose(new[1]), sub(old[0], new[0]))

    def point(p):
        return rig.r5([rig.apply(m, p)[i] + shift[i] for i in range(3)])

    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = point(solid["p0"]), point(solid["p1"])
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = point(section["at"])
            section["rotation"] = rig.r5(rig.product(q, section.get("rotation", [0, 0, 0, 1])))
    elif solid["kind"] == "ellipsoid":
        transform = solid.setdefault("transform", {})
        transform["translation"] = point(transform.get("translation", [0, 0, 0]))
        transform["rotation"] = rig.r5(rig.product(q, transform.get("rotation", [0, 0, 0, 1])))
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} is not converted")


def clavicle_solid(base, side, length, floor):
    sign = 1 if side == "l" else -1
    solid = {key: base[key] for key in ("id", "material", "role", "tone") if key in base}
    solid.update({"kind": "loft", "bone": f"clavicle-{side}", "sections": [{"at": [round(f * length, 5), v, sign * w, ], "rx": max(rx, floor), "ry": max(ry, floor), "exponent": 2, "rotation": [0.5, 0.5, 0.5, 0.5]} for f, v, w, rx, ry in CLAVICLE_STATIONS], "capStart": True, "capEnd": True, "member": f"clavicle-{side}", "part": "member"})
    return {key: solid[key] for key in ("id", "kind", "bone", "material", "role", "tone", "sections", "capStart", "capEnd", "member", "part")}


def joint_solid(like, bone, factor):
    solid = copy.deepcopy(like)
    solid.update({"id": f"joint-{bone}", "bone": bone, "member": bone, "radii": [round(r * factor, 5) for r in like["radii"]], "transform": {"translation": [0.0, 0.0, 0.0]}})
    return solid


def convert(asset, volumes):
    bones = asset["rig"]["bones"]
    if any(bone["id"] == "clavicle-l" for bone in bones):
        return False
    old_bones = copy.deepcopy(bones)
    old = rig.frames(old_bones)
    made = {}
    for name, parent, origin, tip, joint in (("clavicle", "torso", STERNOCLAVICULAR, ACROMIOCLAVICULAR, CLAVICLE_JOINT), ("ilium", "pelvis", SACROILIAC, None, {"l": ILIUM_JOINT, "r": ILIUM_JOINT})):
        tip = tip or old["thigh-l"][0]
        u = sub(tip, origin)
        left = rig.placed(old[parent], list(origin), u, BACK)
        length = round(math.sqrt(sum(c * c for c in u)), 5)
        for side, fields in (("l", left), ("r", mirrored(left))):
            made[f"{name}-{side}"] = ordered({"id": f"{name}-{side}", "parent": parent, "title": TITLES[f"{name}-{side}"], **fields, "lengthM": length, "joint": joint[side]})
    for child, girdle in (("arm", "clavicle"), ("thigh", "ilium")):
        for side in ("l", "r"):
            index = next(i for i, bone in enumerate(bones) if bone["id"] == f"{child}-{side}")
            bones.insert(index, made[f"{girdle}-{side}"])
    by_id = {bone["id"]: bone for bone in bones}
    now = rig.frames(bones)
    for child, girdle in (("arm", "clavicle"), ("thigh", "ilium")):
        for side in ("l", "r"):
            bone = by_id[f"{child}-{side}"]
            bone.update({"parent": f"{girdle}-{side}", **rig.fields(now[f"{girdle}-{side}"], old[bone["id"]][0], old[bone["id"]][1])})
            fresh = ordered(bone)
            bone.clear()
            bone.update(fresh)
    new = rig.frames(bones)
    for bone in old_bones:
        drift = max(abs(new[bone["id"]][0][i] - old[bone["id"]][0][i]) for i in range(3))
        turned = max(abs(new[bone["id"]][1][i][j] - old[bone["id"]][1][i][j]) for i in range(3) for j in range(3))
        if drift > 3e-5 or turned > 3e-5:
            raise SystemExit(f"{bone['id']} moved by {drift} m / {turned}")

    def lift(offsets, before_offsets, degrees):
        before = rig.frames(old_bones, before_offsets)
        for side in ("l", "r"):
            offsets[f"clavicle-{side}"] = {"angleDeg": degrees}
        for side in ("l", "r"):
            arm = f"arm-{side}"
            parent = rig.frames(bones, {key: value for key, value in offsets.items() if key != arm})[arm][1]
            a, b, t = rig.angles(rig.mul(rig.transpose(parent), before[arm][1]))
            offsets[arm] = {key: value for key, value in (("angleDeg", round(a, 1) + 0.0), ("abductionDeg", round(b, 1) + 0.0), ("twistDeg", round(t, 1) + 0.0)) if key == "angleDeg" or value != 0}

    for pose in asset["poses"]:
        if pose["id"] in LIFT:
            lift(pose["bones"], copy.deepcopy(pose["bones"]), LIFT[pose["id"]])
    for clip in asset["clips"]:
        for key in clip["keys"]:
            own = key.get("bones", {})
            if key.get("pose") in LIFT and own.get("head", {}).get("angleDeg", 0) > 12:
                for side in ("l", "r"):
                    own[f"clavicle-{side}"] = {"angleDeg": INHALE_LIFT}
    for scale, volume in volumes.items():
        if volume.get("status") != "authored":
            continue
        structure = volume["structure"]
        floor = {"50": 0.0, "100": 0.0078}.get(scale)
        for position, solid in enumerate(structure):
            name, side = solid["id"][:-2], solid["id"][-1]
            stem = name.rstrip("-0123456789") if name.startswith("sk-iliac-crest") else name
            if solid["id"][-2:] in ("-l", "-r") or name.startswith("sk-iliac-crest"):
                if name.startswith("sk-iliac-crest"):
                    side = solid["id"].split("-")[3]
                    stem = "sk-iliac-crest"
                target = f"clavicle-{side}" if stem in SHOULDER else f"ilium-{side}" if stem in HIP else None
                if target and solid.get("bone") in ("torso", "pelvis"):
                    if stem == "sk-clavicle":
                        length = made[target]["lengthM"]
                        if floor is None:
                            solid.update({"bone": target, "member": target, "p0": [0.0, 0.0, 0.0], "p1": [length, 0.0, 0.0]})
                        else:
                            structure[position] = clavicle_solid(solid, side, length, floor)
                        continue
                    rebind(solid, old[solid["bone"]], new[target])
                    solid["bone"] = solid["member"] = target
            if solid["id"] == "sk-sternum" and floor is not None:
                for section in solid["sections"]:
                    if section["at"][0] in MANUBRIUM:
                        section["at"][0], section["at"][1] = MANUBRIUM[section["at"][0]]
        ids = [solid["id"] for solid in structure]
        for side in ("l", "r"):
            factor = 0.8 if floor is not None else 1
            structure.insert(ids.index(f"joint-arm-{side}"), joint_solid(structure[ids.index(f"joint-arm-{side}")], f"clavicle-{side}", factor))
            ids = [solid["id"] for solid in structure]
            structure.insert(ids.index(f"joint-thigh-{side}"), joint_solid(structure[ids.index(f"joint-thigh-{side}")], f"ilium-{side}", factor))
            ids = [solid["id"] for solid in structure]
    return True


def main():
    original = text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    source_text = open(SOURCE, encoding="utf8").read()
    source = json.loads(source_text)
    document["asset"]["volumes"] = source["volumes"]
    if not convert(document["asset"], source["volumes"]):
        raise SystemExit("already converted")
    for bone in document["asset"]["rig"]["bones"]:
        if bone["id"][:-2] in ("clavicle", "ilium", "arm", "thigh"):
            print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass")}))
    if "--apply" not in sys.argv:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        print(CANDIDATE)
        return
    for key in ("rig", "poses", "clips"):
        text = rig.splice(text, key, before[key], document["asset"][key])
    if open(CATALOG, encoding="utf8").read() != original or open(SOURCE, encoding="utf8").read() != source_text:
        raise SystemExit("a file changed while converting: run again")
    open(CATALOG, "w", encoding="utf8").write(text)
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("applied: now run volumes-insert.py")


if __name__ == "__main__":
    main()
