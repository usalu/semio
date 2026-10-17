import json,sys
t=open(sys.argv[1],encoding='utf8').read()
i=t.index('\n{\n  "census"')
j,_=json.JSONDecoder().raw_decode(t[i+1:])
print("oracle only-candidate:",{k:v["onlyCandidate"] for k,v in j["oracle"].items()}, "layout:",{k:v["onlyCombined"] for k,v in j["layout"].items()})
print("tree",j["moduleTreeProblems"][:3],"vis",len(j["visibility"]),"orphan",len(j["orphan"]),"glue",j["glue"][:3])
print("unresolved",{k:v for k,v in j["unresolvedCrates"].items() if v})
for k in j["oracle"]:
    for s in j["oracle"][k]["sampleOnlyCandidate"]: print("O",k,s[:260].replace("✏️s/🔌️plugins/X/🏅️standards/🔖️1/🪆️subsets/✳️any/",""))
for k in j["layout"]:
    for s in j["layout"][k]["sampleOnlyCombined"]: print("L",k,s[:260].replace("✏️s/🔌️plugins/X/🏅️standards/🔖️1/🪆️subsets/✳️any/",""))
