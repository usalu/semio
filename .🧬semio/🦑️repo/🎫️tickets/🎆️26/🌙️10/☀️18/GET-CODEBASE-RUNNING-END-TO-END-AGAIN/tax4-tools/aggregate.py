# 🔬 Sums the tax4 per-scope inventories into tax4-findings.json and prints totals per code and scope against the tax3 simulation. usage: python3 aggregate.py
import json,glob,collections,os,sys
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
rows=[];scopes={}
for f in sorted(glob.glob(f"{C}/🗑️generated/{sys.argv[1] if len(sys.argv)>1 else 'tax4-inv'}/inv-*.json")):
    d=json.load(open(f,encoding="utf8"))
    scopes[d["scope"]]=len(d["violations"])
    for v in d["violations"]: v["scope"]=d["scope"]; rows.append(v)
json.dump({"scopes":scopes,"total":len(rows),"findings":rows},open(f"{C}/🗑️generated/{sys.argv[2] if len(sys.argv)>2 else 'tax4-findings.json'}","w"),ensure_ascii=False,indent=1)
by=collections.Counter(v["code"] for v in rows)
sim=json.load(open(f"{C}/🗑️generated/tax3-run/sim-summary.json"))
print("total",len(rows),"of which ✏️s",sum(n for s,n in scopes.items() if s.startswith("✏️s")))
base=collections.Counter()
if "after" in sim: base=collections.Counter(sim["after"])
for c,n in by.most_common(): print(f"{c:52s} {n:6d}  tax3-after {base.get(c,'-')}")
print({s:n for s,n in sorted(scopes.items(),key=lambda x:-x[1]) if n})
