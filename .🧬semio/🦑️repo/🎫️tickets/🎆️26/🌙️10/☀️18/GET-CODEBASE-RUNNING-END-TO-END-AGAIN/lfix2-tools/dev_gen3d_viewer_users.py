from devlib import *
import sys
R = EXT_ROOT + '🧩️composition/🧪️tests/🧊️generation3d/'
CTX = R + '🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs'
dry = '--apply' not in sys.argv
for dp, dn, fn in os.walk(R):
    for f in sorted(fn):
        if not f.endswith('.rs'):
            continue
        p = os.path.join(dp, f)
        if p == CTX or ('👁️viewer' not in p and '🔭️viewer-laws' not in p):
            continue
        s0 = read(p)
        s, counts, roots = port_dev_file(p, 'Generation3dViewerFixture', 'Generation3dViewerHarness', dry=dry, ctors=('app',))
        if s != s0:
            print(('DRY ' if dry else 'APPLY ') + p[len(R):][-90:], counts, sorted(roots))
