#!/usr/bin/env python3
"""🧮️ Counts the vectors of every quiz fixture group, for the report.

Run from the repository root: ``.venv/Scripts/python.exe <this file>``.
"""

import glob
import json
import os

for path in sorted(glob.glob(os.path.join("🧰️framework", "🛍️products", "❓️quiz", "🧫️fixtures", "*", "🔣️.json"))):
    with open(path, encoding="utf-8") as handle:
        document = json.load(handle)
    if "$comment" not in document:
        continue
    groups = {name: len(value) for name, value in document.items() if isinstance(value, list)}
    nested = {"%s.%s" % (name, inner): len(value[inner]) for name, value in document.items() if isinstance(value, dict) for inner in value if isinstance(value[inner], list)}
    print("[DEBUG] %s %d bytes %r %r" % (os.path.basename(os.path.dirname(path)), os.path.getsize(path), groups, nested))
