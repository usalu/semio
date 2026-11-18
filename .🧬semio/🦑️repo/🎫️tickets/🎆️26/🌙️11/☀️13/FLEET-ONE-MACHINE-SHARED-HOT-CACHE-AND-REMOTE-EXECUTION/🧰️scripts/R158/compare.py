import sys
def load(p):
    r={}
    for l in open(p,encoding="utf8"):
        d,a,b,*_=l.rstrip("\n").split("|")
        r[d]=(int(a.replace("pass","") or 0),int(b.replace("fail","") or 0))
    return r
b,a=load(sys.argv[1]),load(sys.argv[2]);bad=[]
for d,(p,f) in a.items():
    if d in b and (f>b[d][1] or p<b[d][0]): bad.append((d,b[d],(p,f)))
must=["heavy-refusal","never-here","too-heavy","🧯️failure","🌐️host","doomed-handoff","plan-refusal","roster-scope"]
for d,(p,f) in a.items():
    if any(m in d for m in must) and f!=0: bad.append((d,"must be green",(p,f)))
print("worse:",bad);sys.exit(1 if bad else 0)
