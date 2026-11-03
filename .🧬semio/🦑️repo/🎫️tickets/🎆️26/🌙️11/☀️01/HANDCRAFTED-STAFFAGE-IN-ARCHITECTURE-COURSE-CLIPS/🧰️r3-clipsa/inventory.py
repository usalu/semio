"""📋 List every manifest cast string a batch-A clip mentions, with file:line and the enclosing function."""
import ast
import json
import sys
from pathlib import Path

ARCH = Path(sys.argv[1])
ENERGY = ARCH / "⚡️energy"
MANIFEST = json.loads((ENERGY / "🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json").read_text(encoding="utf-8"))["casts"]
FOLDERS = [ENERGY / "🔥️heating/🎬️clip", ENERGY / "❄️cooling/🎬️clip"]


def enclosing(tree):
    owner = {}

    def visit(node, name):
        for child in ast.iter_child_nodes(node):
            label = child.name if isinstance(child, (ast.FunctionDef, ast.ClassDef)) else name
            if isinstance(child, ast.Constant) and isinstance(child.value, str):
                owner[id(child)] = name
            visit(child, label)

    visit(tree, "<module>")
    return owner


for folder in FOLDERS:
    for path in sorted(folder.glob("*/🐍️.py")):
        tree = ast.parse(path.read_text(encoding="utf-8"))
        owner = enclosing(tree)
        for node in ast.walk(tree):
            if isinstance(node, ast.Constant) and isinstance(node.value, str) and node.value in MANIFEST:
                entry = MANIFEST[node.value]
                detail = entry.get("clip") or entry.get("pose")
                print("\t".join([f"{folder.parent.parent.name}/{path.parent.name}", str(node.lineno), owner.get(id(node), ""), node.value, entry["asset"], ("clip:" if "clip" in entry else "pose:") + str(detail), entry["view"], entry.get("environment", "-"), entry["viewpoint"], str(entry.get("transport", "-"))]))
