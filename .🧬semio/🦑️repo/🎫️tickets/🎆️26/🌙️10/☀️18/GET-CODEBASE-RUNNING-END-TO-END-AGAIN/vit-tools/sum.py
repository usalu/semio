import json,re,sys
for l in open(sys.argv[1],encoding="utf-8"):
    r=json.loads(l)
    if r["rc"]==0: continue
    print("=====",r["file"])
    t=open(r["log"],encoding="utf-8").read()
    errs=re.findall(r"^(?:\s*)((?:\w*Error|Error)[^\n]{0,260})",t,re.M)
    seen={}
    for e in errs: seen[e]=seen.get(e,0)+1
    for e,n in sorted(seen.items(),key=lambda x:-x[1])[:int(sys.argv[2]) if len(sys.argv)>2 else 6]: print(n,e)
