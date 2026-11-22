"""🔍 Peek into a catalog asset section by section: python3 author-peek.py <category/slug> [json-path] [maxchars]."""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
FOLD = {"objects": "🪑️objects", "appliances": "🔌️appliances", "humans": "👤️humans"}
cat, slug = sys.argv[1].split("/")
p = sys.argv[1] if sys.argv[1].endswith(".json") else f"{S}/{FOLD.get(cat, cat)}/{slug}/🔣️.json"
d = json.load(open(p))
node = d
for k in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(k)] if isinstance(node, list) else node[k]
mx = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict):
    for k, v in node.items():
        print(f"{k}: {len(json.dumps(v))} :: {json.dumps(v, ensure_ascii=False)[:mx // max(1, len(node))]}")
else:
    print(json.dumps(node, ensure_ascii=False, indent=1)[:mx])
