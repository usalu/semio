from planeval import *
from units import *
tree, mods = load_mods()
nodes = build(mods)
g, amb = edges(nodes, 'none')
def lane(n): return 'W' if n.mod.startswith('🌍️world') else ('B' if n.mod.startswith('🎲️board') else 'X')
ids = set(n.id for n in nodes if lane(n) == 'W')
STATE = {'World3dState'}
def seed(n):
    s = short(n.mod)
    if s == '🌍️world':
        if n.name in STATE or n.parent in STATE or n.name in ('set_world3d_declared_actions','world3d_declares_action','world3d_offers_transform_gumball','world_test_mesh_query'): return 'state'
        return 'model'
    return {'⏯️tool-run-trace':'trace','📇️registry':'registry','🔍️marquee':'marquee','🧲️picking':'picking','🕹️gumball':'gumball','📡️assets':'assets','🧱️scene':'scene','🧹️lifecycle':'lifecycle','🌉️bridge':'bridge','🖌️overlay':'overlay','🖱️pointer':'pointer','⚙️authority':'authority','🎛️frame':'frame'}[s]
order = ['model','trace','registry','marquee','picking','gumball','state','assets','scene','lifecycle','bridge','overlay','pointer','authority','frame']
pos = {b:i for i,b in enumerate(order)}
srank = {n.id: pos[seed(n)] for n in nodes if n.id in ids}
detached, units, unit_of, ug = detach_loop(nodes, g, ids, srank, verbose=False)
units, unit_of, _, rep = merge_cycles(nodes, units, unit_of, ug)
print('merged cycle units:', rep)
ug = unit_graph(nodes, g, units, unit_of, ids)
assert not [c for c in scc(ug) if len(c) > 1]
# unit-level repair
class UN:  # adapter: nodes-like for repair
    pass
unodes = []
for u in units:
    x = UN(); x.lines = u.lines; unodes.append(x)
useed = {}
for u in units:
    rs = [srank[m] for m in u.members]
    tm = [m for m in u.members if nodes[m].kind in ('struct','enum','union','type','trait')]
    useed[u.id] = srank[(tm or u.members)[0]]
uids = set(u.id for u in units)
A, moves = repair(unodes, ug, useed, uids)
mv = {x for x, a, b in moves if A[x] != useed[x]}
print('units', len(units), 'moved units', len(mv), sum(unodes[x].lines for x in mv))
sizes = collections.Counter()
for u in units: sizes[A[u.id]] += u.lines
for r in sorted(sizes): print(r, order[r], sizes[r])
# extension methods: detached methods whose band > type band
tyband = {}
for u in units:
    for m in u.members:
        if nodes[m].kind in ('struct','enum','union','type','trait'): tyband[nodes[m].name] = A[u.id]
ext = [nodes[m] for m in detached if A[unit_of[m]] > tyband.get(nodes[m].parent, -1)]
print('extension methods', len(ext), sum(n.lines for n in ext))
byseed = collections.Counter((order[useed[x]], order[A[x]]) for x in mv)
for k, v in byseed.most_common(25): print(k, v)
