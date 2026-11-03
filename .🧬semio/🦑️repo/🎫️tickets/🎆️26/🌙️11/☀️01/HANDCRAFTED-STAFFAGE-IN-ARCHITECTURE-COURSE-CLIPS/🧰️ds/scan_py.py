import ast, io, sys, tokenize, unicodedata, collections

def emoji_lead(s):
    s = s.lstrip()
    if not s:
        return None
    i = 0
    out = ""
    while i < len(s) and ord(s[i]) > 0x2000 and not s[i].isalnum():
        out += s[i]
        i += 1
    return out or None

def scan(path):
    src = open(path, encoding="utf-8").read()
    tree = ast.parse(src)
    res = collections.defaultdict(list)
    seen = collections.defaultdict(list)
    spans = []
    for n in ast.walk(tree):
        if isinstance(n, (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            d = ast.get_docstring(n, clean=False)
            if d is not None:
                lead = emoji_lead(d)
                line = n.body[0].lineno
                name = getattr(n, "name", "<module>")
                if lead is None:
                    res["noemoji"].append((line, name, d.strip()[:30]))
                else:
                    seen[lead].append((line, name))
            if not isinstance(n, ast.Module):
                spans.append((n.body[0].lineno, n.end_lineno))
    for k, v in seen.items():
        if len(v) > 1:
            res["dup"].append((k, v))
    try:
        for t in tokenize.generate_tokens(io.StringIO(src).readline):
            if t.type == tokenize.COMMENT:
                l = t.start[0]
                if any(a <= l <= b for a, b in spans):
                    res["comment"].append((l, t.string[:50]))
    except Exception as e:
        res["tokerr"].append(str(e))
    for i, line in enumerate(src.splitlines(), 1):
        if "print(" in line and "[DEBUG]" not in line:
            res["print"].append((i, line.strip()[:60]))
    defs = [n.name for n in ast.walk(tree) if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef))]
    res["defs"] = defs
    return res

if __name__ == "__main__":
    for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
        p = p.strip().strip('"')
        if not p.endswith(".py"):
            continue
        try:
            r = scan(p)
        except Exception as e:
            print("ERR", p, e)
            continue
        short = p.split("🏛️architecture/")[-1]
        for k in ("noemoji", "dup", "comment", "print", "tokerr"):
            for item in r[k]:
                print(short, k, item)
