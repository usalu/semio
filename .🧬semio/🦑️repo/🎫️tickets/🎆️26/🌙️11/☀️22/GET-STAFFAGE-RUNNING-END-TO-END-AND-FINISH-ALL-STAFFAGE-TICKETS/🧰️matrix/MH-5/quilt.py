"""🛌️ The flat quilt baffles of `sleeping-person` 1:50 as skinned lofts: quilt.py <asset.json> <stations> <exponent> <axis u|w> prints `add` ops (fit.ts) that draw every baffle which is flat in the side or the front elevation as one `loft` shape (D54) with the skin weights of its body: the polyhedron is cut at equal steps along the axis and every cut becomes the superellipse of its box."""
import json,sys
a=json.load(open(sys.argv[1]))["asset"]; n=int(sys.argv[2]); ex=float(sys.argv[3]); axis=sys.argv[4]
k=0 if axis=="u" else 2; o=2 if k==0 else 0
g=a["representations"]["50"]["geometry"]; body=a["volumes"]["50"]["body"]
rot=[0.5,0.5,0.5,0.5] if axis=="u" else [0,0,0,1]
ids=sorted({s["id"] for v in("elevation","front") for s in g[v] if s.get("bone","").startswith("quilt") and "solid" not in s})
ops=[]
for i in ids:
    d=next(x for x in body if x["id"]==i); V=d["vertices"]; F=d["triangles"]
    lo,hi=min(p[k] for p in V),max(p[k] for p in V); ins=(hi-lo)/(8*n); sec=[]
    for s in range(n+1):
        t=lo+ins if s==0 else hi-ins if s==n else lo+(hi-lo)*s/n; cut=[]
        for f in F:
            for x,y in zip(f,f[1:]+f[:1]):
                p,q=V[x],V[y]
                if (p[k]-t)*(q[k]-t)<=0 and p[k]!=q[k]:
                    r=(t-p[k])/(q[k]-p[k]); cut.append((p[1]+r*(q[1]-p[1]),p[o]+r*(q[o]-p[o])))
        v0,v1=min(c[0] for c in cut),max(c[0] for c in cut); w0,w1=min(c[1] for c in cut),max(c[1] for c in cut)
        at=[t,(v0+v1)/2,(w0+w1)/2] if k==0 else [(w0+w1)/2,(v0+v1)/2,t]
        rv,rw=max((v1-v0)/2,1e-3),max((w1-w0)/2,1e-3)
        sec.append({"at":[round(x,5) for x in at],"rx":round(rv if k==0 else rw,5),"ry":round(rw if k==0 else rv,5),"exponent":ex,"rotation":rot})
    for v in("elevation","front","plan"):
        sh=next(x for x in g[v] if x["id"]==i)
        new={"id":i,"kind":"loft","bone":sh["bone"],**{q:sh[q] for q in("role","z","filled","tone","region","material","penMm") if q in sh},"sections":sec,"weights":d["weights"]}
        ops.append({"op":"add","scale":"50","view":v,"id":i,"why":"the baffle as one skinned loft in the three views (quilt.py): the flat outline of the silhouette round was not carried by the skin weights","shape":new})
print(json.dumps(ops))
