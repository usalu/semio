"""Board lane planning with host-coupled units pinned to the component chain."""
import sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from blane import *
import bplan
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
CHAIN = {
 'BoardHost': ['scene', 'style', 'focus', 'outbox', 'brush', 'gestures', 'commit', 'closing'],
 'DagHost': ['graph', 'viewport', 'pointer', 'paint', 'minimap', 'notes'],
}
PFX = {'BoardHost': 'b', 'DagHost': 'd'}
LOW = ['schema', 'infer', 'iotext', 'engine', 'ports']
NUNP = ['n-types', 'n-style', 'n-fill', 'n-events', 'n-scene', 'n-pointer']
DUNP = ['dm-root', 'dm-input', 'dm-sel', 'dm-frame', 'dm-mutate', 'dm-paint', 'dm-wasm']
ORDER = LOW + NUNP + ['b-' + c for c in CHAIN['BoardHost']] + ['b-fill', 'b-host'] + DUNP + ['d-' + c for c in CHAIN['DagHost']] + ['d-host']

def load_meta():
    meta = {}
    for w in ('board', 'dag'):
        for m in json.load(open(f'{SC}/b-meta-{w}.json', encoding='utf-8')):
            meta[(m['rel'], m['name'], m['parent'])] = m
    return meta

def plan(ambiguous='none', bias=1.0):
    import graph2
    tree, mods = graph2.load_mods()
    nodes = graph2.build(mods)
    import bedges
    g, amb = bedges.edges2(nodes, ambiguous)
    ids = set(n.id for n in nodes if n.mod.startswith('🎲️board'))
    meta = load_meta()
    comp_of = {}
    for host, chain in CHAIN.items():
        pre = host.replace('Host', '')
        for c in chain: comp_of[pre + c.capitalize() + 'State'] = (host, c)
    pins = {}
    for n in nodes:
        if n.id not in ids: continue
        key = (n.mod, n.name, n.parent if n.kind == 'method' else None)
        if n.kind in ('struct',) and n.name in comp_of:
            host, c = comp_of[n.name]; pins[n.id] = PFX[host] + '-' + c
        elif n.kind == 'struct' and n.name in ('BoardHost', 'DagHost'):
            pins[n.id] = PFX[n.name] + '-host'
        elif n.kind == 'method' and n.parent in comp_of:
            host, c = comp_of[n.parent]; pins[n.id] = PFX[host] + '-' + c
        elif n.kind in ('method', 'traitimpl') and n.parent in ('BoardHost', 'DagHost'):
            pins[n.id] = PFX[n.parent] + '-host'
        elif n.kind == 'fn':
            m = meta.get((n.mod, n.name, None)) or meta.get((n.mod, n.name, 'None'))
            for (r, nm, par), mm in meta.items():
                pass
    # free/narrow fns from decomposition
    byfn = {}
    for (rel, name, parent), m in meta.items():
        byfn.setdefault((rel, name), []).append(m)
    for n in nodes:
        if n.id not in ids or n.kind != 'fn' or n.parent is not None: continue
        ms = [m for m in byfn.get((n.mod, n.name), []) if m['mode'] in ('narrow', 'free', 'owner') and m['kind'] in ('method', 'pfn') and m['parent'] != 'None']
        ms = [m for m in byfn.get((n.mod, n.name), []) if m['mode'] == 'narrow']
        if ms:
            m = ms[0]; host = m['host']
            hi = max(m['comps'], key=lambda c: CHAIN[host].index(c))
            pins[n.id] = PFX[host] + '-' + hi
    # types with host-param methods live at the highest component they take
    pfn_type = {}
    for (rel, name, parent), m in meta.items():
        if m['kind'] == 'pfn' and m['mode'] == 'narrow' and m['parent'] not in (None, 'None') and m['comps']:
            host = m['host']
            hi = max(m['comps'], key=lambda c: CHAIN[host].index(c))
            cur = pfn_type.get((rel, m['parent']))
            if cur is None or CHAIN[host].index(hi) > CHAIN[host].index(cur[1]): pfn_type[(rel, m['parent'])] = (host, hi)
    def is_fill(n):
        nm = (n.parent or n.name) or ''
        return nm.startswith('BoardFill') or nm.startswith('BOARD_FILL') or nm.startswith('board_fill') or n.mod.endswith('🪣️fill-job/🦀️.rs')
    for n in nodes:
        if n.id not in ids or n.id in pins: continue
        key = (n.mod, n.name if n.kind == 'struct' else n.parent)
        if key in pfn_type and (n.kind == 'struct' or n.kind in ('method', 'traitimpl')):
            host, hi = pfn_type[key]
            pins[n.id] = 'b-fill' if is_fill(n) else PFX[host] + '-' + hi
    def seed(n):
        if n.id in pins: return pins[n.id]
        if is_fill(n): return 'b-fill'
        b = bplan.band_of(n.mod)
        return {'n-host': 'b-host'}.get(b, b.replace('d-', 'dm-') if b.startswith('d-') else b)
    res = run_lane(nodes, g, ids, seed, ORDER, set(), bias=bias, pins=pins)
    import modtree
    return dict(mods=mods, nodes=nodes, g=g, amb=amb, res=res, ids=ids, pins=pins, tree=tree, abs={rel: modtree.INFROOT + rel for rel in mods})

if __name__ == '__main__':
    P = plan(sys.argv[1] if len(sys.argv) > 1 else 'none')
    res = P['res']; nodes = P['nodes']
    print('units', len(res['units']), 'merged cycles', len(res['merged']), 'detached', len(res['detached']), 'moved', len(res['moved']), 'pinned', len(res['pinned']), 'pinned nodes', len(P['pins']))
    sizes = res['sizes']
    for b, name in enumerate(ORDER): print(f"{name:12s} {sizes.get(b, 0)}")
    print('ext methods', len(res['ext']), sum(n.lines for n in res['ext']))
    for lines, names in sorted(res['merged'], reverse=True)[:10]: print('merged', lines, names[:6])

    if len(sys.argv) > 2:
        U = res['units']
        for b, name in enumerate(ORDER):
            us = sorted([u for u in U if res['A'][u.id] == b], key=lambda u: -u.lines)
            if not us: continue
            print(f"== {name} {sum(u.lines for u in us)} lines {len(us)} units")
            for u in us[:int(sys.argv[2])]:
                print(f"   {u.lines:5d} {u.kind:8s} {u.name[:70]:70s} {u.mod.replace('🎲️board/', '')[:50]}")


def assignment(P):
    """item_ref -> crate idx, with conflicts (impl blocks split across crates)."""
    import bcrates
    res = P['res']; nodes = P['nodes']
    band_crate = {}
    for ci, (cid, bands, folder) in enumerate(bcrates.CRATES):
        for b in bands: band_crate[ORDER.index(b)] = ci
    unit_crate = {u.id: band_crate[res['A'][u.id]] for u in res['units']}
    item_crate = {}; conflicts = []
    for u in res['units']:
        for m in u.members:
            n = nodes[m]; key = n.item_ref; c = unit_crate[u.id]
            if key in item_crate and item_crate[key] != c: conflicts.append((key, item_crate[key], c, n.name))
            item_crate[key] = c
    return unit_crate, item_crate, conflicts
