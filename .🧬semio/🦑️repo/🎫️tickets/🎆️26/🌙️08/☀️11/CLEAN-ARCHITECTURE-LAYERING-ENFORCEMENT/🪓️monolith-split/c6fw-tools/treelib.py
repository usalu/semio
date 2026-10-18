"""🌳 c6fw module-tree reader (python mirror of readModuleTreeSource: parent first, children in declaration order, `#[path]` and `include!("lit")` mounts, test mounts and mounts above the scope skipped)."""
import os, re
R = "/Users/ueli/Documents/semio"
PATHRX = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCRX = re.compile(r'\binclude!\(\s*"([^"]+)"\s*\)')
TESTS = "🧪️tests"


def children(file, text, scope):
    out = []
    d = os.path.dirname(file)
    marks = sorted([(m.start(), m.group(1), "path") for m in PATHRX.finditer(text)] + [(m.start(), m.group(1), "inc") for m in INCRX.finditer(text)])
    for _, rel, via in marks:
        target = os.path.normpath(os.path.join(d, rel))
        seg = os.path.relpath(target, scope).split(os.sep)
        if TESTS in seg:
            continue
        if seg[0] == "..":
            continue
        if not os.path.exists(target):
            continue
        out.append((target, scope if via == "inc" else os.path.dirname(target)))
    return out


def tree(root):
    files, seen = [], set()
    def visit(f, scope):
        if f in seen:
            return
        seen.add(f)
        t = open(f, encoding="utf-8", errors="ignore").read()
        files.append((f, t))
        for c, s in children(f, t, scope):
            visit(c, s)
    visit(root, os.path.dirname(root))
    return files


def tree_text(root):
    return "".join(t if t.endswith("\n") else t + "\n" for _, t in tree(root))
