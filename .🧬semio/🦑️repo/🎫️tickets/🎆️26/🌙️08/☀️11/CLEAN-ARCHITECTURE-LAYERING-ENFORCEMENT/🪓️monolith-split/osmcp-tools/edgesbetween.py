import sys, collections, fnmatch
sys.path.insert(0, ".")
from graph2 import *
w, edges, flags = build_graph()
byid = {id(a): a for a in w.atoms}
prod = [a for a in w.atoms if not flags[id(a)] and id(a) not in w.mount_atoms]
ch = {id(a): chunk_of(a) for a in prod}
src, dst = sys.argv[1], sys.argv[2]
for s, ts in edges.items():
    if not fnmatch.fnmatchcase(ch[s], src): continue
    for t in ts:
        if fnmatch.fnmatchcase(ch[t], dst):
            print(f"{ch[s]}::{byid[s].name}  ->  {ch[t]}::{byid[t].name}")
