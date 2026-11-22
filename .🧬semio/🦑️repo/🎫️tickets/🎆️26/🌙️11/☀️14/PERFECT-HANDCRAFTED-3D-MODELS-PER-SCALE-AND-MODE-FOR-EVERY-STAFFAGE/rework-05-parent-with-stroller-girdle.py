"""🦴 Shoulder and pelvic girdle of `humans/parent-with-stroller` on lateral rest placement, shoulder joint at its anatomical height and breadth.

Usage: python3 rework-05-parent-with-stroller-girdle.py   (reads the catalog asset once, writes the candidate 🗑️generated/rework-parent-with-stroller/pws-cand/🔣️.json; then run the coarse script, then the publish script)
Hand-chosen numbers (rest pose, root at the hip-joint axis, [ahead, up, left], metres): sternoclavicular joint [0.066, 0.490, ±0.025], acromioclavicular joint [0, 0.502, ±0.185], glenohumeral centre [0, 0.4654, ±0.178] (was [0, 0.527, ±0.1925]), sacroiliac joint [-0.055, 0.070, ±0.035], hip joint [0, 0, ±0.092].
See rig-lateral-rest.md section 1.
"""
import json
import math
import os

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/parent-with-stroller/🔣️.json"
SOURCE = T + "/volumes/humans/parent-with-stroller.json"
CANDIDATE = T + "/🗑️generated/rework-parent-with-stroller/pws-cand/🔣️.json"


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (
            all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160
        )
    return False


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


SHOULDER_OLD, SHOULDER_NEW, HALF_OLD, HALF_NEW = 0.4276, 0.366, 0.1925, 0.178
L1, L2 = 0.3082, 0.2685
CLAVICLE = math.sqrt(0.066**2 + 0.012**2 + 0.16**2)
ILIUM = math.sqrt(0.055**2 + 0.07**2 + 0.057**2)


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def apply(m, p):
    return [sum(m[i][k] * p[k] for k in range(3)) for i in range(3)]


def transpose(m):
    return [[m[j][i] for j in range(3)] for i in range(3)]


def rest(angle, abduction, twist):
    a, b, c = (math.radians(v) for v in (angle, abduction, twist))
    rw = [[math.cos(a), -math.sin(a), 0], [math.sin(a), math.cos(a), 0], [0, 0, 1]]
    rv = [[math.cos(b), 0, -math.sin(b)], [0, 1, 0], [math.sin(b), 0, math.cos(b)]]
    ru = [[1, 0, 0], [0, math.cos(c), math.sin(c)], [0, -math.sin(c), math.cos(c)]]
    return mul(mul(rw, rv), ru)


def quaternion(m):
    w = math.sqrt(max(0, 1 + m[0][0] + m[1][1] + m[2][2])) / 2
    x = math.sqrt(max(0, 1 + m[0][0] - m[1][1] - m[2][2])) / 2
    y = math.sqrt(max(0, 1 - m[0][0] + m[1][1] - m[2][2])) / 2
    z = math.sqrt(max(0, 1 - m[0][0] - m[1][1] + m[2][2])) / 2
    x = math.copysign(x, m[2][1] - m[1][2])
    y = math.copysign(y, m[0][2] - m[2][0])
    z = math.copysign(z, m[1][0] - m[0][1])
    return [x, y, z, w]


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def r5(values):
    return [round(v, 5) for v in values]


UPRIGHT = [[0, -1, 0], [1, 0, 0], [0, 0, 1]]
FRAMES = {"torso": ([0, 0.0994, 0], UPRIGHT), "pelvis": ([0, 0, 0], UPRIGHT)}
GIRDLE = {
    "clavicle-l": ("torso", [0.3906, -0.066, 0.025], (79.6952, 67.2535, 168.8471)),
    "clavicle-r": ("torso", [0.3906, -0.066, -0.025], (79.6952, -67.2535, -168.8471)),
    "ilium-l": ("pelvis", [0.07, 0.055, 0.035], (-141.8428, 32.6309, -67.0387)),
    "ilium-r": ("pelvis", [0.07, 0.055, -0.035], (-141.8428, -32.6309, 67.0387)),
}
for bone, (parent, at, angles) in GIRDLE.items():
    origin, basis = FRAMES[parent]
    FRAMES[bone] = ([o + d for o, d in zip(origin, apply(basis, at))], mul(basis, rest(*angles)))


def local(bone, parent, point):
    origin, basis = FRAMES[parent]
    world = [o + d for o, d in zip(origin, apply(basis, point))]
    target, frame = FRAMES[bone]
    return r5(apply(transpose(frame), [w - t for w, t in zip(world, target)]))


def rebind(solid, bone):
    parent = solid["bone"]
    turn = mul(transpose(FRAMES[bone][1]), FRAMES[parent][1])
    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = local(bone, parent, solid["p0"]), local(bone, parent, solid["p1"])
    else:
        old = solid.get("transform", {})
        moved = {"translation": local(bone, parent, old.get("translation", [0, 0, 0]))}
        moved["rotation"] = r5(qmul(quaternion(turn), old.get("rotation", [0, 0, 0, 1])))
        solid["transform"] = moved
    solid["bone"] = bone
    solid["member"] = bone


def side(name):
    return 1 if name.endswith("-l") else -1


def mirror(point, sign):
    return [point[0], point[1], point[2] * sign]


def member(identifier, bone, **fields):
    return {"id": identifier, "bone": bone, "material": "bone", "role": "body", "tone": 0.93, **fields, "member": bone, "part": "member"}


def capsule(identifier, bone, parent, a, b, radius, **fields):
    sign = side(identifier)
    return member(identifier, bone, kind="capsule", p0=local(bone, parent, mirror(a, sign)), p1=local(bone, parent, mirror(b, sign)), r0=radius, r1=radius, **fields)


def clavicle(suffix, scale):
    bone = "clavicle-" + suffix
    if scale != "50":
        fields = {} if scale == "100" else {"slender": True}
        return member(bone, bone, kind="capsule", p0=[0 if scale == "100" else -0.014, 0, 0], p1=[round(CLAVICLE, 5), 0, 0], r0=0.0075, r1=0.0075, **fields)
    stations = [(0, 0, 0.011, 0.011), (0.03, -0.004, 0.0085, 0.0085), (0.085, -0.008, 0.0065, 0.0065), (0.14, 0.003, 0.0065, 0.009), (round(CLAVICLE, 5), 0, 0.006, 0.011)]
    sections = [{"at": [x, 0, w], "rx": rx, "ry": ry, "exponent": 2, "rotation": [0.5, 0.5, 0.5, 0.5]} for x, w, rx, ry in stations]
    return member(bone, bone, kind="loft", capStart=True, capEnd=True, sections=sections)


SHOULDER = [(0.365, 0.197, 0.12, 2.45, -0.002), (0.392, 0.198, 0.113, 2.4, 0), (0.41, 0.184, 0.106, 2.3, 0.002), (0.428, 0.156, 0.098, 2.2, 0.004), (0.446, 0.126, 0.089, 2.15, 0.006), (0.464, 0.1, 0.079, 2.1, 0.008), (0.48, 0.084, 0.07, 2.05, 0.009), (0.494, 0.076, 0.064, 2, 0.009)]
KEEP = {"50": 0.34, "100": 0.35, "200": 0.27, "500": 0.31}
STATIONS = {"50": [0.365, 0.392, 0.41, 0.428, 0.446, 0.464, 0.48], "100": [0.385, 0.415, 0.455], "200": [0.37, 0.405, 0.45], "500": [0.385, 0.43]}


def shoulder(x):
    for low, high in zip(SHOULDER, SHOULDER[1:]):
        if low[0] <= x <= high[0]:
            t = (x - low[0]) / (high[0] - low[0])
            return [a + (b - a) * t for a, b in zip(low, high)]
    raise SystemExit("station outside the shoulder table")


def jacket(solid, scale, inset=(0, 0)):
    sections = solid["sections"]
    kept, last = [s for s in sections if s["at"][0] <= KEEP[scale]], sections[-1]
    for station in STATIONS[scale]:
        x, rx, ry, exponent, back = shoulder(station)
        kept.append({"at": [x, round(back, 5), 0], "rx": round(rx - inset[0], 5), "ry": round(ry - inset[1], 5), "exponent": round(exponent, 3), "rotation": [0, -0.70711, 0, 0.70711]})
    solid["sections"] = kept + [last]


def reach(offsets, name):
    suffix = name[-2:]
    upper, fore, hand = (offsets.get(b + suffix, {}).get("angleDeg", 0) for b in ("upper-arm", "forearm", "hand"))
    a = math.radians(upper - 180)
    b = a + math.radians(fore)
    wrist = (SHOULDER_OLD + L1 * math.cos(a) + L2 * math.cos(b), L1 * math.sin(a) + L2 * math.sin(b))
    d = (wrist[0] - SHOULDER_NEW, wrist[1])
    distance = min(math.hypot(*d), L1 + L2 - 1e-6)
    bend = math.acos(max(-1, min(1, (distance**2 - L1**2 - L2**2) / (2 * L1 * L2))))
    swing = math.atan2(d[1], d[0]) - math.atan2(L2 * math.sin(bend), L1 + L2 * math.cos(bend))
    wrap = lambda v: round((v + 180) % 360 - 180, 4)
    result = {"upper-arm" + suffix: wrap(math.degrees(swing) + 180), "forearm" + suffix: wrap(math.degrees(bend)), "hand" + suffix: wrap(upper + fore + hand - math.degrees(swing) - 180 - math.degrees(bend))}
    return result, math.hypot(*d) - (L1 + L2)


def repose(bones, base, label):
    merged = {**base, **bones}
    for suffix in ("-l", "-r"):
        names = ["upper-arm" + suffix, "forearm" + suffix, "hand" + suffix]
        if base and not any(n in bones for n in names):
            continue
        solved, over = reach(merged, suffix)
        if over > 0:
            print("[DEBUG] out of reach by", round(over, 4), label, suffix)
        for name, angle in solved.items():
            bones[name] = {**merged.get(name, {}), "angleDeg": angle}
        print(label, suffix, [merged.get(n, {}).get("angleDeg", 0) for n in names], "->", [solved[n] for n in names])


def rig(bones):
    def bone(identifier, en, de, low, high):
        parent, at, (angle, abduction, twist) = GIRDLE[identifier]
        length = CLAVICLE if "clavicle" in identifier else ILIUM
        return {"id": identifier, "parent": parent, "title": {"en": en, "de": de}, "x": at[0], "y": at[1], "z": at[2], "angleDeg": angle, "abductionDeg": abduction, "twistDeg": twist, "lengthM": round(length, 5), "joint": {"type": "revolute", "minDeg": low, "maxDeg": high}}

    index = {b["id"]: b for b in bones}
    limbs = {
        "upper-arm-l": ("clavicle-l", 0.16451, -0.03597, 0.0037, -93.9677, 1.6304, 112.3031),
        "upper-arm-r": ("clavicle-r", 0.16451, -0.03597, -0.0037, -93.9677, -1.6304, -112.3031),
        "thigh-l": ("ilium-l", round(ILIUM, 5), 0, 0, -43.6369, 23.7948, 68.9577),
        "thigh-r": ("ilium-r", round(ILIUM, 5), 0, 0, -43.6369, -23.7948, -68.9577),
    }
    for identifier, (parent, x, y, z, angle, abduction, twist) in limbs.items():
        index[identifier].update({"parent": parent, "x": x, "y": y, "z": z, "angleDeg": angle, "abductionDeg": abduction, "twistDeg": twist})
    index["stroller"]["z"] = round(HALF_NEW - HALF_OLD, 5)
    at = [b["id"] for b in bones].index("upper-arm-l")
    bones[at:at] = [bone("clavicle-l", "Left clavicle and shoulder blade", "Linkes Schlüsselbein mit Schulterblatt", -8, 35), bone("clavicle-r", "Right clavicle and shoulder blade", "Rechtes Schlüsselbein mit Schulterblatt", -8, 35)]
    at = [b["id"] for b in bones].index("thigh-l")
    bones[at:at] = [bone("ilium-l", "Left hip bone", "Linkes Hüftbein", -5, 5), bone("ilium-r", "Right hip bone", "Rechtes Hüftbein", -5, 5)]


def structure(solids, scale):
    result = []
    for solid in solids:
        identifier = solid["id"]
        if identifier in ("clavicle-l", "clavicle-r"):
            result.append(clavicle(identifier[-1], scale))
        elif identifier in ("scapular-spine-l", "scapular-spine-r"):
            result.append(capsule(identifier, "clavicle" + identifier[-2:], "torso", [0.372, 0.098, 0.075], [0.4026, 0.006, 0.187], solid["r0"]))
            result.append(capsule("scapula-neck" + identifier[-2:], "clavicle" + identifier[-2:], "torso", [0.366, 0.05, 0.135], [0.366, 0.014, 0.163], 0.012))
        elif identifier in ("scapula-l", "scapula-r"):
            rebind(solid, "clavicle" + identifier[-2:])
            result.append(solid)
        elif identifier in ("ilium-l", "ilium-r", "ischium-l", "ischium-r"):
            rebind(solid, "ilium" + identifier[-2:])
            result.append(solid)
            if identifier.startswith("ischium"):
                result.append(capsule("pubic-ramus" + identifier[-2:], "ilium" + identifier[-2:], "pelvis", [-0.03, -0.062, 0.05], [-0.004, -0.012, 0.084], 0.009))
        elif identifier == "shoulder-girdle":
            radius = 0.015 if scale == "200" else 0.0375
            a, b = ([0.36, 0.095, 0.08], [0.372, 0.02, 0.165]) if scale == "200" else ([0.355, 0.085, 0.09], [0.37, 0.03, 0.15])
            for suffix in ("l", "r"):
                result.append(clavicle(suffix, scale))
                result.append(capsule("scapula-" + suffix, "clavicle-" + suffix, "torso", a, b, radius))
        elif identifier == "pelvic-girdle":
            result.append(solid)
            radii = [0.05, 0.022, 0.03] if scale == "200" else [0.058, 0.04, 0.045]
            for suffix in ("l", "r"):
                result.append(member("hip-bone-" + suffix, "ilium-" + suffix, kind="ellipsoid", radii=radii, transform={"translation": [round(ILIUM / 2, 5), 0, 0]}))
        else:
            result.append(solid)
    return result


def volumes(model):
    for scale in ("50", "100", "200", "500"):
        lists = model[scale]
        body = next(s for s in lists["body"] if s["id"] == "jacket-body")
        before = {round(s["at"][0], 4): (s["rx"], s["ry"]) for s in body["sections"]}
        for solid in lists["thermal"]:
            if solid.get("bone") == "torso" and solid.get("kind") == "loft" and solid["id"].startswith("th-jacket-body"):
                station = max(x for x in before if x <= KEEP[scale])
                own = next(s for s in solid["sections"] if abs(s["at"][0] - station) < 1e-4)
                jacket(solid, scale, (before[station][0] - own["rx"], before[station][1] - own["ry"]))
        jacket(body, scale)
        lists["structure"] = structure(lists["structure"], scale)


document = json.load(open(CATALOG, encoding="utf-8"))
text = open(CATALOG, encoding="utf-8").read()
print("round trip identical:", dump(document, 0) + "\n" == text)
asset = document["asset"]
if any(b["id"] == "clavicle-l" for b in asset["rig"]["bones"]):
    raise SystemExit("already applied to the catalog asset")
rig(asset["rig"]["bones"])
poses = {}
for pose in asset["poses"]:
    repose(pose["bones"], {}, pose["id"])
    poses[pose["id"]] = pose["bones"]
for clip in asset["clips"]:
    for key in clip["keys"]:
        if "bones" in key:
            repose(key["bones"], poses.get(key.get("pose"), {}), clip["id"] + "@" + str(key["t"]))
volumes(asset["volumes"])
for scale, representation in asset["representations"].items():
    for view in ("front", "plan"):
        loose = [s["id"] for s in representation["geometry"][view] if "solid" not in s]
        print(scale, view, "shapes without solid:", len(loose), loose[:12])
out = dump(document, 0) + "\n"
os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
open(CANDIDATE, "w", encoding="utf-8").write(out)
print("candidate", CANDIDATE)
