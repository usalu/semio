# 🔧 Drops catalog scenarios whose bundle directory is physically absent (neither fixture nor implementation exists) from vectors flagged projection-catalog-unrealized; empty vectors are dropped with their kind. usage: python3 prune-stale-scenarios.py <findings-s.json> [--apply]
import json,os,re,sys,unicodedata,collections
rows=[v for v in json.load(open(sys.argv[1],encoding="utf8")) if v["code"]=="projection-catalog-unrealized"]
apply="--apply" in sys.argv
fold=lambda s:unicodedata.normalize("NFC",s).replace("️","")
SOURCE_OWNERS=json.load(open("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",encoding="utf8"))["mutationCatalogSourceOwners"]
by=collections.defaultdict(set)
for v in rows:
    m=re.match(r"Registered vector (.+?)/(.+?)/([^/]+) has no physical bundle$",v["message"])
    if m: by[v["path"]].add((m.group(1),m.group(2),m.group(3)))
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
total=0
for path,items in sorted(by.items()):
    catalog_owner=path[:-len("/🔮️oracles/🔣️.json")]
    owner=SOURCE_OWNERS.get(catalog_owner,catalog_owner)
    text=open(path,encoding="utf8").read(); d=json.loads(text)
    fx=f"{owner}/🧪️tests/🧫️fixtures/🧬️mutations"
    physical=collections.defaultdict(set)
    for r,ds,fs in os.walk(fx):
        parent=fold(os.path.basename(r))
        for x in ds: physical[parent].add(fold(x))
    sch=f"{owner}/🧬️schema/🧬️mutations"
    for r,ds,fs in os.walk(sch):
        if os.path.basename(r)=="🧪️tests":
            parent=fold(os.path.basename(os.path.dirname(r)))
            for x in ds: physical[parent].add(fold(x))
    changed=False;dropped=0
    for c in d["mutationCatalogs"]:
        vs=[]
        for v in c["vectors"]:
            keep=[]
            for s in v["scenarios"]:
                if (v["mutationId"],v["sourceMutationDirectoryName"],s["id"]) in items and fold(s["directoryName"]) not in physical.get(fold(v["mutationDirectoryName"]),set()) and fold(s["directoryName"]) not in physical.get(fold(v["sourceMutationDirectoryName"]),set()):
                    dropped+=1; changed=True; continue
                keep.append(s)
            if keep: vs.append({**v,"scenarios":keep})
            else: changed=True
        if changed: c["vectors"]=vs
    total+=dropped
    print(path.split("🗿️artifacts/")[-1][:90],"flagged",len(items),"dropped",dropped)
    if apply and changed:
        c=d["mutationCatalogs"][0]
        t=replace_array(text,"vectors",c["vectors"]) if len(d["mutationCatalogs"])==1 else text
        open(path,"w",encoding="utf8").write(t)
print("scenarios dropped",total)
