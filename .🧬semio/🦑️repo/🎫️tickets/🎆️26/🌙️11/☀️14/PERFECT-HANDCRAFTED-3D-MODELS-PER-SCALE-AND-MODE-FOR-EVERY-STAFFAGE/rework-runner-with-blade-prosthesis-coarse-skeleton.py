"""🦴 runner-with-blade-prosthesis: the skeleton of 1:200 and 1:500 by hand. Long bones keep their true shaft (`slender`), the forearm is radius and ulna, the femur leaves the hip joint over its neck and runs obliquely to the knee, the pelvis is a basin that narrows to the pubis, the collarbone of 1:200 is the true rod and the spine of 1:500 is one true rod up to the skull base that also is the member of the neck.

Usage: python3 rework-runner-with-blade-prosthesis-coarse-skeleton.py     rewrites the structure lists of 1:200 and 1:500 in the volumes source
"""
import copy
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "runner-with-blade-prosthesis.json")
UP = [0, -0.70711, 0, 0.70711]
FEMUR = [(0.0, 0.0, 0.0, 0.016), (0.022, -0.003, 0.034, 0.0135), (0.055, -0.004, 0.052, 0.0155), (0.2, 0.01, 0.03, 0.0138), (0.33, 0.01, 0.012, 0.0142), (0.435, 0.0, 0.0, 0.018)]
PELVIS = [(-0.075, 0.004, 0.045, 0.034), (-0.04, 0.004, 0.074, 0.048), (0.0, 0.004, 0.104, 0.06), (0.06, 0.003, 0.124, 0.067), (0.112, 0.0, 0.138, 0.07), (0.136, 0.0, 0.118, 0.054)]
NECK = [(0.56, -0.006, 0.018, 0.016), (0.627, -0.017, 0.019, 0.016)]
SHAFTS = {"sk-tibia-l": (0.425, 0.0135, 0.0115), "sk-tibia-r": (0.18, 0.0135, 0.0125), "sk-humerus-l": (0.33, 0.0125, 0.0118), "sk-humerus-r": (0.33, 0.0125, 0.0118)}


def loft(base, stations, sign=1):
    solid = {key: base[key] for key in ("id", "bone", "material", "role", "tone")}
    solid.update({"kind": "loft", "sections": [{"at": [x, y, sign * z], "rx": r, "ry": r, "exponent": 2, "rotation": UP} for x, y, z, r in stations], "capStart": True, "capEnd": True, "member": base["member"], "part": "member", "slender": True})
    return solid


source = json.load(open(SOURCE, encoding="utf8"))
fine = {solid["id"]: solid for solid in source["volumes"]["50"]["structure"]}
for scale in ("200", "500"):
    out = []
    for solid in source["volumes"][scale]["structure"]:
        name = solid["id"]
        if name in ("sk-femur-l", "sk-femur-r"):
            out.append(loft(solid, FEMUR, 1 if name.endswith("-l") else -1))
        elif name in SHAFTS:
            length, r0, r1 = SHAFTS[name]
            solid.update({"p0": [0.0, 0.0, 0.0], "p1": [length, 0.0, 0.0], "r0": r0, "r1": r1, "slender": True})
            out.append(solid)
        elif name in ("sk-forearm-l", "sk-forearm-r"):
            for bone in ("sk-ulna", "sk-radius"):
                twin = copy.deepcopy(fine[bone + name[-2:]])
                twin["slender"] = True
                out.append(twin)
        elif name == "sk-pelvis":
            solid["sections"] = [{"at": [x, y, 0.0], "rx": rx, "ry": ry, "exponent": 2, "rotation": UP} for x, y, rx, ry in PELVIS]
            out.append(solid)
        elif name in ("sk-clavicle-l", "sk-clavicle-r") and scale == "200":
            solid.update({"r0": 0.0085, "r1": 0.0065, "slender": True})
            out.append(solid)
        elif name == "sk-spine" and scale == "500":
            solid["sections"] = copy.deepcopy(next(s for s in source["volumes"]["200"]["structure"] if s["id"] == "sk-spine")["sections"])
            solid["sections"] += [{"at": [x, y, 0.0], "rx": rx, "ry": ry, "exponent": 2, "rotation": UP} for x, y, rx, ry in NECK]
            solid.update({"members": ["neck"], "slender": True})
            out.append(solid)
        elif name == "sk-cervical" and scale == "500":
            continue
        else:
            out.append(solid)
    source["volumes"][scale]["structure"] = out
json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
open(SOURCE, "a", encoding="utf8").write("\n")
