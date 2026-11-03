import ast, os, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "🧰️ds"))
from scan_py import emoji_lead

def norm(e):
    return e.replace("️", "").replace("‍", "")

def check_py(path):
    tree = ast.parse(open(path, encoding="utf-8").read())
    seen = collections.defaultdict(list)
    for n in ast.walk(tree):
        if isinstance(n, (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            d = ast.get_docstring(n, clean=False)
            if d is None:
                continue
            lead = emoji_lead(d) or ""
            if lead:
                seen[norm(lead)].append((n.body[0].lineno, getattr(n, "name", "<module>")))
    return {k: v for k, v in seen.items() if len(v) > 1}

if __name__ == "__main__":
    total = 0
    for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
        p = p.strip()
        if not p.endswith(".py") or not os.path.exists(p):
            continue
        g = check_py(p)
        if g:
            total += len(g)
            print("FILE", p.split("🏛️architecture/")[-1])
            for k, v in g.items():
                print("  ", repr(k), v)
    print("TOTAL_GROUPS", total)
