import sys
R="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/"
def sub(path,pairs,root=R):
    p=root+path
    s=open(p,encoding="utf-8").read()
    for o,n in pairs:
        c=s.count(o)
        if c!=1: raise SystemExit(f"{path}: {c} matches for {o[:90]!r}")
        s=s.replace(o,n)
    open(p,"w",encoding="utf-8").write(s)
