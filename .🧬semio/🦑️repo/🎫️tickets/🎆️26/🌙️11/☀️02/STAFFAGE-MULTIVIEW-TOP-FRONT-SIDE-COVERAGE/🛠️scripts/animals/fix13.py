import json
p = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/labrador-dog/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
for s in d["asset"]["representations"]["500"]["geometry"]["front"]:
    if s["id"] == "head" and s["bone"] == "head":
        s["points"] = [[a, round(r * 0.066 / 0.062, 4)] for a, r in s["points"]]
open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
