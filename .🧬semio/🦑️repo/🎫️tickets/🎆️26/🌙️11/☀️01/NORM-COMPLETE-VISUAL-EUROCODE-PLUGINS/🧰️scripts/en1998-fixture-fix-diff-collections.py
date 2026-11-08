#!/usr/bin/env python3
"""EN 1998: the diff fixtures (objects whose `slopes` is a keyed delta, not a list) get the keyed-delta shape for isolationSystems, pierBases and criticalRegions instead of an empty list (idempotent, layout preserved)."""
import os, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent / "field-adder"))
import jsonspan

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"
KEYS = ["isolationSystems", "pierBases", "criticalRegions"]
DELTA = {"removed": [], "inserted": [], "moved": [], "modified": []}


def fix(text):
    root = jsonspan.parse(text)
    edits = []
    for path, obj in jsonspan.find_objects(root, lambda o: o.get("slopes") and o.get("slopes").value.kind == "object" and o.get("tieBeams")):
        level = len(path)
        for key in KEYS:
            member = obj.get(key)
            if member and member.value.kind == "array" and not member.value.items:
                edits.append((member.value.start, member.value.end, jsonspan.render(DELTA, text, level + 1)))
    for start, end, new in sorted(edits, key=lambda e: -e[0]):
        text = text[:start] + new + text[end:]
    return text if edits else None


count = 0
for dirpath, _, files in os.walk(ROOT):
    if "__pycache__" in dirpath:
        continue
    for name in files:
        if not name.endswith(".json"):
            continue
        path = os.path.join(dirpath, name)
        text = open(path, encoding="utf-8").read()
        if '"isolationSystems": []' not in text and '"isolationSystems":[]' not in text:
            continue
        out = fix(text)
        if out is not None:
            open(path, "w", encoding="utf-8").write(out)
            count += 1
print("fixed", count)
