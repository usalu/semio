import os, re, sys
sys.path.insert(0, 'lib')
import carve, rs, crategen
from carve import R, UI, rd
spec = carve.Spec('specs/wgpu.json')
normal = {}
for k, c in spec.crates.items():
    t = rd(c['abs'] + '/' + carve.PKG + '/Cargo.toml'); sect = ''; normal[k] = set()
    for l in t.split('\n'):
        if l.startswith('['): sect = l.strip()
        m = re.match(r'(semio-framework-ui-wgpu-[a-z0-9-]+)\s*=', l)
        if m and 'dev-dependencies' not in sect:
            for k2, c2 in spec.crates.items():
                if c2['pkg'] == m.group(1): normal[k].add(k2)
def reach(a):
    seen = set(); st = [a]
    while st:
        n = st.pop()
        for m in normal[n]:
            if m not in seen: seen.add(m); st.append(m)
    return seen
above = {k: {k2 for k2 in spec.crates if k in reach(k2)} for k in spec.crates}
for k, c in spec.crates.items():
    if c.get('tests_only'): continue
    for f in crategen.crate_files(spec, c):
        if not f.endswith('.rs') or not os.path.exists(f): continue
        for name, p, attrs, pub in carve.mounts(f):
            if name is None or not ('test' in attrs or '/🧪️tests/' in p): continue
            files = carve.reachable(p)
            text = ''.join(rs.mask(rd(x)) for x in files if x.endswith('.rs') and os.path.exists(x))
            hit = sorted(k2 for k2 in above[k] if re.search(r'\b%s\b' % re.escape(spec.crates[k2]['ident']), text))
            if hit: print('%-14s %-34s %-60s -> %s' % (k, name, os.path.relpath(p, UI)[-58:], hit))
