import json,sys,collections,re
d=json.load(open(sys.argv[1],encoding="utf8"))
meta=d["snapshot"]["meta"];nf=meta["node_fields"];NF=len(nf);ni={n:i for i,n in enumerate(nf)}
nodes=d["nodes"];strings=d["strings"];types=meta["node_types"][0]
N=len(nodes)//NF
cnt=collections.Counter();by=collections.Counter();dup=collections.Counter()
for n in range(N):
    if types[nodes[n*NF+ni["type"]]]!="string":continue
    t=strings[nodes[n*NF+ni["name"]]]
    L=len(t);b=("<32" if L<32 else "<64" if L<64 else "<128" if L<128 else "<256" if L<256 else "<1k" if L<1024 else "<16k" if L<16384 else ">=16k")
    cnt[b]+=1;by[b]+=L;dup[t]+=1
print("by length class (count, chars):")
for b in ["<32","<64","<128","<256","<1k","<16k",">=16k"]:print(b,cnt[b],by[b])
tot=sum(by.values());uniq=sum(len(t) for t in dup);print("total chars",tot,"distinct chars",uniq,"distinct",len(dup))
print("most duplicated (count, len, text):")
for t,k in sorted(dup.items(),key=lambda x:-x[1]*len(x[0]))[:12]:print(k,len(t),repr(t[:110]))
kinds=collections.Counter()
for t,k in dup.items():
    key="file-path" if t.startswith("/") else "json" if t[:1] in "{[" else "other"
    kinds[key]+=k*len(t)
print(kinds)
