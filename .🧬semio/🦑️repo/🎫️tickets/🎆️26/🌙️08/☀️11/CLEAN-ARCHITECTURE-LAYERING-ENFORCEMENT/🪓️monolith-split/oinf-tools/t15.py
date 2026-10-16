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
res = evaluate(nodes, g, ids, seed, order)
report(res, nodes, ids)
# hill climb on order
best = res['score']; bo = order[:]
random.seed(3)
for rnd in range(400):
    o = bo[:]
    i = random.randrange(len(o)); b = o.pop(i); j = random.randrange(len(o)+1); o.insert(j, b)
    r = evaluate(nodes, g, ids, seed, o)
    if r['score'] < best:
        best = r['score']; bo = o; print('improved', best, o)
res = evaluate(nodes, g, ids, seed, bo)
report(res, nodes, ids)
