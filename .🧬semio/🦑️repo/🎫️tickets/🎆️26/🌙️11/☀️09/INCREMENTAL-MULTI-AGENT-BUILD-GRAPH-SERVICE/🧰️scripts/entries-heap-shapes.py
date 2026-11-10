import json,sys,collections
d=json.load(open(sys.argv[1],encoding="utf8"))
meta=d["snapshot"]["meta"];nf=meta["node_fields"];ef=meta["edge_fields"];NF=len(nf);EF=len(ef)
nodes=d["nodes"];edges=d["edges"];strings=d["strings"]
ni={n:i for i,n in enumerate(nf)};ei={n:i for i,n in enumerate(ef)}
types=meta["node_types"][0];etypes=meta["edge_types"][0]
N=len(nodes)//NF;c=0;cnt=collections.Counter();firsts=0
pos=0
for n in range(N):
    ec=nodes[n*NF+ni["edge_count"]]
    t=types[nodes[n*NF+ni["type"]]];nm=strings[nodes[n*NF+ni["name"]]]
    if t=="object" and nm=="Object":
        names=[]
        for k in range(pos,pos+ec):
            e=k*EF
            if etypes[edges[e+ei["type"]]]=="property":names.append(strings[edges[e+ei["name_or_index"]]])
        cnt[",".join(sorted(names))[:120]]+=1
    pos+=ec
for k,v in cnt.most_common(14):print(v,k)
