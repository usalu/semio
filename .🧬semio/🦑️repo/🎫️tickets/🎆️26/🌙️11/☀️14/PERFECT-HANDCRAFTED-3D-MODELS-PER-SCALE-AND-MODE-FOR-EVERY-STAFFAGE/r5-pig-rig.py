"""🐗 Round 5 root fix of animals/pig: the limb and ear chains leave the midline at their real joint centres (hip joint 0.085 m, dorsal border of the scapula 0.05 m and shoulder joint a further 0.045 m from the median plane, ear base 0.05 m; 100 kg finishing pig, pelvis 0.17 m between the acetabula, chest 0.30 m wide over the shoulder joints). Only the lateral offsets `z` of these bones change; solids are stated in world metres and are rebuilt by the model scripts on the new frames.

Usage: python3 r5-pig-rig.py
@see ./r3-animals-pig.md
"""
import importlib.util

spec = importlib.util.spec_from_file_location("io", "r3-pig-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)

Z = {"thigh-h": 0.085, "scapula-f": 0.05, "humerus-f": 0.045, "ear-": 0.05}


def lateral(bone):
    for stem, z in Z.items():
        for side, sign in (("n", -1), ("f", 1)):
            if bone == stem + side:
                return round(sign * z, 4)
    return None


asset = io.load()
rig = asset["rig"]
bones = []
for b in rig["bones"]:
    z = lateral(b["id"])
    if z is None:
        bones.append(b)
        continue
    keys = list(b.keys())
    out = {}
    for k in keys:
        if k == "z":
            continue
        out[k] = b[k]
        if k == "y":
            out["z"] = z
    bones.append(out)
io.put({"rig": {**rig, "bones": bones}})
print("rig lateral offsets:", {b["id"]: b.get("z") for b in bones if "z" in b})
