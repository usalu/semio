import json,sys,os
T=os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
d=json.load(open(os.path.join(T,"🗑️generated","oracle-en1993","audit.json"),encoding="utf-8"))["en1993"]["rows"]
ids=set(sys.argv[1:])
for r in d:
    if r["kind"] in("check","design-case") and (not ids or r["id"] in ids):
        o=r["oracle"]; print(r["id"], "|", (o[0][0].split("🔮️oracles/")[-1]+":"+str(o[0][1])) if o else "none", "|", r["oracle_deps"][0][:60] if r["oracle_deps"] else "")
