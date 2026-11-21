"""🌴 Retain analytic circular bearings and frond axes where coarse faceting clipped their physical section."""
import json
import sys
import math
import runpy
from pathlib import Path
ticket=Path(__file__).parent
args=sys.argv[1:];sys.argv=sys.argv[:1];b=runpy.run_path(str(ticket/'living-botanical-shells.py'));edit=runpy.run_path(str(ticket/'living-shape-rows.py'))['edit']
add,sub,mul,dot,cross,unit=(b[k] for k in ('add','sub','mul','dot','cross','unit'))
def quaternion(x,y,z):
    diag=(1+x[0]+y[1]+z[2],1+x[0]-y[1]-z[2],1-x[0]+y[1]-z[2],1-x[0]-y[1]+z[2])
    i=max(range(4),key=lambda j:diag[j]);s=2*math.sqrt(diag[i])
    q=([(y[2]-z[1])/s,(z[0]-x[2])/s,(x[1]-y[0])/s,s/4],
       [s/4,(x[1]+y[0])/s,(x[2]+z[0])/s,(y[2]-z[1])/s],
       [(x[1]+y[0])/s,s/4,(y[2]+z[1])/s,(z[0]-x[2])/s],
       [(x[2]+z[0])/s,(y[2]+z[1])/s,s/4,(x[1]-y[0])/s])[i]
    return [round(v,9) for v in q]
def author(name,k,m,s):
    if s.get('kind')!='polyhedron' or (k,m,s['id']) not in keys:return False
    v=s.pop('vertices');del s['triangles'];n=8 if (name=='multi-stem-birch' and k=='100') or (name=='bamboo-clump' and s.get('part') in ('mass','joint')) else 6
    if s.get('part') in ('mass','joint'):
        s['kind']='ellipsoid';s['radii']=[round(max(abs(p[0]) for p in v),8),round(max(abs(p[1]) for p in v)/max(abs(math.sin(math.tau*i/n)) for i in range(n)),8),round(max(abs(p[2]) for p in v),8)]
        return True
    sections=[]
    for j in range(0,len(v)-2,n):
        ring=v[j:j+n]
        c=[sum(p[i] for p in ring)/n for i in range(3)]
        x=sub(ring[0],c);y=mul(sub(ring[1],ring[-1]),.5/math.sin(math.tau/n))
        u=unit(x);w=unit(cross(x,y));vaxis=cross(w,u)
        sections.append({'at':[round(q,8) for q in c],'rx':round(math.hypot(*x),8),'ry':round(math.hypot(*y),8),'rotation':quaternion(u,vaxis,w)})
    s.update(kind='loft',sections=sections,capStart=True,capEnd=True)
    return True
for line in (ticket/'🗑️generated'/(args[0] if args else 'living-feature-latest.jsonl')).read_text().splitlines():
    j=json.loads(line);name=j['asset'];keys=set()
    if not j['findings']:continue
    live=next(b['root'].glob('*/'+name+'/*.json'));asset=json.loads(live.read_text())['asset']
    for f in j['findings']:
        _,k,tag=f['source'].split('.');m,i=tag.split('[')
        s=asset['volumes'][k][m][int(i[:-1])];keys.add((k,'body' if 'of' in s else m,s.get('of',s['id'])))
    for p in (live,ticket/'volumes/vegetation'/f'{name}.json'):edit(p,lambda k,m,s:author(name,k,m,s))

