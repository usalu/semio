import re,sys
t=open(sys.argv[1],errors="ignore").read()
want=sys.argv[2] if len(sys.argv)>2 else None
lines=t.split("\n");cur=[];out=[]
i=0
blocks=re.split(r"\n(?=error(?:\[E\d+\])?: |warning: )",t)
pend=[]
for b in blocks:
    ls=b.split("\n")
    if ls[0].startswith("error: could not compile"):
        m=re.search(r"`([^`]+)` \(([^)]*)\)",ls[0]);name=m.group(1)+" "+m.group(2) if m else ls[0]
        out.append((name,pend));pend=[];continue
    if ls[0].startswith("error") and len(ls)>1 and "-->" in ls[1]:
        pend.append(b)
for name,es in out:
    if want and want not in name: continue
    print("#####",name,len(es))
    if want:
        for b in es:
            ls=b.split("\n");print(ls[0][:160]);print("  ",ls[1].split("rust/")[-1].strip()[:200])
            for l in ls[2:12]:
                if "^" in l and len(l)<600: print("    ",l.strip()[:300])
