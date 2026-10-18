import json,sys
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
t=open(F+"/🗑️generated/"+sys.argv[1],encoding="utf-8").read()
i=t.index("\n{\n"); rep=json.loads(t[i:t.rindex("}")+1])
def short(s): return s.replace("✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/","~/")[:int(sys.argv[2]) if len(sys.argv)>2 else 240]
print("census",len(rep['census']),"over",[r for r in rep['census'] if not r.get('withinHardBudget',True)][:2])
for k in ('moduleTreeProblems','visibility','orphan','glue'): print(k,len(rep[k]))
print("unresolved",{k:len(v) for k,v in rep['unresolvedCrates'].items() if v})
for k in ('oracle','layout'):
    for cfg,v in rep[k].items():
        print("##",k,cfg,{x:y for x,y in v.items() if not isinstance(y,list)})
for k,key in (('oracle','sampleOnlyCandidate'),('layout','sampleOnlyCombined')):
    for cfg,v in rep[k].items():
        print("--",k,cfg,key,len(v[key]))
        for s in v[key]: print("   ",short(s))
