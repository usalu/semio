import json,sys,re
from collections import Counter,defaultdict
d=json.load(open(sys.argv[1],encoding="utf8"))
by=defaultdict(list)
for f in d["testResults"]:
    name=f["name"].split("/semio/")[-1].replace("🧰️framework/🛍️products/💻️os/","OS/")
    st=Counter(a["status"] for a in f["assertionResults"])
    print(f["status"],name,dict(st),(f.get("message") or "")[:160].replace("\n"," | "))
    if "-v" in sys.argv:
        shown=set()
        for a in f["assertionResults"]:
            if a["status"]=="failed":
                m=(a.get("failureMessages") or [""])[0][:220].replace("\n"," | ")
                if m not in shown:
                    shown.add(m); print("    ",a["fullName"][:100],"::",m)
