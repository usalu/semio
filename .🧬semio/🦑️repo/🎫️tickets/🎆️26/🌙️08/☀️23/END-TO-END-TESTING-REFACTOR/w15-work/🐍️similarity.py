"""📐 5-gram Jaccard similarity over Python adapters, docstrings and comments stripped."""
import ast, io, itertools, json, sys, tokenize
from pathlib import Path


def token_stream(path):
    source = Path(path).read_text(encoding="utf-8")
    tree = ast.parse(source)
    # drop docstrings
    for node in ast.walk(tree):
        if isinstance(node, (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            body = node.body
            if body and isinstance(body[0], ast.Expr) and isinstance(body[0].value, ast.Constant) and isinstance(body[0].value.value, str):
                node.body = body[1:] or [ast.Pass()]
    stripped = ast.unparse(ast.fix_missing_locations(tree))
    out = []
    for tok in tokenize.generate_tokens(io.StringIO(stripped).readline):
        if tok.type in (tokenize.COMMENT, tokenize.NL, tokenize.NEWLINE, tokenize.INDENT, tokenize.DEDENT, tokenize.ENCODING, tokenize.ENDMARKER):
            continue
        out.append(tok.string)
    return out


def grams(tokens, n=5):
    return {tuple(tokens[i:i + n]) for i in range(len(tokens) - n + 1)}


def main(paths):
    sets = {p: grams(token_stream(p)) for p in paths}
    rows = []
    for a, b in itertools.combinations(paths, 2):
        A, B = sets[a], sets[b]
        j = len(A & B) / len(A | B) if (A | B) else 1.0
        rows.append((j, a, b))
    rows.sort(reverse=True)
    for j, a, b in rows:
        print("%.4f  %s  %s" % (j, Path(a).parent.name, Path(b).parent.name))
    vals = [r[0] for r in rows]
    print("pairs=%d min=%.4f max=%.4f mean=%.4f" % (len(vals), min(vals), max(vals), sum(vals) / len(vals)))
    print("tokens: " + json.dumps({Path(p).parent.name: len(token_stream(p)) for p in paths}))


main(sys.argv[1:])
