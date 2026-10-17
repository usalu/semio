import os,re,sys,json,collections,posixpath
G=os.environ["G"]
man=json.load(open(G+"/cpf-manifests.json",encoding="utf-8"))
rows=json.load(open(G+"/cpf-cfg-rows.json",encoding="utf-8"))
bypath={m["path"]:m for m in man if "name" in m and m["name"]}
byname=collections.defaultdict(list)
for m in bypath.values(): byname[m["name"]].append(m)
def roots(m):
    d=posixpath.dirname(m["path"])
    rs=set()
    for r in [m["lib"]]+m["tests"]+m["bins"]:
        if r: rs.add(posixpath.dirname(posixpath.normpath(posixpath.join(d,r))))
    return rs
croots=[(m,roots(m)) for m in bypath.values()]
def crates_of(p):
    best=[];bl=-1
    for m,rs in croots:
        for r in rs:
            if p==r or p.startswith(r+"/"):
                if len(r)>bl: best=[m]; bl=len(r)
                elif len(r)==bl and m not in best: best.append(m)
    return best
def reach_default(m):
    seen=set(); stack=["default"]
    while stack:
        f=stack.pop()
        if f in seen: continue
        seen.add(f)
        for x in m["features"].get(f,[]):
            if "/" not in x and not x.startswith("dep:"): stack.append(x)
    return seen
used=collections.defaultdict(set)   # (crate name, feature) -> files
nocrate=[]; undeclared=[]
for p,i,l,feats,inh in rows:
    cs=crates_of(p)
    for f in feats:
        if f=="testkit": continue
        if not cs: nocrate.append((p,f)); continue
        decl=[m for m in cs if f in m["features"]]
        if not decl: undeclared.append((p,f,[m["name"] for m in cs]))
        for m in (decl or cs): used[(m["name"],f)].add(p)
print("pairs",len(used),"nocrate",len(nocrate),"undeclared",len(undeclared))
for u in undeclared[:30]: print("UNDECLARED",u[0][-70:],u[1],u[2])
for n in nocrate[:10]: print("NOCRATE",n[0][-70:],n[1])
# default reach + enablers
nameset=set(byname)
viol=[]; defviol=[]
for (cname,f),files in sorted(used.items()):
    for m in byname[cname]:
        if f in reach_default(m): defviol.append((cname,f))
# enablers: other manifests depending on crate with features containing f, or forwarding crate/f in [features]
for mm in bypath.values():
    for d in mm["deps"]+[]:
        if d["name"] in nameset:
            for (cname,f) in used:
                if cname==d["name"] and f in d["features"] and d["section"]!="dev-dependencies":
                    viol.append((mm["name"],mm["role"],d["section"],cname,f,mm["path"]))
    for fk,fv in mm["features"].items():
        for x in fv:
            if "/" in x:
                dep,feat=x.split("/",1); dep=dep.rstrip("?")
                for d in mm["deps"]:
                    if d["key"]==dep and d["name"] in nameset and (d["name"],feat) in used:
                        viol.append((mm["name"],mm["role"],"forward:"+fk,d["name"],feat,mm["path"]))
print("default-reachable",len(defviol),defviol[:10])
print("non-dev enablers",len(viol))
for v in viol[:80]: print(v[:5])
json.dump(dict(used=[[k[0],k[1],sorted(v)] for k,v in used.items()],viol=viol,defviol=defviol,undeclared=undeclared,nocrate=nocrate),open(G+"/cpf-cfg-verify.json","w"),ensure_ascii=False)

print("=== refined")
TESTISH=re.compile(r"(-tests?(-|$)|-laws|-testing|test-support|-oracle|-fixtures?|-acceptance|-conformance|-companion)")
def testish(m): return m["role"]=="test" or bool(TESTISH.search(m["name"])) or "🧪️" in m["path"] or "🧷️companions" in m["path"]
A=[];B=[]
for mm in bypath.values():
    for d in mm["deps"]:
        for (cname,f) in used:
            if cname==d["name"] and f in d["features"] and d["section"]!="dev-dependencies" and not testish(mm):
                A.append((mm["name"],mm["role"],d["section"],cname,f,mm["path"]))
    dr=reach_default(mm)
    for fk,fv in mm["features"].items():
        for x in fv:
            if "/" in x:
                dep,feat=x.split("/",1); dep=dep.rstrip("?")
                for d in mm["deps"]:
                    if d["key"]==dep and d["name"] in nameset and (d["name"],feat) in used and fk in dr:
                        B.append((mm["name"],fk,d["name"],feat,mm["path"]))
print("A direct non-test enablers:",len(A))
for a in sorted(set(A)): print(" A",a)
print("B forwarders reachable from default:",len(B))
for b in sorted(set(B)): print(" B",b)
