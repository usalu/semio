import json,sys,collections
d=json.load(open(sys.argv[1],encoding="utf8"))
meta=d["snapshot"]["meta"];nf=meta["node_fields"];ef=meta["edge_fields"];NF=len(nf);EF=len(ef)
nodes=d["nodes"];edges=d["edges"];strings=d["strings"]
ni={n:i for i,n in enumerate(nf)};ei={n:i for i,n in enumerate(ef)}
types=meta["node_types"][0];etypes=meta["edge_types"][0]
N=len(nodes)//NF
typ=lambda n:types[nodes[n*NF+ni["type"]]]
nm=lambda n:strings[nodes[n*NF+ni["name"]]]
first=[0]*(N+1);c=0
for n in range(N):
    first[n]=c;c+=nodes[n*NF+ni["edge_count"]]
first[N]=c
def ename(k):
    e=k*EF;t=etypes[edges[e+ei["type"]]];x=edges[e+ei["name_or_index"]]
    return t,(strings[x] if t in("property","internal","context","variable","shortcut") and isinstance(x,int) and x<len(strings) else str(x))
# shapes
def shape(n):
    return ",".join(sorted(ename(k)[1] for k in range(first[n],first[n+1]) if ename(k)[0]=="property"))
par=[-1]*N;pe=[None]*N;par[0]=0
from collections import deque
q=deque([0])
while q:
    u=q.popleft()
    for k in range(first[u],first[u+1]):
        t,nmx=ename(k)
        if t=="weak":continue
        v=edges[k*EF+ei["to_node"]]//NF
        if par[v]==-1:par[v]=u;pe[v]=(t,nmx);q.append(v)
def lab(n):return typ(n)+":"+(nm(n)[:40] if typ(n)!="string" else "(str)")
def path(n):
    out=[]
    while n!=0 and len(out)<30:
        out.append((lab(n),pe[n]));n=par[n]
    return out[::-1]
shapes=collections.defaultdict(list)
for n in range(N):
    if typ(n)=="object" and nm(n)=="Object":shapes[shape(n)].append(n)
want=sys.argv[2:]
for s,lst in sorted(shapes.items(),key=lambda x:-len(x[1]))[:int(want[0]) if want and want[0].isdigit() else 12]:
    print("##",len(lst),s[:150])
    # distinct anchor = first path element that is a named context/property from module env
    anchors=collections.Counter()
    for n in lst:
        p=path(n);key=" > ".join(f"{e[1]}" for l,e in p[:6] if e and e[1]!="")[:160]
        anchors[key]+=1
    for k,v in anchors.most_common(5):print("   ",v,k)
