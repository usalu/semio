"""🔍️ Peek into a catalog asset section by section: `python3 author-peek.py <category-folder>/<slug> <json path> [maxchars]`."""
import json, sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
doc = json.loads((S / sys.argv[1] / "🔣️.json").read_text())
node = doc["asset"]
for part in sys.argv[2].split("."):
    if part:
        node = node[int(part)] if isinstance(node, list) else node[part]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 300
if isinstance(node, list):
    for item in node:
        print(json.dumps(item, ensure_ascii=False)[:limit])
elif isinstance(node, dict):
    for k, v in node.items():
        print(k, "=", json.dumps(v, ensure_ascii=False)[:limit])
else:
    print(node)
