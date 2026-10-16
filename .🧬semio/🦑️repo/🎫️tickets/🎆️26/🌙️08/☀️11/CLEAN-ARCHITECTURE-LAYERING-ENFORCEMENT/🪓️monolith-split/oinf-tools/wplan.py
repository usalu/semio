"""World lane crate plan: band merge -> crates -> item chunks."""
import sys, os, collections, pickle, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lane import *
from wmap import ORDER as CORDER

BAND_ORDER = ['model','⏯️tool-run-trace','📇️registry','comp_a','comp_b','🧲️picking','🔍️marquee','🕹️gumball','📡️assets','🧱️scene','🧹️lifecycle','🖌️overlay','🌉️bridge','🖱️pointer','iauth','🎛️frame','state']
# crate id -> bands (adjacent merge, bottom to top)
CRATES = [
 ('model',      ['model','⏯️tool-run-trace','📇️registry'],        '🗃️model'),
 ('components', ['comp_a','comp_b'],                               '🧩️components'),
 ('cursors',    ['🧲️picking','🔍️marquee','🕹️gumball'],           '🧲️cursors'),
 ('scene',      ['📡️assets','🧱️scene'],                          '🧱️scene'),
 ('bridge',     ['🧹️lifecycle','🖌️overlay','🌉️bridge'],         '🌉️bridge'),
 ('interaction',['🖱️pointer','iauth'],                             '🧵️interaction'),
 ('host',       ['🎛️frame','state'],                               '🎛️host'),
]

def plan(verbose=True):
    tree, mods = load_mods()
    nodes = build(mods)
    g, amb = edges(nodes, 'none')
    ids = set(n.id for n in nodes if n.mod.startswith('🌍️world'))
    TY = {'World' + c.capitalize() + 'State': c for c in CORDER}
    HOSTFN = {'World3dState', 'world3d_dynamic_retirement_terminal_is_empty', 'sync_world3d_state', 'render_world_3d', 'step_world3d_scene_bridge', 'step_component_face_overlay_build', 'append_component_face_translucent_overlays', 'step_world3d_snapshot', 'handle_world3d_pointer_button', 'step_world3d_interaction', 'step_world3d_dynamic_retirement'}
    IAUTH = {'WorldInteractionAuthority', 'WorldInteractionActive', 'world_interaction_active_census'}
    def seed(n):
        nm = n.parent or n.name
        if nm in TY and TY[nm] == 'authority': return 'iauth'
        if nm in TY: return 'comp_a' if TY[nm] in ('session','camera','selection','gumball','interaction','terrain','trace') else 'comp_b'
        if nm == 'World3dState' or n.name in HOSTFN: return 'state'
        if nm in IAUTH: return 'iauth'
        if n.mod == '🌍️world/🧵️interaction/⚙️authority/🦀️.rs': return 'iauth'
        if n.mod == '🌍️world/🦀️.rs': return 'model'
        return short(n.mod)
    res = run_lane(nodes, g, ids, seed, BAND_ORDER, set())
    band_crate = {}
    for ci, (cid, bands, folder) in enumerate(CRATES):
        for b in bands: band_crate[BAND_ORDER.index(b)] = ci
    unit_crate = {u.id: band_crate[res['A'][u.id]] for u in res['units']}
    # item-level assignment
    item_crate = {}   # (rel, idx) -> crate idx
    conflicts = []
    for u in res['units']:
        for m in u.members:
            n = nodes[m]
            key = n.item_ref
            c = unit_crate[u.id]
            if key in item_crate and item_crate[key] != c: conflicts.append((key, item_crate[key], c, n.name))
            item_crate[key] = c
    return dict(mods=mods, nodes=nodes, g=g, res=res, unit_crate=unit_crate, item_crate=item_crate, conflicts=conflicts, ids=ids)

if __name__ == '__main__':
    P = plan()
    print('conflicts (impl blocks split across crates):', len(P['conflicts']))
    for c in P['conflicts'][:20]: print(c)
    sz = collections.Counter()
    for u in P['res']['units']: sz[CRATES[P['unit_crate'][u.id]][0]] += u.lines
    print(dict(sz))
    # coverage
    miss = []
    for rel, m in P['mods'].items():
        if not rel.startswith('🌍️world'): continue
        for idx, it in enumerate(m.items):
            if it.kind in ('use','mod','extern_crate','extern','attr-only'): continue
            if (rel, idx) not in P['item_crate']: miss.append((rel, idx, it.kind, it.name, it.impl_ty))
    print('items without crate:', len(miss)); 
    for x in miss[:30]: print(x)
