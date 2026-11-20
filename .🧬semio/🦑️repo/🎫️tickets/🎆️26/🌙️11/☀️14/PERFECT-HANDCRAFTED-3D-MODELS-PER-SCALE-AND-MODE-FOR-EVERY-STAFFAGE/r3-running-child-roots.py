"""🦴 Root fixes of `humans/running-child` (round 3): the hip joints rise from crotch height (0.443 H) to the child's hip joint height (0.50 H), thigh and shank get their real lengths, the ankle its real height over a 12 mm sole, the trunk shortens to the real distance from hip joint to C7 (0.82 H), neck and head take the remainder up to the unchanged vertex, the shoulder joints stay at their real height, the hands get their real length; centres of mass follow at their old ratios, anchors move with the corrected foot and head, the poses and clip keys take the retargeted leg angles and flight heights of `r3-running-child-motion.json`, `presentation.defaultPose` becomes the running pose and `dimensions` states the extents of the corrected body in it. Idempotent: states the final values.

Usage: python3 r3-running-child-roots.py [--candidate]
"""
import importlib.util
import json
import os
import sys

here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("splice", os.path.join(here, "r3-running-child-splice.py"))
splice = importlib.util.module_from_spec(spec)
spec.loader.exec_module(splice)

BONES = {
    "pelvis": {"y": 0.5745, "lengthM": 0.07},
    "torso": {"lengthM": 0.29, "mass.comX": 0.1336},
    "pack": {"x": 0.075, "mass.comX": 0.035, "mass.comY": 0.08},
    "neck": {"x": 0.29, "lengthM": 0.07, "mass.comX": 0.035},
    "head": {"x": 0.07, "lengthM": 0.1335, "mass.comX": 0.0611},
}
SIDE = {
    "arm": {"x": 0.2255, "lengthM": 0.1848},
    "hand": {"lengthM": 0.12, "mass.comX": 0.06},
    "thigh": {"lengthM": 0.2565, "mass.comX": 0.1052},
    "shank": {"x": 0.2565, "lengthM": 0.262, "mass.comX": 0.1168},
    "foot": {"x": 0.262, "angleDeg": 55.34, "lengthM": 0.0985, "mass.comX": 0.0419},
    "toe": {"x": 0.0985, "angleDeg": 34.66, "lengthM": 0.05, "mass.comX": 0.023, "mass.comY": 0.0164},
}
ANCHORS = {"heel-l": (0.0088, -0.062), "heel-r": (0.0088, -0.062), "ball-l": (0.0985, 0), "ball-r": (0.0985, 0), "mouth": (-0.0185, -0.079), "nostrils": (0.0005, -0.086), "crown": (0.1335, 0)}
DIMENSIONS = {"heightM": 1.096, "widthM": 0.671, "depthM": 0.29}
DEFAULT_POSE = "toe-off"
ADVANCE = {"sprint-cycle": 1.1228}


def change(asset):
    for bone in asset["rig"]["bones"]:
        values = dict(BONES.get(bone["id"], {}))
        values.update(SIDE.get(bone["id"][:-2], {}) if bone["id"][-2:] in ("-l", "-r") else {})
        for key, value in values.items():
            if key.startswith("mass."):
                bone["mass"][key[5:]] = value
            else:
                bone[key] = value
    for anchor in asset["anchors"]:
        if anchor["id"] in ANCHORS:
            anchor["x"], anchor["y"] = ANCHORS[anchor["id"]]
    asset["dimensions"].update(DIMENSIONS)
    asset["presentation"]["defaultPose"] = DEFAULT_POSE
    motion = json.load(open(os.path.join(here, "r3-running-child-motion.json"), encoding="utf-8"))
    for pose in asset["poses"]:
        pose["bones"] = motion["poses"][pose["id"]]["bones"]
        if "root" in motion["poses"][pose["id"]]:
            pose["root"] = motion["poses"][pose["id"]]["root"]
    for clip in asset["clips"]:
        if clip["id"] in ADVANCE:
            clip["advanceM"] = ADVANCE[clip["id"]]
        for key, moved in zip(clip["keys"], motion["clips"][clip["id"]]):
            assert key["t"] == moved["t"] and key["pose"] == moved["pose"]
            key["bones"] = moved["bones"]
    return ["rig", "anchors", "dimensions", "presentation", "poses", "clips"]


if __name__ == "__main__":
    print(splice.replace_members(change, "--candidate" in sys.argv))
