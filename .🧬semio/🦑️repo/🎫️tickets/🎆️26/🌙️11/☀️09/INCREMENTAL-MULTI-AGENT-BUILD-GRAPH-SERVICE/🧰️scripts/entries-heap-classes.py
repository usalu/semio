import json,sys,collections
d=json.load(open(sys.argv[1],encoding="utf8"))
meta=d["snapshot"]["meta"];nf=meta["node_fields"];NF=len(nf);ni={n:i for i,n in enumerate(nf)}
nodes=d["nodes"];strings=d["strings"];types=meta["node_types"][0]
N=len(nodes)//NF;c=collections.Counter();s=collections.Counter()
for n in range(N):
    t=types[nodes[n*NF+ni["type"]]];nm=strings[nodes[n*NF+ni["name"]]] if t!="string" else "(string)"
    k=t+":"+nm[:40];c[k]+=1;s[k]+=nodes[n*NF+ni["self_size"]]
print("nodes",N,"MB",round(sum(s.values())/1048576,1))
for k,v in s.most_common(25):print(round(v/1048576,1),"MB",c[k],k)
