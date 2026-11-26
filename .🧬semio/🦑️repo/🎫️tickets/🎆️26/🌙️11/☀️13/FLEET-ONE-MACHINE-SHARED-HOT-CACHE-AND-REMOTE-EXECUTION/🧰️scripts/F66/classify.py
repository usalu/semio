#!/usr/bin/env python3
"""F66: buckets the tsc errors of a run into causes (declaration-backed = the erroring file imports a .mjs that has a hand-written sibling .d.mts)."""
import re,sys,collections;sys.path.insert(0,__import__("os").path.dirname(__file__))
from lib import *
imp=re.compile(r'from\s*"([^"]+\.mjs)"|import\("([^"]+\.mjs)"\)')
cache={}
def backed(f):
    if f in cache:return cache[f]
    s=open(f,encoding="utf8").read();r=False
    for m in imp.finditer(s):
        p=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1) or m.group(2)))
        if os.path.exists(p[:-4]+".d.mts"):r=True;break
    cache[f]=r;return r
c=collections.Counter();tot=0
byc=collections.Counter()
for f,ln,col,code,msg in errors(sys.argv[1]):
    tot+=1;byc[code]+=1
    c["declaration-backed" if backed(f) else "test-local/other"]+=1
print(tot,dict(c));print(byc.most_common(12))
