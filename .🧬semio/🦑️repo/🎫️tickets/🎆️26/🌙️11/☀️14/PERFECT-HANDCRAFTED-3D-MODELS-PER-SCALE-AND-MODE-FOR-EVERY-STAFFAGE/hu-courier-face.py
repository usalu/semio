"""🧢 Models the courier's exposed face from the chin to the wool cap rim in the two reduced thermal scales."""
from pathlib import Path
from runpy import run_path
from math import sqrt
import json
t=Path(__file__).resolve().parent;sp=run_path(str(t/'r3-kneeling-gardener-splice.py'))
p=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/courier-with-hand-truck/*'))
old=p.read_text();text=old;asset=json.loads(old)['asset']
for k in ['200','500']:
    vol=asset['volumes'][k];face=next(q for q in vol['thermal'] if q['id']=='th-head')
    if 'of' not in face:continue
    head=next(q for q in vol['body'] if q['id']=='head')
    rim=next(q for q in vol['body'] if q['id']=='cap')['sections'][0]['at'][0]
    if k=='200':
        q,r=head['sections'][1:3];f=(rim-q['at'][0])/(r['at'][0]-q['at'][0])
        edge={**q,'at':[rim,round(q['at'][1]+f*(r['at'][1]-q['at'][1]),5),0],'rx':round(q['rx']+f*(r['rx']-q['rx']),5),'ry':round(q['ry']+f*(r['ry']-q['ry']),5)}
        sections=[head['sections'][0],q,edge]
    else:
        cx,cy,_=head['transform']['translation'];u,v,w=head['radii'];sections=[]
        for x in [-.054,-.040,-.012,.023,rim]:
            radius=sqrt(1-((x-cx)/u)**2)
            sections.append(dict(at=[x,cy,0],rx=round(w*radius,5),ry=round(v*radius,5),exponent=2,rotation=[0,-.70711,0,.70711]))
    exposed=dict(id='th-head',kind='loft',bone='head',material='skin',role='body',tone=head['tone'],sections=sections,capStart=True,capEnd=False,region='face-skin')
    idx=text.index('"'+k+'": {',text.index('"volumes": {'));start=text.index('{\n            "id": "th-head"',idx);end=sp['member_end'](text,start)
    text=text[:start]+sp['dump'](exposed,5,'compact')+text[end:]
if text!=old:
    doc=json.loads(text)['asset'];assert p.read_text()==old;p.write_text(text)
    snap=t/'volumes'/'humans'/'courier-with-hand-truck.json';s=snap.read_text();d=json.loads(s)
    for k in ['200','500']:d['volumes'][k]['thermal']=doc['volumes'][k]['thermal']
    assert snap.read_text()==s;snap.write_text(json.dumps(d,ensure_ascii=False,separators=(',',':'))+'\n')
