"""🔎️ Prints one section of a catalog asset compactly: `python3 author-peek.py <category>/<slug> <json path dotted> [maxchars]`."""
import json, sys
from pathlib import Path
S = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
cats = {"objects": "🪑️objects", "furniture": "🛋️furniture", "components": "🧱️components"}
cat, slug = sys.argv[1].split("/")
p = Path(sys.argv[1]) if sys.argv[1].endswith(".json") else S / cats[cat] / slug / "🔣️.json"
d = json.loads(p.read_text())
node = d
for k in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(k)] if isinstance(node, list) else node[k]
n = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict):
    for k, v in node.items():
        s = json.dumps(v, ensure_ascii=False)
        print(f"{k}: ({len(s)}) {s[:n]}")
elif isinstance(node, list):
    for i, v in enumerate(node):
        s = json.dumps(v, ensure_ascii=False)
        print(f"[{i}] ({len(s)}) {s[:n]}")
else:
    print(node)
