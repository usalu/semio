#!/usr/bin/env python3
# usage: fixpaths.py <file>... : rewrites quoted repo-relative paths that do not exist but exist after inserting 🧪️tests/ before 🧫️fixtures/
import sys,re,os
FX="🧫️fixtures"
for f in sys.argv[1:]:
    s=open(f,encoding="utf-8").read(); n=0
    def rep(m):
        global n
        p=m.group(1)
        if os.path.exists(p): return m.group(0)
        q=p.replace("/"+FX+"/","/🧪️tests/"+FX+"/")
        if q!=p and os.path.exists(q): n+=1; return m.group(0).replace(p,q)
        return m.group(0)
    s2=re.sub(r'"((?:🧰️framework|🌎️hub|✏️s)/[^"\n]*?/'+FX+r'/[^"\n]*)"',rep,s)
    if n: open(f,"w",encoding="utf-8").write(s2)
    print(f[-80:],n)
