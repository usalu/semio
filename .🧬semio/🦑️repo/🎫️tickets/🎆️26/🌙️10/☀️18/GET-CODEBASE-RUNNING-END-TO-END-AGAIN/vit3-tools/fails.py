import json,sys,re
for l in open(sys.argv[1],encoding="utf-8"):
    r=json.loads(l)
    if r["rc"]==0: continue
    print("=====",r["file"][-80:],r["rc"],r["files"],r["tests"])
    if not r.get("log"): continue
    t=open(r["log"],encoding="utf-8").read()
    seen=set()
    for m in re.finditer(r"(FAIL .*|(?:\w*Error)[^\n]*|Failed to (?:resolve|load)[^\n]*|Cannot find [^\n]*|ERR_[A-Z_]+[^\n]*)",t):
        s=m.group(1)[:240]
        if s in seen: continue
        seen.add(s); print("  ",s)
        if len(seen)>int(sys.argv[2] if len(sys.argv)>2 else 12): break
