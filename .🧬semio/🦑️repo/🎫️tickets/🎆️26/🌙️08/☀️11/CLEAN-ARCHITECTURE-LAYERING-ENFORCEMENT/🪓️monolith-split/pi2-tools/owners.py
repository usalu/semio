#!/usr/bin/env python3
"""🗺️ File -> owning crate by longest crate-root directory prefix (Appendix H roots), plus the cross-owner mount edges of the live tree."""
import json, os, sys, collections
from lib import *
import tree as T

CR = json.load(open(TOOLS + "/crates.json"))
ROOTS = {}
for k, v in CR.items():
    if v.get("kind") in ("lower", "instance"):
        ROOTS[v["root"].lstrip("†").strip() + "/"] = k
EXTRA = {"📣️macros/": "FACADE", "🪝️mounts/": "FACADE", "🧪️tests/": "TESTS", "🧪️testing/": "testing-laws"}


MAP = json.load(open(os.environ.get("PI2_MAP", TOOLS + "/crate_map.json")))
REAL_OWNER = {}
REAL_SET = set()


def load_real_owners():
    """files reachable from the root of an already flipped crate belong to that crate, whatever the directory map says"""
    import uo
    REAL_OWNER.clear()
    REAL_SET.clear()
    for k, root in uo.real_crates().items():
        REAL_SET.add(k)
        for f, parent, n in T.tree(root):
            if not rel(f).startswith(("🧪️testing/🎟️caller/",)):
                REAL_OWNER[rel(f)] = k


def owner_of(relpath):
    if relpath in REAL_OWNER:
        return REAL_OWNER[relpath]
    best = None
    for pre, k in MAP.items():
        if relpath.startswith(pre) and (best is None or len(pre) > len(best[0])):
            best = (pre, k)
    if best:
        if best[1].startswith("TOP-") or best[1] in REAL_SET:
            return "FACADE"
        return best[1]
    best = None
    for pre, k in ROOTS.items():
        if relpath.startswith(pre) and (best is None or len(pre) > len(best[0])):
            best = (pre, k)
    if best:
        return best[1]
    for pre, k in EXTRA.items():
        if relpath.startswith(pre):
            return k
    return "FACADE"


def hub_files():
    out = [PLUG + "/🦀️.rs"]
    d = PLUG + "/🪝️mounts"
    out += [d + "/" + h + "/🦀️.rs" for h in sorted(os.listdir(d))]
    return out

if __name__ == "__main__":
    # every file reachable from the root and hubs, with owner; list edges where parent owner != child owner
    seen = {}
    edges = []
    def walk(p, parent):
        if p in seen:
            return
        seen[p] = owner_of(rel(p))
        for c, n, a, v in T.children(p):
            if os.path.exists(c):
                edges.append((p, c, n, a))
                walk(c, p)
    for h in hub_files():
        walk(h, None)
    only = sys.argv[1:]
    for p, c, n, a in edges:
        op, oc = seen[p], seen[c]
        if op != oc and (not only or op in only or oc in only):
            print(f"{op:24s} -> {oc:24s} {n:22s} {rel(p)}  =>  {rel(c)}  {a}")
