import re,sys,glob
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R118/"
def load(d):
    r={}
    for f in glob.glob(SC+d+"/*"):
        k,v=open(f,encoding="utf8").read().split(" :",1);m=re.search(r"(\d+) pass",v);x=re.search(r"(\d+) fail",v);r[k.strip()]=(int(m.group(1)) if m else 0,int(x.group(1)) if x else 0)
    return r
a,b=load(sys.argv[1]),load(sys.argv[2]);bad=0
for k,(p,f) in sorted(b.items()):
    p0,f0=a.get(k,(0,99))
    tag="RED" if (p<p0 or f>f0) else "ok "
    if tag=="RED":bad=1
    if tag=="RED" or "--all" in sys.argv:print(tag,k.encode("utf8").decode()[-70:],(p0,f0),"->",(p,f))
print("compared",len(b),"of",len(a),"bad" if bad else "none worse")
sys.exit(bad)
