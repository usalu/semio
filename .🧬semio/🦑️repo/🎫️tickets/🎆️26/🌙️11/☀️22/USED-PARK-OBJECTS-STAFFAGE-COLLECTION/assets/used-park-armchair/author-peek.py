"""🔍 Peek into a catalog asset section by section: python3 author-peek.py <category-folder>/<slug> [json.path] [maxchars]"""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
doc = json.load(open(S + sys.argv[1] + "/🔣️.json"))
node = doc
for k in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(k)] if isinstance(node, list) else node[k]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict):
    for k, v in node.items():
        s = json.dumps(v, ensure_ascii=False)
        print(f"{k}: ({len(s)}) {s[:limit]}")
else:
    print(json.dumps(node, ensure_ascii=False)[:limit])
