import json, glob, os
from collections import defaultdict

A = "C:/git/semio/🎓️teaching/🏛️architecture"
m = json.load(open(A + "/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json", encoding="utf8"))
d = defaultdict(set)
for k, v in m["casts"].items():
    d[v["asset"]].add((v.get("pose") or v.get("clip") or "-", v.get("viewpoint")))
cat = {}
for p in glob.glob(A + "/🧍️staffage/🗂️catalog/*/*/🔣️.json"):
    q = p.replace(chr(92), "/").split("/")
    cat[q[-2]] = q[-3]
for a, s in sorted(d.items()):
    print(a, cat.get(a, "MISSING"), sorted(s))
print("UNUSED", sorted(a for a in cat if a not in d))
