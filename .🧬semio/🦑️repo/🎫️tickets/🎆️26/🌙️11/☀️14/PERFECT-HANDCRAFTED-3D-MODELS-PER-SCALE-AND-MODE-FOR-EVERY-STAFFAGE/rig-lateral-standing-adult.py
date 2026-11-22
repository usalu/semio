"""🦴 standing-adult: the clavicles become lateral bones (sternoclavicular joint to acromioclavicular joint) and the upper arms hang from them, on the full 3D rest placement of the rig (see `rig-lateral-rest.md`).

What changes: the rest placement and joint of `clavicle-l/-r`, the parent-frame fields of `arm-l/-r` (their world rest frames stay where they were), the shrug of the poses and of the clip keys that inherit it (clavicle `angleDeg` instead of the former twist about the antero-posterior stub, the arm offsets solved so that the arms keep their posed direction) and the local coordinates of every solid bound to a clavicle (the same world geometry in the new bone frame). Nothing else is touched.

Usage: python3 rig-lateral-standing-adult.py            writes the candidate `🗑️generated/rig/standing-adult/🔣️.json` (catalog document + volumes source, both converted)
       python3 rig-lateral-standing-adult.py --apply    re-reads the catalog file and the volumes source and writes the converted members (`rig`, `poses`, `clips`, `volumes` of the catalog; the source), every other byte unchanged
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
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "standing-adult", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "standing-adult.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rig", "standing-adult", "🔣️.json")

ACROMIOCLAVICULAR = (0.004, 1.42, 0.165)
CLAVICLE_AXIS = {"l": (-1.0, 0.0, 0.0), "r": (1.0, 0.0, 0.0)}
CLAVICLE_JOINT = {"l": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40, "twistMaxDeg": 5}, "r": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -5, "twistMaxDeg": 40}}
DEG = 180 / math.pi


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def apply(m, p):
    return [sum(m[i][k] * p[k] for k in range(3)) for i in range(3)]


def transpose(m):
    return [[m[j][i] for j in range(3)] for i in range(3)]


def turn(angle, abduction, twist):
    a, b, t = angle / DEG, abduction / DEG, twist / DEG
    rw = [[math.cos(a), -math.sin(a), 0], [math.sin(a), math.cos(a), 0], [0, 0, 1]]
    rv = [[math.cos(b), 0, -math.sin(b)], [0, 1, 0], [math.sin(b), 0, math.cos(b)]]
    ru = [[1, 0, 0], [0, math.cos(t), math.sin(t)], [0, -math.sin(t), math.cos(t)]]
    return mul(mul(rw, rv), ru)


def angles(local):
    u = [local[i][0] for i in range(3)]
    v = [local[i][1] for i in range(3)]
    b = math.asin(max(-1.0, min(1.0, u[2])))
    a = 0.0 if math.hypot(u[0], u[1]) < 1e-9 else math.atan2(u[1], u[0])
    v1 = [-math.sin(a), math.cos(a), 0.0]
    w2 = [-math.cos(a) * math.sin(b), -math.sin(a) * math.sin(b), math.cos(b)]
    t = math.atan2(-sum(v[i] * w2[i] for i in range(3)), sum(v[i] * v1[i] for i in range(3)))
    return a * DEG, b * DEG, t * DEG


def frames(bones, offsets=None):
    offsets = offsets or {}
    by_id = {bone["id"]: bone for bone in bones}
    out = {}

    def solve(bone_id):
        if bone_id in out:
            return out[bone_id]
        bone = by_id[bone_id]
        offset = offsets.get(bone_id, {})
        origin, basis = ([0.0, 0.0, 0.0], turn(0, 0, 0)) if bone["parent"] is None else solve(bone["parent"])
        here = [origin[i] + apply(basis, [bone["x"], bone["y"], bone.get("z", 0.0)])[i] for i in range(3)]
        rest = turn(bone["angleDeg"], bone.get("abductionDeg", 0.0), bone.get("twistDeg", 0.0))
        moved = turn(offset.get("angleDeg", 0.0), offset.get("abductionDeg", 0.0), offset.get("twistDeg", 0.0))
        out[bone_id] = (here, mul(mul(basis, rest), moved))
        return out[bone_id]

    for bone in bones:
        solve(bone["id"])
    return out


def quaternion(m):
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0:
        s = math.sqrt(trace + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return q


def product(a, b):
    return [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]


def r5(values):
    return [round(x, 5) + 0.0 for x in values]


def placed(parent, origin, u, axis):
    """The parent-frame fields of a bone with the world origin, the axis `u` and the joint axis nearest to `axis`."""
    length = math.sqrt(sum(c * c for c in u))
    u = [c / length for c in u]
    k = sum(axis[i] * u[i] for i in range(3))
    w = [axis[i] - k * u[i] for i in range(3)]
    n = math.sqrt(sum(c * c for c in w))
    w = [c / n for c in w]
    v = [w[1] * u[2] - w[2] * u[1], w[2] * u[0] - w[0] * u[2], w[0] * u[1] - w[1] * u[0]]
    world = [[u[i], v[i], w[i]] for i in range(3)]
    return fields(parent, origin, world)


def fields(parent, origin, world):
    local = mul(transpose(parent[1]), world)
    at = apply(transpose(parent[1]), [origin[i] - parent[0][i] for i in range(3)])
    a, b, t = angles(local)
    return {"x": round(at[0], 5) + 0.0, "y": round(at[1], 5) + 0.0, "z": round(at[2], 5) + 0.0, "angleDeg": round(a, 4) + 0.0, "abductionDeg": round(b, 4) + 0.0, "twistDeg": round(t, 4) + 0.0}


def rebind(solid, turn_matrix):
    """The same world solid written in the new frame of its bone (both frames share their origin)."""
    q = quaternion(turn_matrix)
    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = r5(apply(turn_matrix, solid["p0"])), r5(apply(turn_matrix, solid["p1"]))
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = r5(apply(turn_matrix, section["at"]))
            section["rotation"] = r5(product(q, section["rotation"]))
    elif solid["kind"] == "ellipsoid":
        transform = solid["transform"]
        transform["translation"] = r5(apply(turn_matrix, transform["translation"]))
        transform["rotation"] = r5(product(q, transform.get("rotation", [0, 0, 0, 1])))
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} on {solid['bone']} is not converted")


def convert(asset, volumes_sets):
    bones = asset["rig"]["bones"]
    by_id = {bone["id"]: bone for bone in bones}
    if "abductionDeg" in by_id["clavicle-l"]:
        return False
    old_bones = copy.deepcopy(bones)
    old = frames(old_bones)
    torso = old["torso"]
    for side, sign in (("l", 1), ("r", -1)):
        clavicle, arm = by_id[f"clavicle-{side}"], by_id[f"arm-{side}"]
        origin = old[clavicle["id"]][0]
        tip = [ACROMIOCLAVICULAR[0], ACROMIOCLAVICULAR[1], sign * ACROMIOCLAVICULAR[2]]
        u = [tip[i] - origin[i] for i in range(3)]
        clavicle.update({**placed(torso, origin, u, CLAVICLE_AXIS[side]), "lengthM": round(math.sqrt(sum(c * c for c in u)), 5), "joint": CLAVICLE_JOINT[side]})
        ordered = {key: clavicle[key] for key in ("id", "parent", "title", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg", "lengthM", "joint") if key in clavicle}
        ordered.update({key: value for key, value in clavicle.items() if key not in ordered})
        clavicle.clear()
        clavicle.update(ordered)
        now = frames(bones)
        placement = fields(now[clavicle["id"]], old[arm["id"]][0], old[arm["id"]][1])
        arm.update({key: value for key, value in placement.items() if key not in ("abductionDeg", "twistDeg") or abs(value) > 1e-9})
    new = frames(bones)
    for side in ("l", "r"):
        for bone_id in (f"arm-{side}", f"forearm-{side}", f"hand-{side}"):
            drift = max(abs(new[bone_id][0][i] - old[bone_id][0][i]) for i in range(3))
            turned = max(abs(new[bone_id][1][i][j] - old[bone_id][1][i][j]) for i in range(3) for j in range(3))
            if drift > 2e-5 or turned > 2e-5:
                raise SystemExit(f"{bone_id} moved by {drift} m / {turned}")
    old_poses = {pose["id"]: copy.deepcopy(pose.get("bones", {})) for pose in asset["poses"]}

    def reshrug(offsets, merged_before, inherited):
        """Writes the clavicle lift and the arm offsets that keep the posed arm directions into `offsets`; `inherited` are the already converted offsets below them (the pose of a clip key)."""
        before = frames(old_bones, merged_before)
        for side, sign in (("l", 1), ("r", -1)):
            if f"clavicle-{side}" in offsets:
                offsets[f"clavicle-{side}"] = {"angleDeg": round(-sign * offsets[f"clavicle-{side}"].get("twistDeg", 0), 3) + 0.0}
        for side in ("l", "r"):
            arm = f"arm-{side}"
            if arm not in offsets:
                continue
            parent = frames(bones, {key: value for key, value in {**inherited, **offsets}.items() if key != arm})[arm][1]
            a, b, t = angles(mul(transpose(parent), before[arm][1]))
            offsets[arm] = {key: value for key, value in (("angleDeg", round(a, 1) + 0.0), ("abductionDeg", round(b, 1) + 0.0), ("twistDeg", round(t, 1) + 0.0)) if key == "angleDeg" or value != 0}

    for pose in asset["poses"]:
        offsets = pose.get("bones", {})
        if any(bone_id.startswith("clavicle-") for bone_id in offsets):
            reshrug(offsets, old_poses[pose["id"]], {})
    new_poses = {pose["id"]: pose.get("bones", {}) for pose in asset["poses"]}
    for clip in asset["clips"]:
        for key in clip["keys"]:
            own = key.get("bones", {})
            merged = {**old_poses.get(key.get("pose"), {}), **own}
            if any(bone_id.startswith("clavicle-") for bone_id in merged) and any(bone_id in own for bone_id in ("arm-l", "arm-r", "clavicle-l", "clavicle-r")):
                reshrug(own, merged, new_poses.get(key.get("pose"), {}))
    turn_of = {f"clavicle-{side}": mul(transpose(new[f"clavicle-{side}"][1]), old[f"clavicle-{side}"][1]) for side in ("l", "r")}
    for volumes in volumes_sets:
        for volume in volumes.values():
            for name in ("body", "thermal", "structure"):
                for solid in volume.get(name, []):
                    if any(weight["bone"] in turn_of for weight in solid.get("weights", [])):
                        raise SystemExit(f"solid {solid['id']} is weighted to a clavicle: convert it by hand")
                    if solid.get("bone") in turn_of:
                        rebind(solid, turn_of[solid["bone"]])
    return True


def insert_module():
    """The layout helpers of `volumes-insert.py` (its text up to `main`, which runs on import)."""
    text = open(os.path.join(T, "volumes-insert.py"), encoding="utf8").read()
    scope = {}
    exec(compile(text[: text.index("\ndef main(")], "volumes-insert.py", "exec"), scope)
    return type("VolumesInsert", (), {name: staticmethod(value) for name, value in scope.items() if callable(value)})


def lib_module():
    spec = importlib.util.spec_from_file_location("rework_01_lib", os.path.join(T, "rework-01-lib.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def splice(text, key, before, after):
    """Replaces one member of the asset in the catalog text, in the layout the member already has; every other byte stays."""
    insert, lib = insert_module(), lib_module()
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    depth = len(indent) // 2
    start, end = insert.member_range(text, indent, key)
    current = text[start:end]
    for layout in (lambda value: lib.catalog_text(value, depth), lambda value: insert.dump(value, depth)):
        if indent + json.dumps(key) + ": " + layout(before) == current:
            return text[:start] + indent + json.dumps(key) + ": " + layout(after) + text[end:]
    raise SystemExit(f"member {key} has an unknown layout: not written")


def main():
    if "--apply" not in sys.argv:
        document = json.load(open(CATALOG, encoding="utf8"))
        source = json.load(open(SOURCE, encoding="utf8"))
        document["asset"]["volumes"] = source["volumes"]
        if not convert(document["asset"], [document["asset"]["volumes"]]):
            raise SystemExit("already converted")
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        for bone in document["asset"]["rig"]["bones"]:
            if bone["id"] in ("clavicle-l", "clavicle-r", "arm-l", "arm-r"):
                print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass")}))
        for pose in document["asset"]["poses"]:
            print(pose["id"], json.dumps({key: value for key, value in pose["bones"].items() if key[:-2] in ("clavicle", "arm")}))
        print(CANDIDATE)
        return
    original = text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    source_text = open(SOURCE, encoding="utf8").read()
    source = json.loads(source_text)
    if not convert(document["asset"], [document["asset"]["volumes"], source["volumes"]]):
        raise SystemExit("already converted")
    for key in ("rig", "poses", "clips", "volumes"):
        text = splice(text, key, before[key], document["asset"][key])
    if json.loads(text) != document:
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf8").read() != original:
        raise SystemExit("the catalog file changed while converting: run again")
    open(CATALOG, "w", encoding="utf8").write(text)
    if open(SOURCE, encoding="utf8").read() != source_text:
        raise SystemExit("the volumes source changed while converting: catalog written, run again for the source")
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("applied to", CATALOG, "and", SOURCE)


if __name__ == "__main__":
    main()
