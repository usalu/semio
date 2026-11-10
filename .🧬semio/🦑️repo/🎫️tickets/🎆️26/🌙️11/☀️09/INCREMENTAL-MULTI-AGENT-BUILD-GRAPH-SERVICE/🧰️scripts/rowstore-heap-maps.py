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
pred=collections.defaultdict(list)
for u in range(N):
    for k in range(first[u],first[u+1]):
        e=k*EF;t=etypes[edges[e+ei["type"]]]
        v=edges[e+ei["to_node"]]//NF
        nameidx=edges[e+ei["name_or_index"]]
        en=strings[nameidx] if t in("property","internal","context","variable","shortcut") and isinstance(nameidx,int) and nameidx<len(strings) else str(nameidx)
        pred[v].append((u,t,en))
lab=lambda n:typ(n)+":"+nm(n)[:50]
big=sorted([n for n in range(N) if typ(n)=="object" and nm(n) in("Map","Set","WeakMap","Array")],key=lambda n:-sz(n))
# Map retained = butterfly sizes: use succ sizes
def total(n):return sz(n)+sum(sz(v) for (v,) in [(edges[k*EF+ei["to_node"]]//NF,) for k in range(first[n],first[n+1])])
tops=sorted(big,key=lambda n:-total(n))[:int(sys.argv[2]) if len(sys.argv)>2 else 40]
for n in tops:
    print(round(total(n)/1048576,2),"MB",lab(n),"edges",nodes[n*NF+ni["edge_count"]],"<-",[(lab(u),t,en) for u,t,en in pred[n][:4]])
