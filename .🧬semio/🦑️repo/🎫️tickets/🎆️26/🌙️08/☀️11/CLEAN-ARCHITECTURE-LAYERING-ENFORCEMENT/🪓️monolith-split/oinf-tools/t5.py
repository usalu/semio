from model import *
mods = load()
nodes, typenode, order = build_nodes(mods)
print(len(nodes), 'nodes', sum(n['lines'] for n in nodes.values()), 'lines')
kinds = collections.Counter(n['kind'] for n in nodes.values())
print(kinds)
ext = [(k, n['lines']) for k, n in nodes.items() if n['kind']=='impl-ext']
print(len(ext), 'external impls', sum(x[1] for x in ext))
for k, l in sorted(ext, key=lambda x: -x[1])[:15]: print(k, l)
mc = [(k, n['lines']) for k, n in nodes.items() if n['kind']=='macro_call']
print(len(mc), 'macro calls')
for k,l in mc[:10]: print(k)
