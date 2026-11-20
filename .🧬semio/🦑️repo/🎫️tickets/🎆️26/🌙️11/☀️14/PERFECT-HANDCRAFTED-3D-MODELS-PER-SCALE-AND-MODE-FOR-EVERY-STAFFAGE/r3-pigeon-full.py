"""🧩 Round 3 pigeon: writes the trial document with the authoring source's `volumes` in it (🗑️generated/r3-pigeon-full/🔣️.json), so that `r3-validate.ts --file` and `r3-2d.ts --file` judge roots and volumes together before the catalog file is written.

Usage: python3 r3-pigeon-full.py
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
doc = json.load(open(T + "/🗑️generated/r3-pigeon/🔣️.json", encoding="utf-8"))
doc["asset"]["volumes"] = json.load(open(T + "/volumes/animals/pigeon.json", encoding="utf-8"))["volumes"]
os.makedirs(T + "/🗑️generated/r3-pigeon-full", exist_ok=True)
json.dump(doc, open(T + "/🗑️generated/r3-pigeon-full/🔣️.json", "w", encoding="utf-8"), ensure_ascii=False)
print("wrote trial document with volumes")
