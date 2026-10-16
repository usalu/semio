#!/usr/bin/env python3
"""🧱️ Module graph of a Rust crate tree: each node merges the file text of its module with the text of every `include!`d file (textual inclusion), keeps its items, `use` entries and child modules (file mounts and inline), and remembers which file part each item came from."""
import os, re
from lib import *
from rs import blank, expand_use
import rsparse as P

PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCLUDE = re.compile(r'include!\(\s*"([^"]+)"\s*\)\s*;?')


class Part:
    def __init__(self, file, text, lo, hi):
        self.file, self.text, self.lo, self.hi = file, text, lo, hi
        self.clean = blank(text)
        self.host_part = None
        self.include_attrs = ""
        self.node = None


class Item:
    def __init__(self, kind, name, vis, attrs, part, stmt):
        self.kind, self.name, self.vis, self.attrs, self.part, self.stmt = kind, name, vis, attrs, part, stmt


class Use:
    def __init__(self, path, alias, glob, vis, attrs, part, stmt):
        self.path, self.alias, self.glob, self.vis, self.attrs, self.part, self.stmt = path, alias, glob, vis, attrs, part, stmt

    @property
    def local(self):
        if self.glob:
            return None
        return self.alias or (self.path[-1] if self.path and self.path[-1] != "self" else (self.path[-2] if len(self.path) > 1 else None))


class Node:
    def __init__(self, name, parent, file, inline=False):
        self.name, self.parent, self.file, self.inline = name, parent, file, inline
        self.items, self.uses, self.mods, self.parts, self.macros = [], [], {}, [], []
        self.externs = []
        self.crate = None

    def path(self):
        p, n = [], self
        while n.parent is not None:
            p.append(n.name)
            n = n.parent
        return list(reversed(p))

    def base_dir(self):
        if self.inline:
            return os.path.join(self.parent.base_dir(), self.name)
        if getattr(self, "via_path", True):
            return os.path.dirname(self.file)
        return os.path.join(os.path.dirname(self.file), os.path.splitext(os.path.basename(self.file))[0])

    def root(self):
        n = self
        while n.parent is not None:
            n = n.parent
        return n


def resolve_child(node, name, attrs, file):
    """file of `mod name;` declared in `node` by text of `file` (None when absent). `#[path]` is relative to the directory of the declaring file (plus the inline module names when the module header lives in that file)."""
    pm = PATH_ATTR.search(attrs or "")
    base = node.base_dir() if file == node.file else os.path.dirname(file)
    if pm:
        return os.path.normpath(os.path.join(base, pm.group(1)))
    for cand in (os.path.join(base, name + ".rs"), os.path.join(base, name, "mod.rs")):
        if os.path.exists(cand):
            return cand
    return None


def _scan(node, file, text, lo, hi, loader, seen_files, host_part=None, include_attrs=""):
    part = Part(file, text, lo, hi)
    part.host_part = host_part
    part.include_attrs = include_attrs
    part.node = node
    node.parts.append(part)
    clean = part.clean
    for st in P.statements(text, clean, lo, hi):
        if st.kind == "use":
            for path, alias, glob in P.use_entries(st, clean):
                node.uses.append(Use(path, alias, glob, st.vis, st.attrs, part, st))
        elif st.kind == "extern crate":
            node.externs.append(Item("extern crate", st.name, st.vis, st.attrs, part, st))
            node.items.append(Item("extern crate", st.name, st.vis, st.attrs, part, st))
        elif st.kind == "mod":
            if st.inline:
                child = Node(st.name, node, file, inline=True)
                child.attrs = st.attrs
                child.vis = st.vis
                node.mods[st.name] = child
                node.items.append(Item("mod", st.name, st.vis, st.attrs, part, st))
                _scan(child, file, text, st.body[0], st.body[1], loader, seen_files)
                child.mount_part = part
            else:
                node.items.append(Item("mod", st.name, st.vis, st.attrs, part, st))
                cf = resolve_child(node, st.name, st.attrs, file)
                if cf and os.path.exists(cf):
                    child = Node(st.name, node, cf)
                    child.via_path = bool(PATH_ATTR.search(st.attrs or "")) or os.path.basename(cf) in ("mod.rs", "🦀️.rs")
                    child.attrs = st.attrs
                    child.vis = st.vis
                    child.mount_stmt = (node, st)
                    child.mount_part = part
                    node.mods[st.name] = child
                    loader(child, cf)
                else:
                    child = Node(st.name, node, cf or "")
                    child.attrs = st.attrs
                    child.vis = st.vis
                    child.missing = True
                    node.mods[st.name] = child
        elif st.kind == "macro_rules!":
            node.macros.append(Item("macro", st.name, st.vis, st.attrs, part, st))
            node.items.append(Item("macro", st.name, st.vis, st.attrs, part, st))
        elif st.kind in ("fn", "struct", "enum", "union", "trait", "type", "const", "static"):
            if st.name:
                node.items.append(Item(st.kind, st.name, st.vis, st.attrs, part, st))
        elif st.kind == "other":
            attrs_text, body_at = P.split_attrs(text, clean, st)
            body = clean[body_at:st.end]
            m = INCLUDE.match(text, body_at) if clean[body_at:body_at + 8] == "include!" else None
            if m:
                inc = os.path.normpath(os.path.join(os.path.dirname(file), m.group(1)))
                if os.path.exists(inc) and inc.endswith(".rs"):
                    node.includes = getattr(node, "includes", [])
                    node.includes.append((inc, st.attrs))
                    itext = read(inc)
                    _scan(node, inc, itext, 0, len(itext), loader, seen_files, host_part=part, include_attrs=attrs_text)
            elif re.match(r"(thread_local|component_persistent_local|lazy_static)!", body.lstrip()):
                for sm in re.finditer(r"\bstatic\s+(?:ref\s+)?([A-Za-z_][A-Za-z0-9_]*)", body):
                    node.items.append(Item("static", sm.group(1), "", st.attrs, part, st))
        # impl / other: ignored for the namespace


def load_crate(root_file, key=None):
    """root Node of the module tree below root_file (textual includes merged)"""
    seen = set()
    root = Node("", None, root_file)
    root.crate = key
    root.attrs = ""
    root.vis = "pub"

    def loader(node, file):
        if file in seen:
            return
        seen.add(file)
        text = read(file)
        node.crate = key
        _scan(node, file, text, 0, len(text), loader, seen)

    loader(root, root_file)
    return root


def walk(node):
    yield node
    for c in node.mods.values():
        yield from walk(c)


def files_of(node):
    out = []
    for n in walk(node):
        for p in n.parts:
            if p.file not in out:
                out.append(p.file)
    return out

if __name__ == "__main__":
    import sys
    r = load_crate(sys.argv[1])
    for n in walk(r):
        print("/".join(n.path()) or "<root>", rel(n.file) if n.file else "?", "items", len(n.items), "uses", len(n.uses), "parts", len(n.parts))


def attach(parent, name, file, key="FACADE"):
    """load `file` as an extra child module `name` of parent (a file the monolith does not mount yet, for example a lift file the owner wires at the flip)"""
    seen = set()
    child = Node(name, parent, file)
    child.via_path = True
    child.attrs = ""
    child.vis = "pub"
    child.crate = key
    child.mount_stmt = None
    child.mount_part = parent.parts[0]

    def loader(node, f):
        if f in seen:
            return
        seen.add(f)
        text = read(f)
        node.crate = key
        _scan(node, f, text, 0, len(text), loader, seen)

    parent.mods[name] = child
    loader(child, file)
    return child
