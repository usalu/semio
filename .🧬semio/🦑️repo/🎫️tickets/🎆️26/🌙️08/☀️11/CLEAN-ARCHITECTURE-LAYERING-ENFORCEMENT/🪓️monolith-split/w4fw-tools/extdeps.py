import json, collections, sys
exec(open('plan.py').read().split("if __name__")[0])
from imports import EXTERN
IM = json.load(open('imports.json'))
U = json.load(open('units.json'))
def table(f):
    t = {}
    files = [f]
    if f.startswith('m_') and f not in ('m_media','m_canonical','m_utilinit'): files.append('manifest')
    if f in ('m_utilinit','m_canonical'): files.append('manifest')
    for ff in files:
        for u in IM[ff]:
            for it in u['items']:
                if it['alias'] in ('*','_'): continue
                root = it['path'].split('::')[0]
                t[it['alias']] = it['path']
    return t
ext = set(EXTERN) - {'std','core','alloc'}
cl = {i: assign(i) for i in E['order']}
use = collections.defaultdict(lambda: collections.defaultdict(set))
for i, e in ents.items():
    c = cl.get(i)
    if not c: continue
    t = table(e['file'])
    toks = set(e['tokens'])
    for tok in toks:
        if tok in ext: use[c][tok].add(e['name'])
        elif tok in t:
            root = t[tok].split('::')[0]
            if root in ext: use[c][root].add(e['name'])
    # macro-derive paths like semio_framework_value::ToValue handled by tok in ext
for c in sorted(use):
    print(c, {k: len(v) for k, v in sorted(use[c].items())})
json.dump({c:{k:sorted(v) for k,v in m.items()} for c,m in use.items()}, open('extdeps.json','w'), indent=0, ensure_ascii=False)
