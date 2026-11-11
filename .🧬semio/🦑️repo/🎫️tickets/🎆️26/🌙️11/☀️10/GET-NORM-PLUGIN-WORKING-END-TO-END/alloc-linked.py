import sys,collections,re,subprocess
T=sys.argv[1];G=sys.argv[2]
src=open(T+"/alloc-static.py").read()
src=src[:src.index("for f in files:")]
exec(src)
removed=[l.split(" ")[1] for l in open(G+"/alloc-remove.log") if l.startswith("EDIT")]
removed+= [l.strip() for l in open(G+"/alloc-extra-removed.txt")]
removed=[os.path.abspath(f) for f in removed]
bad=[]
for f in removed:
    for r in sorted(owner.get(f,())):
        edges.clear();on=resolve(r)
        if A not in on:continue
        direct=edges.get(r,set())
        keys={}
        p,d=pk[r]
        for (dn,opt,feats,dflt,key,isdev) in deps_of(d,True,p):keys.setdefault(dn,set()).add(key.replace("-","_"))
        def reaches(x,seen=None):
            seen=seen or set()
            if x==A:return True
            if x in seen:return False
            seen.add(x)
            return any(reaches(y,seen) for y in edges.get(x,()))
        cand=[dn for dn in direct if reaches(dn)]
        txt="".join(open(g,errors="ignore").read() for g,o in owner.items() if r in o)
        ok=False
        for dn in cand:
            names=set(keys.get(dn,()))|{dn.replace("-","_")}
            lib=pk[dn][1].get("lib",{}).get("name")
            if lib:names.add(lib)
            if any(re.search(r'\b'+n+r'\b',txt) for n in names):ok=True;break
        if not ok:bad.append((r,f[-50:],cand))
seen=set()
for b in bad:
    if b[0] in seen:continue
    seen.add(b[0]);print("UNREFERENCED",b[0],b[2][:4])
print(len(seen),"of",len({r for f in removed for r in owner.get(f,())}))
