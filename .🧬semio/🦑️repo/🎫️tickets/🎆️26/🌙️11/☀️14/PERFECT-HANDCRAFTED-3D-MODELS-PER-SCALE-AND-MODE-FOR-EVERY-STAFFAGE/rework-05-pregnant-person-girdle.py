"""🦴 pregnant-person: shoulder girdle and rib heads as a load path. The acromion reaches the clavicle (acromioclavicular joint), the scapular neck carries the glenoid, every rib head sits on its vertebral body; at 1:100 and 1:200 the girdle rod runs clavicle, acromion, glenoid, scapula and the rib heads start in the spine.

Usage: python3 rework-05-pregnant-person-girdle.py    edits `volumes/humans/pregnant-person.json` (run once)
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
STRUCTURE = importlib.util.spec_from_file_location("structure", os.path.join(T, "rework-05-pregnant-person-structure.py"))

THORACIC_AXIS = [(0, -0.0012), (0.02, 0.004), (0.12, 0.031), (0.22, 0.026), (0.288, 0)]
GLENOID = (0.12745, -0.03036, 0.015)
SCAPULA_LATERAL = (0.147, -0.0174, 0.0955)
ACROMION_TIP = (0.15, 0.004, 0.004)


def spine_y(x):
    for (x0, y0), (x1, y1) in zip(THORACIC_AXIS, THORACIC_AXIS[1:]):
        if x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return THORACIC_AXIS[-1][1]


def main():
    source = json.load(open(SOURCE, encoding="utf-8"))
    asset = json.load(open(RIG.CATALOG, encoding="utf-8"))["asset"]
    arm = next(b for b in asset["rig"]["bones"] if b["id"] == "arm-l")
    shoulder = (arm["x"], arm["y"], arm["z"])
    fine = source["volumes"]["50"]["structure"]
    by_id = {s["id"]: s for s in fine}
    if "scapular-neck-l" in by_id:
        raise SystemExit("already edited")
    vertebrae = [s for s in fine if s["id"].startswith("vertebra-t")]
    for solid in fine:
        if solid["id"].startswith("rib-"):
            head = solid["sections"][0]
            near = min(vertebrae, key=lambda v: abs((v["p0"][0] + v["p1"][0]) / 2 - head["at"][0]))
            centre = [(near["p0"][i] + near["p1"][i]) / 2 for i in range(3)]
            reach = near["r0"] + head["rx"] - 0.003
            offset = [head["at"][i] - centre[i] for i in range(3)]
            distance = math.sqrt(sum(c * c for c in offset))
            if distance > reach:
                head["at"] = [round(centre[i] + offset[i] * reach / distance, 5) for i in range(3)]
    for side, sign in (("l", 1), ("r", -1)):
        flip = lambda p: [p[0], p[1], sign * p[2]]
        spine = by_id[f"scapular-spine-{side}"]["sections"]
        spine.append({"at": flip(ACROMION_TIP), "rx": 0.0072, "ry": 0.0072})
        fine.append({"id": f"scapular-neck-{side}", "kind": "loft", "bone": f"clavicle-{side}", "material": "bone", "role": "body", "tone": 0.92, "sections": [{"at": flip(SCAPULA_LATERAL), "rx": 0.011, "ry": 0.011}, {"at": flip(GLENOID), "rx": 0.012, "ry": 0.012}], "capStart": True, "capEnd": True, "member": f"clavicle-{side}", "part": "member"})
    for scale in ("100", "200"):
        for solid in source["volumes"][scale]["structure"]:
            if solid["id"].startswith("rib-"):
                head = solid["sections"][0]
                z = 0.003 if solid["id"].endswith("-l") else -0.003
                head["at"] = [head["at"][0], round(spine_y(head["at"][0]), 5), z]
            if solid["id"].startswith("shoulder-girdle-"):
                sign = 1 if solid["id"].endswith("-l") else -1
                sections = solid["sections"]
                glenoid = dict(sections[2], at=[round(shoulder[0] - 0.006, 5), round(shoulder[1] + 0.004, 5), round(sign * (shoulder[2] + 0.01), 5)])
                sections.insert(3, glenoid)
    open(SOURCE, "w", encoding="utf-8").write(RIG.DUMP({"volumes": source["volumes"]}, 0) + "\n")
    print("girdle edited; glenohumeral centre in the clavicle frame", shoulder)


main()
