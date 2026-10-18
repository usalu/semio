"""🌳️ C5G python twin of `readModuleTreeSource` (parent first, `#[path]` and literal `include!` children in declaration order, canonical `🧪️tests` mounts and paths leaving the scope skipped) used to prove the re-pointed source-text assertions without a TS runtime."""
import os, re
REPO = "/Users/ueli/Documents/semio"
PATH = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;')
INC = re.compile(r'(?m)^\s*include!\(\s*"([^"]+)"\s*\)\s*;')
def children(file, text, scope):
    d = os.path.dirname(file)
    out = []
    ms = [(m.start(), m.group(1), "path") for m in PATH.finditer(text)] + [(m.start(), m.group(1), "inc") for m in INC.finditer(text)]
    for _, lit, via in sorted(ms):
        target = os.path.normpath(os.path.join(d, lit))
        rel = os.path.relpath(target, scope).split(os.sep)
        if "🧪️tests" in rel or rel[0] == "..":
            continue
        out.append((target, scope if via == "inc" else os.path.dirname(target)))
    return out
def files(root):
    seen, res = set(), []
    def visit(f, scope):
        if f in seen: return
        seen.add(f)
        t = open(f, encoding="utf-8").read()
        res.append((f, t))
        for c, s in children(f, t, scope): visit(c, s)
    visit(root, os.path.dirname(root))
    return res
def tree(root):
    return "".join(t if t.endswith("\n") else t + "\n" for _, t in files(root))
