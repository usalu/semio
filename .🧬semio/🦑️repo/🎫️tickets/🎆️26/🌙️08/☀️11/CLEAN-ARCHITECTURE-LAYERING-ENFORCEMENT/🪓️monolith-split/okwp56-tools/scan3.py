import json, collections, sys
sys.path.insert(0, '../okwp0-tools')
import em, rs
t = em.read()
cof = json.load(open('../🗑️generated/okwp23-cof.json'))['items'][em.ROOT]
by = collections.defaultdict(list)
for r in cof: by[(r['kind'], r['name'])].append(r['crate'])
cur = rs.outline(t)
tot = collections.Counter()
rows = []
for it in cur:
    k = (it['kind'], it['name'])
    cs = by.get(k)
    n = it['e'] - it['s'] + 1
    rows.append((it['s'], it['e'], n, it['kind'], it['name'][:60], cs))
print('current root items', len(cur), 'cof rows', len(cof))
unk = [r for r in rows if r[5] is None and r[3] not in ('use','extern crate')]
print('unassigned (by name):', len(unk), sum(r[2] for r in unk))
for r in unk: 
    if r[2] > 5: print(r)
print()
for r in rows:
    if r[5]:
        tot[r[5][0]] += r[2]
print(tot)
