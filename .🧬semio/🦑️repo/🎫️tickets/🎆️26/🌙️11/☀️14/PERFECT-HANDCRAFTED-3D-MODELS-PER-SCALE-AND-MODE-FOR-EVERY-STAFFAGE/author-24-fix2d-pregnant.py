"""Root repair of pregnant-person 1:50 plan: the four arm polygons are 112 mm wide against 96 mm in the front view; their lateral half-width is scaled by 0.93 so that the whole-asset depth of plan (0.513 m) and front (0.499 m) agree (C11)."""
import json
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/pregnant-person/🔣️.json"
d = json.load(open(P))
plan = d["asset"]["representations"]["50"]["geometry"]["plan"]
n = 0
for s in plan:
    if s["id"] in ("upper-arm-l", "upper-arm-r", "forearm-l", "forearm-r") and s["kind"] == "poly" and not s.get("_scaled"):
        s["points"] = [[p[0], round(p[1] * 0.93, 4)] for p in s["points"]]
        n += 1
json.dump(d, open(P, "w"), indent=2, ensure_ascii=False)
open(P, "a").write("\n")
print("scaled", n)
