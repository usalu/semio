# 🔎 WP2/WP3 extern-path scan: every `ext_crate::…` path in the mounted closure of each carved store crate must name an item the target crate exposes (extcheck.py machinery over the crate's own `#[path]` tree).
# usage: extcheck23.py <crate-key>...   (default: every crate of the b* batches)
import os, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE)
import extcheck as X, okcrates as C
from wp1tree import tree
model = C.build()[1]
M = C.M + '/'
keys = sys.argv[1:] or [k for n in ('b1', 'b2', 'b3') if os.path.exists(HERE + f'/../🗑️generated/okwp23-{n}.crates') for k in open(HERE + f'/../🗑️generated/okwp23-{n}.crates').read().split()]
bad_total = 0
for k in keys:
    pkg = C.pkg_abs(k, model)
    extdeps = X.deps_of(pkg)
    files = [f for f in tree([C.node_abs(k, model) + '/🦀️.rs']) if f.startswith(M)]
    tot = collections.Counter(); bad = []
    for f in files:
        r, b = X.scan_file(f, extdeps); tot.update(r); bad += b
    bad_total += tot[False]
    print('#####', k, 'files', len(files), 'externs', len(extdeps), 'ok', tot[True], 'unknown', tot[None], 'MISSING', tot[False])
    for b in bad[:40]: print('   MISSING', b[1], '|', os.path.relpath(b[0], M)[:70], b[2] if len(b) > 2 else '')
print('MISSING total', bad_total)
