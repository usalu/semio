"""🦴 Root fix of the city-car rig and envelope: axles to the real overhangs (front 0.80 m, rear 0.65 m at the unchanged wheelbase of 2.45 m), the body centre of mass moved with them (static front share stays 62 %), and the lateral envelope widened to the real body width of 1.68 m plus door mirrors (1.90 m). Re-reads the asset right before writing and replaces only the members `dimensions` and `rig`."""
import glob
import json
import re

ASSET = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*vehicles/city-car/*.json")[0]
AXLE_F_X, AXLE_R_X, CHASSIS_COM_X, DEPTH_M = 1.15, -1.3, 0.28, 1.9


def member(text, key):
    found = re.search(r'\n  "' + key + r'"\s*:\s*', text)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), index + 1
        index += 1


def dumped(value):
    lines = json.dumps(value, indent=1, ensure_ascii=False).split("\n")
    return "\n".join(line if i == 0 else "  " + line for i, line in enumerate(lines))


def replace(text, key, value):
    start, end = member(text, key)
    return text[:start] + dumped(value) + text[end:]


text = open(ASSET, encoding="utf-8").read()
asset = json.loads(text)["asset"]
bones = {bone["id"]: bone for bone in asset["rig"]["bones"]}
old = {"axle-f.x": bones["axle-f"]["x"], "axle-r.x": bones["axle-r"]["x"], "chassis.comX": bones["chassis"]["mass"]["comX"], "depthM": asset["dimensions"]["depthM"]}
bones["axle-f"]["x"] = AXLE_F_X
bones["axle-r"]["x"] = AXLE_R_X
bones["chassis"]["mass"]["comX"] = CHASSIS_COM_X
asset["dimensions"]["depthM"] = DEPTH_M
text = replace(replace(text, "dimensions", asset["dimensions"]), "rig", asset["rig"])
json.loads(text)
open(ASSET, "w", encoding="utf-8").write(text)
print(old, "->", {"axle-f.x": AXLE_F_X, "axle-r.x": AXLE_R_X, "chassis.comX": CHASSIS_COM_X, "depthM": DEPTH_M})
