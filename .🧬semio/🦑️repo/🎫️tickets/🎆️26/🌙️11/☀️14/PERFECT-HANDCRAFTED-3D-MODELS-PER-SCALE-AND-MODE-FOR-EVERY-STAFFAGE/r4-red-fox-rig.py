"""🦴 Round 4 root fix of animals/red-fox: the limb and ear bones get their lateral joint positions (`z`, metres, near side negative), so the rig runs inside the legs and ears instead of on the midline. Sagittal joint positions, lengths, angles, poses and clips stay as round 3 set them.

Usage: python3 r4-red-fox-rig.py [--candidate|--apply]
Lateral landmarks (half spacing from the median plane, adult red fox of 6 kg): hip joint 0.036, stifle 0.042, hock 0.037, hind paw 0.036; dorsal border of the scapula 0.030, shoulder joint 0.044, elbow 0.037, carpus 0.033, forepaw 0.032; ear base 0.031.
"""
import importlib.util
import json
import sys

spec = importlib.util.spec_from_file_location("io", "r3-red-fox-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)

STEP = {
    "thigh-h": 0.036, "shank-h": 0.006, "metatarsus-h": -0.005, "paw-h": -0.001,
    "scapula-f": 0.030, "humerus-f": 0.014, "forearm-f": -0.007, "metacarpus-f": -0.004, "paw-f": -0.001,
    "ear-": 0.031,
}


def with_z(bone):
    for stem, z in STEP.items():
        if bone["id"].startswith(stem) and bone["id"][-1] in "nf":
            out = {}
            for key, value in bone.items():
                if key == "z":
                    continue
                out[key] = value
                if key == "y":
                    out["z"] = -z if bone["id"].endswith("n") else z
            return out
    return bone


asset = io.load()
rig = {**asset["rig"], "bones": [with_z(b) for b in asset["rig"]["bones"]]}
print(sum(1 for b in rig["bones"] if "z" in b), "bones with a lateral position")
if "--candidate" in sys.argv:
    doc = json.load(open(io.FILE, encoding="utf-8"))
    doc["asset"]["rig"] = rig
    json.dump(doc, open("🗑️generated/r4-red-fox/🔣️.json", "w", encoding="utf-8"), ensure_ascii=False)
    print("candidate written")
if "--apply" in sys.argv:
    io.put({"rig": rig})
    print("applied")
