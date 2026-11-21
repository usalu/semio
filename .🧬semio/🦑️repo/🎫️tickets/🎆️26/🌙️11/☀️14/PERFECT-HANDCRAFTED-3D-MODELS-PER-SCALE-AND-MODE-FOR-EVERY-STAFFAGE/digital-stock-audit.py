"""📐 Flag changes in the minimum section of like-named physical parts with the same primitive across drawing scales."""
import json, math
from pathlib import Path

here=Path(__file__).resolve().parent
catalog=next(here.parents[6].glob('🎓*/*/*staffage/🗂*'))

def width(s):
    t=s['kind']
    if t=='box':return min(s['size'])
    if t=='ellipsoid':return 2*min(s['radii'])
    if t=='revolve':return min(max(p[0] for p in s['profile'])-min(p[0] for p in s['profile']),2*max(p[1] for p in s['profile'])*min(1,s.get('lateralScale',1)))
    if t=='capsule':return 2*max(s['r0'],s['r1'])*min(1,s.get('lateralScale',1))
    if t=='extrude':
        pts=s['profile']; spans=[]
        for j,(x,y) in enumerate(pts):
            dx,dy=pts[j-1][0]-x,pts[j-1][1]-y;l=math.hypot(dx,dy)
            if l:
                dist=[(dx*q[1]-dy*q[0])/l for q in pts];spans.append(max(dist)-min(dist))
        return min(s['to']-s['from'],min(spans,default=math.inf))
    if t=='loft':return 2*min(max(sec[k] for sec in s['sections']) for k in ('rx','ry'))
    return 0

if __name__=='__main__':
    rows=[]
    for p in catalog.glob('*/*/🔣️.json'):
        if not p.parent.parent.name.endswith(('furniture','objects','appliances')):continue
        a=json.loads(p.read_bytes())['asset'];fine={s['id']:s for s in a['volumes']['50']['body']}
        for s in a['volumes']['500']['body']:
            f=fine.get(s['id']);
            if not f or f['kind']!=s['kind'] or f['bone']!=s['bone'] or f['material']!=s['material']:continue
            w0,w1=width(f),width(s)
            if w0>.001 and w1>2*w0 and w1-w0>.018:rows.append((w1/w0,a['id'],s['id'],w0,w1))
    for r in sorted(rows,reverse=True):print(f'{r[1]:27} {r[2]:28} {1000*r[3]:7.1f} {1000*r[4]:7.1f}  {r[0]:5.1f}x')
