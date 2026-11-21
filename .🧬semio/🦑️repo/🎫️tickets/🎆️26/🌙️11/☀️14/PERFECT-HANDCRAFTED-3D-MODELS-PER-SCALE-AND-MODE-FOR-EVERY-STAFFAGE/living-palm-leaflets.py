"""🌴 Model folded date-palm leaflets and the rounded transition along fruit stalks and adventitious roots."""
import math
import runpy
from pathlib import Path

ticket=Path(__file__).parent
b=runpy.run_path(str(ticket/'living-botanical-shells.py'));edit=runpy.run_path(str(ticket/'living-shape-rows.py'))['edit']
add,sub,mul,dot,cross,unit,mesh,loft=(b[k] for k in ('add','sub','mul','dot','cross','unit','mesh','loft'))

def leaf(s):
    pts=s['profile'];n=len(pts);mid=n//2
    if n%2==0:raise RuntimeError(s['id'])
    rings=[]
    for j in range(mid):
        a,other=pts[j],pts[-1-j]
        c=[(x+y)/2 for x,y in zip(a,other)]
        rings.append([[*a,0],[*c,s['to']],[*other,0],[*c,s['from']]])
    rings.append([[*pts[mid],0]])
    mesh(s,rings)

def stalk(s,n):
    a,bp,r,sr=s['p0'],s['p1'],s['r0'],s['r1']
    axis=unit(sub(bp,a))
    lateral=unit(cross(axis,min(([1,0,0],[0,1,0],[0,0,1]),key=lambda v:abs(dot(axis,v)))))
    normal=cross(axis,lateral)
    sections=[(add(a,mul(axis,-r)),0),(add(a,mul(axis,-.66*r)),.75*r),(a,r),(bp,sr),(add(bp,mul(axis,.66*sr)),.75*sr),(add(bp,mul(axis,sr)),0)]
    rings=[]
    for center,radius in sections:
        if not radius:rings.append([center]);continue
        rings.append([add(center,add(mul(lateral,radius*math.cos(math.tau*j/n)),mul(normal,radius*math.sin(math.tau*j/n)))) for j in range(n)])
    mesh(s,rings)
    for field in ('p0','p1','r0','r1','lateralScale'):s.pop(field,None)

def author(scale,mode,s):
    mat=s.get('material');kind=s.get('kind')
    if kind=='extrude' and mat=='palm-frond' and s.get('role')=='texture' and s.get('plane')=='xy':
        leaf(s)
    elif kind=='capsule' and (s['id'].startswith('bunch-') or s['id'].startswith('root-')) and s.get('lateralScale',1)==1:
        stalk(s,10 if scale=='50' else 8)
    elif kind=='loft' and mat=='palm-stem' and s.get('capStart') and s.get('capEnd'):
        trunk=s['id'].startswith(('m-stem-','m-crown','m-root-'))
        loft(s,({'50':16,'100':12,'200':10,'500':8} if trunk else {'50':8,'100':6,'200':6,'500':4})[scale])
    else:return False
    return True

for p in (next(b['root'].glob('*/date-palm/*.json')),ticket/'volumes/vegetation/date-palm.json'):edit(p,author)

