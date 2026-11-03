import ast
import glob
import sys
from pathlib import Path

ROOT = "🎓️teaching/🏛️architecture/⚡️energy"
for path in sorted(glob.glob(f"{ROOT}/*/🎬️clip/*/🐍️.py")):
    src = Path(path).read_text(encoding="utf-8")
    tree = ast.parse(src)
    parents = {}
    for p in ast.walk(tree):
        for c in ast.iter_child_nodes(p):
            parents[c] = p
    mod, shadow = [], []
    for n in ast.walk(tree):
        if isinstance(n, ast.Call) and isinstance(n.func, ast.Name) and n.func.id == "tr":
            p = parents.get(n)
            fn = None
            while p is not None:
                if isinstance(p, (ast.FunctionDef, ast.AsyncFunctionDef, ast.Lambda)):
                    fn = p
                    break
                p = parents.get(p)
            if fn is None:
                mod.append(n.lineno)
            else:
                if isinstance(fn, ast.Lambda):
                    continue
                for s in ast.walk(fn):
                    if isinstance(s, ast.Name) and s.id == "tr" and isinstance(s.ctx, ast.Store):
                        shadow.append((fn.name, n.lineno))
                        break
                for a in fn.args.args + fn.args.kwonlyargs:
                    if a.arg == "tr":
                        shadow.append((fn.name, n.lineno))
    if mod or shadow:
        print(path.split("clip")[-1], "MODULE-LEVEL:", mod[:12], "SHADOW:", sorted(set(shadow))[:6])
