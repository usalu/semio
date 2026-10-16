# 🧭️ Explains the union differential's remaining new unresolved imports: every new (file, imported name) must be a name of a carved facade module (surface parity is proven by pubnames.py), i.e. a glob-propagation artifact of the sandbox.
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import okoracle as O, pubnames as P, extcheck as X
tag, cfgs = sys.argv[1], sys.argv[2].split(',')
def names(path, sb):
    out = set()
    for line in open(path, encoding='utf-8'):
        try: d = json.loads(line)
        except Exception: continue
        m = d.get('message', '')
        if d.get('level') != 'error' or not m.startswith('unresolved import'): continue
        sp = next((s for s in d.get('spans', []) if s.get('is_primary')), None)
        f = os.path.normpath((sp or {}).get('file_name', '').replace(sb + '/', ''))
        for p in re.findall(r'`([^`]*)`', m):
            out.add((f, p.split('::')[-1], p.split('::')[0] if '::' in p else ''))
    return out
old_text = open(P.SCR + '/orig/' + P.KREL + '/🦀️.rs', encoding='utf-8').read()
deps = X.deps_of(P.K); X._root_cache.clear()
old = X.Index(old_text, P.SCR + '/orig/' + P.KREL, 'old'); old.deps = deps; X.ROOT_OF['old'] = old
facade = set()
for m in P.MODULES:
    n, _ = P.module_names(old, deps, m)
    if n: facade |= n
bad_total = 0
for c in cfgs:
    base = O.SCR + '/ok-union-' + tag
    a = names(f'{O.GEN}/okwp0-oracle-{tag}-A-{c}.jsonl', base + '/A'); b = names(f'{O.GEN}/okwp0-oracle-{tag}-B-{c}.jsonl', base + '/B')
    new = a - b
    explained = {x for x in new if x[1] in facade or x[1].startswith('semio_framework_') or x[1] in ('protocol', 'pack')}
    unexplained = new - explained
    print(f'{c}: new unresolved imports {len(new)}, explained (carved facade module names, or extern crate idents of the carved crates checked by extcheck.py) {len(explained)}, unexplained {len(unexplained)}')
    for x in sorted(unexplained)[:20]: print('   UNEXPLAINED', x)
    bad_total += len(unexplained)
sys.exit(1 if bad_total else 0)
