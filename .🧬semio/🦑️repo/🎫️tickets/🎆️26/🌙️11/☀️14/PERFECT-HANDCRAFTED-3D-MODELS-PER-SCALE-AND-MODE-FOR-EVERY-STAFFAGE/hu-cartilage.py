"""🦴 Reconnects interrupted sternal load paths with discrete costal-cartilage bars in the six affected fine skeletons."""
import json
from pathlib import Path
from runpy import run_path
from math import dist
t=Path(__file__).resolve().parent;f=run_path(str(t/'hu-open-ribs.py'));hum=f['hum'];sp=f['sp']
cartilage={'id':'costal-cartilage','title':{'en':'Costal cartilage','de':'Rippenknorpel'},'emissivity':.96,'solarAbsorptivity':.5,'source':'authored-estimate'}
sets={
 'worker-carrying-box':('rib-{s}-{n}',range(1,8),4.5),
 'standing-adult':('rib-{s}-{n}',[7],4.2),
 'standing-phone-user':('rib-{s}-{n}',[7],4.2),
 'leaning-person':('rib-{n}-{s}',[4,5,6,7],4.1),
 'person-cooking':('rib-{n}-{s}',[2,3,4,5,6,7],4.2),
 'pregnant-person':('rib-{n}-{s}',[5,6,7],4.3)}
for slug,(fmt,numbers,radius) in sets.items():
    p=next((hum/slug).glob('*'));old=p.read_text();text=old
    asset=json.loads(old)['asset'];v=asset['volumes']['50'];st={q['id']:q for q in v['structure']}
    if any(q.get('material')=='costal-cartilage' for q in st.values()):continue
    breastbone=f['stations'](st['sternum']);parts=[]
    for n in numbers:
        for side,sign in [('l',1),('r',-1)]:
            rib=st[fmt.format(n=n,s=side)];points=[[v*1000 for v in q['at']] for q in rib['sections']]
            a,b=points[-2:]
            length=dist(a,b);start=[y+(x-y)*min(.45,2.5/length) for x,y in zip(a,b)]
            h=max(breastbone[0][0]+1.5,min(breastbone[-1][0]-1.5,b[0]));target=[h,f['centre'](breastbone,h),3*sign]
            middle=[.52*b[0]+.48*target[0]+1,.52*b[1]+.48*target[1]-1.4,.52*b[2]+.48*target[2]]
            assert dist(start,target)>7.5,(slug,n,dist(start,target))
            part=f['tube']('cartilage-'+str(n)+'-'+side,[start,middle,target],radius)
            part['material']='costal-cartilage';part['tone']=.78
            parts.append(part)
    idx=text.index('"50": {',text.index('"volumes": {'))
    start=text.index('{\n            "id": "sternum"',idx);end=sp['member_end'](text,start)
    text=text[:end]+',\n          '+',\n          '.join(sp['dump'](q,5,'compact') for q in parts)+text[end:]
    i,j=sp['member_range'](text,'    ','materials');s=text[i:j];end=s.rindex('}')+1
    text=text[:i]+s[:end]+',\n      '+sp['dump'](cartilage,3,'compact')+s[end:]+text[j:]
    before=v['decisions'][-1]
    for lang,suffix in [('en',' Discrete costal cartilage joins the anterior bony ends to the sternum.'),('de',' Eigenständige Rippenknorpel verbinden die vorderen Knochenenden mit dem Brustbein.')]:
        text=text.replace(json.dumps(before[lang],ensure_ascii=False),json.dumps(before[lang]+suffix,ensure_ascii=False),1)
    doc=json.loads(text)['asset']
    assert p.read_text()==old;p.write_text(text)
    snap=t/'volumes'/'humans'/(slug+'.json');s=snap.read_text();draft=json.loads(s)
    for k in ['decisions','structure']:draft['volumes']['50'][k]=doc['volumes']['50'][k]
    assert snap.read_text()==s;snap.write_text(json.dumps(draft,ensure_ascii=False,separators=(',',':'))+'\n')
    print(slug,len(parts))
