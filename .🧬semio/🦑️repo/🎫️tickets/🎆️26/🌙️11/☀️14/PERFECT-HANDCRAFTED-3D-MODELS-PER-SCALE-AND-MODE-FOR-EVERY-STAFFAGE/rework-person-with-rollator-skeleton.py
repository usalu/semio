"""🦴 person-with-rollator: shoulder and pelvic girdles on lateral rest bones, and the skeleton, joints and coarse scales corrected by hand (see `rework/assets/humans--person-with-rollator.md`).

Every number below is a hand-chosen world rest coordinate `[ahead, up, left]` in metres of the left side (the right side is its mirror) or a bone-local value; the script only converts them into the bone frames.

Usage: python3 rework-person-with-rollator-skeleton.py            writes the candidate `🗑️generated/rework-person-with-rollator/cand/🔣️.json`
       python3 rework-person-with-rollator-skeleton.py --apply    writes the rig into the catalog file and the volumes into the source (publish them with `volumes-insert.py`)
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "person-with-rollator", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "person-with-rollator.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-person-with-rollator", "cand", "🔣️.json")

spec = importlib.util.spec_from_file_location("lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)

SCALES = ("50", "100", "200", "500")
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}
LIMB = [0.5, 0.5, 0.5, 0.5]

STERNOCLAVICULAR = (0.078, 1.312, 0.018)
ACROMIOCLAVICULAR = (0.008, 1.327, 0.146)
SACROILIAC = (-0.047, 0.922, 0.035)
HIP = (0.0, 0.8494, 0.0846)
SCAPULA_CENTRE = (-0.0485, 1.235, 0.1025)
SCAPULA_LONG = (0.438, 0.850, 0.294)
SCAPULA_NORMAL = (-0.80, 0.25, 0.55)
SCAPULA_RADII = (0.0765, 0.046)
SPINE_ROOT = (-0.070, 1.275, 0.072)
ACROMION_ANGLE = (-0.030, 1.318, 0.148)
ACROMION_TIP = (0.004, 1.328, 0.150)
CORACOID = ((-0.010, 1.312, 0.120), (0.024, 1.300, 0.122))
CLAVICLE_STATIONS = ((0.0, 0.0, 0.0105, 0.0115), (0.25, 0.007, 0.0075, 0.007), (0.5, 0.006, 0.0062, 0.006), (0.78, -0.006, 0.005, 0.0075), (1.0, -0.002, 0.005, 0.009))
SHEET = 0.0022
GLENOID = ((-0.0284, 1.274, 0.116), (-0.006, 1.3005, 0.130))
STERNAL_END = {"50": 0.018, "100": 0.010, "200": 0.006, "500": 0.006}
SHOULDER_CAP = {"centre": (0.3846, 0.004, 0.140), "radii": (0.030, 0.050, 0.045)}


def side(point, sign):
    return [point[0], point[1], sign * point[2]]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def unit(v):
    n = math.sqrt(sum(c * c for c in v))
    return [c / n for c in v]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def local(frame, point):
    return K.r5(K.apply(K.transpose(frame[1]), sub(point, frame[0])))


def turned(frame, axes):
    """Quaternion of an ellipsoid whose axes are the world vectors `axes`, in the frame of its bone."""
    a = [unit(axis) for axis in axes]
    if dot(cross(a[0], a[1]), a[2]) < 0:
        a[2] = [-c for c in a[2]]
    world = [[a[j][i] for j in range(3)] for i in range(3)]
    return K.r5(K.quaternion(K.mul(K.transpose(frame[1]), world)))


def carried(solid, old, new, bone):
    """The same world solid bound to another bone."""
    m = K.mul(K.transpose(new[1]), old[1])
    shift = K.apply(K.transpose(new[1]), sub(old[0], new[0]))
    move = lambda p: K.r5([K.apply(m, p)[i] + shift[i] for i in range(3)])
    q = K.quaternion(m)
    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = move(solid["p0"]), move(solid["p1"])
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = move(section["at"])
            if "rotation" in section:
                section["rotation"] = K.r5(K.product(q, section["rotation"]))
            elif abs(section["rx"] - section["ry"]) > 1e-9:
                raise SystemExit(f"loft {solid['id']} has an oval section without rotation")
    elif solid["kind"] == "ellipsoid":
        transform = solid["transform"]
        transform["translation"] = move(transform["translation"])
        transform["rotation"] = K.r5(K.product(q, transform.get("rotation", [0, 0, 0, 1])))
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} is not carried")
    solid["bone"] = solid["member"] = bone
    return solid


def bone_member(identifier, bone, **geometry):
    return {"id": identifier, **{"kind": geometry.pop("kind")}, "bone": bone, "material": "bone", "role": "body", "tone": 0.62, **geometry, "member": bone, "part": "member"}


def rig(asset):
    bones = asset["rig"]["bones"]
    by_id = {bone["id"]: bone for bone in bones}
    if "clavicle-l" in by_id:
        raise SystemExit("already converted")
    old = K.frames(copy.deepcopy(bones))
    titles = {
        "clavicle": ({"en": "Left clavicle with scapula", "de": "Linkes Schlüsselbein mit Schulterblatt"}, {"en": "Right clavicle with scapula", "de": "Rechtes Schlüsselbein mit Schulterblatt"}),
        "ilium": ({"en": "Left hip bone", "de": "Linkes Hüftbein"}, {"en": "Right hip bone", "de": "Rechtes Hüftbein"}),
    }
    for letter, sign, index in (("l", 1, 0), ("r", -1, 1)):
        for name, parent, joint, tip, limb, limits in (
            ("clavicle", "torso", STERNOCLAVICULAR, ACROMIOCLAVICULAR, "arm", {"type": "revolute", "minDeg": -8, "maxDeg": 30, "twistMinDeg": -30 if sign > 0 else -5, "twistMaxDeg": 5 if sign > 0 else 30}),
            ("ilium", "pelvis", SACROILIAC, HIP, "thigh", {"type": "revolute", "minDeg": -3, "maxDeg": 3}),
        ):
            origin, end = side(joint, sign), side(tip, sign)
            u = sub(end, origin)
            placement = K.placed(old[parent], origin, u, (-1.0 * sign, 0.0, 0.0))
            bone = {"id": f"{name}-{letter}", "parent": parent, "title": titles[name][index], **placement, "lengthM": round(math.sqrt(dot(u, u)), 5), "joint": limits}
            child = by_id[f"{limb}-{letter}"]
            bones.insert(bones.index(child), bone)
            now = K.frames(bones)
            hung = K.fields(now[bone["id"]], old[child["id"]][0], old[child["id"]][1])
            rebuilt = {"id": child["id"], "parent": bone["id"], "title": child["title"], **{key: value for key, value in hung.items() if key in ("x", "y", "z", "angleDeg") or abs(value) > 1e-9}}
            rebuilt.update({key: value for key, value in child.items() if key not in ("id", "parent", "title", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg")})
            child.clear()
            child.update(rebuilt)
    new = K.frames(bones)
    for bone_id, frame in old.items():
        drift = max(abs(new[bone_id][0][i] - frame[0][i]) for i in range(3))
        turn = max(abs(new[bone_id][1][i][j] - frame[1][i][j]) for i in range(3) for j in range(3))
        if drift > 3e-5 or turn > 3e-5:
            raise SystemExit(f"{bone_id} moved by {drift} m / {turn}")
    return old, new


def shoulder(scale, frame, letter, sign):
    bone = f"clavicle-{letter}"
    sc, ac = side((STERNOCLAVICULAR[0], STERNOCLAVICULAR[1], STERNAL_END[scale]), sign), side(ACROMIOCLAVICULAR, sign)
    axis = sub(ac, sc)
    flat = unit([axis[0], 0, axis[2]])
    forward = [flat[2] * sign, 0, -flat[0] * sign]
    out = []
    if scale == "50":
        sections = [{"at": local(frame, [sc[i] + t * axis[i] + bow * forward[i] for i in range(3)]), "rx": rx, "ry": ry, "exponent": 2, "rotation": LIMB} for t, bow, rx, ry in CLAVICLE_STATIONS]
        out.append(bone_member(bone, bone, kind="loft", sections=sections, capStart=True, capEnd=True))
    elif scale == "100":
        out.append(bone_member(bone, bone, kind="capsule", p0=local(frame, sc), p1=local(frame, ac), r0=0.0075, r1=0.0075))
    else:
        out.append(bone_member(bone, bone, kind="capsule", p0=local(frame, sc), p1=local(frame, ac), r0=0.0065, r1=0.0065, slender=True))
    long_axis = side(unit(SCAPULA_LONG), sign)
    normal = side(SCAPULA_NORMAL, sign)
    normal = unit([normal[i] - dot(normal, long_axis) * long_axis[i] for i in range(3)])
    blade = bone_member(f"scapula-{letter}", bone, kind="ellipsoid", radii=[SCAPULA_RADII[0], SCAPULA_RADII[1], 0.004 if scale == "50" else SHEET], transform={"translation": local(frame, side(SCAPULA_CENTRE, sign)), "rotation": turned(frame, [long_axis, cross(normal, long_axis), normal])})
    if scale != "50":
        blade["sheet"] = True
    out.append(blade)
    if scale in ("50", "100"):
        out.append(bone_member(f"glenoid-{letter}", bone, kind="capsule", p0=local(frame, side(GLENOID[0], sign)), p1=local(frame, side(GLENOID[1], sign)), r0=0.009, r1=0.0105))
    if scale == "50":
        out.append(bone_member(f"scapular-spine-{letter}", bone, kind="capsule", p0=local(frame, side(SPINE_ROOT, sign)), p1=local(frame, side(ACROMION_ANGLE, sign)), r0=0.0045, r1=0.0055))
        out.append(bone_member(f"acromion-{letter}", bone, kind="capsule", p0=local(frame, side(ACROMION_ANGLE, sign)), p1=local(frame, side(ACROMION_TIP, sign)), r0=0.006, r1=0.006))
        out.append(bone_member(f"coracoid-{letter}", bone, kind="capsule", p0=local(frame, side(CORACOID[0], sign)), p1=local(frame, side(CORACOID[1], sign)), r0=0.0055, r1=0.0045))
    elif scale == "100":
        out.append(bone_member(f"acromion-{letter}", bone, kind="capsule", p0=local(frame, side(SPINE_ROOT, sign)), p1=local(frame, side(ACROMION_TIP, sign)), r0=0.0075, r1=0.0075))
    return out


def hip(scale, frames_old, frames_new, letter, sign, at100, at50):
    bone = f"ilium-{letter}"
    frame = frames_new[bone]
    out = []
    if scale == "100":
        centre = side((0.0, 0.8494, 0.0786), sign)
        out.append(bone_member(f"acetabulum-{letter}", bone, kind="ellipsoid", radii=[0.027, 0.027, 0.015], transform={"translation": local(frame, centre), "rotation": turned(frame, [[1, 0, 0], [0, 1, 0], [0, 0, 1]])}))
        out.append(bone_member(f"hip-bone-{letter}", bone, kind="capsule", p0=local(frame, side(SACROILIAC, sign)), p1=local(frame, side(HIP, sign)), r0=0.0125, r1=0.0125))
        out.extend(rami(scale, frames_old, frame, letter, at50))
        return out
    wing = carried(copy.deepcopy(at100[f"ilium-{letter}"]), frames_old["pelvis"], frame, bone)
    wing["radii"] = [wing["radii"][0], wing["radii"][1], 0.0029]
    wing["sheet"] = True
    out.append(wing)
    radius = 0.02 if scale == "200" else 0.0375
    out.append(bone_member(f"hip-bone-{letter}", bone, kind="capsule", p0=local(frame, side(SACROILIAC, sign)), p1=local(frame, side(HIP, sign)), r0=radius, r1=radius))
    if scale == "200":
        out.extend(rami(scale, frames_old, frame, letter, at50))
    return out


def rami(scale, frames_old, frame, letter, at50):
    """Pubic and ischial rami of the 1:50 skeleton, from the acetabulum to the symphysis and to the ischial tuberosity."""
    out = []
    for name in ("pubis", "ischium"):
        ramus = copy.deepcopy(at50[f"{name}-{letter}"])
        if name == "pubis":
            ramus["p1"][2] = z_signed(letter, 0.009)
        carried(ramus, frames_old["pelvis"], frame, f"ilium-{letter}")
        if scale == "200":
            ramus["r0"] = ramus["r1"] = 0.015
        out.append(ramus)
    return out


def z_signed(letter, value):
    return value if letter == "l" else -value


def limbs(scale, letter):
    """Long bones of one side at 1:500: one loft per bone from joint to joint, true shaft sections."""
    s = lambda v: z_signed(letter, v)
    round_loft = lambda identifier, bone, rows: {**bone_member(identifier, bone, kind="loft", sections=[{"at": [x, y, s(z)], "rx": r, "ry": r} for x, y, z, r in rows], capStart=True, capEnd=True), "slender": True}
    femur = round_loft(f"femur-{letter}", f"thigh-{letter}", [(-0.004, 0.0, 0.0, 0.019), (0.004, -0.001, 0.008, 0.0215), (0.016, -0.003, 0.03, 0.015), (0.034, -0.006, 0.054, 0.0175), (0.07, -0.002, 0.052, 0.014), (0.115, 0.006, 0.044, 0.0128), (0.2, 0.011, 0.029, 0.0122), (0.287, 0.008, 0.014, 0.0128), (0.355, 0.001, 0.004, 0.0165), (0.378, -0.004, 0.0, 0.027), (0.39, -0.004, 0.0, 0.024)])
    tibia = round_loft(f"tibia-{letter}", f"shank-{letter}", [(0.003, -0.002, 0.0, 0.024), (0.016, -0.001, 0.0, 0.027), (0.04, 0.004, 0.003, 0.0165), (0.118, 0.009, 0.003, 0.0122), (0.246, 0.007, 0.003, 0.011), (0.36, 0.003, 0.002, 0.0122), (0.384, 0.001, 0.0, 0.0165), (0.392, 0.001, 0.0, 0.015)])
    humerus = round_loft(f"humerus-{letter}", f"arm-{letter}", [(-0.012, 0.0, -0.026, 0.013), (0.0, 0.0, -0.021, 0.02), (0.012, 0.0, -0.016, 0.018), (0.034, 0.001, -0.012, 0.0115), (0.096, 0.003, -0.007, 0.0098), (0.192, 0.002, -0.002, 0.0094), (0.262, 0.0, 0.0, 0.0105), (0.284, 0.0, 0.0, 0.0145), (0.296, 0.0, 0.0, 0.013)])
    return {f"femur-{letter}": femur, f"tibia-{letter}": tibia, f"humerus-{letter}": humerus}


def structure(scale, solids, frames_old, frames_new, at100, at50):
    drop = {"clavicle-l", "clavicle-r", "scapula-l", "scapula-r", "shoulder-girdle", "pelvic-girdle"}
    merged = {}
    for letter in "lr":
        if scale in ("100", "200"):
            drop |= {f"ischiopubic-{letter}"}
        if scale in ("200", "500"):
            drop |= {f"patella-{letter}", f"malleolus-{letter}", f"member-spoke-rear-{letter}", f"member-spoke-front-{letter}"}
        if scale == "500":
            drop |= {f"femur-neck-{letter}", f"femur-trochanter-{letter}", f"femur-condyles-{letter}", f"tibia-plateau-{letter}", f"fibula-{letter}", f"humerus-head-{letter}", f"humerus-condyles-{letter}"}
            merged.update(limbs(scale, letter))
    out = []
    for solid in solids:
        identifier = solid["id"]
        if identifier in drop:
            continue
        solid = copy.deepcopy(merged.get(identifier, solid))
        letter = identifier[-1]
        s = lambda v: z_signed(letter, v)
        if solid.get("bone") == "pelvis" and identifier[:-2] in ("ilium", "iliac-crest", "acetabulum", "pubis", "ischium", "ischiopubic"):
            carried(solid, frames_old["pelvis"], frames_new[f"ilium-{letter}"], f"ilium-{letter}")
        if identifier == "cervical-spine":
            last = solid["sections"][-1]
            solid["sections"].append({**copy.deepcopy(last), "at": [0.118, 0.008, 0]})
        if identifier[:-2] == "mandible-ramus":
            solid["p0"] = [0.058, -0.004, s(0.044)]
        if identifier[:-2] == "pubis":
            solid["p1"][2] = s(0.009)
        if identifier[:-2] == "patella":
            solid["transform"]["translation"] = [0.379, 0.03, 0]
        if identifier[:-2] == "malleolus":
            solid["radii"] = [0.013, 0.011, 0.0095]
            solid["transform"]["translation"] = [0.384, 0, s(-0.022)]
        if identifier[:-2] == "fibula":
            solid["p0"], solid["p1"], solid["r0"], solid["r1"] = [0.03, -0.014, s(0.027)], [0.392, -0.008, s(0.0215)], 0.0085, 0.0075
            if scale != "50":
                solid["slender"] = True
        if identifier[:-2] in ("ulna", "radius") and scale != "50" or identifier[:-2] == "humerus" and scale == "200":
            solid["slender"] = True
        if scale == "50" and identifier[:-2] == "joint-arm":
            solid["radii"] = [0.0255, 0.0255, 0.0255]
        if scale == "200":
            if identifier[:-2] == "hand-bones":
                solid["p1"] = [0.05, -0.012, solid["p1"][2]]
            if identifier[:-2] == "joint-forearm":
                solid["radii"] = [0.015, 0.015, 0.015]
            if identifier[:-2] == "femur-trochanter":
                solid["radii"] = [0.021, 0.0165, 0.015]
            if identifier[:-2] == "tibia-plateau":
                solid["radii"] = [0.015, 0.024, 0.035]
            if identifier[:-2] == "humerus-condyles":
                solid["radii"] = [0.015, 0.015, 0.028]
        if scale == "500":
            if identifier[:-2] in ("joint-thigh", "joint-shank"):
                solid["radii"] = [0.0375, 0.0375, 0.0375]
            if identifier[:-2] == "joint-arm":
                solid["radii"] = [0.025, 0.025, 0.025]
        out.append(solid)
        if identifier == "sternum":
            for letter, sign in (("l", 1), ("r", -1)):
                out.extend(shoulder(scale, frames_new[f"clavicle-{letter}"], letter, sign))
        if identifier == "sacrum" and scale != "50" or identifier == "mass-pelvis" and scale == "500":
            for letter, sign in (("l", 1), ("r", -1)):
                out.extend(hip(scale, frames_old, frames_new, letter, sign, at100, at50))
        if identifier == "mass-pelvis" and scale == "500":
            out.append(bone_member("sacrum", "pelvis", kind="loft", sections=[{"at": [0.128, 0.02, 0], "rx": 0.0375, "ry": 0.0375}, {"at": [0.085, 0.04, 0], "rx": 0.0375, "ry": 0.0375}, {"at": [0.04, 0.05, 0], "rx": 0.0375, "ry": 0.0375}], capStart=True, capEnd=True))
        if identifier[:-2] == "member-rear-leg" and scale == "200":
            out.append({"id": f"member-axle-rear-{letter}", "of": f"axle-rear-{letter}", "member": "frame", "part": "member"})
    return out


def body(scale, solids, at50, at100):
    out = []
    for solid in solids:
        identifier = solid["id"]
        letter = identifier[-1]
        if scale in ("200", "500") and identifier[:-2] in ("spoke-rear", "spoke-front"):
            continue
        solid = copy.deepcopy(solid)
        if identifier[:-2] in ("spoke-rear", "spoke-front") and scale == "100":
            solid["size"] = [0.046, 0.015, 0.022]
        if identifier[:-2] in ("rim-rear", "rim-front") and scale in ("200", "500"):
            grow = 0.003 if scale == "500" else 0.0
            near = min(abs(p[0]) for p in solid["profile"])
            solid["profile"] = [[round(p[0] + math.copysign(grow, p[0]) * (-1 if abs(abs(p[0]) - near) < 1e-9 else 1), 5), 0 if p[1] < 0.07 else p[1]] for p in solid["profile"][:4]]
        if identifier[:-2] == "rear-leg" and scale in ("100", "200"):
            solid["sections"][-1]["at"] = [0, 0.1, solid["sections"][-1]["at"][2]]
        if identifier == "seat" and scale in ("50", "100"):
            solid["size"] = [0.25, 0.024, 0.452]
        if identifier == "basket" and scale == "500":
            solid["profile"] = [[p[0], 0.425 if abs(p[1] - 0.44) < 1e-9 else p[1]] for p in solid["profile"]]
        out.append(solid)
        if identifier == "coat":
            for side_letter, sign in (("l", 1), ("r", -1)):
                c, r = SHOULDER_CAP["centre"], SHOULDER_CAP["radii"]
                out.append({"id": f"coat-shoulder-{side_letter}", "kind": "ellipsoid", "bone": "torso", "material": solid["material"], "role": "body", "tone": solid["tone"], "radii": [max(r[0], 0.0375) if scale == "500" else r[0], r[1], r[2]], "transform": {"translation": [c[0], c[1], sign * c[2]]}})
        if scale != "50" and identifier[:-2] == "shank":
            sock = copy.deepcopy(at50[f"sock-{letter}"])
            if scale == "500":
                for section, x in zip(sock["sections"], (0.386, 0.345, 0.3)):
                    section["at"][0], section["rx"], section["ry"] = x, max(section["rx"], 0.0375), max(section["ry"], 0.0375)
            out.append(sock)
        if scale in ("200", "500") and identifier[:-2] == "forearm":
            wrist = copy.deepcopy(at50[f"wrist-{letter}"])
            for section in wrist["sections"]:
                section["rx"], section["ry"] = max(section["rx"], 0.0245), max(section["ry"], 0.0265)
            out.append(wrist)
        if scale == "200" and identifier[:-2] == "rear-leg":
            axle = copy.deepcopy(at100[f"axle-rear-{letter}"])
            z = z_signed(letter, 1.0)
            axle["p0"], axle["p1"], axle["r0"], axle["r1"] = [0, 0.1, z * 0.235], [0, 0.1, z * 0.283], 0.015, 0.015
            out.append(axle)
    return out


def thermal(scale, entries, thermal50, thermal100, body_ids):
    out = [copy.deepcopy(entry) for entry in entries if entry.get("of") is None or entry["of"] in body_ids]
    have = {entry.get("of") for entry in out}
    coat = next(entry for entry in out if entry.get("of") == "coat")
    for letter in "lr":
        out.insert(out.index(coat) + 1, {**copy.deepcopy(coat), "id": f"th-coat-shoulder-{letter}", "of": f"coat-shoulder-{letter}"})
        have.add(f"coat-shoulder-{letter}")
    for source in (thermal50, thermal100):
        for entry in source:
            if entry.get("of") in body_ids and entry["of"] not in have:
                added = copy.deepcopy(entry)
                if scale != "50" and "layer" not in coat:
                    added.pop("layer", None)
                out.append(added)
                have.add(entry["of"])
    return out


def build(asset, volumes):
    frames_old, frames_new = rig(asset)
    original = copy.deepcopy(volumes)
    index = lambda scale, name: {solid["id"]: solid for solid in original[scale][name]}
    at100 = index("100", "structure")
    for scale in SCALES:
        volumes[scale]["structure"] = structure(scale, original[scale]["structure"], frames_old, frames_new, at100, index("50", "structure"))
        volumes[scale]["body"] = body(scale, original[scale]["body"], index("50", "body"), index("100", "body"))
        body_ids = {solid["id"] for solid in volumes[scale]["body"]}
        volumes[scale]["thermal"] = thermal(scale, original[scale]["thermal"], original["50"]["thermal"], original["100"]["thermal"], body_ids)
        for entry in volumes[scale]["structure"]:
            if "of" in entry and entry["of"] not in body_ids:
                raise SystemExit(f"{scale}: {entry['id']} points at a missing body solid")
        ids = [solid["id"] for name in ("body", "thermal", "structure") for solid in volumes[scale][name]]
        doubles = {identifier for identifier in ids if ids.count(identifier) > 1}
        if doubles:
            raise SystemExit(f"{scale}: duplicate ids {sorted(doubles)}")


def main():
    text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"]["rig"])
    source_text = open(SOURCE, encoding="utf8").read()
    source = json.loads(source_text)
    build(document["asset"], source["volumes"])
    document["asset"]["volumes"] = source["volumes"]
    if "--apply" not in sys.argv:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        for bone in document["asset"]["rig"]["bones"]:
            if bone["id"][:-2] in ("clavicle", "ilium", "arm", "thigh"):
                print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass")}))
        print({scale: {name: len(source["volumes"][scale][name]) for name in ("body", "thermal", "structure")} for scale in SCALES})
        print(CANDIDATE)
        return
    spliced = K.splice(text, "rig", before, document["asset"]["rig"])
    if open(CATALOG, encoding="utf8").read() != text or open(SOURCE, encoding="utf8").read() != source_text:
        raise SystemExit("catalog or source changed meanwhile: run again")
    open(CATALOG, "w", encoding="utf8").write(spliced)
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("rig written to", CATALOG, "and volumes to", SOURCE)


if __name__ == "__main__":
    main()
