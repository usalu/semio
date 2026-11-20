"""🧸 Root fixes of `humans/parent-carrying-toddler` (round 3): rig, anchors, poses, texts, dimensions and, when present, the redrawn 2D shapes, the representation decisions and the volumes source, written as a candidate asset document; `--commit` writes the catalog file.

The toddler (24 months, 0.865 m) sits facing ahead on the parent's near (right) forearm in front of the near hip, its back against the parent's chest: every bone stays in the sagittal plane, so the planar rig carries the carry exactly.

Usage: python3 r3-parent-carrying-toddler-roots.py [--with-volumes] [--commit]
Reads the untouched catalog copy `🗑️generated/r3-parent-carrying-toddler/original.json`, writes `🗑️generated/r3-parent-carrying-toddler/🔣️.json`.
Optional inputs: `🗑️generated/r3-parent-carrying-toddler/shapes.json` ({scale: {view: [shapes]}}), `r3-parent-carrying-toddler-texts.json`, `r3-parent-carrying-toddler-dimensions.json`, `volumes/humans/parent-carrying-toddler.json`.
"""
import glob
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(T, "🗑️generated", "r3-parent-carrying-toddler")
CATALOG = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/parent-carrying-toddler/🔣️.json")[0]
MANIFEST = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(CATALOG))), "🔣️.json")
OWNED = ["summary", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "poses", "clips", "thermal", "structure"]
BONE_MATERIAL = {"id": "bone", "title": {"en": "Bone", "de": "Knochen"}, "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 1900, "source": "authored-estimate"}

OLD_THIGH, OLD_SHANK, OLD_ANKLE = 0.4153, 0.4056, 0.0772
THIGH, SHANK, ANKLE, BALL = 0.40, 0.418, 0.088, 0.13
SCALE_LEG = (THIGH + SHANK) / (OLD_THIGH + OLD_SHANK)


def r(value, digits=4):
    return round(value, digits)


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth=0, compact=False):
    """🖨️ The file's own style: two-space indent, every list expanded, except inside `volumes` where scalar lists stay inline."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1, compact or (depth == 1 and key == "volumes")) for key, item in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if compact and inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1, compact) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def text(en, de):
    return {"en": en, "de": de}


def mass(kg, x, y=0.0, ref="de-leva-1996"):
    return {"kg": kg, "comX": r(x), "comY": r(y), "distribution": "point", "source": "literature", "reference": ref}


def bone(id_, parent, title, x, y, angle, length, lo, hi, m=None, z=None):
    b = {"id": id_, "parent": parent, "title": title, "x": r(x), "y": r(y), "angleDeg": r(angle, 3), "lengthM": r(length), "joint": {"type": "revolute", "minDeg": lo, "maxDeg": hi}}
    if m is not None:
        b["mass"] = m
    if z is not None:
        b["z"] = r(z)
    return b


# ---------------------------------------------------------------- parent rig (rest: upright, arms hanging, sole level)

PELVIS = 0.105
SACRUM_BACK = 0.04
TORSO = 0.4735
HEAD = 0.2388
HEAD_REST = -6.0
SHOULDER_U, SHOULDER_V, SHOULDER_Z = 0.402, -0.04, 0.175
UPPER, FORE, HAND = 0.316, 0.248, 0.184
CARRY_ELBOW_Z = -0.03
CARRY_HAND_Z = 0.05
FREE_ELBOW_Z = 0.04
FREE_HAND_Z = 0.03
HIP_Z = 0.0909

DEFAULT = {"torso": 3.0, "head": -3.0, "upper-arm-l": -3.0, "forearm-l": 12.0, "hand-l": 0.0, "upper-arm-r": -5.0, "forearm-r": 95.0, "hand-r": 0.0, "thigh-l": 2.0, "shank-l": -5.0, "foot-l": 3.0, "thigh-r": 0.0, "shank-r": -3.0, "foot-r": 3.0}

# ---------------------------------------------------------------- toddler, stated in the default pose of the parent (world: x ahead, y up, z left)

TODDLER = {
    "hip": (0.18, 1.20, -0.17),
    "torso": (95.0, 0.28),
    "head": (90.0, 0.205),
    "shoulder": (0.25, -0.005, 0.09),
    "elbow-z": 0.03,
    "upper-arm": (-100.0, 0.135),
    "forearm": (-28.0, 0.11),
    "hip-joint-z": 0.05,
    "thigh": (-8.0, 0.18),
    "shank": (-100.0, 0.17),
    "foot": (-12.0, 0.115),
}


def frames(rig, pose, grounded=True):
    """🦴 Planar forward kinematics of the rig in a pose (angle offsets added to the rest angles): world origin, angle of u and lateral z per bone; `grounded` puts the left sole on y = 0 as the product's automatic ground does in the default pose."""
    out = {}
    order = list(rig)
    while len(out) < len(order):
        for bid in order:
            if bid in out:
                continue
            b = rig[bid]
            parent = b["parent"]
            if parent is None:
                out[bid] = ((0.0, 0.0), b["angleDeg"] + pose.get(bid, 0.0), b.get("z", 0.0))
                continue
            if parent not in out:
                continue
            (px, py), pa, pz = out[parent]
            a = math.radians(pa)
            ux, uy, vx, vy = math.cos(a), math.sin(a), -math.sin(a), math.cos(a)
            o = (px + b["x"] * ux + b["y"] * vx, py + b["x"] * uy + b["y"] * vy)
            out[bid] = (o, pa + b["angleDeg"] + pose.get(bid, 0.0), pz + b.get("z", 0.0))
    if grounded and "foot-l" in out:
        (fx, fy), fa, _ = out["foot-l"]
        a = math.radians(fa)
        lift = -(fy + BALL * math.sin(a) - ANKLE * math.cos(a))
        out = {k: ((o[0], o[1] + lift), a_, z) for k, (o, a_, z) in out.items()}
    return out


def local(frame, p):
    (ox, oy), a, _ = frame
    a = math.radians(a)
    dx, dy = p[0] - ox, p[1] - oy
    return dx * math.cos(a) + dy * math.sin(a), -dx * math.sin(a) + dy * math.cos(a)


def parent_rig():
    t = lambda en, de: text(en, de)
    b = [
        bone("pelvis", None, t("Pelvis", "Becken"), 0, 0, 90, PELVIS, -60, 30, mass(8.23, PELVIS / 2)),
        bone("torso", "pelvis", t("Trunk (L5/S1 to C7)", "Rumpf (L5/S1 bis C7)"), PELVIS, SACRUM_BACK, 0, TORSO, -95, 30, mass(19.866, 0.216)),
        bone("head", "torso", t("Head and neck (C7 to vertex)", "Kopf und Hals (C7 bis Scheitel)"), TORSO, 0, HEAD_REST, HEAD, -50, 45, mass(4.409, 0.119)),
    ]
    for side, z in (("l", SHOULDER_Z), ("r", -SHOULDER_Z)):
        name = {"l": ("Left", "Linker", "Linke"), "r": ("Right", "Rechter", "Rechte")}[side]
        b.append(bone(f"upper-arm-{side}", "torso", t(f"{name[0]} upper arm", f"{name[1]} Oberarm"), SHOULDER_U, SHOULDER_V, -180, UPPER, -60, 180, mass(1.683, 0.182), z))
        b.append(bone(f"forearm-{side}", f"upper-arm-{side}", t(f"{name[0]} forearm", f"{name[1]} Unterarm"), UPPER, 0, 0, FORE, 0, 150, mass(0.911, 0.1134), CARRY_ELBOW_Z if side == "r" else FREE_ELBOW_Z))
        b.append(bone(f"hand-{side}", f"forearm-{side}", t(f"{name[0]} hand", f"{name[2]} Hand"), FORE, 0, 0, HAND, -65, 75, mass(0.37, 0.069), CARRY_HAND_Z if side == "r" else FREE_HAND_Z))
    for side, z in (("l", HIP_Z), ("r", -HIP_Z)):
        name = {"l": ("Left", "Linker", "Linke"), "r": ("Right", "Rechter", "Rechte")}[side]
        b.append(bone(f"thigh-{side}", "pelvis", t(f"{name[0]} thigh", f"{name[1]} Oberschenkel"), 0, 0, -180, THIGH, -60, 125, mass(9.755, 0.164), z))
        b.append(bone(f"shank-{side}", f"thigh-{side}", t(f"{name[0]} shank", f"{name[1]} Unterschenkel"), THIGH, 0, 0, SHANK, -150, 0, mass(3.175, 0.1864)))
        b.append(bone(f"foot-{side}", f"shank-{side}", t(f"{name[0]} foot", f"{name[1]} Fuß"), SHANK, 0, 90, 0.21, -65, 60, mass(0.851, 0.06, -0.05)))
        b.append(bone(f"toe-{side}", f"foot-{side}", t(f"{name[0]} forefoot", f"{name[1]} Vorfuß"), BALL, -ANKLE, 0, 0.08, 0, 70))
    return b


def toddler_rig(parent):
    """🧒 The toddler's bones: rest = their place in the default pose, stated in world terms and carried into each parent frame."""
    t = lambda en, de: text(en, de)
    rig = {x["id"]: x for x in parent}
    pose = dict(DEFAULT)
    out = []

    def add(b):
        out.append(b)
        rig[b["id"]] = b

    f = frames(rig, pose)
    hip = TODDLER["hip"]
    x, y = local(f["forearm-r"], hip[:2])
    tz = hip[2] - f["forearm-r"][2]
    ta = TODDLER["torso"][0] - f["forearm-r"][1]
    add(bone("toddler-torso", "forearm-r", t("Toddler trunk (hip joints to C7)", "Rumpf des Kleinkinds (Hüftgelenke bis C7)"), x, y, ta, TODDLER["torso"][1], -30, 30, mass(5.49, 0.14, 0, "who-2006"), tz))
    add(bone("toddler-head", "toddler-torso", t("Toddler head and neck", "Kopf und Hals des Kleinkinds"), TODDLER["torso"][1], 0, TODDLER["head"][0] - TODDLER["torso"][0], TODDLER["head"][1], -40, 40, mass(1.83, 0.105, 0, "who-2006")))
    su, sv, sz = TODDLER["shoulder"]
    for side, z in (("l", sz), ("r", -sz)):
        n = {"l": ("left", "linker", "linke"), "r": ("right", "rechter", "rechte")}[side]
        add(bone(f"toddler-arm-{side}", "toddler-torso", t(f"Toddler {n[0]} upper arm", f"{n[1].capitalize()} Oberarm des Kleinkinds"), su, sv, TODDLER["upper-arm"][0] - TODDLER["torso"][0], TODDLER["upper-arm"][1], -90, 150, mass(0.32, 0.06, 0, "who-2006"), z))
        add(bone(f"toddler-forearm-{side}", f"toddler-arm-{side}", t(f"Toddler {n[0]} forearm and mitten", f"{n[1].capitalize()} Unterarm mit Fäustling des Kleinkinds"), TODDLER["upper-arm"][1], 0, TODDLER["forearm"][0] - TODDLER["upper-arm"][0], TODDLER["forearm"][1], 0, 140, mass(0.29, 0.075, 0, "who-2006"), TODDLER["elbow-z"] if side == "l" else -TODDLER["elbow-z"]))
    hz = TODDLER["hip-joint-z"]
    for side, z in (("l", hz), ("r", -hz)):
        n = {"l": ("left", "linker", "linke", "Linkes"), "r": ("right", "rechter", "rechte", "Rechtes")}[side]
        add(bone(f"toddler-thigh-{side}", "toddler-torso", t(f"Toddler {n[0]} thigh", f"{n[1].capitalize()} Oberschenkel des Kleinkinds"), 0, 0, TODDLER["thigh"][0] - TODDLER["torso"][0], TODDLER["thigh"][1], -150, 30, mass(0.976, 0.08, 0, "who-2006"), z))
        add(bone(f"toddler-shank-{side}", f"toddler-thigh-{side}", t(f"Toddler {n[0]} shank", f"{n[1].capitalize()} Unterschenkel des Kleinkinds"), TODDLER["thigh"][1], 0, TODDLER["shank"][0] - TODDLER["thigh"][0], TODDLER["shank"][1], -140, 10, mass(0.66, 0.075, 0, "who-2006")))
        add(bone(f"toddler-foot-{side}", f"toddler-shank-{side}", t(f"Toddler {n[0]} foot in its rubber boot", f"{n[2].capitalize()} Fuß des Kleinkinds im Gummistiefel"), TODDLER["shank"][1], 0, TODDLER["foot"][0] - TODDLER["shank"][0], TODDLER["foot"][1], -50, 40, mass(0.194, 0.045, -0.02, "who-2006")))
    return out


# ---------------------------------------------------------------- poses

OLD_TODDLER = {"toddler-thigh-n": "toddler-thigh-l", "toddler-thigh-f": "toddler-thigh-r"}
SWING = {"heel-strike-l": 0.0, "walk-1": 0.125, "midstance-l": 0.25, "walk-3": 0.375, "walk-4": 0.5, "walk-5": 0.625, "walk-6": 0.75, "walk-7": 0.875}


def old_leg_world(bones, side):
    """🦵 Hip-to-ankle vector and foot angle of an old pose leg (old lengths, old rest angles, pelvis upright)."""
    dt = math.radians(-90 + bones.get(f"thigh-{side}", {}).get("angleDeg", 0))
    ds = dt + math.radians(bones.get(f"shank-{side}", {}).get("angleDeg", 0))
    df = ds + math.radians(90 + bones.get(f"foot-{side}", {}).get("angleDeg", 0))
    ax = OLD_THIGH * math.cos(dt) + OLD_SHANK * math.cos(ds)
    ay = OLD_THIGH * math.sin(dt) + OLD_SHANK * math.sin(ds)
    sole = (ax + OLD_ANKLE * math.sin(df), ay - OLD_ANKLE * math.cos(df))
    return sole, df


def solve_leg(target_sole, df):
    """🦿 New thigh, shank and foot offsets that put the new sole point (under the new ankle) where the old one was, foot angle kept."""
    ax = target_sole[0] - ANKLE * math.sin(df)
    ay = target_sole[1] + ANKLE * math.cos(df)
    d = min(math.hypot(ax, ay), THIGH + SHANK - 1e-9)
    knee = math.acos(max(-1, min(1, (THIGH * THIGH + SHANK * SHANK - d * d) / (2 * THIGH * SHANK))))
    hip = math.acos(max(-1, min(1, (THIGH * THIGH + d * d - SHANK * SHANK) / (2 * THIGH * d))))
    base = math.atan2(ay, ax)
    dt = base + hip
    ds = dt - (math.pi - knee)
    return r(math.degrees(dt) + 90, 3), r(math.degrees(ds - dt), 3), r(math.degrees(df - ds) - 90, 3)


def pose_bones(old, pose_id):
    new = {}
    if "pelvis" in old:
        new["pelvis"] = {"angleDeg": old["pelvis"]["angleDeg"]}
    walking = pose_id in SWING
    hoist = pose_id == "hoisting"
    torso = 7.0 if hoist else (old.get("torso", {}).get("angleDeg", 0) + 2.0 if walking else DEFAULT["torso"])
    new["torso"] = {"angleDeg": r(torso, 3)}
    new["head"] = {"angleDeg": r(-1.0 - (torso - 3.0) if hoist else DEFAULT["head"] - (torso - DEFAULT["torso"]), 3)}
    if walking:
        phase = SWING[pose_id]
        swing = -14.0 * math.cos(2 * math.pi * phase)
        new["upper-arm-l"] = {"angleDeg": r(swing - (torso - DEFAULT["torso"]) - 2, 3)}
        new["forearm-l"] = {"angleDeg": r(12.0 + 0.5 * max(swing, 0), 3)}
        new["hand-l"] = {"angleDeg": 0}
    else:
        for k in ("upper-arm-l", "forearm-l", "hand-l"):
            new[k] = {"angleDeg": DEFAULT[k]}
    if hoist:
        new["upper-arm-r"] = {"angleDeg": 3.0}
        new["forearm-r"] = {"angleDeg": 110.0}
        new["hand-r"] = {"angleDeg": 5.0}
    else:
        new["upper-arm-r"] = {"angleDeg": r(DEFAULT["upper-arm-r"] - (torso - DEFAULT["torso"]), 3)}
        new["forearm-r"] = {"angleDeg": DEFAULT["forearm-r"]}
        new["hand-r"] = {"angleDeg": DEFAULT["hand-r"]}
    for side in ("l", "r"):
        if pose_id == "standing-carry":
            for k in ("thigh", "shank", "foot"):
                new[f"{k}-{side}"] = {"angleDeg": DEFAULT[f"{k}-{side}"]}
            continue
        sole, df = old_leg_world(old, side)
        sole = (sole[0] * SCALE_LEG, sole[1] * SCALE_LEG)
        a, b, c = solve_leg(sole, df)
        new[f"thigh-{side}"] = {"angleDeg": a}
        new[f"shank-{side}"] = {"angleDeg": b}
        new[f"foot-{side}"] = {"angleDeg": c}
    bob = old.get("toddler-torso", {}).get("angleDeg", 0)
    new["toddler-torso"] = {"angleDeg": r(3.0 - (torso - DEFAULT["torso"]) - (new["upper-arm-r"]["angleDeg"] - DEFAULT["upper-arm-r"]) - (new["forearm-r"]["angleDeg"] - DEFAULT["forearm-r"]) if hoist else bob - (new["forearm-r"]["angleDeg"] - DEFAULT["forearm-r"]) - (new["upper-arm-r"]["angleDeg"] - DEFAULT["upper-arm-r"]) - (torso - DEFAULT["torso"]), 3)}
    new["toddler-head"] = {"angleDeg": 6.0 if hoist else 0}
    for side in ("l", "r"):
        new[f"toddler-arm-{side}"] = {"angleDeg": (14.0 if side == "l" else -6.0) if hoist else 0}
        new[f"toddler-forearm-{side}"] = {"angleDeg": (22.0 if side == "l" else 10.0) if hoist else 0}
    for old_id, new_id in OLD_TODDLER.items():
        new[new_id] = {"angleDeg": r(old.get(old_id, {}).get("angleDeg", 0), 3)}
    if hoist:
        new["toddler-thigh-l"] = {"angleDeg": 12.0}
        new["toddler-thigh-r"] = {"angleDeg": -6.0}
    for side in ("l", "r"):
        new[f"toddler-shank-{side}"] = {"angleDeg": (-12.0 if side == "l" else 8.0) if hoist else 0}
        new[f"toddler-foot-{side}"] = {"angleDeg": 0}
    return new


POSE_TEXT = {
    "standing-carry": (
        text("Standing with the toddler on the near arm", "Stehen mit dem Kleinkind auf dem nahen Arm"),
        text("Upright stance leaning 3 degrees back against the load; the near forearm runs forward under the toddler's seat and thighs and the hand cups its thighs; the toddler sits facing ahead with its back against the parent's chest, its legs hanging over the forearm and its mittens on its thighs; the far arm hangs.", "Aufrechter Stand, 3 Grad gegen die Last zurückgelehnt; der nahe Unterarm läuft unter Gesäß und Oberschenkeln des Kleinkinds nach vorn, die Hand umfasst die Oberschenkel; das Kleinkind sitzt mit Blick nach vorn, den Rücken an der Brust des Elternteils, die Beine hängen über den Unterarm, die Fäustlinge liegen auf den Oberschenkeln; der ferne Arm hängt."),
    ),
    "hoisting": (
        text("Hoisting the toddler up", "Das Kleinkind höher nehmen"),
        text("Knees bent and trunk leaning back while the near arm lifts the toddler on the forearm; the toddler leans back, kicks and raises its arms.", "Knie gebeugt und Rumpf zurückgelehnt, während der nahe Arm das Kleinkind auf dem Unterarm anhebt; das Kleinkind lehnt sich zurück, strampelt und hebt die Arme."),
    ),
}
WALK_TEXT = ("Walking while carrying: gait phase {p} of the left foot with the right foot half a cycle later; the near arm holds the toddler, the far arm swings and the toddler bobs and swings its legs.", "Gehen mit Last: Gangphase {p} des linken Fußes, der rechte Fuß folgt einen halben Zyklus später; der nahe Arm hält das Kleinkind, der ferne Arm schwingt, das Kleinkind wippt und schwingt die Beine.")


def anchors_new(asset, rig):
    f = frames({b["id"]: b for b in rig}, DEFAULT)
    head = lambda p: local(f["head"], p)
    th = lambda p: local(f["toddler-head"], p)
    a = []
    for side in ("l", "r"):
        nm = {"l": ("Left", "Linker", "Linke"), "r": ("Right", "Rechter", "Rechte")}[side]
        a.append({"id": f"contact-{side}", "bone": f"foot-{side}", "x": BALL, "y": -ANKLE, "kind": "contact", "title": text(f"{nm[0]} ball of foot", f"{nm[1]} Fußballen"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.093, "footprintWidthM": 0.084, "frictionCoefficient": 0.8})
        a.append({"id": f"heel-{side}", "bone": f"foot-{side}", "x": -0.04, "y": -ANKLE, "kind": "contact", "title": text(f"{nm[0]} heel", f"{nm[2]} Ferse"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.065, "footprintWidthM": 0.065, "frictionCoefficient": 0.8})
    order = ["contact-l", "heel-l", "contact-r", "heel-r"]
    a.sort(key=lambda x: order.index(x["id"]))
    spec = json.load(open(os.path.join(T, "r3-parent-carrying-toddler-anchors.json"), encoding="utf8"))
    for item in asset["anchors"]:
        if item["id"] in order:
            continue
        n = dict(item)
        if item["id"] in spec:
            s = spec[item["id"]]
            n["bone"] = s["bone"]
            n["x"], n["y"] = (r(v) for v in (local(f[s["bone"]], s["world"][:2]) if "world" in s else s["local"][:2]))
            z = s["world"][2] - f[s["bone"]][2] if "world" in s else s["local"][2]
            n.pop("z", None)
            if abs(z) > 1e-6:
                keys = list(n.keys())
                rebuilt = {}
                for k in keys:
                    rebuilt[k] = n[k]
                    if k == "y":
                        rebuilt["z"] = r(z)
                n = rebuilt
            if "title" in s:
                n["title"] = s["title"]
        a.append(n)
    return a


def apply(asset):
    if all(m["id"] != "bone" for m in asset["materials"]):
        asset["materials"].append(BONE_MATERIAL)
    parent = parent_rig()
    rig = parent + toddler_rig(parent)
    asset["rig"]["bones"] = rig
    asset["anchors"] = anchors_new(asset, rig)
    for pose in asset["poses"]:
        pose["bones"] = pose_bones(pose["bones"], pose["id"])
        if pose["id"] in POSE_TEXT:
            pose["title"], pose["description"] = POSE_TEXT[pose["id"]]
        elif pose["id"] in SWING:
            p = f"{SWING[pose['id']]:.3f}"
            pose["description"] = text(WALK_TEXT[0].format(p=p), WALK_TEXT[1].format(p=p))
    for clip in asset["clips"]:
        if clip["id"] == "walk-carrying":
            clip["advanceM"] = 0.9466
            clip["description"] = text(
                "Walking cycle of 1.0 s (about 0.95 m/s) with the toddler on the near arm: the feet are planted while in contact, the far arm swings, the carrying arm stays still and the toddler bobs and swings its legs.",
                "Gehzyklus von 1,0 s (etwa 0,95 m/s) mit dem Kleinkind auf dem nahen Arm: Die Füße gleiten im Kontakt nicht, der ferne Arm schwingt, der tragende Arm bleibt ruhig und das Kleinkind wippt und schwingt die Beine.")
        if clip["id"] == "shift-toddler":
            clip["description"] = text(
                "Non-looping 3 s: the parent bends the knees, leans back and lifts the toddler higher on the near forearm, then stands again; the toddler fusses briefly, kicks and settles.",
                "Nicht wiederholte Folge von 3 s: Das Elternteil beugt die Knie, lehnt sich zurück und hebt das Kleinkind auf dem nahen Unterarm höher, dann steht es wieder; das Kleinkind quengelt kurz, strampelt und beruhigt sich.")
    asset["summary"] = text(
        "A 1.70 m parent in a wool coat, scarf, beanie, jeans, leather gloves and ankle boots carrying a 24-month toddler (0.865 m, 12.2 kg) in a padded overall, knit hat, mittens and rubber boots on the near forearm: the toddler sits facing ahead with its back against the parent's chest and is a second living body with its own breathing; standing, a walking cycle with planted feet, hoisting and settling; four individually drawn and modelled scales.",
        "Ein 1,70 m großes Elternteil in Wollmantel, Schal, Mütze, Jeans, Lederhandschuhen und Stiefeletten trägt ein 24 Monate altes Kleinkind (0,865 m, 12,2 kg) in gefüttertem Overall, Strickmütze, Fäustlingen und Gummistiefeln auf dem nahen Unterarm: Das Kleinkind sitzt mit Blick nach vorn, den Rücken an der Brust des Elternteils, und ist ein zweiter lebender Körper mit eigener Atmung; Stehen, ein Gehzyklus mit festen Füßen, Hochnehmen und Zurechtsetzen; vier einzeln gezeichnete und modellierte Maßstäbe.")
    prov = asset["provenance"]
    prov["notes"] = text(
        "Round 3 (2026-11-20): rig rebuilt on anthropometric joint heights (Winter 2009) for a 1.70 m parent in 22 mm boot heels and a 24-month toddler (WHO 2006); the toddler is a passenger sitting on the near forearm with its own forearms and feet, every bone in the sagittal plane; 2D drawings of every scale are the views of the handcrafted 3D body in the default pose; the walking gait is the earlier inverse-kinematics gait re-solved for the new leg lengths with the soles kept in place.",
        "Runde 3 (2026-11-20): Rig nach anthropometrischen Gelenkhöhen (Winter 2009) für ein 1,70 m großes Elternteil mit 22 mm Stiefelabsatz und ein 24 Monate altes Kleinkind (WHO 2006) neu aufgebaut; das Kleinkind ist ein Passagier auf dem nahen Unterarm mit eigenen Unterarmen und Füßen, alle Knochen in der Sagittalebene; die 2D-Zeichnungen aller Maßstäbe sind die Ansichten des handgefertigten 3D-Körpers in der Standardpose; der Gang ist der frühere Gang aus inverser Kinematik, für die neuen Beinlängen mit unveränderten Sohlenpunkten neu gelöst.")
    refs = {x["id"] for x in prov["references"]}
    for ref in json.load(open(os.path.join(T, "r3-parent-carrying-toddler-references.json"), encoding="utf8")):
        if ref["id"] not in refs:
            prov["references"].append(ref)
    asset["structure"]["assumptions"] = [text(
        "Inertial model: segment masses as point masses at de Leva centres for the parent and at WHO-scaled centres for the toddler; the toddler's 12.2 kg rest on the parent's near forearm and hand (seat and thighs) and lean on the chest; heel and ball contacts are unilateral supports.",
        "Trägheitsmodell: Segmentmassen als Punktmassen in den Schwerpunkten nach de Leva für das Elternteil und nach WHO skaliert für das Kleinkind; die 12,2 kg des Kleinkinds ruhen auf dem nahen Unterarm und der Hand des Elternteils (Gesäß und Oberschenkel) und lehnen an der Brust; Fersen- und Ballenkontakte sind einseitige Auflager.")]
    dims = os.path.join(T, "r3-parent-carrying-toddler-dimensions.json")
    if os.path.exists(dims):
        asset["dimensions"].update(json.load(open(dims, encoding="utf8")))
    shapes = os.path.join(G, "shapes.json")
    if not os.path.exists(shapes):
        known = {b["id"] for b in rig}
        for rep in asset["representations"].values():
            for view, items in rep.get("geometry", {}).items():
                rep["geometry"][view] = [x for x in items if x.get("bone") in known]
    if os.path.exists(shapes):
        for scale, views in json.load(open(shapes, encoding="utf8")).items():
            for view, items in views.items():
                asset["representations"][scale]["geometry"][view] = items
    texts = os.path.join(T, "r3-parent-carrying-toddler-texts.json")
    if os.path.exists(texts):
        for scale, decisions in json.load(open(texts, encoding="utf8")).get("representations", {}).items():
            asset["representations"][scale]["decisions"] = decisions
    source = os.path.join(T, "volumes", "humans", "parent-carrying-toddler.json")
    if "--with-volumes" in sys.argv and os.path.exists(source):
        asset["volumes"] = json.load(open(source, encoding="utf8"))["volumes"]


def main():
    raw = open(os.path.join(G, "original.json"), encoding="utf8").read()
    doc = json.loads(raw)
    if dump(doc) + "\n" != raw:
        print("[DEBUG] style does not round-trip the original", len(dump(doc)) + 1, len(raw))
    before = json.loads(raw)
    apply(doc["asset"])
    out = os.path.join(G, "🔣️.json")
    open(out, "w", encoding="utf8").write(dump(doc) + "\n")
    print(out)
    if "--commit" in sys.argv:
        live = json.loads(open(CATALOG, encoding="utf8").read())
        last = os.path.join(G, "committed.json")
        mine = json.load(open(last, encoding="utf8")) if os.path.exists(last) else before
        for member in OWNED:
            if live["asset"][member] not in (before["asset"][member], mine["asset"][member], doc["asset"][member]):
                raise SystemExit(f"catalog member {member} changed under us; merge by hand")
        for member in OWNED:
            live["asset"][member] = doc["asset"][member]
        body = dump(live) + "\n"
        open(CATALOG, "w", encoding="utf8").write(body)
        open(last, "w", encoding="utf8").write(json.dumps({"asset": {member: doc["asset"][member] for member in OWNED}}, ensure_ascii=False))
        print("committed", CATALOG, len(body))
        manifest = open(MANIFEST, encoding="utf8").read()
        at = manifest.find('"id": "parent-carrying-toddler",')
        if at >= 0:
            end = manifest.index("}", at)
            row = manifest[at:end]
            for key in ("heightM", "widthM", "depthM"):
                row = re.sub(r'("' + key + r'": )[0-9.]+', lambda m: m.group(1) + json.dumps(doc["asset"]["dimensions"][key]), row)
            if row != manifest[at:end]:
                open(MANIFEST, "w", encoding="utf8").write(manifest[:at] + row + manifest[end:])
                print("manifest row updated")


main()
