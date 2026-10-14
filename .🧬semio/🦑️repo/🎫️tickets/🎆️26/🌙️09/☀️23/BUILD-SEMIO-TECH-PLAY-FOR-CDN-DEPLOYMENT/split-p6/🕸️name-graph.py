#!/usr/bin/env python3
"""🕸️ Name-based item reference graph of one `#[path]`-tree crate (catches glob-imported uses the path graph misses).

Top-level items (indent 0, or one level inside an inline `mod {}`) define names; a non-test file references a name
defined uniquely in another file. Files fold onto groups (first matching regex on the artifact-relative path).
Prints per-group lines, group edges with the names that cause them, and the strongly connected components.

usage: 🕸️name-graph.py <crate-root.rs> <groups.json> [--names N] [--files]
"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
CENSUS = os.path.join(HERE, "..", "e3-format-split", "🕸️module-graph.py")
ns = {}
exec(compile(open(CENSUS, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CENSUS, "exec"), ns)
crate = ns["Crate"](sys.argv[1]); test_spans = ns["test_spans"]
rules = [(re.compile(r), g) for r, g in json.load(open(sys.argv[2]))]
show = int(sys.argv[sys.argv.index("--names") + 1]) if "--names" in sys.argv else 8
rel = lambda f: os.path.relpath(f, crate.base)
is_test = lambda f: f in crate.test_files or "🧪️tests" in rel(f)
def group(f):
    for r, g in rules:
        if r.search(rel(f)): return g
    return "?" + rel(f)
DEF = re.compile(r"(?m)^(?:    )?(?:pub(?:\s*\([^)]*\))?\s+)?(?:(?:const|async|unsafe|extern\s+\"C\")\s+)*(fn|struct|enum|trait|type|const|static|union)\s+([A-Za-z_][A-Za-z0-9_]*)|^\s*macro_rules!\s*([A-Za-z_][A-Za-z0-9_]*)")
defs = collections.defaultdict(set); body = {}
for f in crate.files:
    if is_test(f): continue
    s = crate.text[f]
    for a, b in sorted(test_spans(s), reverse=True): s = s[:a] + " " * (b - a + 1) + s[b + 1:]
    # blank impl/trait bodies' nested fns from definitions: only count indent-0 or 4-space items not inside impl
    body[f] = s
    depth_at = []
    for m in DEF.finditer(s):
        line_start = s.rfind("\n", 0, m.start()) + 1
        prefix = s[line_start:m.start()]
        before = s[:m.start()]
        depth = before.count("{") - before.count("}")
        if depth == 0 or (depth == 1 and re.search(r"mod\s+\w+\s*\{[^{}]*$", before[-4000:] if False else "")):
            defs[m.group(2) or m.group(3)].add(f)
unique = {n: next(iter(fs)) for n, fs in defs.items() if len(fs) == 1 and (n[0].isupper() or "_" in n) and len(n) > 6}
lines = collections.Counter(); files = collections.Counter()
for f in crate.files:
    if is_test(f): continue
    lines[group(f)] += len(body[f].splitlines()); files[group(f)] += 1
edges = collections.defaultdict(lambda: collections.defaultdict(set))
for f, s in body.items():
    a = group(f)
    for name in set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", s)):
        g = unique.get(name)
        if g is None or g == f: continue
        b = group(g)
        if a != b: edges[a][b].add(name)
nodes = sorted(lines)
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
    print(f"G {lines[g]:6d} {files[g]:4d} {g}")
    for b, names in sorted(edges[g].items()):
        print(f"      -> {b} ({len(names)}): {', '.join(sorted(names)[:show])}")
for comp in sccs:
    if len(comp) > 1: print(f"SCC {sum(lines[c] for c in comp)} lines: {sorted(comp)}")
if "--files" in sys.argv:
    for f in sorted(body, key=rel): print(f"F {group(f)}\t{len(body[f].splitlines())}\t{rel(f)}")
