import re,sys,collections
log=sys.argv[1]; want=sys.argv[2] if len(sys.argv)>2 else None; lim=int(sys.argv[3]) if len(sys.argv)>3 else 40
cur=[];out=collections.OrderedDict()
for l in open(log,errors="replace"):
    l=l.rstrip()
    m=re.match(r"(.*?\.rs):(\d+):(\d+): error(\[E\d+\])?: (.*)",l)
    if m: cur.append((m.group(1),m.group(2),m.group(3),m.group(4) or "",m.group(5))); continue
    m=re.match(r"error: could not compile `([^`]+)` \((lib|lib test|[^)]*)\)",l)
    if m:
        k=(m.group(1),m.group(2))
        if k not in out: out[k]=cur
        cur=[]
if not want:
    for k,v in out.items(): print(k,len(v))
else:
    for k,v in out.items():
        if k[0]==want and (len(sys.argv)<5 or k[1]==sys.argv[4]):
            seen=set()
            for f,a,b,e,msg in v:
                if (f,a,b,msg) in seen: continue
                seen.add((f,a,b,msg))
                if len(seen)<=lim: print(re.sub(r".*📦️packages/🦀️rust/","",f)[-85:],a+":"+b,e,msg[:220])
            print("unique",len(seen),k)
