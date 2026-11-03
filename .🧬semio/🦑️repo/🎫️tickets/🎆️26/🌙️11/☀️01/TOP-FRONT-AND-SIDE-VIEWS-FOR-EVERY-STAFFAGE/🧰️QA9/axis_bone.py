"""QA9: point every revolved solid of an asset at the given axis bone (argv: asset-dir-kind asset bone [only-from-bone]). Atomic write."""
import json, os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧰️FW"))
from atomic_json import write_json_atomic

kind, asset, bone = sys.argv[1:4]
p = os.path.join(r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog", kind, asset, "🔣️.json")
doc = json.load(open(p, encoding="utf-8"))
n = 0
for rep in doc["asset"]["representations"].values():
    if rep["status"] != "authored":
        continue
    for vp in ("elevation", "front", "plan"):
        for s in rep["geometry"][vp]:
            if s.get("solid", {}).get("kind") == "revolved":
                s["solid"]["axisBone"] = bone
                n += 1
write_json_atomic(p, doc)
print(asset, bone, n)
