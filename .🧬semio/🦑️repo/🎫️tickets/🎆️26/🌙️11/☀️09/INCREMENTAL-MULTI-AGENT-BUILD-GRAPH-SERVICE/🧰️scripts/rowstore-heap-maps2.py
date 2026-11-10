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
coll=[n for n in range(N) if typ(n)=="object" and nm(n) in("Map","Set","WeakMap","WeakSet")]
cs=set(coll)
names=collections.defaultdict(list)
for u in range(N):
    for k in range(first[u],first[u+1]):
        e=k*EF;v=edges[e+ei["to_node"]]//NF
        if v in cs:
            t=etypes[edges[e+ei["type"]]];x=edges[e+ei["name_or_index"]]
            if t in("context","property","variable") and isinstance(x,int) and x<len(strings):names[v].append(strings[x])
def tot(n):return sz(n)+sum(sz(edges[k*EF+ei["to_node"]]//NF) for k in range(first[n],first[n+1]))
rows=sorted(((tot(n),nm(n),",".join(sorted(set(names[n])))[:60]) for n in coll),reverse=True)
agg=collections.defaultdict(lambda:[0,0])
for t,k,nmx in rows:
    a=agg[(k,nmx)];a[0]+=t;a[1]+=1
print("collections",len(coll),"MB",sum(r[0] for r in rows)/1048576)
for (k,nmx),(t,c) in sorted(agg.items(),key=lambda x:-x[1][0])[:45]:print(round(t/1048576,2),"MB",c,"x",k,nmx)
