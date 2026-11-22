"""🔍️ Prints a section of a catalog asset compactly: `python3 author-peek.py <category/slug> <json pointer path dotted> [maxchars]`."""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
folders = {"furniture": "🛋️furniture", "objects": "🪑️objects", "appliances": "🔌️appliances"}
cat, slug = sys.argv[1].split("/")
doc = json.load(open(f"{S}/{folders[cat]}/{slug}/🔣️.json"))
node = doc
for part in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(part)] if isinstance(node, list) else node[part]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict):
    for k, v in node.items():
        s = json.dumps(v, ensure_ascii=False)
        print(f"{k}: {s[:limit]}{'…' if len(s) > limit else ''}")
elif isinstance(node, list):
    for i, v in enumerate(node):
        s = json.dumps(v, ensure_ascii=False)
        print(f"[{i}] {s[:limit]}{'…' if len(s) > limit else ''}")
else:
    print(node)
