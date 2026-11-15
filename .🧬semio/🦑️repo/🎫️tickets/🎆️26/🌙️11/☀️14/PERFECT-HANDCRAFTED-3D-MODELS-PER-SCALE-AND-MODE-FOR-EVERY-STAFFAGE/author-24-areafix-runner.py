"""areaM2 corrections at the root of runner-with-blade-prosthesis (C16: non-body regions, within a factor 1.5, from the 1:50 model): socket 0.0959 -> 0.068, blade 0.09 -> 0.076."""
import json
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/runner-with-blade-prosthesis/🔣️.json"
d = json.load(open(P))
new = {"socket": (0.0959, 0.068), "blade": (0.09, 0.076)}
for r in d["asset"]["thermal"]["regions"]:
    if r["id"] in new and r["areaM2"] == new[r["id"]][0]:
        r["areaM2"] = new[r["id"]][1]
        print(r["id"], "->", r["areaM2"])
json.dump(d, open(P, "w"), indent=2, ensure_ascii=False)
open(P, "a").write("\n")
