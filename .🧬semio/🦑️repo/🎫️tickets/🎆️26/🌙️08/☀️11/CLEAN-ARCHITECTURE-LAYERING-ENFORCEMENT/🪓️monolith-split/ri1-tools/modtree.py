#!/usr/bin/env python3
# 🌳️ Files reachable from a Rust root through #[path] mounts and include! (declaration order, each once).
import os
import re
from common import read
from rs import blank

MOUNT = re.compile(r"((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;")
PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCLUDE = re.compile(r'\binclude!\(\s*"([^"]+)"\s*\)')


def children(path):
    text = read(path)
    clean = blank(text)
    here = os.path.dirname(path)
    out = []
    # blank() erases string contents, so attribute literals are taken from the original text at the same offsets
    for m in MOUNT.finditer(clean):
        attrs = text[m.start(1):m.end(1)]
        pm = PATH_ATTR.search(attrs)
        if pm:
            out.append(os.path.normpath(os.path.join(here, pm.group(1))))
        else:
            stem = os.path.splitext(os.path.basename(path))[0]
            base = here if stem in ("mod",) else os.path.join(here, stem)
            for cand in (os.path.join(base, m.group(2) + ".rs"), os.path.join(base, m.group(2), "mod.rs")):
                if os.path.exists(cand):
                    out.append(cand)
                    break
    for m in INCLUDE.finditer(text):
        if clean[m.start():m.start() + 8] == "include!":
            out.append(os.path.normpath(os.path.join(here, m.group(1))))
    return out


def tree(root, skip_tests=False):
    seen, order = set(), []

    def walk(p):
        if p in seen or not os.path.exists(p):
            return
        seen.add(p)
        order.append(p)
        for c in children(p):
            walk(c)

    walk(os.path.normpath(root))
    return order
