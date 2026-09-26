#!/usr/bin/env python3
"""🧷️ The `🖊️mutate-semio-drawing` Python reference carried a rename codemod's damage: the scene-graph NODE kind
`group` read `group-nodes` (the MUTATION id) and the transform key `scale` read `scale-node`, so every committed
document (whose nodes are `"kind": "group"` and transforms `{translation, rotation, scale}`) failed with
`KeyError: 'group'`. Node-kind and transform-key uses go back to the document's own spelling; mutation ids
(`group-nodes`, `scale-node` as verbs/labels) stay. Usage: semio-drawing-node-kind.py [--write]"""
import re
import sys
from pathlib import Path

PATH = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🐍️.py")
NODE_FUNCTIONS = {"parse_node", "print_node", "read_node", "write_node"}
lines = PATH.read_text(encoding="utf-8").split("\n")
func, changes = None, 0
for index, line in enumerate(lines):
    match = re.match(r"\s*def (\w+)\(", line)
    if match:
        func = match.group(1)
    original = line
    line = re.sub(r'(\["kind"\] (?:!=|==) )"group-nodes"', r'\1"group"', line)
    line = line.replace('"kind": "group-nodes"', '"kind": "group"')
    if func in NODE_FUNCTIONS:
        line = line.replace('if kind == "group-nodes":', 'if kind == "group":')
    if line.startswith("NODE_ORDER = ") or line.startswith("NODE_LETTER = "):
        line = line.replace('"group-nodes"', '"group"')
    line = line.replace('"group-nodes": 0', '"group": 0')
    line = line.replace('"scale-node": ', '"scale": ').replace('["scale-node"]', '["scale"]')
    if line != original:
        changes += 1
        print(f"{index + 1}: {original.strip()[:90]}\n   → {line.strip()[:90]}")
    lines[index] = line
print(f"changed lines={changes}")
if "--write" in sys.argv:
    PATH.write_text("\n".join(lines), encoding="utf-8")
