import ast
import io
import os
import sys
import tokenize

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = r"C:\git\semio"
DEFS = (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)


def read(path):
    stat = os.stat(path)
    with io.open(path, encoding="utf-8", newline="") as handle:
        return handle.read(), stat.st_mtime_ns


def write(path, text, mtime):
    if os.stat(path).st_mtime_ns != mtime:
        print("CONFLICT", path)
        return False
    tmp = os.path.join(HERE, "tmp-" + str(os.getpid()) + ".txt")
    with io.open(tmp, "w", encoding="utf-8", newline="") as handle:
        handle.write(text)
    os.replace(tmp, path)
    return True


def strip_comments(path):
    text, mtime = read(path)
    tree = ast.parse(text)
    before = ast.dump(tree)
    spans = [(node.lineno, node.end_lineno) for node in ast.walk(tree) if isinstance(node, DEFS)]
    lines = text.splitlines(keepends=True)
    edits = {}
    for tok in tokenize.generate_tokens(io.StringIO(text).readline):
        if tok.type != tokenize.COMMENT:
            continue
        row, col = tok.start
        if not any(a < row <= b for a, b in spans):
            continue
        head = lines[row - 1][:col]
        edits[row] = None if not head.strip() else head.rstrip() + ("\n" if lines[row - 1].endswith("\n") else "")
    if not edits:
        return 0
    out = []
    for number, line in enumerate(lines, 1):
        if number in edits:
            if edits[number] is not None:
                out.append(edits[number])
        else:
            out.append(line)
    new = "".join(out)
    if ast.dump(ast.parse(new)) != before:
        print("AST-CHANGED (skipped)", path)
        return 0
    if write(path, new, mtime):
        print("COMMENTS", len(edits), path)
    return len(edits)


def drop_defs(path, names):
    text, mtime = read(path)
    tree = ast.parse(text)
    before = ast.dump(tree)
    lines = text.splitlines(keepends=True)
    kill = set()
    for node in list(tree.body):
        if isinstance(node, ast.FunctionDef) and node.name in names:
            first = min([node.lineno] + [d.lineno for d in node.decorator_list])
            kill.update(range(first, node.end_lineno + 1))
            tree.body.remove(node)
            names = names - {node.name}
    if names:
        print("NOT-FOUND", names, path)
    if not kill:
        return 0
    trailing = max(kill)
    while trailing < len(lines) and not lines[trailing].strip():
        kill.add(trailing + 1)
        trailing += 1
    out = [line for number, line in enumerate(lines, 1) if number not in kill]
    new = "".join(out)
    if ast.dump(ast.parse(new)) != ast.dump(tree):
        print("AST-MISMATCH (skipped)", path)
        return 0
    if write(path, new, mtime):
        print("DROPPED", len(kill), path)
    return len(kill)


def main(argv):
    mode = argv[1]
    if mode == "comments":
        for path in argv[2:]:
            strip_comments(path)
    elif mode == "defs":
        names = set(argv[2].split(","))
        drop_defs(argv[3], names)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
