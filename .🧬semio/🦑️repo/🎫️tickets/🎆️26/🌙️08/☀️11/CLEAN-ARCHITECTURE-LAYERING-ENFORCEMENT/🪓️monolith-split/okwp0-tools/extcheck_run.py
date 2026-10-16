import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import extcheck as X, okcrates as C, okreal
from wp1tree import tree
model = C.build()[1]
M = C.M + '/'
ROOT_FILES = {'os-list-delta': ['🪡️list-delta/🦀️.rs'], 'os-semio': ['🧬️semio/🦀️.rs'], 'os-space-sqlite-cells': ['🪐️space/🪶️sqlite/🦀️.rs'], 'os-plugin-registry-schema': ['🔌️plugin/📇️registry/📦️deployment/🧬️schema/🦀️.rs'],
              'os-extension': ['🧩️extension/🦀️.rs'], 'os-pack-tools': ['🎒️pack/⌨️cli/🦀️.rs'], 'os-spr': ['📡️spr/🦀️.rs'], 'os-spr-cli': ['📡️spr/⌨️cli/🦀️.rs'], 'os-dsl': ['🗣️dsl/🦀️.rs']}
keys = sys.argv[1:] or list(ROOT_FILES)
for k in keys:
    pkg = C.pkg_abs(k, model)
    extdeps = X.deps_of(pkg)
    files = tree([M + r for r in ROOT_FILES[k]])
    files = [f for f in files if f.startswith(M)]
    tot = collections.Counter(); bad = []
    for f in files:
        r, b = X.scan_file(f, extdeps); tot.update(r); bad += b
    print('#####', k, 'files', len(files), 'externs', len(extdeps), 'ok', tot[True], 'unknown', tot[None], 'MISSING', tot[False])
    for b in bad[:40]: print('   MISSING', b[1], '|', os.path.relpath(b[0], M)[:60], b[2] if len(b) > 2 else '')
