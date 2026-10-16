#!/usr/bin/env python3
# 🗺️ Unit table of the item graph (plan_tools) for WP2: every unit whose source file or final crate is a WP2 crate, with its member spans (owner item plus bound inherent impls).
# writes 🗑️generated/pwp2-units.json
import json, sys, collections, os
sys.argv = ['plan_tools.py']
import plan_tools as pt
from common import GEN

WP2 = {'decl-views', 'decl-emission', 'decl-app', 'decl-surface', 'decl-registry', 'app-builder', 'plugin-program'}
files, items, defs, owner = pt.build()
size, adj, crate_of = pt.graph(items, defs, owner)
final, moves, sccs = pt.simulate(size, adj, crate_of, items)
members = collections.defaultdict(list)
for it in items:
    u = owner.get(it['id'], it['id'])
    if u in final:
        members[u].append(it)
rows = []
for u, c in final.items():
    src = crate_of[u]
    if True:
        its = sorted(members[u], key=lambda x: (x['file'], x['s']))
        rows.append(dict(id=u, file=items[u]['file'], kind=items[u]['kind'], name=items[u].get('defname') or items[u].get('target') or items[u]['head'][:50], head=items[u]['head'][:100], src=src, dst=c, size=size[u],
                         members=[dict(file=m['file'], s=m['s'], e=m['e'], kind=m['kind'], head=m['head'][:100]) for m in its]))
json.dump(rows, open(GEN + '/pwp2-units.json', 'w'), ensure_ascii=False, indent=0)
print(len(rows), 'units')
