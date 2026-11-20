"""📏 Round 3, appliances/gas-boiler: sets the thermal `areaM2` of the casing and the new hidden regions to the hand calculation of the modelled parts (casing capped at the C16 factor 1.5 of the committed 0.9). Re-reads the catalog file, changes only these numbers in place and keeps the formatting.

Usage: python3 r3-gas-boiler-areas.py
"""
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-boiler/🔣️.json"))
AREAS = {"casing": 1.35, "heat-exchanger": 0.34, "fan": 0.04, "expansion-vessel": 0.18}


def main():
    """🖊️ Replaces the areaM2 value inside each named region block of the thermal member."""
    text = open(CATALOG, encoding="utf-8").read()
    start = text.index('\n    "thermal": {')
    for rid, value in AREAS.items():
        i = text.index(f'"id": "{rid}"', start)
        m = re.compile(r'"areaM2": [0-9.e-]+').search(text, i)
        text = text[:m.start()] + f'"areaM2": {value}' + text[m.end():]
    regions = {r["id"]: r["areaM2"] for r in json.loads(text)["asset"]["thermal"]["regions"]}
    for rid, value in AREAS.items():
        if regions[rid] != value:
            raise SystemExit(f"{rid} not set")
    open(CATALOG, "w", encoding="utf-8").write(text)
    print("areaM2 set:", AREAS)


if __name__ == "__main__":
    main()
