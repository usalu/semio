import sys, collections, statistics
sys.path.insert(0, ".")
from chunks import *
files, allitems, atoms, defs, loose = build()
edges, amb, flags = atom_edges(files, atoms, defs)
byid = {id(a): a for a in atoms}
prod = [a for a in atoms if not flags[id(a)] and a.kind != 'mod']
pid = [id(a) for a in prod]
comps = tarjan(pid, edges)
comp_of = {}
for i, c in enumerate(comps):
    for x in c: comp_of[x] = i
cg = collections.defaultdict(set)
for s, ts in edges.items():
    if s not in comp_of: continue
    for t in ts:
        if t in comp_of and comp_of[s] != comp_of[t]: cg[comp_of[s]].add(comp_of[t])
depth = {}
sys.setrecursionlimit(100000)
def d(c):
    if c in depth: return depth[c]
    depth[c] = 0
    depth[c] = 1 + max([d(x) for x in cg.get(c, ())] + [-1])
    return depth[c]
for c in range(len(comps)): d(c)
byfacet = collections.defaultdict(list)
for a in prod:
    byfacet[topmod(a.file)].append((depth[comp_of[id(a)]], a))
for fct, lst in sorted(byfacet.items(), key=lambda kv: statistics.median([x[0] for x in kv[1]])):
    ds = [x[0] for x in lst]
    print(f"{fct:16s} n={len(ds):4d} min={min(ds):2d} med={statistics.median(ds):4.1f} max={max(ds):2d}")
print("max depth", max(depth.values()))
