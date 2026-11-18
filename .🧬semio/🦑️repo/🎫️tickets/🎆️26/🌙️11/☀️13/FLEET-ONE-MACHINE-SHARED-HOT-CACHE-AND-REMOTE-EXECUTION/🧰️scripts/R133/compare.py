import sys
def load(p):
    d={}
    for l in open(p,encoding="utf8"):
        parts=l.rstrip("\n").rsplit(" ",2)
        if len(parts)==3: d[parts[0]]=(parts[1],parts[2])
    return d
b=load(sys.argv[1]);a=load(sys.argv[2]);worse=[]
for k,(p,f) in a.items():
    if k not in b: continue
    bp,bf=b[k]
    def n(x): return int(x) if x.isdigit() else -1
    if n(f)>n(bf) or (n(p)<n(bp) and n(bp)>=0): worse.append((k,b[k],a[k]))
for w in worse: print("WORSE",w)
print("compared",len([k for k in a if k in b]),"worse",len(worse))
sys.exit(1 if worse else 0)
