"""🌿 Follow seven pollard shoots and the central birch spray through their widest sections."""
import math
import runpy
from pathlib import Path
ticket=Path(__file__).parent
b=runpy.run_path(str(ticket/'living-botanical-shells.py'));edit=runpy.run_path(str(ticket/'living-shape-rows.py'))['edit']
add,sub,mul,dot,cross,unit,mesh=(b[k] for k in ('add','sub','mul','dot','cross','unit','mesh'))
targets={'london-plane-pollarded':('leaf-e2-6','leaf-e2-7','leaf-e1-7','leaf-w1-7','leaf-w2-6','leaf-c-8','leaf-c-9'),'multi-stem-birch':('leaf-c-a-s',)}
def author(name,k,mode,s):
    if k!='100' or mode!='body' or s['id'] not in targets[name]:return False
    n=12 if name=='london-plane-pollarded' else 8
    pts=s['vertices']
    original=[pts[j:j+n] for j in range(0,len(pts)-2,n)]
    c=[[sum(p[a] for p in ring)/len(ring) for a in range(3)] for ring in original]
    along=unit(sub(c[-1],c[0]));spacing=math.dist(c[-1],c[0])*.004
    stations=[dot(sub(x,c[0]),along) for x in c]
    levels=[]
    for i,q in enumerate(stations):
        levels.append(([i],q-i*spacing))
        while len(levels)>1 and levels[-1][1]<levels[-2][1]:
            b,bv=levels.pop();a,av=levels.pop();levels.append((a+b,(len(a)*av+len(b)*bv)/(len(a)+len(b))))
    best=stations[:]
    for ids,value in levels:
        for i in ids:best[i]=value+i*spacing
    target=[add(x,mul(along,a-b)) for x,a,b in zip(c,best,stations)]
    rings=[];axis=None
    for j,ring in enumerate(original):
        t=unit(sub(c[-1],c[0]))
        before=sub(ring[0],c[j])
        direction=unit(sub(before,mul(t,dot(t,before))))
        if axis and dot(axis,direction)<0:direction=mul(direction,-1)
        axis=direction;ry=cross(t,axis)
        x=math.dist(ring[0],c[j]);y=math.dist(ring[n//4],c[j])
        rings.append([add(target[j],add(mul(axis,x*math.cos(i*math.tau/n)),mul(ry,y*math.sin(i*math.tau/n)))) for i in range(n)])
    mesh(s,rings)
    return True
for name in targets:
    for p in (next(b['root'].glob('*/'+name+'/*.json')),ticket/'volumes/vegetation'/f'{name}.json'):
        edit(p,lambda k,m,s:author(name,k,m,s))

