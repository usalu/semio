"""📦 uicarve fixt: embedded json fixtures of companion test crates become run-time reads (fixture_text!), taking them out of the source closure.
usage: fixt.py <spec.json> <crate-key>... [--write]"""
import os, re, sys
sys.path.insert(0, 'lib')
import carve, rs, peel, crategen
from carve import R, UI, PKG, rd, wr
spec = carve.Spec(sys.argv[1]); keys = [a for a in sys.argv[2:] if not a.startswith('--')]; write = '--write' in sys.argv
IMPORT = 'use semio_framework_test_fixtures::fixture_text;'
for key in keys:
    c = spec.crates[key]; pkgdir = c['abs'] + '/' + PKG
    files = [f for f in crategen.crate_files(spec, c) if f.endswith('.rs') and os.path.exists(f)]
    includer = {}
    for f in files:
        for name, p, attrs, pub in carve.mounts(f):
            if name is None and p.endswith('.rs'): includer[p] = f
    changed = 0; need_import = set()
    for f in files:
        s = rd(f)
        def rep(m):
            p = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            return 'fixture_text!("%s")' % os.path.relpath(p, pkgdir)
        n = re.sub(r'include_str!\(\s*"([^"]+\.json)"\s*\)', rep, s)
        if n != s:
            changed += 1
            host = f
            while host in includer: host = includer[host]
            need_import.add(host)
            if write: wr(f, n)
    for host in need_import:
        s = rd(host)
        if IMPORT in s: continue
        us = peel.header_uses(s)
        if us: at = max(u.end for u in us); s = s[:at] + '\n' + IMPORT + s[at:]
        else:
            m = re.search(r'(?m)^(?!\s*//|\s*#!\[)', s); at = m.start() if m else 0
            s = s[:at] + IMPORT + '\n' + s[at:]
        if write: wr(host, s)
    print(key, 'files converted', changed, 'imports', len(need_import))
