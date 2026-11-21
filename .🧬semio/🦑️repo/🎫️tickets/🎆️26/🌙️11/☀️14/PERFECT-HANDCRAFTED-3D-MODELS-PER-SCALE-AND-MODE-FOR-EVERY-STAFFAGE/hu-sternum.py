"""🦴 Seats the costal cartilage along the guide’s sternal angle after the six-pair reduction."""
import json
from pathlib import Path
from runpy import run_path
ticket=Path(__file__).resolve().parent
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
p=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/tour-guide/*'))
old=p.read_text();a=json.loads(old)['asset']
sternum=next(q for q in a['volumes']['100']['structure'] if q['id']=='sternum')
if len(sternum['sections'])==2:
    sternum['sections'].insert(1,{**sternum['sections'][0],'at':[.29888,-.11434,0],'rx':.019,'ry':.0075})
    off=old.index('"100": {',old.index('"volumes": {'))
    i=old.index('{\n            "id": "sternum"',off)
    end=sp['member_end'](old,i)
    text=old[:i]+sp['dump'](sternum,5,'compact')+old[end:]
    assert p.read_text()==old
    p.write_text(text)
snap=ticket/'volumes'/'humans'/'tour-guide.json';s=snap.read_text();d=json.loads(s)
ss=d['volumes']['100']['structure']
ss[next(i for i,q in enumerate(ss) if q['id']=='sternum')]=sternum
m=json.dumps(d,ensure_ascii=False,separators=(',',':'))+'\n'
assert snap.read_text()==s
if m!=s:snap.write_text(m)
p=ticket/'r3-tour-guide-coarse.ts';old=p.read_text()
before='rr(0.013)), HA("torso", 1.437'
after='rr(0.013)), ...(scale === 100 ? [HA("torso", 1.315, 0.10684, 0.12184, 0.019)] : []), HA("torso", 1.437'
if before in old:
    assert old.count(before)==1 and p.read_text()==old
    p.write_text(old.replace(before,after))
