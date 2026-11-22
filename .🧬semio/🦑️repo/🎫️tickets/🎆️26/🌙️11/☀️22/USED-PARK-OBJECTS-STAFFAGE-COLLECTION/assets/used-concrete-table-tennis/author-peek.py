"""🔍️ Prints a section of a catalog asset document compactly: author-peek.py <category>/<slug> <json path dotted> [maxchars]."""
import json, sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
folders = {p.name.split("️")[-1] if "️" in p.name else p.name: p for p in S.iterdir() if p.is_dir()}
cat, slug = sys.argv[1].split("/")
path = next(p for k, p in folders.items() if k.endswith(cat)) / slug / "🔣️.json"
doc = json.loads(path.read_text())
node = doc
for key in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(key)] if isinstance(node, list) else node[key]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict) and limit < 0:
    for k, v in node.items():
        print(k, len(json.dumps(v, ensure_ascii=False)))
else:
    print(json.dumps(node, ensure_ascii=False)[:abs(limit)])
