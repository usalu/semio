"""🐈 domestic-cat: shoulder and pelvic girdle on the lateral rest placement of the rig (see `rig-lateral-rest.md`).

Scapula: runs from its dorsal border 22 mm beside the median plane down, forward and outward to the shoulder joint 40 mm beside it (66.5 mm, 15.7 degrees out of the sagittal plane); the humerus hangs from its tip with its sagittal frame unchanged. Hip bone (os coxae, new bones `hip-hf`, `hip-hn`): fixed from the sacroiliac joint 19 mm beside the plane to the hip joint 30 mm beside it; the femur hangs from its tip unchanged. Poses that swing the scapula get the humerus offset that keeps the posed limb parasagittal. Solids of the scapulae are re-expressed in the inclined frame, the ilia and ischia are bound to the hip bones, the coarse pelvis block spans both hip bones, the elevation shapes of the scapulae are stretched to the longer bone.

Usage: python3 rework-domestic-cat-girdle.py            writes the candidate `🗑️generated/rework-domestic-cat/🔣️.json`
       python3 rework-domestic-cat-girdle.py --apply    writes rig, poses, representations and volumes into the catalog file and the volumes source
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🐕️animals", "domestic-cat", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "animals", "domestic-cat.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-domestic-cat", "🔣️.json")

SACROILIAC = (0.040, 0.004, 0.019)
SIDES = (("f", 1), ("n", -1))
HIP_TITLE = {"f": {"en": "Far hip bone (os coxae)", "de": "Fernes Hüftbein (Os coxae)"}, "n": {"en": "Near hip bone (os coxae)", "de": "Nahes Hüftbein (Os coxae)"}}
MODE = "none"
HUMERUS_TWIST = {"f": (-20, 20), "n": (-20, 20)}


def kit():
    spec = importlib.util.spec_from_file_location("rig_lateral_standing_adult", os.path.join(T, "rig-lateral-standing-adult.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


K = kit()


def moved(solid, old, new):
    """The same world solid written in the frame of another bone (`old` and `new` are `(origin, basis)`)."""
    turn = K.mul(K.transpose(new[1]), old[1])
    shift = K.apply(K.transpose(new[1]), [old[0][i] - new[0][i] for i in range(3)])
    q = K.quaternion(turn)
    at = lambda p: K.r5([K.apply(turn, p)[i] + shift[i] for i in range(3)])
    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = at(solid["p0"]), at(solid["p1"])
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = at(section["at"])
            section["rotation"] = K.r5(K.product(q, section["rotation"]))
    elif solid["kind"] == "ellipsoid":
        transform = solid["transform"]
        transform["translation"] = at(transform["translation"])
        transform["rotation"] = K.r5(K.product(q, transform.get("rotation", [0, 0, 0, 1])))
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} is not converted")


def end_of(frames, bone):
    origin, basis = frames[bone["id"]]
    return [origin[i] + basis[i][0] * bone["lengthM"] for i in range(3)]


def shapes(node, found):
    if isinstance(node, dict):
        if str(node.get("bone", "")).startswith("scapula-") and "points" in node:
            found.append(node)
        for value in node.values():
            shapes(value, found)
    elif isinstance(node, list):
        for value in node:
            shapes(value, found)


def convert(asset, volumes_sets, report=False):
    bones = asset["rig"]["bones"]
    by_id = {bone["id"]: bone for bone in bones}
    if "hip-hf" in by_id:
        return False
    old_bones = copy.deepcopy(bones)
    old_by = {bone["id"]: bone for bone in old_bones}
    old = K.frames(old_bones)
    stretch = {}
    for side, sign in SIDES:
        scapula, humerus = by_id[f"scapula-f{side}"], by_id[f"humerus-f{side}"]
        origin, tip = old[scapula["id"]][0], old[humerus["id"]][0]
        u = [tip[i] - origin[i] for i in range(3)]
        length = math.sqrt(sum(c * c for c in u))
        stretch[scapula["id"]] = length / scapula["lengthM"]
        scapula.update({**K.placed(old["thorax-b"], origin, u, (0.0, 0.0, 1.0)), "lengthM": round(length, 5)})
        scapula["mass"]["comX"] = round(scapula["mass"]["comX"] * stretch[scapula["id"]], 4)
        now = K.frames(bones)
        humerus.pop("z", None)
        humerus.update({key: value for key, value in K.fields(now[scapula["id"]], old[humerus["id"]][0], old[humerus["id"]][1]).items() if key not in ("abductionDeg", "twistDeg") or abs(value) > 1e-9})
        humerus["joint"] = {**humerus["joint"], "twistMinDeg": HUMERUS_TWIST[side][0], "twistMaxDeg": HUMERUS_TWIST[side][1]}
        thigh = by_id[f"thigh-h{side}"]
        pelvis = old["pelvis"]
        joint = [pelvis[0][i] + K.apply(pelvis[1], [SACROILIAC[0], SACROILIAC[1], sign * SACROILIAC[2]])[i] for i in range(3)]
        socket = old[thigh["id"]][0]
        u = [socket[i] - joint[i] for i in range(3)]
        hip = {"id": f"hip-h{side}", "parent": "pelvis", "title": HIP_TITLE[side], **K.placed(pelvis, joint, u, (0.0, 0.0, 1.0)), "lengthM": round(math.sqrt(sum(c * c for c in u)), 5), "joint": {"type": "fixed"}}
        bones.insert(bones.index(thigh), hip)
        by_id[hip["id"]] = hip
        now = K.frames(bones)
        thigh.pop("z", None)
        thigh["parent"] = hip["id"]
        thigh.update({key: value for key, value in K.fields(now[hip["id"]], old[thigh["id"]][0], old[thigh["id"]][1]).items() if key not in ("abductionDeg", "twistDeg") or abs(value) > 1e-9})
    new = K.frames(bones)
    for bone in old_bones:
        if bone["id"].startswith("scapula-"):
            continue
        drift = max(abs(new[bone["id"]][0][i] - old[bone["id"]][0][i]) for i in range(3))
        turned = max(abs(new[bone["id"]][1][i][j] - old[bone["id"]][1][i][j]) for i in range(3) for j in range(3))
        if drift > 2e-5 or turned > 2e-5:
            raise SystemExit(f"{bone['id']} moved by {drift} m / {turned}")
    for pose in asset["poses"]:
        offsets = pose.get("bones", {})
        before = K.frames(old_bones, copy.deepcopy(offsets))
        for side, sign in SIDES:
            scapula, humerus, paw = f"scapula-f{side}", f"humerus-f{side}", f"paw-f{side}"
            if scapula not in offsets:
                continue
            if MODE == "shoulder" or (MODE == "gait" and pose["id"].startswith(("walk-", "stalk-"))):
                parent = K.frames(bones, {key: value for key, value in offsets.items() if key != humerus})[humerus][1]
                a, b, t = K.angles(K.mul(K.transpose(parent), before[humerus][1]))
                offsets[humerus] = {key: value for key, value in (("angleDeg", round(a, 3) + 0.0), ("abductionDeg", round(b, 3) + 0.0), ("twistDeg", round(t, 3) + 0.0)) if key == "angleDeg" or abs(value) > 5e-4}
            else:
                target = end_of(before, old_by[paw])
                start = offsets.get(humerus, {}).get("angleDeg", 0.0)
                best = min((start + step * 0.01 for step in range(-800, 801)), key=lambda angle: math.dist(end_of(K.frames(bones, {**offsets, humerus: {"angleDeg": angle}}), by_id[paw])[:2], target[:2]))
                offsets[humerus] = {"angleDeg": round(best, 2) + 0.0}
        if report:
            after = K.frames(bones, offsets)
            worst = [round(max(abs(end_of(after, by_id[b])[i] - end_of(before, old_by[b])[i]) for b in ("paw-ff", "paw-fn")) * 1000, 2) for i in range(3)]
            print("[DEBUG] pose", pose["id"], "forepaw tip moved [ahead, up, left] mm", worst, {k: v for k, v in offsets.items() if k.startswith("humerus")})
    for clip in asset["clips"]:
        for key in clip["keys"]:
            if any(bone_id.startswith(("scapula-", "humerus-")) for bone_id in key.get("bones", {})):
                raise SystemExit(f"clip {clip['id']} overrides a shoulder bone in a key: convert it by hand")
    hip_bone = {"near-ilium": "hip-hn", "near-ischium": "hip-hn", "far-ilium": "hip-hf", "far-ischium": "hip-hf"}
    for volumes in volumes_sets:
        for scale, volume in volumes.items():
            for name in ("body", "thermal", "structure"):
                for solid in volume.get(name, []):
                    bone = solid.get("bone")
                    if bone in stretch:
                        moved(solid, old[bone], new[bone])
                    elif bone == "pelvis" and solid["id"] in hip_bone and name == "structure":
                        target = hip_bone[solid["id"]]
                        moved(solid, old["pelvis"], new[target])
                        solid["bone"] = target
                        solid["member"] = target
                    elif bone == "pelvis" and solid["id"] == "pelvis" and name == "structure":
                        solid["members"] = ["hip-hf", "hip-hn"]
                        if scale == "500":
                            for section in solid["sections"]:
                                section["ry"] = 0.0152
    found = []
    shapes(asset["representations"], found)
    for scale, representation in asset["representations"].items():
        elevation = []
        shapes(representation.get("elevation", representation.get("geometry", {}).get("elevation", [])) if isinstance(representation, dict) else [], elevation)
        for shape in elevation:
            shape["points"] = [[round(point[0] * stretch[shape["bone"]], 5), *point[1:]] for point in shape["points"]]
        if report:
            print("[DEBUG] scale", scale, "keys", list(representation.keys())[:8], "elevation scapula shapes", [shape["id"] for shape in elevation], "of", len([s for s in found]))
    if report:
        for anchor in asset["anchors"]:
            if anchor["bone"].startswith(("scapula", "thigh", "humerus")):
                print("[DEBUG] anchor", anchor["id"], anchor["bone"])
        for bone in bones:
            if bone["id"].startswith(("scapula-", "humerus-", "hip-", "thigh-")):
                print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass")}))
    return True


def main():
    if "--apply" not in sys.argv:
        document = json.load(open(CATALOG, encoding="utf8"))
        source = json.load(open(SOURCE, encoding="utf8"))
        document["asset"]["volumes"] = source["volumes"]
        if not convert(document["asset"], [document["asset"]["volumes"]], True):
            raise SystemExit("already converted")
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        print(CANDIDATE)
        return
    original = text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    source_text = open(SOURCE, encoding="utf8").read()
    source = json.loads(source_text)
    if not convert(document["asset"], [document["asset"]["volumes"], source["volumes"]]):
        raise SystemExit("already converted")
    for key in ("rig", "poses", "volumes"):
        text = K.splice(text, key, before[key], document["asset"][key])
    was, now = [], []
    shapes(before["representations"], was)
    shapes(document["asset"]["representations"], now)
    for old_shape, new_shape in zip(was, now):
        if old_shape["points"] != new_shape["points"]:
            text = text.replace('"points": ' + json.dumps(old_shape["points"]), '"points": ' + json.dumps(new_shape["points"]))
    if json.loads(text) != document:
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf8").read() != original:
        raise SystemExit("the catalog file changed while converting: run again")
    open(CATALOG, "w", encoding="utf8").write(text)
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("applied to", CATALOG, "and", SOURCE)


if __name__ == "__main__":
    main()
