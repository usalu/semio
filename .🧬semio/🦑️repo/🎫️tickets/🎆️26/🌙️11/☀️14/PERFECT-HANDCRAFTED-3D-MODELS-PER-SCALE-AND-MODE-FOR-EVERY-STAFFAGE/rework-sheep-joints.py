"""🐑 Sheep skeleton, last contacts: the fore digits reach the pedal bones at 1:50, the scapula bars of 1:200 and 1:500 lie on the ribs, and the 1:500 ribs end on a sternum; written to the volumes source.

Usage: python3 rework-sheep-joints.py
"""
import copy
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "animals", "sheep.json")
SCAPULA_TO_RIBS = {"200": 0.006, "500": 0.008}


def dump():
    names = {}
    exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("def member_end")[0], names)
    return names["dump"]


def main():
    source = json.load(open(SOURCE, encoding="utf-8"))
    volumes = source["volumes"]
    if any(solid["id"] == "sternum" for solid in volumes["500"]["structure"]):
        raise SystemExit("already applied")
    for solid in volumes["50"]["structure"]:
        if "-fore-digit-" in solid["id"]:
            solid["sections"][-1]["at"][0], solid["sections"][-1]["at"][1] = 0.071, -0.004
    for scale, shift in SCAPULA_TO_RIBS.items():
        for solid in volumes[scale]["structure"]:
            if solid.get("bone", "").startswith("scapula-f") and solid.get("part") == "member":
                sign = -1 if solid["bone"].endswith("n") else 1
                for part in solid["sections"]:
                    part["at"][2] = round(part["at"][2] - sign * shift, 5)
    sternum = copy.deepcopy(next(solid for solid in volumes["200"]["structure"] if solid["id"] == "sternum"))
    for part in sternum["sections"]:
        part["rx"], part["ry"] = 0.009, 0.008
    sternum["slender"] = True
    structure = volumes["500"]["structure"]
    structure.insert(next(i for i, solid in enumerate(structure) if solid["id"] == "near-rib-2"), sternum)
    open(SOURCE, "w", encoding="utf-8").write('{"volumes":' + dump()(volumes, 0) + "}\n")


main()
