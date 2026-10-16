import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bplan
from lane import *
import graph2
tree, mods = graph2.load_mods()
nodes = graph2.build(mods)
g, amb = graph2.edges(nodes, sys.argv[1] if len(sys.argv) > 1 else 'none')
ids = set(n.id for n in nodes if n.mod.startswith('🎲️board'))
srank = {i: 0 for i in ids}
detached, units, unit_of, ug = detach_loop2(nodes, g, ids, srank, set())
units, unit_of, _, rep = merge_cycles(nodes, units, unit_of, ug)
ug = unit_graph(nodes, g, units, unit_of, ids)
print('units', len(units), 'cycles merged', len(rep), 'detached', len(detached))
for lines, names in sorted(rep, reverse=True)[:8]: print('  merged', lines, names[:5])
level = {}
import functools
sys.setrecursionlimit(100000)
def lv(u):
    if u in level: return level[u]
    level[u] = 0
    level[u] = 1 + max([lv(v) for v in ug[u]] or [-1])
    return level[u]
for u in ug: lv(u)
by = collections.defaultdict(list)
for u in units: by[level[u.id]].append(u)
for l in sorted(by):
    us = by[l]; tot = sum(u.lines for u in us)
    mc = collections.Counter(bplan.band_of(u.mod) for u in us)
    big = sorted(us, key=lambda u: -u.lines)[:3]
    print(f"L{l:2d} units={len(us):4d} lines={tot:6d} {dict(mc.most_common(4))} big={[(u.name, u.lines) for u in big]}")
