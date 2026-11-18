"""🖐️ Ops that widen the finger capsules of the volume of one scale until neighbouring fingers touch (radius at least half the spacing of the finger axes). python3 humansb-B-fingers.py <slug> <scale> <out.json>"""
import json, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
slug, scale, out = sys.argv[1:4]
body = json.load(open(CAT + slug + "/🔣️.json"))["asset"]["volumes"][scale]["body"]
fingers = [s for s in body if s.get("kind") == "capsule" and "finger" in s["id"]]
ops = []
for hand in ("hand-l", "hand-r"):
    own = sorted([s for s in fingers if s["bone"] == hand and not s["id"].endswith("-tip")], key=lambda s: s["p0"][2])
    gaps = [b["p0"][2] - a["p0"][2] for a, b in zip(own, own[1:])]
    if not gaps: continue
    need = max(gaps) / 2 * 1.04
    for s in [x for x in fingers if x["bone"] == hand]:
        ops.append({"op": "solid-set", "scale": scale, "id": s["id"], "set": {"r0": round(max(s["r0"], need), 5), "r1": round(max(s["r1"], need * 0.95), 5)}})
json.dump(ops, open(out, "w"), indent=1)
