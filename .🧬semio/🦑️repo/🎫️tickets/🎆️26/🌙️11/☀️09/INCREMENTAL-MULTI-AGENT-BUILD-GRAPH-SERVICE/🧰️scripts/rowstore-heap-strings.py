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
inc=collections.defaultdict(list)
def shape(n):
    return ",".join(sorted(strings[edges[k*EF+ei["name_or_index"]]] for k in range(first[n],first[n+1]) if etypes[edges[k*EF+ei["type"]]]=="property"))[:60]
for u in range(N):
    for k in range(first[u],first[u+1]):
        v=edges[k*EF+ei["to_node"]]//NF
        if typ(v)=="string":
            t=etypes[edges[k*EF+ei["type"]]];x=edges[k*EF+ei["name_or_index"]]
            en=strings[x] if t=="property" else t
            inc[v].append((typ(u)+":"+(nm(u)[:20] if typ(u)!="string" else "s"),en,shape(u) if typ(u)=="object" and nm(u)=="Object" else ""))
cat=collections.Counter();bytes_=collections.Counter();ex={}
def cls(s):
    if s.startswith("["):return "json-array"
    if s.startswith("/"):return "abs-path"
    if s.startswith("{"):return "json-object"
    for p in("lstat ","exists ","field ","stat ","file ","readdir"):
        if s.startswith(p):return "key:"+p.strip()
    if len(s)==64 and all(ch in"0123456789abcdef" for ch in s):return "sha256"
    return "other"
tot=0
for n in range(N):
    if typ(n)!="string":continue
    s=nm(n);k=cls(s);cat[k]+=1;bytes_[k]+=sz(n);ex.setdefault(k,s[:120]);tot+=sz(n)
print("strings total MB",tot/1048576)
for k,v in cat.most_common():print(v,round(bytes_[k]/1048576,1),"MB",k,ex[k])
own=collections.Counter();ownb=collections.Counter()
for v,lst in inc.items():
    for key in set(lst):own[key]+=1;ownb[key]+=sz(v)
print("---- string incoming (owner,edge,shape)")
for k,v in sorted(own.items(),key=lambda x:-ownb[x[0]])[:30]:print(v,round(ownb[k]/1048576,2),"MB",k)
print("---- samples by class for array-held and root-held strings")
byc=collections.defaultdict(collections.Counter);exs={}
for v,lst in inc.items():
    s=nm(v);k=cls(s)
    for o in set(lst):
        byc[(o[0],o[1])][k]+=1;exs.setdefault(((o[0],o[1]),k),s[:150])
for key,cn in sorted(byc.items(),key=lambda x:-sum(x[1].values()))[:6]:
    print(key,dict(cn.most_common(5)))
    for k in list(cn)[:4]:print("    ",k,exs[(key,k)])
