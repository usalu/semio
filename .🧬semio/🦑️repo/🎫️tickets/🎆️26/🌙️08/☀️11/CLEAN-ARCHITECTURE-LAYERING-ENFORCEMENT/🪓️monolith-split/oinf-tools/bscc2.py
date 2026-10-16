import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bplan2
from blane import *
import graph2, bedges
P = bplan2.plan('none')
res = P['res']; nodes = P['nodes']; g = P['g']; ids = P['ids']
# rebuild units as in run_lane before merge
pos = {b: i for i, b in enumerate(bplan2.ORDER)}
srank = res['srank']
detached, units, unit_of, ug = detach_loop2(nodes, g, ids, srank, set())
comps = [c for c in scc(ug) if len(c) > 1]
comps.sort(key=lambda c: -sum(units[u].lines for u in c))
print('cyclic SCCs after detach', [(len(c), sum(units[u].lines for u in c)) for c in comps])
want = sys.argv[1] if len(sys.argv) > 1 else None
for c in comps[:int(sys.argv[2]) if len(sys.argv) > 2 else 3]:
    cs = set(c)
    print('=== SCC', len(c), sum(units[u].lines for u in c))
    for u in sorted(c, key=lambda u: -units[u].lines):
        U = units[u]
        outs = []
        for m in U.members:
            for d in g[m]:
                if d in ids and unit_of[d] in cs and unit_of[d] != u:
                    outs.append((nodes[m].name, nodes[d].name, units[unit_of[d]].name[:25]))
        print(f"  {U.name[:50]:50s} {U.lines:5d} members={len(U.members)} edges={outs[:6]}")
