"""🦴 Root fixes of `humans/kneeling-gardener` (round 3), stated as final values (idempotent).

Poses: the forward lean is shared between the hips and the lumbar spine (pelvis 55 % of the trunk inclination, the hip joints flex by the same amount so the thighs stay where they were) and the feet lie plantarflexed by 65 degrees so the toe box rests on the pad. Rig: C7 at 0.86 of the stature (trunk 0.456 m, head-and-neck 0.2325 m up to the vertex at 1.66 m), shoulder joints at 0.818 of the stature and 0.32 m apart, the trowel held in a hammer grip (50 degrees to the hand axis instead of in line with the fingers). Poses: both arms solved again in the sagittal plane, the left palm on the front of the left thigh, the right fist carrying the trowel with a wrist deviation a wrist can do. Anchors: mouth, nostrils and crown on the modelled head, the left palm on the glove surface.

Usage: python3 r3-kneeling-gardener-roots.py [--print] [--candidate | --write]
"""
import copy
import importlib.util
import json
import math
import os
import sys

here = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(here, "r3-kneeling-gardener-%s.py" % name))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


fk = load("fk")
splice = load("splice")

BONES = {
    "torso": {"lengthM": 0.456, "mass.comX": 0.2082},
    "head": {"x": 0.456, "lengthM": 0.2325, "mass.comX": 0.1163},
    "upper-arm-l": {"x": 0.387, "z": 0.16},
    "upper-arm-r": {"x": 0.387, "z": -0.16},
    "trowel": {"angleDeg": 50, "lengthM": 0.26, "mass.comX": 0.11},
}
ANCHORS = {"mouth": (0.0655, -0.0905), "nostrils": (0.088, -0.105), "crown": (0.266, 0), "palm-l": (0.09, -0.019), "trowel-tip": (0.26, 0), "instep-l": (0.17, -0.014), "instep-r": (0.17, -0.014)}
DIMENSIONS = {"heightM": 1.098}
GARDEN_SPEC = "Typical garden equipment: 0.76 x 0.4 x 0.04 m extra-large EVA kneeling mat (the shins and insteps rest on it), steel hand trowel about 0.3 m long and 0.25 kg with a 0.08 m wide blade, flexible 0.5 m polyethylene tub (trug) 0.21 m high with two moulded handles. Dimensions and masses are authored estimates from catalogue ranges."
PALM = (0.09, -0.019)
FIST = 0.07
TROWEL = 0.26
INCLINATION = {"kneeling-weeding": -58, "kneeling-upright": -8, "reaching-trug": -76, "dig-up": -60, "dig-in": -58, "dig-pull": -54}
THIGH = {"kneeling-weeding": (10, 11), "kneeling-upright": (10, 11), "reaching-trug": (14, 15), "dig-up": (10, 11), "dig-in": (10, 11), "dig-pull": (10, 11)}
PELVIS_SHARE = 0.55
FOOT = -65
LEFT = {
    "kneeling-weeding": (0.24, 0.084, 0),
    "kneeling-upright": (0.13, 0.093, 0),
    "reaching-trug": (0.31, 0.073, 0),
    "dig-up": (0.24, 0.084, 0),
    "dig-in": (0.24, 0.084, 0),
    "dig-pull": (0.24, 0.084, 0),
}
RIGHT = {
    "kneeling-weeding": ((0.7, 0.0), -50, -5),
    "kneeling-upright": ((0.58, 0.6), 8, 0),
    "reaching-trug": ((1.26, 0.5), 12, -40),
    "dig-up": ((0.68, 0.24), -18, 0),
    "dig-in": ((0.73, 0.0), -53, -10),
    "dig-pull": ((0.6, 0.0), -52, 0),
}


def norm(a):
    return (a + 180.0) % 360.0 - 180.0


def two_link(shoulder, wrist, a, b):
    """World angles of upper arm and forearm reaching `wrist`, the elbow flexing forward (forearm turned counter-clockwise from the upper arm)."""
    dx, dy = wrist[0] - shoulder[0], wrist[1] - shoulder[1]
    d = math.hypot(dx, dy)
    if d > a + b - 1e-6:
        raise SystemExit("unreachable: %.4f > %.4f for %s" % (d, a + b, wrist))
    base = math.degrees(math.atan2(dy, dx))
    at_shoulder = math.degrees(math.acos((a * a + d * d - b * b) / (2 * a * d)))
    at_elbow = math.degrees(math.acos((a * a + b * b - d * d) / (2 * a * b)))
    upper = base - at_shoulder
    return upper, upper + (180.0 - at_elbow)


def solve(asset, pose, report):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    lean = INCLINATION[pose["id"]]
    pelvis = round(PELVIS_SHARE * lean)
    pose["bones"]["pelvis"]["angleDeg"] = pelvis
    pose["bones"]["torso"]["angleDeg"] = lean - pelvis
    for side, thigh in zip("lr", THIGH[pose["id"]]):
        pose["bones"]["thigh-" + side]["angleDeg"] = thigh - pelvis
        pose["bones"]["foot-" + side]["angleDeg"] = FOOT
    f = fk.frames(asset, pose)
    a, b = bones["upper-arm-l"]["lengthM"], bones["forearm-l"]["lengthM"]
    torso = f["torso"][2]
    u, v, lift = LEFT[pose["id"]]
    palm = fk.point(f["thigh-l"], u, v)
    hand = f["thigh-l"][2] + lift
    c, s = math.cos(math.radians(hand)), math.sin(math.radians(hand))
    wrist = (palm[0] - PALM[0] * c + PALM[1] * s, palm[1] - PALM[0] * s - PALM[1] * c)
    upper, fore = two_link(f["upper-arm-l"][:2], wrist, a, b)
    pose["bones"]["upper-arm-l"]["angleDeg"] = round(norm(upper - torso - bones["upper-arm-l"]["angleDeg"]), 3)
    pose["bones"]["forearm-l"]["angleDeg"] = round(norm(fore - upper), 3)
    pose["bones"]["hand-l"]["angleDeg"] = round(norm(hand - fore), 3)
    report.append("%-17s L upper %7.2f elbow %6.2f wrist %6.2f" % (pose["id"], norm(upper), norm(fore - upper), norm(hand - fore)))
    tip, direction, offset = RIGHT[pose["id"]]
    hand = direction - bones["trowel"]["angleDeg"] - offset
    fist = (tip[0] - TROWEL * math.cos(math.radians(direction)), tip[1] - TROWEL * math.sin(math.radians(direction)))
    wrist = (fist[0] - FIST * math.cos(math.radians(hand)), fist[1] - FIST * math.sin(math.radians(hand)))
    upper, fore = two_link(f["upper-arm-r"][:2], wrist, a, b)
    pose["bones"]["upper-arm-r"]["angleDeg"] = round(norm(upper - torso - bones["upper-arm-r"]["angleDeg"]), 3)
    pose["bones"]["forearm-r"]["angleDeg"] = round(norm(fore - upper), 3)
    pose["bones"]["hand-r"]["angleDeg"] = round(norm(hand - fore), 3)
    pose["bones"]["trowel"]["angleDeg"] = offset
    report.append("%-17s R upper %7.2f elbow %6.2f wrist %6.2f  fist %.3f %.3f" % (pose["id"], norm(upper), norm(fore - upper), norm(hand - fore), *fist))


def change(asset, report=None):
    report = [] if report is None else report
    for bone in asset["rig"]["bones"]:
        for key, value in BONES.get(bone["id"], {}).items():
            if key.startswith("mass."):
                bone["mass"][key[5:]] = value
            else:
                bone[key] = value
    for anchor in asset["anchors"]:
        if anchor["id"] in ANCHORS:
            anchor["x"], anchor["y"] = ANCHORS[anchor["id"]]
    for pose in asset["poses"]:
        solve(asset, pose, report)
    asset["dimensions"].update(DIMENSIONS)
    height = DIMENSIONS["heightM"]
    symbols = json.loads(json.dumps(asset["symbols"], ensure_ascii=False).replace("1.145", "%.3f" % height).replace("1,145", ("%.3f" % height).replace(".", ",")))
    asset["symbols"][:] = symbols
    for reference in asset["provenance"]["references"]:
        if reference["id"] == "garden-spec":
            reference["citation"] = GARDEN_SPEC
    return ["rig", "anchors", "poses", "dimensions", "symbols", "provenance"]


if __name__ == "__main__":
    if "--write" in sys.argv:
        print(splice.replace_members(change))
    else:
        document = json.load(open(splice.ASSET, encoding="utf-8"))
        report = []
        change(document["asset"], report)
        print("\n".join(report))
        if "--candidate" in sys.argv:
            target = os.path.join(here, "🗑️generated", "r3-kneeling-gardener", "🔣️.json")
            json.dump(document, open(target, "w", encoding="utf-8"), ensure_ascii=False)
            print(target)
