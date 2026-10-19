from devlib import *
import sys
R = EXT_ROOT + '🧩️composition/🧪️tests/🧊️generation3d/'
SKIP = ('👁️viewer', '🔭️viewer-laws', '🔬️brep-extension')
CTX = R + '🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs'
dry = '--apply' not in sys.argv
tot = {}
for dp, dn, fn in os.walk(R):
    for f in sorted(fn):
        if not f.endswith('.rs'):
            continue
        p = os.path.join(dp, f)
        if p == CTX or any(k in p for k in SKIP):
            continue
        s0 = read(p)
        s, counts, roots = port_dev_file(p, 'Generation3dAppFixture', 'Generation3dApp', dry=dry)
        if s != s0:
            print(('DRY ' if dry else 'APPLY ') + p[len(R):][-90:], counts, sorted(roots))
