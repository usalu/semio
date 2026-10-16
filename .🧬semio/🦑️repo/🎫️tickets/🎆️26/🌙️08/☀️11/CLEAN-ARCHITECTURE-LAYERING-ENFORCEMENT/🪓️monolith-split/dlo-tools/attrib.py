# -*- coding: utf-8 -*-
"""Cruiser output diff by rule: rows are keyed on the ASCII skeleton (the output has broken emoji bytes) and compared as multisets. Usage: attrib.py <before.txt> <after.txt> [--rows RULE]"""
import re,collections,sys
def parse(p):
    s=collections.Counter(); raw={}
    for l in open(p,encoding="utf8",errors="replace"):
        m=re.match(r'^\s*(error|warn) ([^:]+): (.*?) → (.*)$',l.rstrip("\n"))
        if m:
            k=tuple(re.sub('[^\\x00-\\x7f]','',x).strip() for x in (m.group(2),m.group(3),m.group(4)))
            s[k]+=1; raw.setdefault(k,(m.group(3),m.group(4)))
    return s,raw
a,ra=parse(sys.argv[1]); b,rb=parse(sys.argv[2])
rules=sorted({k[0] for k in list(a)+list(b)})
for r in rules:
    ta=sum(n for k,n in a.items() if k[0]==r); tb=sum(n for k,n in b.items() if k[0]==r)
    rem=sum(n-b.get(k,0) for k,n in a.items() if k[0]==r and b.get(k,0)<n)
    add=[(k,n-a.get(k,0)) for k,n in b.items() if k[0]==r and a.get(k,0)<n]
    if ta!=tb or rem or add: print(f"{r}: before={ta} after={tb} removed={rem} added={sum(n for _,n in add)}")
    if "--rows" in sys.argv and r==sys.argv[sys.argv.index("--rows")+1]:
        for k,n in add: print("   +",n,rb[k][0][-110:],"->",rb[k][1][-110:])
        for k,n in a.items():
            if k[0]==r and b.get(k,0)<n: print("   -",n-b.get(k,0),ra[k][0][-110:],"->",ra[k][1][-110:])
