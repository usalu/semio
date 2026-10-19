"""🔎 uicarve probe: a temporary probe crate (in the repo tree, removed after) whose `use` lines are every rule target; run through the oracle to prove each rewrite target resolves."""
import json, os, sys, shutil
sys.path.insert(0, 'lib')
import carve, reg
from carve import R, UI, PKG
cmd = sys.argv[1]
D = UI + '/🧪️tests/🔎️consumer-probe'
PD = D + '/' + PKG
if cmd == 'make':
    lines = set()
    for sp in ('specs/wgpu.json', 'specs/tui.json'):
        spec = carve.Spec(sp)
        for _, r in spec.rules.rules:
            segs = spec.render('__probe__', r, [x for x in r['to'].split('::') if x]) if False else None
            if r.get('ident'): path = [r['ident']] + [x for x in r['to'].split('::') if x]
            else: path = [spec.crates[r['crate']]['ident']] + [x for x in r['to'].split('::') if x]
            lines.add('::'.join(path))
    body = '//! 🔎️ consumer probe (temporary).\n\n' + ''.join('#[allow(unused_imports)]\npub use %s;\n' % l if False else '#[allow(unused_imports)]\nuse %s;\n' % l for l in sorted(lines))
    os.makedirs(PD, exist_ok=True)
    open(D + '/🦀️.rs', 'w', encoding='utf8').write(body)
    deps = set(l.split('::')[0] for l in lines)
    ident_pkg = {}
    for sp in ('specs/wgpu.json', 'specs/tui.json'):
        for c in carve.Spec(sp).crates.values(): ident_pkg[c['ident']] = (c['pkg'], os.path.relpath(c['abs'] + '/' + PKG, PD))
    depl = []
    for d in sorted(deps):
        if d in ident_pkg: pk, rel = ident_pkg[d]; depl.append('%s = { path = "%s" }' % (pk, rel))
        else:
            key = reg.EXT_BY_IDENT.get(d)
            if not key: print('NO DEP FOR', d); continue
            sp_ = reg.EXT[key].get('dependencies') or reg.EXT[key].get('dev') or {}
            if 'abspath' in sp_: depl.append(reg.dep_line(key, {**sp_, 'optional': False}, PD))
    man = reg.manifest_text('semio-framework-ui-probe', 'ui-probe', 'temporary consumer probe', PD, depl, [], [], [])
    open(PD + '/Cargo.toml', 'w', encoding='utf8').write(man)
    print(len(lines), 'probe lines', len(depl), 'deps')
elif cmd == 'drop':
    shutil.rmtree(D, ignore_errors=True); print('removed')
