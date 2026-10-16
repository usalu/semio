#!/usr/bin/env python3
# 📚️ The WP1 file universe: module trees of the hub-1 mounts and the root-resident WP1 units, with the owning crate of each production file.
import os, re, sys, json
from lib import *
from rs import blank

MOUNT = re.compile(r"((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*(;|\{)")
PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCLUDE = re.compile(r'\binclude!\(\s*"([^"]+)"\s*\)')


def children(path):
    text = read(path)
    clean = blank(text)
    here = os.path.dirname(path)
    out = []
    for m in MOUNT.finditer(clean):
        if m.group(3) == "{":
            continue
        attrs = text[m.start(1):m.end(1)]
        pm = PATH_ATTR.search(attrs)
        cfg = "test" if re.search(r"cfg\(\s*test\s*\)", attrs) else ""
        if pm:
            out.append((os.path.normpath(os.path.join(here, pm.group(1))), m.group(2), cfg))
        else:
            stem = os.path.splitext(os.path.basename(path))[0]
            base = os.path.join(here, stem)
            for cand in (os.path.join(base, m.group(2) + ".rs"), os.path.join(base, m.group(2), "mod.rs")):
                if os.path.exists(cand):
                    out.append((cand, m.group(2), cfg))
                    break
    for m in INCLUDE.finditer(text):
        if clean[m.start():m.start() + 8] == "include!":
            before = clean[max(0, m.start() - 160):m.start()]
            cfg = "test" if re.search(r"#\[\s*cfg\(\s*test\s*\)\s*\]\s*$", before) else ""
            out.append((os.path.normpath(os.path.join(here, m.group(1))), "<include>", cfg))
    return out


def tree(root):
    seen, order = set(), []

    def walk(p, parent):
        if p in seen or not os.path.exists(p):
            return
        seen.add(p)
        order.append((p, parent))
        for c, name, cfg in children(p):
            walk(c, p)

    walk(os.path.normpath(root), None)
    return order


def tree_split(root):
    """(production files, test-only files): a file is test-only when every mount path to it passes a cfg(test) mount or it lives below a tests directory."""
    root = os.path.normpath(root)
    prod, test = [], []
    seen = {}

    def walk(p, test_only):
        if not os.path.exists(p):
            return
        if p in seen and (seen[p] is False or seen[p] == test_only):
            return
        seen[p] = test_only and seen.get(p, True)
        for c, name, cfg in children(p):
            walk(c, test_only or cfg == "test" or "/🧪️tests/" in c)

    walk(root, False)
    for p, v in seen.items():
        (test if v else prod).append(p)
    return sorted(prod), sorted(test)

if __name__ == "__main__":
    for p, parent in tree(sys.argv[1]):
        print(p.replace(PLUG + "/", ""), "<-", (parent or "").replace(PLUG + "/", ""))
