#!/usr/bin/env python3
"""🧪️ Test relocation: cfg(test)-gated units (included/mounted test files, inline test modules) of a flipped crate that name higher layers or facade fixtures cannot stay in the crate. They move verbatim into a facade test host module whose header imports reproduce the old module scope with explicit paths."""
import os, re, collections
from lib import *
from rs import blank
import rsparse as P
import modnode as M
import translate as TR
import world as Wd

IDENT = TR.IDENT


def is_test_gated(attrs):
    return bool(re.search(r"cfg\(\s*(?:any\()?\s*test\b", attrs or "")) or bool(re.search(r'cfg\([^\]]*feature\s*=\s*"[a-z-]*testing"', attrs or ""))


def is_pure_feature_gate(attrs):
    """gated by a testing feature without `test` (production-visible helper code that is compiled in feature builds)"""
    return bool(re.search(r'feature\s*=', attrs or "")) and not re.search(r"\btest\b", attrs or "")


def is_pure_test(attrs):
    """gated by `test` alone (feature-gated testing code builds in normal profiles and needs normal dependencies)"""
    return bool(re.search(r"cfg\(\s*(?:all\()?\s*test\b", attrs or "")) and not re.search(r"feature\s*=", attrs or "") or bool(re.search(r"cfg\(\s*test\s*\)", attrs or ""))


def rebase_paths(text, old_dir, new_dir, path_dir=None):
    """relative `include!("..")` / `#[path = ".."]` literals of text written for old_dir, rewritten for new_dir (`#[path]` inside the inline wrapper module of a host unit is relative to path_dir = host dir + wrapper name)"""
    clean = blank(text)
    out, last = [], 0
    rx = re.compile(r'((?:include_str|include_bytes|include)!\(\s*")([^"]+)(")|(#\[\s*path\s*=\s*")([^"]+)(")')
    for m in rx.finditer(text):
        if clean[m.start():m.start() + 4] == "    " and False:
            continue
        # blank() erased string contents; make sure this is code
        if not (clean[m.start():m.start() + 7] == "include" or clean[m.start():m.start() + 2] == "#["):
            continue
        grp = (m.group(1), m.group(2), m.group(3)) if m.group(1) else (m.group(4), m.group(5), m.group(6))
        absolute = os.path.normpath(os.path.join(old_dir, grp[1]))
        new_rel = os.path.relpath(absolute, path_dir if (path_dir and grp[0].startswith('#')) else new_dir)
        out.append(text[last:m.start()])
        out.append(grp[0] + new_rel + grp[2])
        last = m.end()
    out.append(text[last:])
    return "".join(out)


class Unit:
    def __init__(self, K, node, part, st, kind, files, ids, reasons):
        self.K, self.node, self.part, self.st, self.kind, self.files, self.ids, self.reasons = K, node, part, st, kind, files, ids, reasons


def closure_files(plan, seed_files):
    S = set(seed_files)
    parts = [p for n in M.walk(plan.W.mono) for p in n.parts]
    changed = True
    while changed:
        changed = False
        for p in parts:
            if p.file in S:
                continue
            hp = p.host_part
            mp = getattr(p.node, "mount_part", None)
            if (hp is not None and hp.file in S) or (hp is None and mp is not None and mp.file in S and p.node.parts and p.node.parts[0] is p):
                S.add(p.file)
                changed = True
    return S


SHARED_FIXTURES = ("🧪️testing/🎟️caller/",)


def fixture_node(res):
    """a resolution into a shared test fixture file (mounted by any crate that needs it, never relocated)"""
    part = res.item.part if res.kind == "item" else (res.node.parts[0] if res.kind == "node" and res.node.parts else None)
    if part is not None:
        return rel(part.file).startswith(SHARED_FIXTURES)
    return res.kind == "node" and getattr(res.node, "file", "") and rel(res.node.file).startswith(SHARED_FIXTURES)


def use_reasons(plan, K, node, part, st):
    """a `use` that names a module, item or higher crate through the old scope (`use super::turn::X`, `use alias::Y`) flags the unit like a body path would"""
    out = []
    for path, alias, glob in P.use_entries(st, part.clean):
        if glob or not path or path[0] in ("crate", "self"):
            continue
        if path[0] != "super" and not (len(path) > 1 and plan.W.lookup(node, path[0])):
            continue
        res = plan.W.resolve_path(node, path)
        if res is None or res.kind not in ("item", "node", "crate") or fixture_node(res):
            continue
        if res.kind == "crate":
            ko = res.key
        else:
            rt = res.node.root().crate
            if rt not in (None, "FACADE"):
                ko = rt
            elif res.kind == "item":
                ko = plan.LY.part_owner(res.item.part)
            else:
                ko = plan.LY.part_owner(res.node.parts[0]) if res.node.parts else "FACADE"
        if ko in ("FACADE", "TESTS", "EXT") or plan.outranks(ko, K):
            out.append(("::".join(path[-2:]), ko))
    return out


def local_fixture(binds, res):
    """the name is bound to a shared fixture by a local `mod` or a non-`crate::` `use` (the unit mounts the fixture itself)"""
    if not fixture_node(res):
        return False
    for kind, node, obj in binds:
        if kind == "use" and obj.path and obj.path[0] in ("crate", "self", "super"):
            return False
    return True


def unit_ids(plan, node, part, st):
    """identifiers used by a test unit, with the part/node whose scope resolves them"""
    clean = part.clean
    out = []
    if st.kind == "other":
        attrs, at = P.split_attrs(part.text, clean, st)
        m = M.INCLUDE.match(part.text, at)
        inc = os.path.normpath(os.path.join(os.path.dirname(part.file), m.group(1)))
        ip = plan.file_part.get(inc)
        if ip is not None:
            out.append((node, ip))
    elif st.kind == "mod" and not st.inline:
        ch = node.mods.get(st.name)
        if ch is not None and ch.parts:
            out.append((ch, ch.parts[0]))
    elif st.kind == "mod" and st.inline:
        ch = node.mods.get(st.name)
        if ch is not None:
            out.append((ch, ch.parts[0]))
    return out


def flagged_parts(plan, K, parts):
    """reasons (name, owner) why test parts cannot stay in crate K: they name facade items, fixtures or higher layers"""
    reasons = []
    seen = set()
    for p2 in parts:
        n2 = p2.node
        for st in P.statements(p2.text, p2.clean, p2.lo, p2.hi):
            if st.kind == "use":
                reasons += use_reasons(plan, K, n2, p2, st)
                continue
            for name in set(IDENT.findall(p2.clean[st.start:st.end])):
                if name in Wd.PRIM or (id(n2), name) in seen:
                    continue
                seen.add((id(n2), name))
                b = plan.W.lookup(n2, name)
                if not b:
                    continue
                r = plan.W.follow_any(b)
                if r is None or local_fixture(b, r):
                    continue
                if r.kind == "item":
                    ko = r.node.root().crate if r.node.root().crate not in (None, "FACADE") else plan.LY.part_owner(r.item.part)
                elif r.kind == "node":
                    ko = r.node.root().crate if r.node.root().crate not in (None, "FACADE") else (plan.LY.part_owner(r.node.parts[0]) if r.node.parts else "FACADE")
                elif r.kind == "crate":
                    ko = r.key
                else:
                    continue
                if ko in ("FACADE", "TESTS", "EXT") or plan.outranks(ko, K):
                    reasons.append((name, ko))
    for p2 in parts:
        n2 = p2.node
        for m in TR.BODY_PATH.finditer(p2.clean, p2.lo, p2.hi):
            segs = [x for x in m.group(1).split("::")] + m.group(2).split("::")[1:]
            res = plan.W.resolve_path(n2, segs)
            if res is None or res.kind not in ("item", "node", "crate"):
                continue
            if res.kind == "crate":
                ko = res.key
            else:
                rt = res.node.root().crate
                if rt not in (None, "FACADE"):
                    ko = rt
                elif res.kind == "item":
                    ko = plan.LY.part_owner(res.item.part)
                else:
                    ko = plan.LY.part_owner(res.node.parts[0]) if res.node.parts else "FACADE"
            if ko in ("FACADE", "TESTS", "EXT") or plan.outranks(ko, K):
                reasons.append(("::".join(segs[-2:]), ko))
    return reasons


def flagged_spans(plan, K, node, part, stmts):
    """reasons why a set of top-level test statements of a part cannot stay in crate K (identifiers and `crate::` paths they use)"""
    reasons = []
    seen = set()
    for st in stmts:
        seg = part.clean[st.start:st.end]
        for name in set(IDENT.findall(seg)):
            if name in Wd.PRIM or name in seen:
                continue
            seen.add(name)
            b = plan.W.lookup(node, name)
            if not b:
                continue
            r = plan.W.follow_any(b)
            if r is None or local_fixture(b, r):
                continue
            if r.kind == "item":
                ko = r.node.root().crate if r.node.root().crate not in (None, "FACADE") else plan.LY.part_owner(r.item.part)
            elif r.kind == "node":
                ko = r.node.root().crate if r.node.root().crate not in (None, "FACADE") else (plan.LY.part_owner(r.node.parts[0]) if r.node.parts else "FACADE")
            elif r.kind == "crate":
                ko = r.key
            else:
                continue
            if ko in ("FACADE", "TESTS", "EXT") or plan.outranks(ko, K):
                reasons.append((name, ko))
        for m in TR.BODY_PATH.finditer(part.clean, st.start, st.end):
            segs = [x for x in m.group(1).split("::")] + m.group(2).split("::")[1:]
            res = plan.W.resolve_path(node, segs)
            if res is None or res.kind not in ("item", "node", "crate"):
                continue
            if res.kind == "crate":
                ko = res.key
            else:
                rt = res.node.root().crate
                if rt not in (None, "FACADE"):
                    ko = rt
                elif res.kind == "item":
                    ko = plan.LY.part_owner(res.item.part)
                else:
                    ko = plan.LY.part_owner(res.node.parts[0]) if res.node.parts else "FACADE"
            if ko in ("FACADE", "TESTS", "EXT") or plan.outranks(ko, K):
                reasons.append(("::".join(segs[-2:]), ko))
    return reasons
