# 🔬 For every 'scenario root is absent' mutation-fixture-invalid finding, lists the implementation case dirs and fixture dirs of its leaf. usage: python3 absent-roots.py <findings-s.json>
import json,os,sys,collections
rows=[v for v in json.load(open(sys.argv[1],encoding="utf8")) if v["code"]=="mutation-fixture-invalid" and "absent" in v["message"]]
M="/🧪️tests/🧫️fixtures/🧬️mutations/"
out=[]
for v in rows:
    sub,rel=v["path"].split(M,1); parts=rel.split("/"); mut="/".join(parts[:-1]); case=parts[-1]
    S=f"{sub}/🧬️schema/🧬️mutations/{mut}/🧪️tests"; F=f"{sub}{M}{mut}"
    impls=sorted(os.listdir(S)) if os.path.isdir(S) else None
    fx=sorted(os.listdir(F)) if os.path.isdir(F) else None
    out.append((sub.split("/")[2],mut,case,impls,fx))
    print(sub.split("/")[2],"|",mut,"|expected fixture",case,"|impls",impls,"|fixtures",fx)
