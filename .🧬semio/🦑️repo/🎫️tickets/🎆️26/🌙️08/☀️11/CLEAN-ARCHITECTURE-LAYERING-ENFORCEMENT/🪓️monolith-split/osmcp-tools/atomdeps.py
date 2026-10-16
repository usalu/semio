import sys, collections
sys.path.insert(0, ".")
from graph2 import *
w, edges, flags = build_graph()
byid = {id(a): a for a in w.atoms}
names = sys.argv[1:]
for a in w.atoms:
    if a.name in names and id(a) in edges or a.name in names:
        if id(a) in w.mount_atoms: continue
        print(f"== {a.name} [{a.kind}] {topmod(a.file)} {chunk_of(a)} L{a.item.line0} ({a.text.count(chr(10))+1} lines) test={flags[id(a)]}")
        for t in sorted(edges.get(id(a), ()), key=lambda x: byid[x].name):
            b = byid[t]
            print(f"      -> {b.name} [{b.kind}] {topmod(b.file)} {chunk_of(b)}")
        rev = [byid[s] for s, ts in edges.items() if id(a) in ts]
        print("      used by:", sorted(set(f"{topmod(x.file)}::{x.name}" for x in rev))[:40])
