import json,sys,collections
r=[json.loads(l) for l in open(sys.argv[1])]
c=collections.Counter(); assets=collections.defaultdict(set)
rows=[]
for x in r:
    for f in x["geometryFindings"]+x["files"]["findings"]:
        c[f["check"]]+=1; assets[f["check"]].add(x["asset"]); rows.append((x["asset"],f))
print("assets",len(r)); print(dict(c)); print({k:len(v) for k,v in assets.items()})
for chk in sys.argv[2:]:
    for a,f in rows:
        if f["check"]==chk: print(chk,a,f["item"],f["message"][:200])
