# 🔬 For each leftover unpaired bundle, counts text references to its case and leaf names outside the fixtures dir. usage: python3 refs-of-bundles.py <pairs.json> <kinds...>
import json,os,sys,re,subprocess,collections
d=json.load(open(sys.argv[1],encoding="utf8"))
out=collections.defaultdict(list)
for kind in sys.argv[2:]:
    for x in d[kind]:
        if "cases" not in x: continue
        subset=x["subset"]
        for case in x["cases"]:
            name=re.sub(r"^[^A-Za-z0-9]+","",case)
            leaf=re.sub(r"^[^A-Za-z0-9]+","",x["mut"].split("/")[-1])
            p=subprocess.run(["grep","-rIl","--exclude-dir=node_modules","--exclude-dir=target","--exclude-dir=🧫️fixtures","-F",name,subset],capture_output=True,text=True)
            files=[f.replace(subset+"/","") for f in p.stdout.split("\n") if f]
            out[(subset.split("/")[2],x["mut"])].append((case,name,files[:4]))
for k,v in out.items():
    print(k)
    for case,name,files in v[:3]: print("   ",case,name,"->",files)
