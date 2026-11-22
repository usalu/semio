"""🚪 Rig rework of vehicles/delivery-van: the rear door leaves rest closed (rest twist about their vertical hinge bone) and open by a limited twist of 0 to 180 degrees; poses no longer close them. `python3 rework-delivery-van-rig.py <in 🔣️.json> <out 🔣️.json>`."""
import json, sys

REST = {"rear-door-r": -90, "rear-door-l": 90}
LIMIT = {"rear-door-r": (0, 180), "rear-door-l": (-180, 0)}
OPEN = {"rear-door-r": 90, "rear-door-l": -90}

source, target = sys.argv[1:3]
raw = open(source, encoding="utf8").read()
document = json.loads(raw)
indent = 1 if json.dumps(document, ensure_ascii=False, indent=1) == raw.rstrip("\n") else 2
asset = document["asset"]
for bone in asset["rig"]["bones"]:
    if bone["id"] in REST:
        bone["twistDeg"] = REST[bone["id"]]
        bone["joint"]["twistMinDeg"], bone["joint"]["twistMaxDeg"] = LIMIT[bone["id"]]
for pose in asset["poses"]:
    for bone in REST:
        pose["bones"].pop(bone, None)
    if pose["id"] == "rear-doors-open":
        for bone, twist in OPEN.items():
            pose["bones"][bone] = {"twistDeg": twist}
open(target, "w", encoding="utf8").write(json.dumps(document, ensure_ascii=False, indent=indent) + ("\n" if raw.endswith("\n") else ""))
print("written", target, "indent", indent)
