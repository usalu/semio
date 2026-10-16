#!/usr/bin/env python3
"""🌳️ Module tree of a Rust root: files reachable through `#[path]` mounts, default-resolved `mod x;` and `include!`, with the module path of each file relative to the root."""
import os, re
from lib import *
from rs import blank

MOUNT = re.compile(r"((?:#\[[^\]]*\]\s*)*)(?:(pub(?:\([^)]*\))?)\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*(;|\{)")
PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCLUDE = re.compile(r'\binclude!\(\s*"([^"]+)"\s*\)')


def children(path):
    """[(child file, mod name or '<include>', attrs text, vis)] of declared file mounts and includes"""
    text = read(path)
    clean = blank(text)
    here = os.path.dirname(path)
    out = []
    for m in MOUNT.finditer(clean):
        if m.group(4) == "{":
            continue
        attrs = text[m.start(1):m.end(1)]
        pm = PATH_ATTR.search(attrs)
        if pm:
            out.append((os.path.normpath(os.path.join(here, pm.group(1))), m.group(3), " ".join(attrs.split()), m.group(2) or ""))
        else:
            stem = os.path.splitext(os.path.basename(path))[0]
            base = here if stem == "mod" else os.path.join(here, stem)
            for cand in (os.path.join(base, m.group(3) + ".rs"), os.path.join(base, m.group(3), "mod.rs")):
                if os.path.exists(cand):
                    out.append((cand, m.group(3), " ".join(attrs.split()), m.group(2) or ""))
                    break
    for m in INCLUDE.finditer(text):
        if clean[m.start():m.start() + 8] == "include!":
            out.append((os.path.normpath(os.path.join(here, m.group(1))), "<include>", "", ""))
    return out


def tree(root, stop=lambda p: False):
    """ordered [(file, parent file or None, mod name)]; `stop(file)` prunes a subtree"""
    seen, order = set(), []

    def walk(p, parent, name):
        if p in seen or not os.path.exists(p) or stop(p):
            return
        seen.add(p)
        order.append((p, parent, name))
        for c, n, a, v in children(p):
            walk(c, p, n)

    walk(os.path.normpath(root), None, "")
    return order

if __name__ == "__main__":
    import sys
    for p, parent, n in tree(sys.argv[1]):
        print(rel(p), "<-", rel(parent) if parent else "", n)
