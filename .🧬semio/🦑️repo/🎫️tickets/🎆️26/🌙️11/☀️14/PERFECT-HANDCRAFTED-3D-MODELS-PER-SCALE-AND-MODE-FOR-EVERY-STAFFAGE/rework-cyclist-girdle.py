"""🚲 Shoulder and pelvic girdle of `humans/cyclist` on the lateral rest placement, with the shoulders at the true breadth of a 1.72 m woman.

Hand-chosen numbers only: clavicles from the sternoclavicular to the acromioclavicular joint, hip bones from the sacroiliac to the hip joint, glenohumeral centres 0.33 m apart (were 0.38 m), the jacket yoke narrowed to match, arms re-solved so that both hands keep their exact grip frame in every pose and clip key.

Usage: python3 rework-cyclist-girdle.py            writes the candidate `🗑️generated/rework-cyclist/cyclist/🔣️.json`
       python3 rework-cyclist-girdle.py --apply    re-reads the catalog file and the volumes source and writes them
See `rig-lateral-rest.md` (sections 1.3, 1.4, 1.6).
"""
import copy
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
ROOT = T[: T.index("/.🧬semio")]
CATALOG = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "cyclist", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "cyclist.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-cyclist", "cyclist", "🔣️.json")
SCALES = ("50", "100", "200", "500")
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}
ALONG = [0, -0.7071068, 0, 0.7071068]

SHOULDER_Z = 0.165
STERNOCLAVICULAR = (0.392, -0.085, 0.018)
ACROMIOCLAVICULAR = (0.428, -0.018, 0.157)
SACROILIAC = (0.07, 0.04, 0.045)
HIP = (0.0, 0.0, 0.088)
YOKE = ((0.30, 0.0), (0.34, 0.010), (0.38, 0.019), (0.408, 0.022), (0.43, 0.020), (0.446, 0.007), (0.458, 0.0))
CLAVICLE_SHAFT = {
    "50": ((0.0, 0.0, 0.0, 0.012, 0.011), (0.03, 0.001, -0.008, 0.007, 0.006), (0.075, 0.002, -0.012, 0.006, 0.0055), (0.115, 0.001, 0.004, 0.007, 0.005), (0.14, 0.0, 0.006, 0.009, 0.005), (0.156, 0.0, 0.002, 0.008, 0.0045)),
    "100": ((0.0, 0.0, 0.0, 0.012, 0.011), (0.04, 0.001, -0.009, 0.008, 0.008), (0.11, 0.001, 0.003, 0.008, 0.008), (0.156, 0.0, 0.002, 0.009, 0.008)),
    "200": ((0.0, 0.0, 0.0, 0.015, 0.015), (0.075, 0.0, -0.006, 0.015, 0.015), (0.156, 0.0, 0.0, 0.015, 0.015)),
    "500": ((0.0, 0.0, 0.0, 0.038, 0.038), (0.156, 0.0, 0.0, 0.038, 0.038)),
}
ACROMION = {
    "50": (((0.39, 0.05, 0.118), 0.011, 0.005), ((0.412, 0.038, 0.146), 0.011, 0.005), ((0.428, -0.002, 0.166), 0.017, 0.005), ((0.43, -0.01, 0.181), 0.009, 0.004)),
    "100": (((0.39, 0.05, 0.118), 0.011, 0.008), ((0.412, 0.038, 0.146), 0.011, 0.008), ((0.428, -0.002, 0.166), 0.017, 0.008), ((0.43, -0.01, 0.181), 0.009, 0.0075)),
}
ILIAC_BODY = {
    "50": ((0.0, 0.013, 0.016), (0.045, 0.011, 0.018), (0.072, 0.018, 0.02), (0.0914, 0.02, 0.02)),
    "100": ((0.0, 0.013, 0.016), (0.045, 0.011, 0.018), (0.072, 0.018, 0.02), (0.0914, 0.02, 0.02)),
    "200": ((0.0, 0.016, 0.018), (0.045, 0.015, 0.018), (0.0914, 0.02, 0.02)),
}
CONDYLE = ([0.04, -0.004, 0.05], 0.008, 0.007)
SYMPHYSIS = [-0.04, -0.062, 0]
PEDAL_AXLE_Z = 0.0895
CRANK_AXLE = {"50": 0.01, "100": 0.01, "200": 0.015, "500": 0.0375}
JOINT_RADIUS = {"50": 0.012, "100": 0.012, "200": 0.016, "500": 0.038}
TITLES = {
    "clavicle-l": {"en": "Left clavicle", "de": "Linkes Schlüsselbein"},
    "clavicle-r": {"en": "Right clavicle", "de": "Rechtes Schlüsselbein"},
    "ilium-l": {"en": "Left hip bone", "de": "Linkes Hüftbein"},
    "ilium-r": {"en": "Right hip bone", "de": "Rechtes Hüftbein"},
}
HIP_SOLIDS = ("ilium", "ilium-wing", "pubis", "ischium", "ischiopubic-ramus")
DECISION = {
    "en": "Girdles: the clavicles are bones of their own from the sternoclavicular to the acromioclavicular joint and carry scapula, acromion and arm; the hip bones run from the sacroiliac joint to the hip joint and carry the thighs. The glenohumeral centres are 0.33 m apart and the jacket yoke 0.35 m wide, the measure of a 1.72 m woman.",
    "de": "Gürtel: Die Schlüsselbeine sind eigene Knochen vom Sternoklavikular- zum Akromioklavikulargelenk und tragen Schulterblatt, Akromion und Arm; die Hüftbeine laufen vom Iliosakralgelenk zum Hüftgelenk und tragen die Oberschenkel. Die Schultergelenkzentren liegen 0,33 m auseinander, die Jackenschulter ist 0,35 m breit, das Maß einer 1,72 m großen Frau.",
}


def helpers():
    """🧰 The frame and layout helpers of `rig-lateral-standing-adult.py` (its text up to `main`)."""
    text = open(os.path.join(T, "rig-lateral-standing-adult.py"), encoding="utf8").read()
    scope = {"__file__": os.path.join(T, "rig-lateral-standing-adult.py")}
    exec(compile(text[: text.index("\ndef main(")], "rig-lateral-standing-adult.py", "exec"), scope)
    return type("Rig", (), {name: staticmethod(value) for name, value in scope.items() if callable(value)})


RIG = helpers()


def unit(v):
    n = math.sqrt(sum(c * c for c in v))
    return [c / n for c in v]


def basis(u, axis):
    """🧭 Columns u, v, w of a frame along `u` with the joint axis nearest to `axis`."""
    u = unit(u)
    k = sum(axis[i] * u[i] for i in range(3))
    w = unit([axis[i] - k * u[i] for i in range(3)])
    v = [w[1] * u[2] - w[2] * u[1], w[2] * u[0] - w[0] * u[2], w[0] * u[1] - w[1] * u[0]]
    return [[u[i], v[i], w[i]] for i in range(3)]


def mirrored(frame):
    return [[frame[0][0], frame[0][1], -frame[0][2]], [frame[1][0], frame[1][1], -frame[1][2]], [-frame[2][0], -frame[2][1], frame[2][2]]]


def side(point, s):
    return [point[0], point[1], point[2] if s == "l" else -point[2]]


def girdle(start, end, s):
    """🦴 Origin and frame, in the parent frame, of a girdle bone of side `s` (left authored, right mirrored)."""
    left = basis([end[i] - start[i] for i in range(3)], (0.0, 1.0, 0.0))
    return side(start, s), left if s == "l" else mirrored(left)


def local(frame, point):
    origin, m = frame
    return RIG.apply(RIG.transpose(m), [point[i] - origin[i] for i in range(3)])


def placement(frame):
    origin, m = frame
    a, b, t = RIG.angles(m)
    return {"x": round(origin[0], 5) + 0.0, "y": round(origin[1], 5) + 0.0, "z": round(origin[2], 5) + 0.0, "angleDeg": round(a, 4) + 0.0, "abductionDeg": round(b, 4) + 0.0, "twistDeg": round(t, 4) + 0.0}


def child(frame, at, rest):
    """👶 Fields of a child at the parent-of-parent point `at` that keeps the rest matrix `rest` it had under the old parent."""
    origin, m = frame
    return placement((local(frame, at), RIG.mul(RIG.transpose(m), rest)))


def rebound(solid, frame, bone):
    """🔁 The same solid written in the frame of the girdle bone that now carries it."""
    origin, m = frame
    back = RIG.transpose(m)
    q = RIG.quaternion(back)
    if solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = RIG.r5(local(frame, section["at"]))
            section["rotation"] = RIG.r5(RIG.product(q, section.get("rotation", [0, 0, 0, 1])))
    else:
        transform = solid.setdefault("transform", {})
        transform["translation"] = RIG.r5(local(frame, transform.get("translation", [0, 0, 0])))
        transform["rotation"] = RIG.r5(RIG.product(q, transform.get("rotation", [0, 0, 0, 1])))
    solid["bone"] = bone
    solid["member"] = bone


def yoke(x):
    for (x0, d0), (x1, d1) in zip(YOKE, YOKE[1:]):
        if x0 <= x <= x1:
            return d0 + (d1 - d0) * (x - x0) / (x1 - x0)
    return 0.0


def bone_solid(identifier, bone, sections):
    return {"id": identifier, "kind": "loft", "bone": bone, "material": "bone", "role": "body", "tone": 0.9, "sections": sections, "capStart": True, "capEnd": True, "member": bone, "part": "member"}


def joint_solid(bone, radius):
    return {"id": "joint-" + bone, "kind": "ellipsoid", "bone": bone, "material": "bone", "role": "body", "tone": 0.5, "radii": [radius, radius, radius], "transform": {"translation": [0, 0, 0]}, "member": bone, "part": "joint"}


def hand_frame(shoulder, offsets, lengths):
    """✋ Wrist point and hand matrix in the torso frame for the offsets of arm, forearm and hand."""
    arm = RIG.mul(RIG.turn(-158, 0, 0), RIG.turn(*offsets[0:3]))
    forearm = RIG.mul(arm, RIG.turn(offsets[3], 0, 0))
    hand = RIG.mul(forearm, RIG.turn(offsets[4], offsets[5], 0))
    elbow = [shoulder[i] + arm[i][0] * lengths[0] for i in range(3)]
    return [elbow[i] + forearm[i][0] * lengths[1] for i in range(3)], hand


def miss(shoulder, offsets, lengths, target):
    wrist, hand = hand_frame(shoulder, offsets, lengths)
    e = RIG.mul(RIG.transpose(target[1]), hand)
    return [wrist[i] - target[0][i] for i in range(3)] + [0.1 * (e[2][1] - e[1][2]) / 2, 0.1 * (e[0][2] - e[2][0]) / 2, 0.1 * (e[1][0] - e[0][1]) / 2]


def gauss(a, b):
    n = len(b)
    m = [row[:] + [b[i]] for i, row in enumerate(a)]
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(m[r][c]))
        m[c], m[p] = m[p], m[c]
        for r in range(n):
            if r != c:
                f = m[r][c] / m[c][c]
                m[r] = [m[r][k] - f * m[c][k] for k in range(n + 1)]
    return [m[i][n] / m[i][i] for i in range(n)]


def reach(offsets, lengths):
    """🎯 Left-arm offsets from the narrower shoulder that put the hand into exactly the frame it had."""
    old = [offsets["arm"].get("angleDeg", 0.0), offsets["arm"].get("abductionDeg", 0.0), offsets["arm"].get("twistDeg", 0.0), offsets["forearm"].get("angleDeg", 0.0), offsets["hand"].get("angleDeg", 0.0), offsets["hand"].get("abductionDeg", 0.0)]
    target = hand_frame((0.4057, 0.0, 0.19), old, lengths)
    shoulder = (0.4057, 0.0, SHOULDER_Z)
    x = old[:]
    for _ in range(40):
        f = miss(shoulder, x, lengths, target)
        if max(abs(c) for c in f) < 1e-10:
            break
        jac = [[0.0] * 6 for _ in range(6)]
        for k in range(6):
            y = x[:]
            y[k] += 1e-5
            g = miss(shoulder, y, lengths, target)
            for i in range(6):
                jac[i][k] = (g[i] - f[i]) / 1e-5
        step = gauss(jac, [-c for c in f])
        x = [x[k] + step[k] for k in range(6)]
    f = miss(shoulder, x, lengths, target)
    if max(abs(c) for c in f) > 1e-6:
        raise SystemExit(f"hand frame not reached: {f}")
    return [round(c, 4) + 0.0 for c in x]


def rearm(bones, lengths):
    """💪 Re-solves both arms of one pose or clip key (the right side mirrors the left)."""
    if "arm-l" not in bones:
        return None
    left = {part: bones.get(part + "-l", {}) for part in ("arm", "forearm", "hand")}
    right = {part: bones.get(part + "-r", {}) for part in ("arm", "forearm", "hand")}
    for part in left:
        for key in ("angleDeg", "abductionDeg", "twistDeg"):
            sign = 1 if key == "angleDeg" else -1
            if abs(left[part].get(key, 0.0) - sign * right[part].get(key, 0.0)) > 1e-6:
                raise SystemExit(f"arms are not mirrored in {part}.{key}")
    x = reach(left, lengths)
    for s, sign in (("l", 1), ("r", -1)):
        bones["arm-" + s] = {"angleDeg": x[0], "abductionDeg": sign * x[1], "twistDeg": sign * x[2]}
        bones["forearm-" + s] = {**bones.get("forearm-" + s, {}), "angleDeg": x[3]}
        bones["hand-" + s] = {**bones.get("hand-" + s, {}), "angleDeg": x[4], "abductionDeg": round(sign * x[5], 4) + 0.0}
    return x


def fat(solid, radius):
    """🪵 A loft never thinner than the feature of its scale."""
    for section in solid["sections"]:
        section["rx"], section["ry"] = max(section["rx"], radius), max(section["ry"], radius)


def drop(lists, names):
    """✂️ Removes parts a scale cannot show, with the thermal and structure solids that refer to them."""
    for name in ("body", "thermal", "structure"):
        lists[name] = [solid for solid in lists[name] if solid["id"] not in names and solid.get("of") not in names]


def rod(identifier, like, start, end, radius):
    return {"id": identifier, "kind": "loft", "bone": "frame", "material": like["material"], "role": like.get("role", "body"), "tone": like.get("tone", 0.5), "sections": [{"at": list(start), "rx": radius, "ry": radius}, {"at": list(end), "rx": radius, "ry": radius}], "capStart": True, "capEnd": True}


def hardware(lists, scale):
    """🔩 The bicycle hardware that hung in the air: seat post in the seat tube, brake hanger, chain-guard straps, fender stays on the fender."""
    body = lists["body"]
    ids = {solid["id"]: solid for solid in body}
    r = max(0.004, FEATURE[scale] / 2)
    for solid in body:
        if solid["id"] == "seatpost" and solid["profile"][0][0] == 0:
            solid["profile"][0][0] = solid["profile"][1][0] = -0.06
        if "-fender-stay-" in solid["id"]:
            hub, half = ((0.0, 0.34), 0.029) if solid["id"].startswith("rear") else ((1.05, 0.34), 0.027)
            for section in solid["sections"]:
                if math.hypot(section["at"][0] - hub[0], section["at"][1] - hub[1]) > 0.3:
                    section["at"][2] = math.copysign(half, section["at"][2])
    if "brake-rear-l" in ids and "brake-rear-hanger" not in ids:
        like = ids["brake-rear-l"]
        body.append(rod("brake-rear-yoke", like, (0.2, 0.672, -0.044), (0.2, 0.672, 0.044), max(0.005, FEATURE[scale] / 2)))
        body.append(rod("brake-rear-hanger", like, (0.2, 0.672, 0), (0.277, 0.675, 0), max(0.005, FEATURE[scale] / 2)))
        body.append(rod("brake-rear-bolt", like, (0.277, 0.675, -0.04), (0.277, 0.675, 0.04), max(0.005, FEATURE[scale] / 2)))
    if "chain-guard" in ids and scale in ("50", "100") and "chain-guard-strap-front" not in ids:
        like = ids["chain-guard"]
        body.append(rod("chain-guard-strap-front", like, (0.2, 0.324, -0.06), (0.2, 0.396, -0.066), r))
        body.append(rod("chain-guard-strap-rear", like, (0.062, 0.4, -0.066), (0.062, 0.43, -0.062), r))
    structure = {solid["id"] for solid in lists["structure"]}
    if scale == "200":
        for s in ("l", "r"):
            if "shoe-sole-" + s in ids and "st-shoe-sole-" + s not in structure:
                lists["structure"].append({"id": "st-shoe-sole-" + s, "of": "shoe-sole-" + s, "member": "foot-" + s, "part": "member"})
    if "seat-collar" in ids and "st-seatpost" in structure and "st-seat-collar" not in structure:
        lists["structure"].append({"id": "st-seat-collar", "of": "seat-collar", "member": "frame", "part": "member"})


def coarse(lists, scale):
    """🔍 Parts of rider and bicycle modelled for the smallest feature of 1:100, 1:200 and 1:500 (15, 30, 75 mm)."""
    if scale == "50":
        return
    r = FEATURE[scale] / 2
    gone = set()
    if scale in ("200", "500"):
        gone |= {f"fingers-{s}-{k}" for s in "lr" for k in range(4)} | {"glove-thumb-l", "glove-thumb-r", "pedal-spindle-l", "pedal-spindle-r"}
    if scale == "500":
        gone |= {"shoe-sole-l", "shoe-sole-r", "shoe-toe-sole-l", "shoe-toe-sole-r", "tail-lamp", "carrier-stay-l", "carrier-stay-r"} | {f"carrier-bar-{k}" for k in range(5)}
    drop(lists, gone)
    for solid in lists["body"]:
        name = solid["id"]
        stem, _, s = name.rpartition("-")
        sign = -1 if s == "r" else 1
        if stem in ("carrier-stay", "handlebar") or name.startswith("carrier-bar-"):
            fat(solid, 0.0356 if scale == "500" else r)
        if stem == "pedal-spindle":
            solid["profile"] = [[math.copysign(0.079, a) if abs(a) < 0.09 else a, 0.0075 if b else 0] for a, b in solid["profile"]]
        if stem == "glove" and scale in ("200", "500"):
            for section in solid["sections"]:
                fist = section["at"][0] >= 0.06
                section["ry"] = max(section["ry"], 0.0325 if scale == "500" else 0.028 if fist else 0.02)
                section["rx"] = max(section["rx"], r)
                if fist:
                    section["at"][1] = -0.01
        if stem == "shoe-toe" and scale in ("200", "500"):
            fat(solid, 0.0205 if scale == "500" else 0.015)
        if stem in ("shoe-sole", "shoe-toe-sole") and scale == "200":
            solid["from"] = round(solid["to"] - 0.03, 4) if solid["to"] > solid["from"] else round(solid["to"] + 0.03, 4)
        if stem == "pedal" and scale in ("200", "500"):
            solid["profile"] = [[a, math.copysign(0.015, b)] for a, b in solid["profile"]]
            near = "from" if abs(solid["from"]) < abs(solid["to"]) else "to"
            solid[near] = math.copysign(0.085, solid[near])
        if name == "tail-lamp" and scale == "200":
            solid["size"] = [0.03, 0.04, 0.07]
        if name in ("rear-rim", "front-rim") and scale in ("200", "500"):
            solid["profile"] = [[round(a * (3 if scale == "500" else 1.25), 4), b] for a, b in solid["profile"]]
        if scale == "500":
            if stem == "crank-arm":
                solid["profile"] = [[a, math.copysign(0.0375, b) if b else 0] for a, b in solid["profile"]]
                solid["from"], solid["to"] = (0.04, 0.115) if solid["to"] > 0 else (-0.115, -0.04)
            if name == "bar-clamp":
                solid["profile"] = [[-0.036, 0], [-0.036, 0.036], [0.036, 0.036], [0.036, 0]]
            if stem == "grip":
                solid["profile"] = [[a, 0.036 if b else 0] for a, b in solid["profile"]]
            if name in ("rear-tyre", "front-tyre"):
                solid["profile"] = [[round(0.0375 * math.sin(k * math.pi / 12), 4), round(0.322 + 0.018 * math.cos(k * math.pi / 12), 4)] for k in range(24)] + [[0, 0.34]]
            if name in ("rear-hub", "front-hub"):
                solid["profile"] = [[a, round(b * 1.25, 4)] for a, b in solid["profile"]]


def convert_rig(asset):
    bones = asset["rig"]["bones"]
    by_id = {bone["id"]: bone for bone in bones}
    if "clavicle-l" in by_id:
        return None
    frames = {}
    out = []
    for bone in bones:
        for s in ("l", "r"):
            if bone["id"] == "arm-" + s:
                frame = girdle(STERNOCLAVICULAR, ACROMIOCLAVICULAR, s)
                frames["clavicle-" + s] = frame
                out.append({"id": "clavicle-" + s, "parent": "torso", "title": TITLES["clavicle-" + s], **placement(frame), "lengthM": 0.156, "joint": {"type": "revolute", "minDeg": -8, "maxDeg": 35}})
                bone.update(parent="clavicle-" + s, **child(frame, side((0.4057, 0.0, SHOULDER_Z), s), RIG.turn(-158, 0, 0)))
            if bone["id"] == "thigh-" + s:
                frame = girdle(SACROILIAC, HIP, s)
                frames["ilium-" + s] = frame
                out.append({"id": "ilium-" + s, "parent": "pelvis", "title": TITLES["ilium-" + s], **placement(frame), "lengthM": 0.0914, "joint": {"type": "revolute", "minDeg": -3, "maxDeg": 3}})
                bone.update(parent="ilium-" + s, **child(frame, side(HIP, s), RIG.turn(-170, 0, 0)))
        out.append(bone)
    asset["rig"]["bones"] = out
    return frames


def convert_motion(asset):
    by_id = {bone["id"]: bone for bone in asset["rig"]["bones"]}
    lengths = (by_id["forearm-l"]["x"], by_id["hand-l"]["x"])
    for pose in asset["poses"]:
        print(pose["id"], rearm(pose["bones"], lengths))
    for clip in asset["clips"]:
        solved = [rearm(key["bones"], lengths) for key in clip["keys"] if "bones" in key]
        print(clip["id"], len([x for x in solved if x]), "keys re-solved")


def convert_volumes(volumes, frames):
    for scale in SCALES:
        lists = volumes[scale]
        for name in ("body", "thermal", "structure"):
            for solid in lists[name]:
                if solid.get("kind") == "loft" and solid.get("bone") == "torso" and "jacket" in solid["id"]:
                    for section in solid["sections"]:
                        cut = yoke(section["at"][0])
                        if cut and section["rx"] > 0.12:
                            section["rx"] = round(section["rx"] - cut, 4)
        coarse(lists, scale)
        hardware(lists, scale)
        structure = lists["structure"]
        kept = []
        for solid in structure:
            stem, _, s = solid["id"].rpartition("-")
            if stem == "clavicle" and solid.get("bone") == "torso":
                continue
            if stem == "scapula" and solid.get("bone") == "torso":
                rebound(solid, frames["clavicle-" + s], "clavicle-" + s)
            if stem == "mandible-ramus" and scale in ("50", "100") and solid["sections"][-1]["at"][0] < 0.03:
                point, rx, ry = CONDYLE
                solid["sections"].append({"at": side(point, s), "rx": rx, "ry": ry, "rotation": solid["sections"][-1]["rotation"]})
            if solid["id"] == "joint-pelvis" and scale != "500":
                solid["transform"]["translation"] = SYMPHYSIS
            if stem == "joint-pedal":
                solid["transform"]["translation"] = [0, 0, PEDAL_AXLE_Z if s == "l" else -PEDAL_AXLE_Z]
            if stem in HIP_SOLIDS and solid.get("bone") == "pelvis" and s in ("l", "r"):
                rebound(solid, frames["ilium-" + s], "ilium-" + s)
            kept.append(solid)
        for s in ("l", "r"):
            z = 1 if s == "l" else -1
            bone = "clavicle-" + s
            kept.append(bone_solid(bone, bone, [{"at": [x, y, z * w], "rx": rx, "ry": ry, "rotation": ALONG} for x, y, w, rx, ry in CLAVICLE_SHAFT[scale]]))
            if scale in ACROMION:
                frame = frames["clavicle-l"]
                sections = []
                for point, rx, ry in ACROMION[scale]:
                    p = local(frame, point)
                    sections.append({"at": [round(p[0], 4), round(p[1], 4), round(z * p[2], 4)], "rx": rx, "ry": ry, "rotation": ALONG})
                kept.append(bone_solid("acromion-" + s, bone, sections))
            kept.append(joint_solid(bone, JOINT_RADIUS[scale]))
            hip = "ilium-" + s
            if scale in ILIAC_BODY:
                kept.append(bone_solid("ilium-body-" + s, hip, [{"at": [x, 0, 0], "rx": rx, "ry": ry, "rotation": ALONG} for x, rx, ry in ILIAC_BODY[scale]]))
            if scale == "500":
                kept.append(bone_solid("hip-bone-" + s, hip, [{"at": [0.0, 0, 0], "rx": 0.038, "ry": 0.038, "rotation": ALONG}, {"at": [0.0914, 0, 0], "rx": 0.038, "ry": 0.038, "rotation": ALONG}]))
            kept.append(joint_solid(hip, JOINT_RADIUS[scale]))
        r = CRANK_AXLE[scale]
        if not any(solid["id"] == "st-bb-spindle" for solid in kept):
            kept.append({"id": "st-crank-axle", "kind": "revolve", "bone": "frame", "material": "steel", "role": "body", "tone": 0.9, "profile": [[-0.087, 0], [-0.087, r], [0.087, r], [0.087, 0]], "axis": [0, 0, 1], "origin": [0.45, 0.3, 0], "member": "frame", "part": "member"})
        lists["structure"] = kept
        lists["decisions"] = lists["decisions"] + [DECISION]


def convert_drawings(asset):
    """✏️ Jacket yoke of the front and plan drawings drawn in to the new shoulder breadth."""
    for scale in SCALES:
        geometry = asset["representations"][scale]["geometry"]
        for shape in geometry["front"]:
            if shape["id"] == "jacket":
                for point in shape["points"]:
                    cut = yoke((point[1] - 1.0951) / 0.9272)
                    wide = min(1.0, max(0.0, (abs(point[0]) - 0.13) / 0.04))
                    point[0] = round(point[0] - math.copysign(cut * wide, point[0]), 4)
        for shape in geometry["plan"]:
            if shape["id"] == "jacket":
                for point in shape["points"]:
                    ahead = min(1.0, max(0.0, (point[0] - 0.35) / 0.03))
                    wide = min(1.0, max(0.0, (abs(point[1]) - 0.13) / 0.05))
                    point[1] = round(point[1] - math.copysign(0.022 * ahead * wide, point[1]), 4)


def convert(asset, volume_sets):
    frames = convert_rig(asset)
    if frames is None:
        return False
    convert_motion(asset)
    for volumes in volume_sets:
        convert_volumes(volumes, frames)
    convert_drawings(asset)
    return True


def respliced(text, key, value):
    """🧷 Replaces one member of the asset in the catalog text in the layout of `volumes-insert.py`; every other byte stays."""
    import re
    insert = RIG.insert_module()
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    start, end = insert.member_range(text, indent, key)
    return text[:start] + indent + json.dumps(key) + ": " + insert.dump(value, len(indent) // 2) + text[end:]


def main():
    text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    source_text = open(SOURCE, encoding="utf8").read()
    source = json.loads(source_text)
    if "--apply" not in sys.argv:
        document["asset"]["volumes"] = source["volumes"]
        if not convert(document["asset"], [document["asset"]["volumes"]]):
            raise SystemExit("already converted")
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        for bone in document["asset"]["rig"]["bones"]:
            if bone["id"][:-2] in ("clavicle", "arm", "ilium", "thigh"):
                print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass")}))
        print(CANDIDATE)
        return
    before = copy.deepcopy(document["asset"])
    if not convert(document["asset"], [source["volumes"]]):
        raise SystemExit("already converted")
    for key in ("rig", "poses", "clips", "representations"):
        text = respliced(text, key, document["asset"][key])
    if json.loads(text) != document:
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf8").read() != json.dumps(None) and json.loads(open(CATALOG, encoding="utf8").read())["asset"] != before:
        raise SystemExit("the catalog file changed while converting: run again")
    open(CATALOG, "w", encoding="utf8").write(text)
    if open(SOURCE, encoding="utf8").read() != source_text:
        raise SystemExit("the volumes source changed while converting: catalog written, run again for the source")
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, separators=(",", ":"))
    print("applied to", CATALOG, "and", SOURCE, "- now run volumes-insert.py")


if __name__ == "__main__":
    main()
