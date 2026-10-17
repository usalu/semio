import json,sys
d=json.load(open(sys.argv[1],encoding="utf8"))
rows=[]
for f in d["testResults"]:
    name=f["name"].split("/❓️quiz/")[-1] if "/❓️quiz/" in f["name"] else f["name"]
    if not f["assertionResults"]:
        rows.append((f["status"],name,"<file>",f.get("message","")[:100].replace("\n"," ")))
    for a in f["assertionResults"]:
        rows.append((a["status"],name,a["fullName"],""))
out=sys.argv[2]
open(out,"w",encoding="utf8").write("\n".join("\t".join(r[:3]) for r in sorted(rows))+"\n")
from collections import Counter
c=Counter(r[0] for r in rows)
files=Counter(f["status"] for f in d["testResults"])
print("files",dict(files),"tests",dict(c),"total",len(rows))
