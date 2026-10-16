from lane import *
import sys, random, pickle
tree, mods = load_mods()
nodes = build(mods)
g, amb = edges(nodes, 'none')
def lane_of(n): return 'W' if n.mod.startswith('🌍️world') else ('B' if n.mod.startswith('🎲️board') else 'X')
ids = set(n.id for n in nodes if lane_of(n) == 'B')
def seed(n):
    return n.mod
mlist = sorted(set(nodes[i].mod for i in ids))
modof = {i: nodes[i].mod for i in ids}
W = collections.defaultdict(lambda: collections.defaultdict(int))
for i in ids:
    for d in g[i]:
        if d in ids and modof[d] != modof[i]: W[modof[i]][modof[d]] += 1
def cost(order):
    p = {m:i for i,m in enumerate(order)}
    return sum(w for a in W for b,w in W[a].items() if p[b] > p[a])
indeg = collections.Counter()
for a in W:
    for b, w in W[a].items(): indeg[b] += w
order = sorted(mlist, key=lambda m: sum(W[m].values()) - indeg.get(m,0))
best = cost(order); improved = True
while improved:
    improved = False
    for i in range(len(order)):
        m = order[i]; rest = order[:i]+order[i+1:]; bj, bc = i, best
        for j in range(len(rest)+1):
            c = cost(rest[:j]+[m]+rest[j:])
            if c < bc: bc, bj = c, j
        if bc < best: order = rest[:bj]+[m]+rest[bj:]; best = bc; improved = True
print('module order cost', best)
pickle.dump(order, open('/tmp/x/board-order.pkl','wb'))
for i, m in enumerate(order): print(i, m.replace('🎲️board/🔌️ports/➡️directed/','D/'), sum(nodes[x].lines for x in ids if nodes[x].mod == m))
print('--- violations')
p = {m:i for i,m in enumerate(order)}
viol = sorted(((w,a,b) for a in W for b,w in W[a].items() if p[b] > p[a]), reverse=True)
for w,a,b in viol[:25]: print(w, a.split('/')[-2], '->', b.split('/')[-2])
