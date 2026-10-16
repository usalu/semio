#!/usr/bin/env python3
"""🔭️ Resolves every consumer-named path through the PRE-carve facade (text kept in the persisted state) to separate carve regressions from pre-existing consumer mismatches."""
import sys, os, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, ws as wsm, plan_host, state
from carve import Model, def_of

ws0, model0, roots0 = state.load()
base_ws = state.base_ws(ws0)
hcmod.FS = base_ws
base = Model(base_ws, {'semio_framework_plugin_host': plan_host.P('📦️packages/🦀️rust/🦀️.rs')}, allow_dup=True)
root = base.roots['semio_framework_plugin_host']
hcmod.CRATES.clear()
hcmod.CRATES.update(base.roots)
d = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), '../🗑️generated/phc-consumers.json')))
bad = 0
for full in sorted(d['full']):
    r = hcmod.resolve(root, full.split('::'), None, False, exact=False)
    if r is None:
        bad += 1
        print('UNRESOLVED in the monolith', full)
print('paths', len(d['full']), 'unresolved in the monolith', bad)
