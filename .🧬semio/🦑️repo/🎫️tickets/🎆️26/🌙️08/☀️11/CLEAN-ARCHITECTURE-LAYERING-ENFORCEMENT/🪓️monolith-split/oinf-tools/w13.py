from lane import *
tree, mods = load_mods()
print([m for m in mods][:3], list(mods.values())[0].path[:90])
nodes = build(mods)
g, amb = edges(nodes, 'none')
n = [x for x in nodes if x.name == 'object_world_position'][0]
print(n.mod, sorted(n.plain)[:40])
print([ (nodes[d].kind, nodes[d].name) for d in g[n.id]])
