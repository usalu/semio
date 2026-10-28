import ast, sys, collections
from pathlib import Path
for name in sys.argv[1:]:
    tree = ast.parse(Path(name).read_text(encoding="utf-8"))
    seen = collections.defaultdict(list)
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.ClassDef, ast.Module)):
            doc = ast.get_docstring(node, clean=False)
            if doc is None: print("no docstring", getattr(node, "name", "module")); continue
            seen[doc.split(" ")[0].replace("️", "")].append(getattr(node, "name", "module"))
    for k, v in seen.items():
        if len(v) > 1: print("dup", k, v)
