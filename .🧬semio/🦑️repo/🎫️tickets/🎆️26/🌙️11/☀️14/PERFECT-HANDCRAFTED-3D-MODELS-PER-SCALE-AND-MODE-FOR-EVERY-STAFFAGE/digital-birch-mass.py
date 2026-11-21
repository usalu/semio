"""🌿 Place the mass of the three outer birch crowns on their hanging and terminal sprays."""
from pathlib import Path
import json
t=Path(__file__).resolve().parent;g=t/'🗑️generated';doc=json.loads((g/'digital-birch-leaders.json').read_text());vol=doc['asset']['volumes']['500']
for bone in sorted({q['member'] for q in vol['structure'] if q['part']=='mass' and q.get('member','').startswith('fol-')}):
    old=next(q for q in vol['structure']if q['id']=='mass-'+bone)
    leaves=[q for q in vol['body']if q['bone']==bone and q['material']=='birch-leaf']
    sleeve=next(s for s in leaves if s['id'].endswith('-s'))
    def depth(s):return sum(p[2]for p in s['vertices'])/len(s['vertices'])
    sides=sorted(leaves,key=depth)
    old.clear();old.update(id='mass-'+bone,of=sleeve['id'],member=bone,part='mass')
    for label,leaf in [('near',sides[-1]),('far',sides[0])]:
        if leaf!=sleeve:vol['structure'].append(dict(id='mass-'+bone+'-'+label,of=leaf['id'],member=bone,part='mass'))
(g/'digital-birch-branches.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
