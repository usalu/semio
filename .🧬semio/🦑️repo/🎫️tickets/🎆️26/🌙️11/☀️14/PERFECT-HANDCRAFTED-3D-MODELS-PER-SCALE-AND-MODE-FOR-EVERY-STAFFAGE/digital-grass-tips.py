"""🌾 Retain the arching outside blade of every side tiller through the coarsest grass model."""
from pathlib import Path
from collections import defaultdict
from copy import deepcopy
import json,math
t=Path(__file__).resolve().parent;g=t/'🗑️generated'
doc=json.loads((g/'digital-grass-ink.json').read_bytes());a=doc['asset'];near,far=(a['volumes'][k]for k in ('200','500'));groups=defaultdict(list)
for s in near['body']:
    if s['id'].startswith('blade-'):
        groups[s['id'].rsplit('-',1)[0]].append(s)
tips=[]
for group,parts in groups.items():
    if group.startswith(('blade-l','blade-r')):
        s=max(parts,key=lambda q:math.dist(q['vertices'][-1],q['vertices'][-2]))
        tips.append(s)
ids={s['id'] for s in tips}
far['body'].extend(deepcopy(tips))
far['thermal'].extend(deepcopy(q) for q in near['thermal'] if q.get('of') in ids)
for view,g500 in a['representations']['500']['geometry'].items():
    for s in a['representations']['200']['geometry'][view]:
        if s['id'] in ids:
            stroke=deepcopy(s);stroke['penMm']=.10;g500.append(stroke)
    g500.sort(key=lambda s:s['z'])
(g/'digital-grass-sprays.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
