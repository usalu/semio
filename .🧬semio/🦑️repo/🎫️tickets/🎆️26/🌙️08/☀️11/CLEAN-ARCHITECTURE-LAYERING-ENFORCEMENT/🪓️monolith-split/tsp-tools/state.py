import json,sys,glob,re
F=sys.argv[1]
m={}
for x in json.load(open(F+"/tsp-tools/baseline-merged.json")): m[x["path"]]=x
for d in sys.argv[2:]:
    for f in glob.glob(F+"/🗑️generated/"+d+"/*.json"):
        if f.endswith("results.json"): continue
        x=json.load(open(f)); m[x["path"]]=x
fails=[x for x in m.values() if x["status"]!="pass"]
print(len(m),"files",len(fails),"not passing; tests pass",sum(x["pass_"] for x in m.values()),"fail",sum(x["fail"] for x in m.values()))
json.dump(list(m.values()),open(F+"/tsp-tools/current.json","w"),ensure_ascii=False)
for x in sorted(fails,key=lambda x:x["path"]):
    p=x["path"]; s=p.split("🪆️subsets/✳️any/")[-1] if "subsets/" in p else p
    pl=p.split("/")[2] if p.startswith("✏️s/") else ""
    print(f"{x['status']:7} p{x['pass_']} f{x['fail']} {pl} .../{s[-95:]}")
