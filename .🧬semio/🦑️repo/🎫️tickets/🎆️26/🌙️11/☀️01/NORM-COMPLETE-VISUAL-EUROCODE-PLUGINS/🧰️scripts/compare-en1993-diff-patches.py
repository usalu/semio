import json
import re

json_path = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json"
ts_path = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts"
doc = json.load(open(json_path, encoding="utf-8"))
text = open(ts_path, encoding="utf-8").read()
names = [
    "En1993MaterialDuctilityPatch",
    "En1993FormingGainPatch",
    "En1993BucklingLengthPatch",
    "En1993FatigueStressPatch",
    "En1993FastenedJointPatch",
]
for name in names:
    props = list(doc["$defs"][name]["properties"])
    block = re.search(rf"export interface {name} \{{(.*?)\n\}}", text, re.S)
    fields = re.findall(r"(\w+)\?", block.group(1)) if block else []
    print(name, "ok" if props == fields else f"json={props} ts={fields}")
