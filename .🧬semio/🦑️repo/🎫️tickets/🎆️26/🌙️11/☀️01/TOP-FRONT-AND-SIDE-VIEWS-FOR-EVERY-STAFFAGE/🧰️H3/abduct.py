import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from humanlib import *

JOBS = {
    "runner-with-blade-prosthesis": {"hands-on-hips": {"arm-l": 42, "arm-r": -42, "forearm-l": -36, "forearm-r": 36}},
    "skateboarder": {"cruising": {"upper-arm-l": 30, "upper-arm-r": -30}, "ollie-air": {"upper-arm-l": 45, "upper-arm-r": -45}},
}
for aid, poses in JOBS.items():
    doc = load(aid)
    for p in doc["asset"]["poses"]:
        for bone, v in poses.get(p["id"], {}).items():
            p["bones"][bone]["abductionDeg"] = v
    save(aid, doc)
    print(aid, "ok")

ZS = {"runner-with-blade-prosthesis": {"arm-l": .11, "arm-r": -.11}, "skateboarder": {"upper-arm-l": .11, "upper-arm-r": -.11}}
for aid, zs in ZS.items():
    doc = load(aid)
    for b in doc["asset"]["rig"]["bones"]:
        if b["id"] in zs: b["z"] = zs[b["id"]]
    save(aid, doc)
    print(aid, "z ok")
