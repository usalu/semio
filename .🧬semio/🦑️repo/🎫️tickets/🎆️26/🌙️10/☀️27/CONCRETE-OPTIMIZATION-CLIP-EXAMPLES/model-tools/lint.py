import ast, sys
from pathlib import Path
for name in sys.argv[1:]:
    tree = ast.parse(Path(name).read_text(encoding="utf-8"))
    for fn in ast.walk(tree):
        if isinstance(fn, (ast.FunctionDef, ast.Lambda)) and isinstance(fn, ast.FunctionDef):
            assigned = {}
            used = set()
            for node in ast.walk(fn):
                if isinstance(node, ast.Name):
                    if isinstance(node.ctx, ast.Store): assigned.setdefault(node.id, node.lineno)
                    else: used.add(node.id)
            for n, l in assigned.items():
                if n not in used and n != "_": print(name[-20:], fn.name, n, l)
            for node in ast.walk(fn):
                if isinstance(node, ast.Constant) and False: pass
            comments = 0
