"""🔍 Prints one section of a catalog asset document, truncated, for studying the contract.

Usage: python3 author-peek.py <category/slug> <dotted.path> [maxchars]
"""
import json, sys
from pathlib import Path
S = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
cat = {"objects": "🪑️objects", "appliances": "🔌️appliances", "humans": "👤️humans", "furniture": "🛋️furniture"}
c, s = sys.argv[1].split("/")
p = Path(sys.argv[1]) if sys.argv[1].endswith(".json") else S / cat[c] / s / "🔣️.json"
d = json.loads(p.read_text())
node = d
for k in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    node = node[int(k)] if isinstance(node, list) else node[k]
n = int(sys.argv[3]) if len(sys.argv) > 3 else 3000
if isinstance(node, dict):
    print({k: len(json.dumps(v)) for k, v in node.items()})
t = json.dumps(node, ensure_ascii=False)
print(t[:n])
