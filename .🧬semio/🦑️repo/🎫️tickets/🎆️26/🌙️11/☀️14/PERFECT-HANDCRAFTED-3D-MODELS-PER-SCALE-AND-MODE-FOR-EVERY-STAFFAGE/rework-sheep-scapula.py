"""🐑 Sheep scapula on the ribs: the blade of 1:50 and the bar of 1:500 move further towards the rib cage (15.5 mm and 25 mm from the bone line in all) until they lie on the ribs; written to the volumes source.

Usage: python3 rework-sheep-scapula.py
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "animals", "sheep.json")
MORE = {"50": 0.009, "500": 0.005}


def main():
    names = {}
    exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("def member_end")[0], names)
    source = json.load(open(SOURCE, encoding="utf-8"))
    for scale, shift in MORE.items():
        for solid in source["volumes"][scale]["structure"]:
            if solid.get("bone", "").startswith("scapula-f") and solid.get("part") == "member":
                sign = -1 if solid["bone"].endswith("n") else 1
                for part in solid["sections"]:
                    part["at"][2] = round(part["at"][2] - sign * shift, 5)
    open(SOURCE, "w", encoding="utf-8").write('{"volumes":' + names["dump"](source["volumes"], 0) + "}\n")


main()
