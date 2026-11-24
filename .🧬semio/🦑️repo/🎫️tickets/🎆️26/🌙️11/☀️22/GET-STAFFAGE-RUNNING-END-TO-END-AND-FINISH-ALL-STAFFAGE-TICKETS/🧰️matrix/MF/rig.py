"""🦴️ Prints the rig of an asset (bones with parent, rest angle, length, origin), the poses with the bones they turn or move, the anchors, and per scale and view the number of flat and solid shapes per bone: rig.py <asset id|file.json>"""
import glob, json, sys
from collections import Counter

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
arg = sys.argv[1]
a = json.load(open(arg if arg.endswith(".json") else glob.glob(f"{CATALOG}/*/{arg}/🔣️.json")[0]))["asset"]
print("dimensions", a["dimensions"], "default", a["presentation"]["defaultPose"])
print("root", a["rig"].get("root"))
for b in a["rig"]["bones"]:
    print("bone", json.dumps(b, ensure_ascii=False)[:230])
for p in a["poses"]:
    print("pose", p["id"], json.dumps({k: v for k, v in p.items() if k not in ("id", "label", "description")}, ensure_ascii=False)[:420])
for c in a.get("clips", []):
    print("clip", c["id"], sorted({t.get("bone", "root") for t in c.get("tracks", [])})[:12])
for x in a["anchors"]:
    print("anchor", json.dumps(x, ensure_ascii=False)[:200])
for key, rep in a["representations"].items():
    if "geometry" not in rep:
        print(key, rep.get("status"))
        continue
    for view in ("elevation", "front", "plan"):
        c = Counter()
        for s in rep["geometry"][view]:
            c[(s.get("bone"), "solid:" + s["solid"]["kind"][:3] + s["solid"].get("sweep", "") if "solid" in s else ("tip" if s.get("followTip") else "flat"))] += 1
        print(key, view, len(rep["geometry"][view]), " ".join(f"{b}/{k}×{n}" for (b, k), n in sorted(c.items(), key=str)))
