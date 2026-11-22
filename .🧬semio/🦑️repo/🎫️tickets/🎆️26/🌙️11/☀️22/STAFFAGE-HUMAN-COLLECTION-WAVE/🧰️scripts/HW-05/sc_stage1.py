"""[DEBUG] stage-1 probe of the schoolchild geometry (rescale + pack) before the full builder."""
from lib import *
from tools import *
F = {"head": 1.03, "neck": 1.10, "pelvis": 1.11, "torso": 1.11, "pack": 1.11, "clavicle-l": 1.11, "clavicle-r": 1.11, "ilium-l": 1.11, "ilium-r": 1.11}
for s in "lr":
    for b in ("thigh", "shank", "foot", "toe"):
        F[f"{b}-{s}"] = 1.18
    for b in ("arm", "forearm", "hand"):
        F[f"{b}-{s}"] = 1.15
d = rescale(load("running-child"), F)
d["asset"]["id"] = "schoolchild-with-backpack"
d["asset"]["presentation"]["defaultPose"] = "standing-ready"
affine_bone(d, "pack", (0.04, 0.03, 0.0), (1.12, 1.45, 1.25))
slug = "schoolchild-with-backpack"
print(set_dims(d, slug))
found = feature_loop(d, slug)
summarize(found)
