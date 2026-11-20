"""🧷 Root fixes of the dairy cow outside `volumes`: writes whole members (dimensions, representations, ...) from a patch file into the catalog asset, member by member in the file's own style; the file is re-read right before writing and nothing else changes.

Usage: python3 r3-dairy-cow-root.py <patch.json> [--candidate]   (--candidate: writes 🗑️generated/r3-dairy-cow/🔣️.json instead of the catalog file, for `r3-validate.ts --file`)
"""
import json
import os
import sys

import r3_dairy_cow_io as io

patch = json.load(open(sys.argv[1], encoding="utf-8"))
if "--candidate" in sys.argv:
    text, asset = io.read()
    document = json.loads(text)
    document["asset"].update(patch)
    source = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes/animals/dairy-cow.json"), encoding="utf-8"))
    document["asset"]["volumes"] = source["volumes"]
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated/r3-dairy-cow/🔣️.json")
    json.dump(document, open(out, "w", encoding="utf-8"), ensure_ascii=False)
    print("candidate", out)
else:
    print("written", io.write(lambda asset: patch))
