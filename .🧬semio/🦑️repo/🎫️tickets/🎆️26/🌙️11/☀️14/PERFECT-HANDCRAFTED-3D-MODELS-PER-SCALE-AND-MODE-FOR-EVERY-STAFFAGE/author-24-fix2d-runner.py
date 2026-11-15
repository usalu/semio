"""Root repair of runner-with-blade-prosthesis 1:50 plan: the shorts ellipses (r 79.5 mm at +-75 mm) reach |z| 0.1545 against 0.150 in the front view (shoulder width); r becomes 75.5 mm (thigh 75 mm plus 0.5 mm of cloth) so plan and front give the same width (C11)."""
import json
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/runner-with-blade-prosthesis/🔣️.json"
d = json.load(open(P))
plan = d["asset"]["representations"]["50"]["geometry"]["plan"]
n = 0
for s in plan:
    if s["id"] in ("shorts-l", "shorts-r") and s["kind"] == "ellipse" and s["rx"] > 0.077:
        s["rx"] = s["ry"] = 0.0755
        n += 1
json.dump(d, open(P, "w"), indent=2, ensure_ascii=False)
open(P, "a").write("\n")
print("shorts ellipses", n)
