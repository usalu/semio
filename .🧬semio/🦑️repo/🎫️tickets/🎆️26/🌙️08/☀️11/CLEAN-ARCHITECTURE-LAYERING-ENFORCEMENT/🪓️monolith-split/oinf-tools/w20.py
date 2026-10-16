from lane import *
from wmap import ORDER as CORDER
import pickle, sys
tree, mods = load_mods()
nodes = build(mods)
g, amb = edges(nodes, 'none')
def lane_of(n): return 'W' if n.mod.startswith('🌍️world') else ('B' if n.mod.startswith('🎲️board') else 'X')
ids = set(n.id for n in nodes if lane_of(n) == 'W')
TY = {'World' + c.capitalize() + 'State': c for c in CORDER}
HOSTFN = {'World3dState', 'world3d_dynamic_retirement_terminal_is_empty', 'sync_world3d_state', 'render_world_3d', 'step_world3d_scene_bridge', 'step_component_face_overlay_build', 'append_component_face_translucent_overlays', 'step_world3d_snapshot', 'handle_world3d_pointer_button', 'step_world3d_interaction', 'step_world3d_dynamic_retirement'}
IAUTH = {'WorldInteractionAuthority', 'WorldInteractionActive', 'world_interaction_active_census'}
def seed(n):
    nm = n.parent or n.name
    if nm in TY: return 'comp'
    if nm == 'World3dState' or n.name in HOSTFN: return 'state'
    if nm in IAUTH: return 'iauth'
    if n.mod == '🌍️world/🧵️interaction/⚙️authority/🦀️.rs': return 'iauth'
    if n.mod == '🌍️world/🦀️.rs': return 'model'
    return short(n.mod)
ORDER = ['model','⏯️tool-run-trace','📇️registry','🧲️picking','🔍️marquee','🕹️gumball','comp','📡️assets','🧱️scene','🧹️lifecycle','🖌️overlay','🌉️bridge','🖱️pointer','iauth','🎛️frame','state']
if len(sys.argv) > 1:
    ORDER = sys.argv[1].split(',')
res = run_lane(nodes, g, ids, seed, ORDER, set())
print('units', len(res['units']), 'moved', len(res['moved']), sum(res['unodes'][x].lines for x in res['moved']))
for r in sorted(res['sizes']): print(r, ORDER[r], res['sizes'][r])
pickle.dump(dict(A=res['A'], useed=res['useed']), open('/tmp/w20.pkl', 'wb'))
