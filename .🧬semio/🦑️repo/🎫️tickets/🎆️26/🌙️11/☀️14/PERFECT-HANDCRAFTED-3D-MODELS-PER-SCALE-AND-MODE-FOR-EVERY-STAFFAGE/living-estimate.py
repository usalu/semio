"""🍃 Count closed catalog surfaces without allocating mesh vertices or scene buffers."""
import json
import math
from pathlib import Path

root=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))

def triangles(s,segments):
    typ=s['kind']
    if typ=='polyhedron':return len(s['triangles'])
    if typ=='loft':
        rows=s['sections']
        n=segments(max(max(q['rx'],q['ry']) for q in rows))
        return n*(2*(len(rows)-1)+s.get('capStart',False)+s.get('capEnd',False))
    if typ=='ellipsoid':
        n=segments(max(s['radii']))
        return n*(2*math.ceil(n/2)-2)
    if typ=='extrude':
        holes=s.get('holes',[])
        return 4*(len(s['profile'])+sum(map(len,holes)))+4*len(holes)-4
    if typ=='box':
        radius=min(s.get('cornerRadius',0),min(s['size'])/2)
        if not radius: return 12
        n=math.ceil(segments(radius)/4)
        station=[2*n+(1 if sz/2>radius+1e-12 else 0) for sz in s['size']]
        return sum(4*station[i]*station[(i+1)%3] for i in range(3))
    if typ=='revolve':
        points=s['profile'][:]
        if points[0]!=points[-1]:
            if points[0][1]>0:points.insert(0,[points[0][0],0])
            if points[-1][1]>0:points.append([points[-1][0],0])
        n=segments(max(p[1] for p in points)*max(1,s.get('lateralScale',1)))
        return sum((int(a[1]>0)+int(b[1]>0))*n for a,b in zip(points,points[1:]))
    if typ=='capsule':
        a,b=s['r0'],s['r1']
        n=segments(max(a,b)*max(1,s.get('lateralScale',1)))
        q=math.ceil(n/4)
        d=math.dist(s['p0'],s['p1'])
        if d+min(a,b)<=max(a,b)+1e-12: return n*(4*q-2)
        beta=math.asin(max(-1,min(1,(a-b)/d)))
        left=math.ceil(q*(math.pi/2+beta)/(math.pi/2)) if a>0 else 0
        right=math.ceil(q*(math.pi/2-beta)/(math.pi/2)) if b>0 else 0
        return 2*n*(left+right)
    raise ValueError(typ)

def count(a,m,k,fixed=False):
    d=int(k)
    f=.00015*d
    chord=min(.02*f,max(a['dimensions'][field] for field in ('widthM','heightM','depthM'))/3200)
    cap={50:64,100:48,200:40,500:32}[d]
    def segments(r):
        if fixed:return 32
        n=math.ceil(math.pi/math.acos(1-min(1,chord/max(1e-9,r))))
        return min(cap,max(12,24 if r>f else 0,n))
    v=a['volumes'][k]
    body={s['id']:s for s in v['body']}
    solids=[body.get(s.get('of'),s) for s in v[m]]
    return sum(triangles(s,segments)*s.get('repeat',{}).get('count',1) for s in solids)

if __name__=='__main__':
    for p in sorted([*root.glob('*animals/*/*.json'),*root.glob('*vegetation/*/*.json')]):
        a=json.loads(p.read_text())['asset']
        for m in ('body','thermal','structure'):
            row=[count(a,m,str(k)) for k in (50,100,200,500)]
            if row[0]>395000 or any(j>i for i,j in zip(row,row[1:])): print('[DEBUG]',p.parent.name,m,row)

