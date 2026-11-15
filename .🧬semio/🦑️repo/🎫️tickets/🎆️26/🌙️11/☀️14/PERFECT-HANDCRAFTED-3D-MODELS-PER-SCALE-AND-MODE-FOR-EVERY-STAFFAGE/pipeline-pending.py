import json, glob
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
pending = []
for f in sorted(glob.glob(f"{S}/🗂️catalog/*/*/🔣️.json")):
    a = json.load(open(f))["asset"]
    v = a.get("volumes")
    if not (v and any(s["status"] == "authored" for s in v.values())): pending.append(a["id"])
pending.sort()
json.dump({"pending": pending}, open(f"{S}/🧫️fixtures/🧊️model-catalog/🔣️.json", "w"), ensure_ascii=False, indent=1)
print(len(pending), "pending")
