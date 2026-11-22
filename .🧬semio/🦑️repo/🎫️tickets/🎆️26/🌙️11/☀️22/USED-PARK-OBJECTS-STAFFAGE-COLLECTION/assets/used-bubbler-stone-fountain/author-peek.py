"""🔍️ Prints a section of a catalog asset document compactly: `python3 author-peek.py <category>/<slug>|<file> <dotted.path> [maxchars]`."""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
cats = {"furniture": "🛋️furniture", "objects": "🪑️objects", "components": "🧱️components"}
a = sys.argv[1]
if a.endswith(".json"):
    d = json.load(open(a))
else:
    c, s = a.split("/")
    d = json.load(open(S + cats.get(c, c) + "/" + s + "/🔣️.json"))
node = d
for k in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(k)] if isinstance(node, list) else node[k]
n = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict):
    for k, v in node.items():
        print(k, ":", json.dumps(v, ensure_ascii=False)[: n // max(1, len(node))])
else:
    print(json.dumps(node, ensure_ascii=False)[:n])
