import sys
def load(p):
    d={}
    for l in open(p,encoding="utf8"):
        a=l.rstrip("\n").rsplit(" ",2)
        if len(a)==3: d[a[0]]=a
    return d
x=load(sys.argv[1]);y=load(sys.argv[2])
def n(v): return int(v) if v.isdigit() else -1
out=[]
for k in sorted(set(x)|set(y)):
    c=[r for r in (x.get(k),y.get(k)) if r]
    c.sort(key=lambda r:(n(r[2]),-n(r[1])))
    out.append(" ".join(c[0]))
open(sys.argv[3],"w",encoding="utf8").write("\n".join(out)+"\n")
