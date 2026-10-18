import json, os, re, sys, collections
sys.path.insert(0, 'lib')
import carve, rs, reg, crategen
from carve import R, UI, FW, PKG, rd, wr, remove
spec = carve.Spec('specs/tui.json')
k = reg.next_route_index()
FEATURES = {'terminal': ['native = ["dep:libc"]'], 'host': ['bindgen = ["dep:wasm-bindgen"]']}
SPECIAL_BLOCKS = {
 'terminal': ['[target.\'cfg(all(unix, not(target_arch = "wasm32")))\'.dependencies]\nlibc = { version = "0.2.186", optional = true }'],
 'host': ['[target.\'cfg(all(target_arch = "wasm32", not(target_env = "p2")))\'.dependencies]\nwasm-bindgen = { version = "0.2.106", optional = true }'],
}
for key, c in spec.crates.items():
    pkgdir = c['abs'] + '/' + PKG
    deps, devs, targets = crategen.build_deps(spec, c, pkgdir, special={'libc', 'wasm-bindgen'})
    if c.get('tests_only'):
        devs = sorted(set(devs) | set(deps), key=lambda l: l.split(' ')[0]); deps = []
    blocks = list(SPECIAL_BLOCKS.get(key, []))
    for tgt, lines in targets.items(): blocks.append("[target.'%s'.dependencies]\n" % tgt + '\n'.join(sorted(set(lines))))
    txt = reg.manifest_text(c['pkg'], c['id'], c['desc'], pkgdir, deps, devs, FEATURES.get(key, []), blocks)
    wr(pkgdir + '/Cargo.toml', txt)
    wr(pkgdir + '/📋️project.json', reg.project_json(c['id']))
    reg.add_member(pkgdir)
    reg.add_owner(c['id'], pkgdir)
    reg.add_routes(c['pkg'], c['id'], k, 'semio-framework-ui-retained-tree-tree')
    k += 1
    print(key, len(deps), len(devs))
