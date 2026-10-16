import sys, collections
sys.path.insert(0, ".")
from agraph import *
files, allitems, atoms, defs, loose = build()
edges, amb, flags = atom_edges(files, atoms, defs)
byid = {id(a): a for a in atoms}
pairs = [tuple(x.split(">")) for x in sys.argv[1:]]
for fa, fb in pairs:
    print("###", fa, "->", fb)
    for s, ts in edges.items():
        a = byid[s]
        if topmod(a.file) != fa: continue
        for t in ts:
            b = byid[t]
            if topmod(b.file) == fb:
                print("  ", a.kind, a.name, a.item.line0, "->", b.kind, b.name, b.item.line0)
