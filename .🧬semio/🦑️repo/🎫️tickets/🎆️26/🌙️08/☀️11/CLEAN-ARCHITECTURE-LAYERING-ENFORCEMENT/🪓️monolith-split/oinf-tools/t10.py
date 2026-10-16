from graph2 import *
import itertools, random
tree, mods = load_mods()
nodes = build(mods)
g, amb = edges(nodes, 'none')
# module of node: file; but type's methods count with file where they are
modof = {n.id: n.mod for n in nodes}
ms = sorted(set(modof.values()))
W = collections.defaultdict(lambda: collections.defaultdict(int))   # W[a][b] = number of node edges a->b (a depends on b)
for n in nodes:
    for d in g[n.id]:
        a, b = modof[n.id], modof[d]
        if a != b: W[a][b] += 1
size = {m: sum(n.lines for n in nodes if n.mod == m) for m in ms}
# linear ordering: rank low = depended-upon. Minimize sum of W[a][b] where rank(b) > rank(a) (violations).
def cost(order):
    pos = {m: i for i, m in enumerate(order)}
    c = 0
    for a in W:
        for b, w in W[a].items():
            if pos[b] > pos[a]: c += w
    return c
# initial: sort by (out-degree - in-degree)
indeg = collections.Counter(); 
for a in W:
    for b, w in W[a].items(): indeg[b] += w
outdeg = {a: sum(W[a].values()) for a in ms}
order = sorted(ms, key=lambda m: outdeg.get(m,0) - indeg.get(m,0))
best = cost(order)
random.seed(7)
improved = True
while improved:
    improved = False
    for i in range(len(order)):
        m = order[i]
        rest = order[:i] + order[i+1:]
        bj, bc = i, best
        for j in range(len(rest)+1):
            cand = rest[:j] + [m] + rest[j:]
            c = cost(cand)
            if c < bc: bc, bj = c, j
        if bc < best:
            order = rest[:bj] + [m] + rest[bj:]
            best = bc; improved = True
print('violation cost', best, 'total edges', sum(w for a in W for w in W[a].values()))
pos = {m: i for i, m in enumerate(order)}
for i, m in enumerate(order):
    print(i, m.replace('🎲️board/🔌️ports/➡️directed/','D/'), size[m])
print('--- violations')
viol = [(w, a, b) for a in W for b, w in W[a].items() if pos[b] > pos[a]]
for w, a, b in sorted(viol, reverse=True)[:40]:
    print(w, a.split('/')[-2], '->', b.split('/')[-2])
