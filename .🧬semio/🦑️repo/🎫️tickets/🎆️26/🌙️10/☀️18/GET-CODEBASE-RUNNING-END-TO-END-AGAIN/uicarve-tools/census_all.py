import json, os, subprocess, sys
sys.path.insert(0, 'lib')
import carve
R = carve.R
rows = []
for sp in ('specs/tui.json', 'specs/wgpu.json'):
    spec = carve.Spec(sp)
    for k, c in spec.crates.items():
        pk = c['abs'] + '/' + carve.PKG
        if not os.path.exists(pk + '/Cargo.toml'): continue
        out = subprocess.run(['bun', 'census.ts', os.path.relpath(pk, R)], cwd=R if False else None, capture_output=True, text=True, env=dict(os.environ)).stdout if False else None
        rows.append((c['pkg'], os.path.relpath(pk, R)))
rows.append(('semio-framework-ui', '🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust'))
res = []
T = os.getcwd()
for pkg, rel in rows:
    p = subprocess.run(['bun', T + '/census.ts', rel], cwd=R, capture_output=True, text=True)
    worst = {'lib': 0, 'test': 0}
    det = {}
    for l in p.stdout.split('\n'):
        if not l.startswith('{'): continue
        d = json.loads(l)
        if 'error' in d: det.setdefault('errors', []).append(d); continue
        kind = 'test' if d['test'] else 'lib'
        if d['ratio'] > worst[kind]: worst[kind] = d['ratio']; det[kind] = (d['label'], d['files'], d['bytes'], d['lines'])
    res.append((pkg, worst['lib'], worst['test'], det))
    print('%-58s lib %.3f test %.3f %s' % (pkg, worst['lib'], worst['test'], det.get('errors', '')))
json.dump(res, open('lists/census-final.json', 'w'), indent=1, ensure_ascii=False)
