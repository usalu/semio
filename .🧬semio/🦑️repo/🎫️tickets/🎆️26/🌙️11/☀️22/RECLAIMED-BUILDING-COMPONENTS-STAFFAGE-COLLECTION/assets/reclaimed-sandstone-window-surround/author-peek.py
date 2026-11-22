"""🔎️ Prints one section of a catalog asset document, compactly, so a sibling can be studied piece by piece.

Usage: python3 author-peek.py <category/slug | path> <dotted.path> [max chars]
"""
import json
import sys
from pathlib import Path

CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")


def load(name):
    path = Path(name)
    if not path.exists():
        path = CATALOG / name / "🔣️.json"
    return json.loads(path.read_text())


def walk(node, dotted):
    for key in [k for k in dotted.split(".") if k]:
        node = node[int(key)] if isinstance(node, list) else node[key]
    return node


def main():
    doc = load(sys.argv[1])
    node = walk(doc, sys.argv[2] if len(sys.argv) > 2 else "")
    limit = int(sys.argv[3]) if len(sys.argv) > 3 else 6000
    if isinstance(node, dict) and limit < 0:
        print({k: (type(v).__name__, len(json.dumps(v))) for k, v in node.items()})
        return
    print(json.dumps(node, ensure_ascii=False, separators=(",", ":"))[:limit])


main()
