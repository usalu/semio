import ast
import io
import os
import re
import sys

ROOT = r"C:\git\semio"
LIST = os.path.join(os.path.dirname(os.path.abspath(__file__)), "files.txt")


def files():
    with io.open(LIST, encoding="utf-8") as handle:
        for line in handle:
            path = line.strip()
            if path.endswith(".py") and "🧪️tests" not in path.split("🧪️tests")[0] and False:
                continue
            if path:
                yield path


def comments_in_defs(source, tree):
    lines = source.splitlines()
    spans = []
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            spans.append((node.lineno, node.end_lineno))
    hits = []
    for number, text in enumerate(lines, 1):
        stripped = text.strip()
        if stripped.startswith("#") and any(a < number <= b for a, b in spans):
            hits.append((number, stripped[:80]))
    return hits


def docstring_problems(tree):
    problems = []
    for node in ast.walk(tree):
        if isinstance(node, (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            doc = ast.get_docstring(node, clean=False)
            if doc is None:
                continue
            first = doc.strip()[:1]
            if not first or (ord(first) < 128) or first.isalpha():
                problems.append((getattr(node, "lineno", 1), doc.strip()[:60]))
    return problems


TOKENS = {}


def tokens():
    if not TOKENS:
        from collections import Counter
        counter = Counter()
        for base, _, names in os.walk(os.path.join(ROOT, "🎓️teaching", "🏛️architecture")):
            for name in names:
                if name.endswith((".py", ".ts")):
                    with io.open(os.path.join(base, name), encoding="utf-8", errors="ignore") as handle:
                        counter.update(re.findall(r"[A-Za-z_]\w*", handle.read()))
        TOKENS.update(counter)
    return TOKENS


def dead_defs(tree, source, rel):
    names = [n.name for n in tree.body if isinstance(n, ast.FunctionDef)]
    return [name for name in names if not name.startswith("test_") and tokens().get(name, 0) <= 1]


def main():
    for rel in files():
        full = os.path.join(ROOT, rel)
        if not rel.endswith(".py") or not os.path.exists(full):
            if rel.endswith(".py"):
                print("MISSING", rel)
            continue
        with io.open(full, encoding="utf-8") as handle:
            source = handle.read()
        try:
            tree = ast.parse(source)
        except SyntaxError as error:
            print("SYNTAX", rel, error)
            continue
        for number, text in comments_in_defs(source, tree):
            print("COMMENT-IN-DEF", rel, number, text)
        for number, text in docstring_problems(tree):
            print("DOCSTRING-NOEMOJI", rel, number, text)
        for name in dead_defs(tree, source, rel):
            print("DEAD-DEF?", rel, name)
        for number, line in enumerate(source.splitlines(), 1):
            if "[DEBUG]" in line:
                print("DEBUG", rel, number)
    return 0


if __name__ == "__main__":
    sys.exit(main())
