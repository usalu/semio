"""🦴 kneeling-gardener: shoulder and pelvic girdle on the lateral rest placement of the rig (`rig-lateral-rest.md`).

Adds `clavicle-l/-r` (sternoclavicular to acromioclavicular joint) and `hip-bone-l/-r` (sacroiliac joint to hip joint), hangs the upper arms and thighs from them without moving their rest frames, and replaces the girdle solids of the structure lists by hand-modelled ones bound to the new bones. All points below are hand-chosen world points of the rest pose, `[ahead, up, left]` in metres, left side; the right side is the mirror.

Usage: python3 rework-kneeling-gardener-girdles.py            writes the candidate document
       python3 rework-kneeling-gardener-girdles.py --apply    writes `rig` into the catalog file and the volumes source (publish with volumes-insert.py)
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "kneeling-gardener", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "kneeling-gardener.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-kneeling-gardener", "candidate", "🔣️.json")

spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)

STERNOCLAVICULAR = (0.050, 1.0023, 0.020)
ACROMIOCLAVICULAR = (-0.012, 1.0223, 0.158)
SACROILIAC = (-0.058, 0.565, 0.040)
HIP = (0.0, 0.5, 0.09)
TITLES = {
    "clavicle-l": {"en": "Left clavicle with scapula", "de": "Linkes Schlüsselbein mit Schulterblatt"},
    "clavicle-r": {"en": "Right clavicle with scapula", "de": "Rechtes Schlüsselbein mit Schulterblatt"},
    "hip-bone-l": {"en": "Left hip bone", "de": "Linkes Hüftbein"},
    "hip-bone-r": {"en": "Right hip bone", "de": "Rechtes Hüftbein"},
}
JOINTS = {
    "clavicle-l": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40, "twistMaxDeg": 5},
    "clavicle-r": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -5, "twistMaxDeg": 40},
    "hip-bone-l": {"type": "revolute", "minDeg": -3, "maxDeg": 3},
    "hip-bone-r": {"type": "revolute", "minDeg": -3, "maxDeg": 3},
}
OLD = ("sk-clavicle-", "sk-scapula-", "sk-acromion-", "sk-hip-bone-", "sk-pubis-")
TONE = {"50": 0.85, "100": 0.85, "200": 0.57, "500": 0.57}


def side_point(p, sign):
    return [p[0], p[1], sign * p[2]]


def unit(v):
    n = math.sqrt(sum(c * c for c in v))
    return [c / n for c in v]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


class Bone:
    """One girdle bone of one side: writes solids from world rest points into its frame."""

    def __init__(self, bone_id, frame, sign, scale):
        self.id, self.origin, self.basis, self.sign, self.scale = bone_id, frame[0], frame[1], sign, scale
        self.suffix = bone_id[-2:]

    def local(self, p):
        return K.r5(K.apply(K.transpose(self.basis), sub(side_point(p, self.sign), self.origin)))

    def base(self, name, extra=None):
        return {"id": name + self.suffix, "bone": self.id, "material": "bone", "role": "body", "tone": TONE[self.scale], **(extra or {})}

    def tail(self):
        return {"member": self.id, "part": "member"}

    def capsule(self, name, p0, p1, r0, r1, slender=False):
        return {**self.base(name), "kind": "capsule", "p0": self.local(p0), "p1": self.local(p1), "r0": r0, "r1": r1, **({"slender": True} if slender else {}), **self.tail()}

    def ellipsoid(self, name, at, radii):
        return {**self.base(name), "kind": "ellipsoid", "radii": radii, "transform": {"translation": self.local(at)}, **self.tail()}

    def loft(self, name, sections, sheet=False):
        """Sections `(centre, across, upward, rx, ry)`: an ellipse with radius `rx` along `across` and `ry` along `upward`, both world directions of the left side."""
        out = []
        for centre, across, upward, rx, ry in sections:
            x = unit(side_point(across, self.sign))
            y = unit(side_point(upward, self.sign))
            k = sum(x[i] * y[i] for i in range(3))
            y = unit([y[i] - k * x[i] for i in range(3)])
            n = cross(x, y)
            world = [[x[i], y[i], n[i]] for i in range(3)]
            out.append({"at": self.local(centre), "rx": rx, "ry": ry, "exponent": 2, "rotation": K.r5(K.quaternion(K.mul(K.transpose(self.basis), world)))})
        return {**self.base(name), "kind": "loft", "sections": out, "capStart": True, "capEnd": True, **({"sheet": True} if sheet else {}), **self.tail()}


AHEAD, UP, LEFT = (1, 0, 0), (0, 1, 0), (0, 0, 1)
BLADE = (0.86, 0.0, 0.51)
BLADE_NORMAL = (-0.51, 0.0, 0.86)


def shoulder(b):
    """Clavicle, scapula with spine, acromion and coracoid of one side for the scale of `b`."""
    if b.scale == "50":
        return [
            b.loft("sk-clavicle", [
                ((0.050, 1.0023, 0.020), AHEAD, UP, 0.011, 0.0105),
                ((0.052, 1.0080, 0.062), AHEAD, UP, 0.0065, 0.0060),
                ((0.030, 1.0150, 0.108), AHEAD, UP, 0.0062, 0.0052),
                ((0.000, 1.0200, 0.140), AHEAD, UP, 0.0080, 0.0048),
                ((-0.012, 1.0223, 0.158), AHEAD, UP, 0.0095, 0.0050),
            ]),
            b.loft("sk-scapula", [
                ((-0.086, 0.938, 0.070), BLADE_NORMAL, UP, 0.0040, 0.072),
                ((-0.072, 0.944, 0.094), BLADE_NORMAL, UP, 0.0040, 0.060),
                ((-0.050, 0.958, 0.116), BLADE_NORMAL, UP, 0.0050, 0.040),
                ((-0.026, 0.972, 0.132), BLADE_NORMAL, UP, 0.0080, 0.022),
                ((-0.014, 0.979, 0.141), BLADE_NORMAL, UP, 0.0120, 0.019),
            ]),
            b.capsule("sk-scapular-spine", (-0.092, 0.982, 0.068), (-0.052, 1.008, 0.140), 0.0045, 0.0060),
            b.capsule("sk-acromion", (-0.052, 1.008, 0.140), (-0.016, 1.0215, 0.166), 0.0060, 0.0072),
            b.capsule("sk-coracoid", (-0.014, 0.998, 0.132), (0.020, 0.992, 0.128), 0.0060, 0.0050),
        ]
    if b.scale == "100":
        return [
            b.capsule("sk-clavicle", STERNOCLAVICULAR, ACROMIOCLAVICULAR, 0.0095, 0.0085),
            b.loft("sk-scapula", [
                ((-0.086, 0.938, 0.070), BLADE_NORMAL, UP, 0.0075, 0.072),
                ((-0.056, 0.954, 0.110), BLADE_NORMAL, UP, 0.0075, 0.046),
                ((-0.016, 0.978, 0.140), BLADE_NORMAL, UP, 0.0110, 0.020),
            ]),
            b.capsule("sk-acromion", (-0.090, 0.984, 0.070), (-0.016, 1.0215, 0.164), 0.0075, 0.0080),
        ]
    if b.scale == "200":
        return [b.capsule("sk-clavicle", (0.056, 0.997, 0.004), ACROMIOCLAVICULAR, 0.0085, 0.0080, slender=True)]
    return [b.capsule("sk-clavicle", (0.056, 0.997, 0.004), (-0.006, 1.006, 0.156), 0.0085, 0.0100, slender=True)]


def hip(b):
    """Ilium wing with its posterior part, acetabulum, ischium and both pubic rami of one side for the scale of `b`."""
    if b.scale == "50":
        return [
            b.loft("sk-ilium", [
                ((-0.004, 0.528, 0.088), LEFT, AHEAD, 0.0130, 0.030),
                ((-0.008, 0.555, 0.104), LEFT, AHEAD, 0.0060, 0.046),
                ((-0.010, 0.585, 0.122), LEFT, AHEAD, 0.0045, 0.062),
                ((-0.012, 0.606, 0.131), LEFT, AHEAD, 0.0060, 0.066),
                ((-0.012, 0.616, 0.130), LEFT, AHEAD, 0.0070, 0.058),
            ]),
            b.capsule("sk-ilium-posterior", (-0.036, 0.598, 0.122), (-0.078, 0.578, 0.046), 0.0090, 0.0110),
            b.capsule("sk-sacroiliac", (-0.058, 0.565, 0.040), (-0.012, 0.528, 0.078), 0.0110, 0.0130),
            b.ellipsoid("sk-acetabulum", (0.0, 0.5, 0.078), [0.031, 0.031, 0.020]),
            b.capsule("sk-ischium", (-0.014, 0.486, 0.070), (-0.036, 0.424, 0.058), 0.0120, 0.0140),
            b.capsule("sk-pubis", (0.018, 0.496, 0.070), (0.066, 0.476, 0.008), 0.0085, 0.0090),
            b.capsule("sk-ischiopubic-ramus", (-0.030, 0.428, 0.054), (0.062, 0.458, 0.010), 0.0060, 0.0070),
        ]
    if b.scale == "100":
        return [
            b.loft("sk-ilium", [
                ((-0.004, 0.528, 0.088), LEFT, AHEAD, 0.0130, 0.030),
                ((-0.010, 0.580, 0.118), LEFT, AHEAD, 0.0075, 0.058),
                ((-0.012, 0.614, 0.130), LEFT, AHEAD, 0.0080, 0.062),
            ]),
            b.capsule("sk-sacroiliac", (-0.062, 0.572, 0.042), (-0.010, 0.526, 0.080), 0.0110, 0.0130),
            b.ellipsoid("sk-acetabulum", (0.0, 0.5, 0.078), [0.031, 0.031, 0.020]),
            b.capsule("sk-ischium", (-0.014, 0.486, 0.070), (-0.036, 0.426, 0.058), 0.0120, 0.0140),
            b.capsule("sk-pubis", (0.018, 0.494, 0.070), (0.064, 0.470, 0.008), 0.0090, 0.0090),
        ]
    return []


def place(asset):
    bones = asset["rig"]["bones"]
    if any(bone["id"] == "clavicle-l" for bone in bones):
        raise SystemExit("already converted")
    old = K.frames(copy.deepcopy(bones))
    for side, sign in (("l", 1), ("r", -1)):
        for new_id, parent, limb, joint, tip in ((f"clavicle-{side}", "torso", f"upper-arm-{side}", STERNOCLAVICULAR, ACROMIOCLAVICULAR), (f"hip-bone-{side}", "pelvis", f"thigh-{side}", SACROILIAC, HIP)):
            origin, end = side_point(joint, sign), side_point(tip, sign)
            u = sub(end, origin)
            bone = {"id": new_id, "parent": parent, "title": TITLES[new_id], **K.placed(old[parent], origin, u, (-sign, 0.0, 0.0)), "lengthM": round(math.sqrt(sum(c * c for c in u)), 5), "joint": JOINTS[new_id]}
            index = next(i for i, each in enumerate(bones) if each["id"] == limb)
            bones.insert(index, bone)
            child = bones[index + 1]
            placement = K.fields(K.frames(bones)[new_id], old[limb][0], old[limb][1])
            child["parent"] = new_id
            child.update({key: value for key, value in placement.items() if key not in ("abductionDeg", "twistDeg") or abs(value) > 1e-9})
    new = K.frames(bones)
    for bone_id, (origin, basis) in old.items():
        drift = max(abs(new[bone_id][0][i] - origin[i]) for i in range(3))
        turned = max(abs(new[bone_id][1][i][j] - basis[i][j]) for i in range(3) for j in range(3))
        if drift > 3e-5 or turned > 3e-5:
            raise SystemExit(f"{bone_id} moved by {drift} m / {turned}")
    return new


def model(volumes, frames):
    for scale, volume in volumes.items():
        kept = [solid for solid in volume["structure"] if not solid["id"].startswith(OLD)]
        index = next(i for i, solid in enumerate(kept) if solid["id"] in ("sk-sternum", "sternum")) + 1
        girdle = []
        for side, sign in (("l", 1), ("r", -1)):
            girdle += shoulder(Bone(f"clavicle-{side}", frames[f"clavicle-{side}"], sign, scale))
            girdle += hip(Bone(f"hip-bone-{side}", frames[f"hip-bone-{side}"], sign, scale))
        kept[index:index] = girdle
        for solid in kept:
            if solid["id"] == "sk-pelvic-ring":
                solid["members"] = ["hip-bone-l", "hip-bone-r"]
        volume["structure"] = kept


def insert_module():
    text = open(os.path.join(T, "volumes-insert.py"), encoding="utf8").read()
    scope = {}
    exec(compile(text[: text.index("\ndef main(")], "volumes-insert.py", "exec"), scope)
    return scope


def main():
    original = open(CATALOG, encoding="utf8").read()
    document = json.loads(original)
    asset = document["asset"]
    frames = place(asset)
    model(asset["volumes"], frames)
    if "--apply" not in sys.argv:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        for bone in asset["rig"]["bones"]:
            if bone["id"][:-2] in ("clavicle", "hip-bone", "upper-arm", "thigh"):
                print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass")}))
        print(CANDIDATE)
        return
    insert = insert_module()
    indent = "    "
    start, end = insert["member_range"](original, indent, "rig")
    text = original[:start] + indent + '"rig": ' + insert["dump"](asset["rig"], 2) + original[end:]
    if json.loads(text)["asset"]["rig"] != asset["rig"]:
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf8").read() != original:
        raise SystemExit("the catalog file changed while converting: run again")
    open(CATALOG, "w", encoding="utf8").write(text)
    json.dump({"volumes": asset["volumes"]}, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("applied rig to", CATALOG, "and volumes to", SOURCE)


if __name__ == "__main__":
    main()
