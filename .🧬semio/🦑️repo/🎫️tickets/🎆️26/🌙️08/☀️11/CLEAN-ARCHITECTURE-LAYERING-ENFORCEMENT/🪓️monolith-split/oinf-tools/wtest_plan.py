import sys; sys.path.insert(0,'.')
from wplan import *
from model import INF
P = plan(); nodes = P['nodes']; res = P['res']
# world lane unit -> crate
node_crate = {}
for u in res['units']:
    for m in u.members: node_crate[m] = P['unit_crate'][u.id]
top = collections.defaultdict(list)
for n in nodes:
    if n.id in node_crate and n.kind in ('fn','struct','enum','union','trait','type','const','static'):
        top[n.name].append(n)
# test files
tdirs = [INF + '/🌍️world/🧪️tests']
tfiles = []
for d in tdirs:
    for dp, dn, fn in os.walk(d):
        for f in sorted(fn):
            if f.endswith('.rs'): tfiles.append(os.path.join(dp, f))
tot = collections.Counter(); per = collections.defaultdict(collections.Counter)
for p in tfiles:
    text = open(p, encoding='utf-8').read()
    items, _ = chunk_file(text)
    # test-local defs
    local = {it.name: it for it in items if it.name and it.kind in ('fn','struct','enum','const','static','type')}
    # crate of each item = max crate of referenced world top-level names (cheap, non-transitive)
    memo = {}
    def cr(it, stack=()):
        key = id(it)
        if key in memo: return memo[key]
        memo[key] = 0
        c = 0
        for idn in it.plain:
            if idn in local and local[idn] is not it and id(local[idn]) not in stack:
                c = max(c, cr(local[idn], stack + (key,)))
            for d in top.get(idn, ()):
                c = max(c, node_crate[d.id])
        for (ty, nm) in it.assoc: pass
        memo[key] = c
        return c
    for it in items:
        if it.kind in ('use','mod'): continue
        ln = text.count('\n', it.full_start, it.full_end)
        per[p.split('/')[-2]][cr(it)] += ln
for f, c in per.items(): print(f, dict(sorted(c.items())))
agg = collections.Counter()
for f, c in per.items():
    for k, v in c.items(): agg[k] += v
print('tests lines by crate:', {CRATES[k][0]: v for k, v in sorted(agg.items())})
