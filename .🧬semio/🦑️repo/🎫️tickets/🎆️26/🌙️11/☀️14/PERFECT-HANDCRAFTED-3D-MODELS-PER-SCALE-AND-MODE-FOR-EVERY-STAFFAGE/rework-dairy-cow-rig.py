"""🦴 Dairy cow rig chisel: raises the hip joint, opens stifle, hock, shoulder and elbow to the standing angles of cattle, adds the four pastern bones and re-binds poses, anchors, solids and 2D shapes.

Usage: python3 rework-dairy-cow-rig.py <asset document in> <asset document out>
Hand-chosen numbers: hip joint +0.06 m (1.19 m), femur 0.42 m, tibia 0.385 m, scapula 0.44 m at 61 degrees, humerus 0.32 m, pastern 0.10 m, coffin-to-toe 0.05 m.
Every pose keeps its hock, carpus, cannon and digit where they were against the spine (fore limb moved 73 mm ahead as a whole), solved by two-bone inverse kinematics.
"""
import copy
import json
import math
import sys

HIP_RAISE = 0.06
FEMUR, TIBIA = 0.42, 0.385
SCAPULA, SCAPULA_WORLD, HUMERUS = 0.44, -61.0, 0.32
PASTERN, TOE = 0.10, 0.05
SIDES_HIND, SIDES_FORE = ("hn", "hf"), ("fn", "ff")
PASTERN_TITLES = {
    "hn": ("Near hind pastern (fetlock to coffin joint)", "Nahe Hinterfessel (Fesselgelenk bis Klauengelenk)"),
    "hf": ("Far hind pastern (fetlock to coffin joint)", "Ferne Hinterfessel (Fesselgelenk bis Klauengelenk)"),
    "fn": ("Near fore pastern (fetlock to coffin joint)", "Nahe Vorderfessel (Fesselgelenk bis Klauengelenk)"),
    "ff": ("Far fore pastern (fetlock to coffin joint)", "Ferne Vorderfessel (Fesselgelenk bis Klauengelenk)"),
}
HOOF_TITLES = {
    "hn": ("Near hind cloven hoof", "Nahe Hinterklaue"),
    "hf": ("Far hind cloven hoof", "Ferne Hinterklaue"),
    "fn": ("Near fore cloven hoof", "Nahe Vorderklaue"),
    "ff": ("Far fore cloven hoof", "Ferne Vorderklaue"),
}


def rad(value):
    return value * math.pi / 180


def deg(value):
    return value * 180 / math.pi


def norm(angle):
    while angle > 180:
        angle -= 360
    while angle <= -180:
        angle += 360
    return angle


def forward(bones, pose):
    world = {}
    for bone in bones:
        delta = pose.get(bone["id"], {})
        if bone["parent"] is None:
            px, py, pa = 0.0, 0.0, 0.0
        else:
            px, py, pa = world[bone["parent"]]
        x, y = bone["x"] + delta.get("dx", 0), bone["y"] + delta.get("dy", 0)
        c, s = math.cos(rad(pa)), math.sin(rad(pa))
        world[bone["id"]] = (px + c * x - s * y, py + s * x + c * y, pa + bone["angleDeg"] + delta.get("angleDeg", 0))
    return world


def tip(entry, length):
    return (entry[0] + length * math.cos(rad(entry[2])), entry[1] + length * math.sin(rad(entry[2])))


def two_bone(origin, target, first, second, near):
    dx, dy = target[0] - origin[0], target[1] - origin[1]
    reach = math.hypot(dx, dy)
    clamped = min(max(reach, abs(first - second) + 1e-6), first + second - 1e-6)
    inner = math.acos((first * first + clamped * clamped - second * second) / (2 * first * clamped))
    base = deg(math.atan2(dy, dx))
    options = []
    for sign in (1, -1):
        a = base + sign * deg(inner)
        ex, ey = origin[0] + first * math.cos(rad(a)), origin[1] + first * math.sin(rad(a))
        b = deg(math.atan2(origin[1] + dy * clamped / reach - ey, origin[0] + dx * clamped / reach - ex))
        options.append((abs(norm(a - near)), a, b))
    options.sort()
    return options[0][1], options[0][2], reach - clamped


def rotate_z(quaternion, degrees):
    half = rad(degrees) / 2
    qx, qy, qz, qw = 0.0, 0.0, math.sin(half), math.cos(half)
    x, y, z, w = quaternion
    return [round(v, 6) for v in (qw * x + qx * w + qy * z - qz * y, qw * y - qx * z + qy * w + qz * x, qw * z + qx * y - qy * x + qz * w, qw * w - qx * x - qy * y - qz * z)]


def main():
    source, target = sys.argv[1], sys.argv[2]
    document = json.load(open(source, encoding="utf-8"))
    asset = document["asset"]
    old_bones = copy.deepcopy(asset["rig"]["bones"])
    old = {bone["id"]: bone for bone in old_bones}
    bones = asset["rig"]["bones"]
    by = {bone["id"]: bone for bone in bones}
    old_rest = forward(old_bones, {})

    pelvis = by["pelvis"]
    old_angle, old_length = pelvis["angleDeg"], pelvis["lengthM"]
    tx, ty = old_length * math.cos(rad(old_angle)), old_length * math.sin(rad(old_angle)) - HIP_RAISE
    new_length, new_angle = round(math.hypot(tx, ty), 5), round(deg(math.atan2(ty, tx)), 5)
    turn = old_angle - new_angle

    def pelvic(x, y):
        wx, wy = x * math.cos(rad(old_angle)) - y * math.sin(rad(old_angle)), x * math.sin(rad(old_angle)) + y * math.cos(rad(old_angle)) - HIP_RAISE
        c, s = math.cos(rad(-new_angle)), math.sin(rad(-new_angle))
        return round(c * wx - s * wy, 5), round(s * wx + c * wy, 5)

    pelvis["angleDeg"], pelvis["lengthM"] = new_angle, new_length
    pelvis["mass"]["comX"], pelvis["mass"]["comY"] = pelvic(pelvis["mass"]["comX"], pelvis["mass"]["comY"])
    by["lumbar"]["x"], by["lumbar"]["angleDeg"] = new_length, -new_angle
    for child in ("tail-1", "udder"):
        by[child]["x"], by[child]["y"] = pelvic(by[child]["x"], by[child]["y"])
        by[child]["angleDeg"] = round(by[child]["angleDeg"] + turn, 5)

    hock_rest = (old_rest["metatarsus-hn"][0], old_rest["metatarsus-hn"][1] - HIP_RAISE)
    femur_world, tibia_world, _ = two_bone((0, 0), hock_rest, FEMUR, TIBIA, old_rest["thigh-hn"][2])
    for side in SIDES_HIND:
        thigh, shank, cannon = by["thigh-" + side], by["shank-" + side], by["metatarsus-" + side]
        thigh["lengthM"], thigh["angleDeg"] = FEMUR, round(norm(femur_world - new_angle), 5)
        thigh["mass"]["comX"] = round(thigh["mass"]["comX"] * FEMUR / old["thigh-" + side]["lengthM"], 5)
        shank["x"], shank["lengthM"], shank["angleDeg"] = FEMUR, TIBIA, round(norm(tibia_world - femur_world), 5)
        cannon["x"], cannon["angleDeg"] = TIBIA, round(norm(old_rest["metatarsus-" + side][2] - tibia_world), 5)

    scapula_old = old_rest["scapula-fn"]
    thorax_world = old_rest["thorax"][2]
    shoulder = tip((scapula_old[0], scapula_old[1], SCAPULA_WORLD), SCAPULA)
    elbow_old = old_rest["forearm-fn"]
    humerus_world = -180 + deg(math.asin((shoulder[1] - elbow_old[1]) / HUMERUS))
    elbow_new = tip((shoulder[0], shoulder[1], humerus_world), HUMERUS)
    fore_shift = (elbow_new[0] - elbow_old[0], elbow_new[1] - elbow_old[1])
    for side in SIDES_FORE:
        scapula, humerus, forearm = by["scapula-" + side], by["humerus-" + side], by["forearm-" + side]
        scapula["lengthM"], scapula["angleDeg"] = SCAPULA, round(SCAPULA_WORLD - thorax_world, 5)
        scapula["mass"]["comX"] = round(SCAPULA / 2, 5)
        humerus["x"], humerus["lengthM"], humerus["angleDeg"] = SCAPULA, HUMERUS, round(norm(humerus_world - SCAPULA_WORLD), 5)
        humerus["mass"]["comX"] = round(HUMERUS / 2, 5)
        forearm["x"], forearm["angleDeg"] = HUMERUS, round(norm(elbow_old[2] - humerus_world), 5)

    for side in SIDES_HIND + SIDES_FORE:
        hoof = by["hoof-" + side]
        index = bones.index(hoof)
        pastern = copy.deepcopy(hoof)
        pastern["id"] = "pastern-" + side
        pastern["title"] = {"en": PASTERN_TITLES[side][0], "de": PASTERN_TITLES[side][1]}
        pastern["lengthM"] = PASTERN
        pastern["mass"] = {**hoof["mass"], "kg": round(hoof["mass"]["kg"] / 2, 3), "comX": 0.05, "comY": 0, "distribution": "uniform"}
        hoof["parent"], hoof["x"], hoof["y"], hoof["angleDeg"], hoof["lengthM"] = pastern["id"], PASTERN, 0.0, 0.0, TOE
        hoof["title"] = {"en": HOOF_TITLES[side][0], "de": HOOF_TITLES[side][1]}
        hoof["joint"] = {"type": "revolute", "minDeg": -25, "maxDeg": 35}
        hoof["mass"] = {**hoof["mass"], "kg": round(hoof["mass"]["kg"] / 2, 3), "comX": 0.02, "comY": -0.02}
        bones.insert(index, pastern)
    by = {bone["id"]: bone for bone in bones}
    new_rest = forward(bones, {})
    spine_offset = (old_rest["lumbar"][0] - new_rest["lumbar"][0], old_rest["lumbar"][1] - new_rest["lumbar"][1])
    print("[DEBUG] pelvis", new_angle, new_length, "femur/tibia world", round(femur_world, 2), round(tibia_world, 2), "humerus world", round(humerus_world, 2), "fore shift", [round(v, 4) for v in fore_shift])

    used = {}
    limb = tuple(name + side for name in ("thigh-", "shank-", "metatarsus-") for side in SIDES_HIND) + tuple(name + side for name in ("humerus-", "forearm-", "cannon-") for side in SIDES_FORE)

    def retarget(table, explicit):
        before = forward(old_bones, table)
        for side in SIDES_HIND + SIDES_FORE:
            if "hoof-" + side in table:
                table["pastern-" + side] = table.pop("hoof-" + side)
        for bone in limb:
            if "angleDeg" in table.get(bone, {}):
                del table[bone]["angleDeg"]

        def store(values):
            for bone, value in values.items():
                if explicit or abs(value) > 0.004:
                    table.setdefault(bone, {})["angleDeg"] = round(value, 5)
                used.setdefault(bone, []).append(value)

        for side in SIDES_HIND:
            now = forward(bones, table)
            shift = (before["lumbar"][0] - now["lumbar"][0], before["lumbar"][1] - now["lumbar"][1])
            hip = now["thigh-" + side]
            goal = (before["metatarsus-" + side][0] - shift[0], before["metatarsus-" + side][1] - shift[1])
            a, b, miss = two_bone((hip[0], hip[1]), goal, FEMUR, TIBIA, before["thigh-" + side][2])
            if abs(miss) > 1e-4:
                print("[DEBUG] hind unreachable", side, round(miss, 4))
            store({"thigh-" + side: norm(a - now["pelvis"][2] - by["thigh-" + side]["angleDeg"]), "shank-" + side: norm(b - a - by["shank-" + side]["angleDeg"]), "metatarsus-" + side: norm(before["metatarsus-" + side][2] - b - by["metatarsus-" + side]["angleDeg"])})
        for side in SIDES_FORE:
            now = forward(bones, table)
            shift = (before["thorax"][0] - now["thorax"][0], before["thorax"][1] - now["thorax"][1])
            swing = rad(before["thorax"][2] - old_rest["thorax"][2])
            moved = (fore_shift[0] * math.cos(swing) - fore_shift[1] * math.sin(swing), fore_shift[0] * math.sin(swing) + fore_shift[1] * math.cos(swing))
            joint = now["humerus-" + side]
            goal = (before["cannon-" + side][0] - shift[0] + moved[0], before["cannon-" + side][1] - shift[1] + moved[1])
            a, b, miss = two_bone((joint[0], joint[1]), goal, HUMERUS, by["forearm-" + side]["lengthM"], before["humerus-" + side][2] + 10)
            if abs(miss) > 1e-4:
                print("[DEBUG] fore unreachable", side, round(miss, 4))
            store({"humerus-" + side: norm(a - now["scapula-" + side][2] - by["humerus-" + side]["angleDeg"]), "forearm-" + side: norm(b - a - by["forearm-" + side]["angleDeg"]), "cannon-" + side: norm(before["cannon-" + side][2] - b - by["cannon-" + side]["angleDeg"])})
        for bone in list(table):
            if not table[bone]:
                del table[bone]
        return table

    old_poses = {pose["id"]: copy.deepcopy(pose["bones"]) for pose in asset["poses"]}
    for pose in asset["poses"]:
        print("[DEBUG] pose", pose["id"])
        retarget(pose["bones"], False)
    new_poses = {pose["id"]: pose["bones"] for pose in asset["poses"]}
    driving = set(limb) | {"pelvis", "lumbar", "thorax"} | {"scapula-" + side for side in SIDES_FORE}
    for clip in asset["clips"]:
        for key in clip["keys"]:
            override = key.get("bones")
            if not override:
                continue
            if not driving & set(override):
                for side in SIDES_HIND + SIDES_FORE:
                    if "hoof-" + side in override:
                        override["pastern-" + side] = override.pop("hoof-" + side)
                continue
            named = {("pastern-" + bone[5:] if bone.startswith("hoof-") else bone) for bone in override}
            merged = retarget({**copy.deepcopy(old_poses[key["pose"]]), **copy.deepcopy(override)}, True)
            base = new_poses[key["pose"]]
            key["bones"] = {bone: value for bone, value in merged.items() if bone in named or bone in limb or base.get(bone) != value}
    for bone, values in used.items():
        joint = by[bone]["joint"]
        change = by[bone]["angleDeg"] - old[bone]["angleDeg"]
        low, high = joint["minDeg"] - change, joint["maxDeg"] - change
        joint["minDeg"], joint["maxDeg"] = math.floor(min(low, min(values) - 2)), math.ceil(max(high, max(values) + 2))
        print("[DEBUG] limits", bone, joint["minDeg"], joint["maxDeg"], "poses", round(min(values), 1), round(max(values), 1))

    def rebind(listing):
        fresh = []
        for entry in listing:
            bone = entry.get("bone", "")
            if bone == "pelvis":
                for section in entry.get("sections", []):
                    section["at"][0], section["at"][1] = pelvic(section["at"][0], section["at"][1])
                    if "rotation" in section:
                        section["rotation"] = rotate_z(section["rotation"], turn)
                for key in ("p0", "p1"):
                    if key in entry:
                        entry[key][0], entry[key][1] = pelvic(entry[key][0], entry[key][1])
                if "profile" in entry and entry.get("plane") == "xy":
                    entry["profile"] = [list(pelvic(x, y)) for x, y in entry["profile"]]
                elif "transform" in entry:
                    move = entry["transform"].get("translation", [0, 0, 0])
                    move[0], move[1] = pelvic(move[0], move[1])
                    entry["transform"]["translation"] = move
                    if "rotation" in entry["transform"]:
                        entry["transform"]["rotation"] = rotate_z(entry["transform"]["rotation"], turn)
            if bone.startswith("hoof-"):
                side = bone[5:]
                if "sections" in entry:
                    start = min(section["at"][0] for section in entry["sections"])
                elif "p0" in entry:
                    start = min(entry["p0"][0], entry["p1"][0])
                else:
                    start = entry.get("transform", {}).get("translation", [0])[0]
                ident = entry["id"]
                if ident.startswith("joint-hoof-"):
                    twin = copy.deepcopy(entry)
                    entry["id"], entry["bone"], entry["member"] = "joint-pastern-" + side, "pastern-" + side, "pastern-" + side
                    twin["transform"] = {"translation": [0.0, 0.0, entry["transform"]["translation"][2]]}
                    fresh.append(entry)
                    fresh.append(twin)
                    continue
                if ident.startswith("mass-hoof-"):
                    twin = copy.deepcopy(entry)
                    twin["id"], twin["bone"], twin["member"] = "mass-pastern-" + side, "pastern-" + side, "pastern-" + side
                    twin["transform"]["translation"] = [0.05, 0.0, entry["transform"]["translation"][2]]
                    entry["transform"]["translation"] = [0.02, -0.02, entry["transform"]["translation"][2]]
                    fresh.append(twin)
                    fresh.append(entry)
                    continue
                if start >= 0.06 and "dewclaw" not in ident:
                    for section in entry.get("sections", []):
                        section["at"][0] = round(section["at"][0] - PASTERN, 5)
                    for key in ("p0", "p1"):
                        if key in entry:
                            entry[key][0] = round(entry[key][0] - PASTERN, 5)
                    if "transform" in entry and "translation" in entry["transform"]:
                        entry["transform"]["translation"][0] = round(entry["transform"]["translation"][0] - PASTERN, 5)
                else:
                    entry["bone"] = "pastern-" + side
                    if "member" in entry:
                        entry["member"] = "pastern-" + side
            fresh.append(entry)
        listing[:] = fresh

    for scale in asset["volumes"].values():
        for name in ("body", "thermal", "structure"):
            rebind(scale[name])
    for anchor in asset["anchors"]:
        if anchor["bone"].startswith("hoof-"):
            anchor["x"] = round(anchor["x"] - PASTERN, 5)
        if anchor["bone"] == "pelvis":
            anchor["x"], anchor["y"] = pelvic(anchor["x"], anchor["y"])
    for representation in asset["representations"].values():
        for shape in representation["geometry"]["elevation"]:
            bone = shape.get("bone", "")
            if bone.startswith("hoof-"):
                if "points" not in shape:
                    raise SystemExit("hoof shape without points: " + shape["id"])
                shape["points"] = [[round(x - PASTERN, 4), y] for x, y in shape["points"]]
            if bone == "pelvis":
                if "points" not in shape:
                    raise SystemExit("pelvis shape without points: " + shape["id"])
                shape["points"] = [[round(v, 4) for v in pelvic(x, y)] for x, y in shape["points"]]
    with open(target, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    rest = forward(bones, {})
    ground = min(tip(rest["hoof-" + side], TOE)[1] for side in SIDES_HIND + SIDES_FORE)
    for name in ("pelvis", "shank-hn", "metatarsus-hn", "pastern-hn", "humerus-fn", "forearm-fn", "cannon-fn", "pastern-fn"):
        print("[DEBUG] rest joint", name, round(rest[name][0], 3), round(rest[name][1] - ground, 3), "world", round(norm(rest[name][2]), 1))


main()
