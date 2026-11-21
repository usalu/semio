"""🌴 Retain smooth frond axes and the diameter of circular joints as the structural scale decreases."""
import json
import math
import runpy
from pathlib import Path
ticket=Path(__file__).parent
b=runpy.run_path(str(ticket/'living-botanical-shells.py'));edit=runpy.run_path(str(ticket/'living-shape-rows.py'))['edit']
helper=(ticket/'living-analytic-bearings.py').read_text()
local={'__file__':str(ticket/'living-analytic-bearings.py')};exec(helper.split('for line in (ticket/')[0],local)
quat=local['quaternion']
sub,mul,cross,unit=(b[k] for k in ('sub','mul','cross','unit'))
def profile(s,angles):
    x,y,z=s.pop('radii')
    s.update(kind='revolve',origin=[0,0,0],axis=[0,0,1],profile=[[round(z*math.sin(math.radians(h)),8),0 if abs(h)==90 else round(x*math.cos(math.radians(h)),8)] for h in angles],lateralScale=round(y/x,8))
def wood(s,n):
    v=s.pop('vertices');del s['triangles'];q=[]
    for j in range(0,len(v)-2,n):
        ring=v[j:j+n];c=[sum(p[i] for p in ring)/n for i in range(3)]
        x=sub(ring[0],c);y=mul(sub(ring[1],ring[-1]),.5/math.sin(math.tau/n));u=unit(x);w=unit(cross(x,y))
        q.append({'at':[round(f,8) for f in c],'rx':round(math.hypot(*x),8),'ry':round(math.hypot(*y),8),'rotation':quat(u,cross(w,u),w)})
    s.update(kind='loft',sections=q,capStart=True,capEnd=True)
def author(n,k,m,s):
    if m!='structure':return False
    if n=='bamboo-clump' and s.get('kind')=='ellipsoid' and s['part'] in ('joint','mass'):
        angles={'50':(-90,-60,-30,0,30,60,90),'100':(-90,-46,0,46,90),'200':(-90,-35,0,35,90),'500':(-90,0,90)}
        profile(s,angles[k])
    elif n in ('japanese-maple','multi-stem-birch') and k=='500' and s.get('kind')=='ellipsoid' and s.get('part')=='mass':
        profile(s,(-90,0,90))
    elif n=='date-palm' and k in ('50','100') and s.get('kind')=='polyhedron' and s['id'].startswith('m-fr-') and s['id'].endswith('b'):
        wood(s,8 if k=='50' else 6)
    elif n=='lombardy-poplar' and k=='200' and s.get('kind')=='polyhedron' and s['id'] in ('member-br-l1','member-br-r1','member-br-l2','member-br-r2','member-br-l3','member-br-r3'):
        wood(s,8)
    else:return False
    return True
for n in ('bamboo-clump','japanese-maple','multi-stem-birch','date-palm','lombardy-poplar'):
    for p in (next(b['root'].glob('*/'+n+'/*.json')),ticket/'volumes/vegetation'/f'{n}.json'):
        edit(p,lambda k,m,s:author(n,k,m,s))

