import re,sys
def read(p):
    rows={}
    for l in open(p):
        m=re.match(r"(\S+)\s+pass=(-?\d+) fail=(-?\d+)",l)
        if m:rows[m[1]]=(int(m[2]),int(m[3]))
    return rows
a,b=read(sys.argv[1]),read(sys.argv[2]);bad=0
for k,(p,f) in b.items():
    pf=a.get(k,(0,0))
    if "fresh-namespace" in k or "selected-fleet" in k:
        if f!=0 or p<1:print("RED new law",k,p,f);bad=1
    elif f>pf[1] or p<pf[0]:print("RED",k,pf,"->",(p,f));bad=1
print("compare ok" if not bad else "compare RED");sys.exit(bad)
