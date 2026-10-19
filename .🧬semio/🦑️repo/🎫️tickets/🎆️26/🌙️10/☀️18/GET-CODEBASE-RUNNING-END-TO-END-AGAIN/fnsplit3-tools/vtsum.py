import json,sys
d=json.load(open(sys.argv[1],encoding='utf8'))
rows={}
for f in d["testResults"]:
    name=f["name"].split("/semio/")[-1]
    rows[name+" :: <file>"]=f["status"] if not f["assertionResults"] else "ok"
    for a in f["assertionResults"]:
        rows[name+" :: "+a["fullName"]]=a["status"]
json.dump(rows,open(sys.argv[2],"w",encoding="utf8"),ensure_ascii=False,indent=0,sort_keys=True)
c={}
for v in rows.values(): c[v]=c.get(v,0)+1
print(c)
