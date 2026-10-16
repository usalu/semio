# 📊 Census rows (lib + test closures per cfg) of the WP5/WP6 crates through the canonical counter: files / bytes / active lines. usage: census56.py [crate-key[:features] ...]
import subprocess, sys, os, json
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools')
import okcrates as C
model = C.build()[1]
specs = sys.argv[1:] or open(HERE + '/../🗑️generated/okwp56-b1.crates').read().split()
out = {}
for spec in specs:
    key, _, feats = spec.partition(':')
    pkg = C.pkg_abs(key, model); entry = C.node_abs(key, model) + '/🦀️.rs'
    env = dict(os.environ, PKG=pkg, ENTRY=entry, FEATURES=feats)
    r = subprocess.run(['bun', HERE + '/../okwp0-tools/closure.ts'], env=env, capture_output=True, text=True, cwd=C.R)
    print('#####', spec); print(r.stdout.strip())
    if r.returncode: print(r.stderr[-300:])
