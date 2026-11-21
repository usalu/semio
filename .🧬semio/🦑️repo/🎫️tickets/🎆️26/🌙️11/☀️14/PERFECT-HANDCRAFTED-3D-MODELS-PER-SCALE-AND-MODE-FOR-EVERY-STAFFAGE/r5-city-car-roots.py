"""🌱 Round 5 root values of vehicles/city-car outside `volumes`: the thermal areas of the regions the hollow body in white changed (C16, each within a factor 1.5 of the committed value, both regions passive), written into the catalog file by replacing only the `areaM2` number of each named region (the file is re-read right before writing). Usage: python3 r5-city-car-roots.py [--apply]"""
import glob
import json
import re
import sys

AREAS = {"body-panels": (15.4, 20.2), "interior": (5, 7.2)}
ASSET = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*vehicles/city-car/*.json")[0]


def patched(text):
    for region, (old, new) in AREAS.items():
        assert 1 / 1.5 <= new / old <= 1.5, region
        start = text.index(f'"id": "{region}",', text.index('"regions"', text.index('\n  "thermal"')))
        found = re.compile(r'"areaM2": ([0-9.]+)').search(text, start)
        assert float(found.group(1)) in (old, new), (region, found.group(1))
        text = text[:found.start(1)] + repr(new) + text[found.end(1):]
    json.loads(text)
    return text


text = patched(open(ASSET, encoding="utf-8").read())
regions = {r["id"]: r["areaM2"] for r in json.loads(text)["asset"]["thermal"]["regions"]}
print({k: regions[k] for k in AREAS})
if "--apply" in sys.argv:
    text = patched(open(ASSET, encoding="utf-8").read())
    open(ASSET, "w", encoding="utf-8").write(text)
    print("catalog written")
