#!/usr/bin/env python3
"""🕸️ File-level reference graph of one `#[path]`-tree crate, folded onto groups (first matching regex on the
artifact-relative path), with per-group non-test lines and strongly connected components of the group graph.

usage: 🕸️file-graph.py <crate-root.rs> <groups.json> [--edges]
groups.json: [[regex, group]...]   (first match wins; unmatched files fall into their own path)
"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
CENSUS = os.path.join(HERE, "..", "e3-format-split", "🕸️module-graph.py")
ns = {}
exec(compile(open(CENSUS, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CENSUS, "exec"), ns)
crate = ns["Crate"](sys.argv[1]); refs = ns["references"]
rules = [(re.compile(r), g) for r, g in json.load(open(sys.argv[2]))]
rel = lambda f: os.path.relpath(f, crate.base)
is_test = lambda f: f in crate.test_files or "🧪️tests" in rel(f)
def group(f):
    for r, g in rules:
        if r.search(rel(f)): return g
    return "?" + rel(f)
lines = collections.Counter(); tl = collections.Counter(); files = collections.Counter()
for f in crate.files:
    n = len(ns["strip"](open(f, encoding="utf-8").read()).splitlines())
    (tl if is_test(f) else lines)[group(f)] += n; files[group(f)] += 1
edges = collections.defaultdict(collections.Counter); detail = collections.defaultdict(set)
for f in crate.files:
    if is_test(f): continue
    a = group(f)
    for g in refs(crate, f):
        if is_test(g): continue
        b = group(g)
        if a != b: edges[a][b] += 1; detail[(a, b)].add((rel(f), rel(g)))
nodes = sorted(set(lines) | set(tl))
index, low, stack, on, sccs, counter = {}, {}, [], set(), [], [0]
sys.setrecursionlimit(10000)
def strong(v):
    index[v] = low[v] = counter[0]; counter[0] += 1; stack.append(v); on.add(v)
    for w in edges[v]:
        if w not in index: strong(w); low[v] = min(low[v], low[w])
        elif w in on: low[v] = min(low[v], index[w])
    if low[v] == index[v]:
        comp = []
        while True:
            w = stack.pop(); on.discard(w); comp.append(w)
            if w == v: break
        sccs.append(comp)
for v in nodes:
    if v not in index: strong(v)
for g in nodes:
    print(f"G\t{lines[g]}\t{tl[g]}\t{files[g]}\t{g}\t-> {', '.join(f'{b}({n})' for b, n in sorted(edges[g].items()))}")
for comp in sccs:
    if len(comp) > 1:
        print(f"SCC {sum(lines[c] for c in comp)} lines: {sorted(comp)}")
        if "--edges" in sys.argv:
            for a in comp:
                for b in comp:
                    if (a, b) in detail:
                        for x, y in sorted(detail[(a, b)])[:6]: print(f"   {a}->{b}: {x} -> {y}")
