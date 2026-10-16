import os, sys, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import SourceFile
from imports import file_imports
def names(path, key='x'):
    f = SourceFile(key, path); out = set()
    for u in f.units:
        if u.test: continue
        if u.kind in ('fn','struct','enum','type','trait','static','const','union','mod','macro_rules!') and u.name and (u.vis == 'pub' or (u.kind=='macro_rules!' and 'macro_export' in u.attrs)):
            out.add(u.name)
        if u.kind == 'use':
            body = re.sub(r'//[^\n]*', '', u.text)
            if re.search(r'(^|\n)\s*pub\s+use\b', body):
                for pub, items in file_imports(u.text):
                    if pub:
                        for path_, alias in items:
                            if alias != '*': out.add(alias)
    return out
if __name__ == '__main__':
    base, new, bl, census = sys.argv[1:5]
    base_list = [l.strip() for l in open(bl, encoding='utf-8') if l.strip() and l.strip().endswith('.rs')]
    skip = ('/🧪️tests/', 'test_allocation', '/📥️requests/', '📽️projection', '📦️packages/🦀️rust/🦀️.rs')
    B = set(); N = set()
    for rel in base_list:
        if any(s in rel for s in skip): continue
        p = os.path.join(base, rel)
        if os.path.exists(p): B |= names(p)
    new_list = sorted({p for r in json.load(open(census)) for p, k, b, l in r.get('list', []) if p.endswith('.rs')})
    for rel in new_list:
        if any(s in rel for s in skip): continue
        N |= names(os.path.join(new, rel))
    print('base pub names', len(B), 'new', len(N), 'missing', sorted(B - N)[:40], 'new-only', sorted(N - B)[:40])
