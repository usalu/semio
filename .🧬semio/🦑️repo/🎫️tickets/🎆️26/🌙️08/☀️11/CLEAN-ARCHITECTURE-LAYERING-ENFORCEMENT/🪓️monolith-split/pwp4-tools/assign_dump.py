# 🗺️ Per-item crate assignment of the planner (plan_tools.simulate) restricted to the WP4 files; writes JSON for the carve engine.
import json, collections
from common import *
files, items, defs, owner = PT.build()
size, adj, crate_of = PT.graph(items, defs, owner)
final, moves, sccs = PT.simulate(size, adj, crate_of, items)
unit = lambda i: owner.get(i, i)
WP4 = lambda f: (f.startswith('⚛️reactor/') and not f.startswith(('⚛️reactor/🧵️executor/', '⚛️reactor/🪟️surfaces/'))) or f.startswith('🚪️lifetime/')
rows = []
for it in items:
    f = it['file']
    if not WP4(f): continue
    u = unit(it['id'])
    c = final.get(u) or ('TEST' if (it['cfgtest'] or '/🧪️tests/' in f) else crate_of.get(u, '?'))
    rows.append(dict(file=f, s=it['s'], e=it['e'], kind=it['kind'], name=it['defname'] or it.get('target') or '', head=it['head'][:90], crate=c, planned=PT.item_crate(f, it['s']), cfgtest=it['cfgtest']))
json.dump(rows, open(GEN + '/pwp4-assign.json', 'w'), ensure_ascii=False, indent=0)
agg = collections.defaultdict(collections.Counter)
for r in rows:
    if r['crate'] != 'TEST': agg[r['file']][r['crate']] += r['e'] - r['s'] + 1
for f, c in sorted(agg.items()): print(f, dict(c))
print('--- differences between file-rule and optimizer assignment')
for r in rows:
    if r['crate'] not in ('TEST',) and r['crate'] != r['planned']: print(r['file'], r['s'], r['e'], r['kind'], r['name'], r['planned'], '->', r['crate'])
