# 🔬 Classifies mutation-fixture-unpaired findings by what exists on disk. usage: python3 classify-unpaired.py <violations.json> <repoRoot> [out.json]
import json,os,sys,collections
src,repo=sys.argv[1],sys.argv[2]
rows=[v for v in json.load(open(src,encoding="utf8")) if v["code"]=="mutation-fixture-unpaired"]
M="/🧪️tests/🧫️fixtures/🧬️mutations/"
def exists(p): return os.path.lexists(os.path.join(repo,p))
def isdir(p): return os.path.isdir(os.path.join(repo,p))
out=collections.defaultdict(list)
for v in rows:
    p=v["path"]
    if M not in p: out["unknown-shape"].append(p); continue
    subset,rel=p.split(M,1)
    parts=rel.split("/")
    mut="/".join(parts[:-1]); case=parts[-1]
    schema=f"{subset}/🧬️schema/🧬️mutations/{mut}"
    impl=f"{schema}/🧪️tests/{case}"
    if not isdir(schema): cat="orphan-no-schema-mutation"
    elif not isdir(f"{schema}/🧪️tests"): cat="no-tests-dir"
    elif not isdir(impl): cat="no-impl-case"
    else:
        ents=os.listdir(os.path.join(repo,impl))
        if not ents: cat="impl-empty-dir"
        elif ents==["🦀️.rs"]: cat="impl-stub-rs-only"
        else: cat="impl-other:"+",".join(sorted(ents))
    out[cat].append(p)
print(json.dumps({k:len(v) for k,v in out.items()},ensure_ascii=False,indent=1))
if len(sys.argv)>3: json.dump(out,open(sys.argv[3],"w",encoding="utf8"),ensure_ascii=False)
