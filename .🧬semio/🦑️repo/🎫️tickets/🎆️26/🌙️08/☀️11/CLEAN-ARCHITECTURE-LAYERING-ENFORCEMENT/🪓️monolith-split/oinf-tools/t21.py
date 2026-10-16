from cfgboard import *
tree, mods = load_mods()
nodes = build(mods)
g, amb = edges(nodes, 'none')
def merge(res, order, budget=3200):
    groups=[];cur=[];cs=0
    for r in sorted(res['sizes']):
        s=res['sizes'][r]
        if cur and cs+s>budget: groups.append((cur,cs)); cur=[];cs=0
        cur.append(r); cs+=s
    if cur: groups.append((cur,cs))
    return groups
ids = set(n.id for n in nodes if n.mod.startswith('🎲️board'))
res = run_lane(nodes, g, ids, seed, ORDER, SPLIT)
print('BOARD crates after adjacent merge (budget 3200 node-lines):')
for gr, sz in merge(res, ORDER):
    print(sz, [ORDER[r].replace(D,'D/').split('/')[-2] for r in gr])
print('BoardHost/DagHost methods by band:')
for T in ('BoardHost','DagHost'):
    c = collections.Counter()
    for u in res['units']:
        for m in u.members:
            n = nodes[m]
            if n.parent == T and n.kind == 'method': c[ORDER[res['A'][u.id]].replace(D,'D/').split('/')[-2]] += n.lines
    print(T, dict(c), 'struct band', ORDER[res['tyband'][T]].split('/')[-2])
