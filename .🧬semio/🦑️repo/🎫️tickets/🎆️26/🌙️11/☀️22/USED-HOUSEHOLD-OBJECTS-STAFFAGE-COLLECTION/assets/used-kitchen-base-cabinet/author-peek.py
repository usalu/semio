"""🔍 Peek into a catalog asset section by section: python3 author-peek.py <category-folder/slug> <json-path> [maxchars]."""
import json, sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
p = sys.argv[1]
f = next(S.glob(f"*/{p}/🔣️.json")) if "/" not in p else S / p / "🔣️.json"
d = json.loads(f.read_text())
node = d
for k in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(k)] if isinstance(node, list) else node[k]
n = int(sys.argv[3]) if len(sys.argv) > 3 else 3000
if isinstance(node, dict):
    for k, v in node.items():
        s = json.dumps(v, ensure_ascii=False)
        print(f"{k}: [{len(s)}] {s[:n]}")
else:
    print(json.dumps(node, ensure_ascii=False)[:n])
