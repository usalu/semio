"""✏️ Applies the 2D redraw of r3-sheep-2d.ts (🗑️generated/r3-sheep/ops2d.json) to `representations` of the sheep asset: per shape the given members are set and the listed geometry members of other shape kinds removed; ids, roles, bones and the order of shapes stay.

Usage: python3 r3-sheep-2d.py
"""
import importlib.util
import json

spec = importlib.util.spec_from_file_location("io", "r3-sheep-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)
reps = io.load()["representations"]
count = 0
for op in json.load(open("🗑️generated/r3-sheep/ops2d.json", encoding="utf-8")):
    shape = next(s for s in reps[str(op["scale"])]["geometry"][op["view"]] if s["id"] == op["id"])
    for key in op.get("del", []):
        shape.pop(key, None)
    shape.update(op["set"])
    count += 1
io.put({"representations": reps})
print("shapes written", count)
