"""💪️ Jogger, second round pass 2: shoulder girdle at its measured height, arm segments at their measured lengths, shoulder swing in the run.

Usage: python3 rework-jogger-shoulder.py <candidate out> [--publish]
Reads the catalog document as it is. Landmarks of a 1.795 m man in 25 mm shoes (ANSUR II P50 scaled, Drillis and Contini):
suprasternal notch 1.49 m, acromion 1.50 to 1.51 m, glenohumeral centre 1.47 to 1.48 m, elbow axis 1.156 m, wrist 0.89 m.
The girdle stood 30 mm above that, so sternoclavicular joint, clavicle, scapula and glenohumeral joint drop by 30 mm, the
upper arm becomes 0.322 m and the forearm 0.268 m, the shirt gets the slope of the trapezius, the manubrium ends under the
clavicles, and the clavicles swing with the arms in the twelve running poses. All numbers by hand.
"""
import copy
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, name + ".py"))
    m = importlib.util.module_from_spec(spec)
    sys.argv, keep = [sys.argv[0]], sys.argv
    spec.loader.exec_module(m)
    sys.argv = keep
    return m


lib = module("rework-02-lib")
ASSET = "humans/jogger"
GIRDLE_DROP = 0.03
LENGTHS = {"upper-arm": (0.336, 0.322), "forearm": (0.2792, 0.268)}
SHOULDER_SLOPE = [
    (0.2125, 0.0), (0.225, 0.0014), (0.2375, 0.005), (0.2465, 0.0093), (0.2555, 0.0158), (0.2645, 0.0214), (0.2735, 0.026),
    (0.2785, 0.0252), (0.2835, 0.0196), (0.2885, 0.0127), (0.2935, 0.008), (0.2965, 0.0056), (0.2995, 0.0036), (0.3025, 0.0019), (0.3055, 0.0),
]
SCAPULA_LOW, SCAPULA_LOW_TO, SCAPULA_GLENOID = 0.097, 0.095, 0.2522
STERNAL_ANGLE, MANUBRIUM_SHARE, FRONT_DEPTH = 0.2, 0.6, 0.06
SHOULDER_SWING, SHOULDER_SWING_MAX = 0.3, 6.0
DEPTH_WITH_SWING = 0.45
RUN_POSES = ("initial-contact", "loading", "midstance", "propulsion", "toe-off", "flight")
DECISIONS = {
    "body": {
        "en": " The shoulders stand at their measured height (acromion 1.51 m, glenohumeral joint 1.48 m in shoes): the shirt falls from the neck to the shoulder with the slope of the trapezius, the upper arm is 0.322 m and the forearm 0.268 m, and the hand ends at the knuckles with the curled fingers on a bone of their own.",
        "de": " Die Schultern stehen auf ihrer gemessenen Höhe (Akromion 1,51 m, Schultergelenk 1,48 m in Schuhen): Das Shirt fällt vom Hals zur Schulter mit der Neigung des Trapezmuskels, der Oberarm misst 0,322 m und der Unterarm 0,268 m, und die Hand endet an den Grundgelenken, die gekrümmten Finger hängen an einem eigenen Knochen.",
    },
    "structure": {
        "en": " The manubrium ends under the sternoclavicular joints at 1.51 m, the first ribs fall to it from T1, and the scapula lies from T3 to T8 with its acromion on the end of the clavicle and its glenoid at the head of the humerus.",
        "de": " Das Manubrium endet unter den Sternoklavikulargelenken auf 1,51 m, die ersten Rippen fallen von Th1 zu ihm ab, und das Schulterblatt liegt von Th3 bis Th8 mit dem Akromion am Ende des Schlüsselbeins und der Pfanne am Oberarmkopf.",
    },
}


def r5(x):
    return round(x, 5)


def drop_at(u):
    """📉️ How far the shirt outline comes in at the chest station u (the trapezius slope)."""
    if u <= SHOULDER_SLOPE[0][0] or u >= SHOULDER_SLOPE[-1][0]:
        return 0.0
    for (u0, d0), (u1, d1) in zip(SHOULDER_SLOPE, SHOULDER_SLOPE[1:]):
        if u0 <= u <= u1:
            return d0 + (d1 - d0) * (u - u0) / (u1 - u0)
    return 0.0


def scapula(u):
    """🦴️ Chest station of a scapula point: glenoid and acromion drop with the girdle, the blade ends at T8."""
    if u >= SCAPULA_GLENOID:
        return u - GIRDLE_DROP
    slope = (SCAPULA_GLENOID - GIRDLE_DROP - SCAPULA_LOW_TO) / (SCAPULA_GLENOID - SCAPULA_LOW)
    return SCAPULA_LOW_TO + (u - SCAPULA_LOW) * slope


def front_cage(p):
    """🫁️ Chest station of a rib or sternum point: the front of the cage above the sternal angle comes down to the manubrium."""
    u, v = p[0], p[1]
    if u <= STERNAL_ANGLE or v >= 0:
        return u
    share = min(-v / FRONT_DEPTH, 1.0)
    return u + share * ((STERNAL_ANGLE + (u - STERNAL_ANGLE) * MANUBRIUM_SHARE) - u)


def each_point(solid):
    """📍️ Every bone-local point of a solid as a mutable list."""
    if solid.get("kind") == "loft":
        return [s["at"] for s in solid["sections"]]
    if solid.get("kind") == "capsule":
        return [solid["p0"], solid["p1"]]
    if "transform" in solid and "translation" in solid["transform"]:
        return [solid["transform"]["translation"]]
    return []


def limb(asset):
    """📏️ Upper arms and forearms at their measured lengths: bones, masses, solids and 2D shapes scaled along the bone below the joint."""
    for b in asset["rig"]["bones"]:
        stem = b["id"][:-2]
        if stem in LENGTHS:
            old, new = LENGTHS[stem]
            assert abs(b["lengthM"] - old) < 1e-6, b["id"]
            b["lengthM"] = new
            b["mass"]["comX"] = r5(b["mass"]["comX"] * new / old)
        parent = (b.get("parent") or "")[:-2]
        if parent in LENGTHS and abs(b["x"] - LENGTHS[parent][0]) < 1e-6:
            b["x"] = LENGTHS[parent][1]
    for v in asset["volumes"].values():
        for name in ("body", "thermal", "structure"):
            for s in v[name]:
                stem = s.get("bone", "")[:-2]
                if stem in LENGTHS:
                    old, new = LENGTHS[stem]
                    for p in each_point(s):
                        if p[0] > 0:
                            p[0] = r5(p[0] * new / old)
    for r in asset["representations"].values():
        for shapes in r["geometry"].values():
            for sh in shapes:
                stem = sh.get("bone", "")[:-2]
                if stem in LENGTHS:
                    old, new = LENGTHS[stem]
                    assert sh["kind"] == "poly", (sh["id"], sh["kind"])
                    for p in sh["points"]:
                        if p[0] > 0:
                            p[0] = r5(p[0] * new / old)


def width_key(solid):
    """↔️ The section member that holds the half breadth of a trunk loft (the larger one at the chest)."""
    s = min(solid["sections"], key=lambda q: abs(q["at"][0] - 0.19))
    return "rx" if s["rx"] > s["ry"] else "ry"


def girdle(asset):
    """🦴️ Clavicles 30 mm lower (arms ride on them), trapezius slope on the shirt, scapulae and the front of the cage to match."""
    for b in asset["rig"]["bones"]:
        if b["id"].startswith("clavicle-"):
            b["x"] = r5(b["x"] - GIRDLE_DROP)
    moved = []
    for scale, v in asset["volumes"].items():
        for name in ("body", "thermal"):
            for s in v[name]:
                if s.get("bone") == "chest" and "shirt" in s["id"] and s.get("kind") == "loft":
                    key = width_key(s)
                    for q in s["sections"]:
                        q[key] = r5(q[key] - drop_at(q["at"][0]))
                    moved.append(f"{scale}:{name}:{s['id']}")
        for s in v["structure"]:
            if s.get("bone") != "chest" or s.get("part") != "member":
                continue
            if "scapul" in s["id"] or "glenoid" in s["id"]:
                for p in each_point(s):
                    p[0] = r5(scapula(p[0]))
                moved.append(f"{scale}:structure:{s['id']}")
            elif s["id"].startswith("sk-rib-") or s["id"] == "sk-sternum":
                points = each_point(s)
                if any(front_cage(p) != p[0] for p in points):
                    for p in points:
                        p[0] = r5(front_cage(p))
                    moved.append(f"{scale}:structure:{s['id']}")
    for scale, r in asset["representations"].items():
        for sh in r["geometry"]["front"]:
            if sh.get("bone") == "chest" and "shirt" in sh["id"] and sh["kind"] == "poly":
                for p in sh["points"]:
                    if abs(p[1]) > 0.07:
                        p[1] = r5((abs(p[1]) - drop_at(p[0])) * (1 if p[1] > 0 else -1))
                moved.append(f"{scale}:front:{sh['id']}")
    return moved


def swing(asset):
    """🏃️ The shoulder goes back with the arm that swings back and forward with the arm that swings forward (clavicle about its upright normal)."""
    done = {}
    for pose in asset["poses"]:
        if pose["id"][:-2] not in RUN_POSES:
            continue
        for side, sign in (("l", -1), ("r", 1)):
            arm = pose["bones"].get(f"upper-arm-{side}", {}).get("angleDeg", 0)
            turn = max(-SHOULDER_SWING_MAX, min(SHOULDER_SWING_MAX, SHOULDER_SWING * arm))
            pose["bones"][f"clavicle-{side}"] = {"abductionDeg": round(sign * turn, 2)}
        done[pose["id"]] = [pose["bones"]["clavicle-l"]["abductionDeg"], pose["bones"]["clavicle-r"]["abductionDeg"]]
    return done


def decisions(asset):
    for v in asset["volumes"].values():
        for i, key in ((0, "body"), (2, "structure")):
            for lang in ("en", "de"):
                v["decisions"][i][lang] += DECISIONS[key][lang]


def main():
    out = sys.argv[1]
    base = lib.load_catalog(ASSET)
    document = copy.deepcopy(base)
    asset = document["asset"]
    limb(asset)
    moved = girdle(asset)
    done = swing(asset) if "--no-swing" not in sys.argv else {}
    if done:
        asset["dimensions"]["depthM"] = DEPTH_WITH_SWING
    decisions(asset)
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w", encoding="utf-8") as f:
        json.dump(document, f, ensure_ascii=False, indent=2)
        f.write("\n")
    if "--publish" in sys.argv:
        assert lib.load_catalog(ASSET) == base, "the catalog document changed since it was read"
        lib.save_source(ASSET, {"volumes": asset["volumes"]})
        lib.save_catalog(ASSET, document)
    print(json.dumps({"moved": moved, "swing": done}))


if __name__ == "__main__":
    main()
