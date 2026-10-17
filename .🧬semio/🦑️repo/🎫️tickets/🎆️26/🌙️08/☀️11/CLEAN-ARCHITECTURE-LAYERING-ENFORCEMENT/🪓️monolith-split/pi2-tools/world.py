#!/usr/bin/env python3
"""🌍️ Name resolution over the plugin partition: the monolith tree (facade root with its mount hubs, textual includes merged) and the module trees of the crates that already own their sources. Resolves `use` targets and path prefixes to definition sites so the flip can write explicit, correct imports."""
import json, os, re, sys
from lib import *
import modnode as M
import owners as O

STD = {"std", "core", "alloc"}
PFX = 'semio_framework_plugin_'
PRIM = set("bool char str u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 Self self crate super".split())


def visible_to(importer, bnode, obj):
    """a private item or import of module bnode is nameable by bnode and its descendants only (glob imports skip the rest)"""
    if importer is None or getattr(obj, "vis", None) is None or (obj.vis or "").strip():
        return True
    n = importer
    while n is not None:
        if n is bnode:
            return True
        n = n.parent
    return False


class Res:
    """resolution result: kind in node|item|crate|ext"""
    __slots__ = ("kind", "node", "item", "key", "segs")

    def __init__(self, kind, node=None, item=None, key=None, segs=None):
        self.kind, self.node, self.item, self.key, self.segs = kind, node, item, key, segs or []

    def __repr__(self):
        return f"Res({self.kind}, node={'/'.join(self.node.path()) if self.node else None}, item={self.item.name if self.item else None}, key={self.key}, segs={self.segs})"


class World:
    def __init__(self):
        self.mono = M.load_crate(PLUG + "/" + PKGDIR + "/🦀️.rs", key="FACADE")
        self.real = {}
        self.cache = {}
        self.trunc = 0
        self.fcache = {}
        self.active = set()
        self.owner_cache = {}
        O.load_real_owners()
        self.crate_roots = {}
        for k, v in O.CR.items():
            self.crate_roots[k] = PLUG + "/" + v["root"].lstrip("†").strip() + "/🦀️.rs"

    def reload(self):
        self.__init__()

    # ownership -------------------------------------------------------------------------------------------------
    def owner(self, file):
        if file not in self.owner_cache:
            self.owner_cache[file] = O.owner_of(rel(file)) if file.startswith(PLUG + "/") else "EXT"
        return self.owner_cache[file]

    def real_tree(self, key):
        """module tree of an already flipped crate (loaded lazily from its root file)"""
        if key not in self.real:
            root = self.crate_roots.get(key)
            self.real[key] = M.load_crate(root, key=key) if root and os.path.exists(root) else None
        return self.real[key]

    # lookups ---------------------------------------------------------------------------------------------------
    def lookup(self, node, name, seen=None):
        """bindings of `name` in the module scope of node: [("item"|"use", node, obj)]; explicit beats glob"""
        key = (id(node), name)
        if key in self.cache:
            return self.cache[key]
        seen = set() if seen is None else seen
        if key in seen:
            self.trunc += 1
            return []
        seen.add(key)
        t0 = self.trunc
        out = [("item", node, it) for it in node.items if it.name == name]
        out += [("use", node, u) for u in node.uses if not u.glob and u.local == name]
        if not out:
            for u in node.uses:
                if u.glob:
                    tgt = self.resolve_path(node, u.path, seen=seen)
                    out += [b for b in self._names_of(tgt, name, seen, via=u, node=node) if visible_to(node, b[1], b[2])]
        if self.trunc == t0 or out:
            self.cache[key] = out
        return out

    def bound_elsewhere(self, node, name, skip_path):
        """True when `name` is bound in node by an item, an explicit import or a glob other than the one with path skip_path"""
        if any(it.name == name for it in node.items):
            return True
        if any((not u.glob) and u.local == name for u in node.uses):
            return True
        for u in node.uses:
            if u.glob and list(u.path) != list(skip_path):
                tgt = self.resolve_path(node, u.path)
                if tgt is not None and self._names_of(tgt, name, set()):
                    return True
        return False

    def public_lookup(self, node, name, seen=None):
        """binding of `name` that is visible to other crates from node (exactly `pub` items and `pub use` chains)"""
        seen = set() if seen is None else seen
        if (id(node), name) in seen:
            return None
        seen.add((id(node), name))
        for it in node.items:
            if it.name == name:
                return ("item", node, it) if (it.vis or "").strip() == "pub" else None
        for u in node.uses:
            if not u.glob and u.local == name:
                return ("use", node, u) if (u.vis or "").strip() == "pub" else None
        for u in node.uses:
            if u.glob and (u.vis or "").strip() == "pub":
                tgt = self.resolve_path(node, u.path)
                if tgt is None:
                    continue
                n2 = None
                if tgt.kind == "node":
                    n2 = tgt.node
                elif tgt.kind == "crate":
                    tree = self.real_tree(tgt.key)
                    n2 = tree
                    for s in tgt.segs:
                        n2 = n2.mods.get(s) if n2 is not None else None
                if n2 is not None:
                    r = self.public_lookup(n2, name, seen)
                    if r is not None:
                        return r
        return None

    def _names_of(self, tgt, name, seen, via=None, node=None):
        if tgt is None:
            return []
        if tgt.kind == "node":
            return [(b[0], b[1], b[2]) for b in self.lookup(tgt.node, name, seen)]
        if tgt.kind == "crate":
            tree = self.real_tree(tgt.key)
            if tree is None:
                return []
            cur = tree
            for s in tgt.segs:
                nxt = cur.mods.get(s)
                if nxt is None:
                    return []
                cur = nxt
            return self.lookup(cur, name, seen)
        return []

    def resolve_path(self, node, segs, seen=None, in_use=True, prefer=None):
        """resolve a module-path prefix: first segment relative to node (crate/self/super/local name/extern crate); remaining segments walk nodes. Stops at the first non-module and returns the item with remaining segments in .segs"""
        if not segs:
            return Res("node", node=node)
        seen = set() if seen is None else seen
        head, rest = segs[0], segs[1:]
        cur = None
        if head == "crate":
            cur = Res("node", node=node.root())
        elif head == "self":
            cur = Res("node", node=node)
        elif head == "super":
            cur = Res("node", node=node.parent or node)
            while rest and rest[0] == "super":
                cur = Res("node", node=cur.node.parent or cur.node)
                rest = rest[1:]
        else:
            b = self.lookup(node, head, seen)
            if b:
                cur = self.pick(b, seen, prefer) if not rest else self.follow_any(b, seen)
            else:
                if head.startswith(PFX):
                    cur = Res("crate", key=head[len(PFX):].replace("_", "-"), segs=[])
                else:
                    return Res("ext", segs=segs)
        if cur is None:
            return None
        return self._walk(cur, rest, seen, prefer)

    def pick(self, binds, seen, prefer):
        return self.follow_any(binds, seen, prefer)

    def follow_any(self, binds, seen=None, prefer=None):
        """first binding that resolves without running into a cycle (glob chains can loop back through a re-export); with prefer="macro" a macro binding wins"""
        last = None
        first = None
        for b in binds:
            r = self._follow(b, set(), prefer)
            if r is None or r.kind == "cycle":
                last = r if last is None else last
                continue
            if prefer == "macro":
                if r.kind == "item" and r.item.kind == "macro":
                    return r
                first = r if first is None else first
                continue
            return r
        return first if first is not None else last

    def _follow(self, bind, seen, prefer=None):
        kind, node, obj = bind
        if kind == "item":
            if obj.kind == "mod":
                ch = node.mods.get(obj.name)
                return Res("node", node=ch) if ch is not None else Res("item", node=node, item=obj)
            if obj.kind == "extern crate":
                text = obj.part.text[obj.stmt.start:obj.stmt.end]
                m = re.search(r"extern\s+crate\s+([A-Za-z_][A-Za-z0-9_]*)", text)
                real = m.group(1) if m else obj.name
                if real.startswith(PFX):
                    return Res("crate", key=real[len(PFX):].replace("_", "-"), segs=[])
                return Res("ext", segs=[real])
            return Res("item", node=node, item=obj)
        u = obj
        k = (id(u), prefer)
        if k in self.fcache:
            return self.fcache[k]
        if k in self.active:
            return Res("cycle")
        self.active.add(k)
        try:
            tgt = self.resolve_path(node, u.path, set(), prefer=prefer)
        finally:
            self.active.discard(k)
        if tgt is None or tgt.kind != "cycle":
            self.fcache[k] = tgt
        return tgt

    def _walk(self, cur, rest, seen, prefer=None):
        while rest:
            if cur is None:
                return None
            if cur.kind == "node":
                b = self.lookup(cur.node, rest[0], seen)
                if not b:
                    if cur.node.name in ("wasip2", "return_walk"):
                        return Res("node", node=cur.node, segs=rest)
                    return Res("unresolved", node=cur.node, segs=rest)
                cur = self.pick(b, seen, prefer) if len(rest) == 1 else self.follow_any(b, seen)
                rest = rest[1:]
            elif cur.kind == "crate":
                tree = self.real_tree(cur.key)
                if tree is None:
                    return Res("crate", key=cur.key, segs=cur.segs + rest)
                n = tree
                for s in cur.segs:
                    n = n.mods.get(s)
                    if n is None:
                        return Res("unresolved", segs=rest)
                cur = Res("node", node=n)
            elif cur.kind == "ext":
                return Res("ext", segs=cur.segs + rest)
            else:
                # item: remaining segments are associated items / variants
                return Res(cur.kind, node=cur.node, item=cur.item, key=cur.key, segs=list(cur.segs) + rest)
        return cur

    def def_of(self, node, name):
        """final definition (node, item) of a name visible in node, following `use` chains; or the external path"""
        b = self.lookup(node, name)
        if not b:
            return None
        return self._follow(b[0], set())

if __name__ == "__main__":
    W = World()
    root = W.mono
    print("mono files", len(M.files_of(root)))
    print(W.resolve_path(root, ["crate", "child_content", "ChildContentView"]))
    print(W.resolve_path(root, ["app_views", "PeerPresence"]))
