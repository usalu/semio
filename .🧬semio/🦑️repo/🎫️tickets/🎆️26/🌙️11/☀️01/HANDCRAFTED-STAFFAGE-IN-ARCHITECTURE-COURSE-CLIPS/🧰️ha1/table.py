import json, sys, os, re
BASE = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances"
ASSETS = "circulation-pump decentralised-hrv-fan desktop-monitor duct-fan electric-fan-heater electric-kettle evaporative-humidifier gas-boiler gas-hob hot-water-cylinder kitchen-extractor-hood".split()
for a in ASSETS:
    d = json.load(open(os.path.join(BASE, a, "🔣️.json"), encoding="utf-8"))["asset"]
    raw = open(os.path.join(BASE, a, "🔣️.json"), encoding="utf-8").read()
    for rep in ["50", "100", "200"]:
        R = d["representations"][rep]
        if R["status"] != "authored": print(a, rep, "UNAV"); continue
        vr = R["validRange"]; p = R["pens"]
        print(f'{a:24s} {rep:4s} vr {vr["minDenominator"]}-{vr["maxDenominator"]} pens b{p.get("body")} d{p.get("detail")} t{p.get("texture")} g{p.get("glazing")} gr{p.get("ground")} hatch{len(R["hatches"])}')
    for m in re.finditer(r'[^"]{0,60}(none mm|keiner mm|null mm|None mm)[^"]{0,30}', raw):
        print("  TEXT:", m.group(0)[:140])
    rd = d["representations"]["50"]["decisions"]
