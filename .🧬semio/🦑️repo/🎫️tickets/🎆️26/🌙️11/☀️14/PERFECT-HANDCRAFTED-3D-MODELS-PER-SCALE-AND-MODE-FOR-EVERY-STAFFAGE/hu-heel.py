"""👣 Places the rollator user's heel supports in the centre of the two actual heel treads."""
import json,math
from pathlib import Path
from runpy import run_path
ticket=Path(__file__).resolve().parent
path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/person-with-rollator/*'))
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
old=path.read_text()
a=json.loads(old)['asset']
text=old
for side in ('l','r'):
    sole=next(p for p in a['volumes']['50']['body'] if p['id']=='shoe-heel-'+side)
    t=sole['transform']['rotation']
    angle=2*math.atan2(t[2],t[3])
    xs=[p[1] for p in sole['profile']]
    u,v=(min(xs)+max(xs))/2,sole['from']
    anchor=next(p for p in a['anchors'] if p['id']=='heel-'+side)
    anchor.update(x=round(u*math.cos(angle)-v*math.sin(angle),6),y=round(u*math.sin(angle)+v*math.cos(angle),6))
    i=text.index('{\n        "id": "heel-'+side+'"',text.index('"anchors": '))
    j=sp['member_end'](text,i)
    text=text[:i]+sp['dump'](anchor,3,'compact')+text[j:]
assert json.loads(text)['asset']['anchors']==a['anchors']
assert path.read_text()==old
path.write_text(text)
