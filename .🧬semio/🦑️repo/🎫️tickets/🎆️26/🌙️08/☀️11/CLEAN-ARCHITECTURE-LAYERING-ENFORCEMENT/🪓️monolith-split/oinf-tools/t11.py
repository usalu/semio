from graph2 import *
tree, mods = load_mods()
nodes = build(mods)
g, amb = edges(nodes, 'none')
def lane(n):
    return 'W' if n.mod.startswith('🌍️world') else ('B' if n.mod.startswith('🎲️board') else 'X')
cross = collections.defaultdict(list)
for n in nodes:
    for d in g[n.id]:
        a, b = lane(n), lane(nodes[d])
        if a != b: cross[(a,b)].append((n, nodes[d]))
for k, v in cross.items():
    print(k, len(v))
    for n, d in v[:60]:
        print('   ', n.mod.split('/')[-2], f"{n.parent+'::' if n.parent else ''}{n.name}", '->', d.mod.split('/')[-2], f"{d.parent+'::' if d.parent else ''}{d.name}")
