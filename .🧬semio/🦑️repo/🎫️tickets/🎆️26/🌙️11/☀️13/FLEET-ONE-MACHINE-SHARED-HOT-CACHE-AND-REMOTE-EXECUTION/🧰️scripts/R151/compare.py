import sys,re
def parse(p):
    out={}
    for l in open(p,encoding="utf8"):
        name,_,rest=l.partition(" :: ")
        m=re.search(r"(\d+) pass",rest);f=re.search(r"(\d+) fail",rest)
        out[name.strip()]=(int(m[1]) if m else 0,int(f[1]) if f else 99)
    return out
base,after=parse(sys.argv[1]),parse(sys.argv[2])
expect_green=["🧪️tests/🕸️graph/🩹️repair-rounds","🧪️tests/🕸️graph/🧵️retain-rows","🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🕰️stale-stage"]
bad=[]
for k,(p,f) in after.items():
    bp,bf=base.get(k,(0,99))
    ok=(f==0) if k in expect_green else (f<=bf and p>=bp)
    print(("OK  " if ok else "BAD ")+k,"base",(bp,bf),"after",(p,f))
    if not ok: bad.append(k)
sys.exit(1 if bad else 0)
