"""Prints volume solids and drawn shapes of one asset scale (optionally filtered by id substring): python3 show.py <category/asset> <scale> [filter]"""
import json, sys
F = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/" + sys.argv[1] + "/🔣️.json"
a = json.load(open(F))["asset"]; sc = sys.argv[2]; q = sys.argv[3] if len(sys.argv) > 3 else ""
def sh(s): return json.dumps({k: v for k, v in s.items() if k not in ("tone", "material", "region", "regions", "bone", "z")}, ensure_ascii=False)[:int(900)]
if sc in a["volumes"]:
    for l in ("body", "thermal", "structure"):
        pass
    for s in a["volumes"][sc]["body"]:
        if q in s["id"]: print("VOL", s.get("bone"), sh(s))
g = a["representations"][sc]["geometry"]
for v in g:
    for s in g[v]:
        if q in s["id"]: print(v, s.get("bone"), sh(s))
