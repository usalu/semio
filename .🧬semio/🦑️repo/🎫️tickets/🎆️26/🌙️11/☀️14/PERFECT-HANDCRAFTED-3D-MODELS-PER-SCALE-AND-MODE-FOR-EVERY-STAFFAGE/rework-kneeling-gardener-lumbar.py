"""🐛 kneeling-gardener: the back bends in two places. A `lumbar` bone (L5/S1 to T12/L1, 0.16 m) is put between `pelvis` and `torso`; `torso` becomes the rib cage (T12/L1 to C7/T1, 0.296 m).

Everything bound to `torso` moves 0.16 m down its axis so that nothing moves in the rest pose. The shirt, its air layer and the spine are cut at the thoracolumbar joint into a waist part on `lumbar` and a chest part on `torso`; both run past the joint (the waist 40 mm up and 1.5 mm inside the chest, the chest 8 mm down and 3 mm inside the waist) so the back stays closed when it rounds. The lumbar spinous processes go to `lumbar`. Every pose gives 60 % of its former trunk bend to the lumbar joint and bends the rib cage until the shoulders are at their former height; the neck takes the difference back so the gaze stays, and the arms are solved again so that wrists and hands stay where they were.

Usage: python3 rework-kneeling-gardener-lumbar.py            candidate document
       python3 rework-kneeling-gardener-lumbar.py --apply    catalog file (rig, poses, anchors, representations) and volumes source (publish with volumes-insert.py)
"""
import copy
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "kneeling-gardener", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "kneeling-gardener.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-kneeling-gardener", "lumbar", "🔣️.json")

LUMBAR = 0.16
WAIST_UP = 0.04
CHEST_DOWN = 0.008
INSIDE = 0.0015
CHEST_INSIDE = 0.003
KEEP = 0.008
LUMBAR_SHARE = 0.6
PELVIS_LENGTH = 0.0943
SHOULDER_ALONG_TORSO = 0.387
UPPER_ARM = 0.2925
FOREARM = 0.2546
MASS_LUMBAR = 9.376
MASS_CHEST = 9.888
SPLIT = {"shirt": "shirt-waist", "sk-spine": "sk-spine-lumbar"}
LUMBAR_SPINOUS = {"sk-spinous-%d" % i for i in range(1, 6)}


def r5(value):
    return round(value, 5)


def section_at(sections, u, inside=0.0):
    ordered = sorted(sections, key=lambda s: s["at"][0])
    below = [s for s in ordered if s["at"][0] <= u]
    above = [s for s in ordered if s["at"][0] > u]
    a, b = (below[-1], above[0]) if below and above else ((ordered[0], ordered[1]) if not below else (ordered[-2], ordered[-1]))
    t = (u - a["at"][0]) / (b["at"][0] - a["at"][0])
    t = min(1.0, max(0.0, t))
    made = copy.deepcopy(a)
    made["at"] = [r5(u), r5(a["at"][1] + t * (b["at"][1] - a["at"][1])), r5(a["at"][2] + t * (b["at"][2] - a["at"][2]))]
    for key in ("rx", "ry", "exponent"):
        if key in a:
            made[key] = r5(a[key] + t * (b.get(key, a[key]) - a[key]) - (inside if key != "exponent" else 0.0))
    return made


def shift_solid(solid):
    kind = solid["kind"]
    if kind == "loft":
        for s in solid["sections"]:
            s["at"][0] = r5(s["at"][0] - LUMBAR)
    elif kind == "capsule":
        solid["p0"][0] = r5(solid["p0"][0] - LUMBAR)
        solid["p1"][0] = r5(solid["p1"][0] - LUMBAR)
    elif kind == "ellipsoid":
        transform = solid.setdefault("transform", {})
        translation = transform.setdefault("translation", [0, 0, 0])
        translation[0] = r5(translation[0] - LUMBAR)
    else:
        raise SystemExit("kind %s of %s on torso is not handled" % (kind, solid["id"]))


def waist_id(identifier):
    if identifier.startswith("th-shirt-"):
        return "th-shirt-waist-" + identifier[len("th-shirt-"):]
    return SPLIT[identifier]


def splits(solid):
    if solid["kind"] != "loft":
        return False
    us = [s["at"][0] for s in solid["sections"]]
    return (solid["id"] in SPLIT or solid["id"].startswith("th-shirt-")) and min(us) < LUMBAR - 0.05 and max(us) > LUMBAR + 0.05


def split_loft(solid):
    spine = solid["id"] == "sk-spine"
    up, down, inside = (0.006, 0.006, 0.0) if spine else (WAIST_UP, CHEST_DOWN, INSIDE)
    ordered = sorted(solid["sections"], key=lambda s: s["at"][0])
    waist = copy.deepcopy(solid)
    waist["id"] = waist_id(solid["id"])
    waist["bone"] = "lumbar"
    waist["sections"] = [copy.deepcopy(s) for s in ordered if s["at"][0] < LUMBAR - KEEP] + [section_at(ordered, LUMBAR), section_at(ordered, LUMBAR + up, inside)]
    if "member" in waist:
        waist["member"] = "lumbar"
    chest = copy.deepcopy(solid)
    chest["sections"] = [section_at(ordered, LUMBAR - down, 0.0 if spine else CHEST_INSIDE), section_at(ordered, LUMBAR)] + [copy.deepcopy(s) for s in ordered if s["at"][0] > LUMBAR + KEEP]
    shift_solid(chest)
    return waist, chest


def mass_radius(kg):
    return r5(0.02198 * (kg / 7.981) ** (1.0 / 3.0))


def rebuild_list(solids, name):
    out = []
    for solid in solids:
        if solid.get("of") == "shirt":
            waist = copy.deepcopy(solid)
            waist["id"] = solid["id"].replace("th-shirt", "th-shirt-waist", 1)
            waist["of"] = "shirt-waist"
            out.extend([waist, solid])
            continue
        if solid.get("bone") != "torso":
            out.append(solid)
            continue
        if splits(solid):
            out.extend(split_loft(solid))
            continue
        if solid["id"] in LUMBAR_SPINOUS:
            solid["bone"] = "lumbar"
            solid["member"] = "lumbar"
            out.append(solid)
            continue
        if solid["id"] == "joint-torso":
            lumbosacral = copy.deepcopy(solid)
            lumbosacral["id"] = "joint-lumbar"
            lumbosacral["bone"] = "lumbar"
            lumbosacral["member"] = "lumbar"
            out.append(lumbosacral)
            solid["transform"]["translation"] = [0, 0.048, 0]
            out.append(solid)
            continue
        if solid["id"] == "mass-torso":
            low = copy.deepcopy(solid)
            low["id"] = "mass-lumbar"
            low["bone"] = "lumbar"
            low["member"] = "lumbar"
            low["radii"] = [mass_radius(MASS_LUMBAR)] * 3
            low["transform"]["translation"] = [0.08, 0, 0]
            out.append(low)
            solid["radii"] = [mass_radius(MASS_CHEST)] * 3
            solid["transform"]["translation"] = [0.1698, 0, 0]
            out.append(solid)
            continue
        shift_solid(solid)
        out.append(solid)
    return out


def rebuild_volumes(volumes):
    for scale in volumes.values():
        for name in ("body", "thermal", "structure"):
            scale[name] = rebuild_list(scale[name], name)


def clip(points, limit, keep_below):
    inside = (lambda p: p[0] <= limit) if keep_below else (lambda p: p[0] >= limit)
    out = []
    for i, p in enumerate(points):
        q = points[(i + 1) % len(points)]
        if inside(p):
            out.append([p[0], p[1]])
        if inside(p) != inside(q):
            t = (limit - p[0]) / (q[0] - p[0])
            out.append([limit, r5(p[1] + t * (q[1] - p[1]))])
    return out


def rebuild_shapes(shapes):
    out = []
    for shape in shapes:
        if shape.get("bone") != "torso":
            out.append(shape)
            continue
        if shape["kind"] != "poly":
            raise SystemExit("2D shape %s on torso is %s" % (shape["id"], shape["kind"]))
        points = shape["points"]
        if shape["id"] == "shirt-hem":
            shape["bone"] = "lumbar"
            out.append(shape)
            continue
        if shape["id"] == "shirt" and shape.get("closed"):
            waist = copy.deepcopy(shape)
            waist["id"] = "shirt-waist"
            waist["bone"] = "lumbar"
            waist["z"] = round(shape["z"] - 0.01, 3)
            waist["points"] = clip(points, LUMBAR + WAIST_UP, True)
            if "solid" in waist:
                waist["solid"].pop("zTip", None)
            out.append(waist)
            shape["points"] = [[r5(p[0] - LUMBAR), p[1]] for p in clip(points, LUMBAR - CHEST_DOWN, False)]
            out.append(shape)
            continue
        if shape["id"] == "shirt-placket":
            ordered = sorted(points, key=lambda p: p[0])
            cut = None
            for a, b in zip(ordered, ordered[1:]):
                if a[0] < LUMBAR <= b[0]:
                    t = (LUMBAR - a[0]) / (b[0] - a[0])
                    cut = [LUMBAR, r5(a[1] + t * (b[1] - a[1]))]
            low = copy.deepcopy(shape)
            low["id"] = "shirt-placket-waist"
            low["bone"] = "lumbar"
            low["points"] = [p for p in ordered if p[0] < LUMBAR] + [cut]
            out.append(low)
            shape["points"] = [[0.0, cut[1]]] + [[r5(p[0] - LUMBAR), p[1]] for p in ordered if p[0] > LUMBAR]
            out.append(shape)
            continue
        shape["points"] = [[r5(p[0] - LUMBAR), p[1]] for p in points]
        out.append(shape)
    return out


def unit(deg):
    return (math.cos(math.radians(deg)), math.sin(math.radians(deg)))


def rebuild_pose(pose):
    bones = pose["bones"]
    bend = bones.get("torso", {}).get("angleDeg", 0.0)
    lumbar = round(LUMBAR_SHARE * bend)
    pelvis = 90.0 + bones.get("pelvis", {}).get("angleDeg", 0.0)
    was = pelvis + bend
    old = tuple(SHOULDER_ALONG_TORSO * c for c in unit(was))
    ul = unit(pelvis + lumbar)
    level = (old[1] - LUMBAR * ul[1]) / (SHOULDER_ALONG_TORSO - LUMBAR)
    chest = round(math.degrees(math.asin(level)) - (pelvis + lumbar), 1)
    trunk = pelvis + lumbar + chest
    ut = unit(trunk)
    new = (LUMBAR * ul[0] + (SHOULDER_ALONG_TORSO - LUMBAR) * ut[0], LUMBAR * ul[1] + (SHOULDER_ALONG_TORSO - LUMBAR) * ut[1])
    rebuilt = {}
    for key, value in bones.items():
        if key == "torso":
            rebuilt["lumbar"] = {"angleDeg": lumbar}
            rebuilt["torso"] = {**value, "angleDeg": chest}
        else:
            rebuilt[key] = value
    if "neck" in rebuilt:
        rebuilt["neck"] = {**rebuilt["neck"], "angleDeg": round(rebuilt["neck"]["angleDeg"] - (trunk - was), 1)}
    for side in ("l", "r"):
        a1 = was - 180.0 + bones["upper-arm-" + side]["angleDeg"]
        a2 = a1 + bones["forearm-" + side]["angleDeg"]
        a3 = a2 + bones["hand-" + side]["angleDeg"]
        u1, u2 = unit(a1), unit(a2)
        wrist = (old[0] + UPPER_ARM * u1[0] + FOREARM * u2[0], old[1] + UPPER_ARM * u1[1] + FOREARM * u2[1])
        dx, dy = wrist[0] - new[0], wrist[1] - new[1]
        reach = min(math.hypot(dx, dy), UPPER_ARM + FOREARM - 1e-6)
        elbow = math.degrees(math.acos((reach * reach - UPPER_ARM ** 2 - FOREARM ** 2) / (2 * UPPER_ARM * FOREARM)))
        inner = math.degrees(math.atan2(FOREARM * math.sin(math.radians(elbow)), UPPER_ARM + FOREARM * math.cos(math.radians(elbow))))
        n1 = math.degrees(math.atan2(dy, dx)) - inner
        n2 = n1 + elbow
        wrap = lambda d: (d + 180.0) % 360.0 - 180.0
        rebuilt["upper-arm-" + side] = {**bones["upper-arm-" + side], "angleDeg": round(wrap(n1 - (trunk - 180.0)), 3)}
        rebuilt["forearm-" + side] = {**bones["forearm-" + side], "angleDeg": round(elbow, 3)}
        rebuilt["hand-" + side] = {**bones["hand-" + side], "angleDeg": round(wrap(a3 - n2), 3)}
    pose["bones"] = rebuilt


def rebuild_rig(rig):
    bones = []
    for bone in rig["bones"]:
        if bone["id"] == "torso":
            bones.append({
                "id": "lumbar", "parent": "pelvis", "title": {"en": "Lumbar spine with abdomen", "de": "Lendenwirbelsäule mit Bauch"},
                "x": PELVIS_LENGTH, "y": 0, "angleDeg": 0, "lengthM": LUMBAR,
                "joint": {"type": "revolute", "minDeg": -50, "maxDeg": 20},
                "mass": {"kg": MASS_LUMBAR, "comX": 0.08, "comY": 0, "distribution": "point", "source": "literature", "reference": "de-leva-1996"},
            })
            bone["parent"] = "lumbar"
            bone["title"] = {"en": "Rib cage with thoracic spine", "de": "Brustkorb mit Brustwirbelsäule"}
            bone["x"] = LUMBAR
            bone["lengthM"] = r5(bone["lengthM"] - LUMBAR)
            bone["joint"] = {"type": "revolute", "minDeg": -45, "maxDeg": 20}
            bone["mass"]["kg"] = MASS_CHEST
            bone["mass"]["comX"] = 0.1698
        elif bone.get("parent") == "torso":
            bone["x"] = r5(bone["x"] - LUMBAR)
        bones.append(bone)
    rig["bones"] = bones


def rebuild(asset, volumes):
    if any(b["id"] == "lumbar" for b in asset["rig"]["bones"]):
        raise SystemExit("the rig already has a lumbar bone")
    rebuild_rig(asset["rig"])
    for pose in asset["poses"]:
        rebuild_pose(pose)
    for anchor in asset["anchors"]:
        if anchor["id"] == "lumbar":
            anchor["bone"] = "lumbar"
        elif anchor.get("bone") == "torso":
            anchor["x"] = r5(anchor["x"] - LUMBAR)
    for representation in asset["representations"].values():
        geometry = representation.get("geometry")
        if geometry:
            for view in ("elevation", "front", "plan"):
                geometry[view] = rebuild_shapes(geometry[view])
    rebuild_volumes(volumes)


KEYS = ("rig", "anchors", "poses", "representations")


def main():
    original = open(CATALOG, encoding="utf-8").read()
    document = json.loads(original)
    source = json.load(open(SOURCE, encoding="utf-8"))
    asset = document["asset"]
    rebuild(asset, source["volumes"])
    if "--apply" not in sys.argv:
        asset["volumes"] = source["volumes"]
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print(CANDIDATE)
        return
    text = open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read()
    insert = {}
    exec(compile(text[: text.index("\ndef main(")], "volumes-insert.py", "exec"), insert)
    result = original
    for key in KEYS:
        start, end = insert["member_range"](result, "    ", key)
        result = result[:start] + '    "' + key + '": ' + insert["dump"](asset[key], 2) + result[end:]
    check = json.loads(result)["asset"]
    if any(check[key] != asset[key] for key in KEYS):
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf-8").read() != original:
        raise SystemExit("the catalog file changed while converting: run again")
    open(CATALOG, "w", encoding="utf-8").write(result)
    json.dump(source, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf-8").write("\n")
    print("applied to", CATALOG, "and", SOURCE)


if __name__ == "__main__":
    main()
