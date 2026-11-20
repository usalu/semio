"""📝 Writes the round 3 excavator into its catalog file: the members the build corrected (summary, provenance, dimensions, rig, anchors, representations, poses, clips, thermal, volumes) are taken from the validated draft, every other member must be unchanged; the catalog file is re-read right before writing and keeps its member order and one-space indentation. Usage: python3 r3-excavator-apply.py (then volumes-insert.py restyles the volumes member)."""
import glob
import json
import os

here = os.path.dirname(os.path.abspath(__file__))
CATALOG = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*machinery/excavator/*.json")[0]
DRAFT = os.path.join(here, "🗑️generated", "r3-excavator", "draft", "excavator", "🔣️.json")
CHANGED = ("summary", "provenance", "dimensions", "rig", "anchors", "representations", "poses", "clips", "thermal", "volumes")

draft = json.load(open(DRAFT, encoding="utf8"))["asset"]
doc = json.load(open(CATALOG, encoding="utf8"))
asset = doc["asset"]
assert list(asset) == list(draft), "member order differs"
for key in asset:
    if key in CHANGED:
        asset[key] = draft[key]
    else:
        assert asset[key] == draft[key], key
with open(CATALOG, "w", encoding="utf8") as fh:
    fh.write(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
print("written", CATALOG)
