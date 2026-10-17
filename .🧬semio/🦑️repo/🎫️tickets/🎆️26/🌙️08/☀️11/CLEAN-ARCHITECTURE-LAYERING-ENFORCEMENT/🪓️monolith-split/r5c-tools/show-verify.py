import json,sys
t=open(sys.argv[1]).read()
try:
    i=t.index("\n{\n"); j=t.rindex("\n}\n"); d=json.loads(t[i:j+3])
except Exception as e:
    print("no json:", t[-600:]); sys.exit()
bad=[r for r in d["census"] if not r["withinHardBudget"]]
print("crates",len(d["census"]),"overHard",[r["name"] for r in bad])
print("moduleTree",len(d["moduleTreeProblems"]),[x[:200] for x in d["moduleTreeProblems"][:4]])
for k in("visibility","orphan","glue"): print(k,[x[:200] for x in d[k][:4]])
print("unresolved",{k:v for k,v in d["unresolvedCrates"].items() if v})
print({k:(v["original"],v["candidate"],v["onlyCandidate"],[x[:200] for x in v["sampleOnlyCandidate"][:3]]) for k,v in (d.get("oracle") or {}).items()})
for k,v in (d.get("layout") or {}).items(): print("layout",k,v["union"],v["combined"],v["onlyCombined"],[x[:240] for x in v["sampleOnlyCombined"][:5]])
