"""🔎 Prints a compact outline of a catalog asset document: top-level members, sizes and chosen sub-paths.

Usage: python3 author-peek.py <path to 🔣️.json> [json-pointer-ish dotted path] [max chars]
"""
import json, sys

doc = json.load(open(sys.argv[1]))
node = doc
if len(sys.argv) > 2 and sys.argv[2] != "-":
    for key in sys.argv[2].split("."):
        node = node[int(key)] if isinstance(node, list) else node[key]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 3000
if isinstance(node, dict):
    for k, v in node.items():
        s = json.dumps(v, ensure_ascii=False)
        print(f"{k}: {len(s)} chars :: {s[:limit // max(1, len(node))]}")
else:
    print(json.dumps(node, ensure_ascii=False)[:limit])
