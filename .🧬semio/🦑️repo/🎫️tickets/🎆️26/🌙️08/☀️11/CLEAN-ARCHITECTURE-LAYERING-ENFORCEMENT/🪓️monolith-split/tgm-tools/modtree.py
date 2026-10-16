"""Module-tree walk of a Rust crate: reachable files through #[path] mounts and include!, inline-mod nesting aware."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from tgmlib import *

PATH_ATTR = re.compile(r"#\s*\[\s*path\s*=")
MOD_RX = re.compile(r"\bmod\s+([A-Za-z_]\w*)\s*([;{])")
INC_RX = re.compile(r"\binclude!\s*\(")


def mounts_of(text):
    """Return list of (kind, relpath, inline_mod_names) in a file; kind in path|include|plain."""
    m = mask(text)
    out = []
    stack = []
    depth = 0
    pend_path = None
    i = 0
    n = len(m)
    mod_at = {}
    for mm in MOD_RX.finditer(m):
        mod_at[mm.start()] = mm
    path_at = {}
    for pm in PATH_ATTR.finditer(m):
        lit = re.match(r'\s*"([^"]*)"', text[pm.end():pm.end() + 400])
        if lit:
            path_at[pm.start()] = lit.group(1)
    inc_at = {}
    for im in INC_RX.finditer(m):
        lit = re.match(r'\s*"([^"]*)"', text[im.end():im.end() + 400])
        if lit:
            inc_at[im.start()] = lit.group(1)
    while i < n:
        c = m[i]
        if i in path_at:
            pend_path = path_at[i]
        if i in inc_at:
            out.append(("include", inc_at[i], list(stack_names(stack))))
        if i in mod_at:
            mm = mod_at[i]
            if mm.group(2) == ";":
                if pend_path is not None:
                    out.append(("path", pend_path, list(stack_names(stack))))
                else:
                    out.append(("plain", mm.group(1), list(stack_names(stack))))
                pend_path = None
            else:
                stack.append((mm.group(1) if pend_path is None else pend_path, depth))
                pend_path = None
                depth += 1
                i = mm.end()
                continue
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            while stack and stack[-1][1] >= depth:
                stack.pop()
        i += 1
    return out


def stack_names(stack):
    return [s[0] for s in stack]


def reachable(lib_abs, include_includes=True):
    """Files reachable from lib_abs; returns dict path->list of (kind, target)."""
    seen = {}
    todo = [lib_abs]
    while todo:
        f = todo.pop()
        if f in seen or not os.path.isfile(f):
            continue
        text = read(f)
        ms = mounts_of(text)
        seen[f] = []
        d = os.path.dirname(f)
        for kind, p, inl in ms:
            if kind == "path":
                t = os.path.normpath(os.path.join(d, *inl, p))
            elif kind == "include":
                t = os.path.normpath(os.path.join(d, p))
            else:
                t = None
            seen[f].append((kind, p, t))
            if t and t.endswith(".rs"):
                todo.append(t)
    return seen


if __name__ == "__main__":
    base = sys.argv[1]
    lib = os.path.join(base, sys.argv[2])
    r = reachable(lib)
    for f, ms in sorted(r.items()):
        print(os.path.relpath(f, base), len(read(f).split("\n")))
        for k, p, t in ms:
            if k == "plain":
                print("   PLAIN mod", p)
            elif not (t and os.path.isfile(t)):
                print("   MISSING", k, p)
