"""🔍 Prints one section of a catalog asset compactly: `python3 author-peek.py <category>/<slug> <dotted.path> [maxchars]`."""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
folders = {"furniture": "🛋️furniture", "appliances": "🔌️appliances", "objects": "🪑️objects"}
cat, slug = sys.argv[1].split("/")
doc = json.load(open(f"{S}/{folders[cat]}/{slug}/🔣️.json"))
node = doc
for key in [k for k in sys.argv[2].split(".") if k]:
    node = node[int(key)] if isinstance(node, list) else node[key]
text = json.dumps(node, ensure_ascii=False)
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict):
    print("KEYS", list(node.keys()))
print(text[:limit])
