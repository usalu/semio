"""🐈 domestic-cat: walk and stalk key poses on the inclined scapulae.

A scapula resting 15.7 degrees out of the sagittal plane moves the forepaw sideways when it swings (0.7 mm per degree), so the authored swing of up to 15.4 degrees let a planted paw wander 11 mm. Each key pose keeps half of its scapular swing; humerus, forearm, metacarpus and paw are solved in the sagittal plane so that wrist, paw joint and paw tip stay where they were in side elevation (a pose whose limb cannot reach keeps its authored angles). One-shot: `python3 rework-domestic-cat-gait.py --apply` writes `poses` of the catalog file.
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("girdle", os.path.join(T, "rework-domestic-cat-girdle.py"))
G = importlib.util.module_from_spec(spec)
spec.loader.exec_module(G)
K = G.K
KEEP = 0.5
MARK = "gait-scapula-halved"


def chain(bones, side):
    """The bones whose frames a forelimb solve needs: the limb and its ancestors."""
    by_id = {bone["id"]: bone for bone in bones}
    wanted, bone = [], by_id[f"paw-f{side}"]
    while bone is not None:
        wanted.append(bone)
        bone = by_id.get(bone["parent"])
    return list(reversed(wanted))


def resolve(bones, offsets, side):
    limb = chain(bones, side)
    paw = limb[-1]
    names = [f"{part}-f{side}" for part in ("scapula", "humerus", "forearm", "metacarpus", "paw")]
    was = K.frames(limb, copy.deepcopy(offsets))
    wrist, knuckle, toe = was[names[3]][0][:2], was[names[4]][0][:2], G.end_of(was, paw)[:2]
    start = {name: offsets.get(name, {}).get("angleDeg", 0.0) for name in names}
    base = {key: value for key, value in offsets.items() if key not in names}
    scapula = round(start[names[0]] * KEEP, 2)
    trial = lambda extra: K.frames(limb, {**base, names[0]: {"angleDeg": scapula}, **{bone: {"angleDeg": angle} for bone, angle in extra.items()}})
    h, f = start[names[1]], start[names[2]]
    for span, step in ((16, 1.0), (1, 0.05), (0.05, 0.0025)):
        count = round(span / step)
        h, f = min(((h + i * step, f + j * step) for i in range(-count, count + 1) for j in range(-count, count + 1)), key=lambda pair: math.dist(trial({names[1]: pair[0], names[2]: pair[1]})[names[3]][0][:2], wrist))
    fixed = {names[1]: round(h, 2), names[2]: round(f, 2)}
    m = min((start[names[3]] + i * 0.02 for i in range(-1000, 1001)), key=lambda angle: math.dist(trial({**fixed, names[3]: angle})[names[4]][0][:2], knuckle))
    fixed[names[3]] = round(m, 2)
    p = min((start[names[4]] + i * 0.02 for i in range(-1000, 1001)), key=lambda angle: math.dist(G.end_of(trial({**fixed, names[4]: angle}), paw)[:2], toe))
    fixed[names[4]] = round(p, 2)
    now = trial(fixed)
    miss = max(math.dist(now[names[3]][0][:2], wrist), math.dist(now[names[4]][0][:2], knuckle), math.dist(G.end_of(now, paw)[:2], toe))
    return {names[0]: scapula, **fixed}, miss, G.end_of(was, paw)[2], G.end_of(now, paw)[2]


def main():
    original = text = open(G.CATALOG, encoding="utf8").read()
    document = json.loads(text)
    asset = document["asset"]
    before = copy.deepcopy(asset["poses"])
    bones = asset["rig"]["bones"]
    rest = K.frames(bones)
    limits = {bone["id"]: bone["joint"] for bone in bones}
    if abs(next(pose for pose in asset["poses"] if pose["id"] == "walk-phase-3")["bones"]["scapula-ff"]["angleDeg"]) < 10:
        raise SystemExit("already applied")
    for pose in asset["poses"]:
        if not pose["id"].startswith(("walk-phase", "stalk-phase")):
            continue
        for side in ("f", "n"):
            if f"scapula-f{side}" not in pose["bones"]:
                continue
            solved, miss, z0, z1 = resolve(bones, pose["bones"], side)
            inside = all(limits[bone]["minDeg"] <= angle <= limits[bone]["maxDeg"] for bone, angle in solved.items())
            zr = G.end_of(rest, next(bone for bone in bones if bone["id"] == f"paw-f{side}"))[2]
            print("[DEBUG]", pose["id"], side, "miss mm", round(miss * 1000, 2), "paw beside rest mm: before", round((z0 - zr) * 1000, 1), "now", round((z1 - zr) * 1000, 1), "kept" if miss <= 0.0005 and inside else "AUTHORED ANGLES KEPT")
            if miss <= 0.0005 and inside:
                for bone, angle in solved.items():
                    pose["bones"][bone] = {"angleDeg": angle}
    if "--apply" not in sys.argv:
        return
    text = K.splice(text, "poses", before, asset["poses"])
    if json.loads(text) != document or open(G.CATALOG, encoding="utf8").read() != original:
        raise SystemExit("not written: splice did not round-trip or the catalog changed")
    open(G.CATALOG, "w", encoding="utf8").write(text)
    print("applied")


if __name__ == "__main__":
    main()
