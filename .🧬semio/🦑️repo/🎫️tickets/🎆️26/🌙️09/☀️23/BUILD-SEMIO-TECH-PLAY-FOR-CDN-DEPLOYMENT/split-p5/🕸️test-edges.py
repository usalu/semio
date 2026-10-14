#!/usr/bin/env python3
"""🧪️ Test-file layer edges of one `#[path]`-tree crate: every `🧪️tests` file's references to files of another layer.

usage: 🕸️test-edges.py <crate-root.rs> <layers.json>
"""
import os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
g = {}
src = open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0]
exec(compile(src, "module-graph", "exec"), g)
import json
root, layers = sys.argv[1], sys.argv[2]
cfg = json.load(open(layers)); crate = g["Crate"](root)
rules = [(re.compile(r), l) for r, l in cfg["rules"]]
rel = lambda f: os.path.relpath(f, crate.base)
layer = lambda f: next((l for r, l in rules if r.search(rel(f))), "?")
edges = collections.defaultdict(set)
for f in crate.files:
    if "🧪️tests" not in rel(f): continue
    a = layer(f)
    for t in g["references"](crate, f):
        b = layer(t)
        if a != b: edges[(a, b)].add((rel(f), rel(t)))
for (a, b), es in sorted(edges.items()):
    print(f"### {a} -> {b} ({len(es)})")
    for x, y in sorted(es): print(f"  {x}  ->  {y}")
