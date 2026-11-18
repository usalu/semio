"""🔩 Ops that set keys of the `solid` of drawn shapes. python3 humansb-B-solidset.py <slug> <scale> <view> <out.json> '{"axisZ":-0.037}' id,id,...  (existing solid keys are merged)"""
import json, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
slug, scale, view, out, setj, ids = sys.argv[1:7]
g = json.load(open(CAT + slug + "/🔣️.json"))["asset"]["representations"][scale]["geometry"][view]
ops = []
for i in ids.split(","):
    s = next(x for x in g if x["id"] == i)
    sol = dict(s["solid"]); sol.update(json.loads(setj))
    ops.append({"op": "shape-set", "scale": scale, "view": view, "id": i, "set": {"solid": sol}})
json.dump(ops, open(out, "w"), indent=1)
