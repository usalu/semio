import os,sys,json,tomllib
R="/Users/ueli/Documents/semio"
SKIP={"node_modules","target",".git",".cache","🗑️generated","🎫️tickets","dist"}
mans=[]
for d,ds,fs in os.walk(R):
    ds[:]=[x for x in ds if x not in SKIP and not x.startswith(".cache")]
    if "Cargo.toml" in fs: mans.append(os.path.join(d,"Cargo.toml"))
data={}
for m in mans:
    try: data[m]=tomllib.load(open(m,"rb"))
    except Exception as e: print("PARSEFAIL",m,e)
pkg={}  # path dir -> name
byname={}
for m,t in data.items():
    n=t.get("package",{}).get("name")
    if n: pkg[os.path.dirname(m)]=n; byname.setdefault(n,[]).append(m)
# workspace roots
roots={m for m,t in data.items() if "workspace" in t}
def root_of(m):
    d=os.path.dirname(m)
    while d.startswith(R):
        c=os.path.join(d,"Cargo.toml")
        if c in roots: return c
        d=os.path.dirname(d)
def deptables(t):
    out=[]
    for k in ("dependencies","dev-dependencies","build-dependencies"):
        if k in t: out.append((k,t[k]))
    for tg,tv in t.get("target",{}).items():
        for k in ("dependencies","dev-dependencies","build-dependencies"):
            if k in tv: out.append((tg+"/"+k,tv[k]))
    if "workspace" in t and "dependencies" in t["workspace"]: out.append(("ws",t["workspace"]["dependencies"]))
    return out
issues=[]
for m,t in data.items():
    for tn,tab in deptables(t):
        for key,v in tab.items():
            real=key; path=None
            if isinstance(v,dict):
                real=v.get("package",key); path=v.get("path")
                if v.get("workspace"):
                    r=root_of(m); wt=data.get(r,{}).get("workspace",{}).get("dependencies",{}).get(key)
                    if wt is None: issues.append(("WS-MISSING-KEY",m,key,"")); continue
                    continue
            if path:
                tgt=os.path.normpath(os.path.join(os.path.dirname(m),path))
                n=pkg.get(tgt)
                if n is None: issues.append(("PATH-NO-PKG",m,key,tgt))
                elif n!=real: issues.append(("NAME-MISMATCH",m,key+"|"+real,n))
for r in roots:
    pass
json.dump(issues,open("census.json","w"),ensure_ascii=False,indent=0)
from collections import Counter
print(len(mans),len(issues),Counter(i[0] for i in issues))
print(Counter((i[0],i[2].split("|")[-1],i[3]) for i in issues if i[0]=="NAME-MISMATCH").most_common(40))
print(Counter((i[2]) for i in issues if i[0]=="WS-MISSING-KEY").most_common(40))
print([i for i in issues if i[0]=="PATH-NO-PKG"][:10])
