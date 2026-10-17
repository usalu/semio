import sys,json,re
t=open(sys.argv[1],encoding="utf-8").read()
i=t.index("\n{")+1
dec=json.JSONDecoder()
j,_=dec.raw_decode(t[i:])
for l in t[:i].split("\n"):
    if l.startswith("problem") or l.startswith("[script]") is False and l.strip(): print(l[:300])
print({k:(len(v) if isinstance(v,(list,dict)) else v) for k,v in j.items()})
for r in j.get("census",[]): print("census",{k:v for k,v in r.items() if k in("name","lib","test","withinHardBudget")})
for k in ("moduleTreeProblems","visibility","orphan","glue"):
    print(k, json.dumps(j.get(k),ensure_ascii=False)[:500])
print("unresolved", json.dumps(j.get("unresolvedCrates"),ensure_ascii=False)[:400])
for sect in ("oracle","layout"):
    for name,row in (j.get(sect) or {}).items():
        print(sect,name,{a:(b if not isinstance(b,list) else len(b)) for a,b in row.items()})
        lim=int(sys.argv[2]) if len(sys.argv)>2 else 12
        for key in ("sample","samples"):
            for s in (row.get(key) or [])[:lim]: print("    ",str(s)[:260])
