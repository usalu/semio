"""🦴 worker-carrying-box, second round pass 1: shoulder and pelvic girdle on the lateral rest placement of the rig and an anatomically joined skeleton at every scale.

Rig: `clavicle-l/-r` (sternoclavicular to acromioclavicular joint) under `torso`, `hip-bone-l/-r` (sacroiliac joint to hip joint) under `pelvis`; upper arms and thighs are re-parented without moving. Structure: the girdle solids are re-expressed in the frames of their new bones, the clavicle becomes an S-curved loft, the iliac body joins sacrum and socket, the jaw joint, the tarsus at 1:100 and the costal cartilages at 1:200 close the load path, the long bones keep their true shafts as `slender` members and the compact parts below the feature of 1:200 and 1:500 are merged into joint masses.

Usage: python3 rework-wcb-girdles.py            writes the candidate `🗑️generated/rework-worker-carrying-box/candidate/🔣️.json`
       python3 rework-wcb-girdles.py --apply    re-reads the catalog file, writes `rig` and `volumes` there and the volumes source

@see ./rig-lateral-rest.md
@see ./rework-brief.md
"""
import copy
import importlib.util
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rl", os.path.join(T, "rig-lateral-standing-adult.py"))
rl = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rl)
ins_spec = importlib.util.spec_from_file_location("ins", os.path.join(T, "volumes-insert.py"))
CATALOG = os.path.join(rl.REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "worker-carrying-box", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "worker-carrying-box.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-worker-carrying-box", "candidate", "🔣️.json")

STERNOCLAVICULAR_IN_TORSO = [0.41891, -0.07812, 0.022]
ACROMIOCLAVICULAR_FROM_STERNAL = [-0.055, 0.028]
ACROMIOCLAVICULAR_LATERAL = 0.176
ACROMION_FROM_JOINT = [-0.008, -0.003, 0.012]
SACROILIAC_IN_PELVIS = [0.078, 0.058, 0.046]
LIMB = [0.5, 0.5, 0.5, 0.5]
BONE = {"material": "bone", "role": "body", "tone": 0.85}
TO_CLAVICLE = re.compile(r"^(scapula|scapular-spine|glenoid)-([lr])$")
TO_HIP = re.compile(r"^(ilium|ilium-wing|pubic-ramus|ischium|ischial-ramus|acetabulum)-([lr])$")


def r5(values):
    return [round(x, 5) + 0.0 for x in values]


def add(a, b, k=1.0):
    return [a[i] + b[i] * k for i in range(3)]


def world(frame, p):
    return add(frame[0], rl.apply(frame[1], p))


def local(frame, p):
    return rl.apply(rl.transpose(frame[1]), add(p, frame[0], -1.0))


def mirror_fields(f):
    return {**f, "z": -f["z"], "abductionDeg": -f["abductionDeg"], "twistDeg": -f["twistDeg"]}


def mirror(solid):
    m = copy.deepcopy(solid)
    for key in ("id", "bone", "member"):
        m[key] = re.sub(r"-l$", "-r", m[key])
    flip = lambda q: [-q[0], -q[1], q[2], q[3]]
    for section in m.get("sections", []):
        section["at"][2] = -section["at"][2]
        section["rotation"] = flip(section["rotation"])
    for key in ("p0", "p1"):
        if key in m:
            m[key][2] = -m[key][2]
    if "transform" in m:
        m["transform"]["translation"][2] = -m["transform"]["translation"][2]
        if "rotation" in m["transform"]:
            m["transform"]["rotation"] = flip(m["transform"]["rotation"])
    return m


def rebind(solid, old, new, bone):
    """The same world solid in the frame of another bone: its own transform carries the change of frame."""
    turn = rl.mul(rl.transpose(new[1]), old[1])
    shift = rl.apply(rl.transpose(new[1]), add(old[0], new[0], -1.0))
    transform = solid.get("transform", {})
    solid["transform"] = {"rotation": r5(rl.product(rl.quaternion(turn), transform.get("rotation", [0, 0, 0, 1]))), "translation": r5(add(shift, rl.apply(turn, transform.get("translation", [0, 0, 0]))))}
    if solid.get("member") == solid["bone"]:
        solid["member"] = bone
    solid["bone"] = bone


def capsule(id, bone, p0, p1, r0, r1, part="member", **extra):
    return {"id": id, "kind": "capsule", "bone": bone, **BONE, "p0": r5(p0), "p1": r5(p1), "r0": r0, "r1": r1, "member": bone, "part": part, **extra}


def ball(id, bone, radii, at, part="joint"):
    return {"id": id, "kind": "ellipsoid", "bone": bone, **BONE, "radii": radii, "transform": {"translation": r5(at)}, "member": bone, "part": part}


def loft(id, bone, stations):
    return {"id": id, "kind": "loft", "bone": bone, **BONE, "capStart": True, "capEnd": True, "sections": [{"at": r5(at), "rx": rx, "ry": ry, "exponent": 2, "rotation": LIMB} for at, rx, ry in stations], "member": bone, "part": "member"}


def pair(solid):
    return [solid, mirror(solid)]


def convert(asset):
    bones = asset["rig"]["bones"]
    by_id = {bone["id"]: bone for bone in bones}
    if "clavicle-l" in by_id:
        raise SystemExit("already converted")
    old = rl.frames(copy.deepcopy(bones))
    sternal = world(old["torso"], STERNOCLAVICULAR_IN_TORSO)
    acromial = [sternal[0] + ACROMIOCLAVICULAR_FROM_STERNAL[0], sternal[1] + ACROMIOCLAVICULAR_FROM_STERNAL[1], ACROMIOCLAVICULAR_LATERAL]
    clavicle_length = round(math.dist(sternal, acromial), 5)
    clavicle = rl.placed(old["torso"], sternal, add(acromial, sternal, -1.0), (-1.0, 0.0, 0.0))
    sacroiliac = world(old["pelvis"], SACROILIAC_IN_PELVIS)
    socket = old["thigh-l"][0]
    hip_length = round(math.dist(sacroiliac, socket), 5)
    hip = rl.placed(old["pelvis"], sacroiliac, add(socket, sacroiliac, -1.0), (-1.0, 0.0, 0.0))
    titles = {"clavicle-l": ("Left clavicle (shoulder girdle)", "Linkes Schlüsselbein (Schultergürtel)"), "clavicle-r": ("Right clavicle (shoulder girdle)", "Rechtes Schlüsselbein (Schultergürtel)"), "hip-bone-l": ("Left hip bone (pelvic girdle)", "Linkes Hüftbein (Beckengürtel)"), "hip-bone-r": ("Right hip bone (pelvic girdle)", "Rechtes Hüftbein (Beckengürtel)")}
    joints = {"clavicle-l": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40, "twistMaxDeg": 5}, "clavicle-r": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -5, "twistMaxDeg": 40}, "hip-bone-l": {"type": "revolute", "minDeg": -4, "maxDeg": 4}, "hip-bone-r": {"type": "revolute", "minDeg": -4, "maxDeg": 4}}
    made = {}
    for id, parent, fields, length in (("clavicle-l", "torso", clavicle, clavicle_length), ("clavicle-r", "torso", mirror_fields(clavicle), clavicle_length), ("hip-bone-l", "pelvis", hip, hip_length), ("hip-bone-r", "pelvis", mirror_fields(hip), hip_length)):
        made[id] = {"id": id, "parent": parent, "title": {"en": titles[id][0], "de": titles[id][1]}, **fields, "lengthM": length, "joint": joints[id]}
    order = []
    for bone in bones:
        girdle = {"upper-arm-l": "clavicle-l", "upper-arm-r": "clavicle-r", "thigh-l": "hip-bone-l", "thigh-r": "hip-bone-r"}.get(bone["id"])
        if girdle:
            order.append(made[girdle])
        order.append(bone)
    with_girdles = rl.frames(copy.deepcopy(order))
    for limb, girdle in (("upper-arm-l", "clavicle-l"), ("upper-arm-r", "clavicle-r"), ("thigh-l", "hip-bone-l"), ("thigh-r", "hip-bone-r")):
        by_id[limb].update({"parent": girdle, **rl.fields(with_girdles[girdle], old[limb][0], old[limb][1])})
    asset["rig"]["bones"] = order
    new = rl.frames(copy.deepcopy(order))
    moved = max(max(abs(new[id][0][i] - old[id][0][i]) for i in range(3)) + max(abs(new[id][1][i][j] - old[id][1][i][j]) for i in range(3) for j in range(3)) for id in old)
    print(f"clavicle {clavicle_length} m, hip bone {hip_length} m, largest move of an existing bone frame {moved:.2e}")
    for id in made:
        print(id, json.dumps({k: v for k, v in made[id].items() if k not in ("title", "joint")}), "tip", r5(world(new[id], [made[id]["lengthM"], 0, 0])))
    for id in ("upper-arm-l", "thigh-l"):
        print(id, json.dumps({k: v for k, v in by_id[id].items() if k not in ("title", "joint", "mass")}))
    acromion = {side: add([acromial[0], acromial[1], sign * acromial[2]], [ACROMION_FROM_JOINT[0], ACROMION_FROM_JOINT[1], sign * ACROMION_FROM_JOINT[2]]) for side, sign in (("l", 1.0), ("r", -1.0))}
    L, H = clavicle_length, hip_length

    for scale, lists in asset["volumes"].items():
        structure = lists["structure"]
        for solid in structure:
            match = TO_CLAVICLE.match(solid["id"]) or TO_HIP.match(solid["id"])
            if not match:
                continue
            bone = ("clavicle-" if TO_CLAVICLE.match(solid["id"]) else "hip-bone-") + match.group(2)
            if solid["id"].startswith("scapular-spine-"):
                frame = new[bone]
                solid.update({"p0": r5(local(frame, world(old["torso"], solid["p0"]))), "p1": r5(local(frame, acromion[match.group(2)])), "bone": bone, "member": bone})
            else:
                rebind(solid, old[solid["bone"]], new[bone], bone)
        drop, more = set(), []
        if scale == "50":
            drop = {"clavicle-l", "clavicle-r"}
            more += pair(loft("clavicle-l", "clavicle-l", [([0, 0, 0], 0.011, 0.012), ([0.02, 0, -0.003], 0.009, 0.01), ([0.25 * L, 0, -0.011], 0.0065, 0.0075), ([0.5 * L, 0, -0.012], 0.006, 0.007), ([0.75 * L, -0.001, 0.003], 0.0055, 0.009), ([0.92 * L, 0, 0.005], 0.005, 0.011), ([L, 0, 0], 0.005, 0.011)]))
            more += pair(ball("joint-sternoclavicular-l", "clavicle-l", [0.011, 0.011, 0.011], [0, 0, 0]))
            more += pair(capsule("iliac-body-l", "hip-bone-l", [0, 0, 0], [H - 0.012, 0, 0], 0.017, 0.019))
            more += pair(ball("joint-sacroiliac-l", "hip-bone-l", [0.014, 0.02, 0.008], [0, 0, 0]))
            more += pair(ball("joint-jaw-l", "head", [0.012, 0.012, 0.012], [0.004, -0.006, 0.056]))
        if scale == "100":
            drop = {"clavicle-l", "clavicle-r"}
            more += pair(loft("clavicle-l", "clavicle-l", [([0, 0, 0], 0.011, 0.012), ([0.25 * L, 0, -0.011], 0.008, 0.0085), ([0.5 * L, 0, -0.012], 0.0075, 0.008), ([0.75 * L, -0.001, 0.003], 0.0075, 0.0095), ([L, 0, 0], 0.0075, 0.011)]))
            more += pair(ball("joint-sternoclavicular-l", "clavicle-l", [0.011, 0.011, 0.011], [0, 0, 0]))
            more += pair(capsule("iliac-body-l", "hip-bone-l", [0, 0, 0], [H - 0.012, 0, 0], 0.017, 0.019))
            more += pair(ball("joint-sacroiliac-l", "hip-bone-l", [0.014, 0.02, 0.008], [0, 0, 0]))
            more += pair(ball("joint-jaw-l", "head", [0.012, 0.012, 0.012], [0.004, -0.006, 0.056]))
            more += pair({"id": "tarsus-l", "kind": "ellipsoid", "bone": "foot-l", **BONE, "radii": [0.028, 0.017, 0.03], "transform": {"translation": [0.048, -0.0298, 0.002], "rotation": [0, 0, -0.12187, 0.99255]}, "member": "foot-l", "part": "member"})
        if scale == "200":
            drop = {"shoulder-girdle", "patella-l", "patella-r", "occipital-condyle-l", "occipital-condyle-r"}
            more += pair(capsule("clavicle-l", "clavicle-l", [0, 0, 0], [L, 0, 0], 0.016, 0.015))
            more += pair(capsule("iliac-body-l", "hip-bone-l", [0, 0, 0], [H - 0.012, 0, 0], 0.017, 0.019))
            more.append(ball("joint-atlanto-occipital", "head", [0.015, 0.016, 0.028], [0, 0, 0]))
            radii = {"joint-knee": [0.016, 0.016, 0.046], "joint-elbow": [0.015, 0.015, 0.034], "joint-c7": [0.018, 0.015, 0.021], "humeral-condyle": [0.016, 0.015, 0.031]}
            for solid in structure:
                stem = re.sub(r"-[lr]$", "", solid["id"])
                if stem in radii:
                    solid["radii"] = radii[stem]
                if stem in ("fibula", "ulna", "radius"):
                    solid["slender"] = True
                if re.match(r"^rib-[lr]-\d+$", solid["id"]):
                    last = solid["sections"][-1]
                    side = 1.0 if last["at"][2] > 0 else -1.0
                    x = min(0.412, max(0.246, last["at"][0]))
                    y = -0.08206 + (0.42103 - x) / (0.42103 - 0.23751) * (-0.13274 + 0.08206)
                    if abs(x - last["at"][0]) > 0.01:
                        solid["sections"].append({**last, "at": r5([(x + last["at"][0]) / 2, (y + last["at"][1]) / 2, side * 0.036])})
                    solid["sections"].append({**last, "at": r5([x, y + 0.004, side * 0.014])})
        if scale == "500":
            drop = {"shoulder-girdle", "cervical-spine", "neck-bar", "joint-c7"} | {f"{stem}-{side}" for side in "lr" for stem in ("occipital-condyle", "femoral-head", "femoral-neck", "trochanter", "femoral-condyles", "patella", "fibula", "humeral-head", "humeral-condyle")}
            more.append(capsule("cervical-spine", "neck", [0.002, 0.0002, 0], [0.085, -0.0065, 0], 0.0375, 0.0375))
            more += pair(capsule("clavicle-l", "clavicle-l", [0.01, -0.012, 0], [L - 0.02, -0.02, 0], 0.0375, 0.0375))
            more += pair(capsule("hip-bone-l", "hip-bone-l", [0.035, 0, 0], [H, 0, 0], 0.0375, 0.0375))
            more += pair(ball("joint-hip-l", "thigh-l", [0.0375, 0.0375, 0.0375], [0, 0, 0]))
            more += pair(ball("joint-knee-l", "shank-l", [0.0375, 0.0375, 0.0375], [0, 0, 0]))
            more += pair(ball("joint-shoulder-l", "upper-arm-l", [0.03, 0.03, 0.03], [0, 0, 0]))
            more += pair(ball("joint-elbow-l", "forearm-l", [0.024, 0.024, 0.024], [0, 0, 0]))
            for solid in structure:
                if re.sub(r"-[lr]$", "", solid["id"]) in ("femur", "tibia", "humerus", "ulna", "radius"):
                    solid["slender"] = True
        present = {solid["id"] for solid in structure}
        missing = drop - present
        if missing:
            raise SystemExit(f"1:{scale}: not in the list: {sorted(missing)}")
        kept = [solid for solid in structure if solid["id"] not in drop]
        clash = {solid["id"] for solid in kept} & {solid["id"] for solid in more}
        if clash:
            raise SystemExit(f"1:{scale}: already in the list: {sorted(clash)}")
        lists["structure"] = kept + more
        print(f"1:{scale} structure {len(structure)} -> {len(lists['structure'])}")


def splice(text, key, value, ins):
    start = re.search(r'\n    "' + key + '": ', text)
    begin = start.end()
    return text[:begin] + ins.dump(value, 2) + text[ins.member_end(text, begin):]


def main():
    document = json.load(open(CATALOG, encoding="utf8"))
    source = json.load(open(SOURCE, encoding="utf8"))
    if document["asset"]["volumes"] != source["volumes"]:
        raise SystemExit("catalog and source volumes differ")
    convert(document["asset"])
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False)
    print(CANDIDATE)
    if "--apply" in sys.argv:
        ins = importlib.util.module_from_spec(ins_spec)
        sys.argv = sys.argv[:1]
        text = open(CATALOG, encoding="utf8").read()
        before = json.loads(text)
        text = splice(splice(text, "rig", document["asset"]["rig"], load_insert()), "volumes", document["asset"]["volumes"], load_insert())
        after = json.loads(text)
        for key in before["asset"]:
            if key not in ("rig", "volumes") and before["asset"][key] != after["asset"][key]:
                raise SystemExit(f"member {key} changed")
        open(CATALOG, "w", encoding="utf8").write(text)
        open(SOURCE, "w", encoding="utf8").write(json.dumps({"volumes": document["asset"]["volumes"]}, ensure_ascii=False, separators=(",", ":")) + "\n")
        print("applied")


def load_insert():
    text = open(os.path.join(T, "volumes-insert.py"), encoding="utf8").read()
    scope = {}
    exec(text.split("\ndef main")[0], scope)
    return type("Insert", (), {"dump": staticmethod(scope["dump"]), "member_end": staticmethod(scope["member_end"])})


main()
