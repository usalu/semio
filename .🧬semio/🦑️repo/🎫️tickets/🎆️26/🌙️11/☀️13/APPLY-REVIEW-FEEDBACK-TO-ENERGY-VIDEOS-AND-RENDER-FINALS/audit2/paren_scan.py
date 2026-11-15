import ast, pathlib, re, collections, json
ROOT = pathlib.Path(r"C:/git/semio")
AUDIT = pathlib.Path(r"C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS/audit2")
files = [l.strip() for l in (AUDIT / "files.txt").read_text(encoding="utf-8").splitlines() if l.strip()]
paren = re.compile(r"[()]")
MATH = {"MathTex", "Tex", "math_text", "math_label", "MathTex_", "Matrix", "SingleStringMathTex", "tex", "formula", "math", "Formula", "tex_text"}
rows = []
def folder_of(rel):
    parts = rel.split("/")
    try:
        i = parts.index("⚡️energy")
    except ValueError:
        return "other"
    tail = parts[i+1:]
    return tail[0] if tail else "root"
def callname(node):
    f = node.func
    if isinstance(f, ast.Attribute): return f.attr
    if isinstance(f, ast.Name): return f.id
    return "?"
for rel in files:
    if not rel.endswith(".py"):
        continue
    path = ROOT / rel
    if not path.is_file():
        continue
    try:
        tree = ast.parse(path.read_text(encoding="utf-8"))
    except Exception as e:
        rows.append({"file": rel, "error": str(e)}); continue
    parents = {}
    for node in ast.walk(tree):
        for child in ast.iter_child_nodes(node):
            parents[child] = node
    for node in ast.walk(tree):
        if isinstance(node, ast.Constant) and isinstance(node.value, str) and paren.search(node.value):
            # docstring check
            p = parents.get(node)
            if isinstance(p, ast.Expr):
                kind = "docstring"; call = ""
            else:
                call = ""; kind = "string"
                cur = node
                while cur in parents:
                    cur = parents[cur]
                    if isinstance(cur, ast.Call):
                        call = callname(cur); break
                    if isinstance(cur, (ast.FunctionDef, ast.ClassDef, ast.Module)):
                        break
                if isinstance(parents.get(node), ast.JoinedStr):
                    kind = "fstring"
            rows.append({"file": rel, "folder": folder_of(rel), "line": node.lineno, "kind": kind, "call": call, "text": node.value[:160]})
(AUDIT / "paren-scan.jsonl").write_text("\n".join(json.dumps(r, ensure_ascii=False) for r in rows), encoding="utf-8")
print("total rows", len(rows))
c = collections.Counter((r.get("folder"), r.get("kind"), r.get("call")) for r in rows)
for k, v in sorted(c.items(), key=lambda kv: (str(kv[0][0]), -kv[1])):
    print(k, v)
