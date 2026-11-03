import ast
import glob
from pathlib import Path

for path in sorted(glob.glob("🎓️teaching/🏛️architecture/⚡️energy/*/🎬️clip/*/🐍️.py")):
    tree = ast.parse(Path(path).read_text(encoding="utf-8"))
    keys = set()
    for n in ast.walk(tree):
        if isinstance(n, ast.Tuple) and len(n.elts) == 3 and all(isinstance(e, ast.Constant) and isinstance(e.value, str) for e in n.elts):
            keys.add(n.elts[0].value)
    for n in ast.walk(tree):
        if isinstance(n, ast.Call) and isinstance(n.func, ast.Name) and n.func.id in {"subtitle_text", "swap_caption", "hold_for"}:
            for c in ast.walk(n):
                if isinstance(c, ast.Call) and isinstance(c.func, ast.Name) and c.func.id == "tr":
                    print("CALL", path.split("clip")[-1], c.lineno)
        if isinstance(n, ast.Call) and isinstance(n.func, ast.Name) and n.func.id == "tr" and len(n.args) == 2 and isinstance(n.args[1], ast.Constant) and n.args[1].value in keys:
            print("KEY?", path.split("clip")[-1], n.lineno, n.args[1].value)
