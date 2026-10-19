# 🔬 Classifies projection-catalog-unrealized findings: is the registered scenario's fixture bundle physically present? usage: python3 classify-unrealized.py <violations.json> <repoRoot> [out.json]
import json,os,sys,collections
src,repo=sys.argv[1],sys.argv[2]
rows=[v for v in json.load(open(src,encoding="utf8")) if v["code"]=="projection-catalog-unrealized"]
fold=lambda s:s.replace("️","")
cat=collections.Counter(); out=collections.defaultdict(list)
cache={}
def catalog(p):
    if p not in cache: cache[p]=json.load(open(os.path.join(repo,p),encoding="utf8"))
    return cache[p]
def find_dirs(root, name, depth=3):
    res=[]
    if not os.path.isdir(root): return res
    for r,ds,fs in os.walk(root):
        rel=os.path.relpath(r,root)
        if rel.count(os.sep)>=depth: ds[:]=[]; continue
        for d in ds:
            if fold(d)==fold(name): res.append(os.path.join(r,d))
    return res
for v in rows:
    p=v["path"]; sub=p[:-len("/🔮️oracles/🔣️.json")]
    key=v["message"].split("Registered vector ",1)[1].split(" has no")[0]
    mid,src_dir,scen=key.split("/",2)
    c=catalog(p)
    dn=None
    for mc in c.get("mutationCatalogs",[]):
        for vec in mc["vectors"]:
            if vec["mutationId"]==mid and vec["sourceMutationDirectoryName"]==src_dir:
                for s in vec["scenarios"]:
                    if s["id"]==scen: dn=s["directoryName"]
    froot=os.path.join(repo,sub,"🧪️tests/🧫️fixtures/🧬️mutations")
    hit=None
    for md in find_dirs(froot,src_dir,2):
        if dn and os.path.isdir(os.path.join(md,dn)): hit=md; break
        if dn:
            alts=[x for x in os.listdir(md) if fold(x)==fold(dn)] if os.path.isdir(md) else []
            if alts: hit=md; break
    schema=find_dirs(os.path.join(repo,sub,"🧬️schema/🧬️mutations"),src_dir,2)
    k=("fixture-present" if hit else ("mutation-fixtures-present-scenario-absent" if find_dirs(froot,src_dir,2) else "no-fixture-dir"), "schema-mutation" if schema else "no-schema-mutation")
    cat[k]+=1; out[str(k)].append({"catalog":p,"key":key,"dir":dn})
for k,v in sorted(cat.items()): print(k,v)
if len(sys.argv)>3: json.dump(out,open(sys.argv[3],"w"),ensure_ascii=False)
