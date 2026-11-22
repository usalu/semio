"""🐑 Sheep rib cage at 1:50 and 1:100: the cranial ribs become as narrow as the chest between the elbows really is (half-widths 45, 55, 66, 78, 90, 102, 112, 122 mm for ribs 1 to 8), and the 1:50 scapula blade returns to its bone line, clear of the ribs by the muscle layer; written to the volumes source.

Usage: python3 rework-sheep-ribs.py
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "animals", "sheep.json")
NARROW = {"1": 0.82, "2": 0.79, "3": 0.78, "4": 0.80, "5": 0.83, "6": 0.88, "7": 0.92, "8": 0.96}
SCAPULA_BACK = 0.0155


def main():
    names = {}
    exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("def member_end")[0], names)
    source = json.load(open(SOURCE, encoding="utf-8"))
    for scale in ("50", "100"):
        for solid in source["volumes"][scale]["structure"]:
            name = solid["id"]
            if "-rib-" in name and name.rsplit("-", 1)[1] in NARROW:
                for part in solid["sections"]:
                    part["at"][2] = round(part["at"][2] * NARROW[name.rsplit("-", 1)[1]], 5)
    for solid in source["volumes"]["50"]["structure"]:
        if solid.get("bone", "").startswith("scapula-f") and solid.get("part") == "member":
            sign = -1 if solid["bone"].endswith("n") else 1
            for part in solid["sections"]:
                part["at"][2] = round(part["at"][2] + sign * SCAPULA_BACK, 5)
    open(SOURCE, "w", encoding="utf-8").write('{"volumes":' + names["dump"](source["volumes"], 0) + "}\n")


main()
