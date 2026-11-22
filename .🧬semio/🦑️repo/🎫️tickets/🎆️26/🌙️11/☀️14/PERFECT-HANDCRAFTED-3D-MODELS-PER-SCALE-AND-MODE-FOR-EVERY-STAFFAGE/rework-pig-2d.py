"""🐖 pig 2D root fixes: at 1:500 the snout disc is part of the head shape and the tail curl part of the hindquarter shape (neither is a printable part of its own at 1.4 mm animal height), so those shapes name both thermal regions.

Usage: python3 rework-pig-2d.py     re-reads the catalog asset and writes `representations` (idempotent)
"""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("pig_io", os.path.join(T, "r3-pig-io.py"))
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)

SHARED = {"head": ["skin-head", "snout"], "hindbody": ["skin-trunk", "tail"]}

asset = io.load()
representations = asset["representations"]
changed = 0
for shapes in representations["500"]["geometry"].values():
    for shape in shapes:
        if shape["id"] in SHARED and shape.get("region") == SHARED[shape["id"]][0]:
            position = list(shape).index("region")
            items = list(shape.items())
            items[position] = ("regions", SHARED[shape["id"]])
            shape.clear()
            shape.update(items)
            changed += 1
if changed:
    io.put({"representations": representations})
print("shapes changed:", changed)
