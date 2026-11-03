import json, os, re
BASE = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances"
ASSETS = "circulation-pump decentralised-hrv-fan desktop-monitor duct-fan electric-fan-heater electric-kettle evaporative-humidifier gas-boiler gas-hob hot-water-cylinder kitchen-extractor-hood".split()
for a in ASSETS:
    d = json.load(open(os.path.join(BASE, a, "🔣️.json"), encoding="utf-8"))["asset"]
    for rep in ["50", "100", "200"]:
        R = d["representations"][rep]
        en = R["decisions"]["styles"]["en"]; p = R["pens"]
        m = {k: re.search(k + r" (?:pen )?(\d+\.\d+|none)", en) for k in ["body", "detail", "ground"]}
        for k, mm in m.items():
            if not mm: continue
            v = mm.group(1); pv = p.get(k if k != "body" else "body")
            if v == "none" and pv is None: continue
            if v != "none" and pv is not None and abs(float(v) - pv) < 1e-9: continue
            print(f"{a} {rep} {k}: text {v} vs pens {pv}")
