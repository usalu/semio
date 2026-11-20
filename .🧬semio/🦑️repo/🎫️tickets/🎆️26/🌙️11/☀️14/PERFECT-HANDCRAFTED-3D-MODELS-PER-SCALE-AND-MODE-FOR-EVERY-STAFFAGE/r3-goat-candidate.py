"""🧪️ Round 3 goat: writes the candidate asset document (the catalog file with the given member files laid over `asset`) to 🗑️generated/r3-goat/🔣️.json for `r3-validate.ts --file` and `r3-2d.ts --file`.
Usage: python3 r3-goat-candidate.py <asset 🔣️.json> <members.json> [...]"""
import json, sys
doc = json.load(open(sys.argv[1], encoding="utf-8"))
for f in sys.argv[2:]: doc["asset"].update(json.load(open(f, encoding="utf-8")))
json.dump(doc, open("🗑️generated/r3-goat/🔣️.json", "w", encoding="utf-8"), ensure_ascii=False)
print("candidate written")
