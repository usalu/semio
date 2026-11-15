"""♻️ Restores the `structure` (and optionally `decisions`) list of the given scales in the source from the spliced catalog asset (undoes an unwanted kit-structure regeneration).
Usage: python3 fauna-restore-struct.py <asset> <asset 🔣️.json> <scale>[,<scale>]"""
import json, sys
asset, af, scales = sys.argv[1], sys.argv[2], sys.argv[3].split(",")
a = json.load(open(af))["asset"]["volumes"]
f = f"volumes/{asset}.json"
d = json.load(open(f))
for s in scales:
    d["volumes"][s]["structure"] = a[s]["structure"]
json.dump(d, open(f, "w"), separators=(",", ":"), ensure_ascii=False)
print("restored", scales)
