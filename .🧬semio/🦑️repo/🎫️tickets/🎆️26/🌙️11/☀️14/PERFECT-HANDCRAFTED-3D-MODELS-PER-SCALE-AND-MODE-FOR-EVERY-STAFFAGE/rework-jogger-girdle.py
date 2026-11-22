"""🦴️ Jogger, second round pass 1: shoulder and pelvic girdle on the lateral rest placement (`T/rig-lateral-rest.md`).

Usage: python3 rework-jogger-girdle.py <base asset document> <candidate out> [--publish]
The base is the catalog document after `rework-jogger-rig.py`. Clavicles run from the sternoclavicular joint to the acromial end,
hip bones from the sacroiliac joint to the hip joint; arms and thighs are re-parented onto them without moving, so their solids,
shapes, anchors, poses and clips keep their meaning. World axes [ahead, up, left], origin at the hip axis; all points by hand.
"""
import copy
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(HERE, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

ASSET = "humans/jogger"
BACK = [-1, 0, 0]
GIRDLE = {
    "clavicle": {
        "parent": "chest",
        "limb": "upper-arm",
        "joint": [0.058, 0.5754, 0.018],
        "tip": [-0.01, 0.5781, 0.151],
        "title": {"en": "{side} clavicle", "de": "{seite} Schlüsselbein"},
        "limits": {"type": "revolute", "minDeg": -8, "maxDeg": 35},
        "solid": "sk-clavicle",
    },
    "ilium": {
        "parent": "pelvis",
        "limb": "thigh",
        "joint": [-0.045, 0.085, 0.04],
        "tip": [0, 0, 0.09],
        "title": {"en": "{side} hip bone", "de": "{seite} Hüftbein"},
        "limits": {"type": "revolute", "minDeg": -5, "maxDeg": 5},
        "solid": "sk-hip",
    },
}
SIDES = {"l": ("Left", "Linkes", 1), "r": ("Right", "Rechtes", -1)}
ILIAC_BODY_RADIUS = 0.014


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def unit(a):
    n = math.sqrt(dot(a, a))
    return [x / n for x in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def frame(u, w):
    """📐️ A right-handed bone frame from its axis and the wanted joint axis (made perpendicular to the axis)."""
    u = unit(u)
    w = unit(sub(w, [dot(w, u) * x for x in u]))
    return [u, cross(w, u), w]


def local(basis, p):
    return [dot(basis[i], p) for i in range(3)]


def placement(parent, origin, child, at):
    """🧮️ `x, y, z, angleDeg, abductionDeg, twistDeg` of a child frame under a parent frame: R = Rw(angle) · Rv(abduction) · Ru(twist)."""
    x, y, z = local(parent, sub(at, origin))
    u, w = local(parent, child[0]), local(parent, child[2])
    b = math.asin(max(-1, min(1, u[2])))
    a = math.atan2(u[1], u[0])
    v0 = [-math.sin(a), math.cos(a), 0]
    w0 = [-math.cos(a) * math.sin(b), -math.sin(a) * math.sin(b), math.cos(b)]
    t = math.atan2(dot(w, v0), dot(w, w0))
    return {"x": round(x, 5), "y": round(y, 5), "z": round(z, 5), "angleDeg": round(math.degrees(a), 4), "abductionDeg": round(math.degrees(b), 4), "twistDeg": round(math.degrees(t), 4)}


def mirrored(p):
    return {**p, "z": -p["z"], "abductionDeg": -p["abductionDeg"], "twistDeg": -p["twistDeg"]}


def main():
    base, out = sys.argv[1], sys.argv[2]
    with open(base, encoding="utf-8") as f:
        document = json.load(f)
    asset = document["asset"]
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    assert "clavicle-l" not in bones, "girdle already placed"
    upright = [[0, 1, 0], [-1, 0, 0], [0, 0, 1]]
    hanging = [[0, -1, 0], [1, 0, 0], [0, 0, 1]]
    chest = [-bones["torso"]["y"], bones["torso"]["x"] + bones["chest"]["x"], 0]
    arm = bones["upper-arm-l"]
    origins = {"pelvis": [0, 0, 0], "chest": chest}
    limb_origin = {"upper-arm": [chest[0] - arm["y"], chest[1] + arm["x"], arm["z"]], "thigh": [0, 0, bones["thigh-l"]["z"]]}
    lengths, rebuilt = {}, []
    for name, g in GIRDLE.items():
        basis = frame(sub(g["tip"], g["joint"]), BACK)
        length = round(math.dist(g["tip"], g["joint"]), 5)
        lengths[name] = length
        own = placement(upright, origins[g["parent"]], basis, g["joint"])
        limb = placement(basis, g["joint"], hanging, limb_origin[g["limb"]])
        for side, (en, de, sign) in SIDES.items():
            place = own if sign > 0 else mirrored(own)
            hang = limb if sign > 0 else mirrored(limb)
            bones[f"{name}-{side}"] = {"id": f"{name}-{side}", "parent": g["parent"], "title": {"en": g["title"]["en"].format(side=en), "de": g["title"]["de"].format(seite=de)}, **place, "lengthM": length, "joint": dict(g["limits"])}
            bones[f"{g['limb']}-{side}"].update(parent=f"{name}-{side}", **hang)
    for b in asset["rig"]["bones"]:
        for name, g in GIRDLE.items():
            if b["id"].startswith(g["limb"] + "-"):
                rebuilt.append(bones[f"{name}-{b['id'][-1]}"])
        rebuilt.append(b)
    asset["rig"]["bones"] = rebuilt
    for scale, v in asset["volumes"].items():
        if "structure" not in v:
            continue
        structure = []
        for s in v["structure"]:
            structure.append(s)
            for name, g in GIRDLE.items():
                for side in SIDES:
                    bone, limb = f"{name}-{side}", f"{g['limb']}-{side}"
                    if s["id"] == f"{g['solid']}-{side}":
                        s.update(bone=bone, member=bone, p0=[0, 0, 0], p1=[lengths[name], 0, 0])
                    elif name == "ilium" and s["id"] == f"sk-sacroiliac-{side}":
                        structure.append({**copy.deepcopy(s), "id": f"sk-iliac-body-{side}", "bone": bone, "member": bone, "p0": [0, 0, 0], "p1": [lengths[name], 0, 0], "r0": ILIAC_BODY_RADIUS, "r1": ILIAC_BODY_RADIUS})
                    elif s["id"] == f"joint-{limb}":
                        structure.append({**copy.deepcopy(s), "id": f"joint-{bone}", "bone": bone, "member": bone})
        v["structure"] = structure
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w", encoding="utf-8") as f:
        json.dump(document, f, ensure_ascii=False, indent=2)
        f.write("\n")
    if "--publish" in sys.argv:
        with open(base, encoding="utf-8") as f:
            assert lib.load_catalog(ASSET) == json.load(f), "the catalog document changed since the base was taken"
        lib.save_source(ASSET, {"volumes": asset["volumes"]})
        lib.save_catalog(ASSET, document)
    print(json.dumps([{k: b[k] for k in b if k not in ("title", "mass", "joint")} for b in rebuilt if b["id"].endswith("-l") and any(b["id"].startswith(n) for n in ("clavicle", "ilium", "upper-arm", "thigh"))]))


main()
