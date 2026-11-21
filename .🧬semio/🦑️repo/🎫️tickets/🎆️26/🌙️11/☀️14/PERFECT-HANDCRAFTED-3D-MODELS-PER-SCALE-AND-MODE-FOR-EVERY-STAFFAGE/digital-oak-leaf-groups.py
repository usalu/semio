"""🌳 Preserve each scaffold’s measured foliage cushions and the gaps between the oak's spreading limbs."""
from pathlib import Path
from runpy import run_path
from collections import defaultdict
from copy import deepcopy
import json
t=Path(__file__).resolve().parent
h=run_path(str(t/'digital-birch-poplar-sprays.py'));reduce_leaf=h['reduce_leaf']
path=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/english-oak/🔣️.json'))
doc=json.loads(path.read_text());a=doc['asset'];v=a['volumes'];fine=v['50']
isleaf=lambda s:s['material'].startswith('oak-leaf')
leaves=[q for q in fine['body']if isleaf(q)]
for d,sides,stations in [('100',12,8),('200',10,6),('500',7,5)]:
    vol=v[d];gone={q['id']for q in vol['body']if isleaf(q)}
    vol['body'][:]=[q for q in vol['body']if q['id']not in gone]
    vol['thermal'][:]=[q for q in vol['thermal']if q.get('of')not in gone]
    keep=[reduce_leaf(q,sides,stations)for q in leaves]
    h['insert'](vol,keep,fine)
    if any(s.get('of') in gone-{q['id'] for q in keep} for s in vol['structure']):raise ValueError('A structural bearing has lost its canopy')
(t/'🗑️generated/digital-oak-cushions.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
