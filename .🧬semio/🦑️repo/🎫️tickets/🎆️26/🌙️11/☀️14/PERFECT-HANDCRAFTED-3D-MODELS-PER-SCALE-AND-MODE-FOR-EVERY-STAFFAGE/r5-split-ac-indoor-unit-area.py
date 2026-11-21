"""📐 Round 5, appliances/split-ac-indoor-unit: the `intake-grille` area as the outer area of the real open lattice measured on the 1:50 model (0.0829 -> 0.11 m2, factor 1.33, within C16); a one-value edit of the catalog file, re-read right before writing.

Usage: python3 r5-split-ac-indoor-unit-area.py
"""
import json
import re

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/split-ac-indoor-unit/🔣️.json"
text = open(CATALOG, encoding="utf-8").read()
start = text.index('"id": "intake-grille"')
m = re.compile(r'"areaM2": 0\.0829').search(text, start)
assert m and m.start() - start < 3000, "intake-grille areaM2 not found next to its id"
new = text[: m.start()] + '"areaM2": 0.11' + text[m.end():]
regions = {r["id"]: r for r in json.loads(new)["asset"]["thermal"]["regions"]}
assert regions["intake-grille"]["areaM2"] == 0.11
open(CATALOG, "w", encoding="utf-8").write(new)
print("intake-grille areaM2 0.0829 -> 0.11")
