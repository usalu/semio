import json,sys,re
F=sys.argv[1]; pats=sys.argv[2:]
r=json.load(open(F+"/tsp-tools/current.json"))
for x in r:
    if x["status"]=="pass": continue
    if not all(p in x["path"] for p in pats): continue
    t=open(x["log"],errors="replace").read()
    t=re.sub(r"/Users/ueli/Documents/semio/","",t)
    errs=[m.start() for m in re.finditer(r"\n(error|TypeError|ReferenceError)",t)]
    print("=====",x["path"])
    print(t[:200] if not errs else t[max(0,errs[0]-300):errs[0]+700])
