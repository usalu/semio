import sys, os
A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
sys.path.insert(0, A2)
import lib, jsonfmt

# wing roots sit beside the symmetry plane: Bone.z is the pivot of the lateral spread (near/right negative, far/left positive)
for aid, bones in (("mallard-duck", {"wing-n": -0.058, "wing-f": 0.058}), ("pigeon", {"humerus-n": -0.05, "humerus-f": 0.05})):
    A = lib.Asset(aid)
    doc = jsonfmt.read(A.path)
    for b in doc["asset"]["rig"]["bones"]:
        if b["id"] in bones:
            b["z"] = bones[b["id"]]
    jsonfmt.write(A.path, doc)
    print("z", aid)
