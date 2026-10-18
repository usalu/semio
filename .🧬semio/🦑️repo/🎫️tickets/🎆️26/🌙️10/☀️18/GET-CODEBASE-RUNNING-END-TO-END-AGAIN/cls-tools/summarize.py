import json,sys
d=json.load(open(sys.argv[1]))
rows=[]
for f in d["testResults"]:
    for t in f["assertionResults"]:
        rows.append((" > ".join(t["ancestorTitles"]+[t["title"]]),t["status"]))
rows.sort()
print(len(rows),sum(1 for r in rows if r[1]=="passed"),"passed",sum(1 for r in rows if r[1]=="failed"),"failed")
if len(sys.argv)>2:
    open(sys.argv[2],"w").write("\n".join(f"{s}\t{n}" for n,s in rows)+"\n")
