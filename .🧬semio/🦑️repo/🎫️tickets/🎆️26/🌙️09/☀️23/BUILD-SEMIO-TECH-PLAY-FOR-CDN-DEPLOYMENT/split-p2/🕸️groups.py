#!/usr/bin/env python3
"""🕸️ File-level reference graph of the remodeling monolith folded onto candidate crate groups (lane P2 cut planning).

Reuses E3's census `Crate` walker, resolves the crate-root aliases (`crate::algebra`, `crate::mutations`, …) and prints
group sizes plus every inter-group edge (non-test and test-only separately).

usage: 🕸️groups.py [--edges GROUP_A GROUP_B]
"""
import os, re, sys, collections, json
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CENSUS = os.path.join(HERE, "..", "e3-format-split", "🕸️module-graph.py")
g = {}
exec(compile(open(CENSUS, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CENSUS, "exec"), g)
ART = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling")
crate = g["Crate"](os.path.join(ART, "🦀️.rs"))
S = ("standards", "v1", "subsets", "any")
ALIASES = {("algebra",): S + ("schema", "algebra_internals"), ("lie",): S + ("schema", "lie_internals"), ("optimize",): S + ("schema", "optimize_internals"),
           ("signal",): S + ("schema", "signal_internals"), ("spatial",): S + ("schema", "spatial_internals"), ("mutations",): S + ("schema", "mutations"),
           ("diff",): S + ("schema", "diff"), ("snapshot", "schema"): S + ("schema", "snapshot"), ("schema",): S + ("schema",), ("diff", "schema"): S + ("schema", "diff")}
orig = crate.resolve
def resolve(mp):
    for k in sorted(ALIASES, key=len, reverse=True):
        if tuple(mp[:len(k)]) == k:
            mp = ALIASES[k] + tuple(mp[len(k):]); break
    return orig(tuple(mp))
crate.resolve = resolve
RULES = json.load(open(os.path.join(HERE, "🕸️groups.json"), encoding="utf-8"))
RULES = [(re.compile(r), n) for r, n in RULES]
def rel(f): return os.path.relpath(f, ART).replace("🏅️standards/🔖️1/🪆️subsets/✳️any/", "S/")
def group(f):
    r = rel(f)
    for rx, n in RULES:
        m = rx.search(r)
        if m: return m.expand(n) if "\\" in n else n
    return "?"
def is_test(f): return f in crate.test_files or "🧪️tests" in rel(f)
size = collections.defaultdict(lambda: [0, 0, 0, 0])
for f in crate.files:
    n = sum(1 for _ in open(f, encoding="utf-8"))
    s = size[group(f)]
    if is_test(f): s[2] += 1; s[3] += n
    else: s[0] += 1; s[1] += n
edges, tedges = collections.defaultdict(set), collections.defaultdict(set)
for f in crate.files:
    a = group(f)
    for t in g["references"](crate, f):
        b = group(t)
        if a != b: (tedges if is_test(f) else edges)[(a, b)].add((rel(f), rel(t)))
want = sys.argv[sys.argv.index("--edges") + 1:sys.argv.index("--edges") + 3] if "--edges" in sys.argv else None
if want:
    for x, y in sorted(edges.get(tuple(want), set()) | tedges.get(tuple(want), set())): print(f"{x}  ->  {y}")
    sys.exit()
print("group: libFiles libLines testFiles testLines (raw lines)")
for k, v in sorted(size.items()): print(f"  {k:28} {v[0]:4} {v[1]:6} {v[2]:4} {v[3]:6}")
print("edges (non-test):")
for (a, b), es in sorted(edges.items()): print(f"  {a} -> {b}: {len(es)}")
print("edges (test-only):")
for (a, b), es in sorted(tedges.items()):
    if (a, b) not in edges: print(f"  {a} -> {b}: {len(es)}")
