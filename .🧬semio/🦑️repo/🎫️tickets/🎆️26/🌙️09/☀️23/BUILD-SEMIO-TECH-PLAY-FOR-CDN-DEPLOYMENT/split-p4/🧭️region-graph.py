#!/usr/bin/env python3
"""🧭️ Region-level reference graph of one Rust file: which `//#region` group's items name which other group's items
(bare identifiers), with line totals per group. Groups are region paths cut at `--depth`.

usage: 🧭️region-graph.py <file.rs> [--depth N] [--group name=regex ...]
"""
import collections, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ns = {"__file__": os.path.join(HERE, "🔀️item-split.py")}
exec(compile(open(os.path.join(HERE, "🔀️item-split.py"), encoding="utf-8").read().split("\nif __name__")[0], "item-split", "exec"), ns)
src, items = ns["parse"](sys.argv[1])
depth = int(sys.argv[sys.argv.index("--depth") + 1]) if "--depth" in sys.argv else 3
key = lambda it: "/".join(it.region[:depth]) or "(top)"
names = collections.defaultdict(list)
for it in items:
    if it.kind not in ("impl", "use", "macro", "?"):
        names[it.name].append(it)
owner = {}
for it in items:
    if it.kind == "impl" and it.target in names:
        owner[it.i] = names[it.target][0]
size = collections.Counter()
graph = collections.defaultdict(collections.Counter)
why = collections.defaultdict(set)
for it in items:
    k = key(owner[it.i]) if it.i in owner else key(it)
    size[k] += it.text.count("\n") + 1
    if it.kind == "use":
        continue
    for ident in it.idents:
        for t in names.get(ident, []):
            tk = key(t)
            if tk != k:
                graph[k][tk] += 1
                why[(k, tk)].add(ident)
for k in sorted(size, key=lambda k: min(i.start for i in items if key(i) == k) if any(key(i) == k for i in items) else 0):
    print(f"{size[k]:5d}  {k}")
    for t, n in sorted(graph[k].items(), key=lambda x: -x[1]):
        print(f"         -> {t}: {n}  [{', '.join(sorted(why[(k, t)])[:8])}]")
