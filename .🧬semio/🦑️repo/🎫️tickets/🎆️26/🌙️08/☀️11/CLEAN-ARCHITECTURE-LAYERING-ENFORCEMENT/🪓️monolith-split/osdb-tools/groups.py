"""🧮 Check a proposed item->group assignment for a set of files: sizes and cross-group edges.
Usage as module: check(files, assign) where assign(fid,item)->group name (None to ignore)."""
import collections
from graph import *
def check(files, assign, order=None, show=12):
    nodes=load(files)
    G={}
    for k,n in nodes.items():
        base={assign(fid,it) for fid,it in n.parts if it.kind!='impl'}
        base.discard(None)
        if base:
            if len(base)>1: print('SPLIT NODE',k,base)
            G[k]=sorted(base)[0]
        else:
            gs={assign(fid,it) for fid,it in n.parts}; gs.discard(None)
            if len(gs)>1: print('SPLIT IMPL-ONLY NODE',k,gs)
            G[k]=sorted(gs)[0] if gs else None
    g=edges(nodes)
    c=collections.defaultdict(list)
    for k in g:
        for w in g[k]:
            if G[k] and G[w] and G[k]!=G[w]: c[(G[k],G[w])].append((k,w))
    size=collections.Counter()
    for k,n in nodes.items():
        if G[k]: size[G[k]]+=n.lines
    print('sizes',dict(size))
    for (a,b),l in sorted(c.items()): print(f'  {a} -> {b}: {len(l)}',l[:show])
    # group-level cycle detection
    from scc2 import sccs
    gg={x:set() for x in size}
    for (a,b) in c: gg[a].add(b)
    cyc=[x for x in sccs(gg) if len(x)>1]
    print('GROUP CYCLES:',cyc)
    return nodes,G,g
