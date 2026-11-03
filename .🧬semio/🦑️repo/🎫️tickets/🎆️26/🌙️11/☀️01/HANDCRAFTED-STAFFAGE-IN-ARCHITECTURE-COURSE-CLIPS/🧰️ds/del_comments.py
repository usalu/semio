import io, os, sys, tokenize, ast, glob

def run(path):
    raw = open(path, "rb").read()
    src = raw.decode("utf-8")
    nl = "\r\n" if "\r\n" in src else "\n"
    text = src.replace("\r\n", "\n")
    tree = ast.parse(text)
    spans = [(n.body[0].lineno, n.end_lineno) for n in ast.walk(tree) if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef))]
    lines = text.split("\n")
    drop = set()
    for t in tokenize.generate_tokens(io.StringIO(text).readline):
        if t.type == tokenize.COMMENT:
            l = t.start[0]
            if any(a <= l <= b for a, b in spans) and lines[l - 1].strip().startswith("#"):
                drop.add(l - 1)
    if not drop:
        return 0
    out = [x for i, x in enumerate(lines) if i not in drop]
    new = nl.join(out)
    ast.parse(new.replace("\r\n", "\n"))
    tmp = path + ".dstmp"
    open(tmp, "wb").write(new.encode("utf-8"))
    os.replace(tmp, path)
    return len(drop)

if __name__ == "__main__":
    for p in sys.argv[1:]:
        print(p, run(p))
