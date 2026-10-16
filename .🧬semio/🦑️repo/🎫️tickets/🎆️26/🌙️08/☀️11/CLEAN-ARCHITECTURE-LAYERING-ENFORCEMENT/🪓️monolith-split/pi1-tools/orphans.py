#!/usr/bin/env python3
"""🧬️ Orphan-rule scan over the monolith graph with the NEW owners: an inherent impl must sit in the crate that defines its type; a trait impl must sit in the crate of the trait or of the type (std and external types count as foreign).
usage: orphans.py [crate...]   (uses the live tree; run it on the planned layout through flipcore.Plan)"""
import re, sys, collections
from lib import *
from rs import blank
import rsparse as P
import modnode as M
import world as Wd
import layout as L

GEN_STRIP = re.compile(r"<[^<>]*>")


def strip_generics(s):
    prev = None
    while prev != s:
        prev = s
        s = GEN_STRIP.sub("", s)
    return s


def split_impl_header(clean_seg):
    """(trait path text | None, type path text) from the text between `impl` and the body"""
    m = re.match(r"\s*impl\s*", clean_seg)
    s = clean_seg[m.end():] if m else clean_seg
    if s.startswith("<"):
        depth = 0
        for i, c in enumerate(s):
            if c == "<":
                depth += 1
            elif c == ">":
                depth -= 1
                if depth == 0:
                    s = s[i + 1:]
                    break
    s = re.split(r"\bwhere\b|\{", s)[0]
    s = s.strip().lstrip("!").strip()
    m2 = re.search(r"\bfor\b", strip_generics(s))
    if m2:
        # find `for` in original s at the same top level
        depth = 0
        for i in range(len(s)):
            c = s[i]
            if c == "<":
                depth += 1
            elif c == ">":
                depth -= 1
            elif depth == 0 and s.startswith("for", i) and (i == 0 or not s[i - 1].isalnum()) and (i + 3 >= len(s) or not (s[i + 3].isalnum() or s[i + 3] == "_")):
                return s[:i].strip(), s[i + 3:].strip()
    return None, s


def path_segments(text):
    t = strip_generics(text)
    t = re.sub(r"^[&*\s]+", "", t)
    t = re.sub(r"^(?:mut|const|dyn|impl)\s+", "", t)
    t = t.strip("() \t\n")
    t = t.split(" ")[0] if t else t
    segs = [s for s in t.split("::") if s]
    return segs


def owner_of_res(W, LY, res):
    if res is None:
        return None
    if res.kind == "item":
        root = res.node.root()
        if root.crate not in (None, "FACADE"):
            return root.crate
        return LY.anchor(res.item.part)[0]
    if res.kind == "node":
        root = res.node.root()
        if root.crate not in (None, "FACADE"):
            return root.crate
        return LY.anchor(res.node.parts[0])[0] if res.node.parts else None
    if res.kind == "crate":
        return res.key
    if res.kind == "ext":
        return "EXT"
    return None


def scan(W, LY, crates=None):
    findings = []
    for node in M.walk(W.mono):
        for part in node.parts:
            K = LY.part_owner(part)
            if crates and K not in crates:
                continue
            stack = [(node, part, part.lo, part.hi)]
            _scan_part(W, LY, K, node, part, part.lo, part.hi, findings)
    return findings


def _scan_part(W, LY, K, node, part, lo, hi, findings):
    text, clean = part.text, part.clean
    for st in P.statements(text, clean, lo, hi):
        if st.kind == "mod" and st.inline:
            ch = node.mods.get(st.name)
            if ch is not None:
                _scan_part(W, LY, K, ch, ch.parts[0], st.body[0], st.body[1], findings)
        if st.kind != "impl":
            continue
        head_end = clean.find("{", st.start, st.end)
        header = clean[st.start:head_end if head_end != -1 else st.end]
        trait_txt, type_txt = split_impl_header(header)
        tsegs = path_segments(type_txt)
        if not tsegs or tsegs[0] in Wd.PRIM and len(tsegs) == 1 and tsegs[0] != "Self":
            type_owner = "EXT"
        else:
            r = W.resolve_path(node, tsegs)
            type_owner = owner_of_res(W, LY, r) or "UNRESOLVED"
        if trait_txt is None:
            if type_owner not in (K, "UNRESOLVED") :
                findings.append((K, rel(part.file), "inherent-foreign", "::".join(tsegs), type_owner))
            continue
        rsegs = path_segments(trait_txt)
        rr = W.resolve_path(node, rsegs)
        trait_owner = owner_of_res(W, LY, rr) or "UNRESOLVED"
        if K not in (type_owner, trait_owner) and "UNRESOLVED" not in (type_owner, trait_owner):
            findings.append((K, rel(part.file), "orphan-trait-impl", "::".join(rsegs) + " for " + "::".join(tsegs), (trait_owner, type_owner)))


if __name__ == "__main__":
    W = Wd.World()
    LY = L.Layout(W)
    f = scan(W, LY, set(sys.argv[1:]) or None)
    for x in f:
        print(x)
    print(len(f), "findings")
