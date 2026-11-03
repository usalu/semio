import ast, json, sys
from pathlib import Path
ROOT = Path(sys.argv[1])
OUT = Path(sys.argv[2])
CALLS = {"staffage", "fitted_staffage", "staffage_motion", "staffage_height", "staffage_parts", "_staffage", "_staffage_parts", "_staffage_motion", "_fitted_staffage", "_cast", "_use_cast", "_figure", "pendant_lamp", "watt_anchor"}
SKIP = {"🧪️tests", ".venv", "node_modules", "__pycache__"}
rows = []
files = sorted(p for p in ROOT.rglob("🐍️.py") if "🎬️clip" in p.parts and not (set(p.parts) & SKIP))
for path in files:
    src = path.read_text(encoding="utf-8")
    try:
        tree = ast.parse(src)
    except SyntaxError as e:
        rows.append({"file": path.relative_to(ROOT).as_posix(), "error": str(e)})
        continue
    consts = {}
    for node in tree.body:
        if isinstance(node, ast.Assign) and len(node.targets) == 1 and isinstance(node.targets[0], ast.Name) and isinstance(node.value, ast.Constant):
            consts[node.targets[0].id] = node.value.value
    for node in ast.walk(tree):
        if not isinstance(node, ast.Call):
            continue
        callee = node.func.id if isinstance(node.func, ast.Name) else node.func.attr if isinstance(node.func, ast.Attribute) else None
        if callee not in CALLS:
            continue
        first = node.args[0] if node.args else None
        cast = None
        if isinstance(first, ast.Constant) and isinstance(first.value, str):
            cast = first.value
        elif isinstance(first, ast.Name) and first.id in consts:
            cast = consts[first.id]
        else:
            cast = "DYN:" + ast.unparse(first)[:60] if first is not None else None
        kw = {k.arg: ast.unparse(k.value)[:80] for k in node.keywords if k.arg}
        rows.append({"file": path.relative_to(ROOT).as_posix(), "line": node.lineno, "call": callee, "cast": cast, "kw": kw})
OUT.write_text(json.dumps(rows, ensure_ascii=False, indent=1), encoding="utf-8")
print(len(files), "files", len(rows), "rows")
