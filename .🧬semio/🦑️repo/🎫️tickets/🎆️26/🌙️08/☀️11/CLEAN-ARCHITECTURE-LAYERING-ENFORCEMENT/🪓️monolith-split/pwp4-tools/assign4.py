# 🔗 Re-solve of the planner assignment with trait impls of local types bound to their type (orphan rule); diff against the frozen plan assignment.
import json, collections, sys
from common import *
files, items, defs, owner = PT.build()
size0, adj0, crate0 = PT.graph(items, defs, owner)
final0, moves0, _ = PT.simulate(size0, adj0, crate0, items)
owner2 = dict(owner)
nb = 0
for it in items:
    if it['kind'] == 'impl' and it.get('trait') and it.get('target') and it['id'] not in owner2:
        d = [j for j in defs.get(it['target'], []) if items[j]['kind'] in ('struct', 'enum', 'union', 'type')]
        if len(d) == 1:
            owner2[it['id']] = d[0]; nb += 1
print('trait impls newly bound to their local type:', nb)
size1, adj1, crate1 = PT.graph(items, defs, owner2)
final1, moves1, _ = PT.simulate(size1, adj1, crate1, items)
unit = lambda i: owner.get(i, i)
diff = collections.Counter(); lines = collections.Counter()
for it in items:
    u0 = unit(it['id'])
    if u0 not in final0: continue
    c0 = final0[u0]; u1 = owner2.get(it['id'], it['id']); c1 = final1.get(u1)
    if c1 is None: continue
    if c0 != c1:
        diff[(c0, c1)] += it['e'] - it['s'] + 1
print('units whose crate changed (from -> to: lines):')
for k, v in sorted(diff.items(), key=lambda kv: -kv[1]): print(' ', k, v)
tot = collections.Counter()
for u, c in final1.items(): tot[c] += size1[u]
print({c: tot[c] for c in PT.ORDER if c.startswith(('reactor', 'runtime', 'extension', 'plugin-program', 'job', 'command'))})
print('--- WP4-file items whose assignment changed')
WP4 = lambda f: (f.startswith('⚛️reactor/') and not f.startswith(('⚛️reactor/🧵️executor/', '⚛️reactor/🪟️surfaces/'))) or f.startswith('🚪️lifetime/')
for it in items:
    if not WP4(it['file']): continue
    u0 = unit(it['id']); u1 = owner2.get(it['id'], it['id'])
    c0 = final0.get(u0); c1 = final1.get(u1)
    if c0 and c1 and c0 != c1: print(it['file'], it['s'], it['e'], it['kind'], it['defname'] or it.get('target'), c0, '->', c1)
