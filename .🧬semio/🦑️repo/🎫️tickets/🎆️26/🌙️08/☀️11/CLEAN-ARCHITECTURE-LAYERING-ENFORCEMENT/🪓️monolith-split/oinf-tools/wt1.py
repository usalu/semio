import sys; sys.path.insert(0,'.')
from wgen import *
from model import INF
P = plan(); crates, ic, imports = build_crates(P)
nodes = P['nodes']
# world name -> crate idx (top-level defs) from crates
wdef = {}
for c in crates:
    for nm in c.defs: wdef[nm] = c.idx
tdirs = sorted(os.path.join(INF, '🌍️world/🧪️tests', d) for d in os.listdir(INF + '/🌍️world/🧪️tests') if os.path.isdir(os.path.join(INF, '🌍️world/🧪️tests', d)) and not d.startswith('🧫️'))
T = {}
for d in tdirs:
    p = d + '/🦀️.rs'
    text = open(p, encoding='utf-8').read()
    items, _ = chunk_file(text)
    local = {}
    for idx, it in enumerate(items):
        if it.name and it.kind in ('fn','struct','enum','const','static','type'): local[it.name] = idx
    memo = {}
    def cr(idx, stack=()):
        if idx in memo: return memo[idx]
        memo[idx] = 0
        it = items[idx]; c = 0
        for idn in it.plain:
            if idn in local and local[idn] != idx and local[idn] not in stack: c = max(c, cr(local[idn], stack + (idx,)))
            if idn in wdef and idn not in local: c = max(c, wdef[idn])
        for (a, b) in it.assoc:
            if a in wdef and a not in local: c = max(c, wdef[a])
        memo[idx] = c
        return c
    rows = []
    for idx, it in enumerate(items):
        if it.kind in ('use', 'mod'): continue
        rows.append((idx, it, cr(idx), text.count('\n', it.full_start, it.full_end)))
    T[p] = (text, items, rows, local)
    print(d.split('/')[-1], len(rows), 'items', sum(r[3] for r in rows), 'lines')
agg = collections.defaultdict(lambda: collections.Counter())
for p, (text, items, rows, local) in T.items():
    for idx, it, c, ln in rows: agg[p.split('/')[-2]][CRATES[c][0]] += ln
for k, v in agg.items(): print(k, dict(v))
# helper sharing: non-test items (no #[test]) used by tests in >1 crate bucket within a file
for p, (text, items, rows, local) in T.items():
    users = collections.defaultdict(set)
    for idx, it, c, ln in rows:
        if '#[test]' in it.attrs or any(a.startswith('#[test') for a in it.attrs):
            stack = [idx]; seen = set()
            while stack:
                x = stack.pop()
                if x in seen: continue
                seen.add(x)
                for idn in items[x].plain:
                    if idn in local and local[idn] != x: stack.append(local[idn])
            for x in seen: users[x].add(c)
    multi = [(items[x].name, sorted(CRATES[c][0] for c in cs)) for x, cs in users.items() if len(cs) > 1 and not any(a.startswith('#[test') for a in items[x].attrs)]
    print(p.split('/')[-2], 'helpers shared across crate buckets:', len(multi), multi[:12])
