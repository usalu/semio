# 🔬 For every mutation-fixture-unpaired finding, looks for an unmatched implementation case dir in the same leaf and proposes the rename. usage: python3 pair-candidates.py <findings-s.json> <out.json>
import json,os,sys,collections,unicodedata
rows=[v for v in json.load(open(sys.argv[1],encoding="utf8")) if v["code"]=="mutation-fixture-unpaired"]
M="/🧪️tests/🧫️fixtures/🧬️mutations/"
def strip(s): return unicodedata.normalize("NFC",s).replace("️","")
res=collections.defaultdict(list)
byleaf=collections.defaultdict(list)
for v in rows:
    subset,rel=v["path"].split(M,1); parts=rel.split("/")
    byleaf[(subset,"/".join(parts[:-1]))].append(parts[-1])
for (subset,mut),cases in sorted(byleaf.items()):
    S=f"{subset}/🧬️schema/🧬️mutations/{mut}"; T=f"{S}/🧪️tests"
    F=f"{subset}{M}{mut}"
    if not os.path.isdir(S): res["orphan-no-schema"].append({"subset":subset,"mut":mut,"cases":cases}); continue
    impls=sorted(os.listdir(T)) if os.path.isdir(T) else []
    fixtures=sorted(d for d in os.listdir(F) if os.path.isdir(f"{F}/{d}"))
    unimpl=[i for i in impls if i not in fixtures]
    cand={}
    for c in cases:
        e=strip(c).split("-")[0][:1]
        key=lambda n: strip(n)[:1]
        m=[i for i in unimpl if key(i)==key(c)]
        cand[c]=m
    kind="renamable" if all(len(m)==1 for m in cand.values()) and len({m[0] for m in cand.values()})==len(cand) else ("no-impl" if not unimpl else "ambiguous")
    res[kind].append({"subset":subset,"mut":mut,"cases":cases,"impls":impls,"cand":cand})
json.dump(res,open(sys.argv[2],"w",encoding="utf8"),ensure_ascii=False,indent=1)
for k,v in res.items(): print(k,len(v),sum(len(x["cases"]) for x in v))
