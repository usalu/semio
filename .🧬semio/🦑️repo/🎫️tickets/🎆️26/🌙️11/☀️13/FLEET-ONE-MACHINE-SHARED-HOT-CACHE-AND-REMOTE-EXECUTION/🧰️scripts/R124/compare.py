import sys,re
def load(p):
    d={}
    for l in open(p,encoding="utf8"):
        if "::" in l:
            k,v=l.rsplit("::",1);m=re.findall(r"(\d+) (pass|fail)",v);d[k.strip()]={b:int(a) for a,b in m}
    return d
b,a=load(sys.argv[1]),load(sys.argv[2]);bad=False
want={"🧮️lifecycle/🟦️.ts":(5,0),"🎛️one-budget/🟦️.ts":(8,0),"🌱️fresh/🧪️tests/🟦️.ts":(5,0),"🤝️pool-reader/🟦️.ts":(3,0)}
for k in sorted(set(a)|set(b)):
    x,y=b.get(k),a.get(k)
    exp=next(((p,0) for s,(p,_) in want.items() if k.endswith(s)),None)
    ok=y is not None and (y.get("fail",1)==0 or (x is not None and y["fail"]<=x.get("fail",0) and y["pass"]>=x["pass"] and exp is None)) and (exp is None and (x is None or y["pass"]>=x["pass"]) or exp is not None and y["pass"]==exp[0])
    print("OK " if ok else "RED",k[-60:],x,y)
    bad|=not ok
    if not ok:open(sys.argv[3] if len(sys.argv)>3 else "/dev/null","a",encoding="utf8").write(k.strip()+"\n")
sys.exit(1 if bad else 0)
