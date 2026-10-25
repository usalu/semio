import json,sys,collections,os,re
log=sys.argv[1]; apply=len(sys.argv)>2 and sys.argv[2]=="apply"
only=set(sys.argv[3].split(",")) if len(sys.argv)>3 else None
sug=collections.defaultdict(set)   # file -> {(start,replacement)}
amb=[];units=collections.defaultdict(int)
for line in open(log,errors="replace"):
    if not line.startswith("{"): continue
    try: r=json.loads(line)
    except Exception: continue
    if r.get("reason")!="compiler-message": continue
    m=r["message"]
    if m.get("level")!="error": continue
    pkg=r["package_id"].split("#")[-1].split("@")[0] if "#" in r["package_id"] else r["package_id"]
    if only and pkg not in only: continue
    cands=[]
    for ch in m.get("children",[]):
        if ch.get("level")!="help": continue
        for sp in ch.get("spans",[]):
            rep=sp.get("suggested_replacement")
            if rep and rep.lstrip().startswith("use ") and sp["byte_start"]==sp["byte_end"]:
                cands.append((sp["file_name"],sp["byte_start"],rep))
    if not cands: continue
    # group by file/start, choose among replacements
    byloc=collections.defaultdict(set)
    for f,s,rep in cands: byloc[(f,s)].add(rep)
    for (f,s),reps in byloc.items():
        reps=sorted(reps)
        if len(reps)==1: sug[f].add((s,reps[0])); units[pkg]+=1
        else:
            order=["semio_framework_value","semio_framework_ui_locale","semio_framework_diagnostic","semio_framework_artifact_reference","semio_framework_job","semio_framework_actor","semio_framework_os_kernel","semio_framework"]
            def rank(x):
                m=re.match(r"\s*use (\w+)::(\w+);",x)
                if not m: return None
                return order.index(m.group(1)) if m.group(1) in order else None
            ranked=sorted([(rank(x),x) for x in reps if rank(x) is not None])
            if ranked: sug[f].add((s,ranked[0][1])); units[pkg]+=1
            else: amb.append((pkg,f,[x.strip() for x in reps][:6]))
cut=float(os.environ.get("CUT","0"))
skipped=[f for f in list(sug) if cut and os.path.getmtime(f)>cut]
for f in skipped: del sug[f]
print("skipped (modified after build start):",len(skipped))
total=0
for f,items in sug.items():
    items=sorted(items,key=lambda x:-x[0])
    if apply:
        b=open(f,"rb").read()
        seen=set()
        for s,rep in items:
            if (s,rep) in seen: continue
            seen.add((s,rep))
            if rep.encode() in b[max(0,s-4000):s+4000] and False: continue
            b=b[:s]+rep.encode()+b[s:]
        open(f,"wb").write(b)
    total+=len(items)
print("files",len(sug),"insertions",total,"units",dict(units))
seen=set()
for pkg,f,reps in amb:
    k=(f,tuple(reps))
    if k in seen: continue
    seen.add(k); print("AMB",pkg,os.path.normpath(f)[-70:],reps)
