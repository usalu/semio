import sys, collections
sys.path.insert(0, ".")
from agraph import *
files, allitems, atoms, defs, loose = build()
edges, amb, flags = atom_edges(files, atoms, defs)
byid = {id(a): a for a in atoms}
cnt = collections.Counter()
lines = collections.Counter()
for s, ts in edges.items():
    a = byid[s]
    for t in ts:
        b = byid[t]
        fa, fb = topmod(a.file), topmod(b.file)
        if fa != fb: cnt[(fa, fb)] += 1
facets = sorted(set(topmod(a.file) for a in atoms if not flags[id(a)]))
sz = collections.Counter()
for a in atoms:
    if not flags[id(a)] and a.kind != 'mod': sz[topmod(a.file)] += a.text.count("\n") + 1
print("facet sizes(lines incl blank):", dict(sz))
for (fa, fb), c in sorted(cnt.items()):
    back = cnt.get((fb, fa), 0)
    print(f"{fa:14s} -> {fb:14s} {c:3d}   (back {back})")
