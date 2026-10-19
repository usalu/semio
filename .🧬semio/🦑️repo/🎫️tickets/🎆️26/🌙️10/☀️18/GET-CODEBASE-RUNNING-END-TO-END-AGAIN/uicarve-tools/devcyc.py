import re, sys
sys.path.insert(0, 'lib')
import carve
specs = [carve.Spec(p) for p in sys.argv[1:]]
crates = {}
for sp in specs: crates.update({c['pkg']: c for c in sp.crates.values()})
normal, dev = {}, {}
for pkg, c in crates.items():
    t = carve.rd(c['abs'] + '/' + carve.PKG + '/Cargo.toml')
    normal[pkg] = set(); dev[pkg] = set(); sect = ''
    for l in t.split('\n'):
        if l.startswith('['): sect = l.strip()
        m = re.match(r'(semio-framework-ui-[a-z0-9-]+)\s*=', l)
        if m and m.group(1) in crates:
            (dev if 'dev-dependencies' in sect else normal)[pkg].add(m.group(1))
def reach(a):
    seen = set(); st = [a]
    while st:
        n = st.pop()
        for m in normal[n]:
            if m not in seen: seen.add(m); st.append(m)
    return seen
for pkg in sorted(dev):
    for d in sorted(dev[pkg]):
        if pkg in reach(d) or pkg == d:
            print('DEV-CYCLE', pkg.replace('semio-framework-ui-', ''), '-dev->', d.replace('semio-framework-ui-', ''))
