import json,glob,re,sys,collections
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G=C+"/🗑️generated/tst4-out/"
pats=sys.argv[1:] or ["b0","b1"]
R={}
for p in pats:
    for f in sorted(glob.glob(G+p+"-*.jsonl")):
        for l in open(f,encoding="utf-8"):
            r=json.loads(l); R[r["file"]]=r
P="🧰️framework/🛍️products/🦑️repo/🔨️modules/"
print("suites",len(R),"green",sum(1 for r in R.values() if r["rc"]==0),"pass",sum(r["pass_"] for r in R.values()),"fail",sum(r["fail"] for r in R.values()))
for r in sorted(R.values(),key=lambda r:r["file"]):
    if r["rc"]==0: continue
    t=open(r["log"],encoding="utf-8",errors="replace").read()
    errs=re.findall(r"^(?:error|Error|TypeError|ReferenceError|SyntaxError):? ?[^\n]*",t,re.M)
    c=collections.Counter(e[:100] for e in errs)
    top=c.most_common(1)[0][0] if c else ""
    print(r["rc"],r["pass_"],r["fail"],r["secs"],r["file"].replace(P,"").replace("🧪️tests/",""),"|",top)
