import json, collections
E = json.load(open("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌤️environments/🔣️.json", encoding="utf-8"))
print(E.keys())
ids = [p["id"] for p in E["presets"]]
print(len(ids), [k for k, n in collections.Counter(ids).items() if n > 1])
for p in E["presets"]:
    print(p["id"], p.get("title"), p["environment"].get("airTempC"), p["environment"].get("meanRadiantTempC"), p["environment"].get("relativeHumidity"), p["environment"].get("windMS"), p["environment"].get("solarWM2"), p["environment"].get("diffuseFraction"), p["environment"].get("pressurePa"), sep=" | ")
