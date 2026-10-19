import json,re,sys
rows=[json.loads(l) for l in open(sys.argv[1],encoding="utf-8")]
pat=sys.argv[2] if len(sys.argv)>2 else ""
for r in rows:
    if r["rc"]==0 or pat not in r["file"]: continue
    t=open(r["log"],encoding="utf-8").read()
    print("##",r["file"].replace("🧰️framework/","FW/").replace("🔨️modules/","M/")[-110:],r["pass_"],r["fail"])
    ms=[m for m in re.findall(r"^(?:error|\s+error): (.{0,230})",t,re.M)]
    seen=[]
    for m in ms:
        if m not in seen: seen.append(m)
    for m in seen[:3]: print("   ",m)
