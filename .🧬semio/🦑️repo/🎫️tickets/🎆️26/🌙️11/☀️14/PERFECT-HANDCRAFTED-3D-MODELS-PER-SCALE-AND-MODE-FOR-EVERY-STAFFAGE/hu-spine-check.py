"""🩻 Measures dorsal and sternal attachments, free ends, handedness and the coarse strut aspect on the published documents."""
import json
from math import dist,cos,sin,atan2
from runpy import run_path
from pathlib import Path
ticket=Path(__file__).resolve().parent
failures=[]
def separation(p,a,b):
    u=[q-r for q,r in zip(b,a)]
    t=max(0,min(1,sum((x-y)*v for x,y,v in zip(p,a,u))/sum(v*v for v in u)))
    return dist(p,[y+t*v for y,v in zip(a,u)]),t
def clearance(p,solid,lateral=False):
    values=[]
    if solid['kind']=='capsule':
        a,b=solid['p0'],solid['p1'];d,t=separation(p,a,b)
        return d-(solid['r0']*(1-t)+solid['r1']*t)
    if solid['kind']=='box':
        x,y,z=solid['transform']['translation'];angle=2*atan2(*solid['transform']['rotation'][2:]);h=solid['size'][0]/2
        a,b=([x+sign*h*cos(angle),y+sign*h*sin(angle),z] for sign in [-1,1])
        return separation(p,a,b)[0]-min(solid['size'][1:])/2
    for a,b in zip(solid['sections'],solid['sections'][1:]):
        d,t=separation(p,a['at'],b['at'])
        r='rx' if lateral else 'ry'
        values.append(d-a[r]*(1-t)-b[r]*t)
    return min(values)
charts=run_path(str(ticket/'hu-rib-charts.py'))['charts']
for slug in ['tour-guide','person-with-umbrella',*charts]:
    a=json.loads(next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/'+slug+'/*')).read_text())['asset']
    for s,v in a['volumes'].items():
        st={q['id']:q for q in v['structure']};spine=next(q for q in st.values() if q.get('bone')=='torso' and q['id'] in ['spine','sk-spine','thoracolumbar-spine','vertebra-t12']);sternum=next(q for q in st.values() if q.get('bone')=='torso' and q['id'].endswith('sternum'));ribs=[q for q in st.values() if q['id'].startswith('costal-')]
        roots=[];front=[];free=[];ratios=[];joined=[]
        for q in ribs:
            name=q['id'];n=int(name.split('-')[1]);sections=q['sections'];points=[r['at'] for r in sections]
            left=name.endswith('-l')
            assert all((p[2]>0)==left for p in points)
            if left:
                assert [[x,y,-z] for x,y,z in points]==[r['at'] for r in st[name[:-1]+'r']['sections']]
            assert points[0][0]>min(p[0] for p in points[1:])
            origin=st['vertebra-t'+str(n)] if spine['id']=='vertebra-t12' else spine
            roots.append(clearance(points[0],origin)-sections[0]['rx'])
            if n<=7:
                front.append(clearance(points[-1],sternum)-sections[-1]['rx'])
            if n in [8,9,10]:
                joined.append(clearance(points[-1],st['costal-7-'+name[-1]])-sections[-1]['rx'])
            if n>=11:
                free.append(abs(points[-1][2]))
            if q.get('slender'):
                ratios.append(sum(dist(x,y) for x,y in zip(points,points[1:]))/(2*max(q['rx'] for q in sections)))
        report=(slug,s,'pairs',len(ribs)//2,'root gap mm',round(max(roots)*1000,2),'sternal gap mm',round(max(front)*1000,2),'costal margin gap mm',round(max(joined,default=0)*1000,2),'floating clearance mm',round(min(free,default=0)*1000,2),'slender ratio',round(min(ratios,default=100),2))
        print(*report)
        if max(roots)>0 or max(front)>.0003 or max(joined,default=0)>.0003 or (free and min(free)<.05) or (ratios and min(ratios)<20):failures.append(report)
assert not failures,failures
