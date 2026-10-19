import re,sys
def parse(path):
    t=open(path,encoding="utf-8").read()
    head,_,rest=t.partition("\n[[package]]")
    blocks=("[[package]]"+rest).split("\n[[package]]") if rest else []
    blocks=[b if b.startswith("[[package]]") else "[[package]]"+b for b in blocks]
    pk=[]
    for b in blocks:
        b=b.rstrip("\n")
        name=re.search(r'^name = "(.*)"',b,re.M).group(1)
        ver=re.search(r'^version = "(.*)"',b,re.M).group(1)
        src=re.search(r'^source = "(.*)"',b,re.M)
        deps=[]
        m=re.search(r'^dependencies = \[\n(.*?)^\]',b,re.M|re.S)
        if m: deps=re.findall(r'^ "(.*)",?$',m.group(1),re.M)
        pk.append(dict(name=name,ver=ver,src=src.group(1) if src else None,deps=deps,text=b))
    return head.rstrip("\n")+"\n",pk
def resolve(pk):
    by={}
    for i,p in enumerate(pk): by.setdefault(p["name"],[]).append(i)
    def r(d):
        parts=d.split(" ")
        c=by[parts[0]]
        if len(parts)>1: c=[i for i in c if pk[i]["ver"]==parts[1]]
        if len(parts)>2:
            s=parts[2].strip("()"); c=[i for i in c if pk[i]["src"]==s]
        assert len(c)==1,(d,c)
        return c[0]
    return [[r(d) for d in p["deps"]] for p in pk]
def reach(edges,roots):
    seen=set(); st=list(roots)
    while st:
        n=st.pop()
        if n in seen: continue
        seen.add(n); st.extend(edges[n])
    return seen
def write(path,head,pk,keep):
    out=head+"".join("\n"+pk[i]["text"]+"\n" for i in sorted(keep,key=lambda i:(pk[i]["name"],pk[i]["ver"],pk[i]["src"] or "")))
    open(path,"w",encoding="utf-8").write(out)
