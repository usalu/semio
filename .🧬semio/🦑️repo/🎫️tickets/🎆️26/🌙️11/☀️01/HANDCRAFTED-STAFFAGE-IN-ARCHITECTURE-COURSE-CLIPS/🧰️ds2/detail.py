import ast, os, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "🧰️ds"))
from scan_py import emoji_lead

def holders(path):
    src = open(path, encoding="utf-8").read()
    tree = ast.parse(src)
    seen = collections.defaultdict(list)
    for n in ast.walk(tree):
        if isinstance(n, (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            d = ast.get_docstring(n, clean=False)
            if d is None:
                continue
            lead = emoji_lead(d)
            line = n.body[0].lineno
            name = getattr(n, "name", "<module>")
            text = d.lstrip()[len(lead or ""):].strip().replace("\n", " ")[:90]
            seen[lead or ""].append((line, name, text))
    return seen

if __name__ == "__main__":
    for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
        p = p.strip()
        if not p.endswith(".py") or not os.path.exists(p):
            continue
        seen = holders(p)
        groups = {k: v for k, v in seen.items() if len(v) > 1}
        if not groups:
            continue
        short = p.split("🏛️architecture/")[-1]
        print("FILE", short)
        for k, v in groups.items():
            print("  GROUP", repr(k))
            for line, name, text in v:
                print("    %d %s | %s" % (line, name, text))
