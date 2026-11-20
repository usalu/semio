"""✏️ Applies the 2D redraw of r3-red-fox-2d.ts (🗑️generated/r3-red-fox/ops2d.json) to `representations` of the red fox asset: per shape the listed geometry members are removed and the given ones set, in the member order the file uses (common members, geometry, solid); ids, roles, bones, tones and the order of shapes stay. Without --apply only the candidate document 🗑️generated/r3-red-fox/🔣️.json is written.

Usage: python3 r3-red-fox-2d.py [--apply]
"""
import importlib.util
import json
import sys

spec = importlib.util.spec_from_file_location("io", "r3-red-fox-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)
reps = io.load()["representations"]
DIMENSIONS = {"heightM": 0.587, "widthM": 1.1, "depthM": 0.152, "massKg": 6}
count = 0
for op in json.load(open("🗑️generated/r3-red-fox/ops2d.json", encoding="utf-8")):
    shapes = reps[str(op["scale"])]["geometry"][op["view"]]
    index = next(i for i, s in enumerate(shapes) if s["id"] == op["id"])
    if op.get("remove"):
        del shapes[index]
        continue
    shape = {k: v for k, v in shapes[index].items() if k not in op.get("del", [])}
    kept = dict(shape)
    kept.update({k: op["set"][k] for k in ("kind", "points", "closed") if k in op["set"]})
    if "solid" in op["set"]:
        kept["solid"] = op["set"]["solid"]
    shapes[index] = kept
    count += 1
doc = json.load(open(io.FILE, encoding="utf-8"))
doc["asset"]["representations"] = reps
doc["asset"]["dimensions"] = DIMENSIONS
json.dump(doc, open("🗑️generated/r3-red-fox/🔣️.json", "w", encoding="utf-8"), ensure_ascii=False)
print("shapes redrawn", count, "candidate written")
if "--apply" in sys.argv:
    io.put({"dimensions": DIMENSIONS, "representations": reps})
    print("applied")
