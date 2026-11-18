"""Shifts the points of poly shapes of a view: python3 humansa-shift.py <slug> <scale> <view> <id> <dx> <dy> <ops.json>  (appends a shape-set op with the shifted points)"""
import json, os, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
slug, sc, view, sid, dx, dy, out = sys.argv[1:8]
a = json.load(open(CAT + slug + "/🔣️.json"))["asset"]
s = next(x for x in a["representations"][sc]["geometry"][view] if x["id"] == sid)
pts = [[round(p[0] + float(dx), 5), round(p[1] + float(dy), 5)] for p in s["points"]]
ops = json.load(open(out)) if os.path.exists(out) else []
ops.append({"op": "shape-set", "scale": sc, "view": view, "id": sid, "set": {"points": pts}})
json.dump(ops, open(out, "w"), indent=1)
