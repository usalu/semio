import json, os, re, sys
BASE = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances"
ASSETS = "circulation-pump decentralised-hrv-fan desktop-monitor duct-fan electric-fan-heater electric-kettle evaporative-humidifier gas-boiler gas-hob hot-water-cylinder kitchen-extractor-hood".split()
def walk(o, path, out):
    if isinstance(o, dict):
        if "en" in o and isinstance(o["en"], str):
            if "de" not in o: out.append(("MISSING_DE", path))
            elif o["de"] == o["en"]: out.append(("SAME", path, o["en"][:60]))
            if set(o.keys()) - {"en", "de"}: out.append(("EXTRA", path, list(o.keys())))
        for k, v in o.items(): walk(v, path + [k], out)
    elif isinstance(o, list):
        for i, v in enumerate(o): walk(v, path + [i], out)
for a in ASSETS:
    raw = open(os.path.join(BASE, a, "🔣️.json"), encoding="utf-8").read()
    d = json.loads(raw)["asset"]
    out = []; walk(d, [], out)
    bad = re.findall(r'(TODO|TBD|human contour|lorem|XXX)', raw)
    print(a, len(out), out[:6], "flags:", bad)
