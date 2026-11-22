"""🔍️ Prints a section of a catalog asset document compactly, for studying the contract of sibling assets.

Usage: python3 author-peek.py <category/slug | path> <dotted.path> [maxchars]
"""
import json, sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
CATS = {p.name.split("️", 1)[-1] if "️" in p.name else p.name: p for p in S.iterdir() if p.is_dir()}
arg = sys.argv[1]
if arg.endswith(".json"):
    path = pathlib.Path(arg)
else:
    cat, slug = arg.split("/")
    folder = next(p for k, p in CATS.items() if k.endswith(cat))
    path = folder / slug / "🔣️.json"
doc = json.loads(path.read_text())
node = doc
for key in [k for k in sys.argv[2].split(".") if k]:
    node = node[int(key)] if isinstance(node, list) else node[key]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict) and len(sys.argv) > 4:
    print(list(node.keys()))
else:
    print(json.dumps(node, ensure_ascii=False)[:limit])
