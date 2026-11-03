import json, os
C = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
idx = json.load(open(os.path.join(C, "🔣️.json"), encoding="utf-8"))
ents = {e["id"]: e for e in idx["entries"]}
for a in ["walking-commuter", "horse", "city-car", "jogger", "dairy-cow", "tram"]:
    e = ents[a]
    print(a, "index clips:", e.get("clips"), "poses:", e.get("poses"))
    cat = e["category"]
    mp = json.load(open(os.path.join(C, [c for c in os.listdir(C) if os.path.isdir(os.path.join(C, c)) and os.path.isdir(os.path.join(C, c, a))][0], a, "🔣️.json"), encoding="utf-8"))["asset"]
    print("   manifest clips:", [c["id"] for c in mp.get("clips", [])][:50], "poses:", [p["id"] for p in mp.get("poses", [])][:20])
print(json.dumps(ents["walking-commuter"], ensure_ascii=False)[:800])
