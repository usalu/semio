"""🔎 Inspect authored straggler animation calls without executing or editing the scene."""

import ast
from pathlib import Path

source = Path.cwd() / "🎓️teaching/energy/demand/3_heating/6_final_calculation/scene_6.py"
text = source.read_text(encoding="utf-8")
tree = ast.parse(text)
scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "AnlagenVerluste")
construct = next(node for node in scene.body if isinstance(node, ast.FunctionDef) and node.name == "construct")
counts = {}
for call in [node for node in ast.walk(construct) if isinstance(node, ast.Call)]:
    name = ast.unparse(call.func)
    if name in ("self.play", "self.wait", "hold_for", "swap_caption"):
        counts[name] = counts.get(name, 0) + 1
print("[DEBUG]", counts)
helpers = Path.cwd() / "🎓️teaching/manim_visuals.py"
helper_text = helpers.read_text(encoding="utf-8")
for node in ast.parse(helper_text).body:
    if isinstance(node, ast.FunctionDef) and node.name in ("hold_for", "swap_caption"):
        print(ast.get_source_segment(helper_text, node))
