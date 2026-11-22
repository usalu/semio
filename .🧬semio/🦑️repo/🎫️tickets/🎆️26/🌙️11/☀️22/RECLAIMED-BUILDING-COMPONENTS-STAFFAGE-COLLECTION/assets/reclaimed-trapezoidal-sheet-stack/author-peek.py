"""🔍 Print one JSON-pointer section of a catalog asset, truncated, for study."""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
d = json.load(open(S + sys.argv[1] + "/🔣️.json"))
node = d
for k in [p for p in sys.argv[2].split("/") if p]:
    node = node[int(k)] if isinstance(node, list) else node[k]
lim = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict) and len(sys.argv) > 4:
    print(list(node.keys()))
else:
    print(json.dumps(node, ensure_ascii=False)[:lim])
