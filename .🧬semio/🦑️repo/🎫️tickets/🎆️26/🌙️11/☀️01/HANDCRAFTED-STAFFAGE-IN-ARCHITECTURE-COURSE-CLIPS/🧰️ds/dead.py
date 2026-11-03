import ast, re, sys, os, subprocess
sys.path.insert(0, os.path.dirname(__file__))

cands = []
for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
    if not p.endswith(".py"):
        continue
    src = open(p, encoding="utf-8").read()
    tree = ast.parse(src)
    for n in ast.walk(tree):
        if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            nm = n.name
            if nm.startswith("__") or nm.startswith("test") or nm.startswith("Beat") or nm in ("construct", "setUp", "tearDown", "setUpClass", "tearDownClass", "main"):
                continue
            if any(isinstance(d, ast.Name) and d.id in ("property",) for d in n.decorator_list):
                pass
            cnt = len(re.findall(r"(?<![\w])" + re.escape(nm) + r"(?![\w])", src))
            if cnt <= 1:
                cands.append((p, nm, n.lineno))
print(len(cands), "in-file unreferenced")
open(sys.argv[2], "w", encoding="utf-8").write("\n".join(f"{p}\t{n}\t{l}" for p, n, l in cands))
