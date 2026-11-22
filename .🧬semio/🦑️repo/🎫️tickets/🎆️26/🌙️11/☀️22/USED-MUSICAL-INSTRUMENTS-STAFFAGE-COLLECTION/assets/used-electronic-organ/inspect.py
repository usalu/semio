"""🔍 Print a section of an asset document compactly: inspect.py FILE PATH [maxchars]"""
import json, sys
d = json.load(open(sys.argv[1]))
node = d
for k in [p for p in sys.argv[2].split('/') if p]:
    node = node[int(k)] if isinstance(node, list) else node[k]
n = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(node, dict) and len(sys.argv) > 4:
    print(list(node.keys()))
else:
    print(json.dumps(node, ensure_ascii=False)[:n])
