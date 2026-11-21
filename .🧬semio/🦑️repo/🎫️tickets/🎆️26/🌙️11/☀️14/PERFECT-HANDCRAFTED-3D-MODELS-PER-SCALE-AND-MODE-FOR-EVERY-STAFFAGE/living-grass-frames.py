"""🌾 Transport the transverse fold of named miscanthus blades around their overhanging arc."""
import math
import runpy
from pathlib import Path
ticket=Path(__file__).parent
b=runpy.run_path(str(ticket/'living-botanical-shells.py'));edit=runpy.run_path(str(ticket/'living-shape-rows.py'))['edit']
keys=set()
for line in (ticket/'🗑️generated/living-poly-grass-ok.log').read_text().splitlines():
    words=line.split()
    if len(words)==6 and words[1]=='miscanthus-grass':keys.add(tuple(words[2:5]))
add,sub,mul,unit,dot,cross,mesh=(b[k] for k in ('add','sub','mul','unit','dot','cross','mesh'))
def author(k,m,s):
    if (k,m,s['id']) not in keys:return False
    n=8 if k=='50' else 6
    pts=s['vertices'];rings=[pts[j:j+n] for j in range(0,len(pts)-2,n)]
    centers=[[sum(p[i] for p in row)/n for i in range(3)] for row in rings]
    result=[];axis=None;previous_tangent=None
    for i,row in enumerate(rings):
        c=centers[i];v=unit(sub(centers[min(i+1,len(centers)-1)],centers[max(i-1,0)]))
        x=sub(row[0],c);y=mul(sub(row[1],row[-1]),.5/math.sin(math.tau/n))
        to=unit(sub(x,mul(v,dot(x,v))))
        if axis is None:axis=to
        else:
            prev=unit(sub(axis,mul(v,dot(axis,v))))
            if dot(prev,to)<0:to=mul(to,-1)
            if k=='100' and s['id'] in ('blade-l3b-4','blade-r3b-4'):
                h=cross(previous_tangent,v);h1=cross(h,axis)
                axis=unit(add(axis,add(h1,mul(cross(h,h1),1/(1+dot(previous_tangent,v))))))
            else:axis=unit(add(mul(prev,.66),mul(to,.34)))
        previous_tangent=v
        side=cross(v,axis);rx=math.hypot(*x);ry=math.hypot(*y)
        result.append([add(c,add(mul(axis,rx*math.cos(j*math.tau/n)),mul(side,ry*math.sin(j*math.tau/n)))) for j in range(n)])
    mesh(s,result)
    return True
for p in (next(b['root'].glob('*/miscanthus-grass/*.json')),ticket/'volumes/vegetation/miscanthus-grass.json'):edit(p,author)

