"""🦴 person-with-umbrella: shoulder and pelvic girdle on the lateral rest placement of the rig (`rig-lateral-rest.md`) and the hand-modelled girdle skeleton of every scale.

Rig: `clavicle-l/-r` (sternoclavicular to acromioclavicular joint) under `torso`, `ilium-l/-r` (sacroiliac joint to hip joint) under `pelvis`; the upper arms and thighs are re-parented without moving. Structure lists: clavicle, scapula (blade, spine, acromion, neck, coracoid), hip bone (iliac wing, acetabulum, pubis, ischium, ischiopubic ramus) and a broad sacrum, authored in the torso / pelvis frame (x up, y back, z left) and written in the frame of the girdle bone.

Usage: python3 rework-05-person-with-umbrella-girdles.py            candidate `🗑️generated/rework-person-with-umbrella/cand/🔣️.json`
       python3 rework-05-person-with-umbrella-girdles.py --apply    re-reads and writes catalog (`rig`, `poses`, `volumes`) and the volumes source
"""
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "person-with-umbrella", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "person-with-umbrella.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-person-with-umbrella", "pwu-cand", "🔣️.json")
DEG = 180 / math.pi

SC = (0.3806, -0.066, 0.022)
AC = (0.415, 0.010, 0.165)
SI = (0.075, 0.045, 0.035)
CLAVICLE = {"x": 0.3806, "y": -0.066, "z": 0.022, "angleDeg": 65.647, "abductionDeg": 59.7417, "twistDeg": 152.3444}
ILIUM = {"x": 0.075, "y": 0.045, "z": 0.035, "angleDeg": -149.0362, "abductionDeg": 31.2143, "twistDeg": -72.7272}
GIRDLE = ("clavicle-l", "clavicle-r", "ilium-l", "ilium-r")
REMOVED = {"50": [], "100": ["ischiopubic"], "200": [], "500": []}
REMOVED_MID = {"50": ["sacrum"], "100": ["sacrum"], "200": ["shoulder-girdle"], "500": ["shoulder-girdle"]}
OWNED = ["ilium-body", "clavicle", "scapula", "scapular-spine", "acromion", "scapula-neck", "coracoid", "ilium", "iliac-wing", "acetabulum", "pubis", "pubis-body", "ischium", "ischiopubic-ramus", "hip-bone", "joint-clavicle", "joint-ilium"]
BLADE = (0.0, -0.53, 0.848)
CREST = (0.0, 0.80, -0.60)


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


def frames(bones):
    by_id = {bone["id"]: bone for bone in bones}
    out = {}

    def solve(bone_id):
        if bone_id in out:
            return out[bone_id]
        bone = by_id[bone_id]
        origin, basis = ([0.0, 0.0, 0.0], turn(0, 0, 0)) if bone["parent"] is None else solve(bone["parent"])
        here = [origin[i] + apply(basis, [bone["x"], bone["y"], bone.get("z", 0.0)])[i] for i in range(3)]
        out[bone_id] = (here, mul(basis, turn(bone["angleDeg"], bone.get("abductionDeg", 0.0), bone.get("twistDeg", 0.0))))
        return out[bone_id]

    for bone in bones:
        solve(bone["id"])
    return out


def norm(a):
    n = math.sqrt(sum(c * c for c in a))
    return [c / n for c in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


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
    return [round(c, 5) for c in q]


def columns(a, b, c):
    return [[a[i], b[i], c[i]] for i in range(3)]


def sections(points, rows, hint):
    out = []
    for i, row in enumerate(rows):
        tangent = norm(sub(points[min(i + 1, len(points) - 1)], points[max(i - 1, 0)]))
        dot = sum(hint[k] * tangent[k] for k in range(3))
        a = norm([hint[k] - dot * tangent[k] for k in range(3)])
        out.append({"at": points[i], "rx": row[1], "ry": row[2], "exponent": 2, "rotation": quaternion(columns(a, cross(tangent, a), tangent))})
    return out


class Frame:
    """📐 Authoring frame of a parent bone and the girdle bone the solid is written in."""

    def __init__(self, solved, parent, bone, side):
        self.side = side
        self.bone = bone
        origin_p, basis_p = solved[parent]
        origin_b, basis_b = solved[bone]
        self.turn = mul(transpose(basis_b), basis_p)
        self.shift = apply(transpose(basis_b), sub(origin_p, origin_b))

    def mirrored(self, p):
        return [p[0], p[1], -p[2]] if self.side == "r" else list(p)

    def point(self, p):
        q = apply(self.turn, self.mirrored(p))
        return [round(q[i] + self.shift[i], 5) for i in range(3)]

    def direction(self, d):
        return norm(apply(self.turn, self.mirrored(d)))

    def base(self, sid):
        return {"id": f"{sid}-{self.side}", "bone": self.bone, "material": "bone", "role": "body", "tone": 0.9, "member": self.bone, "part": "member"}

    def capsule(self, sid, p0, p1, r0, r1, **more):
        return {**self.base(sid), "kind": "capsule", "p0": self.point(p0), "p1": self.point(p1), "r0": r0, "r1": r1, **more}

    def ellipsoid(self, sid, centre, radii, axes=((1, 0, 0), (0, 1, 0)), **more):
        a = self.direction(axes[0])
        b = self.direction(axes[1])
        return {**self.base(sid), "kind": "ellipsoid", "radii": list(radii), "transform": {"translation": self.point(centre), "rotation": quaternion(columns(a, b, cross(a, b)))}, **more}

    def loft(self, sid, rows, wide, **more):
        return {**self.base(sid), "kind": "loft", "sections": sections([self.point(row[0]) for row in rows], rows, self.direction(wide)), "capStart": True, "capEnd": True, **more}


def shoulder(f, scale):
    """🦴 Clavicle and scapula of one side in the torso frame."""
    joint = {**f.ellipsoid("joint-clavicle", SC, (0.011, 0.011, 0.011)), "tone": 0.4, "part": "joint"}
    if scale == "50":
        return [
            f.loft("clavicle", [(SC, 0.011, 0.011), ((0.390, -0.055, 0.065), 0.0075, 0.0065), ((0.399, -0.030, 0.101), 0.0065, 0.006), ((0.408, 0.003, 0.137), 0.009, 0.0055), (AC, 0.011, 0.005)], (0, -1, 0)),
            f.loft("scapula", [((0.398, 0.070, 0.098), 0.040, 0.004), ((0.375, 0.068, 0.104), 0.048, 0.004), ((0.345, 0.072, 0.102), 0.044, 0.004), ((0.305, 0.078, 0.098), 0.034, 0.004), ((0.268, 0.084, 0.095), 0.022, 0.004), ((0.242, 0.088, 0.093), 0.010, 0.004)], BLADE),
            f.capsule("scapular-spine", (0.372, 0.094, 0.066), (0.405, 0.040, 0.150), 0.005, 0.007),
            f.capsule("acromion", (0.405, 0.040, 0.150), (0.416, 0.006, 0.172), 0.007, 0.0075),
            f.capsule("scapula-neck", (0.384, 0.045, 0.128), (0.387, 0.016, 0.150), 0.012, 0.016),
            f.capsule("coracoid", (0.398, 0.030, 0.140), (0.392, -0.022, 0.138), 0.007, 0.005),
            joint,
        ]
    if scale == "100":
        return [
            f.loft("clavicle", [(SC, 0.0095, 0.0095), ((0.3945, -0.0425, 0.083), 0.0075, 0.0075), ((0.408, 0.003, 0.137), 0.008, 0.0075), (AC, 0.009, 0.0075)], (0, -1, 0), slender=True),
            f.loft("scapula", [((0.398, 0.070, 0.098), 0.040, 0.0075), ((0.360, 0.070, 0.103), 0.047, 0.0075), ((0.305, 0.078, 0.098), 0.034, 0.0075), ((0.245, 0.088, 0.093), 0.012, 0.0075)], BLADE),
            f.capsule("scapular-spine", (0.372, 0.094, 0.066), (0.405, 0.040, 0.150), 0.0075, 0.0075),
            f.capsule("acromion", (0.405, 0.040, 0.150), (0.416, 0.006, 0.172), 0.0075, 0.0075),
            f.capsule("scapula-neck", (0.384, 0.045, 0.128), (0.387, 0.016, 0.150), 0.012, 0.016),
            joint,
        ]
    if scale == "200":
        return [
            f.capsule("clavicle", SC, AC, 0.009, 0.008, slender=True),
            f.ellipsoid("scapula", (0.325, 0.074, 0.100), (0.078, 0.048, 0.015), ((1, 0, 0), BLADE)),
        ]
    return [f.capsule("clavicle", SC, AC, 0.009, 0.008, slender=True)]


def hip(f, scale):
    """🦴 Hip bone of one side in the pelvis frame."""
    if scale == "50":
        return [
            f.loft("iliac-wing", [((0.028, 0.004, 0.088), 0.026, 0.011), ((0.052, 0.012, 0.091), 0.046, 0.007), ((0.076, 0.020, 0.092), 0.074, 0.005), ((0.096, 0.023, 0.093), 0.076, 0.005), ((0.110, 0.024, 0.094), 0.060, 0.006), ((0.119, 0.024, 0.094), 0.036, 0.006), ((0.123, 0.024, 0.094), 0.012, 0.005)], CREST),
            f.capsule("ilium-body", (0.080, 0.060, 0.050), (0.028, 0.008, 0.086), 0.012, 0.016),
            f.ellipsoid("acetabulum", (0.006, 0.004, 0.070), (0.030, 0.030, 0.016)),
            f.capsule("pubis", (0.002, -0.020, 0.066), (-0.022, -0.060, 0.010), 0.009, 0.010),
            f.capsule("pubis-body", (-0.020, -0.062, 0.010), (-0.044, -0.056, 0.012), 0.009, 0.008),
            f.capsule("ischium", (-0.010, 0.018, 0.078), (-0.068, 0.036, 0.058), 0.013, 0.015),
            f.capsule("ischiopubic-ramus", (-0.070, 0.028, 0.054), (-0.046, -0.050, 0.014), 0.007, 0.007),
            {**f.ellipsoid("joint-ilium", SI, (0.012, 0.012, 0.012)), "tone": 0.4, "part": "joint"},
        ]
    if scale == "100":
        return [
            f.loft("iliac-wing", [((0.028, 0.004, 0.088), 0.026, 0.011), ((0.060, 0.015, 0.091), 0.058, 0.0075), ((0.090, 0.022, 0.093), 0.076, 0.0075), ((0.112, 0.024, 0.094), 0.056, 0.0075), ((0.122, 0.024, 0.094), 0.020, 0.0075)], CREST),
            f.capsule("ilium-body", (0.080, 0.060, 0.050), (0.028, 0.008, 0.086), 0.012, 0.016),
            f.ellipsoid("acetabulum", (0.006, 0.004, 0.070), (0.030, 0.030, 0.016)),
            f.capsule("pubis", (0.002, -0.020, 0.066), (-0.030, -0.060, 0.011), 0.009, 0.010),
            f.capsule("ischium", (-0.010, 0.018, 0.078), (-0.068, 0.036, 0.058), 0.013, 0.015),
            f.capsule("ischiopubic-ramus", (-0.070, 0.028, 0.054), (-0.046, -0.050, 0.014), 0.0075, 0.0075),
        ]
    if scale == "200":
        return [f.capsule("hip-bone", (0.085, 0.040, 0.050), (0.004, 0.004, 0.078), 0.020, 0.024)]
    return []


def sacrum(scale):
    """🦴 Sacrum with its alae and the coccyx, in the pelvis frame."""
    rows = {
        "50": [((0.112, 0.048, 0), 0.058, 0.022), ((0.075, 0.068, 0), 0.054, 0.016), ((0.045, 0.080, 0), 0.036, 0.012), ((0.015, 0.084, 0), 0.018, 0.008), ((-0.012, 0.074, 0), 0.008, 0.006)],
        "100": [((0.112, 0.048, 0), 0.058, 0.022), ((0.070, 0.070, 0), 0.052, 0.016), ((0.035, 0.082, 0), 0.028, 0.011), ((0.004, 0.082, 0), 0.012, 0.0085)],
    }[scale]
    return {"id": "sacrum", "kind": "loft", "bone": "pelvis", "material": "bone", "role": "body", "tone": 0.9, "sections": sections([list(row[0]) for row in rows], rows, [0.0, 0.0, 1.0]), "capStart": True, "capEnd": True, "member": "pelvis", "part": "member"}


def girdle_bones():
    out = []
    for side, sign, left, right in (("l", 1, "Left", "Linkes"), ("r", -1, "Right", "Rechtes")):
        twist = {"twistMinDeg": -40, "twistMaxDeg": 5} if side == "l" else {"twistMinDeg": -5, "twistMaxDeg": 40}
        out.append({"id": f"clavicle-{side}", "parent": "torso", "title": {"en": f"{left} clavicle", "de": f"{right} Schlüsselbein"}, "x": CLAVICLE["x"], "y": CLAVICLE["y"], "z": sign * CLAVICLE["z"], "angleDeg": CLAVICLE["angleDeg"], "abductionDeg": sign * CLAVICLE["abductionDeg"], "twistDeg": sign * CLAVICLE["twistDeg"], "lengthM": 0.16555, "joint": {"type": "revolute", "minDeg": -8, "maxDeg": 35, **twist}})
        out.append({"id": f"ilium-{side}", "parent": "pelvis", "title": {"en": f"{left} hip bone", "de": f"{right} Hüftbein"}, "x": ILIUM["x"], "y": ILIUM["y"], "z": sign * ILIUM["z"], "angleDeg": ILIUM["angleDeg"], "abductionDeg": sign * ILIUM["abductionDeg"], "twistDeg": sign * ILIUM["twistDeg"], "lengthM": 0.10187, "joint": {"type": "revolute", "minDeg": -4, "maxDeg": 4}})
    return out


def rig(asset):
    before = frames(asset["rig"]["bones"])
    girdle = girdle_bones()
    out = []
    for bone in asset["rig"]["bones"]:
        if bone["id"] in GIRDLE:
            continue
        if bone["id"] == "upper-arm-l":
            out.extend(g for g in girdle if g["id"].startswith("clavicle"))
        if bone["id"] == "thigh-l":
            out.extend(g for g in girdle if g["id"].startswith("ilium"))
        out.append(bone)
    solved = frames([bone for bone in out if bone["id"] in ("pelvis", "torso") + GIRDLE])
    for bone in out:
        limb = {"upper-arm": "clavicle", "thigh": "ilium"}.get(bone["id"][:-2])
        if limb is None:
            continue
        origin, basis = before[bone["id"]]
        origin_p, basis_p = solved[f"{limb}-{bone['id'][-1]}"]
        a, b, t = angles(mul(transpose(basis_p), basis))
        bone["parent"] = f"{limb}-{bone['id'][-1]}"
        bone["x"], bone["y"], bone["z"] = (round(c, 5) for c in apply(transpose(basis_p), sub(origin, origin_p)))
        bone["angleDeg"], bone["abductionDeg"], bone["twistDeg"] = round(a, 4), round(b, 4), round(t, 4)
    asset["rig"]["bones"] = out
    after = frames(out)
    moved = max(abs(after[k][0][i] - before[k][0][i]) for k in before if k not in GIRDLE for i in range(3))
    turned = max(abs(after[k][1][i][j] - before[k][1][i][j]) for k in before if k not in GIRDLE for i in range(3) for j in range(3))
    print(f"[DEBUG] largest move of an existing bone origin {moved:.6f} m, basis entry {turned:.6f}")
    for pose in asset["poses"]:
        for bone_id in GIRDLE:
            pose["bones"].setdefault(bone_id, {"angleDeg": 0})
    return after


def volumes(asset_volumes, solved):
    for scale, lists in asset_volumes.items():
        gone = {f"{name}-{side}" for name in REMOVED[scale] + OWNED for side in "lr"} | set(REMOVED_MID[scale])
        kept = [solid for solid in lists["structure"] if solid["id"] not in gone]
        made = []
        for side in "lr":
            made.extend(shoulder(Frame(solved, "torso", f"clavicle-{side}", side), scale))
            made.extend(hip(Frame(solved, "pelvis", f"ilium-{side}", side), scale))
        if scale in ("50", "100"):
            made.append(sacrum(scale))
        at = next((i for i, solid in enumerate(kept) if solid["id"] == "joint-torso"), len(kept))
        lists["structure"] = kept[:at] + made + kept[at:]


SLENDER = {"50": [], "100": ["fibula"], "200": ["fibula", "humerus", "ulna", "radius"], "500": ["fibula", "humerus", "ulna", "radius", "tibia", "femur"]}
DROPPED = {"50": [], "100": [], "200": ["occipital-condyle", "patella", "malleolus"], "500": ["occipital-condyle", "femur-neck", "femur-head", "femur-trochanter", "femur-condyles", "patella", "tibia-plateau", "malleolus", "humerus-head", "humerus-condyles"]}
FEMUR = [((0, 0, 0), 0.0165), ((0.034, -0.006, 0.058), 0.0165), ((0.13, 0.008, 0.046), 0.0142), ((0.22, 0.013, 0.03), 0.0135), ((0.31, 0.008, 0.014), 0.0142), ((0.398, 0, 0.002), 0.017)]


def coarse(asset_volumes):
    """🦴 Limb skeleton of the coarser scales: long bones keep their true shaft as slender rods, epiphyses below the smallest feature go, joints grow to the feature."""
    for scale, lists in asset_volumes.items():
        gone = {f"{name}-{side}" for name in DROPPED[scale] for side in "lr"}
        slender = {f"{name}-{side}" for name in SLENDER[scale] for side in "lr"}
        kept = [solid for solid in lists["structure"] if solid["id"] not in gone]
        for solid in kept:
            if solid["id"] in slender:
                solid["slender"] = True
            if scale == "200" and solid["id"].startswith("joint-upper-arm"):
                solid["radii"] = [0.015, 0.015, 0.015]
            if scale == "200" and solid["id"].startswith("humerus-condyles"):
                solid["radii"] = [0.015, 0.015, 0.031]
            if scale == "500" and solid["id"].startswith("joint-shank"):
                solid["radii"] = [0.0375, 0.0375, 0.0375]
            if scale == "500" and solid["id"] in ("femur-l", "femur-r"):
                sign = 1 if solid["id"].endswith("-l") else -1
                solid["sections"] = [{"at": [at[0], at[1], sign * at[2]], "rx": r, "ry": r} for at, r in FEMUR]
        if scale == "500" and not any(solid["id"] == "joint-upper-arm-l" for solid in kept):
            for side in "lr":
                kept.append({"id": f"joint-upper-arm-{side}", "kind": "ellipsoid", "bone": f"upper-arm-{side}", "material": "bone", "role": "body", "tone": 0.4, "radii": [0.0375, 0.0375, 0.0375], "transform": {"translation": [0, 0, 0]}, "member": f"upper-arm-{side}", "part": "joint"})
        lists["structure"] = kept


SPINE = [((0.0076, 0.036, 0), 0.024, 0.018), ((0.050, 0.026, 0), 0.023, 0.017), ((0.0926, 0.022, 0), 0.022, 0.017), ((0.150, 0.032, 0), 0.020, 0.016), ((0.2026, 0.048, 0), 0.018, 0.015), ((0.265, 0.060, 0), 0.017, 0.014), ((0.3226, 0.064, 0), 0.016, 0.013), ((0.380, 0.056, 0), 0.015, 0.012), ((0.4276, 0.044, 0), 0.014, 0.012)]
NECK = [((-0.0219, 0.044, 0), 0.013, 0.011), ((0.020, 0.030, 0), 0.013, 0.011), ((0.0631, 0.020, 0), 0.012, 0.010), ((0.1061, 0.014, 0), 0.013, 0.010)]
SHANK_HEM = {"50": 0.006, "100": 0.010, "200": 0.015}


def meet(asset_volumes):
    """🔗 Parts that a joint holds together meet there: column with explicit section frames, trouser hem on the shoe, patella on the condyles, medial malleolus on the tibia, metacarpals up to the knuckles, tibia down to the ankle."""
    for scale, lists in asset_volumes.items():
        by_id = {solid["id"]: solid for solid in lists["structure"]}
        if scale in ("50", "100"):
            by_id["spine"]["sections"] = sections([list(row[0]) for row in SPINE], SPINE, [0.0, 0.0, 1.0])
            by_id["cervical-spine"]["sections"] = sections([list(row[0]) for row in NECK], NECK, [0.0, 0.0, 1.0])
            by_id["joint-head"]["transform"]["translation"] = [-0.005, 0.03, 0]
            for side, sign in (("l", 1), ("r", -1)):
                by_id[f"patella-{side}"]["transform"]["translation"][1] = 0.034
                by_id[f"malleolus-{side}"]["transform"]["translation"][2] = -sign * 0.024
        if scale in ("50", "100"):
            for side in "lr":
                by_id[f"fibula-{side}"]["p1"][0] = 0.432
        if scale == "50":
            for side in "lr":
                by_id[f"mandible-ramus-{side}"]["p0"][0] = 0.1225
        if scale == "50":
            for side in "lr":
                for digit, finger in (("2", "index"), ("3", "middle"), ("4", "ring"), ("5", "little")):
                    by_id[f"metacarpal-{side}-{digit}"]["p1"] = list(by_id[f"phalanges-{side}-{finger}"]["sections"][0]["at"])
        for side in "lr":
            tibia = by_id.get(f"tibia-{side}")
            if tibia is not None:
                tibia["sections"][-1]["at"][0] = 0.4185
        if scale in ("50", "100", "200"):
            for solid in lists["body"]:
                if solid["id"].startswith("rib-") and solid.get("kind") == "loft" and 0.54 < solid["sections"][-1]["at"][0] < 0.5585:
                    solid["sections"][-1]["at"][0] = 0.5585
        if scale in SHANK_HEM:
            for solid in lists["body"]:
                if solid["id"] in ("shank-l", "shank-r"):
                    rows = solid["sections"]
                    if rows[-1]["at"][0] > 0.3955:
                        rows.pop()
                    last = rows[-1]
                    rows.append({**last, "at": [round(last["at"][0] + SHANK_HEM[scale], 5), last["at"][1], last["at"][2]]})


def main():
    document = json.load(open(CATALOG, encoding="utf-8"))
    source = json.load(open(SOURCE, encoding="utf-8"))
    asset = document["asset"]
    solved = rig(asset)
    asset["volumes"] = source["volumes"]
    volumes(asset["volumes"], solved)
    coarse(asset["volumes"])
    meet(asset["volumes"])
    if "--apply" in sys.argv:
        with open(SOURCE, "w", encoding="utf-8") as handle:
            json.dump(source, handle, ensure_ascii=False, indent=2)
            handle.write("\n")
        with open(CATALOG, "w", encoding="utf-8") as handle:
            json.dump(document, handle, ensure_ascii=False, indent=2)
            handle.write("\n")
        print("[DEBUG] applied to catalog and source")
    else:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        with open(CANDIDATE, "w", encoding="utf-8") as handle:
            json.dump(document, handle, ensure_ascii=False, indent=2)
        print("[DEBUG] candidate", CANDIDATE)


main()
