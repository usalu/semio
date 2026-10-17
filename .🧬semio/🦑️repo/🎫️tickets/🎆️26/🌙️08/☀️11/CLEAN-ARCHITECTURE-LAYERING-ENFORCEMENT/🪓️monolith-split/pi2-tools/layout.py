#!/usr/bin/env python3
"""📐️ New-world coordinates of monolith code: which crate owns a part (file text) and the module path it gets inside that crate. A file mounted by a parent of another crate becomes a top-level module of its crate (named by its mount), a crate root file becomes the crate root, an include of a foreign file becomes a promoted module (named by the wrapper module or the PROMOTE table), and everything else nests under its parent as before."""
import re, os
from lib import *
import world as Wd


def slug(file):
    d = os.path.basename(os.path.dirname(file))
    s = re.sub(r"[^A-Za-z0-9]+", "_", d).strip("_")
    return s or "promoted"


class Layout:
    def __init__(self, W, promote=None, roots=None):
        self.W = W
        self.promote = promote or {}
        self.roots = roots if roots is not None else {}      # crate key -> root file (class B roots are monolith module files)
        self.root_files = set(self.roots.values())
        self.cache = {}

    def part_owner(self, part):
        k = self.W.owner(part.file)
        if k in ("TESTS", "FACADE", "EXT"):
            if part.host_part is not None:
                return self.part_owner(part.host_part)
            if part.node.parent is not None and getattr(part.node, "mount_part", None) is not None and k == "TESTS":
                return self.part_owner(part.node.mount_part)
        return k

    def content_owner(self, node):
        """owner of the parts that declare items (a wrapper module's own `use` lines do not count)"""
        for p in node.parts:
            if p.host_part is not None or p is node.parts[0]:
                if any(it.part is p for it in node.items) or any(m.parts and m.parts[0].file == p.file and m.parent is node for m in node.mods.values()):
                    return self.part_owner(p)
        return self.part_owner(node.parts[0]) if node.parts else None

    def anchor(self, part):
        """(crate key, module path list) of a part in the new world"""
        k = id(part)
        if k in self.cache:
            return self.cache[k]
        self.cache[k] = ("CYCLE", [])
        r = self._anchor(part)
        self.cache[k] = r
        return r

    def _anchor(self, part):
        node = part.node
        root = node.root()
        if root.crate not in (None, "FACADE"):
            return (root.crate, node.path())
        K = self.part_owner(part)
        if node.parent is None:
            return (K, [])
        if part.host_part is None:
            if node.inline and part is node.parts[0]:
                hk, hp = self.anchor(node.mount_part)
                ck = self.content_owner(node)
                if ck == hk or ck is None:
                    return (hk, hp + [node.name])
                return (ck, [node.name])
            hk, hp = self.anchor(node.mount_part) if getattr(node, "mount_part", None) else (None, [])
            if K == hk:
                return (K, hp + [node.name])
            if node.file in self.root_files and K in self.roots and self.roots[K] == node.file:
                return (K, [])
            return (K, [node.name])
        hk, hp = self.anchor(part.host_part)
        if K == hk:
            return (K, hp)
        host_node = part.host_part.node
        if host_node.inline and part.host_part is host_node.parts[0] and host_node.parent is not None:
            return (K, [host_node.name])
        return (K, [self.promote.get(part.file) or slug(part.file)])

    def item_site(self, node, item):
        return self.anchor(item.part)

    def node_site(self, node):
        """(crate, path) of a module node in the new world"""
        root = node.root()
        if root.crate not in (None, "FACADE"):
            return (root.crate, node.path())
        part = node.parts[0] if node.parts else None
        if part is None:
            return (None, node.path())
        return self.anchor(part)
