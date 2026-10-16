# 📋️ Static owner table of the final layout: every actor-persistent static of the WP4 files, its owner crate, and the crates that name it (these need an accessor at the flip, the macro has no visibility parameter).
import collections, json
exec(open('dag4.py').read().split("bad = []")[0])
rev = collections.defaultdict(set)
for u, vs in adj.items():
    for v in vs: rev[v].add(u)
rows = []
for it in items:
    if it['kind'] != 'static' or not WP4F(it['file']): continue
    v = unit(it['id']); oc = crate.get(v)
    users = collections.Counter(crate.get(u) for u in rev[v] if crate.get(u) and crate.get(u) != oc)
    rows.append((oc, it['defname'], dict(users)))
for oc, n, u in sorted(rows, key=lambda r: (RANK.get(r[0], 99), r[1])):
    print(f"| `{n}` | {oc} | {', '.join(f'{k} ({v})' for k, v in sorted(u.items(), key=lambda kv: RANK.get(kv[0], 99))) or '-'} |")
