from model import *
from modtree import walk, LIBROOT, INFROOT
tree = walk(LIBROOT)
files = [e['file'] for e in tree if e['file'] and e['file'].startswith(INFROOT) and '🧪️tests' not in e['file']]
mods = {}
for f in files:
    rel = f[len(INFROOT):]
    text = open(f, encoding='utf-8').read()
    items, _ = chunk_file(text)
    m = Mod(); m.rel = rel; m.text = text; m.items = items; m.short = rel.replace('/🦀️.rs','')
    mods[rel] = m
nodes, typenode, order = build_nodes(mods)
# exclude non-def nodes
g = item_graph(nodes)
comps = scc(g)
comps.sort(key=lambda c: -sum(nodes[x]['lines'] for x in c))
print('nodes', len(nodes), 'sccs', len(comps))
for c in comps[:12]:
    ln = sum(nodes[x]['lines'] for x in c)
    mc = collections.Counter(nodes[x]['mod'].split('/')[-2] if '/' in nodes[x]['mod'] else nodes[x]['mod'] for x in c)
    print(len(c), 'items', ln, 'lines', dict(mc.most_common(6)))
