import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import graph2, bedges
from graph2 import scc
tree, mods = graph2.load_mods()
nodes = graph2.build(mods)
g, amb = bedges.edges2(nodes, 'none')
ids = set(n.id for n in nodes if n.mod.startswith('🎲️board'))
sub = {i: {j for j in g[i] if j in ids} for i in ids}
comps = [c for c in scc(sub) if len(c) > 1]
comps.sort(key=lambda c: -sum(nodes[i].lines for i in c))
print('SCCs', [(len(c), sum(nodes[i].lines for i in c)) for c in comps[:12]])
target = sys.argv[1]
for c in comps:
    names = {nodes[i].name for i in c}
    if target in names:
        cs = set(c)
        print('members', len(c))
        for i in sorted(c, key=lambda i: -nodes[i].lines)[:40]:
            n = nodes[i]
            outs = [nodes[j] for j in sub[i] if j in cs]
            print(f"  {n.kind:8s} {(n.parent+'::' if n.parent else '')+n.name:50s} {n.lines:5d} -> {[(o.parent+'::' if o.parent else '')+o.name for o in outs][:8]}")
        break
