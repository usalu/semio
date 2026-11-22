"""🔍 Print one dotted path of a JSON document, truncated, to learn the sibling assets section by section."""
import json, sys

doc = json.load(open(sys.argv[1]))
node = doc
for key in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(key)] if isinstance(node, list) else node[key]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 3000
if isinstance(node, dict) and len(sys.argv) > 4 and sys.argv[4] == "keys":
    for k, v in node.items():
        print(k, type(v).__name__, len(json.dumps(v)))
else:
    print(json.dumps(node, ensure_ascii=False)[:limit])
