# 🔧 Realigns the gltf catalog family (♾️any plus the eight sibling subsets that map to it) to the physical ♾️any tree: leaf directory names from the domain registry, one scenario per physical fixture case. usage: python3 realign-gltf.py [--apply]
import json,os,re,sys,unicodedata,collections
R=os.getcwd(); apply="--apply" in sys.argv
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
tax=json.load(open(f"{L}/🔣️taxonomy.json",encoding="utf8"))
B="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets"
ANY=f"{B}/♾️any"
registry=tax["mutationDomainOwners"][f"{ANY}/🧬️schema/🧬️mutations"]
family=[ANY]+[k for k,v in tax["mutationCatalogSourceOwners"].items() if v==ANY]
fold=lambda s:unicodedata.normalize("NFC",s).replace("️","")
KEBAB=re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
def rest(name): return re.sub(r"^[^A-Za-z0-9]+","",unicodedata.normalize("NFC",name))
docs={s:json.load(open(f"{s}/🔮️oracles/🔣️.json",encoding="utf8")) for s in family}
texts={s:open(f"{s}/🔮️oracles/🔣️.json",encoding="utf8").read() for s in family}
cats={s:d["mutationCatalogs"][0] for s,d in docs.items()}
home={}
for s,c in cats.items():
    for v in c["vectors"]: home.setdefault(v["mutationId"],[]).append((s,v))
import subprocess
history={}
for s in family:
    old=json.loads(subprocess.run(["git","show",f"HEAD:{s}/🔮️oracles/🔣️.json"],capture_output=True,text=True).stdout)
    for c in old["mutationCatalogs"]:
        for v in c["vectors"]:
            for x in v["scenarios"]: history.setdefault((v["mutationId"],fold(x["directoryName"])),x["id"])
FX=f"{ANY}/🧪️tests/🧫️fixtures/🧬️mutations"
FACETS=("🦠️mutation","📸️snapshot","🔺️diff","🎯️outcome")
new_vectors={s:[] for s in family}; log=[]
dup=[k for k,v in home.items() if len(v)>1]
print("mutation ids registered twice:",dup)
for dom,ops in registry.items():
    for op,mid in ops.items():
        base=f"{FX}/{dom}/{op}"
        cases=sorted(c for c in (os.listdir(base) if os.path.isdir(base) else []) if all(os.path.isdir(f"{base}/{c}/{f}") for f in FACETS))
        prev=home.get(mid,[(None,None)])[0]
        s,v=prev
        scs=[]; seen=set()
        if v:
            have={fold(x["directoryName"]):x for x in v["scenarios"]}
            for c in cases:
                if fold(c) in have: scs.append({"id":have[fold(c)]["id"],"directoryName":unicodedata.normalize("NFC",c)}); seen.add(have[fold(c)]["id"])
        for c in cases:
            if any(fold(x["directoryName"])==fold(c) for x in scs): continue
            sid=history.get((mid,fold(c)),rest(c))
            if not KEBAB.match(sid): log.append(("bad-case-id",dom,op,c)); continue
            while sid in seen: sid+="-2"
            seen.add(sid); scs.append({"id":sid,"directoryName":unicodedata.normalize("NFC",c)})
        if not scs: log.append(("no-fixtures",dom,op)); continue
        nv={"mutationId":mid,"sourceMutationDirectoryName":unicodedata.normalize("NFC",op),"mutationDirectoryName":unicodedata.normalize("NFC",op),"scenarios":scs}
        new_vectors[s or ANY].append(nv)
for mid,hs in home.items():
    if not any(mid==m for ops in registry.values() for m in ops.values()): log.append(("stale-vector-no-registry-identity",mid))
def replace_array(text,key,value):
    k=text.index(f'"{key}": [')+len(f'"{key}": ')
    depth=0;i=k;instr=False
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
    ls=text.rfind("\n",0,k)+1; indent=len(text[ls:k])-len(text[ls:k].lstrip())
    body=json.dumps(value,ensure_ascii=False,indent=2).split("\n")
    new=body[0]+"".join("\n"+" "*indent+l for l in body[1:])
    r=text[:k]+new+text[i+1:]; json.loads(r); return r
for s in family:
    c=cats[s]; vs=new_vectors[s]
    order={v["mutationId"]:i for i,v in enumerate(c["vectors"])}
    vs.sort(key=lambda v:(order.get(v["mutationId"],10**6),v["mutationId"]))
    kinds=sorted(set(c["kinds"])|{v["mutationId"] for v in vs}) if s==ANY else c["kinds"]
    kinds=[k for k in kinds if k in {v["mutationId"] for v in vs} or s!=ANY or True]
    changed=json.dumps(vs,sort_keys=True,ensure_ascii=False)!=json.dumps(c["vectors"],sort_keys=True,ensure_ascii=False) or kinds!=c["kinds"]
    print(s.split("🪆️subsets/")[-1],"vectors",len(c["vectors"]),"->",len(vs),"scenarios",sum(len(v["scenarios"]) for v in c["vectors"]),"->",sum(len(v["scenarios"]) for v in vs),"changed" if changed else "same")
    if apply and changed:
        t=replace_array(texts[s],"vectors",vs)
        if kinds!=c["kinds"]: t=replace_array(t,"kinds",kinds)
        open(f"{s}/🔮️oracles/🔣️.json","w",encoding="utf8").write(t)
for l in log: print("  note",l)
