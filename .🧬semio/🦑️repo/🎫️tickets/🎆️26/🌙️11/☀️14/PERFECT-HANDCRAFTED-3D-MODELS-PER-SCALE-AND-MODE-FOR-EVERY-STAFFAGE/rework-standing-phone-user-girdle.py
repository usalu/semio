"""🦴 Girdles of `humans/standing-phone-user` on the lateral rest placement: clavicles and hip bones become bones, arms and thighs hang from them without moving, the girdle solids are re-expressed in the new frames and the coarse skeleton lists are cleaned for their feature size.

Usage: python3 rework-standing-phone-user-girdle.py <findings.txt> [--apply]
Without `--apply` the candidate document is written to `🗑️generated/rework-standing-phone-user/🔣️.json`.
See `rig-lateral-rest.md` (sections 1.3, 1.4, 1.6).
"""
import copy
import importlib.util
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "standing-phone-user", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "standing-phone-user.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-standing-phone-user", "🔣️.json")
SCALES = ("50", "100", "200", "500")
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}

SACROILIAC = (0.075, 0.045, 0.045)
CLAVICLE_JOINT = {"l": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40, "twistMaxDeg": 5}, "r": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -5, "twistMaxDeg": 40}}
ILIUM_JOINT = {"type": "revolute", "minDeg": -5, "maxDeg": 5}
TITLES = {
    "clavicle-l": {"en": "Left clavicle and scapula", "de": "Linkes Schlüsselbein mit Schulterblatt"},
    "clavicle-r": {"en": "Right clavicle and scapula", "de": "Rechtes Schlüsselbein mit Schulterblatt"},
    "ilium-l": {"en": "Left hip bone", "de": "Linkes Hüftbein"},
    "ilium-r": {"en": "Right hip bone", "de": "Rechtes Hüftbein"},
}
SHOULDER_PARTS = ("clavicle", "scapula", "scapular-spine", "acromion", "glenoid", "shoulder-girdle")
HIP_PARTS = ("hip-bone", "pubic-ramus", "ischial-ramus")
SHAFTS = ("femur", "tibia", "fibula", "humerus", "ulna", "radius")
HIP_500 = {"crest": (0.105, 0.02, 0.075), "ischium": (-0.035, 0.0, 0.055), "radius": 0.0375}
SACRUM_500 = {"p0": [0.09, 0.04, 0], "p1": [0.02, 0.065, 0], "radius": 0.0375}
CLAVICLE_500_RADIUS = 0.0065
JOINT_200 = 0.0155
OCCLUSION_X = -0.0205
GRIP_PAD = 0.0085
AURICULAR = ((0.078, 0.045, 0.047), (0.088, 0.012, 0.092))
AURICULAR_RADIUS = {"50": 0.012, "100": 0.012}
HUMERUS = 0.3135
RADIUS_END = 0.24
ELBOW_200 = 0.022
CARPUS_200 = 0.0155
STERNAL_REACH = 0.012
SCAPULA_500 = ((-0.03, 1.40, 0.135), (-0.07, 1.29, 0.10))
MANUBRIUM = (0.40842, -0.07192, 0)


def lateral():
    spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


L = lateral()


def world(frame, p):
    return [frame[0][i] + L.apply(frame[1], p)[i] for i in range(3)]


def local(frame, p):
    return L.apply(L.transpose(frame[1]), [p[i] - frame[0][i] for i in range(3)])


def move(solid, old, new, bone):
    turn = L.mul(L.transpose(new[1]), old[1])
    q = L.quaternion(turn)
    point = lambda p: L.r5(local(new, world(old, p)))
    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = point(solid["p0"]), point(solid["p1"])
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = point(section["at"])
            section["rotation"] = L.r5(L.product(q, section["rotation"]))
    elif solid["kind"] == "ellipsoid":
        transform = solid["transform"]
        transform["translation"] = point(transform["translation"])
        transform["rotation"] = L.r5(L.product(q, transform.get("rotation", [0, 0, 0, 1])))
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} is not converted")
    solid["bone"] = bone
    if "member" in solid:
        solid["member"] = bone


def ordered(bone):
    keys = ("id", "parent", "title", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg", "lengthM", "joint")
    out = {key: bone[key] for key in keys if key in bone}
    out.update({key: value for key, value in bone.items() if key not in out})
    return out


def rig(asset, volumes):
    bones = asset["rig"]["bones"]
    if any(bone["id"] == "clavicle-l" for bone in bones):
        return None
    old = L.frames(copy.deepcopy(bones))
    clavicle = next(solid for solid in volumes["50"]["structure"] if solid["id"] == "clavicle-l")
    sternal, acromial = clavicle["sections"][0]["at"], clavicle["sections"][-1]["at"]
    out = []
    for bone in bones:
        side = bone["id"][-1]
        sign = 1 if side == "l" else -1
        if bone["id"] in ("arm-l", "arm-r", "thigh-l", "thigh-r"):
            shoulder = bone["id"].startswith("arm")
            parent = old["torso" if shoulder else "pelvis"]
            if shoulder:
                origin = world(parent, [sternal[0], sternal[1], sign * abs(sternal[2])])
                tip = world(parent, [acromial[0], acromial[1], sign * abs(acromial[2])])
            else:
                origin = world(parent, [SACROILIAC[0], SACROILIAC[1], sign * SACROILIAC[2]])
                tip = old[bone["id"]][0]
            u = [tip[i] - origin[i] for i in range(3)]
            girdle = {"id": ("clavicle-" if shoulder else "ilium-") + side, "parent": "torso" if shoulder else "pelvis"}
            girdle["title"] = TITLES[girdle["id"]]
            girdle.update(L.placed(parent, origin, u, (-sign, 0.0, 0.0)))
            girdle["lengthM"] = round(math.sqrt(sum(c * c for c in u)), 5)
            girdle["joint"] = CLAVICLE_JOINT[side] if shoulder else ILIUM_JOINT
            out.append(ordered(girdle))
            frame = L.frames(out)[girdle["id"]]
            placement = L.fields(frame, old[bone["id"]][0], old[bone["id"]][1])
            bone = dict(bone, parent=girdle["id"])
            for key, value in placement.items():
                if key in ("abductionDeg", "twistDeg") and abs(value) < 1e-9:
                    bone.pop(key, None)
                else:
                    bone[key] = value
            bone = ordered(bone)
        out.append(bone)
    asset["rig"]["bones"] = out
    new = L.frames(out)
    for bone_id, frame in old.items():
        drift = max(abs(new[bone_id][0][i] - frame[0][i]) for i in range(3))
        turned = max(abs(new[bone_id][1][i][j] - frame[1][i][j]) for i in range(3) for j in range(3))
        if drift > 2e-5 or turned > 2e-5:
            raise SystemExit(f"{bone_id} moved by {drift} m / {turned}")
    return old, new


def rebind(volumes, old, new):
    count = 0
    for scale in SCALES:
        for name in ("body", "thermal", "structure"):
            for solid in volumes[scale][name]:
                side = solid["id"][-1]
                stem = solid["id"][:-2]
                if side not in "lr" or solid["id"][-2] != "-":
                    continue
                if solid.get("bone") == "torso" and stem in SHOULDER_PARTS:
                    move(solid, old["torso"], new["clavicle-" + side], "clavicle-" + side)
                    count += 1
                elif solid.get("bone") == "pelvis" and stem in HIP_PARTS:
                    move(solid, old["pelvis"], new["ilium-" + side], "ilium-" + side)
                    count += 1
    return count


def flagged(path):
    out = {}
    for line in open(path, encoding="utf8"):
        match = re.search(r'volumes-feature volumes\.(\d+)\.structure\[\d+\]: Solid "([^"]+)"', line)
        if match:
            out.setdefault(match.group(1), set()).add(match.group(2))
    return out


def capsule(solid_id, bone, p0, p1, radius, slender=False):
    out = {"id": solid_id, "kind": "capsule", "bone": bone, "material": "bone", "role": "body", "tone": 0.85, "p0": L.r5(p0), "p1": L.r5(p1), "r0": radius, "r1": radius, "member": bone, "part": "member"}
    if slender:
        out["slender"] = True
    return out


def coarse(volumes, findings, bones, new):
    length = {bone["id"]: bone["lengthM"] for bone in bones}
    log = []
    for scale in ("200", "500"):
        kept = []
        for solid in volumes[scale]["structure"]:
            stem = solid["id"][:-2]
            if stem in SHAFTS:
                solid["slender"] = True
                ends = [solid["p0"], solid["p1"]] if solid["kind"] == "capsule" else [solid["sections"][0]["at"], solid["sections"][-1]["at"]]
                log.append(f"{scale} {solid['id']} {solid['kind']} {ends[0]} .. {ends[1]} of {length[solid['bone']]}")
                kept.append(solid)
            elif solid["id"] in findings.get(scale, ()):
                if solid.get("part") == "joint" and scale == "200" and solid["kind"] in ("ellipsoid", "capsule"):
                    if solid["kind"] == "ellipsoid":
                        solid["radii"] = [max(r, JOINT_200) for r in solid["radii"]]
                    else:
                        solid["r0"], solid["r1"] = max(solid["r0"], JOINT_200), max(solid["r1"], JOINT_200)
                    kept.append(solid)
                else:
                    log.append(f"{scale} left out {solid['id']} ({solid.get('part')} {solid.get('kind')})")
            else:
                kept.append(solid)
        volumes[scale]["structure"] = kept
    out = []
    for solid in volumes["500"]["structure"]:
        if solid["id"] == "shoulder-girdle":
            for side in "lr":
                bone = "clavicle-" + side
                out.append(capsule(bone, bone, [0, 0, 0], [length[bone], 0, 0], CLAVICLE_500_RADIUS, True))
        elif solid["id"] == "pelvic-girdle":
            out.append(capsule("sacrum", "pelvis", SACRUM_500["p0"], SACRUM_500["p1"], SACRUM_500["radius"]))
            for side, sign in (("l", 1), ("r", -1)):
                bone = "ilium-" + side
                ends = [local(new[bone], world(new["pelvis"], [p[0], p[1], sign * p[2]])) for p in (HIP_500["crest"], HIP_500["ischium"])]
                out.append(capsule("hip-bone-" + side, bone, ends[0], ends[1], HIP_500["radius"]))
        else:
            out.append(solid)
    volumes["500"]["structure"] = out
    return log


def ball(solid_id, bone, at, radii, part="member"):
    return {"id": solid_id, "kind": "ellipsoid", "bone": bone, "material": "bone", "role": "body", "tone": 0.85, "radii": radii, "transform": {"translation": at, "rotation": [0, 0, 0, 1]}, "member": bone, "part": part}


def find(solids, solid_id):
    return next(solid for solid in solids if solid["id"] == solid_id)


def after(solids, solid_id, new):
    index = next(i for i, solid in enumerate(solids) if solid["id"] == solid_id)
    solids[index + 1 : index + 1] = new


def stretch(solid, x0=None, x1=None):
    near, far = (solid["p0"], solid["p1"]) if solid["p0"][0] < solid["p1"][0] else (solid["p1"], solid["p0"])
    if x0 is not None:
        near[0] = x0
    if x1 is not None:
        far[0] = x1


def connect(volumes, new):
    """Closes the load path where bones meet in reality and adds the sock that carries the shoe at 1:100 and 1:200."""
    fine = volumes["50"]["structure"]
    find(fine, "mandible")["sections"][0]["at"][0] = OCCLUSION_X
    grip = find(fine, "phalanx-r-5b")
    after(fine, "phalanx-r-5b", [{"id": "grip-pad-r", "kind": "capsule", "bone": "hand-r", "material": "skin", "role": "body", "tone": 0.85, "p0": list(grip["p0"]), "p1": list(grip["p1"]), "r0": GRIP_PAD, "r1": GRIP_PAD, "member": "hand-r", "part": "joint"}])
    for scale in ("50", "100"):
        for side, sign in (("l", 1), ("r", -1)):
            bone = "ilium-" + side
            ends = [local(new[bone], world(new["pelvis"], [p[0], p[1], sign * p[2]])) for p in AURICULAR]
            solid = capsule("posterior-ilium-" + side, bone, ends[0], ends[1], AURICULAR_RADIUS[scale])
            after(volumes[scale]["structure"], "hip-bone-" + side, [solid])
    for scale in ("100", "200"):
        body, thermal = volumes[scale]["body"], volumes[scale]["thermal"]
        for side in "lr":
            sock = copy.deepcopy(find(volumes["50"]["body"], "sock-" + side))
            if scale == "200":
                sock["sections"] = [sock["sections"][0], sock["sections"][-1]]
            after(body, "shank-" + side, [sock])
            for entry in volumes["50"]["thermal"]:
                if entry.get("of") == "sock-" + side:
                    thermal.append(copy.deepcopy(entry))
    middle = volumes["100"]["structure"]
    for side in "lr":
        after(middle, "radial-end-" + side, [copy.deepcopy(find(fine, "carpus-" + side))])
    coarse_200, coarse_500 = volumes["200"]["structure"], volumes["500"]["structure"]
    for side, sign in (("l", 1), ("r", -1)):
        stretch(find(coarse_200, "humerus-" + side), x1=HUMERUS)
        stretch(find(coarse_200, "radius-" + side), x1=RADIUS_END)
        after(coarse_200, "radius-" + side, [ball("elbow-" + side, "forearm-" + side, [0, 0, 0], [ELBOW_200] * 3), ball("carpus-" + side, "hand-" + side, [0.008, 0, 0], [CARPUS_200] * 3)])
        girdle = find(coarse_200, "shoulder-girdle-" + side)
        girdle["sections"][0]["at"][0] = round(girdle["sections"][0]["at"][0] - STERNAL_REACH, 5)
        stretch(find(coarse_500, "humerus-" + side), x0=0.0, x1=HUMERUS)
        stretch(find(coarse_500, "radius-" + side), x0=0.0, x1=RADIUS_END)
        find(coarse_500, "clavicle-" + side)["p0"][0] = -STERNAL_REACH
        after(coarse_500, "femur-" + side, [ball("proximal-femur-" + side, "thigh-" + side, [0.01, 0, sign * 0.022], [0.0375, 0.0375, 0.055])])
        after(coarse_500, "tibia-" + side, [ball("knee-" + side, "shank-" + side, [0, 0, 0], [0.0375, 0.0375, 0.045])])
        bone = "clavicle-" + side
        ends = [local(new[bone], [p[0], p[1], sign * p[2]]) for p in SCAPULA_500]
        after(coarse_500, bone, [capsule("scapula-" + side, bone, ends[0], ends[1], 0.0375)])
    sternum = find(coarse_500, "open-sternum")
    sternum["sections"].insert(0, dict(sternum["sections"][0], at=list(MANUBRIUM)))


def build(asset, volumes, findings):
    solved = rig(asset, volumes)
    if solved is None:
        raise SystemExit("already converted")
    old, new = solved
    count = rebind(volumes, old, new)
    log = coarse(volumes, findings, asset["rig"]["bones"], new)
    connect(volumes, new)
    return old, new, count, log


def main():
    findings = flagged(sys.argv[1])
    original = text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    source_text = open(SOURCE, encoding="utf8").read()
    source = json.loads(source_text)
    old, new, count, log = build(document["asset"], source["volumes"], findings)
    document["asset"]["volumes"] = source["volumes"]
    if "--apply" not in sys.argv:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        for bone in document["asset"]["rig"]["bones"]:
            if bone["id"][:-2] in ("clavicle", "ilium", "arm", "thigh"):
                print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass")}))
        for bone_id in ("pelvis", "torso", "clavicle-l", "arm-l", "ilium-l", "thigh-l", "neck", "head"):
            origin, basis = new[bone_id]
            print(bone_id, "origin", L.r5(origin), "u", L.r5([basis[i][0] for i in range(3)]))
        print("rebound", count)
        print("\n".join(log))
        print(CANDIDATE)
        return
    text = L.splice(text, "rig", before["rig"], document["asset"]["rig"])
    if json.loads(text)["asset"]["rig"] != document["asset"]["rig"]:
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf8").read() != original:
        raise SystemExit("the catalog file changed while converting: run again")
    if open(SOURCE, encoding="utf8").read() != source_text:
        raise SystemExit("the volumes source changed while converting: run again")
    open(CATALOG, "w", encoding="utf8").write(text)
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("applied to", CATALOG, "and", SOURCE)


if __name__ == "__main__":
    main()
