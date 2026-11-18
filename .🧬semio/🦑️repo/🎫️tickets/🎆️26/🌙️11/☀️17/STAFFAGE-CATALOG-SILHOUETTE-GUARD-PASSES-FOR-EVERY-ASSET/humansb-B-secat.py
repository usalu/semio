"""📐 Op that moves one loft section along its first `at` coordinate. python3 humansb-B-secat.py <slug> <scale> <solidId> <sectionIndex> <delta> <out.json>"""
import json, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
slug, scale, sid, idx, delta, out = sys.argv[1:7]
body = json.load(open(CAT + slug + "/🔣️.json"))["asset"]["volumes"][scale]["body"]
s = next(x for x in body if x["id"] == sid)
secs = json.loads(json.dumps(s["sections"]))
secs[int(idx)]["at"][0] = round(secs[int(idx)]["at"][0] + float(delta), 5)
json.dump([{"op": "solid-set", "scale": scale, "id": sid, "set": {"sections": secs}}], open(out, "w"), indent=1)
