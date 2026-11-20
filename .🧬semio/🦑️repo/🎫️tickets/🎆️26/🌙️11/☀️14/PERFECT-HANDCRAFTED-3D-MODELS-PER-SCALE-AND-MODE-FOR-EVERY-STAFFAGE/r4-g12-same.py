"""🔍 Compares the catalog `volumes` of an asset with the author source `volumes/<category>/<slug>.json`: python3 r4-g12-same.py humans/courier-with-hand-truck"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CAT = {"humans": "👤️humans"}
category, slug = sys.argv[1].split("/")
catalog = json.load(open(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/{CAT[category]}/{slug}/🔣️.json", encoding="utf-8"))["asset"]["volumes"]
source = json.load(open(os.path.join(T, "volumes", category, f"{slug}.json"), encoding="utf-8"))
source = source.get("volumes", source)
for scale in sorted(set(catalog) | set(source), key=int):
    a, b = catalog.get(scale), source.get(scale)
    print(scale, "same" if a == b else "DIFFERENT", {k: len(v) for k, v in (a or {}).items() if isinstance(v, list)}, {k: len(v) for k, v in (b or {}).items() if isinstance(v, list)})
