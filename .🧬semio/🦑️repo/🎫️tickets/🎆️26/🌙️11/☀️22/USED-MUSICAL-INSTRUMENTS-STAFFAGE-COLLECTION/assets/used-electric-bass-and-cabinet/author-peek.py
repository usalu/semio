"""🔍 Peek into a catalog asset section by section: author-peek.py <category/slug> <json path dotted> [maxchars]."""
import json, sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
cat, slug = sys.argv[1].split("/")
folder = next(p for p in S.iterdir() if p.name.endswith(cat))
doc = json.loads((folder / slug / "🔣️.json").read_text())
node = doc
for k in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(k)] if isinstance(node, list) else node[k]
mx = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict):
    for k, v in node.items():
        s = json.dumps(v, ensure_ascii=False)
        print(f"{k}: ({len(s)}) {s[:mx]}")
elif isinstance(node, list):
    print(f"list len {len(node)}")
    for i, v in enumerate(node):
        print(i, json.dumps(v, ensure_ascii=False)[:mx])
else:
    print(node)
