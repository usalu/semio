"""🔍️ Prints one section of a catalog asset document compactly, so a sibling can be studied section by section.

Usage: python3 author-inspect.py <category-folder>/<slug> <section path, dot separated> [max chars]
"""
import json, sys
from pathlib import Path

CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
doc = json.loads((CATALOG / sys.argv[1] / "🔣️.json").read_text()) if not sys.argv[1].endswith(".json") else json.loads(Path(sys.argv[1]).read_text())
node = doc["asset"]
for key in sys.argv[2].split(".") if sys.argv[2] else []:
    node = node[int(key)] if isinstance(node, list) else node[key]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 6000
print(json.dumps(node, ensure_ascii=False)[:limit])
