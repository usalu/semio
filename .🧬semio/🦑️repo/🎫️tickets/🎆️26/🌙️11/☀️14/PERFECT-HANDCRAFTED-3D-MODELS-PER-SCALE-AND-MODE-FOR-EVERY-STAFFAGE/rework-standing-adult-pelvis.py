"""🦴 standing-adult, second round: the pelvic girdle on the lateral rest placement and a hand-modelled pelvis skeleton per scale.

Rig: `ilium-l/-r` run from the sacroiliac joint to the hip-joint centre; the thighs hang from them without moving. Structure: the hip bones (iliac blade with crest, auricular part, acetabulum, ischium with tuberosity, superior pubic ramus, pubic body, ischiopubic ramus, symphysis) are modelled in world rest coordinates below and written in the frame of their bone; the coarse scales get their own typified pelvis and the declarations the feature rule asks for. All numbers are hand-chosen anatomical landmarks relative to the hip-joint centre (0, 0.903, ±0.085).

Usage: python3 rework-standing-adult-pelvis.py            writes the candidate `🗑️generated/rework-standing-adult/cand/🔣️.json`
       python3 rework-standing-adult-pelvis.py --apply    writes the volumes source and the members `rig` and `structure` of the catalog file (publish the volumes with `volumes-insert.py` afterwards)
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
rig = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rig)
CANDIDATE = os.path.join(T, "🗑️generated", "rework-standing-adult", "cand", "🔣️.json")
SACROILIAC = (-0.056, 0.975, 0.052)
HIP = (0.0, 0.903, 0.085)
BONE = {"material": "bone", "role": "body", "tone": 0.85}


def unit(a):
    n = math.sqrt(sum(c * c for c in a))
    return [c / n for c in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def side(p, s):
    return [p[0], p[1], p[2] * s]


class Frame:
    def __init__(self, frames, bone):
        self.bone = bone
        self.origin, self.basis = frames[bone]
        self.inverse = rig.transpose(self.basis)

    def point(self, p):
        return rig.r5(rig.apply(self.inverse, [p[i] - self.origin[i] for i in range(3)]))

    def turn(self, ex, ey):
        ex = unit(ex)
        k = sum(ex[i] * ey[i] for i in range(3))
        ey = unit([ey[i] - k * ex[i] for i in range(3)])
        ez = cross(ex, ey)
        return rig.r5(rig.quaternion(rig.mul(self.inverse, [[ex[i], ey[i], ez[i]] for i in range(3)])))

    def section(self, at, across, tangent, rx, ry, exponent=2.0):
        tangent = unit(tangent)
        k = sum(across[i] * tangent[i] for i in range(3))
        ex = unit([across[i] - k * tangent[i] for i in range(3)])
        return {"at": self.point(at), "rx": rx, "ry": ry, "exponent": exponent, "rotation": self.turn(ex, cross(tangent, ex))}

    def loft(self, ident, sections, member=None, **more):
        return {"id": ident, "kind": "loft", "bone": self.bone, **BONE, "sections": sections, "capStart": True, "capEnd": True, "member": member or self.bone, "part": "member", **more}

    def tube(self, ident, points, radii, member=None, **more):
        sections = []
        for i, p in enumerate(points):
            a, b = points[max(i - 1, 0)], points[min(i + 1, len(points) - 1)]
            tangent = [b[k] - a[k] for k in range(3)]
            across = cross([0, 0, 1], tangent) if abs(unit(tangent)[2]) < 0.9 else [1, 0, 0]
            sections.append(self.section(p, across, tangent, radii[i], radii[i]))
        return self.loft(ident, sections, member, **more)

    def capsule(self, ident, p0, p1, r0, r1, part="member", **more):
        return {"id": ident, "kind": "capsule", "bone": self.bone, **BONE, "p0": self.point(p0), "p1": self.point(p1), "r0": r0, "r1": r1, "member": self.bone, "part": part, **more}

    def ellipsoid(self, ident, at, ax, ay, radii, part="member", tone=0.85, **more):
        return {"id": ident, "kind": "ellipsoid", "bone": self.bone, "material": "bone", "role": "body", "tone": tone, "radii": radii, "transform": {"translation": self.point(at), "rotation": self.turn(ax, ay)}, "member": self.bone, "part": part, **more}


def blade(frame, ident, s, rows, exponent=2.6):
    sections = []
    for i, (at, rx, ry) in enumerate(rows):
        a, b = rows[max(i - 1, 0)][0], rows[min(i + 1, len(rows) - 1)][0]
        sections.append(frame.section(side(at, s), [1, 0, 0], side([b[k] - a[k] for k in range(3)], s), rx, ry, exponent))
    return frame.loft(ident, sections)


def wing(frame, ident, s, rows):
    sections = []
    for i, (at, rx, ry) in enumerate(rows):
        a, b = rows[max(i - 1, 0)][0], rows[min(i + 1, len(rows) - 1)][0]
        sections.append(frame.section(side(at, s), side([-0.47, 0, -0.88], s), side([b[k] - a[k] for k in range(3)], s), rx, ry, 2.6))
    return frame.loft(ident, sections)


def hip_fine(frames, tag, s, scale):
    f = Frame(frames, "ilium-" + tag)
    p = lambda x, y, z: side((x, y, z), s)
    thin = 0.006 if scale == 50 else 0.0078
    out = [
        f.ellipsoid("joint-sacroiliac-" + tag, p(*SACROILIAC), [1, 0, 0], [0, 1, 0], [0.017, 0.022, 0.015], part="joint", tone=0.4),
        blade(f, "ilium-" + tag, s, [((-0.004, 0.922, 0.086), 0.026, 0.016), ((-0.006, 0.945, 0.094), 0.036, 0.011), ((-0.008, 0.970, 0.105), 0.052, max(thin, 0.007)), ((-0.010, 0.995, 0.117), 0.064, thin), ((-0.012, 1.018, 0.127), 0.066, thin), ((-0.014, 1.034, 0.133), 0.056, 0.008)]),
        f.capsule("ilium-auricular-" + tag, p(*SACROILIAC), p(-0.052, 0.992, 0.100), 0.015, 0.012),
        wing(f, "ilium-posterior-" + tag, s, [((-0.047, 0.962, 0.076), 0.026, 0.009), ((-0.056, 0.988, 0.083), 0.036, thin), ((-0.063, 1.012, 0.091), 0.035, thin), ((-0.062, 1.030, 0.104), 0.022, 0.0075)]),
        f.ellipsoid("acetabulum-" + tag, p(-0.002, 0.905, 0.070), [1, 0, 0], [0, 1, 0], [0.030, 0.031, 0.020]),
        f.capsule("ischium-" + tag, p(-0.012, 0.882, 0.070), p(-0.038, 0.822, 0.058), 0.014, 0.017),
        f.capsule("pubic-ramus-" + tag, p(0.022, 0.905, 0.062), p(0.056, 0.892, 0.014), 0.009, 0.010),
        f.ellipsoid("pubic-body-" + tag, p(0.055, 0.880, 0.0125), [1, 0, 0], [0, 1, 0], [0.010, 0.020, 0.011]),
        f.capsule("ischial-ramus-" + tag, p(0.048, 0.864, 0.016), p(-0.028, 0.826, 0.050), 0.0078, 0.0078),
    ]
    if scale == 50:
        crest = [p(0.050, 0.982, 0.117), p(0.022, 1.022, 0.134), p(-0.015, 1.040, 0.136), p(-0.055, 1.028, 0.118), p(-0.080, 0.990, 0.066)]
        out += [f.capsule("iliac-crest-%s-%d" % (tag, i + 1), crest[i], crest[i + 1], 0.0075, 0.0075) for i in range(4)]
        out.append(f.ellipsoid("iliac-spine-inferior-" + tag, p(0.030, 0.945, 0.098), [1, 0, 0], [0, 1, 0], [0.009, 0.011, 0.008]))
    return out


def hip_200(frames, tag, s):
    f = Frame(frames, "ilium-" + tag)
    p = lambda x, y, z: side((x, y, z), s)
    arc = [p(*SACROILIAC), p(-0.030, 0.945, 0.068), p(-0.002, 0.908, 0.074), p(0.028, 0.897, 0.046), p(0.052, 0.886, 0.016)]
    wing = f.ellipsoid("iliac-wing-" + tag, p(-0.012, 0.985, 0.110), [1, 0, 0], side([0, 0.92, 0.39], s), [0.060, 0.052, 0.015])
    return [f.tube("hip-bone-" + tag, arc, [0.015] * 5), wing]


def pelvic_girdle_500(frames):
    f = Frame(frames, "pelvis")
    rows = [((0.004, 0.850, 0), 0.040, 0.062), ((0.002, 0.895, 0), 0.055, 0.108), ((-0.006, 0.950, 0), 0.060, 0.128), ((-0.016, 1.000, 0), 0.056, 0.136), ((-0.020, 1.036, 0), 0.046, 0.130)]
    sections = [f.section(at, [1, 0, 0], [0, 1, 0], rx, ry, 2.4) for at, rx, ry in rows]
    return f.loft("pelvic-girdle", sections, members=["ilium-l", "ilium-r"])


def spine_500(frames, old):
    f = Frame(frames, "torso")
    points, radii = [], []
    for ident, bone in (("lumbar-spine", "lumbar"), ("thoracic-spine", "torso"), ("cervical-spine", "neck")):
        origin, basis = frames[bone]
        solid = next(x for x in old if x["id"] == ident)
        for section in solid["sections"]:
            r = max(section["rx"], section["ry"])
            if r < 0.009:
                continue
            points.append([origin[i] + rig.apply(basis, section["at"])[i] for i in range(3)])
            radii.append(r)
    order = sorted(range(len(points)), key=lambda i: points[i][1])
    keep = [order[0]] + [order[i] for i in range(2, len(order) - 2, 2)] + [order[-1]]
    return f.tube("spine", [points[i] for i in keep], [radii[i] for i in keep], members=["lumbar", "neck"], slender=True)


def foot_500(frames, tag, s):
    f = Frame(frames, "foot-" + tag)
    rows = [((-0.050, 0.039, 0.085), 0.0375, 0.0375), ((0.030, 0.042, 0.085), 0.040, 0.040), ((0.110, 0.039, 0.085), 0.0375, 0.043), ((0.165, 0.038, 0.085), 0.0375, 0.040)]
    return f.loft("foot-bones-" + tag, [f.section(side(at, s), [0, 1, 0], [1, 0, 0], ry, rz, 2.6) for at, ry, rz in rows])


def thicken(solid, least):
    for section in solid["sections"]:
        section["rx"], section["ry"] = max(section["rx"], least), max(section["ry"], least)


def convert(asset, volumes):
    bones = asset["rig"]["bones"]
    if any(b["id"] == "ilium-l" for b in bones):
        return False
    before = rig.frames(bones)
    titles = {"l": {"en": "Left hip bone (pelvic girdle)", "de": "Linkes Hüftbein (Beckengürtel)"}, "r": {"en": "Right hip bone (pelvic girdle)", "de": "Rechtes Hüftbein (Beckengürtel)"}}
    for tag, s in (("l", 1), ("r", -1)):
        origin, tip = side(SACROILIAC, s), side(HIP, s)
        u = [tip[i] - origin[i] for i in range(3)]
        placed = rig.placed(before["pelvis"], origin, u, (-1.0 * s, 0.0, 0.0))
        bone = {"id": "ilium-" + tag, "parent": "pelvis", "title": titles[tag], **placed, "lengthM": round(math.sqrt(sum(c * c for c in u)), 5), "joint": {"type": "revolute", "minDeg": -4, "maxDeg": 4}}
        bones.insert(next(i for i, b in enumerate(bones) if b["id"] == "thigh-" + tag), bone)
        frame = rig.frames(bones)["ilium-" + tag]
        thigh = next(b for b in bones if b["id"] == "thigh-" + tag)
        thigh["parent"] = "ilium-" + tag
        thigh.update(rig.fields(frame, before["thigh-" + tag][0], before["thigh-" + tag][1]))
    frames = rig.frames(bones)
    for tag in ("l", "r"):
        for i in range(3):
            assert abs(frames["thigh-" + tag][0][i] - before["thigh-" + tag][0][i]) < 2e-5
            assert all(abs(frames["thigh-" + tag][1][i][j] - before["thigh-" + tag][1][i][j]) < 2e-5 for j in range(3))
    asset["structure"]["supports"] = ["heel-l", "ball-l", "heel-r", "ball-r"]
    gone = ("ilium-", "ischium-", "pubic-ramus-", "ischial-ramus-", "joint-sacroiliac-", "hip-bone-")
    for scale in (50, 100, 200, 500):
        old = volumes[str(scale)]["structure"]
        new = [x for x in old if not x["id"].startswith(gone)]
        at = next(i for i, x in enumerate(new) if x["id"] == "sacrum") + 1
        pelvis = Frame(frames, "pelvis")
        if scale in (50, 100):
            added = hip_fine(frames, "l", 1, scale) + hip_fine(frames, "r", -1, scale)
            if scale == 50:
                added.append(pelvis.ellipsoid("pubic-symphysis", (0.055, 0.880, 0), [1, 0, 0], [0, 1, 0], [0.010, 0.019, 0.004], part="joint", tone=0.4))
        elif scale == 200:
            added = hip_200(frames, "l", 1) + hip_200(frames, "r", -1)
            for x in new:
                if x["id"] in ("cervical-spine", "sacrum") or x["id"].startswith(("hand-bones-", "foot-bones-")):
                    thicken(x, 0.015)
                if x["id"] == "thoracic-spine" or x["id"].startswith(("humerus-", "forearm-bones-", "femur-", "tibia-")):
                    x["slender"] = True
        else:
            spine = spine_500(frames, old)
            drop = ("sacrum", "cervical-spine", "thoracic-spine", "lumbar-spine", "hand-bones-l", "hand-bones-r", "calcaneus-l", "calcaneus-r", "foot-bones-l", "foot-bones-r")
            new = [x for x in new if x["id"] not in drop]
            at = 1
            added = [spine, pelvic_girdle_500(frames), foot_500(frames, "l", 1), foot_500(frames, "r", -1)]
            for x in new:
                if x["id"].startswith(("humerus-", "femur-", "tibia-")):
                    x["slender"] = True
                if x["id"].startswith("forearm-bones-"):
                    tag = x["id"][-1]
                    x["slender"] = True
                    x["members"] = ["hand-" + tag]
                    last = x["sections"][-1]
                    x["sections"][-2]["at"][0] = 0.300
                    last["at"][0] = 0.308
        volumes[str(scale)]["structure"] = new[:at] + added + new[at:]
    return True


def main():
    source_path = rig.SOURCE
    if "--apply" not in sys.argv:
        document = json.load(open(rig.CATALOG, encoding="utf8"))
        document["asset"]["volumes"] = json.load(open(source_path, encoding="utf8"))["volumes"]
        if not convert(document["asset"], document["asset"]["volumes"]):
            raise SystemExit("already converted")
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        for bone in document["asset"]["rig"]["bones"]:
            if bone["id"][:-2] in ("ilium", "thigh"):
                print(json.dumps({k: v for k, v in bone.items() if k not in ("title", "mass")}))
        print(CANDIDATE)
        return
    original = text = open(rig.CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    source_text = open(source_path, encoding="utf8").read()
    source = json.loads(source_text)
    if not convert(document["asset"], source["volumes"]):
        raise SystemExit("already converted")
    for key in ("rig", "structure"):
        text = rig.splice(text, key, before[key], document["asset"][key])
    if json.loads(text) != document:
        raise SystemExit("splice did not round-trip")
    if open(rig.CATALOG, encoding="utf8").read() != original or open(source_path, encoding="utf8").read() != source_text:
        raise SystemExit("a file changed while converting: run again")
    open(rig.CATALOG, "w", encoding="utf8").write(text)
    json.dump(source, open(source_path, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(source_path, "a", encoding="utf8").write("\n")
    print("applied to", rig.CATALOG, "and", source_path)


if __name__ == "__main__":
    main()
