"""🌾 Keep the individual leaves and tiller strokes on the same articulated stems at 1:200."""
from pathlib import Path
from copy import deepcopy
import json
t=Path(__file__).resolve().parent;g=t/'🗑️generated'
doc=json.loads((g/'digital-grass-laminae.json').read_bytes());a=doc['asset']
p=a['representations'];body={s['id'] for s in a['volumes']['200']['body']}
for view,shapes in p['200']['geometry'].items():
    stems=[deepcopy(q) for q in p['100']['geometry'][view] if q['id'].startswith(('blade-','stem-','basal-','tiller-')) and q['id']in body]
    assert len(stems)>100
    for q in stems:q['penMm']=round(max(.10,q.get('penMm',.16)*.75),2)
    combined=[q for q in shapes if not q['id'].startswith(('ribbon-','blade-','stem-','basal-','tiller-'))]+stems
    p['200']['geometry'][view]=sorted(combined,key=lambda q:q['z'])
(g/'digital-grass-ink.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
