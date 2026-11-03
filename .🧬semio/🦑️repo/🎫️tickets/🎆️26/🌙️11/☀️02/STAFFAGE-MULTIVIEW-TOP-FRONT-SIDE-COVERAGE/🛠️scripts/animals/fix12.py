import json
p = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/horse/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
n = 0
for s in d["asset"]["representations"]["500"]["geometry"]["plan"]:
    if s["bone"] in ("ear-n", "ear-f") and s["kind"] == "capsule":
        s["x1"] = round(s["x1"] - 0.06, 4); n += 1
open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
print(n)
