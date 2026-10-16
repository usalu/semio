import sys, os, collections
sys.path.insert(0, '.')
from oinf_rs import *
INF = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite"
files = {}
for root in ("🌍️world", "🎲️board"):
    for dp, dn, fn in os.walk(os.path.join(INF, root)):
        if '🧪️tests' in dp or '🧫️fixtures' in dp: continue
        for f in fn:
            if f.endswith('.rs'):
                p = os.path.join(dp, f)
                files[p[len(INF)+1:]] = p
defs = collections.defaultdict(list)
impls = collections.defaultdict(list)
for rel, p in sorted(files.items()):
    text = open(p, encoding='utf-8').read()
    items, _ = chunk_file(text)
    for it in items:
        lines = text.count('\n', it.full_start, it.full_end)
        if it.kind in ('struct', 'enum', 'union', 'trait', 'type'):
            defs[it.name].append((rel, it.kind, lines))
        if it.kind == 'impl':
            impls[it.impl_ty].append((rel, it.impl_trait, lines))
        if it.kind == 'macro_call':
            impls['MACRO:'+it.name].append((rel, None, lines))
rows = []
for ty, lst in impls.items():
    inh = [x for x in lst if x[1] is None]
    tot = sum(x[2] for x in lst)
    inh_t = sum(x[2] for x in inh)
    mods = sorted(set(x[0].split('/')[-2] if x[0].count('/') else x[0] for x in lst))
    rows.append((tot, ty, inh_t, len(lst), mods))
rows.sort(reverse=True)
for r in rows[:50]:
    print(r[0], r[1], 'inherent', r[2], 'blocks', r[3], 'defs', [d[0].split('/')[-2] for d in defs.get(r[1], [])], r[4])
