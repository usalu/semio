"""🔍 Prints a section of a catalog asset document compactly: python3 author-peek.py <category>/<slug> <dotted.path> [maxchars]."""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
import glob
cat, slug = sys.argv[1].split("/")
path = glob.glob(f"{S}/*{cat}/{slug}/🔣️.json")[0]
d = json.load(open(path))
node = d
for k in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(k)] if isinstance(node, list) else node[k]
n = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict):
    print("KEYS", list(node.keys()))
s = json.dumps(node, ensure_ascii=False)
print(len(s)); print(s[:n])
