#!/usr/bin/env python3
"""🧾️ One compact line per solid of a volume list: solids.py <id> <scale> [body|thermal|structure]"""
import glob, json, sys
a, s = sys.argv[1], sys.argv[2]; l = sys.argv[3] if len(sys.argv) > 3 else "body"
d = json.load(open(glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{a}/🔣️.json")[0]))["asset"]
print("##", a, s, l, d["dimensions"])
for x in d["volumes"][s][l]:
    print(" ", json.dumps({k: v for k, v in x.items() if k not in ("tone", "role")}, ensure_ascii=False, separators=(",", ":"))[:330])
