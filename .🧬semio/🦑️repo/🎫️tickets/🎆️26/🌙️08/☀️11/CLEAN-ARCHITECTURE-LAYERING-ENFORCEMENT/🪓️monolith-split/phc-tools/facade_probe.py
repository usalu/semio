import sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm, plan_host
from carve import Model, def_of
ws = wsm.WS()
hcmod.FS = ws
base = Model(ws, {'semio_framework_plugin_host': plan_host.P('📦️packages/🦀️rust/🦀️.rs')}, allow_dup=True)
root = base.roots['semio_framework_plugin_host']
d = json.load(open('../🗑️generated/phc-consumers.json'))
hcmod.CRATES.clear(); hcmod.CRATES.update(base.roots)
H = plan_host.H
out = {}
for full in sorted(d['full']):
    segs = full.split('::')
    r = hcmod.resolve(root, segs, None, False, exact=False)
    if r is None:
        print('UNRESOLVED', full)
        continue
    if r.kind == 'external':
        print('EXT', full, '->', r.ext, r.rest)
    else:
        dd = def_of(r)
        print(r.kind, full, '->', (dd.file.replace(H, '') if dd else None), dd.name if dd else None)
