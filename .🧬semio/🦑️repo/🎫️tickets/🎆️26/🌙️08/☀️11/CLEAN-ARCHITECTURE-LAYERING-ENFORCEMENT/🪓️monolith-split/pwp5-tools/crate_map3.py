"""🗺️ The WP3 crate map on the staged tree: every production and test source file of the gate-counter closures, assigned to its crate by directory (lift files belong to their destination crate). Writes crate-map3.json. usage: CLOSURE=<closure.json> python3 crate_map3.py"""
import json, os, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import graph3 as g
P = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
CL = g.pt.CLOSURE
WP3 = ['child', 'composition-emission', 'composition-publication', 'publication-mounted', 'document-media', 'job', 'command', 'peer', 'retained-command', 'extension']
OTHER_LIFT_DEST = ['decl-emission', 'decl-registry', 'plugin-program', 'time-travel-panel', 'decl-app']
def crate_of(rel):
    return g.pt.file_crate(rel)
def collect(rows):
    out = collections.defaultdict(set)
    for r in rows:
        for x in CL[r]:
            p = x['path']
            if not p.startswith(P) or not p.endswith('.rs') or x.get('kind') == 'embedded': continue
            out[crate_of(p[len(P):])].add(p)
    return out
lib = collect(['lib native', 'lib wasm32-wasip2', 'lib native testing-features'])
test = collect(['test native', 'test wasm32-wasip2', 'test native all-features'])
res = {}
for c in WP3 + OTHER_LIFT_DEST:
    res[c] = dict(lib=sorted(lib.get(c, [])), test=sorted(test.get(c, [])))
json.dump(res, open(HERE + '/../🗑️generated/pwp5-crate-map.json', 'w'), ensure_ascii=False, indent=1)
for c in res: print(c, 'lib files', len(res[c]['lib']), 'test files', len(res[c]['test']))
