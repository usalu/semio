from lane import *
tree, mods = load_mods()
nodes = build(mods)
g, amb = edges(nodes, 'none')
def lane_of(n): return 'W' if n.mod.startswith('🌍️world') else ('B' if n.mod.startswith('🎲️board') else 'X')
ids = set(n.id for n in nodes if lane_of(n) == 'W')
units, unit_of = build_units2(nodes, ids, set(), set())
ug = unit_graph(nodes, g, units, unit_of, ids)
comps = [c for c in scc(ug) if len(c) > 1]
comps.sort(key=lambda c: -sum(units[x].lines for x in c))
print(len(units), 'units;', len(comps), 'cyclic SCCs')
for c in comps[:12]:
    print(sum(units[x].lines for x in c), 'lines', len(c), 'units:', [units[x].name for x in c][:14])
big = comps[0]; S = set(big)
for x in sorted(big, key=lambda x: units[x].name):
    u = units[x]
    outs = sorted(units[y].name for y in ug[x] if y in S)
    print(f"{u.lines:5d} {u.kind:6s} {u.name:50s} -> {', '.join(outs)[:170]}")
