"""🔍 Prints one section of a catalog asset compactly: `python3 author-peek.py <category-folder>/<slug> <json-path> [maxchars]`."""
import json, sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
doc = json.loads((S / sys.argv[1] / "🔣️.json").read_text())
node = doc
for part in sys.argv[2].split("/") if len(sys.argv) > 2 and sys.argv[2] else []:
    node = node[int(part)] if isinstance(node, list) else node[part]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict) and limit < 0:
    for k, v in node.items():
        print(k, len(json.dumps(v)))
else:
    print(json.dumps(node, ensure_ascii=False)[:abs(limit)])
