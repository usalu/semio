"""🌳 Model the irregular outer leaf groups and underside recesses of each English oak scaffold."""
from copy import deepcopy
from pathlib import Path
from math import atan2,cos,sin,pi,hypot,exp
import json,re
t=Path(__file__).resolve().parent;g=t/'🗑️generated';cat=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/english-oak/🔣️.json'))
doc=json.loads(cat.read_bytes());a=doc['asset'];fine=a['volumes']['50'];margins=json.loads((t/'digital-oak-margins.json').read_text())
def sculpt(src,sides,rings):
    s=deepcopy(src);pts=s['vertices'];n=sum(1 for tri in s['triangles'] if tri[0]==0)
    assert 6<=n<=32 and (len(pts)-2)%n==0
    rows=(len(pts)-2)//n;rx,ry,rz=(max(abs(p[i]) for p in pts)for i in range(3))
    turn=float(re.search(r'-(\d+)$',s['id'])[1]) if re.search(r'-(\d+)$',s['id']) else 0
    phase=.54*turn
    angles=[x*pi/180+phase for x in margins[s['bone']]]
    def point(p):
        x,y,z=p;u,v,w=x/rx,y/ry,z/rz;theta=atan2(v,u)
        belt=max(0,1-w*w)**.65
        recess=sum(deep*exp(-((atan2(sin(theta-dir),cos(theta-dir)))/width)**2) for dir,width,deep in zip(angles,[.20,.32,.24,.18,.28],[.23,.17,.32,.20,.25]))
        spread=belt*(.045*sin(3*theta+.8*w+phase)+.035*cos(2*theta-w)-recess)
        underside=max(0,-v)*belt
        return [round(x*(1+spread)+rx*.055*w*belt,7),round(y*(1+spread)+ry*(.085*underside+.042*sin(2*theta+phase)*belt),7),round(z*(1+.3*spread),7)]
    shaped=[point(p) for p in pts];stations=list(range(rows));ring=list(range(n))
    def distance(p,a,b):
        v=[y-x for x,y in zip(a,b)];w=[y-x for x,y in zip(a,p)];u=max(0,min(1,sum(x*y for x,y in zip(w,v))/(sum(x*x for x in v) or 1)))
        return sum((x-u*y)**2 for x,y in zip(w,v))**.5
    budget={24:0,14:.015,12:.035,10:.075}[sides]
    def idx(j,i):return shaped[1+j*n+i]
    while len(ring)>5 and budget:
        errors=[]
        for k,i in enumerate(ring):
            a,b=ring[k-1],ring[(k+1)%len(ring)]
            first=(a+1)%n;removed=[]
            while first!=b:removed.append(first);first=(first+1)%n
            errors.append(max(distance(idx(j,h),idx(j,a),idx(j,b))for h in removed for j in stations))
        if min(errors)>budget*.55:break
        del ring[errors.index(min(errors))]
    while len(stations)>4 and budget:
        errors=[]
        for k in range(1,len(stations)-1):
            a,b=stations[k-1],stations[k+1]
            errors.append(max(distance(idx(j,i),idx(a,i),idx(b,i)) for j in range(a+1,b) for i in ring))
        if min(errors)>budget*.45:break
        del stations[1+errors.index(min(errors))]
    h=len(ring)
    verts=[shaped[0]]+[idx(j,i) for j in stations for i in ring]+[shaped[-1]]
    end=len(verts)-1;tri=[]
    for i in range(h):
        k=(i+1)%h;tri.append([0,1+k,1+i])
        for row in range(len(stations)-1):
            l=1+h*row;tri.extend([[l+i,l+k,l+h+i],[l+k,l+h+k,l+h+i]])
        l=1+h*(len(stations)-1);tri.append([l+i,l+k,end])
    s.update(vertices=verts,triangles=tri);return s
leaves={q['id']:q for q in fine['body'] if q['material'].startswith('oak-leaf')}
for d,sides,rows in [('50',24,20),('100',14,9),('200',12,8),('500',10,7)]:
    v=a['volumes'][d];v['body']=[sculpt(leaves[q['id']],sides,rows) if q['id']in leaves else q for q in v['body']]
(g/'digital-oak-margins.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
