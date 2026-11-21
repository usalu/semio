"""🧪 Writes the round 5 candidate document of `humans/jogger` or `humans/running-child`: the catalog asset with the redrawn 2D geometry of `🗑️generated/r5-<slug>/geometry.json` (when present) and the volumes of `volumes/humans/<slug>.json`, to `🗑️generated/r5-<slug>/🔣️.json`, for `r3-validate.ts --file` and `r3-2d.ts --file`. Nothing in the catalog changes.

Usage: python3 r5-runners-candidate.py <slug>
"""
import json
import os
import sys

here = os.path.dirname(os.path.abspath(__file__))
slug = sys.argv[1]
catalog = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/%s/🔣️.json" % slug
document = json.load(open(catalog, encoding="utf-8"))
out = os.path.join(here, "🗑️generated", "r5-" + slug)
geometry = os.path.join(out, "geometry.json")
if os.path.exists(geometry):
    for scale, views in json.load(open(geometry, encoding="utf-8")).items():
        for view, shapes in views.items():
            document["asset"]["representations"][scale]["geometry"][view] = shapes
document["asset"]["volumes"] = json.load(open(os.path.join(here, "volumes", "humans", slug + ".json"), encoding="utf-8"))["volumes"]
os.makedirs(out, exist_ok=True)
json.dump(document, open(os.path.join(out, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False)
print(os.path.join(out, "🔣️.json"))
