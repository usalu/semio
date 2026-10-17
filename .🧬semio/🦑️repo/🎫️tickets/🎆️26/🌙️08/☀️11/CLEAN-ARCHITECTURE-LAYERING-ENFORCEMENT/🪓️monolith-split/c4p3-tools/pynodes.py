"""usage: pynodes.py <file>  -> top-level statements with defined names and references to other top-level names (by index)."""
import ast, symtable, sys
src = open(sys.argv[1], encoding="utf-8").read()
tree = ast.parse(src)
lines = src.split("\n")
defined = {}
rows = []
for i, node in enumerate(tree.body):
    names = []
    if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
        names = [node.name]
    elif isinstance(node, ast.Assign):
        for t in node.targets:
            names += [n.id for n in ast.walk(t) if isinstance(n, ast.Name)]
    elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
        names = [node.target.id]
    for n in names:
        defined[n] = i
    rows.append((i, node, names))
for i, node, names in rows:
    seg = ast.get_source_segment(src, node) or ""
    used = {n.id for n in ast.walk(node) if isinstance(n, ast.Name)}
    deps = sorted({defined[n] for n in used if n in defined and defined[n] != i})
    first = node.decorator_list[0].lineno if getattr(node, "decorator_list", None) else node.lineno
    print("%d\t%d-%d\t%s\t%s\t-> %s" % (i, first, node.end_lineno, type(node).__name__, ",".join(names) or seg.split("\n")[0][:50], ",".join(map(str, deps))))
