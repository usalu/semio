#!/usr/bin/env python3
# usage: stale_paths.py <fixture.json> ...  lists repo-path-looking strings that do not exist and a candidate remap (fixtures under tests)
import json,os,sys
ROOT="/Users/ueli/Documents/semio"
def walk(o,path=""):
    if isinstance(o,dict):
        for k,v in o.items(): yield from walk(v,path+"/"+str(k))
    elif isinstance(o,list):
        for i,v in enumerate(o): yield from walk(v,path+"/%d"%i)
    elif isinstance(o,str): yield path,o
def candidates(s):
    out=[]
    if "/🧫️fixtures" in s and "/🧪️tests/🧫️fixtures" not in s: out.append(s.replace("/🧫️fixtures","/🧪️tests/🧫️fixtures",1))
    return out
for f in sys.argv[1:]:
    d=json.load(open(f,encoding="utf-8"))
    n=0
    for p,s in walk(d):
        if "/" not in s or len(s)<12: continue
        tops=("🧰️","✏️s","🌎️","🦑️","💻️")
        if not s.startswith(tops): continue
        if os.path.lexists(os.path.join(ROOT,s)): continue
        c=[x for x in candidates(s) if os.path.lexists(os.path.join(ROOT,x))]
        n+=1
        print(("REMAP " if c else "GONE  ")+p+" "+s[-110:]+(" -> ..."+c[0][-60:] if c else ""))
    print(f[-60:],"stale",n)
