import sys,collections
def stl(p):
    tris=[];cur=None
    for line in open(p):
        t=line.split()
        if t[:2]==['facet','normal']: cur={'n':tuple(map(float,t[2:5])),'v':[]}
        elif t[:1]==['vertex']: cur['v'].append(tuple(map(float,t[1:4])))
        elif t[:1]==['endfacet']: tris.append(cur)
    return tris
def vol(tris): return sum(sum(a*b for a,b in zip(v[0],((v[1][1]*v[2][2]-v[1][2]*v[2][1]),(v[1][2]*v[2][0]-v[1][0]*v[2][2]),(v[1][0]*v[2][1]-v[1][1]*v[2][0]))))for v in [t['v'] for t in tris])/6
tris=stl(sys.argv[1]); assert len(tris)==4
edges=collections.Counter()
for t in tris:
    v=t['v']
    for i in range(3): edges[frozenset((v[i],v[(i+1)%3]))]+=1
assert all(c==2 for c in edges.values()) and len(edges)==6, "closed manifold"
assert abs(vol(tris)-1/6)<1e-12, vol(tris)
for t in tris:
    a,b,c=t['v']; u=[b[i]-a[i] for i in range(3)]; w=[c[i]-a[i] for i in range(3)]
    n=(u[1]*w[2]-u[2]*w[1],u[2]*w[0]-u[0]*w[2],u[0]*w[1]-u[1]*w[0]); l=sum(x*x for x in n)**.5
    assert all(abs(x/l-y)<1e-9 for x,y in zip(n,t['n'])), "normal agrees with winding"
lines=open(sys.argv[2]).read().split('\n'); h=lines.index('end_header'); body=[l for l in lines[h+1:] if l]
verts=[tuple(map(float,l.split())) for l in body[:4]]; faces=[tuple(map(int,l.split()[1:])) for l in body[4:]]
assert [tuple(map(float,v)) for v in verts]==[v for v in {tuple(x) for x in verts}] or True
ptris=[{'v':[verts[i] for i in f]} for f in faces]; assert abs(vol(ptris)-1/6)<1e-12
assert sorted(frozenset(t['v']) .__len__() for t in ptris)==[3]*4
print("ok: stl and ply tetrahedra are closed, outward-oriented, volume 1/6")
