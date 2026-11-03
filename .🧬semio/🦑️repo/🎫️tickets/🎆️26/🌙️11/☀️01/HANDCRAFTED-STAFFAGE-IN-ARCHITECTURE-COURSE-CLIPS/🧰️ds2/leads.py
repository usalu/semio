import ast, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "🧰️ds"))
from scan_py import emoji_lead

if __name__ == "__main__":
    targets = sys.argv[2:]
    for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
        p = p.strip()
        if not p.endswith(".py") or not os.path.exists(p):
            continue
        short = p.split("🏛️architecture/")[-1]
        if not any(t in short for t in targets):
            continue
        tree = ast.parse(open(p, encoding="utf-8").read())
        items = []
        for n in ast.walk(tree):
            if isinstance(n, (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
                d = ast.get_docstring(n, clean=False)
                if d is not None:
                    items.append((n.body[0].lineno, emoji_lead(d) or "-"))
        items.sort()
        print("FILE", short)
        print("  " + " ".join("%d:%s" % (l, e) for l, e in items))
