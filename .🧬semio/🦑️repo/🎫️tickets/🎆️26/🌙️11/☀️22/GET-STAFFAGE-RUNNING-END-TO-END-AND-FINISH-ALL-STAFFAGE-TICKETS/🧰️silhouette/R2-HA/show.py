"""🔦️ Prints drawn shapes and body solids of a person by id substring, one per line and cut to a width: python3 show.py <slug|file.json> <scale> <substring,…> [width] [view,…]"""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
slug, scale, subs = sys.argv[1], sys.argv[2], sys.argv[3].split(",")
width = int(sys.argv[4]) if len(sys.argv) > 4 else 700
views = sys.argv[5].split(",") if len(sys.argv) > 5 else ["elevation", "front", "plan"]
a = json.load(open(slug if slug.endswith(".json") else f"{CAT}/{slug}/🔣️.json"))["asset"]
hit = lambda i: any((s == i) if not s.endswith("*") else i.startswith(s[:-1]) for s in subs)
for v in views:
    for s in a["representations"][scale]["geometry"][v]:
        if hit(s["id"]):
            print(f"{v}: {json.dumps(s, ensure_ascii=False)[:width]}")
if "body" in views or len(sys.argv) <= 5:
    for s in a["volumes"][scale]["body"]:
        if hit(s["id"]):
            print(f"body: {json.dumps(s, ensure_ascii=False)[:width]}")
    for b in a["rig"]["bones"]:
        if hit(b["id"]):
            print(f"bone: {json.dumps(b)[:width]}")
