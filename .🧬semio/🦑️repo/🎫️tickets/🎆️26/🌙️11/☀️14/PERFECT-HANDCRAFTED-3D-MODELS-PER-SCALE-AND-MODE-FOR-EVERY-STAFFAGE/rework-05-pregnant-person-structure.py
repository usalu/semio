"""🦴 pregnant-person: the skeleton becomes one load path. Iliac body from the sacroiliac joint to the acetabulum, arched iliac crest, patella on the femoral trochlea with its ligament to the tibia, femoral condyles down to the knee centre, rib heads on the vertebral bodies at the coarser scales, a connected fetal skeleton and the uterine wall as the sling that carries the fetus on the lumbar spine.

Usage: python3 rework-05-pregnant-person-structure.py    edits `volumes/humans/pregnant-person.json` (run once; publish with volumes-insert.py)
"""
import importlib.util
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "pregnant-person.json")
spec = importlib.util.spec_from_file_location("rig", os.path.join(T, "rework-05-pregnant-person-rig.py"))
RIG = importlib.util.module_from_spec(spec)
spec.loader.exec_module(RIG)

HALF = {"50": 0.00375, "100": 0.0078, "200": 0.0153}
THORACIC_AXIS = [(0, -0.0012), (0.02, 0.004), (0.12, 0.031), (0.22, 0.026), (0.288, 0)]
UTERUS = [(0.035, 0.0), (0.045, -0.07), (0.075, -0.135), (0.122, -0.166), (0.175, -0.15), (0.218, -0.095), (0.238, -0.02), (0.232, 0.06), (0.2, 0.12), (0.145, 0.143), (0.095, 0.135), (0.055, 0.085), (0.035, 0.0)]


def member(solid_id, bone, sections, **more):
    return {"id": solid_id, "kind": "loft", "bone": bone, "material": "bone", "role": "body", "tone": 0.92, "sections": sections, "capStart": True, "capEnd": True, "member": bone, "part": "member", **more}


def round_section(at, r):
    return {"at": [round(c, 5) for c in at], "rx": r, "ry": r}


def spine_y(x):
    for (x0, y0), (x1, y1) in zip(THORACIC_AXIS, THORACIC_AXIS[1:]):
        if x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return THORACIC_AXIS[-1][1]


def uterine_wall(thickness):
    sections = []
    count = len(UTERUS) - 1
    for i, (u, v) in enumerate(UTERUS):
        before, after = UTERUS[(i - 1) % count], UTERUS[(i + 1) % count]
        tx, ty = after[0] - before[0], after[1] - before[1]
        n = math.hypot(tx, ty)
        tx, ty = tx / n, ty / n
        q = RIG.K.quaternion([[0, ty, tx], [0, -tx, ty], [1, 0, 0]])
        sections.append({"at": [u, v, 0.012], "rx": 0.02, "ry": thickness, "exponent": 2.6, "rotation": RIG.r5(q)})
    return member("uterine-wall", "belly", sections, tone=0.8)


def main():
    source = json.load(open(SOURCE, encoding="utf-8"))
    for scale in ("50", "100", "200"):
        structure = source["volumes"][scale]["structure"]
        by_id = {s["id"]: s for s in structure}
        if "uterine-wall" in by_id:
            raise SystemExit("already edited")
        half = HALF[scale]
        add = []
        for side, sign in (("l", 1), ("r", -1)):
            r = max(0.014, half)
            add.append(member(f"iliac-body-{side}", f"ilium-{side}", [round_section([-0.004, 0, 0], r + 0.001), round_section([0.045, 0, 0], r), round_section([0.092, 0, 0], max(0.02, half))]))
            femur = by_id[f"femur-{side}"]
            if scale != "50":
                femur["sections"][-1]["at"] = [0.405, -0.004, 0]
            if scale == "50":
                wing = by_id[f"hip-bone-{side}"]["sections"]
                a, b = wing[0]["at"], wing[1]["at"]
                wing.insert(0, dict(wing[0], at=[round(a[i] + 0.55 * (a[i] - b[i]), 5) for i in range(3)], rx=0.05, ry=0.0055, exponent=2))
            patella = by_id.get(f"patella-{side}")
            if patella:
                patella["radii"] = [0.021, max(0.011, half), 0.02]
                patella["transform"]["translation"] = [-0.014, 0.034, 0]
                tendon = max(0.0045, half)
                add.append(member(f"patellar-ligament-{side}", f"shank-{side}", [round_section([0.0, 0.037, 0], tendon), round_section([0.045, 0.0225 if scale == "50" else 0.02, 0], tendon)], tone=0.8))
            if scale != "50":
                for solid in structure:
                    if solid["id"].startswith("rib-") and solid["id"].endswith("-" + side):
                        head = solid["sections"][0]
                        head["at"] = [head["at"][0], round(spine_y(head["at"][0]) + 0.008, 5), sign * 0.012]
        if scale == "50":
            by_id["fetal-spine"]["sections"].insert(0, round_section([0.122, -0.088, 0.02], 0.0075))
            by_id["fetal-humerus-right"]["sections"][0]["at"] = [0.16, -0.026, -0.004]
            by_id["fetal-humerus-left"]["sections"][0]["at"] = [0.16, -0.026, 0.044]
        if scale == "100":
            by_id["fetal-spine"]["sections"].insert(0, round_section([0.122, -0.088, 0.02], 0.0078))
            by_id["fetal-legs"]["sections"][0]["at"] = [0.146, 0.124, 0.026]
        add.append(uterine_wall(max(0.004, half)))
        structure.extend(add)
    open(SOURCE, "w", encoding="utf-8").write(RIG.DUMP({"volumes": source["volumes"]}, 0) + "\n")
    print("structure edited")


main()
