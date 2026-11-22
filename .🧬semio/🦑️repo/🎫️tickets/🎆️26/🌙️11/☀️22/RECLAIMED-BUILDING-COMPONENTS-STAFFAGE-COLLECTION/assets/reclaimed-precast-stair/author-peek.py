"""🔎 Print a section of a catalog asset compactly: author-peek.py <slug-or-path> <dotted.path> [maxchars]."""
import json, sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
arg = sys.argv[1]
p = pathlib.Path(arg) if arg.endswith(".json") else next(S.glob(f"*/{arg}/🔣️.json"))
d = json.loads(p.read_text())
node = d
for k in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(k)] if isinstance(node, list) else node[k]
mx = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict):
    print("KEYS", list(node.keys()))
s = json.dumps(node, ensure_ascii=False)
print(len(s)); print(s[:mx])
