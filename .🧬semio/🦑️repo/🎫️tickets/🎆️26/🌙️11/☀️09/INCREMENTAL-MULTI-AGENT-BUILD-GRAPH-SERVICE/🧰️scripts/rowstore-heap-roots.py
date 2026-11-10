import json,sys,collections
d=json.load(open(sys.argv[1],encoding="utf8"))
meta=d["snapshot"]["meta"];nf=meta["node_fields"];ef=meta["edge_fields"];NF=len(nf);EF=len(ef)
nodes=d["nodes"];edges=d["edges"];strings=d["strings"]
ni={n:i for i,n in enumerate(nf)};ei={n:i for i,n in enumerate(ef)}
types=meta["node_types"][0];etypes=meta["edge_types"][0]
N=len(nodes)//NF
typ=lambda n:types[nodes[n*NF+ni["type"]]]
nm=lambda n:strings[nodes[n*NF+ni["name"]]]
sz=lambda n:nodes[n*NF+ni["self_size"]]
first=[0]*(N+1);c=0
for n in range(N):
    first[n]=c;c+=nodes[n*NF+ni["edge_count"]]
first[N]=c
succ=[[] for _ in range(N)]
for u in range(N):
    for k in range(first[u],first[u+1]):
        e=k*EF
        if etypes[edges[e+ei["type"]]]=="weak":continue
        succ[u].append(edges[e+ei["to_node"]]//NF)
# attribute each node to the set of "named roots": children of GC roots / module envs / globals, by multi-source BFS tagging first-reach
roots=succ[0]
print("root children",len(roots))
def rlabel(r):return typ(r)+":"+nm(r)[:50]
# exclusive+shared reach sizes per root child
reach_sz={}
owner=[-1]*N
import itertools
share=collections.Counter()
for r in roots:
    stack=[r];seen={r}
    tot=0
    while stack:
        u=stack.pop()
        tot+=sz(u)
        for v in succ[u]:
            if v not in seen and v!=0:
                seen.add(v);stack.append(v)
    reach_sz[r]=tot
top=sorted(reach_sz,key=lambda r:-reach_sz[r])[:40]
for r in top:print(round(reach_sz[r]/1048576,1),"MB",rlabel(r),"succ",len(succ[r]))
print("---- GC roots children classes")
gc=succ[0][0]
cc=collections.Counter();cs=collections.Counter()
for v in succ[gc]:
    k=typ(v)+":"+(nm(v)[:40] if typ(v)!="string" else "(string)");cc[k]+=1;cs[k]+=sz(v)
for k,n in cc.most_common(25):print(n,round(cs[k]/1048576,2),k)
print("---- string content classes among root-held strings")
sc=collections.Counter()
ex={}
for v in succ[gc]:
    if typ(v)=="string":
        s=nm(v);k=("path" if s.startswith("/") else "json" if s.startswith("[") else "other:"+s[:12])
        sc[k]+=1;ex.setdefault(k,s[:140])
for k,n in sc.most_common(15):print(n,k,ex[k])
