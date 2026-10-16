#!/usr/bin/env python3
"""🔗️ Crate-to-crate reference edges of the monolith by real name resolution: for each part (file text) of the monolith tree, every identifier that resolves in its module scope to a definition owned by another crate is an edge owner(part) -> owner(definition)."""
import collections, re, sys, json
from lib import *
from rs import blank
import modnode as M
import world as Wd

IDENT = re.compile(r"(?<![A-Za-z0-9_$'])([A-Za-z_][A-Za-z0-9_]*)")


def declared_names(node, part):
    names = set()
    for it in node.items:
        if it.part is part:
            names.add(it.name)
    for u in node.uses:
        if u.part is part and u.local:
            names.add(u.local)
    return names


def def_owner(W, res):
    if res is None:
        return None
    if res.kind == "item":
        n = res.node
        root = n.root()
        if root.crate and root.crate != "FACADE":
            return root.crate
        return W.owner(res.item.part.file)
    if res.kind == "node":
        n = res.node
        root = n.root()
        if root.crate and root.crate != "FACADE":
            return root.crate
        return W.owner(n.file)
    if res.kind == "crate":
        return res.key
    return None


def part_ids(part):
    """(production identifiers, test identifiers) of a part: `use` statements excluded; statements gated by cfg(test) (or `any(test, ..)` feature gates) count as test"""
    import rsparse as P
    clean = part.clean
    prod, test = set(), set()
    for st in P.statements(part.text, clean, part.lo, part.hi):
        if st.kind == "use":
            continue
        seg = clean[st.start:st.end]
        ids = set(IDENT.findall(seg))
        if re.search(r"cfg\((?:any\()?\s*test\b", st.attrs) or re.search(r"cfg\(any\(test", st.attrs):
            test |= ids
        else:
            prod |= ids
    return prod, test


def compute(W, only=None):
    edges = collections.defaultdict(lambda: collections.defaultdict(set))
    for node in M.walk(W.mono):
        for part in node.parts:
            kp = W.owner(part.file)
            if kp in ("FACADE", "TESTS", "EXT"):
                continue
            if only and kp not in only:
                continue
            prod, test = part_ids(part)
            for tag, names in (("", prod), ("T", test - prod)):
              for name in names:
                if name in Wd.PRIM:
                    continue
                b = W.lookup(node, name)
                if not b:
                    continue
                res = W.follow_any(b)
                ko = def_owner(W, res)
                if ko and ko != kp:
                    edges[kp][ko].add((name, rel(part.file), "/".join(node.path()), tag))
    return edges


if __name__ == "__main__":
    W = Wd.World()
    only = set(sys.argv[1:]) or None
    ed = compute(W, only)
    rank = {k: v["rank"] for k, v in Wd.O.CR.items()}
    for kp in sorted(ed, key=lambda k: rank.get(k, 999)):
        for ko in sorted(ed[kp], key=lambda k: rank.get(k, 999)):
            rows = ed[kp][ko]
            prod = sorted(r for r in rows if r[3] == "")
            test = sorted(r for r in rows if r[3] == "T")
            up = rank.get(ko, 999) > rank.get(kp, 999) and ko in rank
            for label, rr in (("prod", prod), ("test", test)):
                if not rr:
                    continue
                flag = "UP " if up else "   "
                print(f"{flag}{kp:22s}->{ko:22s} {label} {len(rr):3d}  " + ", ".join(f"{n}@{f.split('/')[-2]}" for n, f, p, t in rr[:5]))
