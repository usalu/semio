#!/usr/bin/env python3
"""Lists package.json `exports` targets that do not exist on disk, grouped by whether the target lies inside a gitignored build-output directory (dist, pkg, 🤖️generated)."""
import json,os,subprocess,sys
root=os.getcwd()
files=subprocess.run(["git","ls-files","-co","--exclude-standard","-z"],capture_output=True,cwd=root).stdout.decode().split("\0")
pk=[f for f in files if f.endswith("package.json") and not f.startswith(".🧬semio/") and "node_modules" not in f and os.path.exists(f)]
def targets(v):
    if isinstance(v,str):yield v
    elif isinstance(v,dict):
        for x in v.values():yield from targets(x)
    elif isinstance(v,list):
        for x in v:yield from targets(x)
rows=[]
for f in pk:
    try:j=json.load(open(f))
    except Exception:continue
    ex=j.get("exports")
    if not ex:continue
    d=os.path.dirname(f)
    for k,v in (ex.items() if isinstance(ex,dict) and all(x.startswith(".") for x in ex) else [(".",ex)]):
        for t in targets(v):
            if not t.startswith("./") or "*" in t:continue
            p=os.path.join(d,t)
            if not os.path.exists(p):
                rows.append((j.get("name"),k,t,("build-output" if any(s in t for s in ("/dist/","/pkg/","🤖️generated","/target/")) else "SOURCE")))
import collections
c=collections.Counter(r[3] for r in rows);print(dict(c))
for r in rows:
    if r[3]=="SOURCE":print(r)
