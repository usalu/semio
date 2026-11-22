"""🔍 Peek into an asset document section by section: author-peek.py <file> <dotted.path> [maxchars]."""
import json, sys
d = json.load(open(sys.argv[1]))
node = d
for k in [p for p in sys.argv[2].split(".") if p]:
    node = node[int(k)] if isinstance(node, list) else node[k]
lim = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict):
    print("KEYS:", list(node.keys()))
elif isinstance(node, list):
    print("LEN:", len(node))
s = json.dumps(node, ensure_ascii=False)
print(s[:lim])
