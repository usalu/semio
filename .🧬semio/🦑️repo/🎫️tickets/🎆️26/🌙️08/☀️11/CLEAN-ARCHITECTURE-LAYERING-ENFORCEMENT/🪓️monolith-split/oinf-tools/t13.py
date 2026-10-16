from settle import *
tree, mods = load_mods()
nodes = build(mods)
def lane(n): return 'W' if n.mod.startswith('🌍️world') else ('B' if n.mod.startswith('🎲️board') else 'X')
for L in 'WB':
    c = collections.Counter(); k = collections.Counter()
    for n in nodes:
        if lane(n) == L:
            c[n.kind] += n.lines; k[n.kind] += 1
    print(L, dict(c), dict(k))
