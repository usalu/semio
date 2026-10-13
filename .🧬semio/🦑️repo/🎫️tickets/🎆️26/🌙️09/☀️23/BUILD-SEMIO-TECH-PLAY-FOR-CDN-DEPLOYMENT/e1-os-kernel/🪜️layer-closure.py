#!/usr/bin/env python3
"""🪜️ Item-level layering probe for `semio-framework-os-kernel`.

An item is a module-level definition; inherent and trait impls are folded into their self type, so an item's
closure carries every impl that must travel with it (orphan rule). Answers: "if the items facet F needs from
facet G moved below F, what else must move with them?"

Usage:
  layer-closure.py OWNERS_JSON needs FROM_FACET TO_FACET     items of TO the FROM facet references, and their closure in TO
  layer-closure.py OWNERS_JSON closure ITEM [ITEM ...]       forward closure of explicit items (paths without crate prefix)
  layer-closure.py OWNERS_JSON users ITEM                    items that reference ITEM
  layer-closure.py OWNERS_JSON why FROM_FACET TO_FACET TARGET  shortest chain from FROM's needs (inside TO) to TARGET
"""
import importlib.util, json, os, sys
from collections import defaultdict, deque

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("fm", os.path.join(HERE, "🧭️facet-map.py"))
fm = importlib.util.module_from_spec(spec); spec.loader.exec_module(fm)


def item_key(path, modinfo, owners, memo={}):
    if path in memo: return memo[path]
    mod = fm.module_of(path, modinfo)
    rest = path[len(mod):].lstrip(":").split("::") if mod else path.split("::")
    head = rest[0] if rest and rest[0] else ""
    key = f"{mod}::{head}" if mod else head
    if head.startswith("{impl#"):
        o = owners.get(key, {})
        if o.get("self") and not o["self"].startswith("extern::") and not o["self"].startswith(key + "::"):
            key = item_key(o["self"], modinfo, owners)
    memo[path] = key
    return key


def build(owners_path):
    owners, modinfo, node_of, facet_of = fm.build(owners_path)
    lines = fm.node_lines(modinfo, facet_of)
    for n in lines:
        if n not in facet_of:
            facet_of[n] = fm.REGION_FACETS.get(n[11:], "store") if n.startswith("store-root/") else fm.facet_for_file(modinfo[n].file, modinfo[n].kind)
    refs = defaultdict(set)
    item_node = {}
    item_lines = defaultdict(int)
    for p, o in owners.items():
        k = item_key(p, modinfo, owners)
        if p == k: item_node[k] = node_of[p]
        else: item_node.setdefault(k, node_of[p])
        for f, (a, b) in o["spans"].items():
            pass
        for t in o["refs"]:
            tk = item_key(t, modinfo, owners)
            if tk != k: refs[k].add(tk)
    for p, o in owners.items():
        k = item_key(p, modinfo, owners)
        if "::" in p[len(k):].lstrip(":"): continue
        mfile = modinfo[fm.module_of(p, modinfo)].file
        for f, (a, b) in o["spans"].items():
            if os.path.normpath(os.path.join(fm.REPO, f)) == os.path.normpath(mfile) and b >= a:
                item_lines[k] = max(item_lines[k], b - a + 1)
    return owners, modinfo, node_of, facet_of, refs, item_node, item_lines


def closure(seeds, refs, allowed):
    seen, q = set(seeds), deque(seeds)
    while q:
        x = q.popleft()
        for y in refs.get(x, ()):
            if y not in seen and allowed(y):
                seen.add(y); q.append(y)
    return seen


def main():
    owners_path, cmd = sys.argv[1], sys.argv[2]
    owners, modinfo, node_of, facet_of, refs, item_node, item_lines = build(owners_path)
    fac = lambda k: facet_of.get(item_node.get(k, ""), "?")
    if cmd == "needs":
        A, B = sys.argv[3], sys.argv[4]
        seeds = sorted({t for k, ts in refs.items() if fac(k) == A for t in ts if fac(t) == B})
        print(f"## {len(seeds)} items of {B} referenced by {A}")
        for s in seeds: print("  ", s, f"[{item_node.get(s)}]")
        c = closure(seeds, refs, lambda y: fac(y) == B)
        report(c, item_node, item_lines, refs, fac, B)
    elif cmd == "closure":
        seeds = sys.argv[3:]
        allowed_facets = {fac(s) for s in seeds}
        c = closure(seeds, refs, lambda y: fac(y) in allowed_facets)
        report(c, item_node, item_lines, refs, fac, None)
    elif cmd == "why":
        A, B, target = sys.argv[3], sys.argv[4], sys.argv[5]
        seeds = sorted({t for k, ts in refs.items() if fac(k) == A for t in ts if fac(t) == B})
        prev, q = {s: None for s in seeds}, deque(seeds)
        while q:
            x = q.popleft()
            if x == target: break
            for y in sorted(refs.get(x, ())):
                if y not in prev and fac(y) == B:
                    prev[y] = x; q.append(y)
        chain, x = [], target if target in prev else None
        while x: chain.append(x); x = prev[x]
        print(" -> ".join(reversed(chain)) if chain else "unreachable")
    elif cmd == "users":
        t = sys.argv[3]
        for k, ts in sorted(refs.items()):
            if t in ts: print(f"{fac(k):18} {k}  [{item_node.get(k)}]")


def report(c, item_node, item_lines, refs, fac, B):
    by = defaultdict(list)
    for k in c: by[item_node.get(k, "?")].append(k)
    print(f"\n## closure: {len(c)} items, ~{sum(item_lines[k] for k in c)} lines (item spans)")
    for n, ks in sorted(by.items(), key=lambda x: -sum(item_lines[k] for k in x[1])):
        print(f"  {n}: {len(ks)} items, ~{sum(item_lines[k] for k in ks)} lines :: " + ", ".join(sorted(k.split('::')[-1] for k in ks)[:30]))


if __name__ == "__main__":
    main()
