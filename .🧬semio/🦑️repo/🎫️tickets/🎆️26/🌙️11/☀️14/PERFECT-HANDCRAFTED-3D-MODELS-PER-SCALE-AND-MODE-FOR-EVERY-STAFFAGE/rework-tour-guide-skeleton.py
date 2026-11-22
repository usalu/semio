"""🦴 tour-guide: the skeleton of every scale closed into one load path and simplified for its smallest feature. It starts from the structure lists of `rework-tour-guide-structure-base.json` (the lists before this pass) and applies the hand-stated corrections below, so it can be run again.

1:50 and 1:100 keep the anatomical bones and close the joints that were open (sacro-iliac, temporo-mandibular, atlanto-occipital, acromio-clavicular, patello-femoral, tibio-fibular, metacarpo-phalangeal, C7/T1). 1:200 draws no bone under 30 mm, 1:500 none under 75 mm (66 mm in the arm): long bones become rods from pivot to pivot, forearm bones merge, parts below the feature merge into their bone.
Usage: python3 rework-tour-guide-skeleton.py
"""
import copy
import importlib.util
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("rework_01_lib", os.path.join(T, "rework-01-lib.py"))
lib = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(lib)

SOURCE = os.path.join(T, "volumes", "humans", "tour-guide.json")
BASE = os.path.join(T, "rework-tour-guide-structure-base.json")
SIDES = ("l", "r")
FINGERS = (("index", 2), ("middle", 3), ("ring", 4), ("little", 5), ("thumb", 1))
ROD = {"200": {"femur": None, "tibia": 0.0175, "humerus": None, "forearm": 0.0175, "floor": 0.0155}, "500": {"femur": 0.039, "tibia": 0.039, "humerus": 0.0345, "forearm": 0.0295, "floor": 0.039}}
DROP = {
    "200": ("atlas-condyle", "patella", "malleolus", "fibula", "ulna", "radius"),
    "500": ("atlas-condyle", "femur-neck", "femur-head", "femur-trochanter", "femur-condyles", "patella", "tibia-plateau", "malleolus", "fibula", "humerus-head", "humerus-condyles", "ulna", "radius"),
}
FLOOR = {"200": ("joint-upper-arm", "humerus", "humerus-condyles"), "500": ("joint-shank",)}
SACRUM_ALA = (0.040, 0.034, 0.020, 0.0085)
ALA = ((0.090, 0.055, 0.020), (0.087, 0.088, 0.064), 0.014)
ILIUM = ((0.080, 0.030, 0.095), (0.072, 0.076), 28.0)
TEMPORAL = ((0.045, -0.015, 0.050), (0.020, 0.030, 0.014))
STERNAL_NOTCH_200 = (0.432, -0.045)
MOVES = {
    "50": [("move", "patella-{s}", "femur-condyles-{s}", 0.0085), ("end", "fibula-{s}", "tibia-plateau-{s}", 0.008)],
    "100": [("move", "patella-{s}", "femur-condyles-{s}", 0.0085), ("end", "fibula-{s}", "tibia-plateau-{s}", 0.006)],
    "200": [],
    "500": [],
}


def centre(frames, solid):
    if solid["kind"] == "capsule":
        return frames.world(solid["bone"], [(a + b) / 2 for a, b in zip(solid["p0"], solid["p1"])])
    if solid["kind"] == "loft":
        ats = [s["at"] for s in solid["sections"]]
        return frames.world(solid["bone"], [sum(a[i] for a in ats) / len(ats) for i in range(3)])
    return frames.world(solid["bone"], solid.get("transform", {}).get("translation", [0, 0, 0]))


def toward(frames, bone, local, target, d):
    p = frames.world(bone, local)
    n = math.dist(p, target) or 1
    return [lib.r5(c) for c in frames.local(bone, [p[i] + (target[i] - p[i]) * d / n for i in range(3)])]


def floor_radii(solid, r):
    if solid["kind"] == "capsule":
        solid["r0"], solid["r1"] = max(solid["r0"], r), max(solid["r1"], r)
    elif solid["kind"] == "ellipsoid":
        solid["radii"] = [max(x, r) for x in solid["radii"]]
    elif solid["kind"] == "loft":
        for s in solid["sections"]:
            s["rx"], s["ry"] = max(s["rx"], r), max(s["ry"], r)


def rod(sid, bone, length, r0, r1, like):
    out = {"id": sid, "kind": "capsule", "bone": bone, "material": like["material"], "role": like["role"], "tone": like["tone"], "p0": [0, 0, 0], "p1": [lib.r5(length), 0, 0], "r0": r0, "r1": r1}
    out.update({k: like[k] for k in ("member", "part") if k in like})
    out["member"] = bone
    return out


def run():
    source = lib.read(SOURCE)
    if not os.path.exists(BASE):
        lib.write(BASE, {scale: source["volumes"][scale]["structure"] for scale in ROD.keys() | MOVES.keys()})
    base = lib.read(BASE)
    asset = lib.read(lib.catalog_path("👤️humans", "tour-guide"))["asset"]
    frames = lib.Frames(asset["rig"])
    length = {b["id"]: b["lengthM"] for b in asset["rig"]["bones"]}
    for scale in ("50", "100", "200", "500"):
        solids = copy.deepcopy(base[scale])
        by = {s["id"]: s for s in solids}
        own = lambda s: "kind" in s
        for kind, a, b, d in MOVES[scale]:
            for side in SIDES:
                one, two = by.get(a.format(s=side)), by.get(b.format(s=side))
                if one is None or two is None:
                    continue
                target = centre(frames, two)
                if kind == "move":
                    one.setdefault("transform", {})["translation"] = toward(frames, one["bone"], one.get("transform", {}).get("translation", [0, 0, 0]), target, d)
                elif one["kind"] == "capsule":
                    key = min(("p0", "p1"), key=lambda k: math.dist(frames.world(one["bone"], one[k]), target))
                    one[key] = toward(frames, one["bone"], one[key], target, d)
                elif one["kind"] == "loft":
                    section = min((one["sections"][0], one["sections"][-1]), key=lambda s: math.dist(frames.world(one["bone"], s["at"]), target))
                    section["at"] = toward(frames, one["bone"], section["at"], target, d)
        if "sacrum" in by and by["sacrum"]["kind"] == "loft":
            for section, ala in zip(by["sacrum"]["sections"], SACRUM_ALA):
                section["rx"] = max(section["rx"], ala)
        bone_like = by.get("sacrum") or by.get("spine")
        extra = {"material": bone_like["material"], "role": bone_like["role"], "tone": bone_like["tone"], "part": "member"}
        if scale in ("50", "100"):
            if scale == "100":
                fine = {s["id"]: s for s in base["50"]}
                for side in SIDES:
                    at = solids.index(by[f"ischiopubic-{side}"])
                    solids[at:at + 1] = [copy.deepcopy(fine[f"pubis-{side}"]), copy.deepcopy(fine[f"ischium-{side}"])]
            for side, sign in (("l", 1), ("r", -1)):
                (c, (ru, rv), turn) = ILIUM
                ilium = by[f"ilium-{side}"]
                ilium["radii"] = [ru, rv, ilium["radii"][2]]
                ilium["transform"] = {"translation": [c[0], c[1], sign * c[2]], "rotation": [lib.r5(-sign * math.sin(math.radians(turn / 2))), 0, 0, lib.r5(math.cos(math.radians(turn / 2)))]}
                (a, b, r) = ALA
                solids.insert(solids.index(by["sacrum"]) + 1, {"id": f"sacrum-ala-{side}", "kind": "capsule", "bone": "pelvis", **extra, "p0": [a[0], a[1], sign * a[2]], "p1": [b[0], b[1], sign * b[2]], "r0": r, "r1": r, "member": "pelvis"})
        if scale == "50":
            for side, sign in (("l", 1), ("r", -1)):
                (c, radii) = TEMPORAL
                solids.insert(solids.index(by["cranium"]) + 1, {"id": f"temporal-{side}", "kind": "ellipsoid", "bone": "head", **extra, "radii": list(radii), "transform": {"translation": [c[0], c[1], sign * c[2]]}, "member": "head"})
        if scale == "200":
            top = by["sternum"]["sections"][-1]
            top["at"] = [STERNAL_NOTCH_200[0], STERNAL_NOTCH_200[1], 0]
        cervical = by.get("cervical-spine")
        if cervical is not None:
            last = cervical["sections"][-1]
            dens = dict(last)
            dens["at"] = [lib.r5(last["at"][0] + 0.032), last["at"][1], last["at"][2]]
            cervical["sections"].append(dens)
            if "joint-neck" in by:
                first, second = cervical["sections"][0]["at"], cervical["sections"][1]["at"]
                t = (0 - first[0]) / (second[0] - first[0])
                by["joint-neck"].setdefault("transform", {})["translation"] = [0, lib.r5(first[1] + (second[1] - first[1]) * t), 0]
        if scale == "50":
            for side, sign in (("l", 1), ("r", -1)):
                clav, scap = by[f"clavicle-{side}"], by[f"scapula-{side}"]
                c = scap["transform"]["translation"]
                solids.insert(solids.index(scap) + 1, {"id": f"acromion-{side}", "kind": "capsule", "bone": "torso", "material": scap["material"], "role": scap["role"], "tone": scap["tone"], "p0": [lib.r5(c[0] + 0.058), lib.r5(c[1] - 0.012), lib.r5(c[2] + sign * 0.038)], "p1": list(clav["p1"]), "r0": 0.008, "r1": 0.008, "member": "torso", "part": "member"})
                for name, ray in FINGERS:
                    phalanx, meta = by.get(f"phalanges-{side}-{name}"), by.get(f"metacarpal-{side}-{ray}")
                    if phalanx is None or meta is None:
                        continue
                    head = frames.world(meta["bone"], meta["p1"])
                    ends = (phalanx["sections"][0], phalanx["sections"][-1])
                    near = min(ends, key=lambda s: math.dist(frames.world(phalanx["bone"], s["at"]), head))
                    near["at"] = [lib.r5(x) for x in frames.local(phalanx["bone"], head)]
                    for section in phalanx["sections"]:
                        section["rx"], section["ry"] = lib.r5(section["rx"] + 0.0018), lib.r5(section["ry"] + 0.0018)
        if scale in ROD:
            rule = ROD[scale]
            kept = []
            for solid in solids:
                stem = solid["id"][:-2] if solid["id"][-2:] in ("-l", "-r") else solid["id"]
                if stem in DROP[scale]:
                    continue
                kept.append(solid)
            solids = kept
            by = {s["id"]: s for s in solids}
            for side in SIDES:
                for stem, bone, key in (("femur", f"thigh-{side}", "femur"), ("tibia", f"shank-{side}", "tibia"), ("humerus", f"upper-arm-{side}", "humerus")):
                    old = by.get(f"{stem}-{side}")
                    if old is not None and rule[key] is not None:
                        solids[solids.index(old)] = rod(old["id"], bone, length[bone], rule[key], rule[key], old)
                like = next(x for x in solids if x["id"] == f"humerus-{side}")
                solids.insert(solids.index(like) + 1, rod(f"forearm-bones-{side}", f"forearm-{side}", length[f"forearm-{side}"], rule["forearm"], rule["forearm"], like))
            for solid in solids:
                stem = solid["id"][:-2] if solid["id"][-2:] in ("-l", "-r") else solid["id"]
                if own(solid) and stem in FLOOR[scale]:
                    floor_radii(solid, rule["floor"])
        source["volumes"][scale]["structure"] = solids
    with open(SOURCE, "w", encoding="utf8") as handle:
        json.dump(source, handle, ensure_ascii=False, indent=1)
        handle.write("\n")


run()
