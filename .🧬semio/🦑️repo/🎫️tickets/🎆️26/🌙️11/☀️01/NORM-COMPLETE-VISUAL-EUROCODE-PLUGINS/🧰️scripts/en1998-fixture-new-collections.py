#!/usr/bin/env python3
"""EN 1998: adds the empty collections isolationSystems, pierBases and criticalRegions behind `slopes` to every committed snapshot object of the artifact's JSON fixtures (idempotent, layout preserved). Usage: python3 -I en1998-fixture-new-collections.py"""
import os, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent / "field-adder"))
import jsonspan

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"
ORDER = ["slopes", "isolationSystems", "pierBases", "criticalRegions"]
is_snapshot = lambda o: all(o.get(k) for k in ("buildings", "tieBeams", "slopes", "placements"))


def patch(text):
    changed = False
    while True:
        root = jsonspan.parse(text)
        todo = None
        for _, obj in jsonspan.find_objects(root, is_snapshot):
            missing = [k for k in ORDER[1:] if not obj.get(k)]
            if missing:
                todo = (obj, missing[0])
                break
        if todo is None:
            return text if changed else None
        obj, key = todo
        anchor = ORDER[ORDER.index(key) - 1]
        text = jsonspan.insert_member(text, obj, anchor, key, "[]")
        changed = True


count = 0
for dirpath, _, files in os.walk(ROOT):
    if "__pycache__" in dirpath or "/target" in dirpath:
        continue
    for name in files:
        if not name.endswith(".json"):
            continue
        path = os.path.join(dirpath, name)
        try:
            text = open(path, encoding="utf-8").read()
        except UnicodeDecodeError:
            continue
        if '"slopes"' not in text or '"tieBeams"' not in text:
            continue
        out = patch(text)
        if out is not None:
            open(path, "w", encoding="utf-8").write(out)
            count += 1
print("patched", count)
