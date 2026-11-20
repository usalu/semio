"""📇 Keeps the manifest row of `humans/running-child` in the catalog index in step with the asset: `heightM` and `widthM` from its `dimensions`, and `powerW` when a value is passed (the validator names the expected one). Re-reads the manifest right before writing and changes only the three numbers of this row.

Usage: python3 r3-running-child-manifest.py [powerW]
"""
import json
import re
import sys

ROOT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
dimensions = json.load(open(ROOT + "👤️humans/running-child/🔣️.json", encoding="utf-8"))["asset"]["dimensions"]
text = open(ROOT + "🔣️.json", encoding="utf-8").read()
start = text.index('"id": "running-child"')
end = text.index('"poses"', start)
block = text[start:end]
fresh = re.sub(r'"heightM": [0-9.]+,', '"heightM": %s,' % json.dumps(dimensions["heightM"]), block)
fresh = re.sub(r'"widthM": [0-9.]+,', '"widthM": %s,' % json.dumps(dimensions["widthM"]), fresh)
if len(sys.argv) > 1:
    fresh = re.sub(r'"powerW": [0-9.]+,', '"powerW": %s,' % json.dumps(float(sys.argv[1])), fresh)
if fresh != block:
    if open(ROOT + "🔣️.json", encoding="utf-8").read() != text:
        raise SystemExit("manifest changed while editing; retry")
    open(ROOT + "🔣️.json", "w", encoding="utf-8").write(text[:start] + fresh + text[end:])
    json.loads(open(ROOT + "🔣️.json", encoding="utf-8").read())
    print("manifest row updated")
