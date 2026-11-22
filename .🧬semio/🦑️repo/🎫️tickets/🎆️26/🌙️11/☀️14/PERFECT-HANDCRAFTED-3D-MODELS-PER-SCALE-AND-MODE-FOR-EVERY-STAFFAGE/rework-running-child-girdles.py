"""🦴️ Running child: shoulder and pelvic girdle on the lateral rest placement, stated by hand for a six-year-old (1.125 m barefoot).

Usage: python3 rework-running-child-girdles.py            writes the candidate `🗑️generated/rework-running-child/running-child/🔣️.json`
       python3 rework-running-child-girdles.py --apply    writes the volumes source and the members `rig`, `poses`, `clips` of the catalog file

Rig: `clavicle-l/-r` (sternoclavicular to acromioclavicular joint) under `torso`, `ilium-l/-r` (sacroiliac joint to hip joint) under `pelvis`; arms and
thighs are re-parented without moving. Structure: every bone of the two girdles is authored here in the frame of the trunk bone (u up, v back, w left)
and written in the frame of its girdle bone. Idempotent: run it after `rework-02-running-child-structure.py`, which still writes the old stubs.
See `rig-lateral-rest.md`.
"""
import copy
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(T, *[".."] * 7))
CATALOG = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "running-child", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "running-child.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-running-child", "running-child", "🔣️.json")

SC, AC, GH = (0.260, -0.037, 0.013), (0.254, 0.006, 0.102), (0.2255, 0.0, 0.105)
SI, HIP = (0.056, 0.026, 0.024), (0.0, 0.0, 0.055)
BACK = (0.0, 1.0, 0.0)
FLOOR = {50: 0.004, 100: 0.0075, 200: 0.0155, 500: 0.0376}
SHRUG = {"jump-takeoff": 15, "jump-apex": 12}
OLD = ("sk-clavicle", "sk-scapula", "sk-scapular-spine", "sk-glenoid", "sk-acromion", "sk-coracoid", "sk-acetabulum", "sk-iliac-crest", "sk-ilium", "sk-sacroiliac", "sk-pubis", "sk-ischium", "sk-ischiopubic-ramus", "sk-hip", "joint-clavicle", "joint-ilium")
TITLES = {
    "clavicle-l": ("Left clavicle", "Linkes Schlüsselbein"), "clavicle-r": ("Right clavicle", "Rechtes Schlüsselbein"),
    "ilium-l": ("Left hip bone", "Linkes Hüftbein"), "ilium-r": ("Right hip bone", "Rechtes Hüftbein"),
}


def sub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def unit(a):
    n = math.sqrt(dot(a, a))
    return tuple(x / n for x in a)


def columns(*cols):
    return [[cols[c][r] for c in range(3)] for r in range(3)]


def column(m, c):
    return tuple(m[r][c] for r in range(3))


def mul(a, b):
    return [[sum(a[r][k] * b[k][c] for k in range(3)) for c in range(3)] for r in range(3)]


def transpose(m):
    return [[m[c][r] for c in range(3)] for r in range(3)]


def apply(m, p):
    return tuple(sum(m[r][k] * p[k] for k in range(3)) for r in range(3))


def turn(angle, abduction=0.0, twist=0.0):
    """🧭️ Rest or offset rotation of a bone: about w, then its own v (u towards w), then its own u (v towards -w)."""
    a, b, t = (math.radians(x) for x in (angle, abduction, twist))
    rz = [[math.cos(a), -math.sin(a), 0], [math.sin(a), math.cos(a), 0], [0, 0, 1]]
    ry = [[math.cos(b), 0, -math.sin(b)], [0, 1, 0], [math.sin(b), 0, math.cos(b)]]
    rx = [[1, 0, 0], [0, math.cos(t), math.sin(t)], [0, -math.sin(t), math.cos(t)]]
    return mul(mul(rz, ry), rx)


def angles(m):
    """📐️ The three angles of a rotation in the order of `turn`."""
    u, w = column(m, 0), column(m, 2)
    b = math.asin(max(-1.0, min(1.0, u[2])))
    a = math.atan2(u[1], u[0])
    v0 = (-math.sin(a), math.cos(a), 0.0)
    w0 = cross(u, v0)
    t = math.atan2(dot(w, v0), dot(w, w0))
    return tuple(round(math.degrees(x) + 0.0, 4) for x in (a, b, t))


def frame(origin, tip):
    """🦴️ Frame of a girdle bone in its parent: u joint to joint, joint axis w towards the back, so that a positive angle lifts the lateral end."""
    u = unit(sub(tip, origin))
    w = unit(sub(BACK, tuple(dot(BACK, u) * c for c in u)))
    return columns(u, cross(w, u), w)


def quaternion(m):
    tr = m[0][0] + m[1][1] + m[2][2]
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        q = ((m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, 0.25 * s)
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1.0 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = (0.25 * s, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s)
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1.0 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = ((m[0][1] + m[1][0]) / s, 0.25 * s, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s)
    else:
        s = math.sqrt(1.0 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = ((m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, 0.25 * s, (m[1][0] - m[0][1]) / s)
    return [round(c + 0.0, 5) for c in q]


def r5(p):
    return [round(c + 0.0, 5) for c in p]


class Girdle:
    """✍️ Writes solids stated in the parent frame into the frame of one girdle bone of the left side."""

    def __init__(self, bone, origin, tip):
        self.bone, self.origin, self.m = bone, origin, frame(origin, tip)
        self.inverse = transpose(self.m)
        self.length = math.sqrt(dot(sub(tip, origin), sub(tip, origin)))

    def point(self, p):
        return r5(apply(self.inverse, sub(p, self.origin)))

    def axes(self, ex, ey):
        ex = unit(ex)
        ey = unit(sub(ey, tuple(dot(ey, ex) * c for c in ex)))
        return quaternion(mul(self.inverse, columns(ex, ey, cross(ex, ey))))

    def base(self, id, kind):
        return {"id": id + "-l", "kind": kind, "bone": self.bone, "material": "bone", "role": "body", "tone": 0.85}

    def capsule(self, id, p0, p1, r0, r1=None):
        return {**self.base(id, "capsule"), "p0": self.point(p0), "p1": self.point(p1), "r0": r0, "r1": r0 if r1 is None else r1, "member": self.bone, "part": "member"}

    def chain(self, id, points, radii):
        return [self.capsule(f"{id}-{i + 1}", points[i], points[i + 1], radii[i], radii[i + 1]) for i in range(len(points) - 1)]

    def loft(self, id, ex, ey, rows, exponent=2.0):
        sections = [{"at": self.point(c), "rx": rx, "ry": ry, "exponent": exponent, "rotation": self.axes(ex, ey)} for c, rx, ry in rows]
        return {**self.base(id, "loft"), "sections": sections, "capStart": True, "capEnd": True, "member": self.bone, "part": "member"}

    def ellipsoid(self, id, at, e1, e2, radii, part="member", tone=0.85):
        solid = {**self.base(id, "ellipsoid"), "radii": list(radii), "transform": {"translation": self.point(at), "rotation": self.axes(e1, e2)}, "member": self.bone, "part": part}
        solid["tone"] = tone
        return solid


def mirrored(solid):
    """🪞️ The right twin: the right girdle frame is the mirror image of the left one with its joint axis reversed, so local z changes sign."""
    s = copy.deepcopy(solid)
    for key in ("id", "bone", "member"):
        s[key] = s[key][:-2] + "-r" if s[key].endswith("-l") else s[key].replace("-l-", "-r-")
    for key in ("p0", "p1"):
        if key in s:
            s[key][2] = round(-s[key][2] + 0.0, 5)
    for holder in s.get("sections", []) + ([s["transform"]] if "transform" in s else []):
        for key in ("at", "translation"):
            if key in holder:
                holder[key][2] = round(-holder[key][2] + 0.0, 5)
        q = holder["rotation"]
        holder["rotation"] = [round(-q[0] + 0.0, 5), round(-q[1] + 0.0, 5), q[2], q[3]]
    return s


def shoulder(sc):
    """🦴️ Clavicle, scapula with spine, acromion, coracoid and glenoid of the left side, in the trunk frame (up, back, left)."""
    g, f = Girdle("clavicle-l", SC, AC), FLOOR[sc]
    up, wide, normal = (1, 0, 0), (0, -0.45, 0.89), (0, 0.89, 0.45)
    inferior, root, angle, tip, socket = (0.143, 0.050, 0.052), (0.214, 0.052, 0.038), (0.247, 0.026, 0.106), (0.252, -0.004, 0.114), (0.2255, 0.004, 0.0915)
    if sc == 50:
        path = [SC, (0.262, -0.043, 0.036), (0.2615, -0.038, 0.055), (0.260, -0.022, 0.078), (0.257, -0.004, 0.092), AC]
        out = g.chain("sk-clavicle", path, [0.0062, 0.0048, 0.0042, 0.0042, 0.0046, 0.005])
        out.append(g.loft("sk-scapula", wide, normal, [(inferior, 0.006, f), ((0.165, 0.049, 0.056), 0.014, f), ((0.190, 0.046, 0.060), 0.022, f), ((0.215, 0.040, 0.064), 0.029, f), ((0.236, 0.036, 0.063), 0.026, f)]))
        out.append(g.capsule("sk-scapular-spine", root, angle, 0.0042, 0.0048))
        out.append(g.capsule("sk-acromion", angle, tip, 0.005))
        out.append(g.capsule("sk-scapula-neck", (0.218, 0.028, 0.078), (0.2255, 0.008, 0.089), 0.0062))
        out.append(g.ellipsoid("sk-glenoid", socket, up, (0, 1, 0), (0.0115, 0.0085, 0.0042)))
        out.append(g.capsule("sk-coracoid", (0.236, 0.006, 0.088), (0.237, -0.024, 0.084), f))
    elif sc == 100:
        out = g.chain("sk-clavicle", [SC, (0.2615, -0.040, 0.055), AC], [f, f, f])
        out.append(g.loft("sk-scapula", wide, normal, [(inferior, 0.008, f), ((0.190, 0.046, 0.060), 0.022, f), ((0.215, 0.040, 0.064), 0.029, f), ((0.236, 0.036, 0.063), 0.026, f)]))
        out.append(g.capsule("sk-scapular-spine", root, (0.251, 0.008, 0.108), f))
        out.append(g.capsule("sk-scapula-neck", (0.218, 0.028, 0.078), (0.2255, 0.008, 0.089), f))
        out.append(g.ellipsoid("sk-glenoid", socket, up, (0, 1, 0), (0.0115, 0.0085, f)))
    else:
        out = [g.capsule("sk-clavicle", SC, AC, f), g.capsule("sk-scapula", (0.150, 0.040, 0.055), AC, f), g.capsule("sk-glenoid", AC, GH, f)]
    out.append(g.ellipsoid("joint-clavicle", SC, up, (0, 1, 0), JOINT[sc], "joint", 0.5))
    return out


def hip(sc):
    """🦴️ Hip bone of the left side in the pelvis frame (up, back, left): ilium with crest, acetabulum, pubis, ischium and their ramus."""
    g, f = Girdle("ilium-l", SI, HIP), FLOOR[sc]
    long, normal = (0, 0.87, -0.5), (0, 0.5, 0.87)
    psis, top, tubercle, asis, tuber, symphysis = (0.054, 0.040, 0.028), (0.086, 0.004, 0.074), (0.080, -0.020, 0.084), (0.062, -0.036, 0.080), (-0.046, 0.018, 0.036), (-0.024, -0.044, 0.006)
    blade = [((0.012, 0.004, 0.050), 0.017, 0.008), ((0.030, 0.004, 0.056), 0.024, 0.006), ((0.050, 0.002, 0.064), 0.034, 0.005), ((0.068, 0.000, 0.071), 0.036, 0.0045), ((0.080, 0.000, 0.075), 0.026, 0.0045)]
    if sc in (50, 100):
        out = [g.loft("sk-ilium", long, normal, [(c, rx, max(ry, f)) for c, rx, ry in (blade if sc == 50 else blade[::2])], 2.2)]
        if sc == 50:
            out += g.chain("sk-iliac-crest", [psis, (0.078, 0.028, 0.050), top, tubercle, asis], [0.0052, 0.0046, 0.0046, 0.005, 0.0052])
        else:
            out += g.chain("sk-iliac-crest", [psis, top, asis], [f, f, f])
        out.append(g.capsule("sk-sacroiliac", (0.056, 0.026, 0.022), (0.064, 0.022, 0.050), max(0.007, f)))
        out.append(g.ellipsoid("sk-acetabulum", (0.0, 0.0, 0.049), (1, 0, 0), (0, 1, 0), (0.019, 0.019, 0.011)))
        out.append(g.capsule("sk-pubis", (-0.002, -0.014, 0.046), symphysis, max(0.0055, f)))
        out.append(g.capsule("sk-ischium", (-0.008, 0.008, 0.048), tuber, max(0.007, f), max(0.009, f)))
        out.append(g.capsule("sk-ischiopubic-ramus", tuber, (-0.032, -0.040, 0.007), max(0.0045, f)))
    else:
        out = [g.capsule("sk-hip", SI, HIP, max(0.0192, f), max(0.0166, f))]
    out.append(g.ellipsoid("joint-ilium", SI, (1, 0, 0), (0, 1, 0), JOINT[sc], "joint", 0.5))
    return out


PARENT = {"arm": "torso", "thigh": "pelvis"}
REST = {"arm": (GH[0], GH[2]), "thigh": (HIP[0], HIP[2])}
JOINT = {50: (0.00627, 0.00896, 0.00896), 100: (0.0078, 0.00896, 0.00896), 200: (0.0155, 0.0155, 0.0155), 500: (0.0376, 0.0376, 0.0376)}
STRAP = {50: (0.1865, -0.083, 0.055), 100: (0.1865, -0.085, 0.055), 200: (0.183, -0.062, 0.055), 500: (0.183, -0.062, 0.055)}


def structure(solids, sc):
    kept = [s for s in solids if not any(s["id"] == o or s["id"].startswith(o + "-") for o in OLD)]
    for s in kept:
        if s["id"] in ("sk-strap-l", "sk-strap-r"):
            x, y, z = STRAP[sc]
            s["p1"] = [x, y, z if s["id"].endswith("-l") else -z]
    new = shoulder(sc) + hip(sc)
    new += [mirrored(s) for s in new]
    at = next(i for i, s in enumerate(kept) if s.get("part") == "mass" or s["bone"] == "pack")
    return kept[:at] + new + kept[at:]


def placed(parent_frame, origin_in_parent, child_origin, child_rotation):
    """📌️ Fields of a child that keeps its place in the grandparent frame when it is hung from the girdle bone."""
    inverse = transpose(parent_frame)
    x, y, z = r5(apply(inverse, sub(child_origin, origin_in_parent)))
    a, b, t = angles(mul(inverse, child_rotation))
    return {"x": x, "y": y, "z": z, "angleDeg": a, "abductionDeg": b, "twistDeg": t}


def rig(bones):
    if any(b["id"] == "clavicle-l" for b in bones):
        bones = [{**{k: v for k, v in b.items() if k not in ("abductionDeg", "twistDeg")}, "parent": PARENT[b["id"][:-2]], "x": REST[b["id"][:-2]][0], "y": 0, "z": REST[b["id"][:-2]][1] * (1 if b["id"][-1] == "l" else -1), "angleDeg": 180} if b["id"][:-2] in PARENT else b for b in bones if b["id"][:-2] not in ("clavicle", "ilium")]
    out = []
    limb = turn(180)
    for bone in bones:
        side = bone["id"][-1]
        s = 1 if side == "l" else -1
        for limb_id, girdle_id, origin, tip, limits in (("arm", "clavicle", SC, AC, (-8, 30)), ("thigh", "ilium", SI, HIP, (-4, 4))):
            if bone["id"] != f"{limb_id}-{side}":
                continue
            m = frame(origin, tip)
            a, b, t = angles(m)
            en, de = TITLES[f"{girdle_id}-{side}"]
            length = round(math.sqrt(dot(sub(tip, origin), sub(tip, origin))), 5)
            out.append({"id": f"{girdle_id}-{side}", "parent": bone["parent"], "title": {"en": en, "de": de}, "x": origin[0], "y": origin[1], "z": s * origin[2], "angleDeg": a, "abductionDeg": s * b, "twistDeg": s * t, "lengthM": length, "joint": {"type": "revolute", "minDeg": limits[0], "maxDeg": limits[1]}})
            f = placed(m, origin, (bone["x"], bone["y"], abs(bone["z"])), limb)
            bone = {**{k: v for k, v in bone.items() if k != "z"}, "joint": {**bone["joint"], "maxDeg": 180} if limb_id == "arm" else bone["joint"], "parent": f"{girdle_id}-{side}", "x": f["x"], "y": f["y"], "z": s * f["z"], "angleDeg": f["angleDeg"], "abductionDeg": s * f["abductionDeg"], "twistDeg": s * f["twistDeg"]}
        out.append(bone)
    return out


def shrug(bones, lift):
    """🤷️ Lifts both clavicles by `lift` degrees and re-solves the arm offsets so that the arms keep the direction the pose gave them."""
    m = frame(SC, AC)
    rel = mul(transpose(m), turn(180))
    for side, s in (("l", 1), ("r", -1)):
        arm = bones.get(f"arm-{side}", {})
        if "abductionDeg" in arm or "twistDeg" in arm:
            continue
        off = mul(mul(mul(transpose(rel), transpose(turn(lift))), rel), turn(arm.get("angleDeg", 0)))
        a, b, t = angles(off)
        bones[f"clavicle-{side}"] = {"angleDeg": lift}
        bones[f"arm-{side}"] = {**arm, "angleDeg": a, "abductionDeg": round(s * b, 4), "twistDeg": round(s * t, 4)}


def unshrug(bones, stored):
    """↩️ Takes a shrug written for the stored rig out again: the arm offsets become the plain flexion the pose had before."""
    rest = {b["id"]: turn(b["angleDeg"], b.get("abductionDeg", 0), b.get("twistDeg", 0)) for b in stored}
    for side in ("l", "r"):
        lift = bones.pop(f"clavicle-{side}", {}).get("angleDeg", 0)
        arm = bones.get(f"arm-{side}")
        if arm and ("abductionDeg" in arm or "twistDeg" in arm):
            rel = rest[f"arm-{side}"]
            off = mul(mul(mul(transpose(rel), turn(lift)), rel), turn(arm["angleDeg"], arm.get("abductionDeg", 0), arm.get("twistDeg", 0)))
            a, b, t = angles(off)
            if abs(b) > 0.05 or abs(t) > 0.05:
                raise SystemExit(f"[DEBUG] arm-{side} offset is not a plain flexion after taking the shrug out: {a} {b} {t}")
            bones[f"arm-{side}"] = {k: v for k, v in {**arm, "angleDeg": a}.items() if k not in ("abductionDeg", "twistDeg")}


def convert(asset, volumes):
    stored = asset["rig"]["bones"]
    if os.environ.get("STORED_AC"):
        ac = tuple(float(c) for c in os.environ["STORED_AC"].split(","))
        m = frame(SC, ac)
        a, b, t = angles(m)
        f = placed(m, SC, GH, turn(180))
        stored = [{"id": "clavicle-l", "angleDeg": a, "abductionDeg": b, "twistDeg": t}, {"id": "arm-l", **f}, {"id": "arm-r", **{**f, "abductionDeg": -f["abductionDeg"], "twistDeg": -f["twistDeg"]}}]
    holders = [pose["bones"] for pose in asset["poses"] if pose["id"] in SHRUG]
    holders += [key["bones"] for clip in asset["clips"] for key in clip["keys"] if key.get("pose") in SHRUG and "arm-l" in key.get("bones", {})]
    lifts = [SHRUG[pose["id"]] for pose in asset["poses"] if pose["id"] in SHRUG]
    lifts += [SHRUG[key["pose"]] for clip in asset["clips"] for key in clip["keys"] if key.get("pose") in SHRUG and "arm-l" in key.get("bones", {})]
    for bones in holders:
        unshrug(bones, stored)
    asset["rig"]["bones"] = rig(stored if not os.environ.get("STORED_AC") else asset["rig"]["bones"])
    for bones, lift in zip(holders, lifts):
        shrug(bones, lift)
    for sc in (50, 100, 200, 500):
        volumes[str(sc)]["structure"] = structure(volumes[str(sc)]["structure"], sc)


def splice(text, key, before, after, dump):
    for layout in (lambda v: dump(v, 2), lambda v: json.dumps(v, ensure_ascii=False, indent=2).replace("\n", "\n    ")):
        old = '    ' + json.dumps(key) + ": " + layout(before)
        if text.count(old) == 1:
            return text.replace(old, '    ' + json.dumps(key) + ": " + layout(after))
    raise SystemExit(f"[DEBUG] member {key} not found in its stored layout")


def main():
    document = json.load(open(CATALOG, encoding="utf8"))
    source = json.load(open(SOURCE, encoding="utf8"))
    before = copy.deepcopy(document["asset"])
    convert(document["asset"], source["volumes"])
    for sc in ("50", "100", "200", "500"):
        print(f"[DEBUG] 1:{sc} structure {len(source['volumes'][sc]['structure'])}")
    for bone in document["asset"]["rig"]["bones"]:
        if bone["id"][:-2] in ("clavicle", "ilium", "arm", "thigh"):
            print("[DEBUG]", json.dumps({k: v for k, v in bone.items() if k not in ("title", "mass")}))
    if "--apply" not in sys.argv:
        candidate = copy.deepcopy(document)
        candidate["asset"]["volumes"] = source["volumes"]
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(candidate, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        print("[DEBUG] candidate", CANDIDATE)
        return
    scope = {}
    code = open(os.path.join(T, "volumes-insert.py"), encoding="utf8").read()
    exec(compile(code[: code.index("\ndef main(")], "volumes-insert.py", "exec"), scope)
    text = open(CATALOG, encoding="utf8").read()
    for key in ("rig", "poses", "clips"):
        if before[key] != document["asset"][key]:
            text = splice(text, key, before[key], document["asset"][key], scope["dump"])
    open(CATALOG, "w", encoding="utf8").write(text)
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("[DEBUG] applied")


if __name__ == "__main__":
    main()
