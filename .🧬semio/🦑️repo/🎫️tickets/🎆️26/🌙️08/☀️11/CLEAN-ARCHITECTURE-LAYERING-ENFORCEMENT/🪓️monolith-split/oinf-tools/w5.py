from graph2 import *
tree, mods = load_mods()
nodes = build(mods)
for n in nodes:
    if n.parent == 'World3dState':
        print(n.kind, n.name, n.lines, n.mod.split('/')[-2], sorted(n.fields)[:0])
