"""🖐️ Measures the crutch handle in the merged hand channel and the ferrule position relative to the declared ground anchor."""
from pathlib import Path
from math import dist,hypot,atan2,cos,sin,radians
import json
t=Path(__file__).resolve().parent
p=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/person-on-crutches/*'))
asset=json.loads(p.read_text())['asset'];bones={b['id']:b for b in asset['rig']['bones']};anchors={a['id']:a for a in asset['anchors']}
for k in ['200','500']:
    v=asset['volumes'][k];body={q['id']:q for q in v['body']};thermal={q['id']:q for q in v['thermal']}
    for side in ['l','r']:
        c='crutch-'+side;rig=bones[c];q=body['hand-'+side];shaft=body['crutch-tube-'+side];g=body['crutch-grip-'+side];toe=body['crutch-tip-'+side]
        pts=q['profile'];crossings=0;radii=[]
        for a,b in zip(pts,pts[1:]+pts[:1]):
            if (a[1]>0)!=(b[1]>0) and a[0]-(b[0]-a[0])*a[1]/(b[1]-a[1])>0:crossings+=1
            dx,dy=b[0]-a[0],b[1]-a[1];f=max(0,min(1,-(a[0]*dx+a[1]*dy)/(dx*dx+dy*dy)))
            radii.append(hypot(a[0]+f*dx,a[1]+f*dy))
        assert crossings%2==0
        x,y,z,w=q['transform']['rotation'];axis=(2*(x*z+y*w),2*(y*z-x*w),1-2*(x*x+y*y))
        theta=radians(rig['angleDeg']);gx,gy,gz=g['axis'];gaxis=(cos(theta)*gx-sin(theta)*gy,sin(theta)*gx+cos(theta)*gy,gz)
        assert dist(axis,gaxis)<.00002
        assert dist(q['transform']['translation'],[rig['x'],rig['y'],rig['z']])<1e-9
        r=min(radii);mid=(q['from']+q['to'])/2;radius=next(u+(vv-u)*(mid-x)/(xx-x) for (x,u),(xx,vv) in zip(g['profile'],g['profile'][1:]) if x<=mid<xx)
        assert r<=radius
        assert all(thermal['th-'+n]['of']==n for n in [g['id'],toe['id']])
        collar=shaft['sections'][-1]['at'];toAx=sum(c*v for c,v in zip(collar,toe['axis']))
        assert toe['profile'][0][0]<=toAx<=toe['profile'][-1][0]
        tip=anchors['tip-'+side];end=[toe['profile'][-1][0]*v for v in toe['axis']];offset=dist([tip['x'],tip['y'],tip['z']],end)
        assert offset<=.0001
        for pose in asset['poses']:
            assert pose['bones'][c]=={'angleDeg':0}
        print(k,side,'channel radius mm',round(r*1000,2),'handle radius mm',round(radius*1000,2),'tip anchor distance mm',round(offset*1000,2),'poses',len(asset['poses']))
