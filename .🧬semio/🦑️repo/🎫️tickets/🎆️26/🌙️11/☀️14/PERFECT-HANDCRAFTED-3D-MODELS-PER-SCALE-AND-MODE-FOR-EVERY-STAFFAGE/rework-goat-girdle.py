"""🦴 Chisel of the goat author: shoulder and pelvic girdle of `animals/goat` on the lateral rest placement.

The scapula runs from its dorsal border 40 mm beside the withers down, forward and outward to the shoulder joint
75 mm beside the plane; the humerus hangs from its tip. Each hip bone runs from the sacroiliac joint 30 mm beside
the plane back, down and outward to the hip joint 70 mm beside it; the femur hangs from its tip. Limbs keep their
world rest frames, so their solids, drawings, poses and clips stay as they are; solids bound to the scapula are
re-expressed in its new frame and the hip bones move from the pelvis onto their own bones.

Usage: python3 rework-goat-girdle.py            writes the candidate document for checking
       python3 rework-goat-girdle.py --apply    writes the volumes source and the rig of the catalog asset
"""
import json
import math
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes/animals/goat.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/goat/🔣️.json"
CANDIDATE = os.path.join(HERE, "🗑️generated/r3-goat/🔣️.json")
SCAPULA_OUT = 0.035
SACROILIAC = (0.112, 0.01, 0.03)
ILIUM_RADIUS = {"100": 0.009, "200": 0.0155, "500": 0.0378}


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def tr(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def app(a, p):
    return [sum(a[i][k] * p[k] for k in range(3)) for i in range(3)]


def rw(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def rv(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, 0, -s], [0, 1, 0], [s, 0, c]]


def ru(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[1, 0, 0], [0, c, s], [0, -s, c]]


def rest(bone):
    return mul(mul(rw(bone["angleDeg"]), rv(bone.get("abductionDeg", 0))), ru(bone.get("twistDeg", 0)))


def frames(bones):
    by, out = {b["id"]: b for b in bones}, {}

    def frame(ident):
        if ident not in out:
            bone = by[ident]
            if bone["parent"] is None:
                out[ident] = ([bone["x"], bone["y"], bone.get("z", 0)], rest(bone))
            else:
                origin, basis = frame(bone["parent"])
                moved = app(basis, [bone["x"], bone["y"], bone.get("z", 0)])
                out[ident] = ([origin[i] + moved[i] for i in range(3)], mul(basis, rest(bone)))
        return out[ident]

    for bone in bones:
        frame(bone["id"])
    return out


def place(bone, parent, world):
    origin, basis = world
    p_origin, p_basis = parent
    local = app(tr(p_basis), [origin[i] - p_origin[i] for i in range(3)])
    m = mul(tr(p_basis), basis)
    abduction = math.degrees(math.asin(max(-1, min(1, m[2][0]))))
    angle = math.degrees(math.atan2(m[1][0], m[0][0]))
    n = mul(tr(mul(rw(angle), rv(abduction))), m)
    twist = math.degrees(math.atan2(-n[2][1], n[1][1]))
    bone["x"], bone["y"], bone["z"] = round(local[0], 5), round(local[1], 5), round(local[2], 5)
    bone["angleDeg"] = round(angle, 4)
    for key, value in (("abductionDeg", abduction), ("twistDeg", twist)):
        if abs(value) > 1e-6:
            bone[key] = round(value, 4)
        else:
            bone.pop(key, None)


def quat(m):
    w = math.sqrt(max(0, 1 + m[0][0] + m[1][1] + m[2][2])) / 2
    if w > 1e-6:
        return [(m[2][1] - m[1][2]) / (4 * w), (m[0][2] - m[2][0]) / (4 * w), (m[1][0] - m[0][1]) / (4 * w), w]
    x = math.sqrt(max(0, 1 + m[0][0] - m[1][1] - m[2][2])) / 2
    y = math.sqrt(max(0, 1 - m[0][0] + m[1][1] - m[2][2])) / 2
    z = math.sqrt(max(0, 1 - m[0][0] - m[1][1] + m[2][2])) / 2
    return [x, math.copysign(y, m[0][1] + m[1][0]), math.copysign(z, m[0][2] + m[2][0]), 0]


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def rebind(solid, old, new):
    (o_old, b_old), (o_new, b_new) = old, new
    turn = mul(tr(b_new), b_old)
    q = quat(turn)

    def point(p):
        world = app(b_old, p)
        return [round(c, 5) for c in app(tr(b_new), [world[i] + o_old[i] - o_new[i] for i in range(3)])]

    def spin(r):
        return [round(c, 8) for c in qmul(q, r)]

    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = point(solid["p0"]), point(solid["p1"])
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = point(section["at"])
            section["rotation"] = spin(section["rotation"])
    else:
        raise SystemExit("no rebind for " + solid["kind"])


SPANS = {"shoulder": (0.035, 0.118), "arm": (0.053, 0.123), "forearm": (0.055, 0.116), "cannon-f": (0.055, 0.095)}


def sweep(representations):
    for scale in ("50", "100"):
        for shape in representations[scale]["geometry"]["elevation"]:
            name, _, side = shape["id"].rpartition("-")
            if name in SPANS and side in ("n", "f") and shape.get("filled") and "solid" not in shape:
                near, far = SPANS[name]
                shape["solid"] = {"kind": "extruded", "sweep": "w", "z0": -far if side == "n" else near, "z1": -near if side == "n" else far}


def main():
    document = json.load(open(CATALOG, encoding="utf-8"))
    sweep(document["asset"]["representations"])
    source = json.load(open(SOURCE, encoding="utf-8"))
    bones = document["asset"]["rig"]["bones"]
    by = {b["id"]: b for b in bones}
    if "abductionDeg" in by["scapula-fn"]:
        raise SystemExit("girdle already applied")
    old = frames(bones)
    for side, sign in (("n", -1), ("f", 1)):
        blade = by["scapula-f" + side]
        length = blade["lengthM"]
        blade["lengthM"] = round(math.hypot(length, SCAPULA_OUT), 5)
        blade["abductionDeg"] = round(sign * math.degrees(math.asin(SCAPULA_OUT / blade["lengthM"])), 4)
        ilium = {"id": "ilium-" + side, "parent": "pelvis", "title": {"en": ("Near" if side == "n" else "Far") + " hip bone (sacroiliac joint to hip joint)", "de": ("Nahes" if side == "n" else "Fernes") + " Hüftbein (Kreuzdarmbeingelenk bis Hüftgelenk)"}, "x": SACROILIAC[0], "y": SACROILIAC[1], "z": sign * SACROILIAC[2]}
        reach = [-SACROILIAC[0], -SACROILIAC[1], sign * (0.07 - SACROILIAC[2])]
        ilium["lengthM"] = round(math.sqrt(sum(c * c for c in reach)), 5)
        ilium["angleDeg"] = round(math.degrees(math.atan2(reach[1], reach[0])), 4)
        ilium["abductionDeg"] = round(math.degrees(math.asin(reach[2] / ilium["lengthM"])), 4)
        ilium["joint"] = {"type": "revolute", "minDeg": -2, "maxDeg": 2}
        bones.insert(bones.index(by["thigh-h" + side]), ilium)
        by[ilium["id"]] = ilium
        by["thigh-h" + side]["parent"] = ilium["id"]
    new = frames([b for b in bones if b["id"] in ("pelvis", "lumbar", "thorax", "scapula-fn", "scapula-ff", "ilium-n", "ilium-f")])
    for side in ("n", "f"):
        place(by["humerus-f" + side], new["scapula-f" + side], old["humerus-f" + side])
        place(by["thigh-h" + side], new["ilium-" + side], old["thigh-h" + side])
    check = frames(bones)
    worst = max(abs(check[i][0][k] - old[i][0][k]) for i in old if not i.startswith("scapula") for k in range(3))
    print("[DEBUG] largest origin move of an untouched bone %.6f m" % worst, file=sys.stderr)
    for scale, volume in source["volumes"].items():
        for listing in ("body", "thermal", "structure"):
            for solid in volume[listing]:
                bone = solid.get("bone", "")
                if bone.startswith("scapula-f") and "kind" in solid:
                    rebind(solid, old[bone], new[bone])
        structure = volume["structure"]
        for side in ("n", "f"):
            hip = next((s for s in structure if s.get("id") == "hip-bone-" + side), None)
            if hip is None:
                after = next(i for i, s in enumerate(structure) if s.get("id") == "sacral-wings")
                hip = {"id": "hip-bone-" + side, "kind": "capsule", "bone": "pelvis", "material": "bone", "role": "body", "tone": 0.9, "p0": [SACROILIAC[0], SACROILIAC[1], (-1 if side == "n" else 1) * SACROILIAC[2]], "p1": [0, 0, -0.07 if side == "n" else 0.07], "r0": ILIUM_RADIUS[scale], "r1": ILIUM_RADIUS[scale], "part": "member"}
                structure.insert(after + 1, hip)
            rebind(hip, old["pelvis"], new["ilium-" + side])
            hip["bone"] = hip["member"] = "ilium-" + side
    if "--apply" in sys.argv:
        with open(SOURCE, "w", encoding="utf-8") as handle:
            json.dump(source, handle, ensure_ascii=False, indent=1)
            handle.write("\n")
        text = open(CATALOG, encoding="utf-8").read()
        indent = re.search(r'\n( +)"id"\s*:', text).group(1)
        found = re.search(r'\n' + indent + r'"rig"\s*:\s*', text)
        depth, index, quoted = 0, found.end(), False
        while True:
            char = text[index]
            if quoted:
                index += char == "\\"
                quoted = char != '"'
            elif char == '"':
                quoted = True
            elif char in "{[":
                depth += 1
            elif char in "}]":
                depth -= 1
                if depth == 0:
                    break
            index += 1
        rows = ",\n".join(indent + "    " + json.dumps(b, ensure_ascii=False) for b in bones)
        rig = '{\n%s  "kind": %s,\n%s  "root": %s,\n%s  "bones": [\n%s\n%s  ]\n%s}' % (indent, json.dumps(document["asset"]["rig"]["kind"]), indent, json.dumps(document["asset"]["rig"]["root"]), indent, rows, indent, indent)
        result = text[: found.end()] + rig + text[index + 1:]
        if json.loads(result)["asset"]["rig"] != document["asset"]["rig"] or open(CATALOG, encoding="utf-8").read() != text:
            raise SystemExit("rig splice failed or the asset changed; retry")
        open(CATALOG, "w", encoding="utf-8").write(result)
        scope = {"__file__": os.path.join(HERE, "rework-07-2d.py")}
        exec(compile(open(scope["__file__"], encoding="utf-8").read().rsplit("\nmain()", 1)[0], scope["__file__"], "exec"), scope)
        scope["splice"](CATALOG, "representations", lambda representations, asset: sweep(representations))
        print("[DEBUG] girdle applied", file=sys.stderr)
    else:
        document["asset"]["volumes"] = source["volumes"]
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        with open(CANDIDATE, "w", encoding="utf-8") as handle:
            json.dump(document, handle, ensure_ascii=False)
        print(CANDIDATE)


main()
