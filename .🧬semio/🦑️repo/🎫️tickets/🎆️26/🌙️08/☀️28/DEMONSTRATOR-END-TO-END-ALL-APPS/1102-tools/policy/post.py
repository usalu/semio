import sys,re
sys.path.insert(0,".")
from mk import R,crate_of
import a,b,c,d,f,g,h,i
items=[]
for n,(spec,_) in a.S.items():
    for rel,e in spec.items():items.append((rel,e))
items+=[(r,e) for r,e,_ in b.B+c.C+d.D+f.F+g.G+h.H+i.I]
pat=re.compile(r'\.unwrap\(\)|\.expect\(|panic!\(|unreachable!')
for rel,edits in items:
    if not rel.endswith(".rs"):continue
    s=open(f"{R}/{rel}",encoding="utf8").read();t=s
    for e in edits:t=t.replace(e[0],e[1])
    S0=set(s.split("\n"))
    for ln in t.split("\n"):
        if ln not in S0 and pat.search(ln):
            for m in pat.finditer(ln):print(crate_of(rel)[-34:],"|",rel.split("/")[-2][:12]+"/"+rel.split("/")[-1][:12],"|",ln[max(0,m.start()-100):m.end()+50])
