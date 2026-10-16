#!/usr/bin/env python3
"""🧭️ Path translation: old monolith paths (`crate::OLD::x`, `super::x`, glob scopes) to the explicit paths of the new crate layout, and the edit plan that rewrites a part of a file accordingly."""
import re, os
from lib import *
from rs import blank
import rsparse as P
import modnode as M
import world as Wd
import layout as L

PFX = Wd.PFX
STD_TRAITS = set("Display Debug Default Clone Copy Send Sync Drop Iterator IntoIterator From Into TryFrom TryInto AsRef AsMut Borrow BorrowMut ToString ToOwned Deref DerefMut Hash Hasher PartialEq Eq PartialOrd Ord Add Sub Mul Div Neg Not Index IndexMut FromStr Error Read Write Future Fn FnMut FnOnce Extend FromIterator Any".split())
TRAITS = set(read(TOOLS + "/trait_names.txt").split()) | STD_TRAITS
BODY_PATH = re.compile(r"(?<![A-Za-z0-9_$])(?<!::)((?:crate|super|self)(?:::super)*)((?:::[A-Za-z_][A-Za-z0-9_]*)+)")
IDENT = re.compile(r"(?<![A-Za-z0-9_$'])([A-Za-z_][A-Za-z0-9_]*)")


STATIC_OWNER = {}


class Tr:
    """translator for text that will live in crate K"""

    def __init__(self, W, LY, K, strict=True, flipped=(), allowed=None):
        self.W, self.LY, self.K = W, LY, K
        self.flipped = set(flipped)
        self.allowed = set(allowed) if allowed is not None else None
        self.notes = []

    def facade_path(self, node, name=None):
        p = [s for s in node.path()]
        if p and p[0] == "component":
            p = p[1:]
        return ["crate"] + p + ([name] if name else [])

    # emission ----------------------------------------------------------------------------------------------
    def shared_fixture(self, res):
        part = res.item.part if res.kind == "item" else (res.node.parts[0] if res.kind == "node" and res.node.parts else None)
        return part is not None and rel(part.file).startswith(("🧪️testing/🎟️caller/",))

    def item_anchor(self, item):
        if item.kind == "static" and item.name in STATIC_OWNER:
            return STATIC_OWNER[item.name], []
        if rel(item.part.file).startswith(("🧪️testing/🎟️caller/",)):
            return "FACADE", []
        return self.LY.anchor(item.part)

    def site_path(self, K2, mp, name=None):
        base = "crate" if K2 == self.K else PFX + K2.replace("-", "_")
        segs = [base] + list(mp) + ([name] if name else [])
        return segs

    def shorten_node(self, K2, mp, node):
        """prefer the crate-root name when the (already flipped) crate re-exports the module at its root (a private wrapper module such as `persistent_local` must not appear in a path)"""
        if K2 not in self.flipped or len(mp) < 2:
            return mp
        tree = self.W.real.get(K2) if K2 in self.W.real else (self.W.real_tree(K2) if K2 in Wd.O.CR else None)
        if tree is None:
            return mp
        b = self.W.public_lookup(tree, mp[-1])
        if b:
            r = self.W._follow(b, set())
            if r is not None and r.kind == "node" and r.node is node:
                return [mp[-1]]
        return mp

    def shorten(self, K2, mp, item):
        """prefer the crate-root name when the (already flipped) crate re-exports the item at its root"""
        if K2 not in self.flipped:
            return mp
        tree = self.W.real.get(K2) if K2 in self.W.real else (self.W.real_tree(K2) if K2 in Wd.O.CR else None)
        if tree is not None and mp:
            b = self.W.public_lookup(tree, item.name)
            if b:
                r = self.W._follow(b, set())
                if r is not None and r.kind == "item" and r.item is item:
                    return []
        return mp

    def emit(self, res, keep=None):
        """list of path segments in the new world for a resolution; None when the target cannot be named from K"""
        if res is None:
            return None
        if res.kind == "ext":
            return list(res.segs)
        if res.kind == "crate":
            return [PFX + res.key.replace("-", "_")] + list(res.segs)
        if res.kind in ("item", "node") and self.allowed is not None:
            site = self.item_anchor(res.item) if res.kind == "item" else self.LY.node_site(res.node)
            if site[0] not in ("FACADE", "TESTS", None, "EXT", "CYCLE") and site[0] != self.K and site[0] not in self.allowed and self.K != "FACADE":
                self.notes.append(("unflipped-target", (res.item.name if res.kind == "item" else "/".join(res.node.path())), site[0]))
                return None
        if res.kind == "item":
            K2, mp = self.item_anchor(res.item)
            if self.K == "FACADE" and (K2 in ("FACADE", "TESTS") or (K2 not in self.flipped and res.node.root().crate in (None, "FACADE"))):
                return self.facade_path(res.node, res.item.name) + list(res.segs)
            if K2 in ("FACADE", "TESTS", None, "EXT", "CYCLE"):
                self.notes.append(("facade-target", res.item.name, K2))
                return None
            if res.item.kind == "macro" and "macro_export" in (res.item.attrs or ""):
                mp = []
            else:
                mp = self.shorten(K2, mp, res.item)
            return self.site_path(K2, mp, res.item.name) + list(res.segs)
        if res.kind == "node":
            K2, mp = self.LY.node_site(res.node)
            if self.K == "FACADE" and (K2 in ("FACADE", "TESTS") or (K2 not in self.flipped and res.node.root().crate in (None, "FACADE"))):
                return self.facade_path(res.node) + list(res.segs)
            if K2 in ("FACADE", "TESTS", None, "EXT", "CYCLE"):
                self.notes.append(("facade-target-node", "/".join(res.node.path()), K2))
                return None
            return self.site_path(K2, self.shorten_node(K2, mp, res.node)) + list(res.segs)
        return None

    def translate_path(self, node, segs):
        res = self.W.resolve_path(node, segs)
        if res is None or res.kind in ("unresolved", "cycle"):
            return None
        return self.emit(res)

    def binding_path(self, node, name):
        """new path of the definition a name is bound to in node's scope"""
        b = self.W.lookup(node, name)
        if not b:
            return None
        r = self.W.follow_any(b)
        return self.emit(r)

    # statements --------------------------------------------------------------------------------------------
    def body_edits(self, node, text, clean, lo, hi, skip, aliases=()):
        """edits (start, end, replacement) for `crate::`/`super::`/`self::` paths and for paths that start with an imported module alias, in text[lo:hi] outside the spans in skip"""
        edits = []
        matches = list(BODY_PATH.finditer(clean, lo, hi))
        if aliases:
            arx = re.compile(r"(?<![A-Za-z0-9_$])(?<!::)(" + "|".join(re.escape(a) for a in sorted(aliases)) + r")((?:::[A-Za-z_][A-Za-z0-9_]*)+)")
            matches += list(arx.finditer(clean, lo, hi))
        for m in matches:
            if any(a <= m.start() < b for a, b in skip):
                continue
            head = m.group(1)
            segs = [s for s in head.split("::")] + m.group(2).split("::")[1:]
            res = self.W.resolve_path(node, segs, prefer="macro" if re.match(r"\s*!(?!=)", clean[m.end():m.end() + 4]) else None)
            if res is None or res.kind in ("unresolved", "cycle"):
                self.notes.append(("unresolved-body-path", "::".join(segs), clean.count("\n", 0, m.start()) + 1))
                continue
            new = self.emit(res)
            if new is None:
                continue
            edits.append((m.start(), m.end(), "::".join(new)))
        return edits

    def use_lines(self, node, stmt, text, clean, needed_ids):
        """regenerated `use` text for one statement; returns (lines, notes)"""
        entries = P.use_entries(stmt, clean)
        attrs = stmt.attrs
        vis = stmt.vis
        out = []
        for path, alias, glob in entries:
            if glob:
                tgt = self.W.resolve_path(node, path)
                if tgt is not None and tgt.kind == "node" and not (tgt.node.root().crate not in (None, "FACADE")):
                    names = self.glob_names(tgt.node, needed_ids)
                    for n, p in names:
                        out.append((p, None if p[-1] == n else n, False))
                    ext = self.ext_globs(tgt.node)
                    for e in ext:
                        out.append((e, None, True))
                    continue
                if tgt is not None and tgt.kind in ("node", "crate"):
                    new = self.emit(tgt)
                    if new is None:
                        self.notes.append(("glob-target-unnameable", "::".join(path), 0))
                        continue
                    out.append((new, None, True))
                    continue
                out.append((path, None, True))
                continue
            tgt = self.W.resolve_path(node, path)
            if tgt is None or tgt.kind in ("unresolved", "cycle"):
                self.notes.append(("unresolved-use", "::".join(path), 0))
                out.append((path, alias, False))
                continue
            if path[0] not in ("crate", "self", "super") and self.shared_fixture(tgt):
                out.append((path, alias, False))
                continue
            new = self.emit(tgt)
            if new is None:
                self.notes.append(("use-dropped", "::".join(path), 0))
                continue
            local = alias or (path[-1] if path[-1] != "self" else path[-2])
            out.append((new, local if new[-1] != local else None, False))
        return out

    def glob_names(self, mnode, needed_ids):
        """(local name, new path) for every name of the module scope `mnode` the file uses (or that is a trait)"""
        res = []
        seen = set()
        for name in sorted(needed_ids | self.trait_names_in(mnode)):
            if name in seen or name in Wd.PRIM:
                continue
            b = self.W.lookup(mnode, name)
            if not b:
                continue
            r = self.W.follow_any(b)
            p = self.emit(r)
            if p is None:
                continue
            seen.add(name)
            kind, bnode, obj = b[0]
            attrs = obj.attrs if hasattr(obj, "attrs") else ""
            res.append((name, p, re.findall(r"#\[\s*cfg\((?:[^\[\]]|\[[^\]]*\])*\)\s*\]", attrs or "")))
        return res

    def trait_names_in(self, mnode, seen=None):
        seen = set() if seen is None else seen
        if id(mnode) in seen:
            return set()
        seen.add(id(mnode))
        names = set()
        for u in mnode.uses:
            if not u.glob and u.local in TRAITS:
                names.add(u.local)
        for it in mnode.items:
            if it.kind == "trait":
                names.add(it.name)
        for u in mnode.uses:
            if u.glob:
                tgt = self.W.resolve_path(mnode, u.path)
                if tgt is not None and tgt.kind == "node":
                    names |= self.trait_names_in(tgt.node, seen)
        return names

    def ext_globs(self, mnode, seen=None):
        """external glob imports (`use semio_framework_x::*;`) of a scope chain, kept verbatim"""
        seen = set() if seen is None else seen
        if id(mnode) in seen:
            return []
        seen.add(id(mnode))
        out = []
        for u in mnode.uses:
            if u.glob:
                tgt = self.W.resolve_path(mnode, u.path)
                if tgt is None:
                    continue
                if tgt.kind == "ext":
                    out.append(list(u.path))
                elif tgt.kind == "node":
                    out += self.ext_globs(tgt.node, seen)
        return out


def group_uses(entries, vis="", attrs=""):
    """compact `use` text from [(path list, alias, glob[, extra attrs list])]; statement attrs apply to every entry"""
    pre_v = (vis + " " if vis else "")
    byprefix = {}
    order = []
    for e in entries:
        path, alias, glob = e[0], e[1], e[2]
        extra = tuple(e[3]) if len(e) > 3 and e[3] else ()
        if glob:
            key = (extra, tuple(path))
            leaf = "*"
        else:
            key = (extra, tuple(path[:-1]))
            leaf = path[-1] + (f" as {alias}" if alias and alias != path[-1] else "")
        if key not in byprefix:
            byprefix[key] = []
            order.append(key)
        if leaf not in byprefix[key]:
            byprefix[key].append(leaf)
    lines = []
    for key in order:
        extra, prefix = key
        leaves = byprefix[key]
        attr_lines = ([attrs] if attrs else []) + [a for a in extra if a not in (attrs or "")]
        pre = "".join(a + "\n" for a in attr_lines) + pre_v
        if prefix:
            if len(leaves) == 1:
                lines.append(f"{pre}use {'::'.join(prefix)}::{leaves[0]};")
            else:
                lines.append(f"{pre}use {'::'.join(prefix)}::{{{', '.join(leaves)}}};")
        else:
            for lf in leaves:
                lines.append(f"{pre}use {lf};")
    return lines
