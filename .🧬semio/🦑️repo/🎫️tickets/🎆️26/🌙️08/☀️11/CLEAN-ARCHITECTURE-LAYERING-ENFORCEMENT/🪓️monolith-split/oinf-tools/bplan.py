"""Board lane planning: module/type seeds -> units -> repair -> crates."""
import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lane import *

N = '🎲️board/🔌️ports/➡️directed/➕️normal/'
D = '🎲️board/🔌️ports/➡️directed/🕸️dag/'
def band_of(mod):
    m = mod
    if m.startswith('🎲️board/🧬️schema/💡️inferences'): return 'infer'
    if m.startswith('🎲️board/🧬️schema'): return 'schema'
    if m.startswith('🎲️board/🚪️io'): return 'iotext'
    if m == '🎲️board/🦀️.rs' or m.startswith('🎲️board/♻️retirement'): return 'engine'
    if m.startswith('🎲️board/➕️normal'): return 'ports'
    if m.startswith('🎲️board/🔌️ports/') and '➡️directed/➕️normal' not in m and '➡️directed/🕸️dag' not in m: return 'ports'
    if m == N + '🦀️.rs': return 'n-types'
    if m == N + '🎨️style/🦀️.rs': return 'n-style'
    if m == N + '🪣️fill-job/🦀️.rs': return 'n-fill'
    if m == N + '🎛️selection-events/🦀️.rs': return 'n-events'
    if m == N + '🧹️scene-delete-links/🦀️.rs': return 'n-scene'
    if m == N + '🖱️pointer-gestures/🦀️.rs': return 'n-pointer'
    if m == N + '🧭️host/🦀️.rs': return 'n-host'
    if m == D + '🦀️.rs': return 'd-root'
    if m.startswith(D + '🎛️input') or m.startswith(D + '🧹️retirement'): return 'd-input'
    if m.startswith(D + '🧭️host-selection'): return 'd-sel'
    if m.startswith(D + '🧱️frame-minimap-align'): return 'd-frame'
    if m.startswith(D + '🛠️mutate'): return 'd-mutate'
    if m.startswith(D + '🖌️paint'): return 'd-paint'
    if m.startswith(D + '🌐️wasm-session'): return 'd-wasm'
    raise SystemExit('no band for ' + m)

BAND_ORDER = ['schema', 'infer', 'iotext', 'engine', 'ports', 'n-types', 'n-style', 'n-fill', 'n-events', 'n-scene', 'n-pointer', 'n-host', 'd-root', 'd-input', 'd-sel', 'd-frame', 'd-mutate', 'd-paint', 'd-wasm']

def plan(ambiguous='none', bias=1.0):
    import graph2
    tree, mods = graph2.load_mods()
    nodes = graph2.build(mods)
    g, amb = graph2.edges(nodes, ambiguous)
    ids = set(n.id for n in nodes if n.mod.startswith('🎲️board'))
    def seed(n): return band_of(n.mod)
    res = run_lane(nodes, g, ids, seed, BAND_ORDER, set(), bias=bias)
    return dict(mods=mods, nodes=nodes, g=g, amb=amb, res=res, ids=ids)

if __name__ == '__main__':
    P = plan(sys.argv[1] if len(sys.argv) > 1 else 'none')
    res = P['res']; nodes = P['nodes']
    print('units', len(res['units']), 'merged cycles', len(res['merged']), 'detached', len(res['detached']), 'moved', len(res['moved']))
    sizes = res['sizes']
    for b, name in enumerate(BAND_ORDER): print(f"{name:10s} {sizes.get(b, 0)}")
    print('ext methods', len(res['ext']), sum(n.lines for n in res['ext']))
    for lines, names in sorted(res['merged'], reverse=True)[:10]: print('merged', lines, names[:6])
