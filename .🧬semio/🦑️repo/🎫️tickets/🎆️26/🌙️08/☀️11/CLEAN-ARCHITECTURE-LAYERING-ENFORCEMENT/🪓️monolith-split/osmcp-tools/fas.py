import sys, collections
sys.path.insert(0, ".")
from chunks import *
files, allitems, atoms, defs, loose = build()
edges, amb, flags = atom_edges(files, atoms, defs)
byid = {id(a): a for a in atoms}
prod = [a for a in atoms if not flags[id(a)] and a.kind != 'mod']
ch = {id(a): chunk_of(a) for a in prod}
csize = collections.Counter(); catoms = collections.defaultdict(list)
for a in prod:
    csize[ch[id(a)]] += a.text.count("\n") + 1; catoms[ch[id(a)]].append(a)
w = collections.defaultdict(lambda: collections.Counter())  # w[src][dst] = weight
for s, ts in edges.items():
    if s not in ch: continue
    for t in ts:
        if t in ch and ch[s] != ch[t]: w[ch[s]][ch[t]] += 1
nodes = sorted(csize)
# Eades-Lin-Smyth heuristic: order so that edges go from earlier to later = dependents first. We want dependents (sources) later? 
# define order where for edge s->t (s depends on t), t should come BEFORE s (lower layer first).
out_w = {n: sum(w[n].values()) for n in nodes}
in_w = collections.Counter()
for s in w:
    for t, c in w[s].items(): in_w[t] += c
remaining = set(nodes); left = []; right = []
alive_out = {n: dict(w[n]) for n in nodes}
alive_in = collections.defaultdict(dict)
for s in w:
    for t, c in w[s].items(): alive_in[t][s] = c
def remove(n):
    for t in list(alive_out[n]): alive_in[t].pop(n, None)
    for s in list(alive_in[n]): alive_out[s].pop(n, None)
    remaining.discard(n)
while remaining:
    changed = True
    while changed:
        changed = False
        for n in sorted(remaining):
            if not alive_out[n]:   # sink: depends on nothing -> goes to the front (lowest layer)
                left.append(n); remove(n); changed = True
        for n in sorted(remaining):
            if n in remaining and not alive_in[n]:  # source: nobody depends on it -> top
                right.insert(0, n); remove(n); changed = True
    if remaining:
        best = max(remaining, key=lambda n: sum(alive_in[n].values()) - sum(alive_out[n].values()))
        # pick node with max (in - out): many depend on it, depends on few -> lower layer
        left.append(best); remove(best)
order = left + right
pos = {n: i for i, n in enumerate(order)}
back = []
for s in w:
    for t, c in w[s].items():
        if pos[t] > pos[s]: back.append((s, t, c))
print("ORDER (low -> high):")
for n in order: print(f"{pos[n]:3d} {csize[n]:5d} {n}")
print("\nBACK EDGES (s depends on t but t is higher):")
for s, t, c in sorted(back, key=lambda x: -x[2]):
    print(f"  {s}  ->  {t}   x{c}")
print(len(back), "back edges, weight", sum(c for _,_,c in back))
