"""Shows drawn shapes and volume solids of a humans asset by id substring. python3 humansa-show.py <slug> <scale> <view|solids> <idsub,...>"""
import json, sys
CAT="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
slug, sc, view, subs = sys.argv[1:5]
a = json.load(open(CAT + slug + "/🔣️.json"))["asset"]
subs = subs.split(",") if subs != "-" else [""]
items = a["volumes"][sc]["body"] if view == "solids" else a["representations"][sc]["geometry"][view]
for it in items:
    if any(x in it["id"] for x in subs):
        print(json.dumps(it, ensure_ascii=False, separators=(",", ":")))
