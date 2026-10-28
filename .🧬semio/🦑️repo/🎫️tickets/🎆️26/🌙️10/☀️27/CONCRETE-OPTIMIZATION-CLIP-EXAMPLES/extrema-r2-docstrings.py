import ast, sys, collections
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
tree = ast.parse(open(p, encoding="utf-8").read())
seen = collections.defaultdict(list)
for node in ast.walk(tree):
    if isinstance(node, (ast.FunctionDef, ast.ClassDef, ast.Module)):
        doc = ast.get_docstring(node)
        if doc:
            seen[doc.split(" ")[0]].append(getattr(node, "name", "module"))
        elif not isinstance(node, ast.Module):
            print("[DEBUG] no docstring", node.name)
for k, v in seen.items():
    if len(v) > 1: print("[DEBUG] duplicate", k, v)
print("[DEBUG] checked", sum(len(v) for v in seen.values()))
