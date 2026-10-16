"""🧾 Item-level losslessness: multiset of top-level items (kind, name / impl trait+self type) of the old test sources against the new crates."""
import sys, os, json, collections, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
import wp8lib as L, unitgraph
from okrs import items
def key(it):
    if it['kind'] == 'impl':
        tr, st = unitgraph.impl_parts(it['text'])
        return ('impl', tr, st)
    if it['kind'] in ('use', 'mod'): return None
    if it['kind'] is None:
        m = re.match(r'\s*(?:#\[[^\]]*\]\s*)*([A-Za-z_][A-Za-z0-9_:]*)\s*!', it['text'])
        return ('macro', m.group(1).split('::')[-1] if m else it['text'][:30])
    return (it['kind'], it['name'])
def multiset(paths, base):
    c = collections.Counter()
    for f in paths:
        p = base(f)
        if not os.path.exists(p): continue
        for it in items(open(p, encoding='utf8').read()):
            k = key(it)
            if k: c[k] += 1
    return c
if __name__ == '__main__':
    census = json.load(open(L.GEN + '/okwp8-census.json'))
    before_files = sorted({l.split('\t')[1] for l in open(L.GEN + '/okwp8-tests-before.tsv', encoding='utf8')})
    # all test-only sources of the old kernel closure
    d = json.load(open(L.GEN + '/okwp8-closure-before.json'))
    old = [r['path'] for r in d['test native'] if r['kind'] != 'embedded' and r['path'].endswith('.rs')]
    lib_old = {r['path'] for r in d['lib native']}
    old = [f for f in old if f not in lib_old and 'wp8-tests' not in f]
    mb = multiset(old, lambda f: (L.PRE + '/' + f) if os.path.exists(L.PRE + '/' + f) else L.R + '/' + f)
    new_files = set()
    for crate in census:
        dd = json.load(open(L.GEN + '/okwp8-census-%s.json' % crate))
        for r in dd['test native']:
            if r['kind'] != 'embedded' and r['path'].endswith('.rs'): new_files.add(r['path'])
    # the new test-only sources: everything in the new closures that is not part of the kernel lib closure or the facade tests we keep
    ma = multiset(sorted(new_files), lambda f: L.R + '/' + f)
    lost = mb - ma; gained = ma - mb
    print('old items', sum(mb.values()), 'new items', sum(ma.values()))
    print('lost', sum(lost.values()))
    for k, v in sorted(lost.items(), key=lambda x: str(x))[:60]: print('   -', k, v)
    print('gained', sum(gained.values()))
    for k, v in sorted(gained.items(), key=lambda x: str(x))[:60]: print('   +', k, v)
