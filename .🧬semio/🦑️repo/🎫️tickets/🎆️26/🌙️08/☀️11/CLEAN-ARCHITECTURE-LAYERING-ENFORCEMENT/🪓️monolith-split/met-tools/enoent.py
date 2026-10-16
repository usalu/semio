import json,sys,os,re
F=os.environ["F"]
j=json.load(open(sys.argv[1],encoding="utf8"))
paths={}
for f in j["testResults"]:
    for a in f["assertionResults"]:
        for m in a.get("failureMessages",[]):
            for mm in re.finditer(r"ENOENT: no such file or directory, (?:open|scandir|stat|lstat) '([^']+)'",m):
                paths.setdefault(mm.group(1),[]).append(a["fullName"][:70])
FE="️"
tests="\U0001F9EA"+FE+"tests"; fx="\U0001F9EB"+FE+"fixtures"
for p,t in sorted(paths.items()):
    cand=[]
    if "/"+fx+"/" in p:
        cand.append(p.replace("/"+fx+"/","/"+tests+"/"+fx+"/",1))
    exists=[c for c in cand if os.path.exists(c)]
    print(("OK  " if exists else "MISS"), p.replace("/Users/ueli/Documents/semio/",""), "->", (exists[0].replace("/Users/ueli/Documents/semio/","") if exists else "-"), len(t))
