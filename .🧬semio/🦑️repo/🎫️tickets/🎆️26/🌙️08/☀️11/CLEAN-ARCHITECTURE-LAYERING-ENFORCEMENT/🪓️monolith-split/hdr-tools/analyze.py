import sys,re,json,os
F=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
G=F+"/🗑️generated"
cur=None;mode=None;data={}
for line in open(G+"/hdr-mine-closures.txt"):
    line=line.rstrip("\n")
    if line.startswith("## "): cur=line[3:]; data[cur]={'lib':[],'test':[]}; continue
    if line.startswith("# lib"): mode='lib'; continue
    if line.startswith("# test"): mode='test'; continue
    if line.startswith("# ERROR"): continue
    k,b,l,p=line.split("\t",3)
    data[cur][mode].append((k,int(b),int(l),p))
json.dump(data,open(G+"/hdr-mine-closures.json","w"),ensure_ascii=False)
def ext(p):
    base=p.split("/")[-1]
    return base
rows=[]
for n,d in data.items():
    L=d['lib'];T=d['test']
    ls=sum(1 for x in L if x[0]=='source'); le=len(L)-ls
    ts=sum(1 for x in T if x[0]=='source'); te=len(T)-ts
    # embedded .rs (rust facets) / ts / json etc
    emb=[x for x in L if x[0]=='embedded']
    kinds={}
    for x in emb:
        e=x[3].split("/")[-1]
        kinds[e]=kinds.get(e,0)+1
    rows.append((n,len(L),ls,le,len(T),ts,te,sum(x[1] for x in L if x[0]=='embedded'),sum(x[1] for x in T if x[0]=='embedded')))
rows.sort(key=lambda r:-max(r[1],r[4]))
print("crate lib(src/emb) test(src/emb) embB_lib embB_test")
for r in rows: print(r[0].replace('semio-s-artifact-','A:').replace('semio-framework-','F:'),f"{r[1]}({r[2]}/{r[3]})",f"{r[4]}({r[5]}/{r[6]})",r[7],r[8])
