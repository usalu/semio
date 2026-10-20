import sys,json
def rep(f, pairs):
    s=open(f,encoding='utf-8').read()
    for a,b in pairs:
        n=s.count(a)
        if n!=1: raise SystemExit(f"anchor count {n} for: {a[:100]}")
        s=s.replace(a,b)
    open(f,'w',encoding='utf-8').write(s)
