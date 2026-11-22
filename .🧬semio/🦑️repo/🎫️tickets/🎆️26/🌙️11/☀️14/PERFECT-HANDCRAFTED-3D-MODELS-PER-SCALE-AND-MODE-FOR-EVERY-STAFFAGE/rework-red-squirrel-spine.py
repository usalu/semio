"""🦴 Red squirrel: the trunk gets its lumbar spine as a bone of its own (pelvis with sacrum | L3-L7 | thorax with L1-L2).

The lumbosacral joint lies at the column (x 0.0385, y 0.0146 on the pelvis bone). Every trunk solid, drawing and member that
crossed it is cut there by hand-set numbers; the thorax bone keeps its rest frame. Poses with free hands move 40 % of the
trunk flexion into the lumbar joint.

Usage: python3 rework-red-squirrel-spine.py candidate|publish
"""
import copy
import json
import math
import re
import sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/red-squirrel/🔣️.json"
SOURCE = "volumes/animals/red-squirrel.json"
CANDIDATE = "🗑️generated/r3-red-squirrel/🔣️.json"
JOINT_X, JOINT_Y, LENGTH = 0.0385, 0.0146, 0.0335
OVERLAP, TUCK = 0.008, 0.9
FLEXION = {"running": -15, "bound-gather": -15, "bound-reach": -15, "bound-push": -5, "bound-flight": -2}
SPLIT = {"rump": "loin", "belly-patch-rump": "belly-patch-loin", "th-rump-underfur": "th-loin-underfur", "th-belly-patch-rump-underfur": "th-belly-patch-loin-underfur", "rump-core": "loin-core"}


def between(a, b, x):
    t = (x - a["at"][0]) / (b["at"][0] - a["at"][0])
    out = copy.deepcopy(a)
    out["at"] = [round(a["at"][i] + t * (b["at"][i] - a["at"][i]), 5) for i in range(3)]
    out["rx"], out["ry"] = round(a["rx"] + t * (b["rx"] - a["rx"]), 5), round(a["ry"] + t * (b["ry"] - a["ry"]), 5)
    return out


def section_at(sections, x):
    for a, b in zip(sections, sections[1:]):
        if a["at"][0] <= x <= b["at"][0]:
            return between(a, b, x)
    raise SystemExit(f"no section at {x}")


def tucked(section, overlap):
    out = copy.deepcopy(section)
    if overlap:
        out["rx"], out["ry"] = round(out["rx"] * TUCK, 5), round(out["ry"] * TUCK, 5)
    return out


def moved(section):
    out = copy.deepcopy(section)
    out["at"] = [round(out["at"][0] - JOINT_X, 5), round(out["at"][1] - JOINT_Y, 5), out["at"][2]]
    return out


def cut_loft(solid, new_id, overlap):
    sections = solid["sections"]
    rear = [s for s in sections if s["at"][0] < JOINT_X - 1e-6] + [section_at(sections, JOINT_X)]
    fore = [section_at(sections, JOINT_X)] + [s for s in sections if s["at"][0] > JOINT_X + 1e-6]
    if overlap:
        rear.append(tucked(section_at(sections, JOINT_X + overlap), overlap))
        fore.insert(0, tucked(section_at(sections, JOINT_X - overlap), overlap))
    front = copy.deepcopy(solid)
    front["id"], front["bone"], front["sections"] = new_id, "lumbar", [moved(s) for s in fore]
    if "member" in front:
        front["member"] = "lumbar"
    solid["sections"] = rear
    return front


def clip(points, keep_rear, x):
    out = []
    for a, b in zip(points, points[1:] + points[:1]):
        ina, inb = (a[0] <= x) == keep_rear or a[0] == x, (b[0] <= x) == keep_rear or b[0] == x
        if ina:
            out.append(a)
        if ina != inb:
            t = (x - a[0]) / (b[0] - a[0])
            out.append([round(x, 5), round(a[1] + t * (b[1] - a[1]), 5)])
    return out


def rig(asset):
    bones = asset["rig"]["bones"]
    if any(b["id"] == "lumbar" for b in bones):
        raise SystemExit("lumbar bone already there")
    pelvis, thorax = bones[0], bones[1]
    lumbar = copy.deepcopy(thorax)
    lumbar.update(id="lumbar", parent="spine-r", title={"en": "Lumbar spine, L3 to L7", "de": "Lendenwirbelsäule, L3 bis L7"}, x=JOINT_X, y=JOINT_Y, angleDeg=0, lengthM=LENGTH)
    lumbar["joint"] = dict(thorax["joint"], minDeg=-40, maxDeg=40)
    lumbar["mass"] = dict(pelvis["mass"], kg=0.010, comX=0.017, comY=-JOINT_Y)
    pelvis["title"] = {"en": "Pelvis with sacrum", "de": "Becken mit Kreuzbein"}
    pelvis["lengthM"] = JOINT_X
    pelvis["mass"] = dict(pelvis["mass"], kg=0.014, comX=0.019)
    thorax.update(parent="lumbar", x=LENGTH, y=-JOINT_Y)
    thorax["title"] = {"en": "Thoracic spine with chest and L1 to L2", "de": "Brustwirbelsäule mit Brustkorb und L1 bis L2"}
    bones.insert(1, lumbar)


def poses(asset):
    for pose in asset["poses"]:
        share = FLEXION.get(pose["id"])
        if share is None:
            continue
        bones = {}
        for bone, offset in pose["bones"].items():
            if bone == "spine-f":
                bones["lumbar"] = {"angleDeg": share}
                offset = dict(offset, angleDeg=offset["angleDeg"] - share)
            bones[bone] = offset
        pose["bones"] = bones


def turn_w(m, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    u, v, w = m
    return [[c * u[i] + s * v[i] for i in range(3)], [c * v[i] - s * u[i] for i in range(3)], w]


def turn_v(m, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    u, v, w = m
    return [[c * u[i] + s * w[i] for i in range(3)], v, [c * w[i] - s * u[i] for i in range(3)]]


def turn_u(m, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    u, v, w = m
    return [u, [c * v[i] - s * w[i] for i in range(3)], [c * w[i] + s * v[i] for i in range(3)]]


def placed(bone, offset=None):
    """🧭 Axes (u, v, w as rows) and origin of a bone in its parent's frame, rest placement then pose offset."""
    m = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    for step in (bone, offset or {}):
        m = turn_u(turn_v(turn_w(m, step.get("angleDeg", 0)), step.get("abductionDeg", 0)), step.get("twistDeg", 0))
    return m, [bone.get("x", 0), bone.get("y", 0), bone.get("z", 0)]


def out_of(m, o, p):
    return [o[i] + sum(m[k][i] * p[k] for k in range(3)) for i in range(3)]


def collarbones(asset):
    """🦴 The collarbone follows the acromion wherever a pose turns the shoulder blade (both hang on the thorax)."""
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    blade, strut = bones["scapula-n"], bones["clavicle-n"]
    for side in "nf":
        bones["clavicle-" + side]["joint"].update(minDeg=-50, maxDeg=50)
    sm, so = placed(blade)
    cm, co = placed(strut)
    tip = out_of(cm, co, [strut["lengthM"], 0, 0])
    local = [sum(sm[k][i] * (tip[i] - so[i]) for i in range(3)) for k in range(3)]
    for pose in asset["poses"]:
        for side in "nf":
            turned = pose["bones"].get("scapula-" + side)
            if not turned:
                continue
            pm, po = placed(blade, turned)
            goal = out_of(pm, po, local)
            best = (1, 0, 0)
            span, centre = 60, (0, 0)
            for _ in range(6):
                for i in range(-10, 11):
                    for j in range(-10, 11):
                        a, b = centre[0] + i * span / 10, centre[1] + j * span / 10
                        m, o = placed(strut, {"angleDeg": a, "abductionDeg": b})
                        end = out_of(m, o, [strut["lengthM"], 0, 0])
                        gap = math.dist(end, goal)
                        if gap < best[0]:
                            best = (gap, a, b)
                centre, span = best[1:], span / 5
            pose["bones"]["clavicle-" + side] = {"angleDeg": round(best[1], 1), "abductionDeg": round(best[2] * (1 if side == "n" else -1), 1)}
            print("[DEBUG]", pose["id"], side, "collarbone", pose["bones"]["clavicle-" + side], "left gap mm", round(best[0] * 1000, 2))


def drawings(asset):
    for scale, representation in asset["representations"].items():
        shapes = representation.get("geometry", {}).get("elevation", [])
        out = []
        for shape in shapes:
            out.append(shape)
            if shape.get("bone") == "spine-r" and shape["id"] in SPLIT and shape.get("kind") == "poly":
                front = copy.deepcopy(shape)
                front["id"], front["bone"], front["z"] = SPLIT[shape["id"]], "lumbar", round(shape["z"] + 0.01, 3)
                front["points"] = [[round(p[0] - JOINT_X, 5), round(p[1] - JOINT_Y, 5)] for p in clip(shape["points"], False, JOINT_X - 0.004)]
                shape["points"] = clip(shape["points"], True, JOINT_X + 0.004)
                out.append(front)
        if shapes:
            representation["geometry"]["elevation"] = out


def volumes(all_volumes):
    for scale, volume in all_volumes.items():
        for name in ("body", "thermal"):
            out = []
            for solid in volume[name]:
                out.append(solid)
                if solid.get("bone") == "spine-r" and solid["id"] in SPLIT:
                    out.append(cut_loft(solid, SPLIT[solid["id"]], OVERLAP))
                elif solid.get("of") in SPLIT:
                    twin = copy.deepcopy(solid)
                    twin["id"], twin["of"] = solid["id"].replace("rump", "loin"), SPLIT[solid["of"]]
                    if twin["id"] == solid["id"]:
                        raise SystemExit("no new id for " + solid["id"])
                    out.append(twin)
            volume[name] = out
        out = []
        for solid in volume["structure"]:
            out.append(solid)
            if solid["id"] == "m-spine-r":
                if scale in ("50", "100"):
                    out.append(cut_loft(solid, "m-lumbar", 0))
                else:
                    solid["members"] = ["lumbar"]
            elif solid["id"] == "spinous-processes-r":
                solid.update(id="spinous-processes-lumbar", bone="lumbar", member="lumbar")
                solid["sections"] = [moved(s) for s in solid["sections"]]
            elif solid["id"] == "j-spine-f":
                joint = copy.deepcopy(solid)
                joint.update(id="j-lumbar", bone="lumbar", member="lumbar")
                joint["transform"] = {"translation": [0, 0, 0]}
                out.append(joint)
        volume["structure"] = out


def member_end(text, start):
    depth, index, quoted = 0, start, False
    while True:
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


def spliced(text, key, value):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', text)
    return text[: found.end()] + json.dumps(value, ensure_ascii=False, indent=2).replace("\n", "\n    ") + text[member_end(text, found.end()):]


def main():
    text = open(CATALOG, encoding="utf-8").read()
    document = json.loads(text)
    plain = json.dumps(document, ensure_ascii=False, indent=2) + "\n" == text
    print("[DEBUG] catalog text is plain indent 2:", plain)
    asset = document["asset"]
    source = json.load(open(SOURCE, encoding="utf-8"))
    if source["volumes"] != asset["volumes"]:
        raise SystemExit("source and catalog volumes differ")
    rig(asset)
    poses(asset)
    collarbones(asset)
    drawings(asset)
    volumes(asset["volumes"])
    source["volumes"] = asset["volumes"]
    if sys.argv[1] == "candidate":
        json.dump(document, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        return
    result = text
    for key in ("rig", "poses", "representations"):
        result = spliced(result, key, asset[key])
    check = json.loads(result)
    check["asset"]["volumes"] = asset["volumes"]
    if check != document:
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf-8").read() != text:
        raise SystemExit("catalog changed while writing; retry")
    json.dump(source, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    open(CATALOG, "w", encoding="utf-8").write(result)
    print("published rig, poses and drawings; run volumes-insert.py for the volumes")


main()
