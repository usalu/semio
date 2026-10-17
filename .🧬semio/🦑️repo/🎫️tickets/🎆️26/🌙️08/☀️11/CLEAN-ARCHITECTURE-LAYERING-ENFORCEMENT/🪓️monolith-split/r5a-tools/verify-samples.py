import sys,json
t=open(sys.argv[1],encoding="utf-8").read()
i=t.index("\n{")+1
j,_=json.JSONDecoder().raw_decode(t[i:])
print("problems:",[l[:300] for l in t[:i].split("\n") if l.startswith("problem")])
print({k:(len(v) if isinstance(v,(list,dict)) else v) for k,v in j.items()}, [ (r["name"].replace("semio-s-artifact-stdio-",""),r["lib"],r["test"],r["withinHardBudget"]) for r in j["census"]])
for k in ("moduleTreeProblems","visibility","orphan","glue"): print(k,json.dumps(j[k],ensure_ascii=False)[:300])
for sect in ("oracle","layout"):
    for name,row in j[sect].items():
        print(sect,name,{a:b for a,b in row.items() if not isinstance(b,list)})
        for key in ("sampleOnlyCandidate","sampleOnlyCombined"):
            for s in row.get(key,[])[:int(sys.argv[2]) if len(sys.argv)>2 else 15]: print("  ",str(s).replace("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/","")[:300])
