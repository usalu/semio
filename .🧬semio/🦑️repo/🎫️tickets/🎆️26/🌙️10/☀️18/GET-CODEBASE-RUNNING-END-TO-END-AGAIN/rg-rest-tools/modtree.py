"""Module tree of one crate lib root: modpath -> file, with crate::/super:: reference summary. usage: modtree.py <manifest-rel> [--refs]"""
import sys, os, re, json
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rgplug-tools")
from rgp import *

ATTR = re.compile(r'#\[([^\]]*(?:\[[^\]]*\][^\]]*)*)\]')

def strip_comments(s):
    return re.sub(r'(?m)^\s*//[^\n]*\n', '', s)

def parse_chunk(chunk):
    """Return (attrs, kind, name, body or None)."""
    body = strip_comments(chunk)
    attrs = []
    pos = 0
    while True:
        m = re.match(r"\s*#!?\[((?:[^\[\]]|\[[^\]]*\])*)\]", body[pos:])
        if not m:
            break
        attrs.append(m.group(1).strip())
        pos += m.end()
    rest = body[pos:].lstrip()
    m = re.match(r'(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z0-9_]+)\s*(;|\{)', rest)
    if not m:
        return attrs, None, None, None
    if m.group(2) == ";":
        return attrs, "mod", m.group(1), None
    i = rest.index("{", m.start(2))
    j = block_bounds(rest, i)
    return attrs, "mod", m.group(1), rest[i + 1:j - 1]

def attr_path(attrs):
    for a in attrs:
        m = re.match(r'path\s*=\s*"([^"]*)"', a)
        if m:
            return m.group(1)
    return None

def attr_cfg(attrs):
    return [a for a in attrs if a.startswith("cfg")]

class Node:
    def __init__(self, modpath, file, parent_file, cfg, inline_in=None):
        self.modpath = modpath; self.file = file; self.parent_file = parent_file; self.cfg = cfg; self.inline_in = inline_in

def walk(file, dir_, modpath, nodes, cfg=()):
    """file: abs path; dir_: directory for child path attrs; modpath tuple."""
    try:
        src = rd(file)
    except Exception:
        return
    walk_src(src, file, dir_, modpath, nodes, cfg)

def walk_src(src, file, dir_, modpath, nodes, cfg):
    for ch in top_items(src):
        attrs, kind, name, body = parse_chunk(ch)
        if kind != "mod":
            continue
        p = attr_path(attrs)
        c = cfg + tuple(attr_cfg(attrs))
        mp = modpath + (name,)
        if body is None:
            if p is None:
                cand = [os.path.join(dir_, name + ".rs"), os.path.join(dir_, name, "mod.rs")]
                f = next((x for x in cand if os.path.exists(x)), cand[0])
            else:
                f = os.path.normpath(os.path.join(dir_, p))
            nodes.append(Node(mp, f, file, c))
            walk(f, os.path.dirname(f), mp, nodes, c)
        else:
            nd = dir_ if p == "." else (os.path.normpath(os.path.join(dir_, p)) if p else os.path.join(dir_, name))
            nodes.append(Node(mp, None, file, c, inline_in=file))
            walk_src(body, file, nd, mp, nodes, c)

def crate_root(manifest):
    t = rd(manifest)
    d = os.path.dirname(manifest)
    m = re.search(r'(?ms)^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"', t)
    return os.path.normpath(os.path.join(ROOT, d, m.group(1) if m else "src/lib.rs"))

def tree(manifest):
    root = crate_root(manifest)
    nodes = []
    walk(root, os.path.dirname(root), (), nodes)
    return root, nodes

if __name__ == "__main__":
    mf = sys.argv[1]
    refs = "--refs" in sys.argv
    root, nodes = tree(mf)
    base = os.path.dirname(os.path.dirname(os.path.dirname(ROOT + "/" + mf)))
    print("ROOT", root.replace(ROOT + "/", "")[-90:])
    for n in nodes:
        if n.file is None:
            print("  " * len(n.modpath), "::".join(n.modpath), "(inline)")
            continue
        t = rd(n.file) if os.path.exists(n.file) else ""
        lines = len([l for l in t.split("\n") if l.strip() and not l.strip().startswith("//")])
        info = ""
        if refs:
            cr = sorted(set(re.findall(r'\bcrate::([A-Za-z0-9_]+(?:::[A-Za-z0-9_]+){0,3})', t)))
            sup = len(re.findall(r'\bsuper::', t))
            glob = "GLOB" if re.search(r'use\s+super::\*', t) else ""
            info = " crate=%s super=%d %s" % (cr[:6], sup, glob)
        print("  " * len(n.modpath), n.modpath[-1], lines, os.path.relpath(n.file, os.path.dirname(os.path.dirname(os.path.dirname(root))))[-60:], list(n.cfg) if n.cfg else "", info)
