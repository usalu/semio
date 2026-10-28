import json
c = json.load(open("🗑️generated/model-r3/changed.json", encoding="utf-8"))
for k in ("355 0.45 0.6 0.16", "355 0.45 0.3 0.16", "5 0.45 0.6 0.16", "354 0.45 0.6 0.16", "354.5 0.45 0.6 0.16", "345 0.45 0.6 0.16", "300 0.45 0.6 0.16", "90 0.45 0.6 0.16"):
    print(k, c.get(k))
