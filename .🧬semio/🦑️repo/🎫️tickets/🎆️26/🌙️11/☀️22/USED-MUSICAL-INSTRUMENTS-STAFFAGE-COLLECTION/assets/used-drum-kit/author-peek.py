"""🔍️ Prints one section of a catalog asset document, shortened, so the contract can be learned piece by piece."""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
path = sys.argv[1] if sys.argv[1].startswith("/") else S + sys.argv[1] + "/🔣️.json"
doc = json.load(open(path))
node = doc
for key in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(key)] if isinstance(node, list) else node[key]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict) and len(sys.argv) > 4:
    for k, v in node.items():
        print(k, len(json.dumps(v)), json.dumps(v)[:200])
else:
    print(json.dumps(node, ensure_ascii=False)[:limit])
