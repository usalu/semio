"""🎸 Seated guitarist: shoulder and pelvic girdle on lateral rest placement, shoulder breadth to the reference, right hand articulated like the left.

Usage: python3 rework-03-seated-guitarist-rig.py            writes the candidate document (🗑️generated/rework-seated-guitarist/cand/🔣️.json)
       python3 rework-03-seated-guitarist-rig.py --apply    writes rig, poses, representations into the catalog file and the volumes into the source
Hand-chosen numbers are the constants below; see rework/assets/humans--seated-guitarist.md and rig-lateral-rest.md.
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
R = importlib.util.module_from_spec(spec)
spec.loader.exec_module(R)

CATALOG = os.path.join(R.REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "seated-guitarist", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "seated-guitarist.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-seated-guitarist", "cand", "🔣️.json")

SHOULDER_WAS, SHOULDER = 0.19, 0.178
STERNOCLAVICULAR = (0.412, -0.058, 0.02)
ACROMIOCLAVICULAR = (0.452, 0.004, 0.166)
SACROILIAC = (0.07, 0.055, 0.04)
HIP = (0.0, 0.0, 0.09)
LIMB = [0, -0.70711, 0, 0.70711]
TITLES = {
    "clavicle": ({"en": "%s clavicle with scapula", "de": "%s Schlüsselbein mit Schulterblatt"}, {"l": ("Left", "Linkes"), "r": ("Right", "Rechtes")}),
    "hip-bone": ({"en": "%s hip bone", "de": "%s Hüftbein"}, {"l": ("Left", "Linkes"), "r": ("Right", "Rechtes")}),
}
CLAVICLE_JOINT = R.CLAVICLE_JOINT
PICK_GRIP = {
    "index-proximal-r": {"angleDeg": 35}, "index-middle-r": {"angleDeg": 68}, "index-distal-r": {"angleDeg": 32},
    "middle-proximal-r": {"angleDeg": 38, "abductionDeg": 2}, "middle-middle-r": {"angleDeg": 62}, "middle-distal-r": {"angleDeg": 34},
    "ring-proximal-r": {"angleDeg": 40, "abductionDeg": 4}, "ring-middle-r": {"angleDeg": 60}, "ring-distal-r": {"angleDeg": 34},
    "little-proximal-r": {"angleDeg": 40, "abductionDeg": 7}, "little-middle-r": {"angleDeg": 56}, "little-distal-r": {"angleDeg": 32},
    "thumb-metacarpal-r": {"angleDeg": 24, "abductionDeg": -11, "twistDeg": -60}, "thumb-proximal-r": {"angleDeg": 8}, "thumb-distal-r": {"angleDeg": 8},
}
LEFT_HAND = ["hand-l"] + [f"{f}-{p}-l" for f in ("index", "middle", "ring", "little") for p in ("proximal", "middle", "distal")] + ["thumb-metacarpal-l", "thumb-proximal-l", "thumb-distal-l"]
ARM = ("upper-arm", "forearm", "hand", "index", "middle", "ring", "little", "thumb")


def add(a, b):
    return [a[i] + b[i] for i in range(3)]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def world(frame, p):
    return add(frame[0], R.apply(frame[1], list(p)))


def right(name):
    return name[:-2] + "-r" if name.endswith("-l") else name


def side_of(name):
    return 1 if name.endswith("-l") else -1


def mirror_quaternion(q):
    return [-q[0], -q[1], q[2], q[3]]


def mirrored(solid):
    """The left-hand solid reflected to the right hand (w negated in the mirrored bone frame)."""
    s = copy.deepcopy(solid)
    for key in ("id", "bone", "member", "of"):
        if key in s:
            s[key] = right(s[key])
    if "members" in s:
        s["members"] = [right(b) for b in s["members"]]
    for key in ("p0", "p1"):
        if key in s:
            s[key] = [s[key][0], s[key][1], -s[key][2] + 0.0]
    for section in s.get("sections", []):
        section["at"] = [section["at"][0], section["at"][1], -section["at"][2] + 0.0]
        if "rotation" in section and section["rotation"] != LIMB:
            section["rotation"] = mirror_quaternion(section["rotation"])
    if "transform" in s:
        t = s["transform"]
        if "translation" in t:
            t["translation"] = [t["translation"][0], t["translation"][1], -t["translation"][2] + 0.0]
        if "rotation" in t:
            t["rotation"] = mirror_quaternion(t["rotation"])
    return s


def narrowed(solid, factor):
    """A torso-bound girdle solid drawn in to the new shoulder breadth (lateral coordinate scaled about the median plane)."""
    if "transform" in solid and "translation" in solid["transform"]:
        t = solid["transform"]["translation"]
        t[2] = round(t[2] * factor, 5)
        return
    for key in ("p0", "p1"):
        if key in solid:
            solid[key][2] = round(solid[key][2] * factor, 5)
    for section in solid.get("sections", []):
        section["at"][2] = round(section["at"][2] * factor, 5)


def reframed(solid, old, new):
    """The same world solid written in another bone frame (origin and axes may both differ)."""
    m = R.mul(R.transpose(new[1]), old[1])
    t = R.apply(R.transpose(new[1]), sub(old[0], new[0]))
    q = R.quaternion(m)
    if solid["kind"] == "capsule" and "transform" not in solid:
        solid["p0"] = R.r5(add(R.apply(m, solid["p0"]), t))
        solid["p1"] = R.r5(add(R.apply(m, solid["p1"]), t))
        return
    transform = solid.setdefault("transform", {})
    transform["translation"] = R.r5(add(R.apply(m, transform.get("translation", [0, 0, 0])), t))
    transform["rotation"] = R.r5(R.product(q, transform.get("rotation", [0, 0, 0, 1])))


def solve_linear(a, b):
    n = len(b)
    a = [row[:] + [b[i]] for i, row in enumerate(a)]
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(a[r][c]))
        a[c], a[p] = a[p], a[c]
        for r in range(c + 1, n):
            f = a[r][c] / a[c][c]
            for k in range(c, n + 1):
                a[r][k] -= f * a[c][k]
    x = [0.0] * n
    for c in range(n - 1, -1, -1):
        x[c] = (a[c][n] - sum(a[c][k] * x[k] for k in range(c + 1, n))) / a[c][c]
    return x


def reach(bones, offsets, side, target):
    """Offsets of upper arm, forearm and hand that put the hand frame back on `target` (damped Gauss-Newton, numeric Jacobian)."""
    keys = [(f"upper-arm-{side}", "angleDeg"), (f"upper-arm-{side}", "abductionDeg"), (f"upper-arm-{side}", "twistDeg"), (f"forearm-{side}", "angleDeg"), (f"forearm-{side}", "twistDeg"), (f"hand-{side}", "angleDeg"), (f"hand-{side}", "abductionDeg")]
    chain = [b for b in bones if b["id"] in ("stool", "hip-lift", "hip-shift", "pelvis", "torso", f"clavicle-{side}", f"upper-arm-{side}", f"forearm-{side}", f"hand-{side}")]
    q = [offsets.get(b, {}).get(k, 0.0) for b, k in keys]

    def residual(values):
        trial = {b: dict(offsets.get(b, {})) for b in {key[0] for key in keys} | {"pelvis", "torso"}}
        for (b, k), value in zip(keys, values):
            trial[b][k] = value
        o, m = R.frames(chain, trial)[f"hand-{side}"]
        out = sub(o, target[0])
        for column in (0, 1):
            out += [0.1 * (m[i][column] - target[1][i][column]) for i in range(3)]
        return out

    for _ in range(40):
        r0 = residual(q)
        jac = []
        for i in range(len(q)):
            probe = q[:]
            probe[i] += 1e-4
            r1 = residual(probe)
            jac.append([(r1[k] - r0[k]) / 1e-4 for k in range(len(r0))])
        normal = [[sum(jac[i][k] * jac[j][k] for k in range(len(r0))) + (1e-12 if i == j else 0.0) for j in range(len(q))] for i in range(len(q))]
        rhs = [-sum(jac[i][k] * r0[k] for k in range(len(r0))) for i in range(len(q))]
        step = solve_linear(normal, rhs)
        q = [q[i] + step[i] for i in range(len(q))]
        if max(abs(s) for s in step) < 1e-7:
            break
    error = math.sqrt(sum(c * c for c in residual(q)[:3]))
    for (b, k), value in zip(keys, q):
        offsets.setdefault(b, {})[k] = round(value, 3) + 0.0
    return error


def clavicle_solid(side, scale):
    """The S-shaped clavicle in its own bone frame: sternal end round, shaft thin, acromial end flat; w is back on the left and ahead on the right."""
    s = 1 if side == "l" else -1
    length = math.dist(STERNOCLAVICULAR, ACROMIOCLAVICULAR)
    least = {"50": 0.0, "100": 0.0075}[scale]
    stations = [(0.0, 0.0, 0.0125, 0.0115), (0.14, -0.004, 0.0105, 0.0098), (0.33, -0.011, 0.0072, 0.0068), (0.52, -0.006, 0.0062, 0.006), (0.7, 0.006, 0.0068, 0.0058), (0.86, 0.008, 0.0095, 0.0055), (1.0, 0.002, 0.0115, 0.0055)]
    return {
        "id": f"clavicle-{side}", "kind": "loft", "bone": f"clavicle-{side}", "material": "bone", "role": "body", "tone": 0.62,
        "sections": [{"at": [round(f * length, 5), 0, round(s * bow, 5)], "rx": max(rx, least), "ry": max(ry, least), "exponent": 2, "rotation": LIMB} for f, bow, rx, ry in stations],
        "capStart": True, "capEnd": True, "member": f"clavicle-{side}", "part": "member",
    }


def joint_solid(bone, radius):
    return {"id": f"joint-{bone}", "kind": "ellipsoid", "bone": bone, "material": "bone", "role": "body", "tone": 0.4, "radii": [radius, radius, radius], "transform": {"translation": [0, 0, 0]}, "member": bone, "part": "joint"}


def convert(asset, volume_sets):
    bones = asset["rig"]["bones"]
    if any(b["id"] == "clavicle-l" for b in bones):
        return False
    by_id = {b["id"]: b for b in bones}
    rest_before = R.frames(bones)
    posed_before = {pose["id"]: R.frames(bones, pose["bones"]) for pose in asset["poses"]}
    torso, pelvis = rest_before["torso"], rest_before["pelvis"]
    new_frames = {}
    for side, s in (("l", 1), ("r", -1)):
        sc = world(torso, (STERNOCLAVICULAR[0], STERNOCLAVICULAR[1], s * STERNOCLAVICULAR[2]))
        ac = world(torso, (ACROMIOCLAVICULAR[0], ACROMIOCLAVICULAR[1], s * ACROMIOCLAVICULAR[2]))
        clavicle = {"id": f"clavicle-{side}", "parent": "torso", "title": {k: v % TITLES["clavicle"][1][side][i] for i, (k, v) in enumerate(TITLES["clavicle"][0].items())}}
        clavicle.update(R.placed(torso, sc, sub(ac, sc), (-1.0 * s, 0.0, 0.0)))
        clavicle.update({"lengthM": round(math.dist(sc, ac), 5), "joint": CLAVICLE_JOINT[side]})
        si = world(pelvis, (SACROILIAC[0], SACROILIAC[1], s * SACROILIAC[2]))
        hip = world(pelvis, (HIP[0], HIP[1], s * HIP[2]))
        hip_bone = {"id": f"hip-bone-{side}", "parent": "pelvis", "title": {k: v % TITLES["hip-bone"][1][side][i] for i, (k, v) in enumerate(TITLES["hip-bone"][0].items())}}
        hip_bone.update(R.placed(pelvis, si, sub(hip, si), (-1.0 * s, 0.0, 0.0)))
        hip_bone.update({"lengthM": round(math.dist(si, hip), 5), "joint": {"type": "revolute", "minDeg": -3, "maxDeg": 3}})
        bones.insert(bones.index(by_id[f"upper-arm-{side}"]), clavicle)
        bones.insert(bones.index(by_id[f"thigh-{side}"]), hip_bone)
        by_id[clavicle["id"]], by_id[hip_bone["id"]] = clavicle, hip_bone
        frames_now = R.frames(bones)
        new_frames[clavicle["id"]], new_frames[hip_bone["id"]] = frames_now[clavicle["id"]], frames_now[hip_bone["id"]]
        arm = by_id[f"upper-arm-{side}"]
        shoulder = world(torso, (arm["x"], arm["y"], s * SHOULDER))
        arm["parent"] = clavicle["id"]
        arm.update(R.fields(frames_now[clavicle["id"]], shoulder, rest_before[arm["id"]][1]))
        thigh = by_id[f"thigh-{side}"]
        thigh["parent"] = hip_bone["id"]
        thigh.update(R.fields(frames_now[hip_bone["id"]], rest_before[thigh["id"]][0], rest_before[thigh["id"]][1]))
    by_id["hand-r"]["lengthM"] = by_id["hand-l"]["lengthM"]
    for left in LEFT_HAND[1:]:
        twin = copy.deepcopy(by_id[left])
        twin["id"], twin["parent"] = right(left), right(twin["parent"])
        twin["title"] = {"en": twin["title"]["en"].replace("Left", "Right"), "de": twin["title"]["de"].replace("links", "rechts")}
        twin["z"] = -twin.get("z", 0.0) + 0.0
        bones.append(twin)
    report = []
    for pose in asset["poses"]:
        offsets = pose["bones"]
        for bone, value in PICK_GRIP.items():
            offsets[bone] = dict(value)
        for side in ("l", "r"):
            error = reach(bones, offsets, side, posed_before[pose["id"]][f"hand-{side}"])
            report.append(f"{pose['id']} hand-{side} {error * 1000:.3f} mm elbow {offsets['forearm-' + side]['angleDeg']}")
    print("[DEBUG] reach:", "; ".join(report))

    factor = SHOULDER / SHOULDER_WAS
    for volumes in volume_sets:
        for scale, volume in volumes.items():
            if volume.get("status", "authored") != "authored":
                continue
            gone_body = {b["id"] for b in volume["body"] if b.get("bone") == "hand-r"}
            for name in ("body", "thermal", "structure"):
                entries = volume[name]
                gone = {s["id"] for s in entries if s.get("bone") == "hand-r" or s.get("of") in gone_body}
                kept = [s for s in entries if s["id"] not in gone]
                left_ids = {b["id"] for b in volume["body"] if b.get("bone") in LEFT_HAND}
                twins = [mirrored(s) for s in entries if s.get("bone") in LEFT_HAND or (s.get("of") in left_ids)]
                entries[:] = kept + twins
            for solid in volume["structure"]:
                key, bone = solid["id"], solid.get("bone")
                if solid.get("part") != "member" or key[-2:] not in ("-l", "-r"):
                    if bone == "torso" and key == "shoulder-bar":
                        narrowed(solid, factor)
                        solid["members"] = ["clavicle-l", "clavicle-r"]
                    if bone == "pelvis" and key == "pelvic-bar":
                        solid["members"] = ["hip-bone-l", "hip-bone-r"]
                    continue
                side = key[-1]
                if bone == "torso" and key[:-2] in ("clavicle", "scapula", "acromion", "shoulder-bar"):
                    narrowed(solid, factor)
                    reframed(solid, torso, new_frames[f"clavicle-{side}"])
                    solid["bone"] = solid["member"] = f"clavicle-{side}"
                if bone == "pelvis" and key[:-2] in ("ilium", "pubis", "ischium", "hip-bar"):
                    reframed(solid, pelvis, new_frames[f"hip-bone-{side}"])
                    solid["bone"] = solid["member"] = f"hip-bone-{side}"
            if scale in ("50", "100"):
                structure = volume["structure"]
                for side in ("l", "r"):
                    index = next(i for i, s in enumerate(structure) if s["id"] == f"clavicle-{side}")
                    structure[index] = clavicle_solid(side, scale)
                    structure.append(joint_solid(f"clavicle-{side}", 0.011))
                    structure.append(joint_solid(f"hip-bone-{side}", 0.013))

    shift = SHOULDER_WAS - SHOULDER
    for representation in asset["representations"].values():
        geometry = representation.get("geometry")
        if not geometry:
            continue
        for view in ("elevation", "front", "plan"):
            shapes = geometry[view]
            kept = [s for s in shapes if s.get("bone") != "hand-r"]
            twins = []
            for shape in shapes:
                if shape.get("bone") not in LEFT_HAND:
                    continue
                twin = copy.deepcopy(shape)
                twin["id"], twin["bone"] = right(twin["id"]), right(twin["bone"])
                solid = twin.get("solid", {})
                if "axisBone" in solid:
                    solid["axisBone"] = right(solid["axisBone"])
                if solid.get("kind") == "extruded":
                    low, high = -solid["z1"], -solid["z0"]
                    solid["z0"], solid["z1"] = low + 0.0, high + 0.0
                twins.append(twin)
            shapes[:] = kept + twins
            for shape in shapes:
                solid = shape.get("solid", {})
                bone = shape.get("bone", "")
                if solid.get("kind") == "extruded" and solid.get("sweep") == "w" and bone.startswith(ARM):
                    s = side_of(bone)
                    for key in ("z0", "z1", "zTip"):
                        if key in solid:
                            solid[key] = round(solid[key] - s * shift, 5)
    return True


def replace_member(text, key, before, after):
    try:
        return R.splice(text, key, before, after)
    except SystemExit:
        import re
        insert = R.insert_module()
        indent = re.search(r'\n( +)"id"\s*:', text).group(1)
        start, end = insert.member_range(text, indent, key)
        return text[:start] + indent + json.dumps(key) + ": " + insert.dump(after, len(indent) // 2) + text[end:]


def main():
    original = open(CATALOG, encoding="utf8").read()
    document = json.loads(original)
    before = copy.deepcopy(document["asset"])
    keys = ("rig", "poses", "representations", "volumes")
    if "--apply" not in sys.argv:
        source = json.load(open(SOURCE, encoding="utf8"))
        if source["volumes"] != document["asset"]["volumes"]:
            raise SystemExit("source and catalog volumes differ: reconcile first")
        if not convert(document["asset"], [document["asset"]["volumes"]]):
            raise SystemExit("already converted")
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        print(CANDIDATE)
        return
    candidate = json.load(open(CANDIDATE, encoding="utf8"))["asset"]
    if {k: v for k, v in candidate.items() if k not in keys} != {k: v for k, v in before.items() if k not in keys}:
        raise SystemExit("the catalog asset changed outside rig, poses, representations, volumes since the candidate was built: rebuild the candidate")
    text = original
    for key in keys[:3]:
        if before[key] != candidate[key]:
            text = replace_member(text, key, before[key], candidate[key])
    check = json.loads(text)["asset"]
    if any(check[key] != candidate[key] for key in keys[:3]):
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf8").read() != original:
        raise SystemExit("catalog changed while writing: run again")
    open(CATALOG, "w", encoding="utf8").write(text)
    json.dump({"volumes": candidate["volumes"]}, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("applied; now run volumes-insert.py")


if __name__ == "__main__":
    main()
