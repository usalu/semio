import sys, collections, pickle, os
sys.path.insert(0, ".")
from resolver import *
from chunks import chunk_of, tarjan

_W = None
def world():
    global _W
    if _W is None: _W = World()
    return _W

def build_graph(w=None, test=False):
    w = w or world()
    flags = {id(a): flags_for(w, a) for a in w.atoms}
    edges = collections.defaultdict(set)
    for a in w.atoms:
        if id(a) in w.mount_atoms: continue
        if flags[id(a)] and not test: continue
        for t in w.atom_refs(a):
            if id(t) in w.mount_atoms: continue
            if flags[id(t)] and not test: continue
            edges[id(a)].add(id(t))
    return w, edges, flags

if __name__ == "__main__":
    w, edges, flags = build_graph()
    prod = [id(a) for a in w.atoms if not flags[id(a)] and id(a) not in w.mount_atoms]
    comps = tarjan(prod, edges)
    big = [c for c in comps if len(c) > 1]
    byid = {id(a): a for a in w.atoms}
    print("prod atoms", len(prod), "sccs>1", sorted(len(c) for c in big))
    for c in sorted(big, key=len, reverse=True):
        print(len(c), [ (byid[x].name, topmod(byid[x].file)) for x in c][:12])
