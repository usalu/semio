"""File-level symbol dependency graph of one crate closure (rust files only): which file uses names defined by which other file."""
import sys, json, re, os, collections
sys.path.insert(0, os.path.dirname(__file__))
from rgp import *
m = sys.argv[1]
files = [f for f in closure(m) if f["p"].endswith(".rs")]
base = os.path.dirname(os.path.dirname(os.path.dirname(m)))
short = lambda p: p.replace(base, "~")[-48:]
defs = {}
impls = collections.defaultdict(set)
texts = {}
for f in files:
    t = rd(f["p"]); texts[f["p"]] = t
    d = defined_names(t)
    for n, k in d.items():
        if k != "mod":
            defs.setdefault(n, set()).add(f["p"])
    for mm in re.finditer(r'(?m)^\s*impl(?:<[^>]*>)?\s+(?:[A-Za-z0-9_:<>, ]+?\s+for\s+)?([A-Z][A-Za-z0-9_]*)', t):
        impls[f["p"]].add(mm.group(1))
    # macro_rules
    for mm in re.finditer(r'macro_rules!\s*([A-Za-z0-9_]+)', t):
        defs.setdefault(mm.group(1), set()).add(f["p"])
edges = collections.defaultdict(lambda: collections.defaultdict(set))
for f in files:
    p = f["p"]; ids = idents(texts[p])
    for n, fs in defs.items():
        if n in ids and p not in fs:
            for g in fs:
                edges[p][g].add(n)
for f in files:
    p = f["p"]
    print("%5d %s" % (f["l"], short(p)))
    for g, ns in sorted(edges[p].items(), key=lambda x: -len(x[1])):
        print("        -> %-48s %3d  %s" % (short(g), len(ns), sorted(ns)[:5]))
    if impls[p]:
        out = [t for t in impls[p] if t in defs and p not in defs[t]]
        if out: print("        IMPL-FOREIGN-TYPE:", out, [short(x) for t in out for x in defs[t]])
