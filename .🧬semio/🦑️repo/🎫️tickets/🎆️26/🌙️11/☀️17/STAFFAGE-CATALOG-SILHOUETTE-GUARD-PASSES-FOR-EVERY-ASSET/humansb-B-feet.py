"""🦶 Ops file that replaces the flat front foot and toe polygons of one side by the plan slices (side-profile boxes with lateral spans), so a pitched foot projects correctly. python3 humansb-B-feet.py <slug> <scale> <l|r|lr> <out.json>"""
import json, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
slug, scale, sides, out = sys.argv[1:5]
g = json.load(open(CAT + slug + "/🔣️.json"))["asset"]["representations"][scale]["geometry"]
ops = []
for side in sides:
    for part in ("foot", "toe"):
        key = f"{part}-{side}"
        slices = [s for s in g["plan"] if s["id"].startswith(key + "-")]
        if not slices: continue
        ops.append({"op": "shape", "scale": scale, "view": "front", "id": key, "shape": slices[0]})
        prev = slices[0]["id"]
        for s in slices[1:]:
            ops.append({"op": "shape-add", "scale": scale, "view": "front", "after": prev, "shape": s}); prev = s["id"]
json.dump(ops, open(out, "w"), indent=1, ensure_ascii=False)
