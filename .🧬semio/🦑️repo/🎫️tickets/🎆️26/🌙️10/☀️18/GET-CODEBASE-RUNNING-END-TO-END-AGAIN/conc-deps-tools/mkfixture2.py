#!/usr/bin/env python3
"""Writes staged/g2/concurrent.fixture.json: the group-1 cases with pointer sizes plus the admission and size-pressure cases."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
GIB = 1073741824
HOUR = 3600000
NOW = 1_800_000_000_000
with open(os.path.join(HERE, "staged", "concurrent.fixture.json"), encoding="utf-8") as handle:
    fixture = json.load(handle)
for case in fixture["pointer"]:
    for index, step in enumerate(case["steps"]):
        step["bytes"] = (index + 1) * 100000 if index % 3 != 2 else 0
fixture["collect"] += [
    {"name": "a generation that does not fit forces the pressure grace", "nowMs": NOW, "freeFraction": 0.9, "freeBytes": GIB, "neededBytes": 2 * GIB, "pointers": [], "rows": [{"name": "a" * 20, "modifiedAtMs": NOW - 1000}]},
    {"name": "enough free bytes keep the grace", "nowMs": NOW, "freeFraction": 0.9, "freeBytes": 3 * GIB, "neededBytes": 2 * GIB, "pointers": [], "rows": [{"name": "a" * 20, "modifiedAtMs": NOW - 1000}]},
]
fixture["admission"] = [
    {"name": "unknown scope without history takes the declared default", "scope": ".", "freeBytes": 2 * GIB, "pointers": []},
    {"name": "default needs its margin", "scope": ".", "freeBytes": GIB, "pointers": []},
    {"name": "own recorded size wins", "scope": "✏️s", "freeBytes": 3 * GIB, "pointers": [{"scope": "✏️s", "bytes": 2 * GIB}, {"scope": ".", "bytes": 5 * GIB}]},
    {"name": "unrecorded scope takes the largest recorded size", "scope": "🏢️semio-tech", "freeBytes": 3 * GIB, "pointers": [{"scope": "✏️s", "bytes": 2 * GIB}, {"scope": ".", "bytes": 0}]},
    {"name": "exactly the margin is admitted", "scope": ".", "freeBytes": 1234803098, "pointers": [{"scope": ".", "bytes": 1073741824}]},
    {"name": "one byte short is refused", "scope": ".", "freeBytes": 1234803098 - 1, "pointers": [{"scope": ".", "bytes": 1073741824}]},
]
with open(os.path.join(HERE, "staged", "g2", "concurrent.fixture.json"), "w", encoding="utf-8") as handle:
    json.dump(fixture, handle, ensure_ascii=False, indent=1)
    handle.write("\n")
print("fixture 2 written")
