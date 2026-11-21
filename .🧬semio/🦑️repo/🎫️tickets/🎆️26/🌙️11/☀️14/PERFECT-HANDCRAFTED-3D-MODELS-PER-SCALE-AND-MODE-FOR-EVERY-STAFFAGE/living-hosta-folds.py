"""🌿 Separate four cupped hosta lamina sections at their fold without shifting the midrib or the margin width."""
import runpy
from pathlib import Path
t=Path(__file__).parent;b=runpy.run_path(str(t/'living-botanical-shells.py'));edit=runpy.run_path(str(t/'living-shape-rows.py'))['edit']
add,sub,mul,dot,unit,mesh=(b[k] for k in ('add','sub','mul','dot','unit','mesh'))
def author(k,m,s):
    if k!='50' or m!='body' or s['id'] not in ('blade-20-l','blade-22-l','blade-24-l','blade-25-l'):return False
    n=16;v=s['vertices'];rings=[v[i:i+n] for i in range(0,len(v)-2,n)]
    c=[[sum(p[i] for p in ring)/n for i in range(3)] for ring in rings]
    axis=unit(sub(c[-1],c[0]));steps=[dot(sub(v,u),axis) for u,v in zip(c,c[1:])]
    for j in range(3,8):
        allowed=min(steps[j-1],steps[j])*.15
        new=[]
        for p in rings[j]:
            h=dot(sub(p,c[j]),axis)
            new.append(add(p,mul(axis,max(-allowed,min(allowed,h))-h)))
        rings[j]=new
    mesh(s,rings);return True
for p in (next(b['root'].glob('*/hosta-clump/*.json')),t/'volumes/vegetation/hosta-clump.json'):edit(p,author)

