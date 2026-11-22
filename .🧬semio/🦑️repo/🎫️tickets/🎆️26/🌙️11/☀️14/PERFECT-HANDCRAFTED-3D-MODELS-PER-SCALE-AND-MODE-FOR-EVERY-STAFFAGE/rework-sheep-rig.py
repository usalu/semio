"""🐑 Sheep rig on full 3D rest placement: scapulae and hip bones run laterally, every limb bone runs joint centre to joint centre, the digit is split into pastern and claw; all solids, anchors, poses and 2D bindings are re-bound so the world model does not move.

Usage: python3 rework-sheep-rig.py [--write]
Without --write the candidate goes to 🗑️generated/rework-sheep/sheep/🔣️.json only; with --write the volumes source and the catalog file are written too.
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/sheep/🔣️.json"
SOURCE = os.path.join(T, "volumes", "animals", "sheep.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-sheep", "sheep", "🔣️.json")


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


K = load("rig-lateral-standing-adult")


class INSERT:
    """🧵 The catalog formatter of `volumes-insert.py` (its definitions before the splice code)."""

    scope = {}
    exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0], scope)
    dump = staticmethod(scope["dump"])
    member_range = staticmethod(scope["member_range"])

SIDES = {"n": -1.0, "f": 1.0}
LIMB_Z = 0.095
SCAPULA_DORSAL_Z, SHOULDER_Z, ELBOW_Z, CARPUS_Z, FORE_FETLOCK_Z, FORE_TOE_Z = 0.050, LIMB_Z, LIMB_Z, LIMB_Z, LIMB_Z, LIMB_Z
SACROILIAC_X, SACROILIAC_Z, HIP_Z, STIFLE_Z, HOCK_Z, HIND_FETLOCK_Z, HIND_TOE_Z = 0.095, 0.035, LIMB_Z, LIMB_Z, LIMB_Z, LIMB_Z, LIMB_Z
MODELLED_Z = {"thigh-h": (0.088, 0.115), "shank-h": (0.115, 0.097), "metatarsus-h": (0.097, 0.103), "hoof-h": (0.103, 0.104), "humerus-f": (0.095, 0.105), "forearm-f": (0.105, 0.088), "metacarpus-f": (0.088, 0.090), "hoof-f": (0.090, 0.090)}
PASTERN_FRACTION = {"h": 0.070 / 0.1101, "f": 0.068 / 0.107}
TITLES = {
    "ilium-hn": {"en": "Near hip bone (sacroiliac joint to hip joint)", "de": "Nahes Hüftbein (Kreuzdarmbeingelenk bis Hüftgelenk)"},
    "ilium-hf": {"en": "Far hip bone (sacroiliac joint to hip joint)", "de": "Fernes Hüftbein (Kreuzdarmbeingelenk bis Hüftgelenk)"},
    "pastern-hn": {"en": "Near hind pastern (first and second phalanx)", "de": "Nahe Hinterfessel (Fessel- und Kronbein)"},
    "pastern-hf": {"en": "Far hind pastern (first and second phalanx)", "de": "Ferne Hinterfessel (Fessel- und Kronbein)"},
    "pastern-fn": {"en": "Near fore pastern (first and second phalanx)", "de": "Nahe Vorderfessel (Fessel- und Kronbein)"},
    "pastern-ff": {"en": "Far fore pastern (first and second phalanx)", "de": "Ferne Vorderfessel (Fessel- und Kronbein)"},
}
HOOF_TITLES = {
    "hoof-hn": {"en": "Near hind claws (third phalanx in the horn capsule)", "de": "Nahe Hinterklauen (Klauenbein in der Hornkapsel)"},
    "hoof-hf": {"en": "Far hind claws (third phalanx in the horn capsule)", "de": "Ferne Hinterklauen (Klauenbein in der Hornkapsel)"},
    "hoof-fn": {"en": "Near fore claws (third phalanx in the horn capsule)", "de": "Nahe Vorderklauen (Klauenbein in der Hornkapsel)"},
    "hoof-ff": {"en": "Far fore claws (third phalanx in the horn capsule)", "de": "Ferne Vorderklauen (Klauenbein in der Hornkapsel)"},
}


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def norm(a):
    return math.sqrt(sum(c * c for c in a))


def tip(frame, length):
    return [frame[0][i] + frame[1][i][0] * length for i in range(3)]


def lateral(point, z):
    return [point[0], point[1], z]


def straighten(bone, length):
    """The lateral shift that brings the joint centres of the modelled limb onto the straight column of the bone, by the station along the old bone."""
    span = MODELLED_Z.get(bone[:-1])
    if span is None:
        return lambda station: 0.0
    sign = SIDES[bone[-1]]
    return lambda station: sign * (LIMB_Z - (span[0] + (span[1] - span[0]) * station / length))


def rebind(solid, old, new, shift_at=lambda station: 0.0):
    turn = K.mul(K.transpose(new[1]), old[1])
    shift = K.apply(K.transpose(new[1]), sub(old[0], new[0]))
    q = K.quaternion(turn)

    def point(p):
        moved = K.apply(turn, [p[0], p[1], p[2] + shift_at(p[0])])
        return K.r5([moved[i] + shift[i] for i in range(3)])

    if solid.get("kind") == "loft":
        for section in solid["sections"]:
            section["at"] = point(section["at"])
            section["rotation"] = K.r5(K.product(q, section.get("rotation", [0, 0, 0, 1])))
    elif solid.get("kind") == "polyhedron":
        solid["vertices"] = [point(p) for p in solid["vertices"]]
    elif solid.get("kind") in ("ellipsoid", "extrude", "revolve", "box", "cylinder", "capsule") and "p0" not in solid:
        transform = solid.setdefault("transform", {})
        transform["translation"] = point(transform.get("translation", [0, 0, 0]))
        transform["rotation"] = K.r5(K.product(q, transform.get("rotation", [0, 0, 0, 1])))
    elif solid.get("kind") == "capsule":
        solid["p0"], solid["p1"] = point(solid["p0"]), point(solid["p1"])
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid.get('kind')} on {solid.get('bone')} is not converted")


def convert(asset, volumes):
    bones = asset["rig"]["bones"]
    by_id = {bone["id"]: bone for bone in bones}
    old = K.frames(bones)
    old_length = {bone["id"]: bone["lengthM"] for bone in bones}
    new = dict(old)
    plan = []
    for side, sign in SIDES.items():
        fore = [f"scapula-f{side}", f"humerus-f{side}", f"forearm-f{side}", f"metacarpus-f{side}", f"hoof-f{side}"]
        fore_z = [SCAPULA_DORSAL_Z, SHOULDER_Z, ELBOW_Z, CARPUS_Z, FORE_FETLOCK_Z, FORE_TOE_Z]
        hind = [f"thigh-h{side}", f"shank-h{side}", f"metatarsus-h{side}", f"hoof-h{side}"]
        hind_z = [HIP_Z, STIFLE_Z, HOCK_Z, HIND_FETLOCK_Z, HIND_TOE_Z]
        pelvis = old["pelvis"]
        sacroiliac = [pelvis[0][i] + pelvis[1][i][0] * SACROILIAC_X for i in range(3)]
        plan.append((f"ilium-h{side}", "pelvis", lateral(sacroiliac, sign * SACROILIAC_Z), lateral(old[hind[0]][0], sign * HIP_Z), None))
        for chain, zs, first_parent in ((fore, fore_z, "thorax"), (hind, hind_z, f"ilium-h{side}")):
            parent = first_parent
            for index, bone in enumerate(chain):
                a = lateral(old[bone][0], sign * zs[index])
                b = lateral(tip(old[bone], old_length[bone]), sign * zs[index + 1])
                if bone.startswith("hoof"):
                    limb = bone[5]
                    mid = [a[i] + (b[i] - a[i]) * PASTERN_FRACTION[limb] for i in range(3)]
                    pastern = "pastern-" + bone[5:]
                    plan.append((pastern, parent, a, mid, bone))
                    plan.append((bone, pastern, mid, b, None))
                else:
                    plan.append((bone, parent, a, b, None))
                parent = bone
    result = []
    for bone in bones:
        if bone["id"].startswith(("thigh-h", "hoof-")):
            side = bone["id"][-2:]
            fresh = "ilium-" + side if bone["id"].startswith("thigh") else "pastern-" + side
            template = by_id[bone["id"]]
            created = {"id": fresh, "parent": None, "title": TITLES[fresh], "x": 0, "y": 0, "z": 0, "angleDeg": 0, "abductionDeg": 0, "twistDeg": 0, "lengthM": 0, "joint": copy.deepcopy(template["joint"])}
            if fresh.startswith("ilium"):
                created["joint"] = {"type": "revolute", "minDeg": -3, "maxDeg": 3}
            else:
                kg = template["mass"]["kg"]
                created["mass"] = dict(template["mass"], kg=round(kg * 0.57, 3))
                template["mass"] = dict(template["mass"], kg=round(kg - round(kg * 0.57, 3), 3))
                template["joint"] = {"type": "revolute", "minDeg": -15, "maxDeg": 25}
                template["title"] = HOOF_TITLES[bone["id"]]
            by_id[fresh] = created
            result.append(created)
        result.append(bone)
    for bone_id, parent, a, b, _ in plan:
        bone = by_id[bone_id]
        length = norm(sub(b, a))
        placed = K.placed(new[parent], a, sub(b, a), (0.0, 0.0, 1.0))
        bone["parent"] = parent
        bone.update(placed)
        bone["lengthM"] = round(length, 5)
        if "mass" in bone and bone_id.startswith(("pastern", "hoof")):
            bone["mass"]["comX"], bone["mass"]["comY"] = round(length / 2, 4), 0
        ordered = {}
        for key in ("id", "parent", "title", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg", "lengthM"):
            ordered[key] = bone[key]
        for key, value in bone.items():
            ordered.setdefault(key, value)
        bone.clear()
        bone.update(ordered)
        solved = K.frames([by_id[i] for i in by_id if i in new or i == bone_id] if False else chain_of(by_id, bone_id))
        new[bone_id] = solved[bone_id]
    asset["rig"]["bones"] = result
    for scales in volumes:
        for scale in scales.values():
            for name in ("body", "thermal", "structure"):
                extra = []
                for solid in scale.get(name, []):
                    bone = solid.get("bone")
                    if bone is None or "kind" not in solid:
                        continue
                    target = bone
                    if bone == "pelvis" and ("ilium" in solid["id"] or "hip-bone" in solid["id"]):
                        target = "ilium-h" + ("n" if solid["id"].startswith("near") else "f")
                    elif bone.startswith("hoof-") and not any(word in solid["id"] for word in ("claw", "pedal")):
                        target = "pastern-" + bone[5:]
                    if target == bone and old.get(bone) == new.get(bone):
                        continue
                    rebind(solid, old[bone], new[target], straighten(bone, old_length[bone]))
                    solid["bone"] = target
                    if solid.get("member") == bone:
                        solid["member"] = target
                    if solid["id"] == "joint-" + bone and target != bone:
                        solid["id"] = "joint-" + target
                        coffin = copy.deepcopy(solid)
                        coffin["id"], coffin["bone"] = "joint-" + bone, bone
                        if "member" in coffin:
                            coffin["member"] = bone
                        coffin["transform"]["translation"] = [0.0, 0.0, 0.0]
                        extra.append(coffin)
                    if solid["id"] == "joint-" + bone and bone.startswith("thigh-h"):
                        girdle = copy.deepcopy(solid)
                        ilium = "ilium-" + bone[6:]
                        girdle["id"], girdle["bone"] = "joint-" + ilium, ilium
                        if "member" in girdle:
                            girdle["member"] = ilium
                        girdle["transform"]["translation"] = [0.0, 0.0, 0.0]
                        extra.append(girdle)
                scale[name].extend(extra)
    for anchor in asset["anchors"]:
        if anchor["bone"].startswith("hoof-"):
            anchor["x"] = by_id[anchor["bone"]]["lengthM"]
    for pose in asset["poses"]:
        pose["bones"] = {("pastern-" + key[5:] if key.startswith("hoof-") else key): value for key, value in pose["bones"].items()}
    for clip in asset["clips"]:
        for key in clip["keys"]:
            if "bones" in key:
                key["bones"] = {("pastern-" + name[5:] if name.startswith("hoof-") else name): value for name, value in key["bones"].items()}
    for representation in asset["representations"].values():
        for view, shapes in representation.get("geometry", {}).items():
            for shape in shapes:
                span = MODELLED_Z.get(shape["bone"][:-1])
                if span is not None and view in ("front", "plan"):
                    if shape["kind"] != "poly":
                        raise SystemExit(f"shape {shape['id']} of kind {shape['kind']} is not shifted")
                    sign = SIDES[shape["bone"][-1]]
                    top = old[shape["bone"]][0][1]
                    foot = tip(old[shape["bone"]], old_length[shape["bone"]])[1]
                    for point in shape["points"]:
                        if view == "front":
                            station = max(-0.3, min(1.3, (point[1] - top) / (foot - top)))
                            point[0] = round(point[0] + sign * (LIMB_Z - (span[0] + (span[1] - span[0]) * station)), 5)
                        else:
                            point[1] = round(point[1] + sign * (LIMB_Z - (span[0] + span[1]) / 2), 5)
                if shape["bone"].startswith("hoof-"):
                    shape["bone"] = "pastern-" + shape["bone"][5:]
    return old, new


def chain_of(by_id, bone_id):
    chain = []
    while bone_id is not None:
        chain.append(by_id[bone_id])
        bone_id = by_id[bone_id]["parent"]
    return chain[::-1]


MEMBERS = ("rig", "anchors", "representations", "poses", "clips", "volumes")


def main():
    text = open(CATALOG, encoding="utf-8").read()
    document = json.loads(text)
    round_trip = INSERT.dump(document, 0) + "\n" == text
    print("[DEBUG] catalog formatting round-trips:", round_trip)
    source = json.load(open(SOURCE, encoding="utf-8"))
    asset = document["asset"]
    if any(bone["id"].startswith("pastern") for bone in asset["rig"]["bones"]):
        raise SystemExit("rig is already converted")
    old, new = convert(asset, [asset["volumes"], source["volumes"]])
    for bone in asset["rig"]["bones"]:
        if bone["id"][-2:] in ("hn", "fn") or bone["id"] in ("pelvis",):
            print("[DEBUG]", bone["id"], bone["parent"], bone["x"], bone["y"], bone.get("z"), bone["angleDeg"], bone.get("abductionDeg"), bone.get("twistDeg"), bone["lengthM"])
    indent = "    "
    result = text
    for member in MEMBERS:
        start, end = INSERT.member_range(result, indent, member)
        result = result[:start] + indent + json.dumps(member) + ": " + INSERT.dump(asset[member], 2) + result[end:]
    check, before = json.loads(result), json.loads(text)
    if check != document or list(check["asset"]) != list(before["asset"]):
        raise SystemExit("splice did not round-trip")
    if any(check["asset"][key] != before["asset"][key] for key in before["asset"] if key not in MEMBERS):
        raise SystemExit("splice changed other members")
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    open(CANDIDATE, "w", encoding="utf-8").write(result)
    if "--write" in sys.argv:
        if open(CATALOG, encoding="utf-8").read() != text:
            raise SystemExit("catalog file changed meanwhile; run again")
        open(SOURCE, "w", encoding="utf-8").write('{"volumes":' + INSERT.dump(source["volumes"], 0) + "}\n")
        open(CATALOG, "w", encoding="utf-8").write(result)
        print("[DEBUG] written")


main()
