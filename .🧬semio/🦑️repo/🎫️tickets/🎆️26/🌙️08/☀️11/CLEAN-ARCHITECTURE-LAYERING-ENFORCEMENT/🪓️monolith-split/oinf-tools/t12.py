from settle import *
import sys
tree, mods = load_mods()
nodes = build(mods)
g, amb = edges(nodes, 'none')
def lane(n): return 'W' if n.mod.startswith('🌍️world') else ('B' if n.mod.startswith('🎲️board') else 'X')
ids = set(n.id for n in nodes if lane(n) == 'W')
order = ['🌍️world/🦀️.rs','🌍️world/⏯️tool-run-trace/🦀️.rs','🌍️world/🧵️interaction/📇️registry/🦀️.rs','🌍️world/📡️assets/🦀️.rs','🌍️world/🧱️scene/🦀️.rs','🌍️world/🌉️bridge/🦀️.rs','🌍️world/🧹️lifecycle/🦀️.rs','🌍️world/🖌️overlay/🦀️.rs','🌍️world/🖱️pointer/🦀️.rs','🌍️world/🧵️interaction/🧲️picking/🦀️.rs','🌍️world/🧵️interaction/🕹️gumball/🦀️.rs','🌍️world/🎛️frame/🦀️.rs','🌍️world/🧵️interaction/⚙️authority/🦀️.rs','🌍️world/🧵️interaction/🔍️marquee/🦀️.rs']
pos = {m:i for i,m in enumerate(order)}
A0 = {n.id: pos.get(n.mod, 0) for n in nodes}
A, moved = settle(nodes, g, A0, ids)
print('moved', len(moved), sum(nodes[i].lines for i in moved), collections.Counter(nodes[i].kind for i in moved))
sizes = collections.Counter()
for i in ids: sizes[A[i]] += nodes[i].lines
for r in sorted(sizes): print(r, order[r].split('/')[-2], sizes[r])
ext = ext_methods(nodes, A, ids)
print('extension methods', len(ext), sum(n.lines for n in ext))
print(collections.Counter(n.parent for n in ext).most_common(25))

print('--- top lifted nodes into 13 (original rank < 13)')
lifted = [nodes[i] for i in moved if A[i] == 13 and A0[i] < 13]
lifted.sort(key=lambda n: -n.lines)
def why(i):
    for d in g[i]:
        if d in ids and A[d] == A[i] and A0[d] == A[i]: return d
    for d in g[i]:
        if d in ids and A[d] == A[i]: return d
for n in lifted[:40]:
    w = why(n.id)
    print(f"{n.mod.split('/')[-2]:12s} {n.kind:7s} {(n.parent+'::' if n.parent else '')+str(n.name):44s} {n.lines:4d} A0={A0[n.id]} because {nodes[w].mod.split('/')[-2] if w is not None else None} {nodes[w].parent+'::' if w is not None and nodes[w].parent else ''}{nodes[w].name if w is not None else ''}")

print('--- sublevels within band 13')
band = [i for i in ids if A[i] == 13]
lvl = {}
sys.setrecursionlimit(100000)
def level(i):
    if i in lvl: return lvl[i]
    lvl[i] = 0
    m = 0
    for d in g[i]:
        if d in set(band) and d != i:
            m = max(m, level(d) + 1)
    lvl[i] = m
    return m
bs = set(band)
for i in band: 
    try: level(i)
    except RecursionError: pass
by = collections.defaultdict(lambda: collections.Counter())
for i in band: by[lvl[i]][nodes[i].mod.split('/')[-2]] += nodes[i].lines
for l in sorted(by): print(l, sum(by[l].values()), dict(by[l].most_common(6)))
