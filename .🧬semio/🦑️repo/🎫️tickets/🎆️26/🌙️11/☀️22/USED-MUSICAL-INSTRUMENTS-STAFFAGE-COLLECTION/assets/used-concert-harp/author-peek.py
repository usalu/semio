"""🔍️ Prints one section of a catalog asset compactly: `python3 author-peek.py <category/slug> <json path> [maxchars]`."""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
FOLDERS = {"objects": "🪑️objects", "appliances": "🔌️appliances", "humans": "👤️humans"}
cat, slug = sys.argv[1].split("/")
d = json.load(open(S + FOLDERS.get(cat, cat) + "/" + slug + "/🔣️.json"))["asset"]
for part in sys.argv[2].split("."):
    d = d[int(part)] if isinstance(d, list) else d[part]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 6000
s = json.dumps(d, ensure_ascii=False)
print(s[:limit])
