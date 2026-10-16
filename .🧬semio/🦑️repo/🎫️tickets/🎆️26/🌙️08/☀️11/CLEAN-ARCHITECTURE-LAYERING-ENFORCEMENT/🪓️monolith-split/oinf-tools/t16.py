from planeval import *
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
A0 = {n.id: pos[seed(n)] for n in nodes if n.id in ids}
A, moves = repair(nodes, g, A0, ids)
moved = {}
for x, a, b in moves: moved.setdefault(x, a)
mv = [i for i in moved if A[i] != A0[i]]
print('moved nodes', len(mv), sum(nodes[i].lines for i in mv))
sizes = collections.Counter()
for i in ids: sizes[A[i]] += nodes[i].lines
for r in sorted(sizes): print(r, order[r], sizes[r])
ext = ext_methods(nodes, A, ids)
print('ext', len(ext), sum(n.lines for n in ext), collections.Counter(n.parent for n in ext).most_common(20))
byseed = collections.Counter((order[A0[i]], order[A[i]]) for i in mv)
for k, v in byseed.most_common(30): print(k, v)
