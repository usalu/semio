import json, os, re, sys, collections
sys.path.insert(0, 'lib')
import carve, rs, reg, crategen
from carve import R, UI, FW, PKG, rd, wr
spec = carve.Spec('specs/wgpu.json')
k = reg.next_route_index()
TK_CRATES = ('mounted', 'paint', 'hosting', 'engine_surface', 'engine')
FEATURES = {'mounted': ['testkit = []'], 'paint': ['testkit = []'], 'hosting': ['testkit = []'], 'engine_surface': ['testkit = []'],
            'engine': ['testkit = ["semio-framework-ui-wgpu-mounted-layout/testkit", "semio-framework-ui-wgpu-hosting/testkit", "semio-framework-ui-wgpu-engine-surface/testkit", "semio-framework-ui-retained-tree/testkit", "dep:semio-framework-ui-wgpu-paint-reference"]']}
for key, c in spec.crates.items():
    pkgdir = c['abs'] + '/' + PKG
    deps, devs, targets = crategen.build_deps(spec, c, pkgdir)
    blocks = []
    for tgt, lines in targets.items(): blocks.append("[target.'%s'.dependencies]\n" % tgt + '\n'.join(sorted(set(lines))))
    if c.get('tests_only'):
        devs = sorted(set(devs) | set(deps), key=lambda l: l.split(' ')[0]); deps = []
    devs = [d for d in devs if d not in deps]
    if c.get('tests_only'):
        for tk in TK_CRATES:
            pk = spec.crates[tk]['pkg']
            devs = [d.replace(' }', ', features = ["testkit"] }') if d.startswith(pk + ' ') else d for d in devs]
    if key == 'engine':
        pk = spec.crates['paintref']['pkg']
        pl = [d for d in deps if d.startswith(pk + ' ')]
        deps = [d for d in deps if not d.startswith(pk + ' ')] + [d.replace(' }', ', optional = true }') for d in pl]
        devs = devs + pl
    for tn, tp in c.get('targets', []):
        blocks.append('[[test]]\nname = "%s"\npath = "%s"' % (tn, os.path.relpath(R + '/' + tp if not tp.startswith('/') else tp, pkgdir)))
    txt = reg.manifest_text(c['pkg'], c['id'], c['desc'], pkgdir, deps, devs, FEATURES.get(key, []), blocks)
    wr(pkgdir + '/Cargo.toml', txt); wr(pkgdir + '/📋️project.json', reg.project_json(c['id']))
    reg.add_member(pkgdir); reg.add_owner(c['id'], pkgdir)
    if reg.add_routes(c['pkg'], c['id'], k, 'semio-framework-ui-retained-tree-tree'): k += 1
    print(key, len(deps), len(devs), [t for t in targets])
