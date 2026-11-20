"""🦴 Root fixes of `humans/jogger` (round 3): hip joints get their lateral position, the trunk its anatomical length (C7 at 0.865 of the shod stature), the head-and-neck bone the remainder up to the vertex, the two centres of mass follow at their de Leva ratios, the three head anchors move with the head onto the modelled face and crown, and `dimensions` states the extents of the corrected body in the default pose (stride length, pose height, bideltoid breadth). Idempotent: states the final values.

Usage: python3 r3-jogger-roots.py
"""
import importlib.util
import os

here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("splice", os.path.join(here, "r3-jogger-splice.py"))
splice = importlib.util.module_from_spec(spec)
spec.loader.exec_module(splice)

BONES = {
    "torso": {"lengthM": 0.51, "mass.comX": 0.2329},
    "head": {"x": 0.51, "lengthM": 0.2449, "mass.comX": 0.1224},
    "thigh-l": {"z": 0.09},
    "thigh-r": {"z": -0.09},
}
ANCHORS = {"mouth": (0.052, -0.1), "nostrils": (0.081, -0.112), "crown": (0.2533, 0)}
DIMENSIONS = {"heightM": 1.764, "widthM": 1.007, "depthM": 0.475}


def change(asset):
    for bone in asset["rig"]["bones"]:
        for key, value in BONES.get(bone["id"], {}).items():
            if key.startswith("mass."):
                bone["mass"][key[5:]] = value
            else:
                bone[key] = value
    for anchor in asset["anchors"]:
        if anchor["id"] in ANCHORS:
            anchor["x"], anchor["y"] = ANCHORS[anchor["id"]]
    asset["dimensions"].update(DIMENSIONS)
    return ["rig", "anchors", "dimensions"]


print(splice.replace_members(change))
