"""🔍 Print one section of a catalog asset as compact JSON (study aid for the litter-bin author)."""
import json, sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
def load(rel):
    p = pathlib.Path(rel) if rel.startswith("/") else S / rel / "🔣️.json"
    return json.loads(p.read_text())
doc = load(sys.argv[1])
node = doc
for k in sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []:
    node = node[int(k)] if isinstance(node, list) else node[k]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict) and len(sys.argv) > 4 and sys.argv[4] == "keys":
    for k, v in node.items():
        print(k, len(json.dumps(v)))
else:
    print(json.dumps(node, ensure_ascii=False)[:limit])
