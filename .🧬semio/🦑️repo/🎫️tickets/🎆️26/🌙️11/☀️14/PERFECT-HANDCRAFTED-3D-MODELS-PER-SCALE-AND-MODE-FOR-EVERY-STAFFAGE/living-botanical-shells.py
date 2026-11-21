"""🍃 Author indexed apple crowns, palm vanes and folded bamboo blades with their original rig bindings."""
import json
import math
import sys
from pathlib import Path

root=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
ticket=Path(__file__).parent
def add(a,b): return [x+y for x,y in zip(a,b)]
def sub(a,b): return [x-y for x,y in zip(a,b)]
def mul(a,f): return [x*f for x in a]
def dot(a,b): return sum(x*y for x,y in zip(a,b))
def unit(a): return mul(a,1/math.hypot(*a))
def cross(a,b):
    x,y,z=a;u,v,w=b
    return [y*w-z*v,z*u-x*w,x*v-y*u]
def rotate(q,v):
    t=mul(cross(q[:3],v),2)
    return add(v,add(mul(t,q[3]),cross(q[:3],t)))
def mesh(s,rings):
    vertices=[]
    ids=[]
    for ring in rings:
        ids.append(list(range(len(vertices),len(vertices)+len(ring))))
        vertices.extend([[round(q,8) for q in p] for p in ring])
    tris=[]
    for first,second in zip(ids,ids[1:]):
        n=max(len(first),len(second))
        for j in range(n):
            a,b=first[j%len(first)],first[(j+1)%len(first)]
            c,d=second[j%len(second)],second[(j+1)%len(second)]
            if a!=b:tris.append([a,b,c])
            if c!=d:tris.append([b,d,c])
    for cap,is_start in ((ids[0],True),(ids[-1],False)):
        if len(cap)>1:
            c=len(vertices);vertices.append([round(sum(vertices[j][i] for j in cap)/len(cap),8) for i in range(3)])
            tris.extend([[c,cap[(j+1)%len(cap)],cap[j]] if is_start else [c,cap[j],cap[(j+1)%len(cap)]] for j in range(len(cap))])
    volume=sum(dot(sub(vertices[a],vertices[0]),cross(sub(vertices[b],vertices[0]),sub(vertices[c],vertices[0]))) for a,b,c in tris)
    if volume<0:tris=[v[::-1] for v in tris]
    for key in ('radii','sections','capStart','capEnd','plane','profile','from','to','taperTo'):s.pop(key,None)
    s.update(kind='polyhedron',vertices=vertices,triangles=tris)

def oval(s,rad,n,latitudes,fruit=False):
    x,y,z=rad
    rings=[[[0,0,-z]]]
    for angle in latitudes:
        h=math.sin(math.radians(angle)); c=math.cos(math.radians(angle))
        row=[]
        for j in range(n):
            theta=math.tau*j/n
            r=c*(1+.035*math.cos(5*theta)*abs(h)**2 if fruit else 1)
            row.append([x*r*math.cos(theta),y*r*math.sin(theta),z*h])
        rings.append(row)
    rings.append([[0,0,z]])
    mesh(s,rings)

def loft(s,n,stations=None,ends=False):
    sec=s['sections']
    if stations:sec=[sec[j] for j in stations]
    rings=[]
    for i,q in enumerate(sec):
        if ends and i in (0,len(sec)-1):
            rings.append([q['at']])
            continue
        tangent=unit(sub(sec[min(i+1,len(sec)-1)]['at'],sec[max(i-1,0)]['at']))
        if 'rotation' in q:
            x=rotate(q['rotation'],[1,0,0]);y=rotate(q['rotation'],[0,1,0]);z=rotate(q['rotation'],[0,0,1])
        else:
            perp=sub([1,0,0],mul(tangent,tangent[0]))
            if math.hypot(*perp)<1e-6:perp=sub([0,0,1],mul(tangent,tangent[2]))
            x=unit(perp);y=cross(tangent,x);z=tangent
        e=2/q.get('exponent',2)
        row=[]
        for j in range(n):
            theta=math.tau*j/n*(1 if dot(z,tangent)>=0 else -1)
            u,v=math.cos(theta),math.sin(theta)
            row.append(add(q['at'],add(mul(x,q['rx']*math.copysign(abs(u)**e,u)),mul(y,q['ry']*math.copysign(abs(v)**e,v)))))
        rings.append(row)
    mesh(s,rings)

def bamboo_blade(s):
    profile=s['profile'];h=len(profile)//2; rings=[]
    for j in (0,1,2,4,5):
        x,y=profile[j];u,v=profile[-1-j]
        mid=[(x+u)/2,(y+v)/2,0]
        if j in (0,5):rings.append([mid]);continue
        rings.append([[x,y,0],[mid[0],mid[1],s['to']],[u,v,0],[mid[0],mid[1],s['from']]])
    mesh(s,rings)

def author(name,k,s):
    d=int(k);typ=s.get('kind');mat=s.get('material','')
    if typ is None or typ=='polyhedron':return
    if name=='apple-tree':
        foliage='apple-leaf' in mat
        if foliage and typ=='loft' and all(q['at'][:2]==[0,0] for q in s['sections']):
            q=s['sections'];r=[max(p['rx'] for p in q),max(p['ry'] for p in q),max(abs(p['at'][2]) for p in q)]
        elif typ=='ellipsoid' and (foliage or mat=='apple-fruit' or s.get('part') in ('joint','mass')):
            r=s['radii']
        else:return
        n={50:16,100:12,200:12,500:8}[d] if foliage else {50:12,100:10,200:8,500:8}[d]
        lat={50:(-70,-46,-23,0,24,48,71),100:(-63,-33,0,34,65),200:(-54,-25,0,30,57),500:(-47,0,49)}[d]
        if s.get('part') in ('joint','mass'):lat=(-55,0,55)
        oval(s,r,n,lat,mat=='apple-fruit')
    if name=='date-palm':
        if typ=='loft' and s.get('capStart') and s.get('capEnd'):
            if 'dates-' in s['id']:loft(s,{50:8,100:8,200:6,500:6}[d],ends=True)
            elif mat=='palm-frond':loft(s,{50:8,100:6,200:6,500:4}[d])
            elif mat in ('palm-stem-bark','palm-wood','date-fruit'):loft(s,{50:16,100:12,200:10,500:8}[d])
        elif typ=='ellipsoid' and (s.get('part') in ('joint','mass') or mat=='date-fruit'):oval(s,s['radii'],8,(-50,0,50))
    if name=='bamboo-clump':
        if typ=='extrude' and mat=='bamboo-leaf' and s.get('role')=='texture' and len(s['profile'])==12:
            bamboo_blade(s)
        elif typ=='loft' and s.get('capStart') and s.get('capEnd'):
            if mat=='bamboo-leaf' and s.get('role')=='texture' and len(s['sections'])==6:loft(s,4,(0,1,2,4,5),True)
            elif mat=='bamboo-leaf':loft(s,{50:10,100:8,200:8,500:6}[d])
            elif mat=='bamboo-culm':loft(s,{50:12,100:10,200:8,500:6}[d] if len(s['sections'])>2 else 6)
        elif typ=='ellipsoid' and s.get('part') in ('joint','mass'):oval(s,s['radii'],8,(-50,0,50))

for name in sys.argv[1:]:
    for p in (next(root.glob(f'*/{name}/*.json')),ticket/'volumes/vegetation'/f'{name}.json'):
        raw=p.read_text();doc=json.loads(raw);a=doc.get('asset',doc)
        for k,v in a['volumes'].items():
            for mode in ('body','thermal','structure'):
                for s in v[mode]:author(name,k,s)
        start=raw.index('{',raw.index('"volumes":'));old,n=json.JSONDecoder().raw_decode(raw[start:])
        if old==a['volumes']:continue
        if p.read_text()!=raw:raise SystemExit(f'{name} changed concurrently')
        depth='    ' if 'asset' in doc else '  '
        patch=json.dumps(a['volumes'],ensure_ascii=False,indent=2).replace('\n','\n'+depth) if raw.startswith('{\n') else json.dumps(a['volumes'],ensure_ascii=False,separators=(',',':'))
        p.write_text(raw[:start]+patch+raw[start+n:])
        print('[DEBUG]',name,p.name)

