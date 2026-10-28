import json,sys,os,difflib
C=sys.argv[1]; old=C+"/🗑️generated/sfrebuild/staged-before"; new=C+"/schemafirst-tools/staged"
def edits(d,b):
    m=json.load(open(f"{d}/{b}/manifest.json"))
    out={}
    for rel,r in m.items():
        f=open(f"{d}/{b}/files/{r['file']}",encoding="utf8").read().splitlines()
        bs=open(f"{d}/{b}/base/{r['file']}",encoding="utf8").read().splitlines() if r["base"] else []
        add=[];rem=[]
        for l in difflib.unified_diff(bs,f,lineterm="",n=0):
            if l.startswith("+++") or l.startswith("---") or l.startswith("@@"): continue
            (add if l[0]=="+" else rem).append(l[1:])
        out[rel]=(add,rem)
    return out
for b in open(C+"/schemafirst-tools/order.txt").read().split():
    o=edits(old,b); n=edits(new,b)
    if set(o)!=set(n): print(b,"ENTRY SET DIFFERS",set(o)^set(n))
    for rel in n:
        if rel not in o: continue
        oa,orm=o[rel]; na,nrm=n[rel]
        from collections import Counter
        da=Counter(na)-Counter(oa); dr=Counter(oa)-Counter(na)
        ra=Counter(nrm)-Counter(orm); rr=Counter(orm)-Counter(nrm)
        if da or dr or ra or rr:
            print("##",b,rel[-70:])
            for l in dr: print("   old+ only:",l[:160])
            for l in da: print("   new+ only:",l[:160])
            for l in rr: print("   old- only:",l[:160])
            for l in ra: print("   new- only:",l[:160])
