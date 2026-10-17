import json,sys
lines=open(sys.argv[1],encoding="utf8").read().split("\n")
i=lines.index("{"); j=max(k for k,l in enumerate(lines) if l=="}")
j=json.loads("\n".join(lines[i:j+1]))
if "census" in j:
    for r in j["census"]: print(r["name"],r["lib"],r["test"],r["withinHardBudget"])
    for k in ("moduleTreeProblems","visibility","orphan","glue"): print(k,len(j[k]), j[k][:3])
    print("unresolved",{k:len(v) for k,v in j["unresolvedCrates"].items() if v})
    print("oracle",{k:(v["original"],v["candidate"],v["onlyOriginal"],v["onlyCandidate"]) for k,v in (j.get("oracle") or {}).items()})
    print("layout",{k:(v["union"],v["combined"],v["onlyCombined"]) for k,v in (j.get("layout") or {}).items()})
    for k,v in (j.get("oracle") or {}).items():
        if v["onlyCandidate"]: print("ONLY-CAND",k,v["sampleOnlyCandidate"][:10])
    for k,v in (j.get("layout") or {}).items():
        if v["onlyCombined"]: print("ONLY-COMB",k,v["sampleOnlyCombined"][:10])
else:
    print("base",j.get("base"),"crates",j.get("crates"),"healed",j.get("healedBase"))
    for k,v in j["layout"].items():
        print(k,v["base"],v["combined"],v["onlyCombined"])
        for s in v["sampleOnlyCombined"]: print("   EXTRA",s[:400])
    print("admission",j["admission"],"shared",j["shared"],"sharedForeign",j["sharedForeign"])
