"""🫁 Fits the independently authored landmark charts to the bones of each standing, seated, kneeling or sleeping figure."""
import json
import re
from pathlib import Path
from math import sin,cos,atan2
from runpy import run_path
ticket=Path(__file__).resolve().parent
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
charts=run_path(str(ticket/'hu-rib-charts.py'))['charts']
levels={50:[1,2,3,4,5,6,7,8,9,10,11,12],100:[1,3,5,7,9,11],200:[2,7,10],500:[2,7]}
def stations(q):
    if q['kind']=='box':
        x,y,_=q['transform']['translation'];h=q['size'][0]/2
        rz,rw=q['transform']['rotation'][2:];theta=2*atan2(rz,rw)
        points=[[x-h*cos(theta),y-h*sin(theta),0],[x+h*cos(theta),y+h*sin(theta),0]]
    elif q['kind']=='capsule':points=[q['p0'],q['p1']]
    else:points=[r['at'] for r in q['sections']]
    return sorted(([v*1000 for v in p] for p in points),key=lambda p:p[0])
def centre(points,x):
    if x<=points[0][0]:return points[0][1]
    for a,b in zip(points,points[1:]):
        if x<=b[0]:return a[1]+(b[1]-a[1])*(x-a[0])/(b[0]-a[0])
    return points[-1][1]
def tube(id,pts,r,slender=False):
    return dict(id=id,kind='loft',bone='torso',material='bone',role='body',tone=.9,
                sections=[dict(at=[round(v/1000,5) for v in p],rx=r/1000,ry=r/1000) for p in pts],
                capStart=True,capEnd=True,**({'slender':True} if slender else {}),member='torso',part='member')
def cage(chart,reference,spine,scale):
    make=[]
    for n in levels[scale]:
        h,w,hf=chart[n-1]
        back=centre(spine,h)
        front=centre(reference,hf)
        if n>7: front+=(0,3,9,17,30)[n-8]
        d=back-front;fall=h-hf
        def p(u,v,z):return [h-u*fall,back-v*d,w*z]
        path=[p(0,0,0),p(.11,-.075,.44),p(.50,.37,1)]
        path[0][2]=3.5
        if scale<=100:path.insert(2,p(.27,.015,.79))
        if n<=10:
            path.extend([p(.83,.82,.78),p(1,.955,.40)])
            if n<=7: end=[hf,front,3]
            else:
                targetH=chart[6][2]
                end=[targetH-1*(n-7),centre(reference,targetH)+1.7*(n-7),3+5.5*(n-7)]
            if scale==500:path.pop()
            path.append(end)
        else:path.append(p(1,.72 if n==11 else .44,.74 if n==11 else .93))
        radius=5 if scale==50 else 7.5 if scale==100 else 4
        for side,sign in [('l',1),('r',-1)]:
            make.append(tube('costal-'+str(n)+'-'+side,[(x,y,z*sign) for x,y,z in path],radius,scale>=200))
    if scale!=50:
        sel=[0,3,6]
        make.append(tube('sternum',[(chart[i][2],centre(reference,chart[i][2]),0) for i in sel],7.5 if scale==100 else 3,scale>=200))
    return make
def changed_note(old,scale):
    en={50:'twelve pairs of ribs with a costal margin and two floating pairs',100:'six paired rib courses',200:'three paired rib courses',500:'two paired rib courses and a sternal bar'}[scale]
    de={50:'zwölf Rippenpaare mit Rippenbogen und zwei freien Paaren',100:'sechs beidseitige Rippenzüge',200:'drei beidseitige Rippenzüge',500:'zwei beidseitige Rippenzüge und ein Brustbeinstab'}[scale]
    a=re.sub(r'\b(?:ten|nine|eight|seven|six|five|four|three|two) rib hoops?(?: inclined 20 degrees| falling 24 degrees to the front)?',en,old['en'],count=1,flags=re.I)
    b=re.sub(r'\b(?:zehn|neun|acht|sieben|sechs|fünf|vier|drei|zwei) [^.,;:()]{0,24}?Rippen(?:ringe|reifen|bögen)',de,old['de'],count=1,flags=re.I)
    a=re.sub(r'(?:a |the |tapered )?rib[- ]cage (?:as one (?:barrel|form)|loft)',en,a,flags=re.I)
    if a==old['en']:a+=' The torso shows '+en+'.'
    if b==old['de']:b+=' Der Rumpf zeigt '+de+'.'
    return {'en':a,'de':b}
hum=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
for slug,chart in charts.items():
    p=next((hum/slug).glob('*'));old=p.read_text();text=old
    asset=json.loads(old)['asset'];fine=asset['volumes']['50']['structure']
    if any(q['id']=='costal-12-l' for q in fine):continue
    sternum=next(q for q in fine if q.get('bone')=='torso' and q['id'].endswith('sternum'))
    reference=stations(sternum)
    for k,v in asset['volumes'].items():
        sc=int(k);idx=text.index('"'+k+'": {',text.index('"volumes": {'))
        begin,end=sp['member_range'](text[idx:],'        ','structure')
        begin+=idx;end+=idx
        group=text[begin:end]
        spine=next(q for q in v['structure'] if q.get('bone')=='torso' and q.get('part')=='member' and q['id'] in ('spine','sk-spine','thoracolumbar-spine','vertebra-t12'))
        vertical=stations(spine)
        if spine['id']=='vertebra-t12':
            vertical=sorted([[500*(a+b) for a,b in zip(q['p0'],q['p1'])] for q in v['structure'] if q['id'].startswith('vertebra-t')])
        solids=cage(chart,reference,vertical,sc)
        chunks=re.split(r',\n(?=          \{)',group)
        retained=[];done=False
        for i,chunk in enumerate(chunks):
            m=re.search(r'"id":\s*"([^"]+)"',chunk);q=v['structure'][i]
            assert q['id']==m[1],(slug,k,q['id'],m[1])
            oldrib=q.get('bone')=='torso' and 'rib' in q['id']
            remove=oldrib or (sc!=50 and q.get('bone')=='torso' and q['id'].endswith('sternum'))
            insert=not done and (oldrib or q is spine)
            if not remove:retained.append(chunk)
            if insert:
                a=',\n'.join('          '+sp['dump'](s,5,'compact') for s in solids)
                retained.append(a);done=True
        assert done and retained[0].startswith('        "structure"') and retained[-1].endswith(']')
        text=text[:begin]+',\n'.join(retained)+text[end:]
        before=v['decisions'][-1];after=changed_note(before,sc)
        for lang in ['en','de']:
            text=text.replace(json.dumps(before[lang],ensure_ascii=False),json.dumps(after[lang],ensure_ascii=False),1)
    doc=json.loads(text)['asset']
    for k,v in doc['volumes'].items():
        assert len([q for q in v['structure'] if q['id'].startswith('costal-')])==len(levels[int(k)])*2
        assert len(v['structure'])==len({q['id'] for q in v['structure']})
        assert all(q['id']==r['id'] and q==r for q,r in zip(v['body'],asset['volumes'][k]['body']))
        assert v['thermal']==asset['volumes'][k]['thermal']
    assert p.read_text()==old
    p.write_text(text)
    snap=ticket/'volumes'/'humans'/(slug+'.json');orig=snap.read_text();data=json.loads(orig)
    for k,v in doc['volumes'].items():
        data['volumes'][k]['structure']=v['structure'];data['volumes'][k]['decisions']=v['decisions']
    assert snap.read_text()==orig
    snap.write_text(json.dumps(data,ensure_ascii=False,separators=(',',':'))+'\n')
    print(slug)
