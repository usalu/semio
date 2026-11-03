import ast
import json
import sys
from pathlib import Path

A = Path(r"C:\git\semio\🎓️teaching\🏛️architecture")
E = A / "⚡️energy"
MANIFEST = json.loads((E / "🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json").read_text(encoding="utf8"))["casts"]
ROOTS = [E / "🌬️ventilation/🎬️clip", E / "📊️demand/🎬️clip", E / "🧲️physics/🎬️clip", A / "🎯️optimization/🎬️clip"]
NAMES = {"staffage", "staffage_motion", "fitted_staffage", "pendant_lamp", "_staffage", "_staffage_motion", "_fitted_staffage"}


def enclosing(tree):
    owners = {}
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.ClassDef)):
            for child in ast.walk(node):
                if hasattr(child, "lineno"):
                    prev = owners.get(id(child))
                    if prev is None or node.lineno >= prev[0]:
                        owners[id(child)] = (node.lineno, node.name)
    return owners


def describe(cast):
    row = MANIFEST.get(cast)
    if not row:
        return "NOT-IN-MANIFEST"
    parts = [row["asset"], row.get("pose") or "clip:" + row.get("clip", "?"), row["view"], row.get("environment", "-")]
    if row.get("transport"):
        parts.append("transport=" + "+".join(row["transport"]))
    if row.get("mirror"):
        parts.append("mirror")
    return " | ".join(parts)


for root in ROOTS:
    for path in sorted(root.rglob("*.py")):
        if "__pycache__" in path.parts or "tests" in "/".join(path.parts[-3:]) or "🧪" in str(path.relative_to(root)):
            continue
        source = path.read_text(encoding="utf8")
        tree = ast.parse(source)
        owners = enclosing(tree)
        for node in ast.walk(tree):
            if isinstance(node, ast.Call):
                fn = node.func
                name = fn.id if isinstance(fn, ast.Name) else fn.attr if isinstance(fn, ast.Attribute) else ""
                if name in NAMES and node.args:
                    arg = node.args[0]
                    cast = arg.value if isinstance(arg, ast.Constant) else "<" + ast.unparse(arg) + ">"
                    owner = owners.get(id(node), (0, "module"))[1]
                    rel = str(path.relative_to(A)).replace("🐍️.py", "")
                    print(f"{rel}:{node.lineno}\t{owner}\t{name}\t{cast}\t{describe(cast) if not cast.startswith('<') else ''}")
