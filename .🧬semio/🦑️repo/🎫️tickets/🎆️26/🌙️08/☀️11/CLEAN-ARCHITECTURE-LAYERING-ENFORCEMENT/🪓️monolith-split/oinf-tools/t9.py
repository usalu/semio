from graph2 import *
tree, mods = load_mods()
nodes = build(mods)
print(len(nodes), 'nodes', sum(n.lines for n in nodes), 'lines')
for amode in ('none', 'all'):
    g, amb = edges(nodes, amode)
    comps = scc({n.id: g[n.id] for n in nodes})
    comps.sort(key=lambda c: -sum(nodes[x].lines for x in c))
    print('ambiguous=', amode, 'sccs', len(comps), 'ambig names/nodes', sum(len(v) for v in amb.values()))
    for c in comps[:6]:
        ln = sum(nodes[x].lines for x in c)
        mc = collections.Counter(nodes[x].mod.split('/')[-2] for x in c)
        print('  ', len(c), 'nodes', ln, 'lines', dict(mc.most_common(8)))
