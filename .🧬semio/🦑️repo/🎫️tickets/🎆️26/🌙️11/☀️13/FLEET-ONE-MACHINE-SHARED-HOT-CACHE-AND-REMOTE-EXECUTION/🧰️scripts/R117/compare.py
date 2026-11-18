import re,sys
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R117/"
def load(n):
    d={}
    for l in open(SC+n+".txt"):
        if " :" in l:
            k,v=l.split(" :",1);m=re.search(r"(\d+) pass",v);f=re.search(r"(\d+) fail",v);d[k.strip()]=(int(m.group(1)) if m else 0,int(f.group(1)) if f else 0)
    return d
NEW={"essential":6,"first-child":5,"owner-retained":2}
a,b=load(sys.argv[1]),load(sys.argv[2]);bad=0
for k,(p,f) in a.items():
    p2,f2=b.get(k,(0,99))
    if any(n in k for n in NEW): continue
    if p2<p or f2>f: print("RED",k,(p,f),"->",(p2,f2));bad=1
for name,w in NEW.items():
    got=[v for k,v in b.items() if name in k]
    if not got or got[0][1]!=0 or got[0][0]<w: print("RED new test",name,got);bad=1
sys.exit(bad)
