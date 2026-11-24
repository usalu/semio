"""🧪️ Builds a candidate document of the walking commuter from an untouched copy for measuring: python3 wc-cand.py <before.json> <rest.json> <cand.json> <group,…> [toe=capsule|prism]"""
import json
import sys

import wc_redraw

before, rest, out, groups = sys.argv[1:5]
toe = (sys.argv[5] if len(sys.argv) > 5 else "toe=capsule").split("=")[1]
doc = json.load(open(before, encoding="utf8"))
shapes = wc_redraw.redraw(doc["asset"], json.load(open(rest, encoding="utf8")), groups.split(","), toe)
for (scale, view, shape_id), shape in shapes.items():
    items = doc["asset"]["representations"][str(scale)]["geometry"][view]
    ids = [s["id"] for s in items]
    if shape_id in ids:
        items[ids.index(shape_id)] = shape
    else:
        items.insert(ids.index(wc_redraw.FAR[shape_id]), shape)
json.dump(doc, open(out, "w", encoding="utf8"), ensure_ascii=False)
print(f"[DEBUG] {len(shapes)} shapes redrawn ({groups}, toe {toe})")
