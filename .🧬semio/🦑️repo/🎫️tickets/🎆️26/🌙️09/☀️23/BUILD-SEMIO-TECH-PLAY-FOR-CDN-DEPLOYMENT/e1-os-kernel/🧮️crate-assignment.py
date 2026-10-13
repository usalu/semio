#!/usr/bin/env python3
"""🧮️ Checks a target crate graph against the exact HIR reference graph of `semio-framework-os-kernel`.

`🎯️target-crates.json` lists the target crates bottom-up, each with its allowed dependencies and the node rules
(regexes over `🧭️facet-map.py` node names: module paths, or `store-root/<region>` inside the store root) it owns.
Every production node must match exactly one crate. Reports per-crate non-blank lines and every reference that
leaves a crate for one it may not depend on (a seam still to cut), with the referenced items.

Usage: crate-assignment.py OWNERS_JSON [--seams] [--unassigned]
"""
import importlib.util, json, os, re, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("fm", os.path.join(HERE, "🧭️facet-map.py"))
fm = importlib.util.module_from_spec(spec); spec.loader.exec_module(fm)


def main():
    owners_path = sys.argv[1]
    target = json.load(open(os.path.join(HERE, "🎯️target-crates.json")))
    crates = target["crates"]
    owners, modinfo, node_of, facet_of = fm.build(owners_path)
    lines = fm.node_lines(modinfo, facet_of)
    for n in lines:
        if n not in facet_of:
            facet_of[n] = fm.REGION_FACETS.get(n[11:], "store") if n.startswith("store-root/") else fm.facet_for_file(modinfo[n].file, modinfo[n].kind)
    prod = [n for n in lines if facet_of[n] != "laws"]
    crate_of, unassigned, multi = {}, [], []
    for n in prod:
        hits = [c["name"] for c in crates if any(re.search(r, n) for r in c["nodes"])]
        if not hits: unassigned.append(n)
        elif len(hits) > 1 and not any(re.fullmatch(r, n) for c in crates if c["name"] == hits[0] for r in c["nodes"]): multi.append((n, hits)); crate_of[n] = hits[0]
        else: crate_of[n] = hits[0]
    allowed = {}
    for c in crates:
        deps = set(c.get("deps", []))
        frontier = list(deps)
        while frontier:
            d = frontier.pop()
            for c2 in crates:
                if c2["name"] == d:
                    for dd in c2.get("deps", []):
                        if dd not in deps: deps.add(dd); frontier.append(dd)
        allowed[c["name"]] = deps | {c["name"]}
    size = defaultdict(int)
    for n, c in crate_of.items(): size[c] += lines[n]
    seams = defaultdict(lambda: defaultdict(set))
    for p, o in owners.items():
        a = node_of[p]
        if a not in crate_of: continue
        for t in o["refs"]:
            b = node_of.get(t)
            if b is None:
                segs = t.split("::")
                for k in range(len(segs), 0, -1):
                    b = node_of.get("::".join(segs[:k]))
                    if b: break
            if b is None or b not in crate_of: continue
            ca, cb = crate_of[a], crate_of[b]
            if cb not in allowed[ca]:
                seams[(ca, cb)][(a, b)].add(t.split("::")[-1] if not t.split("::")[-1].startswith("{") else "::".join(t.split("::")[-2:]))
    total = sum(size.values())
    print(f"## {len(crates)} target crates, {total} non-blank production lines")
    for c in crates:
        print(f"  {c['name']:34} {size[c['name']]:6}  deps: {', '.join(c.get('deps', [])) or '-'}")
    print(f"\n## seams: {sum(len(v) for v in seams.values())} node pairs, {sum(len(s) for v in seams.values() for s in v.values())} item references")
    for (ca, cb), pairs in sorted(seams.items(), key=lambda x: -sum(len(s) for s in x[1].values())):
        print(f"  {ca} -> {cb}: {sum(len(s) for s in pairs.values())}")
        if "--seams" in sys.argv:
            for (a, b), ts in sorted(pairs.items(), key=lambda x: -len(x[1])):
                print(f"      {a} => {b}: {', '.join(sorted(ts)[:14])}")
    if unassigned:
        print(f"\n## unassigned nodes: {len(unassigned)}")
        if "--unassigned" in sys.argv:
            for n in sorted(unassigned, key=lambda n: -lines[n]): print(f"  {lines[n]:6} {facet_of[n]:16} {n}")
    for n, hits in multi:
        print(f"  [ambiguous] {n}: {hits}")


if __name__ == "__main__":
    main()
