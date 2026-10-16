from lane import *
from wmap import ORDER as CORDER
import random, pickle
tree, mods = load_mods()
nodes = build(mods)
g, amb = edges(nodes, 'none')
def lane_of(n): return 'W' if n.mod.startswith('🌍️world') else ('B' if n.mod.startswith('🎲️board') else 'X')
ids = set(n.id for n in nodes if lane_of(n) == 'W')
TY = {'World' + c.capitalize() + 'State': c for c in CORDER}
HOSTFN = {'World3dState', 'world3d_dynamic_retirement_terminal_is_empty', 'sync_world3d_state', 'render_world_3d', 'step_world3d_scene_bridge', 'step_component_face_overlay_build', 'append_component_face_translucent_overlays', 'step_world3d_snapshot', 'handle_world3d_pointer_button', 'step_world3d_interaction', 'step_world3d_dynamic_retirement'}
def seed(n):
    nm = n.parent or n.name
    if nm in TY: return 'c:' + TY[nm]
    if nm == 'World3dState' or n.name in HOSTFN: return 'state'
    return short(n.mod) if n.mod != '🌍️world/🦀️.rs' else 'model'
bands = sorted(set(seed(nodes[i]) for i in ids))
print(bands)
# linear ordering over bands from unit graph weights
bseed = {i: seed(nodes[i]) for i in ids}
W = collections.defaultdict(lambda: collections.defaultdict(int))
for i in ids:
    for d in g[i]:
        if d in ids and bseed[d] != bseed[i]: W[bseed[i]][bseed[d]] += 1
def cost(order):
    p = {m:i for i,m in enumerate(order)}
    return sum(w for a in W for b,w in W[a].items() if p[b] > p[a])
indeg = collections.Counter()
for a in W:
    for b, w in W[a].items(): indeg[b] += w
order = sorted(bands, key=lambda m: sum(W[m].values()) - indeg.get(m, 0))
best = cost(order); improved = True
while improved:
    improved = False
    for i in range(len(order)):
        m = order[i]; rest = order[:i] + order[i+1:]; bj, bc = i, best
        for j in range(len(rest) + 1):
            c = cost(rest[:j] + [m] + rest[j:])
            if c < bc: bc, bj = c, j
        if bc < best: order = rest[:bj] + [m] + rest[bj:]; best = bc; improved = True
print('order cost', best); print(order)
pickle.dump(order, open('/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf/world-order.pkl', 'wb'))
res = run_lane(nodes, g, ids, seed, order, set())
print('units', len(res['units']), 'moved', len(res['moved']), sum(res['unodes'][x].lines for x in res['moved']), 'ext', len(res['ext']))
for r in sorted(res['sizes']): print(r, order[r], res['sizes'][r])
A = res['A']; useed = res['useed']
c = collections.Counter(); 
for x in res['moved']:
    c[(order[useed[x]], order[A[x]])] += res['unodes'][x].lines
for k, v in c.most_common(25): print(k, v)
