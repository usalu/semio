"""🦽 wheelchair-user: anatomical rig on the full 3D rest placement (see `rig-lateral-rest.md`).

Hand-chosen changes, all stated here:
- the trunk bone is split at the thoracolumbar joint: `lumbar` (L5/S1 to T12/L1, 0.17 m, the old trunk frame) carries `torso` (thorax, T12/L1 to C7/T1, 0.325 m);
- `clavicle-l/-r` run from the sternoclavicular joint to the acromioclavicular joint and carry the arms (world rest frames of the arms unchanged);
- `ilium-l/-r` (hip bones) run from the sacroiliac joint to the hip joint and carry the thighs (world rest frames unchanged);
- trunk offsets of poses and clip keys move to `lumbar`; the left arm of every pose and key is the mirror of the right arm;
- every solid, 2D elevation shape and anchor of the old trunk is re-expressed in the frame of the bone it now belongs to.

Usage: python3 rework-wheelchair-user-rig.py            candidate `🗑️generated/rework-wheelchair-user/cand/🔣️.json`
       python3 rework-wheelchair-user-rig.py --apply    re-reads and writes the catalog file and the volumes source
"""
import copy
import importlib.util
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "wheelchair-user", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "wheelchair-user.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-wheelchair-user", "cand", "🔣️.json")


def load(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(T, file))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


K = load("rig_kit", "rig-lateral-standing-adult.py")
PUBLISH = {"__file__": os.path.join(T, "rework-04-publish.py")}
exec(open(os.path.join(T, "rework-04-publish.py"), encoding="utf-8").read().split("\ndef main")[0], PUBLISH)

LUMBAR_M = 0.17
STERNOCLAVICULAR = (0.436, -0.114, 0.022)
ACROMIOCLAVICULAR = (0.458, -0.056, 0.17)
SACROILIAC = (0.07, 0.058, 0.04)
GIRDLE_AXIS = {"l": (-1.0, 0.0, 0.0), "r": (1.0, 0.0, 0.0)}
SHOULDER = re.compile(r"^(clavicle|scapula|acromion|coracoid|glenoid|scapular-spine|shoulder-bar)(-.*)?-(l|r)$")
HIP = re.compile(r"^(ilium|pubis|ischium|hip-bar|acetabulum)(-.*)?-(l|r)$")
ARM = ("arm", "forearm", "hand")


def side_point(point, side):
    return [point[0], point[1], point[2] if side == "l" else -point[2]]


def world(frame, point):
    moved = K.apply(frame[1], point)
    return [frame[0][i] + moved[i] for i in range(3)]


def moved(solid, matrix, shift):
    """The same world solid in a bone frame turned by `matrix` whose origin lies at `shift` of the old frame."""
    def at(p):
        return K.r5(K.apply(matrix, [p[i] - shift[i] for i in range(3)]))
    q = K.quaternion(matrix)
    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = at(solid["p0"]), at(solid["p1"])
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = at(section["at"])
            if "rotation" in section:
                section["rotation"] = K.r5(K.product(q, section["rotation"]))
            elif abs(section["rx"] - section["ry"]) > 1e-9 and matrix != IDENTITY:
                raise SystemExit(f"loft {solid['id']} has an oval section without rotation")
    elif solid["kind"] == "ellipsoid":
        transform = solid["transform"]
        transform["translation"] = at(transform["translation"])
        if matrix != IDENTITY:
            transform["rotation"] = K.r5(K.product(q, transform.get("rotation", [0, 0, 0, 1])))
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} is not converted")


IDENTITY = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]


def cut(solid):
    """Splits a loft along the trunk at the thoracolumbar joint into its lumbar and its thoracic part."""
    sections = solid["sections"]
    xs = [s["at"][0] for s in sections]
    if xs != sorted(xs) or xs[0] >= LUMBAR_M - 0.02 or xs[-1] <= LUMBAR_M + 0.02:
        return None
    k = next(i for i, x in enumerate(xs) if x > LUMBAR_M)
    a, b = sections[k - 1], sections[k]
    f = (LUMBAR_M - a["at"][0]) / (b["at"][0] - a["at"][0])
    mid = {"at": K.r5([a["at"][i] + f * (b["at"][i] - a["at"][i]) for i in range(3)]), "rx": round(a["rx"] + f * (b["rx"] - a["rx"]), 5), "ry": round(a["ry"] + f * (b["ry"] - a["ry"]), 5)}
    if "rotation" in a:
        mid["rotation"] = a["rotation"]
    low, high = copy.deepcopy(solid), copy.deepcopy(solid)
    low["sections"] = copy.deepcopy(sections[:k]) + [copy.deepcopy(mid)]
    high["sections"] = [copy.deepcopy(mid)] + copy.deepcopy(sections[k:])
    return low, high


def mirror(offset):
    out = dict(offset)
    for key in ("abductionDeg", "twistDeg"):
        if key in out:
            out[key] = round(-out[key], 3) + 0.0
    return out


def symmetric(bones):
    for part in ARM:
        if f"{part}-r" in bones:
            bones[f"{part}-l"] = mirror(bones[f"{part}-r"])
    if "torso" in bones:
        renamed = {}
        for key, value in bones.items():
            renamed["lumbar" if key == "torso" else key] = value
        bones.clear()
        bones.update(renamed)


def convert(asset):
    bones = asset["rig"]["bones"]
    by_id = {bone["id"]: bone for bone in bones}
    if "lumbar" in by_id:
        raise SystemExit("already converted")
    old = K.frames(copy.deepcopy(bones))
    torso = by_id["torso"]
    lumbar = {"id": "lumbar", "parent": "pelvis", "title": {"en": "Lumbar spine", "de": "Lendenwirbelsäule"}, "x": torso["x"], "y": torso["y"], "angleDeg": torso["angleDeg"], "lengthM": LUMBAR_M, "joint": {"type": "revolute", "minDeg": -50, "maxDeg": 30}}
    torso.update({"parent": "lumbar", "title": {"en": "Thorax with hoodie", "de": "Brustkorb mit Kapuzenpullover"}, "x": LUMBAR_M, "y": 0, "angleDeg": 0, "lengthM": round(torso["lengthM"] - LUMBAR_M, 5), "joint": {"type": "revolute", "minDeg": -25, "maxDeg": 15}})
    torso["mass"]["comX"] = round(torso["mass"]["comX"] - LUMBAR_M, 5)
    for bone in bones:
        if bone["parent"] == "torso":
            bone["x"] = round(bone["x"] - LUMBAR_M, 5)
    bones.insert(bones.index(torso), lumbar)
    thorax = (world(old["torso"], [LUMBAR_M, 0, 0]), old["torso"][1])
    girdle = {}
    for side in ("l", "r"):
        sc, ac = (world(old["torso"], side_point(p, side)) for p in (STERNOCLAVICULAR, ACROMIOCLAVICULAR))
        u = [ac[i] - sc[i] for i in range(3)]
        length = sum(c * c for c in u) ** 0.5
        clavicle = {"id": f"clavicle-{side}", "parent": "torso", "title": {"en": f"{'Left' if side == 'l' else 'Right'} clavicle with scapula", "de": f"{'Linkes' if side == 'l' else 'Rechtes'} Schlüsselbein mit Schulterblatt"}, **K.placed(thorax, sc, u, GIRDLE_AXIS[side]), "lengthM": round(length, 5), "joint": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40 if side == "l" else -5, "twistMaxDeg": 5 if side == "l" else 40}}
        arm = by_id[f"arm-{side}"]
        bones.insert(bones.index(arm), clavicle)
        frame = K.frames(copy.deepcopy(bones))[clavicle["id"]]
        arm.update({"parent": clavicle["id"], **K.fields(frame, old[arm["id"]][0], old[arm["id"]][1])})
        girdle[clavicle["id"]] = frame
        si = world(old["pelvis"], side_point(SACROILIAC, side))
        hip = old[f"thigh-{side}"][0]
        u = [hip[i] - si[i] for i in range(3)]
        length = sum(c * c for c in u) ** 0.5
        ilium = {"id": f"ilium-{side}", "parent": "pelvis", "title": {"en": f"{'Left' if side == 'l' else 'Right'} hip bone", "de": f"{'Linkes' if side == 'l' else 'Rechtes'} Hüftbein"}, **K.placed(old["pelvis"], si, u, GIRDLE_AXIS[side]), "lengthM": round(length, 5), "joint": {"type": "revolute", "minDeg": -4, "maxDeg": 4}}
        thigh = by_id[f"thigh-{side}"]
        bones.insert(bones.index(thigh), ilium)
        frame = K.frames(copy.deepcopy(bones))[ilium["id"]]
        thigh.update({"parent": ilium["id"], **K.fields(frame, old[thigh["id"]][0], old[thigh["id"]][1])})
        girdle[ilium["id"]] = frame
    new = K.frames(copy.deepcopy(bones))
    for bone_id, frame in old.items():
        drift = max(abs(frame[0][i] - new[bone_id][0][i]) for i in range(3)) if bone_id != "torso" else 0
        if drift > 2e-5:
            raise SystemExit(f"{bone_id} moved by {drift}")
    for anchor in asset["anchors"]:
        if anchor["bone"] == "torso":
            anchor["x"] = round(anchor["x"] - LUMBAR_M, 5)
    for pose in asset["poses"]:
        symmetric(pose["bones"])
    for clip in asset["clips"]:
        for key in clip["keys"]:
            symmetric(key.get("bones", {}))
    for scale, representation in asset["representations"].items():
        for shape in representation["geometry"].get("elevation", []):
            if shape.get("bone") == "torso":
                if shape["kind"] != "poly":
                    raise SystemExit(f"2D shape {shape['id']} of kind {shape['kind']} is not converted")
                shape["points"] = [[round(p[0] - LUMBAR_M, 5), p[1]] for p in shape["points"]]
    report = []
    for scale, volume in asset["volumes"].items():
        if volume.get("status") != "authored":
            continue
        for name in ("body", "thermal", "structure"):
            out = []
            for solid in volume[name]:
                bone = solid.get("bone")
                if "kind" not in solid:
                    if solid.get("member") == "torso":
                        raise SystemExit(f"reference {solid['id']} on the trunk")
                    out.append(solid)
                    continue
                shoulder, hip = SHOULDER.match(solid["id"]), HIP.match(solid["id"])
                if bone == "torso" and name == "structure" and shoulder:
                    target = f"clavicle-{shoulder.group(3)}"
                    frame = girdle[target]
                    moved(solid, K.mul(K.transpose(frame[1]), old["torso"][1]), K.apply(K.transpose(old["torso"][1]), [frame[0][i] - old["torso"][0][i] for i in range(3)]))
                    solid["bone"] = solid["member"] = target
                    report.append((scale, solid["id"], target))
                elif bone == "pelvis" and name == "structure" and hip:
                    target = f"ilium-{hip.group(3)}"
                    frame = girdle[target]
                    moved(solid, K.mul(K.transpose(frame[1]), old["pelvis"][1]), K.apply(K.transpose(old["pelvis"][1]), [frame[0][i] - old["pelvis"][0][i] for i in range(3)]))
                    solid["bone"] = solid["member"] = target
                    report.append((scale, solid["id"], target))
                elif bone == "torso":
                    xs = [s["at"][0] for s in solid["sections"]] if solid["kind"] == "loft" else [solid["p0"][0], solid["p1"][0]] if solid["kind"] == "capsule" else [solid["transform"]["translation"][0]]
                    parts = cut(solid) if name == "structure" and solid["kind"] == "loft" and solid["id"] == "spine" else None
                    if parts:
                        low, high = parts
                        low.update({"id": "lumbar-spine", "bone": "lumbar", "member": "lumbar"})
                        moved(high, IDENTITY, [LUMBAR_M, 0, 0])
                        out.append(low)
                        solid = high
                        report.append((scale, "spine", "lumbar-spine + spine"))
                    elif name == "structure" and max(xs) <= LUMBAR_M + 0.005:
                        solid["bone"] = "lumbar"
                        if solid.get("member") == "torso":
                            solid["member"] = "lumbar"
                        if solid["id"] == "joint-torso":
                            solid["id"] = "joint-lumbar"
                            fresh = copy.deepcopy(solid)
                            fresh.update({"id": "joint-torso", "bone": "torso", "member": "torso"})
                            out.append(fresh)
                        report.append((scale, solid["id"], "lumbar"))
                    else:
                        moved(solid, IDENTITY, [LUMBAR_M, 0, 0])
                out.append(solid)
            volume[name] = out
        ids = {s["id"] for s in volume["structure"]}
        joint = next((s for s in volume["structure"] if s.get("part") == "joint" and s.get("kind") == "ellipsoid" and s["bone"] == "neck"), None)
        for solid in volume["structure"]:
            if solid["id"] == "shoulder-bar":
                solid["members"] = ["clavicle-l", "clavicle-r"]
            if solid["id"] == "pelvic-bar":
                solid["members"] = ["ilium-l", "ilium-r"]
        for side in ("l", "r"):
            for bone in (f"clavicle-{side}", f"ilium-{side}"):
                if joint:
                    fresh = copy.deepcopy(joint)
                    fresh.update({"id": f"joint-{bone}", "bone": bone, "member": bone})
                    fresh["transform"] = {"translation": [0, 0, 0]}
                    volume["structure"].append(fresh)
                if not any(s.get("member") == bone and s.get("part") == "member" for s in volume["structure"]):
                    report.append((scale, bone, "NO MEMBER"))
    return report


def main():
    text = open(CATALOG, encoding="utf-8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    report = convert(document["asset"])
    load("coarse", "rework-wheelchair-user-coarse.py").apply(document["asset"])
    for row in report:
        if "--quiet" not in sys.argv or "NO MEMBER" in row:
            print(*row)
    if "--apply" in sys.argv:
        for name in [k for k in before if k != "volumes" and before[k] != document["asset"][k]]:
            a, b = PUBLISH["member_range"](text, name)
            text = text[:a] + PUBLISH["dump"](document["asset"][name]) + text[b:]
            print("replaced", name)
        open(CATALOG, "w", encoding="utf-8").write(text)
        open(SOURCE, "w", encoding="utf-8").write(json.dumps({"volumes": document["asset"]["volumes"]}, indent=1, ensure_ascii=False) + "\n")
    else:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        open(CANDIDATE, "w", encoding="utf-8").write(json.dumps(document, indent=1, ensure_ascii=False) + "\n")
    for bone in document["asset"]["rig"]["bones"]:
        if bone["id"] in ("lumbar", "torso", "clavicle-l", "clavicle-r", "arm-l", "ilium-l", "ilium-r", "thigh-l", "neck"):
            print(json.dumps({k: v for k, v in bone.items() if k not in ("title", "mass", "joint")}))


main()
