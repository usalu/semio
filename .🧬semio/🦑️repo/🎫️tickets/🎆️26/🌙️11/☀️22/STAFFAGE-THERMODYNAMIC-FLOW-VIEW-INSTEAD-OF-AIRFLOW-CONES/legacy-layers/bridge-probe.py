"""🧩 Probe of the Python staffage bridge without Manim: loads its validator, provenance and level-of-detail functions by AST (as `🧪️tests/🐍️media_staffage.py` does) and checks them against the shared cast vectors.

Checks: every invalid cast of `🧫️fixtures/🎭️cast-export` is refused with the message of the TypeScript twin, the valid manifest and the course manifest are accepted, and the revisions of the levels of detail equal `🧫️fixtures/🧊️cast-lod`.
Run: uv run --no-project --python 3.12 python -I bridge-probe.py <repo root>
"""
import ast
import hashlib
import json
import sys
import types
from functools import lru_cache
from pathlib import Path
from typing import NamedTuple

root = Path(sys.argv[1])
visuals = root / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals"
staffage = root / "🎓️teaching/🏛️architecture/🧍️staffage"
bridge = visuals / "🧍️staffage/🐍️.py"
functions = {"canonical_json", "entry_issues", "_levels", "lod_level", "_number"}
constants = {"_VIEWPOINTS", "_VIEWS", "_THEMES", "_SCALES", "_FLOW_LAYERS", "_FLOW_OPTIONS", "_CAST_KEYS"}
classes = {"ResolvedLod"}
tree = ast.parse(bridge.read_text(encoding="utf-8"))
nodes = [node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name in functions or isinstance(node, ast.ClassDef) and node.name in classes or isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id in constants for target in node.targets)]
module = types.ModuleType("bridge")
module.__dict__.update(json=json, hashlib=hashlib, lru_cache=lru_cache, NamedTuple=NamedTuple, Path=Path, re=__import__("re"), math=__import__("math"), _LEVELS_FILE=staffage / "🔨️modules/📤️export/🎭️cast/🧊️lod/🔣️.json")
sys.modules["bridge"] = module
exec(compile(ast.Module(body=nodes, type_ignores=[]), str(bridge), "exec"), module.__dict__)

vectors = json.loads((staffage / "🧫️fixtures/🎭️cast-export/🔣️.json").read_text(encoding="utf-8"))
issues = lambda entry: module.entry_issues(entry, "/casts/bad") if module.entry_issues.__code__.co_argcount > 1 else module.entry_issues(entry)
failures = []
for name, entry in vectors["manifest"]["casts"].items():
    if issues(entry):
        failures.append(f"valid cast {name} refused: {issues(entry)}")
for vector in vectors["invalid"]:
    if vector["issue"] not in "\n".join(issues(vector["cast"])):
        failures.append(f"invalid cast {vector['name']!r} lacks {vector['issue']!r}: {issues(vector['cast'])}")
course = json.loads((visuals / "🧍️staffage/🔣️.json").read_text(encoding="utf-8"))
for name, entry in course["casts"].items():
    if issues(entry):
        failures.append(f"course cast {name} refused: {issues(entry)}")
lod = json.loads((staffage / "🧫️fixtures/🧊️cast-lod/🔣️.json").read_text(encoding="utf-8"))
revisions = [row for row in lod.values() if isinstance(row, list) and row and isinstance(row[0], dict) and "revision" in row[0]][0]
for row in revisions:
    resolved = module.lod_level(row["lod"], row["outputFps"])
    if resolved.revision != row["revision"] or resolved.fps_cap != row["fpsCap"]:
        failures.append(f"level {row} resolved to {resolved.revision} {resolved.fps_cap}")
print(json.dumps({"valid": len(vectors["manifest"]["casts"]), "invalid": len(vectors["invalid"]), "course": len(course["casts"]), "levels": len(revisions), "failures": failures}, ensure_ascii=False, indent=1))
sys.exit(1 if failures else 0)
