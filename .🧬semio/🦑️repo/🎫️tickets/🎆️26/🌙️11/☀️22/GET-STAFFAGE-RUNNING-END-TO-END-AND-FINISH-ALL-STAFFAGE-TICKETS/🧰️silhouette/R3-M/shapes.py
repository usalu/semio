"""🔎️ Prints drawn shapes and volume solids of an asset whose id contains a pattern: shapes.py <id> <scale> <pattern>[,<pattern>…] [view]"""
import glob, json, sys
ident, scale, pats = sys.argv[1], sys.argv[2], sys.argv[3].split(",")
only = sys.argv[4] if len(sys.argv) > 4 else None
path = ident if ident.endswith(".json") else glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{ident}/🔣️.json")[0]
asset = json.load(open(path))["asset"]
hit = lambda s: any(p in str(s.get("id")) for p in pats)
for view, shapes in asset["representations"][scale]["geometry"].items():
    if only in (None, view):
        for s in shapes:
            if hit(s): print(f"[{view}] {json.dumps(s)[:1500]}")
if only in (None, "vol"):
    for kind in ("body",):
        for s in asset["volumes"][scale][kind]:
            if hit(s): print(f"[vol] {json.dumps(s)[:1500]}")
