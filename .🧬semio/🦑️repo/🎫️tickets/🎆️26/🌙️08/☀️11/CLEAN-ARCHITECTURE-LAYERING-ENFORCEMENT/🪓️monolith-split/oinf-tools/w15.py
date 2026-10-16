from lane import *
exec(open('w14.py').read().split("bands = sorted")[0])
units, unit_of = build_units2(nodes, ids, set(), set())
ug = unit_graph(nodes, g, units, unit_of, ids)
units, unit_of, _, rep = merge_cycles(nodes, units, unit_of, ug)
ug = unit_graph(nodes, g, units, unit_of, ids)
# longest path rank
import sys
sys.setrecursionlimit(100000)
rank = {}
def R(u):
    if u in rank: return rank[u]
    rank[u] = 0
    m = 0
    for v in ug[u]: m = max(m, R(v) + 1)
    rank[u] = m
    return m
for u in ug: R(u)
fam = collections.defaultdict(list)
for u in units:
    s = collections.Counter(seed(nodes[m]) for m in u.members).most_common(1)[0][0]
    fam[s].append(u)
for s, lst in sorted(fam.items(), key=lambda kv: min(rank[u.id] for u in kv[1])):
    rs = [rank[u.id] for u in lst]
    print(f"{s:22s} units={len(lst):3d} lines={sum(u.lines for u in lst):5d} rank {min(rs)}..{max(rs)}")
print('max rank', max(rank.values()))
bylevel = collections.Counter()
for u in units: bylevel[rank[u.id]] += u.lines
print(sorted(bylevel.items()))
