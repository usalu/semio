import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
pos=0;res={}
for m in re.finditer(r"^error: could not compile `([^`]+)`",t,re.M):
    seg=t[pos:m.start()];pos=m.end()
    errs=[]
    for e in re.finditer(r"^error(\[E\d+\])?: ([^\n]{0,150})\n\s*--> ([^\n]+)",seg,re.M):
        loc=e.group(3);loc=loc.split("/../")[-1] if "/../" in loc else loc
        errs.append((e.group(1) or "",e.group(2)[:110],loc[-75:]))
    res[m.group(1)]=errs
want=sys.argv[2:]
for k,v in res.items():
    if want and not any(w in k for w in want): continue
    print("==",k,len(v))
    seen=set()
    for x in v:
        if x in seen: continue
        seen.add(x);print(" ",*x)
