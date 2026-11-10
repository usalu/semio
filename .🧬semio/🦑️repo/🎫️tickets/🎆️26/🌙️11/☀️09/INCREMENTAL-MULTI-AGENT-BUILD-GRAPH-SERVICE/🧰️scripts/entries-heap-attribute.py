import json,sys,collections
d=json.load(open(sys.argv[1],encoding="utf8"))
meta=d["snapshot"]["meta"];nf=meta["node_fields"];ef=meta["edge_fields"];NF=len(nf);EF=len(ef)
nodes=d["nodes"];edges=d["edges"];strings=d["strings"]
ni={n:i for i,n in enumerate(nf)};ei={n:i for i,n in enumerate(ef)}
types=meta["node_types"][0];etypes=meta["edge_types"][0]
N=len(nodes)//NF
first=[0]*(N+1);c=0
for n in range(N):
    first[n]=c;c+=nodes[n*NF+ni["edge_count"]]
first[N]=c
self_size=[nodes[n*NF+ni["self_size"]] for n in range(N)]
name=[strings[nodes[n*NF+ni["name"]]] for n in range(N)]
ntype=[types[nodes[n*NF+ni["type"]]] for n in range(N)]
parent=[-1]*N;pedge=[""]*N
# BFS from root 0 to assign first retainer by edge name (shortest path)
from collections import deque
q=deque([0]);seen=[False]*N;seen[0]=True
while q:
    u=q.popleft()
    for k in range(first[u],first[u+1]):
        e=k*EF;t=etypes[edges[e+ei["type"]]]
        if t=="weak":continue
        to=edges[e+ei["to_node"]]//NF
        if not seen[to]:
            seen[to]=True;parent[to]=u
            en=edges[e+ei["name_or_index"]]
            pedge[to]=strings[en] if t in("property","internal","context","shortcut") and isinstance(en,int) and en<len(strings) else "[]"
            q.append(to)
agg=collections.Counter();cnt=collections.Counter()
def sig(n):
    parts=[];u=n;depth=0
    while u>0 and depth<40:
        p=parent[u]
        if p<0:break
        if ntype[p] in("object","closure","native") and name[p] not in("Object","Array","Map","Set","(Internal)") or p==0:
            pass
        parts.append(pedge[u]);u=p;depth+=1
    return ">".join(reversed(parts[-5:]))  # last (closest to root)
# attribute by the path from nearest named holder: use top 4 edge names from the root
def top(n):
    parts=[];u=n
    while u>0:
        parts.append(pedge[u]);u=parent[u]
        if u<0:break
    parts.reverse()
    return ">".join(parts[:6])
for n in range(N):
    key=top(n);agg[key]+=self_size[n];cnt[key]+=1
total=sum(self_size)
print("total self bytes",total,"nodes",N)
for k,v in agg.most_common(40):print(round(v/1048576,1),"MB",cnt[k],k[:200])
