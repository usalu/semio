import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from humanlib import *

# abductionDeg: left positive, right negative (inward = towards the symmetry plane)
BACK_N = {"arm-r": -20, "forearm-r": 35, "arm-l": -15, "forearm-l": -10}
GUITAR = {"upper-arm-l": -14, "forearm-l": -8, "upper-arm-r": 10, "forearm-r": 25}
POSES = {
    "pregnant-person": {"hand-on-back": BACK_N, "waddle-contact": BACK_N,
                        "cradling-belly": {"arm-l": -15, "forearm-l": -12, "arm-r": 15, "forearm-r": 12},
                        "stretching-back": {"arm-l": 15, "forearm-l": -30, "arm-r": -15, "forearm-r": 30}},
    "seated-guitarist": {"strumming": GUITAR, "strum-up": GUITAR, "fretting-high": GUITAR,
                         "tuning": {"upper-arm-l": -14, "forearm-l": -8, "upper-arm-r": 14, "forearm-r": 6},
                         "resting": {"upper-arm-l": -12, "forearm-l": -10, "upper-arm-r": 10, "forearm-r": 20}},
    "seated-office-worker": {"typing": {"arm-l": -4, "forearm-l": -6, "arm-r": 4, "forearm-r": 6},
                             "reading-screen": {"arm-r": 6, "forearm-r": 10},
                             "leaning-back": {"arm-l": -15, "forearm-l": -25}},
    "seated-reader": {"seated-upright": {"arm-l": -10, "forearm-l": -12, "arm-r": 4, "forearm-r": 14},
                      "reclined-reading": {"arm-l": -10, "forearm-l": -12, "arm-r": 4, "forearm-r": 14},
                      "reaching-for-cup": {"arm-l": -8, "forearm-l": -12, "arm-r": -6}},
}
Z = {"pregnant-person": {"arm-l": .152, "arm-r": -.152}, "seated-guitarist": {"upper-arm-l": .218, "upper-arm-r": -.218},
     "seated-office-worker": {"arm-l": .145, "arm-r": -.145}, "seated-reader": {"arm-l": .14, "arm-r": -.14}}
for aid, poses in POSES.items():
    doc = load(aid)
    for p in doc["asset"]["poses"]:
        for bone, v in poses.get(p["id"], {}).items():
            p["bones"].setdefault(bone, {})["abductionDeg"] = v
    for b in doc["asset"]["rig"]["bones"]:
        if b["id"] in Z[aid]: b["z"] = Z[aid][b["id"]]
    save(aid, doc)
    print(aid, "ok")
