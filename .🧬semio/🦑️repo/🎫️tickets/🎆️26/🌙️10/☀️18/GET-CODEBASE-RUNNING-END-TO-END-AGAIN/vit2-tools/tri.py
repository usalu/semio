import json,re,sys
C=sys.argv[1]
lo,hi=int(sys.argv[2]),int(sys.argv[3])
for i,l in enumerate(open(C+"/🗑️generated/vit2-out/r2.jsonl",encoding="utf-8")):
    r=json.loads(l)
    if not(lo<=i<hi) or r["rc"]==0: continue
    print("=====",i,r["file"][-90:])
    t=open(r["log"],encoding="utf-8").read()
    seen=set()
    for m in re.finditer(r"(FAIL .*|(?:Error|TypeError|ReferenceError|SyntaxError|AssertionError)[^\n]*|Failed to (?:resolve|load)[^\n]*|Cannot find [^\n]*)",t):
        s=m.group(1)[:230]
        if s in seen: continue
        seen.add(s); print("  ",s)
        if len(seen)>14: break
