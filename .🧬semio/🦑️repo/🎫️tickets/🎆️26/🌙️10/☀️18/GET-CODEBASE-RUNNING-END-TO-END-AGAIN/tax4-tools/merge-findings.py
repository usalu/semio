# 🔬 Merges per-scope inventory dirs (later dirs override earlier ones for the same scope) into one flat ✏️s violations list. usage: python3 merge-findings.py <out.json> <dir>...
import json,glob,sys,os
by={}
for d in sys.argv[2:]:
    for f in sorted(glob.glob(f"{d}/inv-*.json")):
        j=json.load(open(f,encoding="utf8")); by[j["scope"]]=j["violations"]
rows=[v for s,vs in by.items() for v in vs if s.startswith("✏️s")]
json.dump(rows,open(sys.argv[1],"w",encoding="utf8"),ensure_ascii=False)
print(len(by),"scopes",len(rows),"✏️s findings")
