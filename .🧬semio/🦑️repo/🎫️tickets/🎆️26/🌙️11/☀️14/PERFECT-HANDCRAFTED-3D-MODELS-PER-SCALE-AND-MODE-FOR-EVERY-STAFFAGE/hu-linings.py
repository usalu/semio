"""🧥 Preserves the rollator user's authored inner garment layers while dropping redundant intermediate loft rings within a measured tolerance."""
import json
import sys
import re
from math import pi,sin,cos,dist,copysign
from pathlib import Path
from runpy import run_path
t=Path(__file__).resolve().parent;sp=run_path(str(t/'r3-kneeling-gardener-splice.py'))
p=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/person-with-rollator/*'))
targets={'coat-cardigan':.00075,'coat-blouse':.00075,'coat-air':.00075,'trousers-tights':.00065,'trousers-air':.00065,'shoes-socks':.0006,'shoes-air':.0006,'beret-air':.00065}
def ring(s):
    q=s['rotation'];x,y,z,w=q;u=(1-2*y*y-2*z*z,2*x*y+2*z*w,2*x*z-2*y*w);v=(2*x*y-2*z*w,1-2*x*x-2*z*z,2*y*z+2*x*w)
    out=[]
    for i in range(32):
        a=i*2*pi/32;co=cos(a);si=sin(a);exp=2/s.get('exponent',2)
        dx=s['rx']*copysign(abs(co)**exp,co);dy=s['ry']*copysign(abs(si)**exp,si)
        out.append(tuple(p+dx*uu+dy*vv for p,uu,vv in zip(s['at'],u,v)))
    return out
def select(sections,limit):
    rings=[ring(q) for q in sections];length=[0]
    for a,b in zip(sections,sections[1:]):length.append(length[-1]+dist(a['at'],b['at']))
    keep={0,len(sections)-1};worst=0
    def fit(a,b):
        nonlocal worst
        if b-a<2:return
        score=0;at=a
        for i in range(a+1,b):
            f=(length[i]-length[a])/(length[b]-length[a]) if length[b]>length[a] else (i-a)/(b-a)
            gap=max(dist(p,[u+(v-u)*f for u,v in zip(x,y)]) for p,x,y in zip(rings[i],rings[a],rings[b]))
            if gap>score:score=gap;at=i
        if score<=limit:worst=max(worst,score);return
        keep.add(at);fit(a,at);fit(at,b)
    fit(0,len(sections)-1)
    return [sections[i] for i in sorted(keep)],worst
old=p.read_text();text=old;asset=json.loads(old)['asset'];v=asset['volumes']['50'];before=0;after=0;worst={}
for solid in v['thermal']:
    layer=solid.get('layer')
    if layer not in targets or solid.get('kind')!='loft' or len(solid['sections'])<5:continue
    sections,gap=select(solid['sections'],targets[layer]);worst[layer]=max(worst.get(layer,0),gap)
    before+=len(solid['sections']);after+=len(sections)
    if sections==solid['sections']:continue
    original=solid['id'];solid['sections']=sections;idx=text.index('"50": {',text.index('"volumes": {'))
    start=text.index('{\n            "id": "'+original+'"',idx);end=sp['member_end'](text,start)
    text=text[:start]+sp['dump'](solid,5,'compact')+text[end:]
print('interior loft rings',before,'to',after,'maximum sectional error mm',{k:round(v*1000,3) for k,v in worst.items()})
if '--write' in sys.argv and text!=old:
    doc=json.loads(text)['asset'];assert p.read_text()==old;p.write_text(text)
    snap=t/'volumes'/'humans'/'person-with-rollator.json';s=snap.read_text();d=json.loads(s);d['volumes']['50']['thermal']=doc['volumes']['50']['thermal']
    assert snap.read_text()==s;snap.write_text(json.dumps(d,ensure_ascii=False,separators=(',',':'))+'\n')
