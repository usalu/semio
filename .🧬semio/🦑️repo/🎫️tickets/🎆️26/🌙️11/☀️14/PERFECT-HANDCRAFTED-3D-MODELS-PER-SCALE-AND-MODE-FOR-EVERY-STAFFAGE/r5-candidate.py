"""🧪 Round 5: writes a candidate asset document (catalog file + root members from a draw2d run + the volumes source) for `r3-validate.ts --file`, so nothing is written to the catalog before it validates.

Usage: python3 r5-candidate.py <category/slug> <catalog file> <members.json> <out.json>
"""
import json
import os
import sys

slug, catalog, members, out = sys.argv[1:5]
here = os.path.dirname(os.path.abspath(__file__))
doc = json.load(open(catalog, encoding="utf-8"))
doc["asset"].update(json.load(open(members, encoding="utf-8")))
doc["asset"]["volumes"] = json.load(open(os.path.join(here, "volumes", slug + ".json"), encoding="utf-8"))["volumes"]
json.dump(doc, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
print("candidate", out)
