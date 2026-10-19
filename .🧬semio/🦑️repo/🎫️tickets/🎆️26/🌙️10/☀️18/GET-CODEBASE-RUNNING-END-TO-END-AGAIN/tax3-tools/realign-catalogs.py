# 🔧 One-off: realigns the mutation catalog vectors of oracle registries to the physical tree. usage: python3 realign-catalogs.py <sim-violations.json> [--apply]
# A vector is kept per physical mutation leaf that has an implementation case or a fixture bundle; scenarios are the physical case directories (existing logical ids are kept). Vectors without any physical leaf or case are dropped.
import json,os,re,sys,collections,unicodedata
R="/Users/ueli/Documents/semio"; apply="--apply" in sys.argv
L=os.environ.get("TAX3_L") or f"{R}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
tax=json.load(open(f"{L}/🔣️taxonomy.json",encoding="utf8"))
domains=tax["mutationDomainOwners"]; srcowners=tax["mutationCatalogSourceOwners"]
rows=json.load(open(sys.argv[1],encoding="utf8"))
codes={"projection-catalog-coverage","projection-catalog-unrealized","projection-member-unresolved","projection-catalog-invalid"}
fold=lambda s:unicodedata.normalize("NFC",s).replace("️","")
EMOJI_SPLIT=re.compile(r"^((?:[^\x00-\x7f]|‍|️)+)(.*)$",re.S)
def rest(name):
    m=EMOJI_SPLIT.match(unicodedata.normalize("NFC",name))
    return m.group(2) if m else name
KEBAB=re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
def subsets():
    out=set()
    for v in rows:
        if v["code"] not in codes: continue
        p=v["path"]
        for m in ("/🧬️schema/","/🔮️oracles/","/🧪️tests/"):
            if m in p: out.add(p.split(m)[0]); break
    return sorted(out)
FACETS=("🦠️mutation","📸️snapshot","🔺️diff","🎯️outcome")
def isdir(p): return os.path.isdir(p)

PREFIX="shared://🧬️mutations/"
def bundle_of(step):
    at=step.find(PREFIX)
    if at<0: return None
    segs=re.split(r"\s",step[at+len(PREFIX):])[0].split("/")
    for i,x in enumerate(segs):
        if x in FACETS: 
            return None if i<1 else unicodedata.normalize("NFC","/".join(segs[:i]).replace("\uFE0F",""))
    return None
def declared_by_feature(src):
    out=[];templates=[];header=None;examples=False;outline=False
    for raw in re.split(r"\r?\n",src):
        line=raw.strip()
        if not line or line.startswith("#") or line.startswith("@"): continue
        if re.match(r"^(Scenario Outline|Scenario Template):",line): outline=True;examples=False;header=None;templates=[];continue
        if re.match(r"^(Feature|Background|Scenario|Example):",line): outline=False;examples=False;header=None;templates=[];continue
        if re.match(r"^(Examples|Scenarios):",line): examples=True;header=None;continue
        if line.startswith("|"):
            if not outline or not examples: continue
            cells=[c.strip().replace("\\|","|") for c in line[1:-1].split("|")]
            if header is None: header=cells; continue
            row=dict(zip(header,cells))
            for t in templates:
                step=re.sub(r"<([^<>]+)>",lambda m,row=row:row.get(m.group(1),m.group(0)),t)
                b=bundle_of(step)
                if b and b not in out: out.append(b)
            continue
        if outline and not examples:
            if PREFIX in line: templates.append(line)
            continue
        if not outline:
            b=bundle_of(line)
            if b and not re.search(r"<[^<>]+>",b) and b not in out: out.append(b)
    return out
def coverage(S):
    runner=os.path.isfile(f"{R}/{S}/🧬️schema/🧬️mutations/🧪️tests/🗂️fixture-cases/🦀️.rs")
    decl=set()
    td=f"{R}/{S}/🧪️tests"
    for r,ds,fs in os.walk(td):
        if "🧫️fixtures" in ds: ds.remove("🧫️fixtures")
        for f in fs:
            if f=="🥒️.feature": decl.update(declared_by_feature(open(os.path.join(r,f),encoding="utf8",errors="replace").read()))
    return runner,decl

uncovered=[]
def scan(S):
    sroot=f"{S}/🧬️schema/🧬️mutations"; froot=f"{S}/🧪️tests/🧫️fixtures/🧬️mutations"
    reg=domains.get(os.path.relpath(sroot,R))
    leaves=collections.OrderedDict()
    def leaf_entries(root):
        if not isdir(root): return
        for a in sorted(os.listdir(root)):
            ap=os.path.join(root,a)
            if not isdir(ap) or a in ("🧪️tests","📦️packages","🧬️schema","🔺️diff","↩️inverse","🦠️mutation"): continue
            if reg is not None and a in reg:
                for b in sorted(os.listdir(ap)):
                    if isdir(os.path.join(ap,b)) and b in reg[a]: yield (a,b),os.path.join(ap,b)
            elif reg is None: yield (None,a),ap
    impl={}
    for key,p in leaf_entries(sroot):
        t=os.path.join(p,"🧪️tests")
        if isdir(t):
            for c in sorted(os.listdir(t)):
                if isdir(os.path.join(t,c)): impl.setdefault(key,set()).add(c)
    runner,decl=coverage(os.path.relpath(S,R))
    for key,p in leaf_entries(froot):
        for c in sorted(os.listdir(p)):
            cp=os.path.join(p,c)
            if not (isdir(cp) and all(isdir(os.path.join(cp,f)) for f in FACETS)): continue
            rel=unicodedata.normalize("NFC","/".join(x for x in (key[0],key[1],c) if x).replace("\uFE0F",""))
            if c in impl.get(key,set()) or runner or rel in decl: leaves.setdefault(key,{}).setdefault(c,set()).add("fixture")
            else: uncovered.append((key,c))
    return leaves,reg
def replace_vectors(text,vectors):
    key=text.index('"vectors": [')+len('"vectors": ')
    depth=0;i=key;instr=False
    while True:
        ch=text[i]
        if instr:
            if ch=="\\": i+=1
            elif ch=='"': instr=False
        elif ch=='"': instr=True
        elif ch=="[": depth+=1
        elif ch=="]":
            depth-=1
            if depth==0: break
        i+=1
    line_start=text.rfind("\n",0,key)+1
    indent=len(text[line_start:key])-len(text[line_start:key].lstrip())
    body=json.dumps(vectors,ensure_ascii=False,indent=2).split("\n")
    new=body[0]+"".join("\n"+" "*indent+l for l in body[1:])
    assert text.count('"vectors": [')==1
    result=text[:key]+new+text[i+1:]
    json.loads(result)
    return result
report=[]; changed=0
for S in subsets():
    F=f"{R}/{S}/🔮️oracles/🔣️.json"
    if S in srcowners or S in srcowners.values() or not os.path.isfile(F): report.append((S,"skipped: source-owner mapping or no catalog")); continue
    text=open(F,encoding="utf8").read(); d=json.loads(text)
    mcs=d.get("mutationCatalogs",[])
    if len(mcs)!=1: report.append((S,f"skipped: {len(mcs)} catalogs")); continue
    cat=mcs[0]; leaves,reg=scan(f"{R}/{S}")
    existing={fold(v["sourceMutationDirectoryName"]):v for v in cat["vectors"]}
    out=[]; used=set(); added=dropped_s=added_s=0; conflicts=[]
    ids=set()
    def mutation_id(key):
        dom,leaf=key
        return reg[dom][leaf] if reg is not None else rest(leaf)
    order=list(cat["vectors"])
    byid={}
    for k in leaves:
        try: byid.setdefault(mutation_id(k),[]).append(k)
        except KeyError: pass
    def leaf_matches(v):
        m=[k for k in leaves if mutation_id(k)==v["mutationId"]]
        if not m and reg is None: m=[k for k in leaves if fold(k[1])==fold(v["sourceMutationDirectoryName"])]
        return m
    used={k for v in order for k in leaf_matches(v)}
    new_keys=[k for k in leaves if k not in used]
    def vector_for(key,prev):
        global added_s,dropped_s
        mid=prev["mutationId"] if prev else mutation_id(key)
        if not KEBAB.match(mid): conflicts.append((key,"mutation id not kebab")); return None
        if prev is None and reg is None and rest(key[1])!=mid: conflicts.append((key,"physical id differs")); return None
        disk={fold(c):c for c in leaves[key]}
        if prev is None:
            scs=[];seen=set()
            for c in leaves[key]:
                r=rest(c)
                if not KEBAB.match(r) or r==c: conflicts.append((key,f"case {c} not emoji+kebab")); continue
                sid=r
                while sid in seen: sid+="-2"
                seen.add(sid); scs.append({"id":sid,"directoryName":unicodedata.normalize("NFC",c)}); added_s+=1
            return {"mutationId":mid,"sourceMutationDirectoryName":unicodedata.normalize("NFC",key[1]),"mutationDirectoryName":unicodedata.normalize("NFC",key[1]),"scenarios":scs} if scs else None
        have={fold(s["directoryName"]) for s in prev["scenarios"]}
        if not (have & set(disk)):
            renamed.append(prev["mutationId"]); return prev
        scs=list(prev["scenarios"]); seen={s["id"] for s in scs}
        for f,c in disk.items():
            if f in have: continue
            r=rest(c)
            if not KEBAB.match(r) or r==c: conflicts.append((key,f"case {c} not emoji+kebab")); continue
            sid=r
            while sid in seen: sid+="-2"
            seen.add(sid); scs.append({"id":sid,"directoryName":unicodedata.normalize("NFC",c)}); added_s+=1
        return {**prev,"scenarios":scs}
    renamed=[]
    for v in order:
        matches=leaf_matches(v)
        if len(matches)!=1:
            fx=os.path.join(R,S,"🧪️tests/🧫️fixtures/🧬️mutations")
            present=any(fold(v["sourceMutationDirectoryName"]) in {fold(x) for x in os.listdir(os.path.join(fx,d_))} for d_ in (os.listdir(fx) if os.path.isdir(fx) else []) if os.path.isdir(os.path.join(fx,d_))) or any(fold(x)==fold(v["sourceMutationDirectoryName"]) for x in (os.listdir(fx) if os.path.isdir(fx) else []))
            if present: ids.add(v["mutationId"]); out.append(v)
            else: dropped_s+=len(v["scenarios"])
            continue
        nv=vector_for(matches[0],v)
        if nv and nv["mutationId"] not in ids: ids.add(nv["mutationId"]); out.append(nv)
    for k in sorted(new_keys,key=lambda k:fold(k[1])):
        nv=vector_for(k,None)
        if nv and nv["mutationId"] not in ids: ids.add(nv["mutationId"]); out.append(nv); added+=1
    before=json.dumps(cat["vectors"],ensure_ascii=False,sort_keys=True)
    if json.dumps(out,ensure_ascii=False,sort_keys=True)!=before:
        report.append((S,f"vectors {len(cat['vectors'])}->{len(out)} newVectors={added} scenarios+{added_s} droppedStale={dropped_s} renamedDrift={len(renamed)} conflicts={len(conflicts)} {conflicts[:2]}"))
        if apply:
            open(F,"w",encoding="utf8").write(replace_vectors(text,out)); changed+=1
for S,m in report: print(S.replace("✏️s/🔌️plugins/","")[-110:],"|",m)
print("catalogs",("rewritten" if apply else "to rewrite"),sum(1 for _,m in report if m.startswith("vectors")),"skipped",sum(1 for _,m in report if m.startswith("skipped")))
