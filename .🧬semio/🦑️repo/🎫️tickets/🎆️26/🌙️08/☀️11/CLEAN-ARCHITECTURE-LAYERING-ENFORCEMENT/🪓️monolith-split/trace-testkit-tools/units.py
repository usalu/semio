import os,sys,json
from targets import *
cs=crates()
mnt=mounts()
units={}
for f in mnt:
    best=None
    for c in cs:
        for root in [c["lib"]]+c["tests"]:
            dr=os.path.dirname(root)
            if os.path.dirname(f)==dr or f.startswith(dr+"/"):
                if best is None or len(dr)>len(best[2]): best=(c,root,dr)
    if f.endswith("🔤️base64/🦀️.rs"):
        c=[c for c in cs if c["name"]=="semio-framework-io-base64"][0]; best=(c,c["lib"],"")
    c,root,_=best
    if f in [x for x in c["tests"]]+[c["lib"]]: root=f
    elif f.endswith("🔤️base64/🦀️.rs"): root=c["lib"]
    else:
        if f not in c["tests"]: root=c["lib"]
    units.setdefault((c["name"],root),dict(name=c["name"],entry=root,pkg=c["dir"],mounts=[])) ["mounts"].append(f)
rows=sorted(units.values(),key=lambda r:(r["name"],r["entry"]))
json.dump(rows,open(sys.argv[1] if len(sys.argv)>1 else "units.json","w"),ensure_ascii=False,indent=1)
for r in rows: print(r["name"],"|",r["entry"],"|",len(r["mounts"]))
