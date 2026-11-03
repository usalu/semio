import sys
sys.path.insert(0, __file__.rsplit("\\", 1)[0].rsplit("/", 1)[0] + "/🧰️H4")
sys.path.insert(0, "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️H4")
from lib import load, save

PLAN = {
    "standing-adult": {"pointing": {"arm-r": -85}},
    "standing-phone-user": {},
    "tour-guide": {
        "turning-to-group": {"torso": ("t", 20), "head": ("t", 25)},
        "gesturing-to-group": {"upper-arm-r": -35},
        "waving": {"upper-arm-r": -50},
        "wave-swing-a": {"upper-arm-r": -50, "forearm-r": -8},
        "wave-swing-b": {"upper-arm-r": -50, "forearm-r": 8},
    },
    "sleeping-person": {
        "side": {"pelvis": ("t", 35), "torso": ("t", 25)},
        "curled": {"pelvis": ("t", 50), "torso": ("t", 35)},
    },
}
for aid, poses in PLAN.items():
    doc = load(aid)
    for p in doc["asset"]["poses"]:
        for b in list(p["bones"]):
            p["bones"][b].pop("abductionDeg", None); p["bones"][b].pop("twistDeg", None)
        for b, deg in poses.get(p["id"], {}).items():
            if isinstance(deg, tuple):
                p["bones"][b]["twistDeg"] = deg[1]
            else:
                p["bones"][b]["abductionDeg"] = deg
    save(aid, doc)
    print(aid, "ok")

