"""🔍 Compares the catalog `volumes` of an asset with its authoring source per scale and list (solid ids and whole-solid equality)."""
import glob
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
S = os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
for asset in sys.argv[1:]:
    category, slug = asset.split("/")
    path = glob.glob(os.path.join(S, "*" + category, slug, "🔣️.json"))[0]
    cat = json.load(open(path, encoding="utf-8"))["asset"]["volumes"]
    src = json.load(open(os.path.join(T, "volumes", asset + ".json"), encoding="utf-8"))["volumes"]
    for scale in sorted(set(cat) | set(src), key=int):
        a, b = cat.get(scale, {}), src.get(scale, {})
        for part in ("body", "thermal", "structure", "decisions"):
            x, y = a.get(part), b.get(part)
            if x != y:
                n = sum(1 for i, j in zip(x or [], y or []) if i != j) if isinstance(x, list) and isinstance(y, list) else -1
                print(asset, scale, part, "differs", len(x or []), len(y or []), "changed", n)
    print(asset, "compared", path[-60:])
