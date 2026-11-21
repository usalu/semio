"""🦾 Round 5 root fixes of `humans/jogger`: the upper arm gets its anthropometric length (0.186 H = 0.336 m; elbow at 0.636 of the barefoot stature instead of 0.651) and the hand its length (0.108 H = 0.195 m), the forearm follows the new elbow, both centres of mass keep their de Leva ratios. Idempotent: states the final values.

Usage: python3 r5-jogger-roots.py
"""
import importlib.util
import os

here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("splice", os.path.join(here, "r3-jogger-splice.py"))
splice = importlib.util.module_from_spec(spec)
spec.loader.exec_module(splice)

BONES = {}
for side in ("l", "r"):
    BONES["upper-arm-" + side] = {"lengthM": 0.336, "mass.comX": 0.1939}
    BONES["forearm-" + side] = {"x": 0.336}
    BONES["hand-" + side] = {"lengthM": 0.195, "mass.comX": 0.0975}


def change(asset):
    for bone in asset["rig"]["bones"]:
        for key, value in BONES.get(bone["id"], {}).items():
            if key.startswith("mass."):
                bone["mass"][key[5:]] = value
            else:
                bone[key] = value
    return ["rig"]


print(splice.replace_members(change))
