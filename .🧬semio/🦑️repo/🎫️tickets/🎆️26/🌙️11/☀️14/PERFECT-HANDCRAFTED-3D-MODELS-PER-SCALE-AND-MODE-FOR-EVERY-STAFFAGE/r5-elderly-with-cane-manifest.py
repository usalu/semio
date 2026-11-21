"""📇 Writes the round-5 height of `humans/elderly-with-cane` (1.657 m, the 1:50 body in the default pose after the ankle fix) into its row of the catalog manifest. The file is re-read right before writing and only that one number changes.

Usage: python3 r5-elderly-with-cane-manifest.py
"""
import re

MANIFEST = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"
text = open(MANIFEST, encoding="utf-8").read()
start = text.index('"id": "elderly-with-cane"')
found = re.compile(r'"heightM": [0-9.]+').search(text, start)
if text.find('"id":', start + 5) < found.start():
    raise SystemExit("no heightM in the elderly-with-cane row")
print(found.group(0), "->", '"heightM": 1.657')
text = text[: found.start()] + '"heightM": 1.657' + text[found.end() :]
open(MANIFEST, "w", encoding="utf-8").write(text)
