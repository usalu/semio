# 📊 Census rows (lib + test closures per cfg) of the carved crates through the canonical counter. usage: census_all.py key[:features] ...
import subprocess, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import okcrates as C
model = C.build()[1]
for spec in sys.argv[1:]:
    key, _, feats = spec.partition(':')
    pkg = C.pkg_abs(key, model); entry = C.node_abs(key, model) + '/🦀️.rs'
    env = dict(os.environ, PKG=pkg, ENTRY=entry, FEATURES=feats)
    out = subprocess.run(['bun', os.path.dirname(os.path.abspath(__file__)) + '/closure.ts'], env=env, capture_output=True, text=True, cwd=C.R)
    print('#####', spec); print(out.stdout.strip()); 
    if out.returncode: print(out.stderr[-300:])
