"""🦿 Routes the runner's coarse foot member through the talus and along the metatarsal column."""
from pathlib import Path
from runpy import run_path
import json
ticket=Path(__file__).resolve().parent
path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/running-child/*'))
old=path.read_text()
text=old
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
v=json.loads(old)['asset']['volumes']['500']
for side,sign in [('l',1),('r',-1)]:
    s=next(p for p in v['structure'] if p['id']=='sk-foot-'+side)
    for k in ('p0','p1','r0','r1'):s.pop(k)
    positions=[[-.01584,-.04742,0],[-.001,-.022,0],[.001,-.002,0],[.024,-.008,0],[.078,.012,.002*sign],[.12968,.04103,0]]
    s.update(kind='loft',sections=[dict(at=p,rx=.0044,ry=.0042) for p in positions],capStart=True,capEnd=True)
    i=text.rindex('{\n            "id": "'+s['id']+'"')
    j=sp['member_end'](text,i)
    text=text[:i]+sp['dump'](s,5,'compact')+text[j:]
assert path.read_text()==old
assert json.loads(text)['asset']['volumes']['500']==v
path.write_text(text)
