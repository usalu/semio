import json,re,sys
d=json.load(open(sys.argv[1],encoding="utf8"))
n=int(sys.argv[2]) if len(sys.argv)>2 else 500
for f in d["testResults"]:
    name=f["name"].replace("/Users/ueli/Documents/semio/","")
    bad=[a for a in f["assertionResults"] if a["status"]=="failed"]
    if f["status"]=="failed" and not bad:
        print("FILE FAIL",name); print(re.sub(r"\x1b\[[0-9;]*m","",f.get("message",""))[:n]); continue
    for a in bad:
        print("----",name.split("/")[-2],"|",a["fullName"][:120]); print(re.sub(r"\x1b\[[0-9;]*m","","\n".join(a["failureMessages"]))[:n])
