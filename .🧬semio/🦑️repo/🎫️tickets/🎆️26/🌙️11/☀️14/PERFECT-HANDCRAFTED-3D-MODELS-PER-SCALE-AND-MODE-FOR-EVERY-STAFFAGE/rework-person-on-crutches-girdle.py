"""🦴 Shoulder and pelvic girdle of `humans/person-on-crutches` on the lateral rest placement, stated by hand.

Runs once on the catalog asset as `rework-person-on-crutches-skeleton.ts` left it (it refuses a rig that already has `clavicle-l`):
clavicles from the sternoclavicular to the acromioclavicular joint, hip bones from the sacroiliac joint to the hip joint, a lumbar bone
below the thorax, shoulder joints 0.366 m and hip joints 0.176 m apart, the girdle solids re-bound to their bones, sleeve heads lowered
onto the shoulder line, long bones of the coarse scales at their true shaft, front and plan drawings shifted with the limbs.

Usage: python3 rework-person-on-crutches-girdle.py [--publish]   (without the flag only the candidate document is written)

See ./rig-lateral-rest.md and ./rework-brief.md
"""
import glob
import json
import math
import os
import re
import sys

class Vec(list):
    """➡️ A point or direction with the three operations the chisel needs."""

    def __add__(self, other):
        return Vec([a + b for a, b in zip(self, other)])

    def __sub__(self, other):
        return Vec([a - b for a, b in zip(self, other)])

    def __mul__(self, k):
        return Vec([a * k for a in self])


class Mat:
    """🧮 A 3 x 3 matrix whose columns are the axes of a bone frame."""

    def __init__(self, rows):
        self.r = [[float(c) for c in row] for row in rows]

    def __matmul__(self, other):
        if isinstance(other, Mat):
            return Mat([[sum(self.r[i][k] * other.r[k][j] for k in range(3)) for j in range(3)] for i in range(3)])
        return Vec([sum(self.r[i][k] * other[k] for k in range(3)) for i in range(3)])

    @property
    def T(self):
        return Mat([[self.r[j][i] for j in range(3)] for i in range(3)])

    def __getitem__(self, i):
        return self.r[i]

    def col(self, j):
        return Vec([self.r[i][j] for i in range(3)])

    def diff(self, other):
        return max(abs(self.r[i][j] - other.r[i][j]) for i in range(3) for j in range(3))

T = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(T, *[".."] * 7))
CATALOG = glob.glob(os.path.join(ROOT, "*teaching", "*architecture", "*staffage", "*catalog", "*humans", "person-on-crutches", "*.json"))[0]
SOURCE = os.path.join(T, "volumes", "humans", "person-on-crutches.json")
CANDIDATE = os.path.join(glob.glob(os.path.join(T, "*generated"))[0], "rework-person-on-crutches", "cand", os.path.basename(CATALOG))

SHOULDER_HALF, HIP_HALF = 0.183, 0.088
LUMBAR = 0.196
STERNAL_END, ACROMIAL_END = [0.461, -0.056, 0.02], [0.45, 0.01, 0.1844]
ACROMION_DROP = 0.018
ILIUM = {"x": 0.07, "y": 0.045, "z": 0.035, "angleDeg": -147.2648, "abductionDeg": 32.4927, "twistDeg": -70.9482, "lengthM": 0.09863}
SLEEVE_HEAD = 0.77
SHAFTS = {"femur": [0.019, 0.016, 0.014, 0.02], "tibia": [0.02, 0.017, 0.0125, 0.016], "humerus": [0.016, 0.011, 0.0105, 0.0125]}


def r5(x):
    return round(float(x), 5)


def rz(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return Mat([[c, -s, 0], [s, c, 0], [0, 0, 1]])


def ry(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return Mat([[c, 0, s], [0, 1, 0], [-s, 0, c]])


def rx(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return Mat([[1, 0, 0], [0, c, -s], [0, s, c]])


def rest(bone):
    return rz(bone["angleDeg"]) @ ry(-bone.get("abductionDeg", 0)) @ rx(-bone.get("twistDeg", 0))


def frames(bones):
    out = {}
    for bone in bones:
        if bone["parent"] is None:
            origin, basis = Vec([bone.get("x", 0), bone.get("y", 0), bone.get("z", 0)]), Mat([[1, 0, 0], [0, 1, 0], [0, 0, 1]])
        else:
            po, pb = out[bone["parent"]]
            origin, basis = po + pb @ Vec([bone.get("x", 0), bone.get("y", 0), bone.get("z", 0)]), pb
        out[bone["id"]] = (origin, basis @ rest(bone))
    return out


def solve(parent, origin, basis):
    po, pb = parent
    m = pb.T @ basis
    x, y, z = pb.T @ (origin - po)
    beta = -math.asin(max(-1, min(1, m[2][0])))
    alpha, gamma = math.atan2(m[1][0], m[0][0]), math.atan2(m[2][1], m[2][2])
    return {"x": r5(x), "y": r5(y), "z": r5(z), "angleDeg": round(math.degrees(alpha), 4), "abductionDeg": round(-math.degrees(beta), 4), "twistDeg": round(-math.degrees(gamma), 4)}


def quat(m):
    tr = m[0][0] + m[1][1] + m[2][2]
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
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
    return q


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    q = [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]
    n = math.sqrt(sum(c * c for c in q))
    return [r5(c / n) for c in q]


def points(solid):
    if solid.get("kind") == "loft":
        return [s["at"] for s in solid["sections"]]
    if solid.get("kind") == "capsule":
        return [solid["p0"], solid["p1"]]
    if solid.get("kind") in ("ellipsoid", "box"):
        return [solid.setdefault("transform", {}).setdefault("translation", [0, 0, 0])]
    raise SystemExit("kind %s of %s is not handled" % (solid.get("kind"), solid["id"]))


def rebind(solid, old, new, bone):
    turn = new[1].T @ old[1]
    for p in points(solid):
        p[:] = [r5(c) for c in new[1].T @ (old[1] @ Vec(p) + old[0] - new[0])]
    q = quat(turn)
    if solid["kind"] == "loft":
        for s in solid["sections"]:
            s["rotation"] = qmul(q, s.get("rotation", [0, 0, 0, 1]))
    elif solid["kind"] in ("ellipsoid", "box"):
        solid["transform"]["rotation"] = qmul(q, solid["transform"].get("rotation", [0, 0, 0, 1]))
    solid["bone"] = bone
    if "member" in solid:
        solid["member"] = bone


def lateral(solid, law):
    for p in points(solid):
        p[2] = r5(math.copysign(law(abs(p[2])), p[2]))


def lower(solid):
    for p in points(solid):
        p[0] = r5(p[0] - ACROMION_DROP * max(0, min(1, (abs(p[2]) - 0.02) / 0.1)))


def lateral_bone(parent, start, end):
    po, pb = parent
    origin, tip = po + pb @ Vec(start), po + pb @ Vec(end)
    u = tip - origin
    length = math.sqrt(sum(c * c for c in u))
    u = u * (1 / length)
    w = Vec([-1, 0, 0])
    w = w - u * sum(a * b for a, b in zip(w, u))
    w = w * (1 / math.sqrt(sum(c * c for c in w)))
    v = Vec([w[1] * u[2] - w[2] * u[1], w[2] * u[0] - w[0] * u[2], w[0] * u[1] - w[1] * u[0]])
    return {**solve(parent, origin, Mat([[u[i], v[i], w[i]] for i in range(3)])), "lengthM": r5(length)}


def shoulder_law(a):
    return a if a <= 0.09 else 0.09 + (a - 0.09) * 0.944


def hip_law(a):
    return a * 0.073 / 0.081 if a <= 0.081 else 0.073 + (a - 0.081) * 0.069 / 0.061


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def member_end(text, start):
    depth, index, quoted = 0, start, False
    while index < len(text):
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    raise SystemExit("unbalanced asset file")


def splice(text, key, value):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', text)
    return text[: found.end()] + dump(value, 2) + text[member_end(text, found.end()) :]


def mirror(placement):
    return {**placement, "z": -placement["z"], "abductionDeg": -placement["abductionDeg"], "twistDeg": -placement["twistDeg"]}


def main():
    text = open(CATALOG, encoding="utf-8").read()
    doc = json.loads(text)
    asset = doc["asset"]
    bones = asset["rig"]["bones"]
    by = {b["id"]: b for b in bones}
    if "clavicle-l" in by:
        raise SystemExit("the rig already has clavicle-l: this chisel ran")
    old = frames(bones)
    CLAVICLE = lateral_bone(old["torso"], STERNAL_END, ACROMIAL_END)
    print("[DEBUG] clavicle-l placement", CLAVICLE)

    by["torso"].update({"parent": "lumbar", "x": LUMBAR, "lengthM": r5(by["torso"]["lengthM"] - LUMBAR), "title": {"en": "Thorax", "de": "Brustkorb"}, "joint": {"type": "revolute", "minDeg": -25, "maxDeg": 15}})
    by["torso"]["mass"]["comX"] = r5(by["torso"]["mass"]["comX"] - LUMBAR)
    by["neck"]["x"] = by["torso"]["lengthM"]
    lumbar = {"id": "lumbar", "parent": "pelvis", "title": {"en": "Lumbar spine", "de": "Lendenwirbelsäule"}, "x": by["pelvis"]["lengthM"], "y": 0, "angleDeg": 0, "lengthM": LUMBAR, "joint": {"type": "revolute", "minDeg": -50, "maxDeg": 25}}
    extra = {"lumbar": lumbar}
    for side, sign in (("l", 1), ("r", -1)):
        name = {"l": ("Left", "Linkes"), "r": ("Right", "Rechtes")}[side]
        c = CLAVICLE if sign == 1 else mirror(CLAVICLE)
        extra["clavicle-" + side] = {"id": "clavicle-" + side, "parent": "torso", "title": {"en": name[0] + " clavicle and scapula", "de": name[1] + " Schlüsselbein mit Schulterblatt"}, "x": r5(c["x"] - LUMBAR), "y": c["y"], "z": c["z"], "angleDeg": c["angleDeg"], "abductionDeg": c["abductionDeg"], "twistDeg": c["twistDeg"], "lengthM": c["lengthM"], "joint": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40 if sign == 1 else -5, "twistMaxDeg": 5 if sign == 1 else 40}}
        i = ILIUM if sign == 1 else mirror(ILIUM)
        extra["ilium-" + side] = {"id": "ilium-" + side, "parent": "pelvis", "title": {"en": name[0] + " hip bone", "de": name[1] + " Hüftbein"}, "x": i["x"], "y": i["y"], "z": i["z"], "angleDeg": i["angleDeg"], "abductionDeg": i["abductionDeg"], "twistDeg": i["twistDeg"], "lengthM": i["lengthM"], "joint": {"type": "revolute", "minDeg": -3, "maxDeg": 3}}
    order = ["pelvis", "lumbar", "torso", "neck", "head"]
    for side in ("l", "r"):
        order += ["clavicle-" + side, "arm-" + side, "forearm-" + side, "hand-" + side, "ilium-" + side, "thigh-" + side, "shank-" + side, "foot-" + side, "toe-" + side]
    order += ["crutch-l", "crutch-r"]
    by.update(extra)
    bones[:] = [by[i] for i in order]
    for limb, girdle, half in (("arm", "clavicle", SHOULDER_HALF), ("thigh", "ilium", HIP_HALF)):
        for side, sign in (("l", 1), ("r", -1)):
            now = frames([b for b in bones if b["id"] in ("pelvis", "lumbar", "torso", girdle + "-" + side)])
            origin, basis = old[limb + "-" + side]
            target = Vec([origin[0], origin[1], sign * half])
            bone = by[limb + "-" + side]
            bone.pop("z", None)
            bone.update({"parent": girdle + "-" + side, **solve(now[girdle + "-" + side], target, basis)})
    new = frames(bones)
    for i, (o, b) in old.items():
        if new[i][1].diff(b) > 2e-5:
            raise SystemExit("rest frame of %s turned" % i)
    tip = new["clavicle-l"][0] + new["clavicle-l"][1].col(0) * CLAVICLE["lengthM"]
    print("[DEBUG] clavicle-l", [round(c, 4) for c in new["clavicle-l"][0]], "tip", [round(c, 4) for c in tip], "arm-l", [round(c, 4) for c in new["arm-l"][0]], "thigh-l", [round(c, 4) for c in new["thigh-l"][0]], "ilium-l tip", [round(c, 4) for c in new["ilium-l"][0] + new["ilium-l"][1].col(0) * ILIUM["lengthM"]])

    asset["dimensions"]["depthM"] = 0.668
    old["lumbar"] = old["torso"]
    for scale, volume in asset["volumes"].items():
        fine = {s["id"]: s for s in asset["volumes"]["50"]["structure"]}
        for name in ("body", "thermal", "structure"):
            rows = volume[name]
            for solid in list(rows):
                ident, bone = solid["id"], solid.get("bone")
                side = ident[-1] if ident[-2:] in ("-l", "-r") else ""
                if name == "structure" and bone == "torso" and re.match(r"^(clavicle|scapula|scapula-spine|glenoid)-[lr]$", ident):
                    if ident.startswith("glenoid"):
                        t = solid["transform"]["translation"]
                        t[2] = math.copysign(0.157, t[2])
                    else:
                        lateral(solid, shoulder_law)
                        if not ident.startswith("scapula-l") and not ident.startswith("scapula-r"):
                            lower(solid)
                    rebind(solid, old["torso"], new["clavicle-" + side], "clavicle-" + side)
                elif name == "structure" and bone == "pelvis" and re.match(r"^(ilium|acetabulum|pubis|ischium|ischiopubic-ramus)-[lr]$", ident):
                    lateral(solid, hip_law)
                    rebind(solid, old["pelvis"], new["ilium-" + side], "ilium-" + side)
                elif bone == "torso" and re.match(r"^(vertebra|spinous|transverse)-l[1-5]$", ident):
                    solid["bone"] = "lumbar"
                    solid["member"] = "lumbar"
                elif bone == "torso":
                    for p in points(solid):
                        p[0] = r5(p[0] - LUMBAR)
                elif bone in ("arm-l", "arm-r") and solid.get("kind") == "loft" and name != "structure":
                    for s in solid["sections"]:
                        if s["at"][0] < 0:
                            s["at"][0] = r5(s["at"][0] * SLEEVE_HEAD)
                            s["at"][2] = r5(s["at"][2] * 0.5)
                if name == "structure" and scale in ("200", "500"):
                    stem = ident[:-2]
                    if stem in SHAFTS and solid.get("kind") == "loft":
                        for s, radius in zip(solid["sections"], SHAFTS[stem]):
                            s["rx"] = s["ry"] = radius
                        solid["slender"] = True
                    if stem == "radius-ulna":
                        at = rows.index(solid)
                        rows[at : at + 1] = [{**json.loads(json.dumps(fine[b + "-" + side])), "slender": True} for b in ("radius", "ulna")]
                    if ident == "spine":
                        solid["members"] = ["lumbar"]
                    if ident == "pelvis" and scale == "500":
                        solid["members"] = ["ilium-l", "ilium-r"]
                if name == "structure" and scale == "100" and re.match(r"^(radius|ulna)-[lr]$", ident):
                    solid.update({"r0": fine[ident]["r0"], "r1": fine[ident]["r1"], "slender": True})
        volume["decisions"].append({
            "en": "Rig: the arms hang from clavicle bones that run from the sternoclavicular to the acromioclavicular joint (shoulder joints 0.366 m apart, acromia 0.40 m), the legs from hip bones that run from the sacroiliac joint to the hip joint (0.176 m apart), and the trunk bends in a lumbar bone below the thorax; clavicle, scapula and hip bone solids are bound to these bones.",
            "de": "Rig: Die Arme hängen an Schlüsselbeinknochen vom Sternoklavikular- zum Akromioklavikulargelenk (Schultergelenke 0,366 m auseinander, Akromien 0,40 m), die Beine an Hüftbeinen vom Iliosakralgelenk zum Hüftgelenk (0,176 m auseinander), und der Rumpf beugt sich in einem Lendenknochen unter dem Brustkorb; Schlüsselbein, Schulterblatt und Hüftbein sind an diese Knochen gebunden.",
        })

    arm, leg = SHOULDER_HALF - 0.2013, HIP_HALF - 0.0962
    moved = {}
    for side, sign in (("l", 1), ("r", -1)):
        for b in ("arm", "forearm", "hand", "crutch"):
            moved[b + "-" + side] = sign * arm
        for b in ("thigh", "shank", "foot", "toe"):
            moved[b + "-" + side] = sign * leg
    shoulder = float(old["arm-l"][0][1])
    for representation in asset["representations"].values():
        geometry = representation.get("geometry", {})
        for shape in geometry.get("elevation", []):
            for p in shape["points"]:
                if shape["bone"] == "torso":
                    p[0] = round(p[0] - LUMBAR, 4)
                elif shape["bone"] in ("arm-l", "arm-r") and p[0] < 0:
                    p[0] = round(p[0] * SLEEVE_HEAD, 4)
        for shape in geometry.get("front", []):
            for p in shape["points"]:
                p[0] = round(p[0] + moved.get(shape["bone"], 0), 4)
                if shape["bone"] in ("arm-l", "arm-r") and p[1] > shoulder:
                    p[1] = round(shoulder + (p[1] - shoulder) * SLEEVE_HEAD, 4)
        for shape in geometry.get("plan", []):
            for p in shape["points"]:
                p[1] = round(p[1] + moved.get(shape["bone"], 0), 4)

    def offsets(old_offsets):
        out = {}
        for bone in order:
            if bone == "lumbar":
                out[bone] = dict(old_offsets["torso"])
            elif bone == "torso":
                out[bone] = {"angleDeg": 0}
            elif bone in old_offsets:
                out[bone] = old_offsets[bone]
            elif bone[:-2] in ("clavicle", "ilium"):
                out[bone] = {"angleDeg": 0}
        return out

    for pose in asset["poses"]:
        pose["bones"] = offsets(pose["bones"])
    for clip in asset["clips"]:
        for key in clip["keys"]:
            if "torso" in key.get("bones", {}):
                key["bones"] = offsets(key["bones"])

    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    open(CANDIDATE, "w", encoding="utf-8").write(dump(doc, 0) + "\n")
    print("candidate", CANDIDATE)
    if "--publish" in sys.argv:
        if open(CATALOG, encoding="utf-8").read() != text:
            raise SystemExit("catalog asset changed meanwhile; retry")
        result = text
        for key in ("dimensions", "rig", "representations", "poses", "clips", "volumes"):
            result = splice(result, key, asset[key])
        if json.loads(result) != doc:
            raise SystemExit("splice did not round-trip")
        open(SOURCE, "w", encoding="utf-8").write(dump({"volumes": asset["volumes"]}, 0) + "\n")
        open(CATALOG, "w", encoding="utf-8").write(result)
        print("published", CATALOG)


if __name__ == "__main__":
    main()
