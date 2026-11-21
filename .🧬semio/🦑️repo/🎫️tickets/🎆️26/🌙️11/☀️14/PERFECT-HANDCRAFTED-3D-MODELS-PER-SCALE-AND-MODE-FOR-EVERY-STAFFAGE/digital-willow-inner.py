"""🌳 Carry the recessed inner sprays through the open gaps between the willow's exterior curtains."""
from pathlib import Path
from copy import deepcopy
import json
t=Path(__file__).resolve().parent;g=t/'🗑️generated';doc=json.loads((g/'digital-willow-sprigs.json').read_bytes());a=doc['asset'];fine=a['volumes']['50']
leaves=[q for q in fine['body'] if q['id'].startswith(('inner-','fringe-'))]
for k in ('100','200','500'):
    vol=a['volumes'][k];originals={s['id'] for s in vol['body']}
    for q in leaves:
        if q['id'] not in originals:
            vol['body'].append(deepcopy(q))
            vol['thermal'].append(dict(id='th-'+q['id'],of=q['id'],region='leaf-curtain'))
fine_foliage={q['id']:q for q in fine['body'] if q['material']=='willow-leaf'}
vol=a['volumes']['100']
vol['body']=[deepcopy(fine_foliage[s['id']]) if s['id'] in fine_foliage else s for s in vol['body']]
(g/'digital-willow-inner.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
