#!/usr/bin/env python3
# 🗺️ Dumps unit -> (source file, final crate) of plan_tools for every unit whose source or final crate belongs to WP2; writes 🗑️generated/pwp2-homes.json
import json, sys, collections, os
sys.argv = ['plan_tools.py']
import plan_tools as pt
files, items, defs, owner = pt.build()
size, adj, crate_of = pt.graph(items, defs, owner)
final, moves, sccs = pt.simulate(size, adj, crate_of, items)
WP2 = {'decl-views','decl-emission','decl-app','decl-surface','decl-registry','app-builder','plugin-program'}
rows = []
for u, c in final.items():
    it = items[u]
    src = crate_of[u]
    if c in WP2 or src in WP2:
        rows.append(dict(id=u, file=it['file'], s=it['s'], e=it['e'], kind=it['kind'], name=it.get('defname') or it.get('target') or it['head'][:50], head=it['head'][:90], src=src, dst=c, size=size[u], impl=bool(it['kind']=='impl')))
# attach impls bound to their owner item as part of the unit; also list standalone impls (trait impls)
json.dump(rows, open(os.path.dirname(os.path.abspath(__file__)) + '/../🗑️generated/pwp2-homes.json', 'w'), ensure_ascii=False, indent=0)
mv = [r for r in rows if r['src'] != r['dst']]
print(len(rows), 'rows', len(mv), 'moved')
tot = collections.Counter()
for r in rows: tot[r['dst']] += r['size']
print(dict(tot))
for r in sorted(mv, key=lambda r: (r['src'], r['dst'])): print(r['src'], '->', r['dst'], r['file'], r['s'], r['name'], r['size'])
