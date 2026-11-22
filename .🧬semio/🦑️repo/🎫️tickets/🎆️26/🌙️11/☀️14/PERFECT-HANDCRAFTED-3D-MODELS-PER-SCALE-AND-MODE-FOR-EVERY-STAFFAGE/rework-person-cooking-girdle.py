"""🦴 person-cooking: shoulder and pelvic girdle on the lateral rest placement (see `rig-lateral-rest.md`).

Adds `clavicle-l/-r` (sternoclavicular to acromioclavicular joint) under `torso` and `ilium-l/-r` (sacroiliac joint to hip joint) under `pelvis`, hangs the arms and thighs from their tips without moving them, re-expresses the girdle solids of every scale in the new bone frames, models the clavicle as its S-curve and gives 1:500 a girdle member.

Usage: python3 rework-person-cooking-girdle.py            candidate `🗑️generated/rework-person-cooking/person-cooking/🔣️.json`
       python3 rework-person-cooking-girdle.py --apply    writes `rig` and `volumes` of the catalog file and the volumes source
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "person-cooking", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "person-cooking.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-person-cooking", "person-cooking", "🔣️.json")

spec = importlib.util.spec_from_file_location("lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)

STERNOCLAVICULAR = (0.045, 1.417, 0.02)
ACROMIOCLAVICULAR = (-0.012, 1.427, 0.15)
SACROILIAC = (-0.035, 0.985, 0.045)
CLAVICLE_CURVE = [((0.045, 1.417, 0.02), 0.0085), ((0.053, 1.4185, 0.048), 0.0068), ((0.05, 1.421, 0.082), 0.006), ((0.032, 1.424, 0.112), 0.006), ((0.008, 1.4265, 0.136), 0.0064), ((-0.012, 1.427, 0.15), 0.0072)]
GIRDLE_500 = [((0.04, 1.417, 0.03), 0.0378), ((0.02, 1.424, 0.1), 0.0378), ((-0.012, 1.427, 0.15), 0.0378)]
SHOULDER = ("clavicle", "scapula", "scapular-spine", "scapular-neck", "glenoid", "shoulder-girdle")
HIP = ("hip-bone", "pubic-ramus", "ischial-ramus", "acetabulum")
TITLES = {
    "clavicle-l": {"en": "Left clavicle (shoulder girdle)", "de": "Linkes Schlüsselbein (Schultergürtel)"},
    "clavicle-r": {"en": "Right clavicle (shoulder girdle)", "de": "Rechtes Schlüsselbein (Schultergürtel)"},
    "ilium-l": {"en": "Left hip bone (pelvic girdle)", "de": "Linkes Hüftbein (Beckengürtel)"},
    "ilium-r": {"en": "Right hip bone (pelvic girdle)", "de": "Rechtes Hüftbein (Beckengürtel)"},
}

SPINE_Y = [(-0.01, -0.002), (0.09, -0.016), (0.19, 0.004), (0.3, 0.031), (0.4, 0.026), (0.46705, 0.0)]
STERNUM_Y = [(0.412, -0.07), (0.236, -0.108)]
HOOPS = {
    "100": (0.0078, 0.014, [
        ("rib-1", 0.425, [(0.401, 0.02, 0.066), (0.377, -0.06, 0.0442)], (0.37, None, 0.016)),
        ("rib-2", 0.378, [(0.338, 0.02, 0.095), (0.298, -0.06, 0.0637)], (0.286, None, 0.016)),
        ("rib-3", 0.328, [(0.2832, 0.02, 0.1048), (0.2384, -0.06, 0.0702)], (0.24, None, 0.016)),
        ("rib-4", 0.282, [(0.237, 0.02, 0.1106), (0.2, -0.06, 0.074)], (0.232, -0.103, 0.014)),
        ("rib-5", 0.236, [(0.198, 0.02, 0.1106), (0.165, -0.055, 0.078)], (0.216, -0.082, 0.044)),
        ("rib-6", 0.194, [(0.174, 0.02, 0.097), (0.154, -0.05, 0.07)], (0.19, -0.068, 0.061)),
    ]),
    "200": (0.0153, 0.016, [
        ("rib-1", 0.39, [(0.354, 0.02, 0.092), (0.318, -0.06, 0.062)], (0.307, None, 0.02)),
        ("rib-2", 0.308, [(0.263, 0.02, 0.108), (0.218, -0.06, 0.072)], (0.24, -0.104, 0.02)),
        ("rib-3", 0.23, [(0.192, 0.02, 0.11), (0.16, -0.055, 0.076)], (0.229, -0.082, 0.046)),
    ]),
}
HIP_Z = {0.118: 0.098, 0.12: 0.1, 0.112: 0.097, 0.098: 0.091, 0.116: 0.096, 0.108: 0.094}


def along(table, x):
    rows = sorted(table)
    for (x0, y0), (x1, y1) in zip(rows, rows[1:]):
        if x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return rows[-1][1]


def hoop(scale, x0, middle, front, sign):
    radius, beside, _ = HOOPS[scale]
    reach = max(point[2] for point in middle)
    points = [(x0, along(SPINE_Y, x0) + 0.012, beside), (x0 + 0.002, 0.07, 0.45 * reach), *middle, (front[0], along(STERNUM_Y, front[0]) - 0.002 if front[1] is None else front[1], front[2])]
    return [{"at": [x, y, sign * z], "rx": radius, "ry": radius} for x, y, z in points]


def chisel(volumes):
    """The hand corrections in the frames of torso, pelvis and limbs, before the girdle solids move to their bones."""
    for scale, entry in volumes.items():
        solids = {solid["id"]: solid for name in ("body", "structure") for solid in entry[name]}
        for s, sign in (("l", 1), ("r", -1)):
            hip = solids[f"hip-bone-{s}"]
            for section in hip["sections"]:
                section["at"][2] = sign * HIP_Z.get(abs(section["at"][2]), abs(section["at"][2]))
            if scale in HOOPS:
                for stem, x0, middle, front in HOOPS[scale][2]:
                    solids[f"{stem}-{s}"]["sections"] = hoop(scale, x0, middle, front, sign)
                solids[f"femur-{s}"]["sections"][-1]["at"][0] = 0.416
                solids[f"tibia-{s}"]["sections"][0]["at"][0] = -0.004
                solids[f"humerus-{s}"]["sections"][-1]["at"][0] = 0.3135
            if scale == "100":
                solids[f"patella-{s}"]["transform"]["translation"][1] = 0.03
            if scale == "50":
                solids[f"patella-{s}"]["transform"]["translation"][1] = 0.034
                solids[f"femoral-condyles-{s}"]["transform"]["translation"][0] = 0.409
                solids[f"humeral-condyles-{s}"]["radii"][0] = 0.017
                solids[f"humeral-condyles-{s}"]["transform"]["translation"][0] = 0.3125
                solids[f"humerus-{s}"]["sections"][-1]["at"][0] = 0.3
                solids[f"ulna-{s}"]["sections"][0]["at"][:2] = [-0.018, -0.01]
                solids[f"scapular-spine-{s}"]["sections"][-1]["at"] = [0.4255, -0.004, sign * 0.151]
                solids[f"drawstring-tip-{s}"]["transform"]["translation"][0] = 0.283
                at = next(i for i, x in enumerate(entry["structure"]) if x["id"] == f"glenoid-{s}")
                if f"scapular-neck-{s}" not in solids:
                    entry["structure"].insert(at, {"id": f"scapular-neck-{s}", "kind": "loft", "bone": "torso", "material": "bone", "role": "body", "tone": 0.92, "sections": [{"at": [0.403, 0.056, sign * 0.122], "rx": 0.01, "ry": 0.01}, {"at": [0.404, 0.02, sign * 0.132], "rx": 0.009, "ry": 0.009}, {"at": [0.404, -0.012, sign * 0.134], "rx": 0.0095, "ry": 0.0095}], "capStart": True, "capEnd": True, "member": "torso", "part": "member"})
        if scale == "100":
            for level in ("bottom", "middle", "top"):
                handle = solids[f"handle-{level}"]
                handle["p0"][0] = handle["p1"][0] = 0.015
        if scale == "500" and "spoon-handle" not in solids:
            entry["body"].append({"id": "spoon-handle", "kind": "capsule", "bone": "spoon", "material": "beech-wood", "role": "body", "tone": 0.4, "p0": [-0.079, 0, 0], "p1": [0.209, 0, 0], "r0": 0.006, "r1": 0.006, "slender": True})


def side(point, s):
    return [point[0], point[1], point[2] if s == "l" else -point[2]]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def local(frame, point):
    return K.r5(K.apply(K.transpose(frame[1]), sub(point, frame[0])))


def rebound(solid, old, new, bone):
    turn = K.mul(K.transpose(new[1]), old[1])
    shift = K.apply(K.transpose(new[1]), sub(old[0], new[0]))
    q = K.quaternion(turn)

    def point(p):
        moved = K.apply(turn, p)
        return K.r5([moved[i] + shift[i] for i in range(3)])

    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = point(solid["p0"]), point(solid["p1"])
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = point(section["at"])
            if "rotation" in section:
                section["rotation"] = K.r5(K.product(q, section["rotation"]))
    elif solid["kind"] == "ellipsoid":
        transform = solid["transform"]
        transform["translation"] = point(transform["translation"])
        transform["rotation"] = K.r5(K.product(q, transform.get("rotation", [0, 0, 0, 1])))
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} is not converted")
    solid["bone"] = bone
    if "member" in solid:
        solid["member"] = bone


def curve(points, frame, s):
    return [{"at": local(frame, side(p, s)), "rx": r, "ry": r} for p, r in points]


def convert(asset, volume_sets):
    bones = asset["rig"]["bones"]
    by_id = {bone["id"]: bone for bone in bones}
    if "clavicle-l" in by_id:
        return False
    old = K.frames(copy.deepcopy(bones))
    out = []
    for bone in bones:
        for s in ("l", "r"):
            axis = (-1.0, 0.0, 0.0) if s == "l" else (1.0, 0.0, 0.0)
            if bone["id"] == f"arm-{s}":
                a, b = side(STERNOCLAVICULAR, s), side(ACROMIOCLAVICULAR, s)
                placed = K.placed(old["torso"], a, sub(b, a), axis)
                out.append({"id": f"clavicle-{s}", "parent": "torso", "title": TITLES[f"clavicle-{s}"], **placed, "lengthM": round(math.dist(a, b), 5), "joint": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40 if s == "l" else -5, "twistMaxDeg": 5 if s == "l" else 40}})
            if bone["id"] == f"thigh-{s}":
                a, b = side(SACROILIAC, s), old[f"thigh-{s}"][0]
                placed = K.placed(old["pelvis"], a, sub(b, a), axis)
                out.append({"id": f"ilium-{s}", "parent": "pelvis", "title": TITLES[f"ilium-{s}"], **placed, "lengthM": round(math.dist(a, b), 5), "joint": {"type": "revolute", "minDeg": -3, "maxDeg": 3}})
        out.append(bone)
    asset["rig"]["bones"] = out
    new = K.frames(copy.deepcopy(out))
    for limb, girdle in (("arm", "clavicle"), ("thigh", "ilium")):
        for s in ("l", "r"):
            bone = by_id[f"{limb}-{s}"]
            bone["parent"] = f"{girdle}-{s}"
            bone.update(K.fields(new[f"{girdle}-{s}"], old[f"{limb}-{s}"][0], old[f"{limb}-{s}"][1]))
    check = K.frames(copy.deepcopy(out))
    for limb in ("arm-l", "arm-r", "thigh-l", "thigh-r"):
        drift = max(abs(check[limb][0][i] - old[limb][0][i]) for i in range(3))
        turn = max(abs(check[limb][1][i][j] - old[limb][1][i][j]) for i in range(3) for j in range(3))
        assert drift < 2e-5 and turn < 2e-5, (limb, drift, turn)
    for volumes in volume_sets:
        chisel(volumes)
        for scale, entry in volumes.items():
            for name in ("body", "thermal", "structure"):
                for solid in entry[name]:
                    stem, s = solid["id"][:-2], solid["id"][-1]
                    if solid["id"][-2:] not in ("-l", "-r"):
                        continue
                    if stem in SHOULDER and solid.get("bone") == "torso":
                        rebound(solid, old["torso"], new[f"clavicle-{s}"], f"clavicle-{s}")
                        if stem == "clavicle":
                            solid["sections"] = curve(CLAVICLE_CURVE, new[f"clavicle-{s}"], s)
                    elif stem in HIP and solid.get("bone") == "pelvis":
                        rebound(solid, old["pelvis"], new[f"ilium-{s}"], f"ilium-{s}")
            if scale == "500" and not any(x["id"] == "shoulder-girdle-l" for x in entry["structure"]):
                at = next(i for i, x in enumerate(entry["structure"]) if x["id"] == "joint-lumbosacral")
                for s in ("r", "l"):
                    entry["structure"].insert(at, {"id": f"shoulder-girdle-{s}", "kind": "loft", "bone": f"clavicle-{s}", "material": "bone", "role": "body", "tone": 0.92, "sections": curve(GIRDLE_500, new[f"clavicle-{s}"], s), "capStart": True, "capEnd": True, "member": f"clavicle-{s}", "part": "member"})
    return True


def main():
    with open(CATALOG, encoding="utf-8") as handle:
        text = handle.read()
    document = json.loads(text)
    with open(SOURCE, encoding="utf-8") as handle:
        source = json.load(handle)
    if not convert(document["asset"], [document["asset"]["volumes"], source["volumes"]]):
        raise SystemExit("already converted")
    if "--apply" in sys.argv:
        indent = 2 if text.startswith("{\n  \"") else None
        with open(CATALOG, "w", encoding="utf-8") as handle:
            handle.write(json.dumps(document, ensure_ascii=False, indent=indent) + ("\n" if text.endswith("\n") else ""))
        with open(SOURCE, "w", encoding="utf-8") as handle:
            json.dump(source, handle, ensure_ascii=False)
        print("applied")
    else:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        with open(CANDIDATE, "w", encoding="utf-8") as handle:
            json.dump(document, handle, ensure_ascii=False)
        print(CANDIDATE)
    for bone in document["asset"]["rig"]["bones"]:
        if bone["id"][:-2] in ("clavicle", "ilium", "arm", "thigh"):
            print(json.dumps({k: v for k, v in bone.items() if k not in ("title", "mass")}))


if __name__ == "__main__":
    main()
