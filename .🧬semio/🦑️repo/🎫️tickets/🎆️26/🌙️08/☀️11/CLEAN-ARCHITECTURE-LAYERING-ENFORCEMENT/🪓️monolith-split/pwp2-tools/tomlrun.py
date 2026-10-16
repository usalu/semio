import json, os, subprocess, sys
ARGS = list(sys.argv[1:])
from common import *
import model as M
src = ARGS[0] if ARGS else 'out'
crates = {}
for k in M.WP2:
    c = M.CRATES_JSON[k]
    pkg = c['pkg'].split('/🔌️plugin/')[1]
    path = (WS + '/out/📦️pkg/' + k + '/Cargo.toml') if src == 'out' else (PLUG + '/' + pkg + '/Cargo.toml')
    crates[c['name']] = dict(pkg=pkg, root=M.ROOT[k], src=path, id=c['id'], rank=c['rank'], deps=['semio-framework-plugin-' + d for d in c['deps']])
p = subprocess.run(['bun', os.path.dirname(os.path.abspath(__file__)) + '/tomlcheck.ts'], capture_output=True, text=True, env=dict(os.environ, CRATES=json.dumps(crates), RANKS=json.dumps({'semio-framework-plugin-' + k: v['rank'] for k, v in M.CRATES_JSON.items()})))
print(p.stdout + p.stderr[-600:])
