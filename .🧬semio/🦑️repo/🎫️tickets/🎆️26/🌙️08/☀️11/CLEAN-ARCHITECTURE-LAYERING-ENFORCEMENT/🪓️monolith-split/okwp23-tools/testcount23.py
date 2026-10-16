# 🧮️ WP2/WP3 variant (unique files, store crates included). Static test-count gate: attribute-level count of test functions in the test closure before (pristine snapshot, kernel closure) and after (facade + every carved crate and companion closure).
import json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
HERE = os.path.dirname(os.path.abspath(__file__))
import okcrates as C
from okrs import tokenize
R = C.R; SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
ATTR = re.compile(r'#\[\s*(?:[A-Za-z_0-9]+::)*(?:test|async_test|tokio_test|wasm_bindgen_test|quickcheck)\s*(?:\([^\]]*\))?\s*\]')
def count_file(path):
    try: src = open(path, encoding='utf8').read()
    except Exception: return 0
    code = ''.join(t[1] if t[0] not in ('comment',) else ' ' for t in tokenize(src) if t[0] != 'str') if False else src
    code = re.sub(r'//[^\n]*', '', code)
    return len(ATTR.findall(code))
def closure(pkg, entry, feats, tmp):
    env = dict(os.environ, PKG=pkg, ENTRY=entry, FEATURES=feats, LIST=tmp, ROW='test native')
    subprocess.run(['bun', os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools/closure.ts'], env=env, capture_output=True, text=True, cwd=R)
    d = json.load(open(tmp)); return [r['path'] for r in d['test native'] if r['kind'] == 'source' or r['path'].endswith('.rs')]
def main():
    model = C.build()[1]
    before = json.load(open(SCR + '/closure-orig.json'))['test native']
    b = sum(count_file(os.path.join(SCR, 'orig', r['path'])) for r in before if r['path'].endswith('.rs'))
    print('before: kernel test closure files', len(before), 'tests', b)
    total = 0; rows = []
    units = [('os-kernel', C.K, C.K + '/🦀️.rs', 'deflate,sync,ureq,worker')]
    for k in C.RANK + C.COMPANIONS:
        if k in __import__('okreal').REAL: units.append((k, C.pkg_abs(k, model), C.node_abs(k, model) + '/🦀️.rs', 'protocol-laws,mutation-testing' if k in ('os-spr', 'os-spr-laws-tests') else ''))
    STORE = [k for n in ('b1', 'b2', 'b3') for k in open(HERE + f'/../🗑️generated/okwp23-{n}.crates').read().split()]
    for k in STORE: units.append((k, C.pkg_abs(k, model), C.node_abs(k, model) + '/🦀️.rs', 'hot-path-census' if k in ('os-store-ledger', 'os-store-replay') else ''))
    seen_files = {}
    for name, pkg, entry, feats in units:
        files = closure(pkg, entry, feats, SCR + '/tc-' + name + '.json')
        fresh = [f for f in files if f not in seen_files]
        for f in files: seen_files[f] = name
        n = sum(count_file(os.path.join(R, f)) for f in fresh)
        rows.append((name, len(files), n)); total += n
    for r in rows: print('  ', *r)
    print('after: sum over', len(rows), 'closures =', total, '(before', b, ')')
main()
