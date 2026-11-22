"""🔎 Prints a section of a catalog asset document compactly: `python3 author-peek.py <category/slug> <dotted.path> [maxchars]`."""
import json, sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
cat = {"furniture": "🛋️furniture", "appliances": "🔌️appliances", "objects": "🪑️objects"}
c, slug = sys.argv[1].split("/")
folder = cat.get(c, c)
doc = json.loads((S / folder / slug / "🔣️.json").read_text())
node = doc
for part in [p for p in sys.argv[2].split(".") if p]:
    node = node[int(part)] if isinstance(node, list) else node[part]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict) and len(sys.argv) > 4:
    print(list(node.keys()))
else:
    s = json.dumps(node, ensure_ascii=False)
    print(s[:limit], f"... [{len(s)} chars]" if len(s) > limit else "")
