import sys, os, re
sys.path.insert(0, 'lib')
import carve, rs
W = carve.UI + '/🎯️targets/🧊️wgpu'
roots = [W + '/🦀️.rs', W + '/🪀️widgets/🧰️kit/🦀️.rs', W + '/⚙️engine/🪟️surfaces/🦀️.rs', W + '/📌️mounted_layout/🎯️hit/🦀️.rs']
files = []
for r_ in roots: carve.reachable(r_, files)
tot = 0
for f in files:
    if not f.endswith('.rs'): continue
    for name, p, attrs, pub in carve.mounts(f):
        if name is None: continue
        if 'test' in attrs or '/🧪️tests/' in p or 'tests' in p.split('/')[-3:]:
            sz = 0; lines = 0
            if os.path.exists(p):
                t = carve.rd(p); lines = t.count('\n')
            cfg = re.findall(r'#\[cfg\(([^\n]*)\)\]', attrs)
            print('%-28s %5d  %s  <- %s %s' % (name, lines, os.path.relpath(p, carve.UI)[-70:], os.path.relpath(f, W)[-40:], cfg))
            tot += lines
print('total', tot)
